#!/bin/bash
set -e

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

log "npm packages published!"
log "CLI publish process completed!"
log "============================================"
