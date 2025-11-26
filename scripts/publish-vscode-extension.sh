#!/bin/bash
set -e

VSCODE_PKG="packages/vscode-extension/package.json"
CURRENT_VERSION=""
PREVIOUS_VERSION=""

log() {
  echo "publish-vscode-extension - $1"
}

check_package_exists() {
  log "Checking VS Code extension state..."
  log "Package: $VSCODE_PKG"

  if [ ! -f "$VSCODE_PKG" ]; then
    log "ERROR: Package.json not found at $VSCODE_PKG"
    exit 1
  fi
}

get_current_version() {
  CURRENT_VERSION=$(node -p "require('./$VSCODE_PKG').version")
  log "Current version: $CURRENT_VERSION"
}

get_previous_version() {
  if git rev-parse HEAD^ >/dev/null 2>&1; then
    PREVIOUS_VERSION=$(git show HEAD^:./$VSCODE_PKG 2>/dev/null | node -p "try { JSON.parse(require('fs').readFileSync('/dev/stdin', 'utf-8')).version } catch(e) { '' }" || echo "")
    if [ -n "$PREVIOUS_VERSION" ]; then
      log "Previous version: $PREVIOUS_VERSION"
    else
      log "Could not read previous version from git"
    fi
  else
    log "No previous commit available (shallow clone or first commit)"
  fi
}

should_publish() {
  log "Determining if VS Code extension should be published..."

  if [ -n "$PREVIOUS_VERSION" ] && [ "$PREVIOUS_VERSION" != "$CURRENT_VERSION" ]; then
    log "Version bumped in this commit: $PREVIOUS_VERSION → $CURRENT_VERSION"
    return 0
  else
    log "Version not changed in this commit"
    return 1
  fi
}

ensure_vsce_installed() {
  if ! command -v vsce &> /dev/null; then
    log "Installing vsce..."
    npm install -g @vscode/vsce
  fi
}

build_extension() {
  log "Building VS Code extension..."
  cd packages/vscode-extension
  pnpm build
}

package_extension() {
  log "Packaging extension..."
  vsce package --no-dependencies
}

publish_to_marketplace() {
  log "Publishing to Marketplace..."

  if [ -n "$AZURE_VSCODE_PAT" ]; then
    log "Using AZURE_VSCODE_PAT from environment"
    vsce publish --no-dependencies --pat "$AZURE_VSCODE_PAT"
  else
    log "Using PAT from vsce login"
    vsce publish --no-dependencies
  fi

  cd ../..
}

print_debug_info() {
  log "Skipping Marketplace publish"
  log "Debug info:"
  log "   - Current version: $CURRENT_VERSION"
  log "   - Previous version: ${PREVIOUS_VERSION:-unknown}"
  log "   - Package private: $(node -p "require('./$VSCODE_PKG').private || false")"
  log "   - Recent git tags:"
  git tag --list "v*" | tail -3 || log "     (none)"
}

main() {
  log "Starting VSCode extension publish process..."
  log "============================================"

  check_package_exists
  get_current_version
  get_previous_version

  if should_publish; then
    log "Publishing to VS Code Marketplace..."
    ensure_vsce_installed
    build_extension
    package_extension
    publish_to_marketplace
    log "VS Code extension v$CURRENT_VERSION published to Marketplace!"

    log "Creating release tag..."
    bash scripts/create-release-tag.sh "vscode-extension" "$CURRENT_VERSION"
  else
    print_debug_info
  fi

  log "VSCode publish process completed!"
  log "============================================"
}

main
