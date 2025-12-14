//! Toggle component matching shadcn/ui exactly
//!
//! A two-state button that can be either on or off.
//!
//! Reference: <https://ui.shadcn.com/docs/components/toggle>

use egui::{Response, Sense, Ui, Widget};
use crate::theme::ShadcnTheme;

/// Toggle size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleSize {
    /// Small toggle (compact)
    Small,
    /// Default size toggle
    Default,
    /// Large toggle
    Large,
}

/// Toggle style variants matching shadcn/ui
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleVariant {
    /// Default toggle - solid background when pressed
    Default,
    /// Outline toggle - border-based with transparent background
    Outline,
}

/// shadcn/ui Toggle component
///
/// A two-state button for on/off toggles (like toolbar buttons).
///
/// ## Example
/// ```rust,ignore
/// let mut pressed = false;
/// if Toggle::new(&mut pressed, "Bold")
///     .variant(ToggleVariant::Default)
///     .size(ToggleSize::Default)
///     .ui(ui)
///     .changed()
/// {
///     // Handle toggle change
/// }
/// ```
pub struct Toggle<'a> {
    pressed: &'a mut bool,
    text: String,
    variant: ToggleVariant,
    size: ToggleSize,
    enabled: bool,
}

impl<'a> Toggle<'a> {
    /// Create a new toggle bound to a boolean value
    pub fn new(pressed: &'a mut bool, text: impl Into<String>) -> Self {
        Self {
            pressed,
            text: text.into(),
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            enabled: true,
        }
    }

    /// Set the toggle variant
    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the toggle size
    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    /// Set whether the toggle is enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Get padding for the current size
    /// Vertical padding is calculated to ensure 44px minimum height (Apple HIG touch target)
    fn padding(&self, theme: &ShadcnTheme) -> egui::Vec2 {
        // Base font heights approximately: Small ~12px, Default ~14px, Large ~18px
        // To achieve 44px minimum: (44 - font_height) / 2 for vertical padding
        match self.size {
            ToggleSize::Small => egui::vec2(theme.spacing.sm, 16.0), // 8x16 → ~44px height
            ToggleSize::Default => egui::vec2(theme.spacing.md, 15.0), // 16x15 → ~44px height
            ToggleSize::Large => egui::vec2(theme.spacing.lg, 13.0), // 24x13 → ~44px height
        }
    }

    /// Get font size for the current size
    fn font_size(&self, theme: &ShadcnTheme) -> f32 {
        match self.size {
            ToggleSize::Small => theme.typography.small().size,
            ToggleSize::Default => theme.typography.body().size,
            ToggleSize::Large => theme.typography.large().size,
        }
    }

    /// Get colors for the current variant and state
    fn colors(&self, theme: &ShadcnTheme, hovered: bool) -> (egui::Color32, egui::Color32, egui::Stroke) {
        use ToggleVariant::*;

        // All toggles get a visible border for consistency
        // Use muted_foreground for better contrast in both light and dark modes
        let border_color = theme.colors.muted_foreground;

        match self.variant {
            Default => {
                if *self.pressed {
                    // Pressed state: solid accent background
                    let bg = if hovered {
                        Self::darken(theme.colors.accent, 0.05)
                    } else {
                        theme.colors.accent
                    };
                    let border = egui::Stroke::new(1.0, theme.colors.accent);
                    (bg, theme.colors.accent_foreground, border)
                } else {
                    // Unpressed state: transparent with hover effect
                    let bg = if hovered {
                        theme.colors.muted
                    } else {
                        egui::Color32::TRANSPARENT
                    };
                    let border = egui::Stroke::new(1.0, border_color);
                    (bg, theme.colors.foreground, border)
                }
            }
            Outline => {
                if *self.pressed {
                    // Pressed state: solid accent background with border
                    let bg = if hovered {
                        Self::darken(theme.colors.accent, 0.05)
                    } else {
                        theme.colors.accent
                    };
                    let border = egui::Stroke::new(1.0, theme.colors.accent);
                    (bg, theme.colors.accent_foreground, border)
                } else {
                    // Unpressed state: transparent with visible border
                    let bg = if hovered {
                        theme.colors.muted
                    } else {
                        egui::Color32::TRANSPARENT
                    };
                    let border = egui::Stroke::new(1.0, border_color);
                    (bg, theme.colors.foreground, border)
                }
            }
        }
    }

    /// Darken a color
    fn darken(color: egui::Color32, factor: f32) -> egui::Color32 {
        let [r, g, b, a] = color.to_array();
        let r = (r as f32 * (1.0 - factor)).max(0.0) as u8;
        let g = (g as f32 * (1.0 - factor)).max(0.0) as u8;
        let b = (b as f32 * (1.0 - factor)).max(0.0) as u8;
        egui::Color32::from_rgba_premultiplied(r, g, b, a)
    }
}

impl<'a> Widget for Toggle<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let padding = self.padding(&theme);
        let font_size = self.font_size(&theme);
        let corner_radius = theme.radii.md;

        // Measure text first to determine toggle size
        let text_galley = ui.painter().layout_no_wrap(
            self.text.clone(),
            egui::FontId::proportional(font_size),
            egui::Color32::WHITE, // Color doesn't matter for sizing
        );
        let text_size = text_galley.size();

        // Calculate toggle size (text + padding on both sides)
        let toggle_size = egui::vec2(
            text_size.x + padding.x * 2.0,
            (text_size.y + padding.y * 2.0).max(44.0), // Apple HIG minimum
        );

        // Allocate the toggle area
        let (toggle_rect, response) = ui.allocate_exact_size(
            toggle_size,
            if self.enabled { Sense::click() } else { Sense::hover() },
        );

        // Handle click
        if response.clicked() && self.enabled {
            *self.pressed = !*self.pressed;
        }

        if ui.is_rect_visible(toggle_rect) {
            let hovered = response.hovered() && self.enabled;

            // Get colors based on state
            let (mut bg_color, mut text_color, mut border) = self.colors(&theme, hovered);

            // Apply disabled state styling (50% opacity)
            if !self.enabled {
                bg_color = bg_color.linear_multiply(0.5);
                text_color = text_color.linear_multiply(0.5);
                border = egui::Stroke::new(border.width, border.color.linear_multiply(0.5));
            }

            // Draw background
            ui.painter().rect_filled(toggle_rect, corner_radius, bg_color);

            // Draw border
            ui.painter().rect_stroke(
                toggle_rect,
                corner_radius,
                border,
                egui::StrokeKind::Inside,
            );

            // Draw text centered in toggle
            let text_galley = ui.painter().layout_no_wrap(
                self.text.clone(),
                egui::FontId::proportional(font_size),
                text_color,
            );
            let text_pos = toggle_rect.center() - text_galley.size() / 2.0;
            ui.painter().galley(text_pos, text_galley, text_color);
        }

        if !self.enabled {
            response.on_disabled_hover_text("Disabled")
        } else {
            response
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_creation() {
        let mut pressed = false;
        let toggle = Toggle::new(&mut pressed, "Bold");
        assert!(!*toggle.pressed);
        assert_eq!(toggle.text, "Bold");
        assert_eq!(toggle.variant, ToggleVariant::Default);
        assert_eq!(toggle.size, ToggleSize::Default);
        assert!(toggle.enabled);
    }

    #[test]
    fn test_toggle_builder() {
        let mut pressed = true;
        let toggle = Toggle::new(&mut pressed, "Italic")
            .variant(ToggleVariant::Outline)
            .size(ToggleSize::Large)
            .enabled(false);

        assert!(* toggle.pressed);
        assert_eq!(toggle.variant, ToggleVariant::Outline);
        assert_eq!(toggle.size, ToggleSize::Large);
        assert!(!toggle.enabled);
    }
}
