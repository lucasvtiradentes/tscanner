#!/bin/bash
set -e

ACTION_PKG="packages/github-action/package.json"
CURRENT_VERSION=""
PREVIOUS_VERSION=""

log() {
  echo "github-action-publisher - $1"
}

check_package_exists() {
  log "Checking GitHub Action package state..."
  log "Package: $ACTION_PKG"

  if [ ! -f "$ACTION_PKG" ]; then
    log "ERROR: Package.json not found at $ACTION_PKG"
    exit 1
  fi
}

get_current_version() {
  CURRENT_VERSION=$(node -p "require('./$ACTION_PKG').version")
  log "Current version: $CURRENT_VERSION"
}

get_previous_version() {
  if git rev-parse HEAD^ >/dev/null 2>&1; then
    PREVIOUS_VERSION=$(git show HEAD^:./$ACTION_PKG 2>/dev/null | node -p "try { JSON.parse(require('fs').readFileSync('/dev/stdin', 'utf-8')).version } catch(e) { '' }" || echo "")
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
  log "Determining if GitHub Action should be published..."

  if [ -n "$PREVIOUS_VERSION" ] && [ "$PREVIOUS_VERSION" != "$CURRENT_VERSION" ]; then
    log "Version bumped in this commit: $PREVIOUS_VERSION → $CURRENT_VERSION"
    return 0
  else
    log "Version not changed in this commit"
    return 1
  fi
}

build_action() {
  log "Building GitHub Action..."
  cd packages/github-action
  pnpm build
  cd ../..
}

clone_standalone_repo() {
  log "Cloning standalone repository..."

  if [ -z "$GH_PAT_SYNC_TSCANNER_GH_ACTION" ]; then
    log "ERROR: GH_PAT_SYNC_TSCANNER_GH_ACTION environment variable not set"
    exit 1
  fi

  git clone https://x-access-token:${GH_PAT_SYNC_TSCANNER_GH_ACTION}@github.com/lucasvtiradentes/tscanner-action.git /tmp/tscanner-action
}

sync_files_to_standalone() {
  log "Syncing files to standalone repository..."

  cd /tmp/tscanner-action
  rm -rf *

  cp -r $GITHUB_WORKSPACE/packages/github-action/action.yml .
  cp -r $GITHUB_WORKSPACE/packages/github-action/dist .
  cp $GITHUB_WORKSPACE/packages/github-action/README.md .
  cp $GITHUB_WORKSPACE/packages/github-action/LICENSE .

  TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

  log "Adding version notes to README..."
  NOTES="This repository is automatically generated. If you want to contribute or see the source code, you can find it in the [TScanner monorepo](https://github.com/lucasvtiradentes/tscanner/tree/main/packages/github-action).\n\n- **Current version:** \`v${CURRENT_VERSION}\`\n- **Generated at:** \`${TIMESTAMP}\`\n\n<a href=\"#\"><img src=\"https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/divider.png\" /></a>\n"

  sed -i "/<div align=\"center\">/{N;/<div>/{i\\
$NOTES
}}" README.md
}

commit_and_push() {
  log "Committing and pushing changes..."

  cd /tmp/tscanner-action
  git config user.name "github-actions[bot]"
  git config user.email "github-actions[bot]@users.noreply.github.com"
  git add .
  git commit -m "Sync from monorepo ${GITHUB_SHA}" || log "No changes to commit"

  log "Creating and pushing tag v${CURRENT_VERSION}..."
  git tag v${CURRENT_VERSION} -f
  git push origin main
  git push origin v${CURRENT_VERSION} -f
}

print_debug_info() {
  log "Skipping standalone repository publish"
  log "Debug info:"
  log "   - Current version: $CURRENT_VERSION"
  log "   - Previous version: ${PREVIOUS_VERSION:-unknown}"
  log "   - Package private: $(node -p "require('./$ACTION_PKG').private || false")"
}

main() {
  log "Starting GitHub Action publish process..."
  log "============================================"

  check_package_exists
  get_current_version
  get_previous_version

  if should_publish; then
    log "Publishing to standalone repository..."
    build_action
    clone_standalone_repo
    sync_files_to_standalone
    commit_and_push
    log "GitHub Action v$CURRENT_VERSION published to standalone repository!"
  else
    print_debug_info
  fi

  log "GitHub Action publish process completed!"
  log "============================================"
}

main
