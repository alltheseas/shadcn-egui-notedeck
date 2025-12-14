//! Badge component ported from shadcn/ui
//!
//! A lightweight component for displaying labels, status indicators,
//! tags, and notification counts.
//!
//! Reference: <https://ui.shadcn.com/docs/components/badge>
//!
//! ## Features
//! - Multiple variants: default, secondary, destructive, outline
//! - Pill-shaped design with full corner radius
//! - Compact sizing optimized for inline use
//! - Semantic color mapping from theme
//!
//! ## Usage
//! ```rust,ignore
//! use egui_shadcn::components::Badge;
//!
//! ui.add(Badge::new("New").variant(BadgeVariant::Default));
//! ui.add(Badge::new("3").variant(BadgeVariant::Secondary));
//! ui.add(Badge::new("Error").variant(BadgeVariant::Destructive));
//! ```

use egui::{Response, RichText, Ui, Widget};

use crate::theme::ShadcnTheme;

/// Visual style variants for Badge component
///
/// Maps to shadcn/ui badge variants with appropriate color mappings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeVariant {
    /// Default filled badge with primary styling
    Default,
    /// Secondary subtle badge with muted colors
    Secondary,
    /// Destructive/error badge with warning colors
    Destructive,
    /// Outline-only badge with transparent background
    Outline,
}

/// Badge component for displaying labels and status indicators
///
/// A compact, pill-shaped element that displays short text content.
/// Commonly used for tags, counts, status, and categorization.
///
/// ## Example
/// ```rust,ignore
/// use egui_shadcn::components::{Badge, BadgeVariant};
///
/// // Default badge
/// ui.add(Badge::new("New"));
///
/// // Secondary badge
/// ui.add(Badge::new("Beta").variant(BadgeVariant::Secondary));
///
/// // Destructive badge
/// ui.add(Badge::new("Error").variant(BadgeVariant::Destructive));
///
/// // Outline badge
/// ui.add(Badge::new("Draft").variant(BadgeVariant::Outline));
///
/// // Numeric badge (common for notifications)
/// ui.add(Badge::new("99+").variant(BadgeVariant::Destructive));
/// ```
#[derive(Debug, Clone)]
pub struct Badge {
    text: String,
    variant: BadgeVariant,
}

impl Badge {
    /// Create a new badge with the given text
    ///
    /// Default variant is `BadgeVariant::Default`.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: BadgeVariant::Default,
        }
    }

    /// Set the visual variant of the badge
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl Widget for Badge {
    fn ui(self, ui: &mut Ui) -> Response {
        // Get theme from context or fall back to light mode
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        // Badge styling: pill-shaped with compact padding
        let padding = theme.spacing.vec2_xy(3, 1); // 12px horizontal, 4px vertical
        let corner_radius = theme.radii.badge(); // Full pill shape

        // Determine colors based on variant
        let (bg_color, text_color, stroke_color) = match self.variant {
            BadgeVariant::Default => (
                theme.colors.primary,
                theme.colors.primary_foreground,
                None,
            ),
            BadgeVariant::Secondary => (
                theme.colors.secondary,
                theme.colors.secondary_foreground,
                None,
            ),
            BadgeVariant::Destructive => (
                theme.colors.destructive,
                theme.colors.destructive_foreground,
                None,
            ),
            BadgeVariant::Outline => (
                egui::Color32::TRANSPARENT,
                theme.colors.foreground,
                // Use foreground at 40% for visible border
                Some(theme.colors.foreground.linear_multiply(0.4)),
            ),
        };

        // Create the badge frame
        let frame = egui::Frame::NONE
            .fill(bg_color)
            .stroke(if let Some(color) = stroke_color {
                egui::Stroke::new(1.0, color)
            } else {
                egui::Stroke::NONE
            })
            .inner_margin(padding)
            .corner_radius(corner_radius);

        // Render the badge
        frame.show(ui, |ui| {
            // Use small text size for compact display
            let text = RichText::new(&self.text)
                .size(theme.typography.small().size)
                .color(text_color);

            ui.label(text);
        })
        .response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_creation() {
        let badge = Badge::new("Test");
        assert_eq!(badge.text, "Test");
        assert_eq!(badge.variant, BadgeVariant::Default);
    }

    #[test]
    fn test_badge_variants() {
        let badge = Badge::new("Test").variant(BadgeVariant::Secondary);
        assert_eq!(badge.variant, BadgeVariant::Secondary);

        let badge = Badge::new("Error").variant(BadgeVariant::Destructive);
        assert_eq!(badge.variant, BadgeVariant::Destructive);

        let badge = Badge::new("Draft").variant(BadgeVariant::Outline);
        assert_eq!(badge.variant, BadgeVariant::Outline);
    }

    #[test]
    fn test_badge_text_conversion() {
        let badge = Badge::new(String::from("Dynamic"));
        assert_eq!(badge.text, "Dynamic");

        let badge = Badge::new("Static");
        assert_eq!(badge.text, "Static");
    }
}
