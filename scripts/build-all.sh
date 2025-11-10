#!/bin/bash

set -e

echo "🏗️  Building all packages..."

if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

echo "🦀 Building Rust workspace..."
cd packages/lino-core
cargo build --release
cd ../..

echo "📦 Building VSCode extension..."
pnpm --filter lino-vscode build

echo "✅ All packages built successfully!"
