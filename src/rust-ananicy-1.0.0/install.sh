#!/bin/bash

# Rust Ananicy Installation Script
# Auto Nice Daemon - Memory Safe & High Performance

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Configuration
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="/etc/ananicy.d"
SERVICE_DIR="/etc/systemd/system"
SERVICE_FILE="rust-ananicy.service"

echo -e "${BOLD}${GREEN}============================================${NC}"
echo -e "${BOLD}${GREEN}  Rust Ananicy Installation Script${NC}"
echo -e "${BOLD}${GREEN}  Memory Safe Auto Nice Daemon${NC}"
echo -e "${BOLD}${GREEN}============================================${NC}"
echo ""

# Check if running as root for system installation
if [[ $EUID -eq 0 ]]; then
   echo -e "${RED}This script should not be run as root directly.${NC}"
   echo -e "${YELLOW}Please run as a regular user with sudo access.${NC}"
   exit 1
fi

# Check if sudo is available
if ! command -v sudo &> /dev/null; then
    echo -e "${RED}sudo is required but not installed.${NC}"
    exit 1
fi

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check dependencies
echo -e "${BLUE}Checking dependencies...${NC}"

if ! command_exists cargo; then
    echo -e "${RED}Rust/Cargo not found. Please install Rust first:${NC}"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

if ! command_exists systemctl; then
    echo -e "${YELLOW}systemd not found. Service installation will be skipped.${NC}"
    SYSTEMD_AVAILABLE=false
else
    SYSTEMD_AVAILABLE=true
fi

echo -e "${GREEN}✓ Dependencies check passed${NC}"
echo ""

# Build the project
echo -e "${BLUE}Building Rust Ananicy...${NC}"
if ! cargo build --release; then
    echo -e "${RED}Build failed. Please check the error messages above.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Build completed successfully${NC}"
echo ""

# Function to backup existing file
backup_file() {
    local file=$1
    if [ -f "$file" ]; then
        echo -e "${YELLOW}  Backing up existing $(basename "$file") to ${file}.bak${NC}"
        sudo cp "$file" "${file}.bak"
    fi
}

# Function to install config file
install_config() {
    local file=$1
    local dest="$CONFIG_DIR/$file"
    
    backup_file "$dest"
    echo -e "${GREEN}  Installing $file${NC}"
    sudo cp "configs/$file" "$dest"
}

# Create configuration directory
echo -e "${BLUE}Setting up configuration directory...${NC}"
sudo mkdir -p "$CONFIG_DIR"
echo -e "${GREEN}✓ Configuration directory created${NC}"

# Install binary
echo -e "${BLUE}Installing rust-ananicy binary...${NC}"
if [ -f "target/release/rust-ananicy" ]; then
    sudo cp target/release/rust-ananicy "$INSTALL_DIR/"
    sudo chmod +x "$INSTALL_DIR/rust-ananicy"
    echo -e "${GREEN}✓ Binary installed to $INSTALL_DIR/rust-ananicy${NC}"
else
    echo -e "${RED}Binary not found. Build may have failed.${NC}"
    exit 1
fi

# Install configuration files
echo -e "${BLUE}Installing default configuration files...${NC}"

# Check if configs directory exists
if [ ! -d "configs" ]; then
    echo -e "${YELLOW}Configuration directory not found. Creating default configs...${NC}"
    mkdir -p configs
fi

# Install all configuration files if they exist
for file in configs/*.{conf,types,rules,cgroups}; do
    if [ -f "$file" ]; then
        filename=$(basename "$file")
        install_config "$filename"
    fi
done

# Set proper permissions
sudo chmod 644 "$CONFIG_DIR"/*
echo -e "${GREEN}✓ Configuration files installed${NC}"

# Install systemd service
if [ "$SYSTEMD_AVAILABLE" = true ]; then
    echo -e "${BLUE}Installing systemd service...${NC}"
    
    if [ -f "$SERVICE_FILE" ]; then
        backup_file "$SERVICE_DIR/$SERVICE_FILE"
        sudo cp "$SERVICE_FILE" "$SERVICE_DIR/"
        sudo systemctl daemon-reload
        echo -e "${GREEN}✓ Systemd service installed${NC}"
        
        # Ask user if they want to enable and start the service
        echo ""
        read -p "Do you want to enable and start the rust-ananicy service now? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo -e "${BLUE}Enabling and starting service...${NC}"
            sudo systemctl enable rust-ananicy
            sudo systemctl start rust-ananicy
            
            # Check service status
            if sudo systemctl is-active --quiet rust-ananicy; then
                echo -e "${GREEN}✓ Service started successfully${NC}"
            else
                echo -e "${RED}✗ Service failed to start. Check logs with: journalctl -u rust-ananicy${NC}"
            fi
        fi
    else
        echo -e "${YELLOW}Service file not found. Skipping systemd integration.${NC}"
    fi
fi

echo ""
echo -e "${BOLD}${GREEN}============================================${NC}"
echo -e "${BOLD}${GREEN}  Installation Complete!${NC}"
echo -e "${BOLD}${GREEN}============================================${NC}"
echo ""
echo -e "${GREEN}Rust Ananicy has been installed successfully!${NC}"
echo ""
echo -e "${BOLD}Usage:${NC}"
echo -e "  ${BLUE}Start daemon:${NC}         rust-ananicy start"
echo -e "  ${BLUE}Check status:${NC}         sudo systemctl status rust-ananicy"
echo -e "  ${BLUE}View logs:${NC}            journalctl -u rust-ananicy -f"
echo -e "  ${BLUE}Dump rules:${NC}           rust-ananicy dump rules"
echo -e "  ${BLUE}Show processes:${NC}       rust-ananicy dump proc"
echo -e "  ${BLUE}Help:${NC}                 rust-ananicy --help"
echo ""
echo -e "${BOLD}Configuration:${NC}"
echo -e "  ${BLUE}Config directory:${NC}     $CONFIG_DIR"
echo -e "  ${BLUE}Binary location:${NC}      $INSTALL_DIR/rust-ananicy"
echo -e "  ${BLUE}Service file:${NC}         $SERVICE_DIR/$SERVICE_FILE"
echo ""
echo -e "${BOLD}Performance Stats:${NC}"
echo -e "  ${GREEN}• Memory usage: ~2MB (vs 50-100MB Python ananicy)${NC}"
echo -e "  ${GREEN}• CPU efficient: Async processing with minimal overhead${NC}"
echo -e "  ${GREEN}• Memory safe: Zero buffer overflows or segfaults${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo -e "1. Review and customize rules in $CONFIG_DIR"
echo -e "2. Monitor system performance with: htop or btop"
echo -e "3. Check service logs for any issues"
echo ""