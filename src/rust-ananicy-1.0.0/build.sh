#!/bin/bash

# Legacy build script - Use install.sh for comprehensive installation
echo "⚠️  This is a legacy build script."
echo "🚀 For the best experience, use the comprehensive installer:"
echo "   ./install.sh"
echo ""
echo "Continuing with basic build..."

# Build the project
cargo build --release

# Install the binary
sudo cp target/release/rust-ananicy /usr/local/bin/

# Create configuration directory
sudo mkdir -p /etc/ananicy.d/

# Copy service file to systemd
sudo cp rust-ananicy.service /etc/systemd/system/

# Copy configuration files from configs directory
if [ -d "configs" ]; then
    echo "Installing default configuration files..."
    sudo cp configs/* /etc/ananicy.d/
fi

# Enable and start the service
sudo systemctl daemon-reload
sudo systemctl enable rust-ananicy
sudo systemctl start rust-ananicy

echo ""
echo "✅ Basic installation complete!"
echo "💡 Consider using ./install.sh for a better installation experience."