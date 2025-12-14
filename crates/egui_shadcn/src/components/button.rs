//! Button component matching shadcn/ui exactly
//!
//! Provides all shadcn button variants and sizes.
//!
//! Reference: <https://ui.shadcn.com/docs/components/button>

use egui::{Response, Sense, Ui, Widget};
use crate::theme::ShadcnTheme;

/// Button size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    /// Small button (compact padding)
    Small,
    /// Default size button
    Default,
    /// Large button (increased padding)
    Large,
    /// Square icon button
    Icon,
}

/// Button style variants matching shadcn/ui
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Primary/default button - solid background
    Default,
    /// Secondary button - muted solid background
    Secondary,
    /// Outline button - transparent with border
    Outline,
    /// Ghost button - transparent, minimal
    Ghost,
    /// Destructive button - red/danger themed
    Destructive,
    /// Link button - styled as hyperlink
    Link,
}

/// shadcn/ui Button component
///
/// A fully-styled button matching shadcn's design exactly.
///
/// ## Example
/// ```rust,ignore
/// if Button::new("Click me")
///     .variant(ButtonVariant::Default)
///     .size(ButtonSize::Default)
///     .ui(ui)
///     .clicked()
/// {
///     // Handle click
/// }
/// ```
pub struct Button {
    text: String,
    variant: ButtonVariant,
    size: ButtonSize,
    enabled: bool,
}

impl Button {
    /// Create a new button with the given text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            enabled: true,
        }
    }

    /// Set the button variant
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the button size
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Set whether the button is enabled
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
            ButtonSize::Small => egui::vec2(theme.spacing.md, 16.0), // 16x16 → ~44px height
            ButtonSize::Default => egui::vec2(theme.spacing.lg, 15.0), // 24x15 → ~44px height
            ButtonSize::Large => egui::vec2(theme.spacing.xl, theme.spacing.md), // 32x16 → ~50px height
            ButtonSize::Icon => egui::vec2(14.0, 14.0), // 14x14 padding → 44px for ~16px icon
        }
    }

    /// Get font size for the current size
    fn font_size(&self, theme: &ShadcnTheme) -> f32 {
        match self.size {
            ButtonSize::Small => theme.typography.small().size,
            ButtonSize::Default => theme.typography.body().size,
            ButtonSize::Large => theme.typography.large().size,
            ButtonSize::Icon => theme.typography.body().size,
        }
    }

    /// Get colors for the current variant
    fn colors(&self, theme: &ShadcnTheme, hovered: bool, pressed: bool) -> (egui::Color32, egui::Color32, Option<egui::Stroke>) {
        use ButtonVariant::*;

        match self.variant {
            Default => {
                let bg = if pressed {
                    Self::darken(theme.colors.primary, 0.2)
                } else if hovered {
                    Self::darken(theme.colors.primary, 0.1)
                } else {
                    theme.colors.primary
                };
                (bg, theme.colors.primary_foreground, None)
            }
            Secondary => {
                let bg = if pressed {
                    Self::darken(theme.colors.secondary, 0.1)
                } else if hovered {
                    Self::darken(theme.colors.secondary, 0.05)
                } else {
                    theme.colors.secondary
                };
                (bg, theme.colors.secondary_foreground, None)
            }
            Outline => {
                let bg = if pressed {
                    theme.colors.accent
                } else if hovered {
                    theme.colors.secondary
                } else {
                    egui::Color32::TRANSPARENT
                };
                let border = egui::Stroke::new(1.0, theme.colors.border);
                (bg, theme.colors.foreground, Some(border))
            }
            Ghost => {
                let bg = if pressed {
                    theme.colors.accent
                } else if hovered {
                    theme.colors.secondary
                } else {
                    egui::Color32::TRANSPARENT
                };
                (bg, theme.colors.foreground, None)
            }
            Destructive => {
                let bg = if pressed {
                    Self::darken(theme.colors.destructive, 0.2)
                } else if hovered {
                    Self::darken(theme.colors.destructive, 0.1)
                } else {
                    theme.colors.destructive
                };
                (bg, theme.colors.destructive_foreground, None)
            }
            Link => {
                let fg = if hovered {
                    Self::darken(theme.colors.primary, 0.1)
                } else {
                    theme.colors.primary
                };
                (egui::Color32::TRANSPARENT, fg, None)
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

impl Widget for Button {
    fn ui(self, ui: &mut Ui) -> Response {
        // Get theme from context or fall back to light mode
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let padding = self.padding(&theme);
        let font_size = self.font_size(&theme);
        let corner_radius = theme.radii.button();

        // Measure text first to determine button size
        let text_galley = ui.painter().layout_no_wrap(
            self.text.clone(),
            egui::FontId::proportional(font_size),
            egui::Color32::WHITE, // Color doesn't matter for sizing
        );
        let text_size = text_galley.size();

        // Calculate button size (text + padding on both sides)
        let button_size = egui::vec2(
            text_size.x + padding.x * 2.0,
            (text_size.y + padding.y * 2.0).max(44.0), // Apple HIG minimum
        );

        // Allocate the button area
        let (button_rect, response) = ui.allocate_exact_size(
            button_size,
            if self.enabled { Sense::click() } else { Sense::hover() },
        );

        if ui.is_rect_visible(button_rect) {
            let hovered = response.hovered() && self.enabled;
            let pressed = response.is_pointer_button_down_on() && self.enabled;

            // Get colors based on state
            let (mut bg_color, mut text_color, border) = self.colors(&theme, hovered, pressed);

            // Apply disabled state styling (40% opacity for more noticeable muting)
            if !self.enabled {
                bg_color = bg_color.linear_multiply(0.4);
                text_color = text_color.linear_multiply(0.4);
            }

            // Draw background
            ui.painter().rect_filled(button_rect, corner_radius, bg_color);

            // Draw border if present
            if let Some(mut stroke) = border {
                if !self.enabled {
                    stroke = egui::Stroke::new(stroke.width, stroke.color.linear_multiply(0.4));
                }
                ui.painter().rect_stroke(
                    button_rect,
                    corner_radius,
                    stroke,
                    egui::StrokeKind::Inside,
                );
            }

            // Draw text centered in button
            let text_galley = ui.painter().layout_no_wrap(
                self.text.clone(),
                egui::FontId::proportional(font_size),
                text_color,
            );
            let text_pos = button_rect.center() - text_galley.size() / 2.0;
            ui.painter().galley(text_pos, text_galley, text_color);

            // Link underline (below the text, not at bottom of button)
            if self.variant == ButtonVariant::Link && hovered {
                let text_bottom = text_pos.y + text_size.y + 1.0;
                ui.painter().line_segment(
                    [
                        egui::pos2(text_pos.x, text_bottom),
                        egui::pos2(text_pos.x + text_size.x, text_bottom),
                    ],
                    egui::Stroke::new(1.0, text_color),
                );
            }
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
    fn test_button_creation() {
        let button = Button::new("Test");
        assert_eq!(button.text, "Test");
        assert_eq!(button.variant, ButtonVariant::Default);
        assert_eq!(button.size, ButtonSize::Default);
        assert!(button.enabled);
    }

    #[test]
    fn test_button_builder() {
        let button = Button::new("Test")
            .variant(ButtonVariant::Destructive)
            .size(ButtonSize::Large)
            .enabled(false);

        assert_eq!(button.variant, ButtonVariant::Destructive);
        assert_eq!(button.size, ButtonSize::Large);
        assert!(!button.enabled);
    }
}
