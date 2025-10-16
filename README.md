# Desktop Logo Applet for COSMIC

Simple COSMIC desktop applet (prototype) that overlays a logo on top of the current wallpaper, inspired by Fedora's `background-logo` extension. Project name: Desktop Logo.

## Status
- Core image compositing implemented.
- Applet crate provides configuration loading (`cosmic-logo.toml`).
- Flatpak manifest & desktop file placeholders included.
- Pending: Real libcosmic integration (currently stubbed until stable API details).

## Workspace Layout
```
Cargo.toml (workspace)
crates/
  applet/ -> Applet logic + config loader + overlay rendering
assets/   -> Placeholder images (replace with real ones)
cosmic-logo.toml -> Applet configuration
flatpak/com.example.CosmicLogoApplet.json -> Flatpak manifest (prototype)
data/com.example.CosmicLogoApplet.desktop -> Desktop file stub
```

## Building
Ensure you have a recent stable Rust toolchain installed.

```
cargo build --workspace
```

## Configuration
Edit `cosmic-logo.toml`:
```
logo_path = "assets/logo.png"
position = "BottomRight"  # TopLeft | TopRight | BottomLeft | BottomRight
margin = 64
max_logo_percent = 0.18
opacity = 0.85
```
The applet loads this file at startup (future: path may become XDG config).

## Positions
Use: `tl`, `tr`, `bl`, `br` for Top/Bottom + Left/Right.

## Future COSMIC Integration
When libcosmic (and COSMIC desktop APIs) expose a background overlay or desktop-layer interface, the applet can:
1. Subscribe to wallpaper/display change events.
2. Recalculate scaling for multi-monitor setups.
3. Use per-output surfaces for proper placement.

Planned enhancements:
- Configurable scaling modes (relative to width/height separately)
- Optional caching
- SVG logo rendering (using `resvg` or similar)
- Multi-monitor awareness
- Dynamic recoloring

## Testing
```
cargo test --workspace
```

## License
MIT

## Flatpak (Prototype)
To build a Flatpak locally (example outline, adjust for your environment):
1. Ensure `flatpak-builder` is installed.
2. Run (Linux host required):
  ```
  flatpak-builder build-dir flatpak/com.example.CosmicLogoApplet.json --force-clean
  ```
3. Install locally:
  ```
  flatpak-builder --user --install build-dir flatpak/com.example.CosmicLogoApplet.json --force-clean
  ```

Note: Manifest assumes a generic Freedesktop runtime; switch to a COSMIC runtime when one is published. This applet currently renders an overlay inside an applet window (not modifying the wallpaper file).

## Continuous Integration (GitHub Actions)
A workflow `.github/workflows/flatpak.yml` builds the Flatpak on each push/PR to `main` or `master`:
1. Checks out code.
2. Installs stable Rust.
3. Caches Cargo artifacts.
4. Runs `flatpak-builder` to build & install the applet into a local user repository.
5. Exports a bundle artifact `desktop-logo-applet.flatpak` you can sideload.

To trigger manually, use the workflow dispatch event in GitHub’s Actions UI.

### Local Flatpak Build
From a Linux host with Flatpak installed:
```
chmod +x scripts/build-flatpak.sh
./scripts/build-flatpak.sh
```
Then install or run:
```
flatpak install --user desktop-logo-applet.flatpak
flatpak run com.example.CosmicLogoApplet
```

## Disclaimer
Assets are placeholders. Replace with real wallpaper/logo images. Flatpak packaging and libcosmic integration are stubs pending upstream API stability.
