# Publishing tscanner GitHub Action

## Overview

This GitHub Action is automatically used in your repository via `.github/workflows/prs.yml`. For others to use it, you need to publish it to GitHub Marketplace.

## Publishing Steps

### 1. Tag the Repository

GitHub Actions are versioned using Git tags. Create a release tag:

```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

### 2. Create GitHub Release

Go to your repository on GitHub:
1. Click **Releases** → **Draft a new release**
2. Choose the tag you just created (`v1.0.0`)
3. Title: `v1.0.0 - Initial Release`
4. Description: Add release notes describing features
5. Check **"Set as the latest release"**
6. Click **Publish release**

### 3. Publish to GitHub Marketplace (Optional)

To make the action discoverable in GitHub Marketplace:

1. Edit your release
2. Check **"Publish this Action to the GitHub Marketplace"**
3. Fill in required fields (category, icon, etc.)
4. Click **Update release**

## Usage by Others

Once published, others can use your action in their workflows:

```yaml
- uses: lucasvtiradentes/tscanner/.github/action@v1
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
```

Or specify exact version:

```yaml
- uses: lucasvtiradentes/tscanner/.github/action@v1.0.0
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
```

## Versioning Best Practices

### Major Version Tag

Move the major version tag (`v1`, `v2`) to always point to latest patch:

```bash
git tag -fa v1 -m "Update v1 to v1.0.1"
git push origin v1 --force
```

This allows users to use `@v1` and always get the latest stable version.

### Semantic Versioning

- `v1.0.0` → Initial release
- `v1.0.1` → Bug fixes
- `v1.1.0` → New features (backward compatible)
- `v2.0.0` → Breaking changes

## Update Workflow

When making changes:

1. **Make code changes** in `src/`
2. **Rebuild**: `pnpm build`
3. **Commit dist/**: `git add dist/ && git commit -m "chore: rebuild dist"`
4. **Create tag**: `git tag v1.0.1`
5. **Push**: `git push origin main --tags`
6. **Create GitHub Release** for the new tag

## Important Notes

- **Always commit `dist/`** - GitHub Actions needs the compiled JavaScript
- **Use local action in your repo** - Your workflow uses `./packages/github-action` which always uses the current code
- **External users use tags** - They reference `@v1.0.0` or `@v1` which points to releases
- **Test before releasing** - The action runs on every PR in your repo, ensuring it works before you tag a release

## Current Setup

Your repository is configured to:
- Use the action from `./packages/github-action` (local, always current)
- Test on every PR
- Post comments with scan results

External users will use:
- Published tags like `@v1.0.0`
- Marketplace listing (if published)
