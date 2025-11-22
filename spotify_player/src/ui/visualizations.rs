//! Audio visualization module - Psychedelic Waves & Pixel Scatter
//!
//! Renders slow BPM-reactive visualization with multi-color gradients from album art
//! and periodic pixelated album cover scatter/reassemble effect

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Color palette extracted from album art
#[derive(Debug, Clone)]
pub struct AlbumPalette {
    pub colors: Vec<(u8, u8, u8)>,
}

impl AlbumPalette {
    /// Create a default palette with cyan tones
    pub fn default() -> Self {
        Self {
            colors: vec![
                (0, 255, 255),
                (0, 200, 255),
                (50, 150, 255),
                (100, 100, 255),
            ],
        }
    }
}

/// Color schemes for visualization when no album art is available
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorScheme {
    Cyan,    // Default cyan/blue
    Warm,    // Orange/red warm colors
    Purple,  // Purple/magenta
    Green,   // Green/emerald
    Sunset,  // Orange to pink gradient
    Ocean,   // Deep blue to cyan
    Custom,  // From album art
}

/// Render simple horizontal sine wave with album colors
///
/// # Parameters
/// - `frame`: The ratatui frame to render into
/// - `area`: The rectangular area to render the visualization
/// - `beat_progress`: Progress through current beat (0.0-1.0)
/// - `album_color`: Optional RGB color extracted from album art
/// - `show_border`: Whether to show a border around the visualization
pub fn render_concentric_waves(
    frame: &mut Frame,
    area: Rect,
    beat_progress: f64,
    album_color: Option<(u8, u8, u8)>,
    show_border: bool,
) {
    // Use album color only if available (when image feature works), otherwise use theme contrast
    let wave_color = if let Some((r, g, b)) = album_color {
        (r, g, b)
    } else {
        // Default to bright cyan for good contrast with dark themes
        (0, 255, 255)
    };
    
    // Very slow wave animation - tempo between 0.01 and 0.1
    let wave_phase = beat_progress * 0.05; // Ultra slow movement for smooth left-to-right flow
    let center_y = area.height as f64 / 2.0;
    // Flatter wave - reduced amplitude from 1/3 to 1/5 of height
    let amplitude = (area.height as f64 / 5.0).max(1.5);
    
    // Pre-calculate the wave y-position for each x coordinate
    let mut wave_positions: Vec<usize> = Vec::with_capacity(area.width as usize);
    for x in 0..area.width {
        let x_pos = x as f64;
        // Calculate sine wave - moves left to right as wave_phase increases
        let wave_y = center_y + (x_pos * 0.2 + wave_phase).sin() * amplitude;
        wave_positions.push(wave_y.round() as usize);
    }

    // Build the visualization by drawing the continuous wave line
    let mut text = vec![];
    for y in 0..area.height {
        let mut line_spans = vec![];
        for x in 0..area.width {
            // Check if this position is on the wave line
            let wave_y = wave_positions[x as usize];
            
            // Use smooth line characters - horizontal line segments and block characters
            let (char, color) = if y as usize == wave_y {
                // Use full block for smooth continuous line
                ('█', Color::Rgb(wave_color.0, wave_color.1, wave_color.2))
            } else {
                (' ', Color::Black)
            };

            line_spans.push(Span::styled(char.to_string(), Style::default().fg(color)));
        }
        text.push(Line::from(line_spans));
    }

    let widget = if show_border {
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Visualization"))
    } else {
        Paragraph::new(text)
    };
    
    frame.render_widget(widget, area);
}

/// Generate a vibrant color palette from a base album color
fn generate_color_palette(r: u8, g: u8, b: u8) -> AlbumPalette {
    let mut colors = vec![];
    
    // Base color
    colors.push((r, g, b));
    
    // Complementary color (opposite on color wheel)
    colors.push((
        255u8.saturating_sub(r),
        255u8.saturating_sub(g),
        255u8.saturating_sub(b),
    ));
    
    // Analogous colors (rotate hue)
    colors.push((
        ((r as f64 * 0.8 + g as f64 * 0.2) as u8).min(255),
        ((g as f64 * 0.8 + b as f64 * 0.2) as u8).min(255),
        ((b as f64 * 0.8 + r as f64 * 0.2) as u8).min(255),
    ));
    
    colors.push((
        ((r as f64 * 0.2 + b as f64 * 0.8) as u8).min(255),
        ((g as f64 * 0.2 + r as f64 * 0.8) as u8).min(255),
        ((b as f64 * 0.2 + g as f64 * 0.8) as u8).min(255),
    ));
    
    // Lighter variant
    colors.push((
        r.saturating_add(60),
        g.saturating_add(60),
        b.saturating_add(60),
    ));
    
    // Darker variant
    colors.push((
        r.saturating_sub(60),
        g.saturating_sub(60),
        b.saturating_sub(60),
    ));
    
    AlbumPalette { colors }
}

/// Apply intensity to a color
fn apply_intensity(color: (u8, u8, u8), intensity: f64) -> Color {
    Color::Rgb(
        (color.0 as f64 * intensity) as u8,
        (color.1 as f64 * intensity) as u8,
        (color.2 as f64 * intensity) as u8,
    )
}

/// Get color for a given scheme, intensity level, and optional album color
fn get_color_for_scheme(
    scheme: ColorScheme,
    intensity: f64,
    level: u8,
    album_color: Option<(u8, u8, u8)>,
) -> Color {
    match scheme {
        ColorScheme::Custom => {
            if let Some((r, g, b)) = album_color {
                // Generate gradient from album color
                let base = match level {
                    0 => (r, g, b),
                    1 => (
                        (r as f64 * 0.85) as u8,
                        (g as f64 * 0.85) as u8,
                        (b as f64 * 0.85) as u8,
                    ),
                    2 => (
                        (r as f64 * 0.7) as u8,
                        (g as f64 * 0.7) as u8,
                        (b as f64 * 0.7) as u8,
                    ),
                    _ => (
                        (r as f64 * 0.5) as u8,
                        (g as f64 * 0.5) as u8,
                        (b as f64 * 0.5) as u8,
                    ),
                };
                Color::Rgb(
                    (base.0 as f64 * intensity) as u8,
                    (base.1 as f64 * intensity) as u8,
                    (base.2 as f64 * intensity) as u8,
                )
            } else {
                // Fallback to cyan if custom but no color provided
                get_color_for_scheme(ColorScheme::Cyan, intensity, level, None)
            }
        }
        ColorScheme::Cyan => {
            let base = match level {
                0 => (0, 255, 255),
                1 => (0, 200, 255),
                2 => (0, 150, 200),
                _ => (0, 100, 150),
            };
            Color::Rgb(
                (base.0 as f64 * intensity) as u8,
                (base.1 as f64 * intensity) as u8,
                (base.2 as f64 * intensity) as u8,
            )
        }
        ColorScheme::Warm => {
            let base = match level {
                0 => (255, 100, 0),
                1 => (255, 150, 50),
                2 => (200, 100, 0),
                _ => (150, 70, 0),
            };
            Color::Rgb(
                (base.0 as f64 * intensity) as u8,
                (base.1 as f64 * intensity) as u8,
                (base.2 as f64 * intensity) as u8,
            )
        }
        ColorScheme::Purple => {
            let base = match level {
                0 => (200, 50, 255),
                1 => (180, 80, 230),
                2 => (150, 50, 200),
                _ => (100, 30, 150),
            };
            Color::Rgb(
                (base.0 as f64 * intensity) as u8,
                (base.1 as f64 * intensity) as u8,
                (base.2 as f64 * intensity) as u8,
            )
        }
        ColorScheme::Green => {
            let base = match level {
                0 => (50, 255, 150),
                1 => (50, 220, 120),
                2 => (30, 180, 100),
                _ => (20, 120, 70),
            };
            Color::Rgb(
                (base.0 as f64 * intensity) as u8,
                (base.1 as f64 * intensity) as u8,
                (base.2 as f64 * intensity) as u8,
            )
        }
        ColorScheme::Sunset => {
            let base = match level {
                0 => (255, 100, 150),
                1 => (255, 150, 100),
                2 => (200, 100, 100),
                _ => (150, 70, 80),
            };
            Color::Rgb(
                (base.0 as f64 * intensity) as u8,
                (base.1 as f64 * intensity) as u8,
                (base.2 as f64 * intensity) as u8,
            )
        }
        ColorScheme::Ocean => {
            let base = match level {
                0 => (0, 150, 255),
                1 => (20, 120, 220),
                2 => (10, 80, 180),
                _ => (5, 50, 120),
            };
            Color::Rgb(
                (base.0 as f64 * intensity) as u8,
                (base.1 as f64 * intensity) as u8,
                (base.2 as f64 * intensity) as u8,
            )
        }
    }
}
