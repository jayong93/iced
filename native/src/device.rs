//! Test

use crate::{keyboard, mouse};

/// A raw device event
#[derive(PartialEq, Debug, Clone)]
pub enum Event {
    /// Mouse movement
    MouseMotion(f32, f32),
    /// Mouse wheel scrolled
    MouseWheel(mouse::ScrollDelta),
    /// Some button pressed
    ButtonPressed(u32),
    /// Some button released
    ButtonReleased(u32),
    /// Some keyboard key pressed or released
    KeyInput(keyboard::Event),
}
