# Maintainer: CheesyPhoenix
#
# This PKGBUILD was generated by `cargo aur`: https://crates.io/crates/cargo-aur

pkgname=hyprland-relative-workspace-bin
pkgver=1.1.0
pkgrel=1
pkgdesc="A tool to switch Hyprland workspaces relatively to monitor"
url="https://github.com/CheesyPhoenix/hyprland-relative-workspace"
license=("MIT")
arch=("x86_64")
provides=("hyprland-relative-workspace")
conflicts=("hyprland-relative-workspace")
source=("https://github.com/CheesyPhoenix/hyprland-relative-workspace/releases/download/v$pkgver/hyprland-relative-workspace-$pkgver-x86_64.tar.gz")
sha256sums=("10cd5400d7e81a3ba7f9df65d689591e9d60d0326b7b65f357435d5ab3818708")

package() {
    install -Dm755 hyprland-relative-workspace -t "$pkgdir/usr/bin"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
