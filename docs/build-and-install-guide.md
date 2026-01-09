# Dry Dock Build and Install Guide

This guide covers how to build, install, and automate the deployment of Dry Dock on macOS.

## Prerequisites

- Rust and Cargo installed
- `cargo-bundle` installed (run `cargo install cargo-bundle` if not already installed)
- macOS (for .app bundle creation)

## Manual Build and Install

### Step 1: Build the Release Bundle

Navigate to the project directory and build the release bundle:

```bash
cd /Users/logangarrett03/Development/git/dry-dock/dry-dock
cargo bundle --release
```

This creates a `.app` bundle at:
```
target/release/bundle/osx/Dry Dock.app
```

### Step 2: Install to Applications

**Option A: Command Line**
```bash
cp -r target/release/bundle/osx/Dry\ Dock.app /Applications/
```

**Option B: Finder**
1. Navigate to `target/release/bundle/osx/` in Finder
2. Drag `Dry Dock.app` to your Applications folder

### Step 3: Handle macOS Security

Since the app is not signed, macOS may block it on first launch.

**Method 1: System Settings**
1. Try to open the app
2. Go to **System Settings > Privacy & Security**
3. Click **Open Anyway** next to the Dry Dock message

**Method 2: Remove Quarantine (Recommended)**
```bash
xattr -cr /Applications/Dry\ Dock.app
```

## Automated Build and Install

Use the provided script to automate the entire process.

### Build Script Usage

The `build-and-install.sh` script handles:
- Building the release bundle
- Installing to Applications
- Removing quarantine attributes
- Cleaning up old builds (optional)

**Basic usage:**
```bash
./build-and-install.sh
```

**With cleanup:**
```bash
./build-and-install.sh --clean
```

**Skip installation (build only):**
```bash
./build-and-install.sh --build-only
```

### Script Options

| Option | Description |
|--------|-------------|
| `--clean` | Remove previous build artifacts before building |
| `--build-only` | Build the bundle but don't install it |
| `--skip-quarantine` | Skip removing quarantine attributes |
| `--help` | Show help message |

## Troubleshooting

### App won't open

**Error: "Dry Dock is damaged and can't be opened"**
- Run: `xattr -cr /Applications/Dry\ Dock.app`

**Error: Config file not found**
- Ensure `AppConfig.json` is in the bundle resources
- Check that `resources = ["AppConfig.json"]` is in Cargo.toml

### Icon not showing

**App bundle icon:**
- Verify `app_icon.icns` exists in `assets/icons/`
- Check Cargo.toml has: `icon = ["assets/icons/app_icon.icns"]`

**Window icon:**
- Verify `app_icon.png` exists in `assets/icons/`
- Ensure it's included in resources

### Database issues

The app stores its database at:
```
~/Library/Application Support/DryDock/database.db
```

To reset the database:
```bash
rm -rf ~/Library/Application\ Support/DryDock/database.db*
```

## CI/CD Automation

For automated builds in a CI/CD pipeline:

### GitHub Actions Example

```yaml
name: Build macOS App

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Install cargo-bundle
        run: cargo install cargo-bundle
        
      - name: Build Bundle
        run: |
          cd dry-dock
          cargo bundle --release
          
      - name: Create DMG (optional)
        run: |
          brew install create-dmg
          create-dmg \
            --volname "Dry Dock" \
            --window-pos 200 120 \
            --window-size 800 400 \
            --icon-size 100 \
            --app-drop-link 600 185 \
            "Dry-Dock-Installer.dmg" \
            "dry-dock/target/release/bundle/osx/"
            
      - name: Upload Release Asset
        uses: actions/upload-artifact@v3
        with:
          name: Dry-Dock-macOS
          path: dry-dock/target/release/bundle/osx/Dry Dock.app
```

## Development Workflow

### Quick Rebuild and Install

During development, use the build script for quick iterations:

```bash
# Make your changes...
./build-and-install.sh --clean

# Test the app
open /Applications/Dry\ Dock.app
```

### Debug Build (faster, for testing)

For faster builds during development:

```bash
cargo run
```

This runs without creating a bundle, using files from the current directory.

## Distribution

### Creating a DMG for Distribution

To distribute your app, create a DMG file:

```bash
# Install create-dmg
brew install create-dmg

# Create DMG
create-dmg \
  --volname "Dry Dock Installer" \
  --window-pos 200 120 \
  --window-size 800 400 \
  --icon-size 100 \
  --icon "Dry Dock.app" 200 190 \
  --hide-extension "Dry Dock.app" \
  --app-drop-link 600 185 \
  "Dry-Dock-v1.0.0.dmg" \
  "dry-dock/target/release/bundle/osx/"
```

### Code Signing (for public distribution)

For public distribution without security warnings, you need an Apple Developer account and code signing:

```bash
# Sign the app
codesign --force --deep --sign "Developer ID Application: Your Name" /Applications/Dry\ Dock.app

# Verify signing
codesign --verify --verbose /Applications/Dry\ Dock.app

# Notarize with Apple (requires Apple Developer account)
xcrun notarytool submit Dry-Dock-v1.0.0.dmg --apple-id your@email.com --team-id TEAMID --wait
```

## Version Updates

When releasing a new version:

1. Update version in `Cargo.toml`:
   ```toml
   [package]
   version = "1.1.0"
   
   [package.metadata.bundle]
   version = "1.1.0"
   ```

2. Update version in `AppConfig.json`:
   ```json
   {
     "version": "1.1.0"
   }
   ```

3. Build and distribute:
   ```bash
   ./build-and-install.sh --clean
   ```

## Additional Resources

- [cargo-bundle documentation](https://github.com/burtonageo/cargo-bundle)
- [macOS App Distribution Guide](https://developer.apple.com/documentation/xcode/distributing-your-app-for-beta-testing-and-releases)
- [Code Signing Guide](https://developer.apple.com/support/code-signing/)

## Support

For issues or questions:
- Check the [project README](../README.md)
- Review the [LICENSE](../LICENSE.md)
- Contact: Logan Garrett
