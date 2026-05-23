#!/bin/bash
# Build SUMO from source on Fedora
# Run with: bash install_sumo.sh

set -e  # Exit on error

echo "🔧 Installing SUMO dependencies..."
sudo dnf install -y \
    cmake \
    python3 \
    python3-devel \
    xerces-c-devel \
    proj-devel \
    git \
    gcc-c++ \
    make

echo ""
echo "📥 Cloning SUMO repository..."
cd /tmp
if [ -d "sumo" ]; then
    rm -rf sumo
fi
git clone --depth 1 --branch v1_21_0 https://github.com/eclipse/sumo.git
cd sumo

echo ""
echo "🏗️  Building SUMO (this will take 5-10 minutes)..."
mkdir -p build
cd build
cmake ..
make -j$(nproc)

echo ""
echo "📦 Installing SUMO..."
sudo make install

echo ""
echo "✅ Verifying installation..."
netconvert --version

echo ""
echo "🎉 SUMO installed successfully!"
echo ""
echo "Clean up (optional):"
echo "  rm -rf /tmp/sumo"
