# Concentric Waves Integration Plan

## Summary
Successfully tested the **Concentric Waves** visualization with 6 color schemes. Now ready to integrate into the main Spotify player with album art color extraction.

## Test Results
The waveform_test program demonstrates:
- ✅ 40-line tall audio-reactive concentric waves
- ✅ BPM-synchronized wave expansion
- ✅ 6 beautiful color schemes (Cyan, Warm, Purple, Green, Sunset, Ocean)
- ✅ Smooth animations at 20 FPS
- ✅ Character gradients (●, ◉, ○, ·) for depth

## Integration Steps

### 1. Remove Fork Customizations
Delete all existing custom additions:
- `klu1` and `klu2` themes from `spotify_player/src/config/theme.rs` (lines 593-743)
- All TachyonFX effects from `spotify_player/src/ui/effects.rs`
- Sine wave progress bar from `spotify_player/src/ui/widgets.rs`
- Animated boxes code from playback rendering

### 2. Add Album Art Color Extraction
The repo already has `image::DynamicImage` support in `state/data.rs`.

**Add new function** in `spotify_player/src/utils/mod.rs`:
```rust
#[cfg(feature = "image")]
pub fn extract_dominant_colors(img: &image::DynamicImage) -> (u8, u8, u8) {
    // Sample pixels from the album art
    // Calculate average or dominant color
    // Return RGB tuple
}
```

### 3. Create Concentric Waves Visualization Widget
**New file**: `spotify_player/src/ui/visualizations.rs`

Copy the concentric waves rendering logic from `waveform_test/src/main.rs`:
- `ColorScheme` enum
- `render_concentric_waves()` function  
- `get_color_for_scheme()` function

**Modify** to accept RGB color from album art:
```rust
pub fn render_concentric_waves(
    f: &mut Frame,
    area: Rect,
    beat_progress: f64,
    base_color: Option<(u8, u8, u8)>, // From album art
)
```

### 4. Add Visualization Area to Layout
**Option A: Replace Album/Artist boxes** (as you suggested)
- Modify library layout to only show Playlists
- Use freed space for 40-line visualization

**Option B: Separate visualization panel**
- Add new layout section below playback
- Configurable height (20-40 lines)

**File to modify**: `spotify_player/src/ui/page.rs` (layout rendering)

### 5. Connect to Audio Data
Currently simulating BPM. For real audio-reactivity:

**Use Spotify Track Audio Features**:
```rust
// In spotify_player/src/client/mod.rs
// Spotify API provides tempo/BPM per track
let audio_features = client.track_audio_features(track_id).await?;
let bpm = audio_features.tempo;
```

Store current track BPM in `PlayerState` and use for wave timing.

### 6. Configuration
Add to `spotify_player/src/config/mod.rs`:
```toml
[visualization]
enable = true
type = "concentric_waves"  # or "none"
height = 40
use_album_colors = true
fallback_color_scheme = "Cyan"  # When no album art
```

### 7. Update UI State
In `spotify_player/src/state/ui/mod.rs`:
```rust
pub struct UIState {
    // ... existing fields ...
    
    #[cfg(feature = "fx")]
    pub current_album_color: Option<(u8, u8, u8)>,
    pub visualization_start_time: std::time::Instant,
}
```

## Files to Create/Modify

### Create:
1. `spotify_player/src/ui/visualizations.rs` - Main visualization rendering
2. `INTEGRATION_PLAN.md` - This file ✅

### Modify:
1. `spotify_player/src/config/mod.rs` - Add visualization config
2. `spotify_player/src/state/ui/mod.rs` - Store visualization state
3. `spotify_player/src/ui/mod.rs` - Add visualization module
4. `spotify_player/src/ui/page.rs` - Update layout to include visualization
5. `spotify_player/src/utils/mod.rs` - Add color extraction function
6. `spotify_player/src/client/mod.rs` - Fetch track audio features (BPM)

### Delete:
1. Custom theme code (klu1, klu2) from `theme.rs`
2. Old effects code from `effects.rs`
3. Sine wave widget from `widgets.rs`
4. Animated boxes code from `playback.rs`

## Next Steps

1. **Clean up fork** - Remove all custom additions
2. **Implement color extraction** - Extract colors from album art
3. **Add visualization widget** - Port concentric waves code
4. **Update layout** - Add visualization area (replace album/artist boxes or separate panel)
5. **Connect BPM** - Use Spotify audio features API
6. **Test with real music** - Run with actual Spotify playback

## Album Art Color Extraction Details

The app already loads album art as `image::DynamicImage` in:
- `spotify_player/src/state/data.rs` line 55
- Cached in `MemoryCaches.images`

To extract colors:
1. Get current track's album art from cache
2. Sample pixels (e.g., every 10th pixel)
3. Convert to RGB, calculate dominant color
4. Map RGB to one of our 6 color schemes OR use directly
5. Update on track change

## Animation Boxes for Later

You mentioned wanting separate ASCII animation boxes (20x30 lines) like album/artist boxes.

**Ideas for simple animations**:
- Rotating ASCII art (spinning record, cassette tape)
- Audio equalizer bars
- Bouncing Spotify logo
- Scrolling lyrics
- Particle systems

We can implement these after the main visualization is working.

## Test the Integration

After integration:
1. Play a song with colorful album art → waves should match colors
2. Change tracks → colors should update
3. Adjust BPM → wave speed should change
4. Test with/without album art → fallback colors work
5. Verify performance (should stay ~32ms refresh)

---

**Status**: Test program complete ✅ | Ready for integration 🚀
