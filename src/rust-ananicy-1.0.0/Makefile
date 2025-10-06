# Rust Ananicy Development Makefile

.PHONY: build install clean test aur-prep aur-test release help

# Default target
help:
	@echo "Rust Ananicy - Memory Safe Auto Nice Daemon"
	@echo ""
	@echo "Available targets:"
	@echo "  build      - Build release binary"
	@echo "  install    - Install using the installer script"
	@echo "  test       - Run tests"
	@echo "  clean      - Clean build artifacts"
	@echo "  aur-prep   - Prepare files for AUR submission"
	@echo "  aur-test   - Test PKGBUILD locally"
	@echo "  release    - Create a new release"
	@echo "  help       - Show this help message"

# Build the project
build:
	@echo "Building rust-ananicy..."
	cargo build --release
	@echo "✓ Build completed. Binary: target/release/rust-ananicy"

# Install using the installer script
install:
	@echo "Running comprehensive installer..."
	./install.sh

# Run tests
test:
	@echo "Running tests..."
	cargo test --release
	cargo clippy -- -D warnings
	@echo "✓ Tests passed"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -f *.tar.gz
	@echo "✓ Clean completed"

# Prepare for AUR submission
aur-prep: clean
	@echo "Preparing for AUR submission..."
	@echo "Checking required files..."
	@test -f PKGBUILD || (echo "❌ PKGBUILD not found" && exit 1)
	@test -f .SRCINFO || (echo "❌ .SRCINFO not found" && exit 1)
	@echo "✓ AUR files present"
	@echo ""
	@echo "Next steps:"
	@echo "1. Create a release tag: git tag -a v1.0.0 -m 'Release v1.0.0'"
	@echo "2. Push the tag: git push origin v1.0.0"  
	@echo "3. Update checksums in PKGBUILD"
	@echo "4. Test with: make aur-test"
	@echo "5. Submit to AUR following AUR_SUBMISSION.md"

# Test PKGBUILD locally (requires makepkg)
aur-test:
	@echo "Testing PKGBUILD..."
	@command -v makepkg >/dev/null 2>&1 || (echo "❌ makepkg not found. Install base-devel package." && exit 1)
	makepkg -f
	@echo "✓ PKGBUILD test completed"

# Create a new release
release:
	@echo "Creating new release..."
	@read -p "Enter version (e.g., 1.0.0): " version; \
	git tag -a "v$$version" -m "Release version $$version"; \
	echo "✓ Tag v$$version created"; \
	echo ""; \
	echo "Push with: git push origin v$$version"

# Development helpers
dev-deps:
	@echo "Installing development dependencies..."
	cargo install cargo-clippy
	cargo install cargo-audit
	@echo "✓ Development dependencies installed"

security-audit:
	@echo "Running security audit..."
	cargo audit
	@echo "✓ Security audit completed"

format:
	@echo "Formatting code..."
	cargo fmt
	@echo "✓ Code formatted"

check: test security-audit format
	@echo "✓ All checks passed"

# Performance testing
bench:
	@echo "Running benchmarks..."
	cargo bench
	@echo "✓ Benchmarks completed"

# Documentation
docs:
	@echo "Building documentation..."
	cargo doc --no-deps --open
	@echo "✓ Documentation built"

# Install tools for AUR submission
aur-tools:
	@echo "Installing AUR tools..."
	@command -v paru >/dev/null 2>&1 || (echo "Installing paru..."; sudo pacman -S paru)
	@command -v namcap >/dev/null 2>&1 || (echo "Installing namcap..."; sudo pacman -S namcap)
	@echo "✓ AUR tools installed"

# Create source tarball
tarball:
	@echo "Creating source tarball..."
	@version=$$(grep '^pkgver=' PKGBUILD | cut -d'=' -f2); \
	git archive --format=tar.gz --prefix=rust-ananicy-$$version/ HEAD > rust-ananicy-$$version.tar.gz; \
	echo "✓ Created rust-ananicy-$$version.tar.gz"; \
	sha256sum rust-ananicy-$$version.tar.gz

# Update .SRCINFO
srcinfo:
	@echo "Updating .SRCINFO..."
	@command -v makepkg >/dev/null 2>&1 || (echo "❌ makepkg not found" && exit 1)
	makepkg --printsrcinfo > .SRCINFO
	@echo "✓ .SRCINFO updated"