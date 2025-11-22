# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is a **customized fork** of `spotify_player` - a terminal-based Spotify player written in Rust. The fork adds custom themes (klu1, klu2) and experimental TachyonFX visual effects to the progress bar.

**Requires Spotify Premium account.**

## Build and Development Commands

### Building

```bash
# Build with default features (includes fx, rodio-backend, media-control)
cargo build --release

# Build with all optional features
cargo build --release --features fx,image,notify,fzf

# Build without effects
cargo build --release --no-default-features --features rodio-backend,media-control

# Build with different audio backends
cargo build --release --no-default-features --features pulseaudio-backend
cargo build --release --no-default-features --features alsa-backend
```

### Testing

```bash
# Run tests with default features
cargo test

# Run tests with specific features (as CI does)
cargo test --no-default-features --features rodio-backend,media-control,image,notify,fzf
```

### Linting and Formatting

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run clippy (with features)
cargo clippy --no-default-features --features rodio-backend,media-control,image,notify,fzf -- -D warnings

# Run clippy (without features)
cargo clippy --no-default-features -- -D warnings

# Check for unused dependencies
cargo install cargo-machete
cargo machete

# Check for typos
cargo install typos-cli
typos
```

### Running

```bash
# Run with default settings
cargo run

# Run with custom config folder
cargo run -- -c /path/to/config

# Run with custom cache folder
cargo run -- -C /path/to/cache

# Run as daemon (requires daemon feature)
cargo run --features daemon -- -d

# Run with specific theme
cargo run -- --theme klu1
```

### CLI Commands

```bash
# First-time authentication
spotify_player authenticate

# CLI commands (require running instance or create temporary client)
spotify_player get playlist <playlist_id>
spotify_player search "query"
spotify_player playback start track --id <track_id>
spotify_player connect <device_id>
spotify_player like
spotify_player playlist new <name>
```

## Architecture

### Workspace Structure

This is a Cargo workspace with two crates:
- **`spotify_player`**: Main terminal application
- **`lyric_finder`**: Library for fetching song lyrics

### Core Components

#### 1. State Management (`state/`)
- **`State`**: Application state wrapped in `Arc` for thread-safe sharing
  - `UIState`: UI state (theme, pages, popups, orientation)
  - `PlayerState`: Playback state (current track, position, playback metadata)
  - `AppData`: Cached Spotify data (playlists, albums, artists, tracks)
- State uses `parking_lot` synchronization primitives (Mutex, RwLock)
- State is cloned across async tasks via `SharedState = Arc<State>`

#### 2. Client (`client/`)
- **`AppClient`**: Main Spotify API client
  - Wraps `rspotify` for Web API calls
  - Wraps `librespot` for streaming integration
  - Handles authentication via OAuth PKCE
  - Manages integrated Spotify Connect device (when streaming enabled)
- **Request-based architecture**: Commands send `ClientRequest` via `flume` channels
- **Handlers** (`handlers.rs`): Process client requests asynchronously

#### 3. UI (`ui/`)
Built with `ratatui` (TUI framework):
- **Page rendering** (`page.rs`): Library, Search, Context, Browse, Lyrics, Queue, CommandHelp
- **Playback window** (`playback.rs`): Shows current track, progress bar, controls
- **Popups** (`popup.rs`): Action lists, device selection, theme switching, search
- **Effects** (`effects.rs`): TachyonFX animations for progress bar (optional `fx` feature)
- Renders in a loop with configurable refresh rate (`app_refresh_duration_in_ms`)

#### 4. Event Handling (`event/`)
- **Terminal events**: Keyboard and mouse input via `crossterm`
- **Key sequence matching**: Vim-style multi-key commands (e.g., `g g`, `s t`)
- **Count prefixes**: Vim-style number prefixes (e.g., `5j` to move down 5 items)
- Events translate to `Command` or `Action` based on keymap config
- Separate handlers for pages vs popups

#### 5. Commands and Actions (`command.rs`)
- **Commands**: Global operations (navigation, playback control, UI changes)
- **Actions**: Context-specific operations on items (add to playlist, follow artist, etc.)
- Actions have different implementations per item type (Track, Album, Artist, Playlist, etc.)

#### 6. Configuration (`config/`)
- **`app.toml`**: General settings, device config, layout
- **`theme.toml`**: User-defined themes (palette, component styles)
- **`keymap.toml`**: Custom keybindings for commands and actions
- Config folder: `$HOME/.config/spotify-player` (customizable via `-c` flag)

#### 7. Streaming Integration (`streaming.rs`)
- Uses `librespot` to create an integrated Spotify Connect device
- Device appears as "spotify-player" in Spotify Connect device list
- Enabled via `streaming` feature + audio backend feature (e.g., `rodio-backend`)
- Can be set to `Always`, `DaemonOnly`, or `Never`

#### 8. Media Control (`media_control.rs`)
- Cross-platform media key support
- Linux: MPRIS DBus integration
- macOS/Windows: OS window event listeners (requires invisible window)

### Data Flow

1. **Terminal Event** → Event Handler → Keymap Lookup → Command/Action
2. **Command/Action** → Send `ClientRequest` via channel → Client Handler
3. **Client Handler** → Call Spotify API → Update `State`
4. **UI Render Loop** → Read `State` → Render with ratatui

### Async Runtime

- Uses `tokio` multi-threaded runtime
- Spawned tasks:
  - Client socket server (for CLI commands)
  - Client event handler
  - Player event watcher
  - Terminal event handler (blocking task)
  - UI render loop (blocking task)
  - Media control watcher (optional, blocking task)

## Key Files

- `spotify_player/src/main.rs`: Entry point, task spawning, initialization
- `spotify_player/src/state/mod.rs`: Core state structure
- `spotify_player/src/client/mod.rs`: Spotify API client
- `spotify_player/src/ui/mod.rs`: Main UI render loop
- `spotify_player/src/event/mod.rs`: Terminal event handling
- `spotify_player/src/command.rs`: Command and action definitions
- `spotify_player/src/config/`: Configuration parsing

## Custom Fork Features

### Custom Themes
- **klu1**: Orange & Black theme
- **klu2**: Dark Blue & White theme
- Themes defined in application code (not just config files)

### TachyonFX Effects (Experimental)
- Enabled via `fx` feature (included in default features)
- Progress bar effects: Shimmer, Pulse, Rainbow, Wave, None
- Configured via `app.toml`:
  ```toml
  enable_effects = true
  progress_bar_effect = "Shimmer"
  ```
- Implementation in `spotify_player/src/ui/effects.rs`

## Configuration Files

Located at `~/.config/spotify-player/` (or custom path via `-c`):
- `app.toml`: General application settings
- `theme.toml`: User-defined themes
- `keymap.toml`: Custom keybindings

Cache files at `~/.cache/spotify-player/` (or custom path via `-C`):
- Log files: `spotify-player-*.log`
- Backtrace files: `spotify-player-*.backtrace`
- Auth tokens, audio cache, cover images

## Feature Flags

- **Default**: `rodio-backend`, `media-control`, `fx`
- **Audio backends**: `alsa-backend`, `pulseaudio-backend`, `rodio-backend`, `portaudio-backend`, `jackaudio-backend`, `rodiojack-backend`, `sdl-backend`, `gstreamer-backend`
- **Optional**: `streaming`, `image`, `sixel`, `pixelate`, `notify`, `daemon`, `fzf`, `fx`
- When changing audio backend, use `--no-default-features` and explicitly enable features

## Important Notes

- **No unit tests**: Project has minimal test coverage (integration-style testing via CI)
- **Workspace lints**: Pedantic clippy lints enabled at workspace level in `Cargo.toml`
- **Thread-safe state**: Use `RwLock` for read-heavy data, `Mutex` for write-heavy
- **Logging**: Uses `tracing` with `RUST_LOG` env var (default: `spotify_player=info,librespot=info`)
- **Session management**: Client requires re-authentication if tokens expire
- **Daemon mode**: Not supported on Windows; conflicts with `media-control` on macOS

## Dependencies

### Linux
```bash
# Debian/Ubuntu
sudo apt install libssl-dev libasound2-dev libdbus-1-dev

# RHEL/Fedora
sudo dnf install openssl-devel alsa-lib-devel dbus-devel
```

### macOS/Windows
- Only Rust/Cargo required for default features
- Additional system libraries may be needed for specific audio backends
