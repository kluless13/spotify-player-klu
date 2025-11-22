# Sine Wave Progress Bar & Animated Boxes Features

This document describes the new sine wave progress bar and animated boxes features added to spotify-player.

## Features Added

### 1. Sine Wave Progress Bar
A new progress bar visualization that uses Unicode braille characters to render a smooth sine wave pattern showing playback progress.

### 2. Animated Boxes
Small animated indicators positioned around the progress bar that add visual flair to the UI using braille character animations.

### 3. Shape-Based Progress Bar Effects
Only shape-based effects with HIGH CONTRAST colors:
- **Circles** - Bright cyan and blue concentric circular wave patterns
- **Squares** - Bright orange and dark red square wave with sharp transitions
- **Triangles** - Magenta to green triangular/sawtooth gradients
- **None** - No animation

## Configuration Options

Add these options to your `~/.config/spotify-player/app.toml` file:

### Enable Sine Wave Progress Bar
```toml
# Choose progress bar style: "Classic" or "SineWave"
progress_bar_style = "SineWave"  # Default: SineWave
```

### Enable Animated Boxes
```toml
# Show animated boxes around the progress bar
show_animated_boxes = true  # Default: true
```

### Enable Effects
```toml
# Enable all visual effects
enable_effects = true  # Default: true
```

### Choose Progress Bar Effect
```toml
# Available effects: "None", "Circles", "Squares", "Triangles"
progress_bar_effect = "Circles"  # Default: Circles
```

## Default Configuration

By default, the following settings are enabled:
- Sine wave progress bar style
- Animated boxes visible (5 characters wide, HIGH CONTRAST)
- Effects enabled
- Circles effect (bright cyan/blue)

## Example Configurations

### Minimal Classic Look
```toml
enable_effects = false
progress_bar_style = "Classic"
show_animated_boxes = false
```

### Maximum Visual Effects
```toml
enable_effects = true
progress_bar_style = "SineWave"
show_animated_boxes = true
progress_bar_effect = "Circles"
```

### Simple Sine Wave
```toml
enable_effects = true
progress_bar_style = "SineWave"
show_animated_boxes = false
progress_bar_effect = "None"
```

## Animated Box Types

The application displays two animated boxes by default:
- **Left box**: Spinner animation (rotating braille pattern)
- **Right box**: Pulse animation (expanding/contracting blocks)

Each animated box updates at different rates to create varied visual interest.

## Technical Details

### Unicode Braille Characters
The sine wave uses Unicode braille characters (U+2800-U+28FF) which provide a 2x4 pixel grid per character cell, allowing for higher resolution graphics in the terminal.

### Animation Frame Rates
- **Spinner**: Updates every 80ms (10 frames)
- **Pulse**: Updates every 100ms (8 frames)
- **Dots**: Updates every 150ms (6 frames)

### Progress Bar Effects
Effects are applied based on elapsed time and progress position, creating dynamic color animations that respond to playback state.

## Keyboard Commands

Default keybindings (press keys in sequence):

- **`e t`** - Toggle all visual effects on/off
- **`e e`** - Cycle through progress bar effects (Circles → Squares → Triangles → None → Circles...)
- **`e s`** - Toggle sine wave progress bar (SineWave ↔ Classic)
- **`e b`** - Toggle animated boxes on/off

You can customize these in your `~/.config/spotify-player/keymap.toml` file.

## Terminal Requirements

For best results, use a terminal that supports:
- Unicode braille characters (U+2800-U+28FF)
- True color (24-bit RGB)
- Modern terminal fonts (e.g., FiraCode, JetBrains Mono, Hack)

## Troubleshooting

### Sine wave not rendering properly
- Ensure your terminal font supports Unicode braille characters
- Try switching to a monospace programming font
- Check that your terminal supports Unicode properly

### Animated boxes not visible
- Set `show_animated_boxes = true` in config
- Ensure `enable_effects = true`
- Check that terminal width is sufficient (at least 40 columns)

### Effects not animating
- Verify `enable_effects = true` in config
- Ensure the app refresh rate is set appropriately (`app_refresh_duration_in_ms = 32`)

## Building from Source

The features are enabled by default through the `fx` feature flag:

```bash
# Build with all features (includes fx)
cargo build --release

# Build without effects
cargo build --release --no-default-features --features rodio-backend,media-control
```
