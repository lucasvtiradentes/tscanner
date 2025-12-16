#!/bin/bash

TAG_PREFIX="$1"
VERSION="$2"

if [ -z "$VERSION" ]; then
  echo "Usage: create-release-tag.sh <prefix> <version>"
  echo "Example: create-release-tag.sh vscode-extension 1.0.0"
  echo "Example: create-release-tag.sh \"\" 1.0.0  (creates v1.0.0)"
  exit 1
fi

if [ -z "$TAG_PREFIX" ]; then
  TAG_NAME="v${VERSION}"
  RELEASE_TITLE="v$VERSION"
else
  TAG_NAME="${TAG_PREFIX}-v${VERSION}"
  RELEASE_TITLE="$TAG_PREFIX v$VERSION"
fi

echo "🏷️  Creating tag: $TAG_NAME"

git config user.name "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"

if git rev-parse "$TAG_NAME" >/dev/null 2>&1; then
  echo "⚠️  Tag $TAG_NAME already exists, skipping"
  exit 0
fi

git tag "$TAG_NAME" -m "Release $RELEASE_TITLE"
git push origin "$TAG_NAME"

echo "✅ Tag $TAG_NAME created and pushed"

if command -v gh &> /dev/null && [ -n "$GITHUB_TOKEN" ]; then
  echo "📦 Creating GitHub release..."
  gh release create "$TAG_NAME" \
    --title "$RELEASE_TITLE" \
    --notes "Automated release for $RELEASE_TITLE" \
    --latest=false \
    2>/dev/null || echo "⚠️  Release creation skipped (may already exist)"
fi
