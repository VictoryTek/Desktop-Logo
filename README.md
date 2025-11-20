## Important: Dependency Pinning and Cargo.lock

If you encounter build errors about required Rust versions (e.g., icu, indexmap, etc.), you must:

1. Delete your `Cargo.lock` file.
2. Run `cargo update` locally to regenerate the lockfile with the pinned versions (see `Cargo.toml`).
3. Commit and push the new `Cargo.lock`.

This ensures all dependencies are locked to versions compatible with Rust 1.81.0, as required by the Flatpak SDK 23.08.

If you skip this step, the build may still fail in CI due to incompatible crate versions being pulled in by the old lockfile.
 
 
# Desktop Logo Applet# Desktop Logo Applet



Minimal desktop applet prototype that displays a logo image in a small window (corner positioned)..



## Installation## Installation



### Option 1: Download Pre-built Flatpak (Recommended)### Option 1: Download Pre-built Flatpak (Recommended)



1. **Download from GitHub Actions:**1. **Download from GitHub Actions:**

   - Go to the [Actions tab](https://github.com/VictoryTek/Desktop-Logo/actions) of this repository   - Go to the [Actions tab](https://github.com/VictoryTek/Desktop-Logo/actions) of this repository

   - Find the latest successful workflow run   - Find the latest successful workflow run

   - Download the `desktop-logo-applet-flatpak` artifact   - Download the `desktop-logo-applet-flatpak` artifact

   - Extract the `desktop-logo-applet.flatpak` file   - Extract the `desktop-logo-applet.flatpak` file



2. **Install the Flatpak:**2. **Install the Flatpak:**

   ```bash   ```bash

   # Install the downloaded bundle   # Install the downloaded bundle

   flatpak install --user desktop-logo-applet.flatpak   flatpak install --user desktop-logo-applet.flatpak

      

   # Run the applet   # Run the applet

   flatpak run com.example.DesktopLogoApplet   flatpak run com.example.DesktopLogoApplet




### Option 2: Build Locally (Plain Cargo)### Option 2: Build Locally (Plain Cargo)



```bash```bash

git clone https://github.com/VictoryTek/Desktop-Logo.gitgit clone https://github.com/VictoryTek/Desktop-Logo.git

cd Desktop-Logocd Desktop-Logo

cargo run -p desktop_logo_appletcargo run -p desktop_logo_applet

``````



## Configuration### Run Installed Flatpak


## Configuration & Logo

The applet loads its configuration from `desktop-logo.toml`.

- **Flatpak:**
   - By default, the config is loaded from `/app/share/desktop-logo-applet/desktop-logo.toml` (bundled in the Flatpak).
   - To override, place a `desktop-logo.toml` in your XDG config directory (e.g. `~/.config/desktop-logo.toml`).
   - The app will also check the current directory as a last resort.

- **Logo selection order:**
   1. If `~/.config/desktop-logo-applet/logo.png` exists, it is used as the logo.
   2. Otherwise, the bundled `/app/share/desktop-logo-applet/logo.png` is used (Flatpak default).
   3. Otherwise, the path in `logo_path` from the config is used.

**To use your own logo:**
   - Place a PNG file at `~/.config/desktop-logo-applet/logo.png` (recommended for Flatpak and local runs).
   - Or, edit `logo_path` in your config to point to a custom image.

Example `desktop-logo.toml`:

```toml
logo_path = "assets/logo.png"
position = "BottomRight"  # TopLeft | TopRight | BottomLeft | BottomRight
margin = 64
max_logo_percent = 0.18
opacity = 0.85
```

If the config file is missing, the applet exits with an error. If the logo is missing, the applet will try the next fallback location.

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


## Testing on Cosmic Desktop

To test the Flatpak on Cosmic desktop:

1. **Build the Flatpak bundle:**
   ```sh
   ./scripts/build-flatpak.sh
   ```
   This creates `desktop-logo-applet.flatpak` in your project directory.

2. **Transfer the Flatpak bundle to your Cosmic VM:**
   Use SCP, shared folders, or drag-and-drop to copy `desktop-logo-applet.flatpak` to your Cosmic VM.

3. **Install the Flatpak bundle in the VM:**
   ```sh
   flatpak install --user desktop-logo-applet.flatpak
   ```

4. **Run the applet:**
   ```sh
   flatpak run com.example.DesktopLogoApplet
   ```

5. **Observe and test:**
   - The overlay window should appear, be click-through, and show your logo.
   - The main applet should function as expected.
   - Try interacting with the desktop under the overlay to confirm click-through.

If you encounter any issues, check the terminal for error messages and consult the documentation or support channels.

## Dependency Notes (libcosmic)
This project depends on `libcosmic` via a git source because the crates.io published `cosmic` 0.1.0 lacks several features (including `applet`) that are present on the git repository.

Current dependency line (applet crate):
```
cosmic = { git = "https://github.com/pop-os/libcosmic", default-features = false, features = ["applet"], branch = "master" }
```
Why not use crates.io version?
- The crates.io release does not expose the experimental `applet` feature needed for this prototype.
- Git HEAD evolves quickly; pinning a commit SHA makes builds reproducible.

To pin a specific commit (recommended for CI stability), replace `branch = "master"` with:
```
rev = "<commit-sha>"
```
You can obtain the latest master commit with:
```
git ls-remote https://github.com/pop-os/libcosmic master | awk '{print $1}'
```
Then update `Cargo.toml` accordingly.

If/when an official crates.io release exposes `applet` downstream, you can switch back to a version requirement.

## License
MIT

## Flatpak Development

The project includes Flatpak packaging for distribution. The manifest `flatpak/com.example.CosmicLogoApplet.json` uses the Freedesktop runtime (will switch to COSMIC runtime when available).

Note: The applet currently renders an overlay inside an applet window (not modifying the wallpaper file directly).

## Continuous Integration (GitHub Actions)
A workflow `.github/workflows/flatpak.yml` builds the Flatpak on each push/PR to `main` or `master`:
1. Checks out code.
2. Installs stable Rust.
## License
MIT
- Configurable scaling modes (relative to width/height separately)
