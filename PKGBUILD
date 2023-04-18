# Maintainer: CheesyPhoenix

pkgname=hyprland-relative-workspace-git
pkgver=r11.02645f9
pkgrel=1
pkgdesc="A tool to switch Hyprland workspaces relatively to monitor"
url="https://github.com/CheesyPhoenix/hyprland-relative-workspace"
license=("MIT")
arch=("x86_64")
provides=("hyprland-relative-workspace")
conflicts=("hyprland-relative-workspace")
source=("git+https://github.com/CheesyPhoenix/hyprland-relative-workspace.git")
makedepends=(cargo git)

md5sums=('SKIP')

pkgver() {
    cd "hyprland-relative-workspace"
    printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"
}

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "hyprland-relative-workspace"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    cd "hyprland-relative-workspace"
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "hyprland-relative-workspace/target/release/hyprland-relative-workspace"
    install -Dm644 "hyprland-relative-workspace/LICENSE" "$pkgdir/usr/share/licenses/hyprland-relative-workspace/LICENSE"
}

