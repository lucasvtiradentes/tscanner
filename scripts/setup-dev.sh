#!/bin/bash

set -e

echo "🚀 Setting up Lino development environment..."

if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install: https://rustup.rs/"
    echo ""
    echo "Or if already installed, load the environment:"
    echo "  source \"\$HOME/.cargo/env\""
    exit 1
fi

if ! command -v pnpm &> /dev/null; then
    echo "❌ pnpm not found. Please install: npm install -g pnpm"
    exit 1
fi

echo "📦 Installing Node.js dependencies..."
pnpm install

echo "🦀 Building Rust workspace..."
cd packages/lino-core
cargo build
cd ../..

echo "✅ Development environment ready!"
echo ""
echo "Next steps:"
echo "  - Run 'pnpm dev' to start extension development"
echo "  - Run 'cargo watch -x build' in packages/lino-core for Rust auto-rebuild"
echo "  - Press F5 in VSCode to launch Extension Development Host"
