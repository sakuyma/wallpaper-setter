# Maintainer: Your Name <your.email@example.com>
pkgname=wallpaper-setter
pkgver=0.1.0
pkgrel=1
pkgdesc="GTK4 wallpaper selector with modern styling and ImageMagick thumbnails"
arch=('x86_64')
url="https://github.com/sakuyma/wallpaper-setter"
license=('MIT')
depends=('gtk4' 'imagemagick')
makedepends=('cargo' 'rust' 'pkgconf' 'git')  # Added git
optdepends=('awww: wallpaper daemon for applying wallpapers')

# VCS source: pulls from the 'main' branch of your GitHub repo
source=("$pkgname::git+$url.git#branch=main")
sha256sums=('SKIP')  # VCS sources use SKIP

pkgver() {
    cd "$pkgname"
    # Get version from Cargo.toml or use git describe
    # Since your Cargo.toml likely has a version, use that:
    grep -m1 '^version' Cargo.toml | cut -d '"' -f2
    # Alternative using git tags:
    # git describe --tags --always | sed 's/-/+/g'
}

build() {
    cd "$pkgname"
    cargo build --release --locked
}

check() {
    cd "$pkgname"
    cargo test --release --locked
}

package() {
    cd "$pkgname"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
