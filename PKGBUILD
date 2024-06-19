# Maintainer: elric <elric.thatonemail@gmail.com>
pkgname=catnip
pkgver=0.1.0
pkgdesc="Simple fetch displaying system information and a cute cat."
pkgrel=1
url="https://github.com/onelric/catnip/"
arch=('i686' 'x86_64' 'armv6h' 'armv7h' 'aarch64')
license=('MIT')
makedepends=('rust' 'cargo')
_gitroot="git://github.com/onelric/catnip.git"
_gitname="catnip"

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
} 
