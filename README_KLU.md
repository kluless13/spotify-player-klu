# Spotify Player - KLU Edition 🎵✨

A customized fork of [spotify_player](https://github.com/aome510/spotify-player) with enhanced visual effects and custom themes!

## 🌟 New Features

### Custom Themes
- **klu1**: Orange & Black theme with vibrant orange highlights and clean black text
- **klu2**: Dark Blue & White theme with elegant blue tones

### TachyonFX Animations (Experimental)
Added visual effects and animations to make your music experience more engaging!

#### Progress Bar Effects
- **Shimmer**: A sliding shine effect across the progress bar
- **Pulse**: Pulsing glow effect that breathes with your music
- **Rainbow**: Cycling through rainbow colors
- **Wave**: Wave-like movement animation
- **None**: Disable effects (default for compatibility)

## 🚀 Installation

### From Source (with effects enabled)
```bash
cargo install --path spotify_player --features fx
```

### Without effects
```bash
cargo install --path spotify_player
```

## ⚙️ Configuration

### Using Custom Themes

Edit `~/.config/spotify-player/app.toml`:

```toml
theme = "klu1"  # or "klu2"
```

### Theme Descriptions

**klu1** - Orange & Black
- Background: Pure black (#000000)
- Text: Orange tones (#ffa500, #ff8500, #ffcc66)
- Highlights: Various orange shades with black text in selected areas
- Borders: Orange for clear visibility
- Metadata: Grey tones for subtle information

**klu2** - Dark Blue & White  
- Background: Dark blue (#001a33)
- Text: White
- Highlights: Blue shades (#0066cc, #3399ff)
- Borders: White for crisp separation

### Enabling Progress Bar Effects

Add to your `app.toml`:

```toml
enable_effects = true
progress_bar_effect = "Shimmer"  # Options: None, Shimmer, Pulse, Rainbow, Wave
```

## 🎨 Screenshots

*(Coming soon!)*

## 🔧 Building from Source

### Prerequisites
- Rust and Cargo
- Same dependencies as original spotify_player
- For effects: TachyonFX will be automatically installed

### Build Commands

```bash
# Clone the repository
git clone [your-fork-url] spotify-player-klu
cd spotify-player-klu

# Build with all features
cargo build --release --features fx

# Build without effects
cargo build --release --no-default-features --features rodio-backend,media-control

# Run locally
cargo run --features fx
```

## 🐛 Known Issues

- TachyonFX effects are experimental and may not work perfectly in all terminals
- Performance may vary depending on terminal emulator
- Some effects may look different in different color schemes

## 💡 Tips

1. **Terminal Compatibility**: Effects work best with modern terminal emulators (iTerm2, Alacritty, Kitty)
2. **Performance**: If you experience lag, disable effects with `enable_effects = false`
3. **Theme Switching**: You can switch themes on-the-fly using `--theme` flag

## 🙏 Credits

- Original [spotify_player](https://github.com/aome510/spotify-player) by @aome510
- [TachyonFX](https://github.com/junkdog/tachyonfx) for animation effects
- [Ratatui](https://github.com/ratatui/ratatui) for TUI framework

## 📝 License

MIT License - Same as the original spotify_player

## 🤝 Contributing

This is a personal fork, but feel free to:
- Report issues specific to the custom features
- Suggest new themes or effects
- Share your custom themes!

## 📫 Contact

Created by kluless for a more visually engaging Spotify terminal experience! 🎨🎵
