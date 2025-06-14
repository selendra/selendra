#!/bin/bash

# Arch Linux Specific Setup for Selendra Node
# This script optimizes Arch Linux for running Selendra blockchain nodes

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running on Arch Linux
if [ ! -f /etc/arch-release ]; then
    log_error "This script is designed for Arch Linux only"
    exit 1
fi

log_info "Setting up Arch Linux for Selendra blockchain development"

# Update system
log_info "Updating system packages..."
sudo pacman -Syu --noconfirm

# Install essential development packages
log_info "Installing development tools..."
sudo pacman -S --needed --noconfirm \
    base-devel \
    cmake \
    pkgconf \
    openssl \
    git \
    clang \
    llvm \
    rustup \
    protobuf \
    wget \
    curl \
    htop \
    neofetch

# Install Rust through rustup
log_info "Setting up Rust toolchain..."
if ! command -v rustc &> /dev/null; then
    rustup default stable
    rustup update
fi

rustup target add wasm32-unknown-unknown
rustup component add rust-src

# Install additional tools for blockchain development
log_info "Installing blockchain development tools..."
sudo pacman -S --needed --noconfirm \
    docker \
    docker-compose \
    nodejs \
    npm \
    python \
    python-pip

# Enable and start Docker
sudo systemctl enable docker
sudo systemctl start docker
sudo usermod -aG docker $USER

# Install AUR helper (yay) if not present
if ! command -v yay &> /dev/null; then
    log_info "Installing yay AUR helper..."
    cd /tmp
    git clone https://aur.archlinux.org/yay.git
    cd yay
    makepkg -si --noconfirm
    cd -
fi

# Install AUR packages for blockchain development
log_info "Installing AUR packages..."
yay -S --noconfirm \
    polkadot-bin \
    substrate-contracts-node-bin

# Optimize system for blockchain nodes
log_info "Optimizing system settings..."

# Increase file descriptor limits
echo "* soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "* hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# Optimize network settings
cat << EOF | sudo tee /etc/sysctl.d/99-selendra.conf
# Network optimizations for Selendra
net.core.rmem_default = 262144
net.core.rmem_max = 16777216
net.core.wmem_default = 262144
net.core.wmem_max = 16777216
net.ipv4.tcp_rmem = 4096 65536 16777216
net.ipv4.tcp_wmem = 4096 65536 16777216
net.core.netdev_max_backlog = 5000
net.ipv4.tcp_congestion_control = bbr

# File system optimizations
fs.file-max = 2097152
vm.swappiness = 10
vm.dirty_ratio = 15
vm.dirty_background_ratio = 5
EOF

sudo sysctl -p /etc/sysctl.d/99-selendra.conf

# Create directories for blockchain data
sudo mkdir -p /var/lib/selendra
sudo mkdir -p /var/log/selendra

# Install monitoring tools
log_info "Installing monitoring tools..."
sudo pacman -S --needed --noconfirm \
    prometheus \
    grafana \
    htop \
    iotop \
    nethogs

# Create a development environment script
cat << 'EOF' > ~/selendra-dev-env.sh
#!/bin/bash
# Selendra Development Environment

export RUST_LOG=info
export RUSTC_WRAPPER=""
export CARGO_TARGET_DIR="$HOME/.cargo/target"

# Add custom paths
export PATH="$HOME/.cargo/bin:$PATH"

# Blockchain specific environment
export SUBSTRATE_CLI_GIT_COMMIT_HASH=""
export BUILD_DUMMY_WASM_BINARY=1

echo "🦀 Selendra Development Environment Loaded"
echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
echo ""
echo "Quick commands:"
echo "  cargo build --release     # Build optimized binary"
echo "  cargo test                # Run tests"
echo "  ./target/release/selendra-node --dev  # Run dev node"
EOF

chmod +x ~/selendra-dev-env.sh

# Create alias for easy access
echo "alias selendra-env='source ~/selendra-dev-env.sh'" >> ~/.bashrc
echo "alias selendra-env='source ~/selendra-dev-env.sh'" >> ~/.zshrc

# Install VS Code extensions for Rust development (if code is installed)
if command -v code &> /dev/null; then
    log_info "Installing VS Code extensions for Rust development..."
    code --install-extension rust-lang.rust-analyzer
    code --install-extension vadimcn.vscode-lldb
    code --install-extension serayuzgur.crates
    code --install-extension tamasfe.even-better-toml
fi

log_success "Arch Linux setup complete!"
echo ""
echo "🎉 Arch Linux is now optimized for Selendra blockchain development!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📋 Next Steps:"
echo "1. Reboot or re-login to apply group changes"
echo "2. Source the development environment: source ~/selendra-dev-env.sh"
echo "3. Build Selendra: cargo build --release"
echo "4. Run the production setup: ./setup-production-rpc.sh"
echo ""
echo "🔧 Useful Commands:"
echo "   Start dev environment:   selendra-env"
echo "   Check system limits:     ulimit -n"
echo "   Monitor resources:       htop"
echo "   Check Docker:            docker --version"
echo ""
echo "📁 Important Directories:"
echo "   Blockchain data:         /var/lib/selendra"
echo "   Logs:                    /var/log/selendra"
echo "   Development env:         ~/selendra-dev-env.sh"
