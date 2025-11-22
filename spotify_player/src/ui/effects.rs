//! UI effects and animations

#[cfg(feature = "fx")]
#[derive(Debug)]
pub struct EffectsState {
    pub last_update: std::time::Instant,
}

#[cfg(feature = "fx")]
impl Default for EffectsState {
    fn default() -> Self {
        Self {
            last_update: std::time::Instant::now(),
        }
    }
}

#[cfg(feature = "fx")]
impl EffectsState {
    /// Get elapsed time since last update
    pub fn elapsed(&self) -> std::time::Duration {
        std::time::Instant::now().duration_since(self.last_update)
    }
}

// Stub implementations when fx feature is disabled
#[cfg(not(feature = "fx"))]
pub struct EffectsState;

#[cfg(not(feature = "fx"))]
impl Default for EffectsState {
    fn default() -> Self {
        Self
    }
}

#[cfg(not(feature = "fx"))]
impl EffectsState {
    pub fn update(&mut self) {}
    pub fn clear(&mut self) {}
}

#[cfg(not(feature = "fx"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressBarEffect {
    None,
}

#[cfg(not(feature = "fx"))]
impl Default for ProgressBarEffect {
    fn default() -> Self {
        Self::None
    }
}
