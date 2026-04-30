```markdown
# Wallpaper Launcher

A modern, lightweight GTK4 wallpaper picker for Linux with a clean grid interface.

## Features

- GTK4 graphical interface with borderless window
- Grid layout of wallpaper thumbnails
- Thumbnail caching with ImageMagick for fast loading
- Filenames appear only on hover for a clean look
- Close with ESC key
- Integrates with awww wallpaper daemon for smooth transitions

## Dependencies

- GTK4
- ImageMagick (for thumbnail generation)
- awww (wallpaper daemon)
- Rust (latest stable)

## Installation

### From Source

```bash
git clone https://github.com/yourusername/wallpapers-rs
cd wallpapers-rs
cargo build --release
sudo cp target/release/wallpapers-rs /usr/local/bin/
```

### Install Dependencies
```bash
sudo pacman -S gtk4 imagemagick awwww
```

## Configuration

### Hyprland

Add to `~/.config/hypr/hyprland.conf`:

```conf
windowrulev2 = float, class:^(com.wallpaper.setter)$
windowrulev2 = center, class:^(com.wallpaper.setter)$
```

### Wallpaper Directory

Place your wallpapers in `~/Pictures/wallpapers/`. Supported formats: jpg, jpeg, png, webp, bmp.

## Usage

Run the application:

```bash
wallpapers-rs
```

Browse wallpapers in the grid, click to apply. Press ESC to close.

## License

MIT
```
