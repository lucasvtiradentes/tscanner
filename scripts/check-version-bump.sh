#!/bin/bash

echo "🔍 Changed files:"
git diff --name-only HEAD^ HEAD || echo "  (none - initial commit)"
echo ""

SHOULD_RELEASE_NPM="false"
SHOULD_RELEASE_VSCODE="false"
SHOULD_RELEASE_GITHUB_ACTION="false"

CLI_PKG="packages/cli/package.json"
CLI_CURRENT=$(node -p "require('./$CLI_PKG').version")
CLI_PREVIOUS=$(git show HEAD^:./$CLI_PKG 2>/dev/null | node -p "try { JSON.parse(require('fs').readFileSync('/dev/stdin', 'utf-8')).version } catch(e) { '' }" || echo "")

echo "🔍 CLI/NPM: $CLI_PREVIOUS → $CLI_CURRENT"
if [ -n "$CLI_PREVIOUS" ] && [ "$CLI_PREVIOUS" != "$CLI_CURRENT" ]; then
  echo "✅ CLI/NPM version bumped"
  SHOULD_RELEASE_NPM="true"
fi

VSCODE_PKG="packages/vscode-extension/package.json"
VSCODE_CURRENT=$(node -p "require('./$VSCODE_PKG').version")
VSCODE_PREVIOUS=$(git show HEAD^:./$VSCODE_PKG 2>/dev/null | node -p "try { JSON.parse(require('fs').readFileSync('/dev/stdin', 'utf-8')).version } catch(e) { '' }" || echo "")

echo "🔍 VSCode Extension: $VSCODE_PREVIOUS → $VSCODE_CURRENT"
if [ -n "$VSCODE_PREVIOUS" ] && [ "$VSCODE_PREVIOUS" != "$VSCODE_CURRENT" ]; then
  echo "✅ VSCode extension version bumped"
  SHOULD_RELEASE_VSCODE="true"
fi

ACTION_PKG="packages/github-action/package.json"
ACTION_CURRENT=$(node -p "require('./$ACTION_PKG').version")
ACTION_PREVIOUS=$(git show HEAD^:./$ACTION_PKG 2>/dev/null | node -p "try { JSON.parse(require('fs').readFileSync('/dev/stdin', 'utf-8')).version } catch(e) { '' }" || echo "")

echo "🔍 GitHub Action: $ACTION_PREVIOUS → $ACTION_CURRENT"
if [ -n "$ACTION_PREVIOUS" ] && [ "$ACTION_PREVIOUS" != "$ACTION_CURRENT" ]; then
  echo "✅ GitHub Action version bumped"
  SHOULD_RELEASE_GITHUB_ACTION="true"
fi

echo ""
echo "should_release_npm=$SHOULD_RELEASE_NPM" >> $GITHUB_OUTPUT
echo "should_release_vscode=$SHOULD_RELEASE_VSCODE" >> $GITHUB_OUTPUT
echo "should_release_github_action=$SHOULD_RELEASE_GITHUB_ACTION" >> $GITHUB_OUTPUT

if [ "$SHOULD_RELEASE_NPM" = "true" ] || [ "$SHOULD_RELEASE_VSCODE" = "true" ]; then
  echo "✅ Building binaries (version bumped)"
  echo "should_release=true" >> $GITHUB_OUTPUT
else
  echo "⏭️  No version bump, skipping binary build"
  echo "should_release=false" >> $GITHUB_OUTPUT
fi
