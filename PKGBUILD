# Maintainer: CheesyPhoenix
#
# This PKGBUILD was generated by `cargo aur`: https://crates.io/crates/cargo-aur

pkgname=hyprland-relative-workspace-bin
pkgver=1.1.1
pkgrel=1
pkgdesc="A tool to switch Hyprland workspaces relatively to monitor"
url="https://github.com/CheesyPhoenix/hyprland-relative-workspace"
license=("MIT")
arch=("x86_64")
provides=("hyprland-relative-workspace")
conflicts=("hyprland-relative-workspace")
source=("hyprland-relative-workspace-$pkgver.tar.gz::https://static.crates.io/crates/hyprland-relative-workspace/hyprland-relative-workspace-$pkgver.crate")
sha256sums=("22f32b790f5f7b049571a727fb4cf17f483ff6650e9d95adb539ef0372e9af16")

package() {
    install -Dm755 hyprland-relative-workspace -t "$pkgdir/usr/bin"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
