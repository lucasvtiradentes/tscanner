#!/bin/bash

set -e

if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

echo "🧹 Cleaning old Lino installations..."

rm -rf ~/.vscode/extensions/lucasvtiradentes.lino-* 2>/dev/null || true

rm -rf ~/.config/Code/Cache/* 2>/dev/null || true
rm -rf ~/.config/Code/CachedData/* 2>/dev/null || true
rm -rf ~/.vscode/extensions/.obsolete 2>/dev/null || true

echo "📦 Building and installing fresh extension..."
pnpm build

echo ""
echo "✅ Clean installation complete!"
echo ""
echo "🔄 IMPORTANT: Reload VSCode now:"
echo "   - Press Ctrl+Shift+P"
echo "   - Type 'Developer: Reload Window'"
echo "   - Press Enter"
echo ""
echo "This will clear all VSCode extension cache and load the fresh version."
