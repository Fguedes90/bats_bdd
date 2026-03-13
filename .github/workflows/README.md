# Release CI/CD Pipeline

## Overview

This workflow automatically builds and publishes release binaries to GitHub when a version tag is pushed.

### What it does

1. **Builds binaries** for:
   - Linux (x86_64)
   - macOS (x86_64, ARM64)

2. **Creates GitHub Release** with:
   - Binary archives (.tar.gz)
   - Auto-generated release notes

### Trigger

- Push a tag: `git tag v0.1.0 && git push origin v0.1.0`

### Manual Release

Use GitHub Actions workflow dispatch to create a release manually.

## Usage

```bash
# Create and push a version tag
git tag v0.1.0
git push origin v0.1.0

# This will automatically:
# 1. Build binaries for all platforms
# 2. Create a GitHub release
# 3. Upload the binaries as release assets
```
