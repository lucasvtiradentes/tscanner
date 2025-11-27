#!/bin/bash

TAG_PREFIX="$1"
VERSION="$2"

if [ -z "$TAG_PREFIX" ] || [ -z "$VERSION" ]; then
  echo "Usage: create-release-tag.sh <prefix> <version>"
  echo "Example: create-release-tag.sh vscode-extension 1.0.0"
  exit 1
fi

TAG_NAME="${TAG_PREFIX}-v${VERSION}"

echo "🏷️  Creating tag: $TAG_NAME"

git config user.name "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"

if git rev-parse "$TAG_NAME" >/dev/null 2>&1; then
  echo "⚠️  Tag $TAG_NAME already exists, skipping"
  exit 0
fi

git tag "$TAG_NAME" -m "Release $TAG_PREFIX v$VERSION"
git push origin "$TAG_NAME"

echo "✅ Tag $TAG_NAME created and pushed"

if command -v gh &> /dev/null && [ -n "$GITHUB_TOKEN" ]; then
  echo "📦 Creating GitHub release..."
  gh release create "$TAG_NAME" \
    --title "$TAG_PREFIX v$VERSION" \
    --notes "Automated release for $TAG_PREFIX version $VERSION" \
    --latest=false \
    2>/dev/null || echo "⚠️  Release creation skipped (may already exist)"
fi
