//! Animation utilities for smooth UI transitions
//!
//! Provides spring-based animation helpers for components like Sheet and Drawer.

use egui::{Context, Id};

/// Animation state for slide-in/out components
#[derive(Clone, Copy, Debug)]
pub struct SlideAnimation {
    /// Current offset (0.0 = fully visible, 1.0 = fully hidden)
    pub offset: f32,
    /// Whether we're animating towards open (true) or closed (false)
    pub opening: bool,
    /// Whether animation is currently in progress
    pub animating: bool,
}

impl Default for SlideAnimation {
    fn default() -> Self {
        Self {
            offset: 1.0,  // Start fully closed/hidden
            opening: false,
            animating: false,
        }
    }
}

impl SlideAnimation {
    /// Load animation state from egui's temporary storage
    pub fn load(ctx: &Context, id: Id) -> Self {
        ctx.data(|d| d.get_temp(id).unwrap_or_default())
    }

    /// Store animation state in egui's temporary storage
    pub fn store(self, ctx: &Context, id: Id) {
        ctx.data_mut(|d| d.insert_temp(id, self));
    }

    /// Start opening animation
    pub fn start_open(&mut self) {
        self.opening = true;
        self.animating = true;
    }

    /// Start closing animation
    pub fn start_close(&mut self) {
        self.opening = false;
        self.animating = true;
    }

    /// Update animation state, returns true if still animating
    pub fn update(&mut self, ctx: &Context) -> bool {
        if !self.animating {
            return false;
        }

        let target = if self.opening { 0.0 } else { 1.0 };

        if let Some(new_offset) = spring_animate(self.offset, target) {
            self.offset = new_offset;
            ctx.request_repaint();
            true
        } else {
            self.offset = target;
            self.animating = false;
            false
        }
    }

    /// Check if the component should be considered "open" (visible or animating open)
    pub fn is_visible(&self) -> bool {
        self.offset < 1.0 || self.opening
    }

    /// Check if fully closed (animation complete and hidden)
    pub fn is_fully_closed(&self) -> bool {
        self.offset >= 1.0 && !self.opening && !self.animating
    }
}

/// Spring-based animation function
///
/// Smoothly animates from `current` towards `target` using a spring-like curve.
/// Returns `Some(new_value)` if animation should continue, `None` if complete.
fn spring_animate(current: f32, target: f32) -> Option<f32> {
    let diff = current - target;
    let abs_diff = diff.abs();

    // Animation complete when very close to target
    if abs_diff < 0.001 {
        return None;
    }

    // Spring dampening - faster when far, slower when close
    let spring_factor = springy(abs_diff);
    let adjustment = spring_factor * diff.signum() * -1.0;

    let new_value = current + adjustment;

    // Prevent overshooting - clamp to target
    if target > current {
        Some(new_value.min(target))
    } else {
        Some(new_value.max(target))
    }
}

/// Calculate spring dampening factor
///
/// Creates an exponential decay effect - moves faster when far from target,
/// slower as it approaches for a natural feel.
fn springy(offset: f32) -> f32 {
    // Larger values = faster animation
    // 0.15 gives a nice smooth feel (not too slow, not too snappy)
    (offset * 0.15).max(0.008)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_animate_towards_zero() {
        let result = spring_animate(1.0, 0.0);
        assert!(result.is_some());
        assert!(result.unwrap() < 1.0);
    }

    #[test]
    fn test_spring_animate_towards_one() {
        let result = spring_animate(0.0, 1.0);
        assert!(result.is_some());
        assert!(result.unwrap() > 0.0);
    }

    #[test]
    fn test_spring_animate_complete() {
        let result = spring_animate(0.0005, 0.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_slide_animation_default() {
        let anim = SlideAnimation::default();
        assert_eq!(anim.offset, 1.0);  // Starts fully closed
        assert!(!anim.opening);
        assert!(!anim.animating);
    }
}
