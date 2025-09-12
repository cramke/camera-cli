# camera-cli

A command-line tool for camera calculations.

## Development Workflow

This repository uses an automated gitflow:

- **Development Branch**: `development` - All new features and changes should be merged here first
- **Main Branch**: `main` - Production-ready code
- **Automated Sync**: When CI passes on the `development` branch, a pull request is automatically created to merge changes into `main`

### CI/CD Pipeline

1. Push changes to `development` branch
2. CI runs automatically (build, test, format check, clippy)
3. If CI passes, an automated PR is created from `development` to `main`
4. Review and merge the auto-generated PR to deploy to main

This ensures that only tested, properly formatted code reaches the main branch.