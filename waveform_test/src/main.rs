use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

/// Color schemes for visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ColorScheme {
    Cyan,           // Default cyan/blue
    Warm,           // Orange/red warm colors
    Purple,         // Purple/magenta
    Green,          // Green/emerald
    Sunset,         // Orange to pink gradient
    Ocean,          // Deep blue to cyan
}

struct App {
    current_color_scheme: usize,
    start_time: Instant,
    simulated_bpm: f64, // Simulated BPM for testing
}

impl App {
    fn new() -> Self {
        Self {
            current_color_scheme: 0,
            start_time: Instant::now(),
            simulated_bpm: 120.0, // Default BPM
        }
    }

    fn next_color(&mut self) {
        self.current_color_scheme = (self.current_color_scheme + 1) % 6;
    }

    fn prev_color(&mut self) {
        if self.current_color_scheme == 0 {
            self.current_color_scheme = 5;
        } else {
            self.current_color_scheme -= 1;
        }
    }

    fn increase_bpm(&mut self) {
        self.simulated_bpm = (self.simulated_bpm + 10.0).min(200.0);
    }

    fn decrease_bpm(&mut self) {
        self.simulated_bpm = (self.simulated_bpm - 10.0).max(60.0);
    }

    fn color_scheme(&self) -> ColorScheme {
        match self.current_color_scheme {
            0 => ColorScheme::Cyan,
            1 => ColorScheme::Warm,
            2 => ColorScheme::Purple,
            3 => ColorScheme::Green,
            4 => ColorScheme::Sunset,
            _ => ColorScheme::Ocean,
        }
    }

    fn color_scheme_name(&self) -> &str {
        match self.color_scheme() {
            ColorScheme::Cyan => "Cyan (Default)",
            ColorScheme::Warm => "Warm (Orange/Red)",
            ColorScheme::Purple => "Purple/Magenta",
            ColorScheme::Green => "Green/Emerald",
            ColorScheme::Sunset => "Sunset Gradient",
            ColorScheme::Ocean => "Ocean (Deep Blue)",
        }
    }
}

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(50); // 20 FPS for smooth animation
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right | KeyCode::Char('n') => app.next_color(),
                    KeyCode::Left | KeyCode::Char('p') => app.prev_color(),
                    KeyCode::Up => app.increase_bpm(),
                    KeyCode::Down => app.decrease_bpm(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Title
            Constraint::Length(40),  // Visualization area
            Constraint::Length(5),   // Controls
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Concentric Waves - BPM Reactive", Style::default().fg(Color::Cyan)),
            Span::raw(" | "),
            Span::styled(app.color_scheme_name(), Style::default().fg(Color::Yellow)),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Visualization"));
    f.render_widget(title, chunks[0]);

    // Visualization
    render_concentric_waves(f, app, chunks[1]);

    // Controls
    let controls = Paragraph::new(vec![
        Line::from("Audio-reactive expanding rings synchronized to music tempo (40 lines)"),
        Line::from(""),
        Line::from(vec![
            Span::raw("Controls: "),
            Span::styled("←/→ or p/n", Style::default().fg(Color::Green)),
            Span::raw(" = Change colors | "),
            Span::styled("↑/↓", Style::default().fg(Color::Green)),
            Span::raw(format!(" = BPM ({:.0}) | ", app.simulated_bpm)),
            Span::styled("q", Style::default().fg(Color::Red)),
            Span::raw(" = Quit"),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Info"));
    f.render_widget(controls, chunks[2]);
}


fn render_concentric_waves(f: &mut Frame, app: &App, area: Rect) {
    let elapsed = app.start_time.elapsed().as_secs_f64();
    let beat_progress = (elapsed * app.simulated_bpm / 60.0) % 1.0;
    
    let center_x = area.width as f64 / 2.0;
    let center_y = area.height as f64 / 2.0;
    let wave_offset = beat_progress * 15.0; // Faster wave expansion

    let mut text = vec![];
    for y in 0..area.height {
        let mut line_spans = vec![];
        for x in 0..area.width {
            let dx = x as f64 - center_x;
            let dy = (y as f64 - center_y) * 2.0;
            let dist = (dx * dx + dy * dy).sqrt();

            let wave_phase = (dist - wave_offset) % 5.0;
            let intensity = if wave_phase < 1.0 {
                1.0 - wave_phase
            } else {
                0.0
            };

            // Get color based on scheme and intensity
            let (char, color) = if intensity > 0.7 {
                ('●', get_color_for_scheme(app.color_scheme(), intensity, 0))
            } else if intensity > 0.5 {
                ('◉', get_color_for_scheme(app.color_scheme(), intensity, 1))
            } else if intensity > 0.3 {
                ('○', get_color_for_scheme(app.color_scheme(), intensity, 2))
            } else if intensity > 0.15 {
                ('·', get_color_for_scheme(app.color_scheme(), intensity, 3))
            } else {
                (' ', Color::Black)
            };

            line_spans.push(Span::styled(char.to_string(), Style::default().fg(color)));
        }
        text.push(Line::from(line_spans));
    }

    let widget = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Concentric Waves"));
    f.render_widget(widget, area);
}

fn get_color_for_scheme(scheme: ColorScheme, intensity: f64, level: u8) -> Color {
    match scheme {
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
