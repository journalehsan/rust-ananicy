# Build the project
cargo build --release

# Install the binary
sudo cp target/release/rust-ananicy /usr/local/bin/

# Create configuration directory
sudo mkdir -p /etc/ananicy.d/

# Copy service file to systemd
sudo cp rust-ananicy.service /etc/systemd/system/

# Copy configuration files from original ananicy
# (or create new ones following the same format)

# Enable and start the service
sudo systemctl daemon-reload
sudo systemctl enable rust-ananicy
sudo systemctl start rust-ananicy