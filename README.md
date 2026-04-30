# Wallpaper Setter

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

### From Source

```bash
sudo pacman -S gtk4 imagemagick awwww just --needed
git clone https://github.com/yourusername/wallpapers-rs
cd wallpapers-rs
just
```

## Configuration

### Hyprland

Add to `~/.config/hypr/hyprland.conf`:

```conf
windowrule {
    name = wallpaper 
    match:class = com.wallpaper.setter
    float = true
    size = 900 500 
    center = true
    opacity = 1
    animation = popin 85%
}
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
