#!/bin/bash
set -e

CLI_PKG="packages/cli/package.json"

log() {
  echo "cli-publisher - $1"
}

log "Starting CLI packages publish process..."
log "============================================"

if [ -n "$NPM_TOKEN" ]; then
  log "Configuring npm authentication..."
  echo "//registry.npmjs.org/:_authToken=${NPM_TOKEN}" >> ~/.npmrc
fi

log "Publishing npm packages via changesets..."
pnpm exec changeset publish

VERSION=$(node -p "require('./$CLI_PKG').version")
log "npm packages published!"

log "Creating release tag..."
bash scripts/create-release-tag.sh "npm-package" "$VERSION"

log "CLI publish process completed!"
log "============================================"
