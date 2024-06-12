pkgname=fetchcat
pkgver=1.0
pkgdesc="Simple fetch displaying system information and a cute cat."
pkgrel="1"
url="https://github.com/onelric/fetchcat/"
arch=('x86_64')
license=('MIT')
makedepends=('cargo')
_gitroot="git://github.com/onelric/fetchcat.git"
_gitname="fetchcat"

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
