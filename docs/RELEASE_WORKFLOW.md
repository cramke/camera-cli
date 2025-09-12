# Release Workflow Examples

This document shows how the automated release workflow responds to different conventional commit messages:

## Version Bump Examples

### Patch Release (0.1.0 → 0.1.1)
```bash
git commit -m "fix: resolve camera calculation accuracy issue"
git commit -m "fix(cli): handle invalid input gracefully"
```

### Minor Release (0.1.0 → 0.2.0)  
```bash
git commit -m "feat: add new zoom calculation feature"
git commit -m "feat(camera): implement depth of field calculator"
```

### Major Release (0.1.0 → 1.0.0)
```bash
git commit -m "feat!: redesign API for better usability"
git commit -m "fix!: change default behavior for compatibility"

# Or using body:
git commit -m "feat: add new API

BREAKING CHANGE: the old calculate() method has been replaced"
```

## Workflow Trigger

The release workflow will:
1. **Trigger**: On any push to the `main` branch
2. **Analyze**: Conventional commit messages since the last release
3. **Calculate**: New version based on highest change type found
4. **Update**: `Cargo.toml` with new version
5. **Create**: Git tag with semantic version (e.g., `v1.2.3`)
6. **Generate**: Release notes from commit messages
7. **Publish**: GitHub release with changelog

## Skip Release

To push to main without triggering a release:
```bash
git commit -m "docs: update README [skip ci]"
git commit -m "chore: update dependencies [skip ci]"
```

Release commits are automatically skipped to prevent infinite loops.