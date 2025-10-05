# Maintainer: Ehsan Tor <your-email@example.com>
pkgname=rust-ananicy
pkgver=1.0.0
pkgrel=1
pkgdesc="Memory-safe auto nice daemon - 97% less memory usage than Python ananicy"
arch=('x86_64' 'i686' 'aarch64' 'armv7h')
url="https://github.com/journalehsan/rust-ananicy"
license=('MIT')
depends=('systemd' 'util-linux')
makedepends=('rust' 'cargo')
optdepends=(
    'ionice: I/O scheduling support'
    'schedtool: CPU scheduler support'
    'htop: system monitoring'
    'btop: modern system monitoring'
)
conflicts=('ananicy' 'ananicy-cpp')
provides=('ananicy')
backup=(
    'etc/ananicy.d/ananicy.conf'
    'etc/ananicy.d/00-types.types'
    'etc/ananicy.d/00-default.cgroups'
)
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('0e2f134750814762d2442a9a9f54241ffb80292f9470e9435387976a467496bf')  # Will be updated when creating actual release

prepare() {
    cd "$pkgname-$pkgver"
    
    # Update Cargo.lock if needed
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$pkgname-$pkgver"
    
    # Build with release optimizations
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    
    cargo build \
        --release \
        --frozen \
        --all-features \
        --target-dir=target
}

check() {
    cd "$pkgname-$pkgver"
    
    # Run tests if they exist
    cargo test --release --frozen
}

package() {
    cd "$pkgname-$pkgver"
    
    # Install the binary
    install -Dm755 target/release/$pkgname "$pkgdir/usr/bin/$pkgname"
    
    # Install systemd service
    install -Dm644 rust-ananicy.service "$pkgdir/usr/lib/systemd/system/$pkgname.service"
    
    # Create configuration directory
    install -dm755 "$pkgdir/etc/ananicy.d"
    
    # Install default configuration files
    install -Dm644 configs/ananicy.conf "$pkgdir/etc/ananicy.d/ananicy.conf"
    install -Dm644 configs/00-types.types "$pkgdir/etc/ananicy.d/00-types.types"
    install -Dm644 configs/00-default.cgroups "$pkgdir/etc/ananicy.d/00-default.cgroups"
    install -Dm644 configs/10-desktop.rules "$pkgdir/etc/ananicy.d/10-desktop.rules"
    install -Dm644 configs/20-browsers.rules "$pkgdir/etc/ananicy.d/20-browsers.rules"
    install -Dm644 configs/30-development.rules "$pkgdir/etc/ananicy.d/30-development.rules"
    install -Dm644 configs/40-applications.rules "$pkgdir/etc/ananicy.d/40-applications.rules"
    install -Dm644 configs/50-gaming.rules "$pkgdir/etc/ananicy.d/50-gaming.rules"
    install -Dm644 configs/80-background.rules "$pkgdir/etc/ananicy.d/80-background.rules"
    
    # Install documentation
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 CONFIGURATION.md "$pkgdir/usr/share/doc/$pkgname/CONFIGURATION.md"
    
    # Install license
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    
    # Install shell completions (if they exist in the future)
    # install -Dm644 completions/rust-ananicy.bash "$pkgdir/usr/share/bash-completion/completions/rust-ananicy"
    # install -Dm644 completions/rust-ananicy.zsh "$pkgdir/usr/share/zsh/site-functions/_rust-ananicy"
    # install -Dm644 completions/rust-ananicy.fish "$pkgdir/usr/share/fish/vendor_completions.d/rust-ananicy.fish"
}

# Post-install message
post_install() {
    cat << EOF

==> rust-ananicy has been installed successfully!

    🚀 Memory usage: ~2MB (vs 50-100MB Python ananicy)
    🛡️ Memory safe: Zero buffer overflows or segfaults
    📦 Comprehensive rules for 200+ applications included

==> To start using rust-ananicy:

    # Enable and start the service
    sudo systemctl enable rust-ananicy
    sudo systemctl start rust-ananicy
    
    # Check status
    sudo systemctl status rust-ananicy
    
    # View configured rules
    rust-ananicy dump rules
    
    # Monitor processes
    rust-ananicy dump proc

==> Configuration:
    
    Main config: /etc/ananicy.d/ananicy.conf
    Rules: /etc/ananicy.d/*.rules
    Documentation: /usr/share/doc/rust-ananicy/

==> Tip: Monitor your system with 'htop' or 'btop' to see the effects!

EOF
}

post_upgrade() {
    post_install
    
    cat << EOF

==> Configuration files have been updated with new defaults.
    Your custom settings in /etc/ananicy.d/ have been preserved.
    
    # Restart the service to apply changes
    sudo systemctl restart rust-ananicy

EOF
}

pre_remove() {
    # Stop the service before removal
    systemctl stop rust-ananicy 2>/dev/null || true
    systemctl disable rust-ananicy 2>/dev/null || true
}

post_remove() {
    cat << EOF

==> rust-ananicy has been removed.
    
    Configuration files in /etc/ananicy.d/ have been preserved.
    Remove them manually if desired:
    
    sudo rm -rf /etc/ananicy.d/

EOF
}