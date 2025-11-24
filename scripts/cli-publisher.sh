#!/bin/bash
set -e

log() {
  echo "cli-publisher - $1"
}

log "Starting CLI packages publish process..."
log "============================================"

log "Publishing npm packages via changesets..."
changeset publish

log "npm packages published!"
log "CLI publish process completed!"
log "============================================"
