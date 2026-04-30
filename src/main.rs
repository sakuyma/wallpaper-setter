use dirs;
use gtk4::gdk_pixbuf::Pixbuf;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, CssProvider, FlowBox, FlowBoxChild, Image, Label,
    Orientation, ScrolledWindow,
};
use magick_rust::{MagickWand, magick_wand_genesis};
use std::cell::RefCell;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;
use std::sync::Once;
use walkdir::WalkDir;

static START: Once = Once::new();
const WALLPAPER_DIR: &str = "Pictures/wallpapers";
const CACHE_DIR: &str = ".cache/wallpaper-thumbnails";
const PERSISTENT_WALL: &str = ".current_wallpaper";

fn main() {
    let app = Application::builder()
        .application_id("com.wallpaper.setter")
        .build();

    app.connect_activate(|app| {
        // Setup cache directory
        setup_cache_dir();

        let wallpapers = collect_wallpapers();
        let wallpapers = Rc::new(RefCell::new(wallpapers));

        // Generate thumbnails using ImageMagick
        let thumbnails = generate_thumbnails_magick(&wallpapers.borrow());
        let thumbnails = Rc::new(RefCell::new(thumbnails));

        // Create window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Wallpaper Launcher")
            .default_width(800)
            .default_height(600)
            .decorated(false)
            .resizable(true)
            .build();

        window.set_modal(true);
        window.set_default_size(900, 500);
        window.set_size_request(900, 500);
        // Add CSS styling (rofi-like)
        setup_css(&window);

        // Main container
        let main_box = Box::new(Orientation::Vertical, 12);
        main_box.set_margin_start(12);
        main_box.set_margin_end(12);
        main_box.set_margin_top(12);
        main_box.set_margin_bottom(12);

        // Header
        let header_label = Label::new(Some("Select Wallpaper"));
        header_label.add_css_class("header-label");
        main_box.append(&header_label);

        // Grid of wallpapers (FlowBox - like rofi grid)
        let flow_box = FlowBox::new();
        flow_box.set_max_children_per_line(3);
        flow_box.set_min_children_per_line(3);
        flow_box.set_column_spacing(12);
        flow_box.set_row_spacing(12);
        flow_box.set_homogeneous(true);
        flow_box.add_css_class("flow-box");

        // Fill the grid
        let thumbnails_clone = thumbnails.clone();

        for (idx, wallpaper) in wallpapers.borrow().iter().enumerate() {
            let name = wallpaper.file_name().unwrap().to_string_lossy().to_string();
            let thumb_path = thumbnails_clone.borrow().get(idx).unwrap().clone();

            // Create container for wallpaper item
            let item_box = Box::new(Orientation::Vertical, 6);
            item_box.add_css_class("wallpaper-item");

            // Thumbnail image
            let image = Image::new();
            if let Ok(pixbuf) = Pixbuf::from_file(&thumb_path) {
                image.set_from_pixbuf(Some(&pixbuf));
            }
            image.set_pixel_size(200);
            image.add_css_class("thumbnail");

            // Label with filename
            let label = Label::new(Some(&name));
            label.add_css_class("wallpaper-label");
            label.set_max_width_chars(20);
            label.set_ellipsize(gtk4::pango::EllipsizeMode::End);

            item_box.append(&image);
            item_box.append(&label);

            // Create FlowBoxChild and add it
            let child = FlowBoxChild::new();
            child.set_child(Some(&item_box));
            child.add_css_class("flowbox-child");

            // Store wallpaper path as data
            let wallpaper_path = wallpaper.clone();
            let window_clone = window.clone();

            child.connect_activate(move |_| {
                apply_wallpaper(&wallpaper_path);

                // Close after applying
                window_clone.close();
            });

            flow_box.insert(&child, -1);
        }

        // Scrolled window for grid
        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(&flow_box));
        scrolled.set_vexpand(true);
        scrolled.add_css_class("scrolled-window");

        main_box.append(&scrolled);

        // ESC key handler
        let window_clone = window.clone();
        let controller = gtk4::EventControllerKey::new();
        controller.connect_key_pressed(move |_controller, key, _code, _state| {
            if key == gtk4::gdk::Key::Escape {
                window_clone.close();
                return gtk4::glib::Propagation::Stop;
            }
            gtk4::glib::Propagation::Proceed
        });
        window.add_controller(controller);

        window.set_child(Some(&main_box));
        window.show();
    });

    app.run();
}

fn setup_cache_dir() {
    let home = dirs::home_dir().unwrap();
    let cache_dir = home.join(CACHE_DIR);
    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir).expect("Failed to create cache directory");
    }
}

fn collect_wallpapers() -> Vec<PathBuf> {
    let home = dirs::home_dir().unwrap();
    let wallpaper_dir = home.join(WALLPAPER_DIR);

    let mut wallpapers = Vec::new();

    if wallpaper_dir.exists() {
        for entry in WalkDir::new(wallpaper_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(ext) = entry.path().extension() {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                if matches!(ext_lower.as_str(), "jpg" | "jpeg" | "png" | "webp" | "bmp") {
                    wallpapers.push(entry.into_path());
                }
            }
        }
    }

    wallpapers.sort();
    wallpapers
}

fn generate_thumbnails_magick(wallpapers: &[PathBuf]) -> Vec<PathBuf> {
    let home = dirs::home_dir().unwrap();
    let cache_dir = home.join(CACHE_DIR);
    let mut thumbnails = Vec::new();

    // Инициализируем ImageMagick (один раз для всей программы)
    START.call_once(|| {
        magick_wand_genesis();
    });

    for wallpaper in wallpapers {
        let filename = wallpaper.file_stem().unwrap().to_string_lossy().to_string();
        let thumb_path = cache_dir.join(format!("{}.png", filename));

        if !thumb_path.exists() {
            // Используем библиотеку вместо команды
            let wand = MagickWand::new();

            // Читаем изображение
            if wand.read_image(wallpaper.to_str().unwrap()).is_err() {
                eprintln!("Failed to read image: {:?}", wallpaper);
                continue;
            }

            // Устанавливаем размер как в команде magick: 150x85^
            // Сначала получаем оригинальные размеры
            let orig_width = wand.get_image_width();
            let orig_height = wand.get_image_height();

            // Вычисляем масштаб для cover (как ^ в ImageMagick)
            let target_w = 300;
            let target_h = 170;
            let scale_w = target_w as f64 / orig_width as f64;
            let scale_h = target_h as f64 / orig_height as f64;
            let scale = scale_w.max(scale_h); // cover behavior

            let new_width = (orig_width as f64 * scale) as usize;
            let new_height = (orig_height as f64 * scale) as usize;

            // Изменяем размер
            let _ = wand.resize_image(
                new_width,
                new_height,
                magick_rust::FilterType::LanczosRadius,
            );

            // Обрезаем до точного размера с центром (как -gravity center -extent)
            let _ = wand.crop_image(
                target_w,
                target_h,
                ((new_width - target_w) / 2) as isize,
                ((new_height - target_h) / 2) as isize,
            );

            // Сохраняем thumbnail
            if wand.write_image(thumb_path.to_str().unwrap()).is_err() {
                eprintln!("Failed to write thumbnail: {:?}", thumb_path);
                continue;
            }
        }

        thumbnails.push(thumb_path);
    }

    thumbnails
}

fn apply_wallpaper(path: &PathBuf) {
    let home = dirs::home_dir().unwrap();
    let persistent_wall = home.join(PERSISTENT_WALL);

    // Create persistent symlink (like in bash script)
    let _ = std::os::unix::fs::symlink(path, &persistent_wall);

    let path_str = path.to_str().unwrap();

    // Apply wallpaper with awww (like in bash script)
    if Command::new("awww").arg("--version").output().is_ok() {
        let fps = gtk4::gdk::Display::default()
    .unwrap()
    .monitors()
    .item(0)
    .unwrap()
    .downcast::<gtk4::gdk::Monitor>()
    .unwrap()
    .refresh_rate()
    / 1000;
        let _ = Command::new("awww")
            .args([
                "img",
                path_str,
                "--transition-type",
                "any",
                "--transition-fps",
                &fps.to_string(),
            ])
            .status();
        println!("Applied via awww: {}", path_str);
    } else {
        eprintln!("No supported wallpaper daemon found (awww not found)");
        eprintln!("Install awww: https://github.com/arz-arz/awww");
    }
}

fn setup_css(window: &ApplicationWindow) {
    // CSS styled like rofi theme
    let css = r#"
        * {
            background-color: #1e1e2e;
            color: #b4befe;
        }
        
        window {
            background-color: #1e1e2e;
            border: 2px solid #b4befe;
            border-radius: 12px;
            padding: 6px;
        }
        
        .header-label {
            font-size: 16px;
            font-weight: bold;
            margin: 10px;
            color: #b4befe;
        }
        
        .flow-box {
            background-color: transparent;
            margin: 6px;
        }
        
        .flowbox-child {
            background-color: transparent;
            padding: 2px;
            border-radius: 6px;
        }
        
        .flowbox-child:hover {
            background-color: rgba(180, 190, 254, 0.1);
        }
        
        .flowbox-child:selected {
            background-color: #b4befe;
        }
        
        .wallpaper-item {
            background-color: transparent;
            orientation: vertical;
            padding: 2px;
        }
        
        .thumbnail {
            border-radius: 6px;
            background-color: transparent;
        min-width: 200px;
        min-heigh: 113px;
        width: 200px;
        height: 113px;
        }
        
        .wallpaper-label {
            text-align: center;
            font-size: 11px;
            margin-top: 4px;
            color: #b4befe;
            opacity: 0;
            transition: opacity 0.2s ease;
        }
        
        .flowbox-child:selected .wallpaper-label {
            color: #1e1e2e;
        }
        
        .scrolled-window {
            background-color: transparent;
        }
        
        button {
            background-color: #b4befe;
            color: #1e1e2e;
            border: none;
            border-radius: 6px;
            padding: 8px 16px;
            margin: 4px;
        }
        
        button:hover {
            background-color: #cba6f7;
        }
        
        .close-button {
            background-color: #f38ba8;
            color: #1e1e2e;
        }
        
        .close-button:hover {
            background-color: #eba0ac;
        }
        
        scrollbar {
            background-color: transparent;
            opacity: 0;
        }
        
        scrollbar slider {
            background-color: transparent;
            opacity: 0;
        }
        
        scrollbar slider:hover {
            background-color: rgba(180, 190, 254, 0.5);
        }
    "#;

    let provider = CssProvider::new();
    provider.load_from_data(css);

    let display = gtk4::prelude::WidgetExt::display(window);
    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
