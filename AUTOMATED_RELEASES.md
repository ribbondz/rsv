# Automated Multi-Architecture Release Workflow

This document describes the automated release workflow implemented for the rsv project.

## Problem Solved

Previously, releases were created manually using a Taskfile, which led to:
- Inconsistent architecture support (macOS builds sometimes missing)
- Manual process prone to human error
- Time-consuming release process
- Issue #16: Missing aarch64-apple-darwin (macOS Apple Silicon) releases

## Solution

An automated GitHub Actions workflow that triggers on git tag push and builds binaries for 6 architectures in parallel.

## Supported Architectures

Every release now includes binaries for:

1. **x86_64-pc-windows-msvc** - Windows 64-bit (MSVC)
2. **x86_64-unknown-linux-gnu** - Linux 64-bit (glibc)
3. **x86_64-unknown-linux-musl** - Linux 64-bit static (musl libc)
4. **x86_64-apple-darwin** - macOS Intel 64-bit
5. **aarch64-apple-darwin** - macOS Apple Silicon (M1/M2/M3)
6. **aarch64-unknown-linux-gnu** - Linux ARM64 (Raspberry Pi, ARM servers)

## How to Create a Release

### Prerequisites
1. All changes merged to master
2. CI tests passing
3. Version number decided (e.g., v0.4.20)

### Release Process

```bash
# 1. Create and push tag
git tag v0.4.20
git push origin v0.4.20

# 2. Wait for GitHub Actions workflow (~15-25 minutes)
# Monitor at: https://github.com/matrixise/rsv/actions

# 3. Verify release created
# Check: https://github.com/matrixise/rsv/releases

# 4. Download and test binaries (recommended)
# - Test at least: Windows, macOS ARM, Linux musl
```

That's it! The workflow handles everything automatically.

## Workflow Architecture

```
Tag push: v*
    ↓
[create-release] Creates GitHub Release
    ↓
    ├─→ [build-windows] (1 target)  → Upload .zip
    ├─→ [build-linux]   (3 targets) → Upload .zip (matrix)
    └─→ [build-macos]   (2 targets) → Upload .zip (matrix)

All build jobs run in parallel
Total time: ~15-25 minutes
```

## Technical Details

### Windows Build
- Runner: `windows-latest`
- Native compilation with MSVC toolchain
- Creates: `x86_64-pc-windows-msvc.zip`

### Linux Builds
- Runner: `ubuntu-latest`
- Uses `cross` tool for cross-compilation
- Matrix strategy builds 3 targets in parallel:
  - `x86_64-unknown-linux-gnu.zip`
  - `x86_64-unknown-linux-musl.zip`
  - `aarch64-unknown-linux-gnu.zip`

### macOS Builds
- Uses native runners for each architecture:
  - `macos-13` (Intel) for x86_64-apple-darwin
  - `macos-14` (Apple Silicon) for aarch64-apple-darwin
- Native compilation avoids cross-compilation issues
- Creates:
  - `x86_64-apple-darwin.zip`
  - `aarch64-apple-darwin.zip`

### Optimizations
- **Rust cache** (`Swatinem/rust-cache@v2`): Speeds up subsequent builds 5-10x
- **Parallel jobs**: All builds run simultaneously after release creation
- **Release profile**: LTO, strip symbols, optimized for size and performance

## Release Asset Format

Each `.zip` file contains:
- `README.md` - Project documentation
- `rsv` or `rsv.exe` - Compiled binary

## Troubleshooting

### If a build fails
1. Other builds will continue (fail-fast: false)
2. Check the workflow logs: https://github.com/matrixise/rsv/actions
3. Fix the issue and create a new tag (e.g., v0.4.21)

### Manual re-run
If a build fails due to transient issues:
1. Go to the failed workflow run
2. Click "Re-run failed jobs"
3. Only failed jobs will re-run

### Delete a tag (if needed)
```bash
# Local
git tag -d v0.4.20

# Remote
git push origin :refs/tags/v0.4.20
```

## CI Improvements

The CI workflow (`.github/workflows/ci.yml`) now includes:
- **Tests**: `cargo test` on all platforms (Linux, Windows, macOS)
- **Linting**: `cargo clippy` with warnings as errors
- **Formatting**: `cargo fmt` check
- **Multi-platform**: Tests on ubuntu-latest, windows-latest, macos-latest

This ensures code quality before tags are created.

## Files Modified

### New Files
- `.github/workflows/release.yml` - Automated release workflow
- `.github/workflows/ci.yml` - Improved CI workflow with tests and linting
- `AUTOMATED_RELEASES.md` - This documentation

### Modified Files
- `Cargo.toml` - Added release profile optimizations

### Deleted Files
- `.github/workflows/rust.yml` - Replaced by improved `ci.yml`

## Benefits

1. **Reliability**: Every release includes all 6 architectures
2. **Speed**: Parallel builds complete in 15-25 minutes
3. **Consistency**: Reproducible builds, no manual variations
4. **Transparency**: All logs visible in GitHub Actions
5. **Zero intervention**: Push tag → complete release
6. **Scalability**: Easy to add more architectures
7. **Cost**: Uses free GitHub-hosted runners

## Future Improvements

Potential enhancements for later:
- SHA256 checksums for binaries
- Code signing (macOS, Windows)
- Auto-publish to crates.io
- Docker images with rsv pre-installed
- Automated release notes from commits
- Integration tests on release binaries

## Support

For issues with the release workflow:
1. Check GitHub Actions logs
2. Open an issue on the repository
3. Tag with `ci` or `release` labels
