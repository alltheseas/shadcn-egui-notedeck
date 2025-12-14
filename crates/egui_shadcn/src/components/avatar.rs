//! Avatar component ported from shadcn/ui
//!
//! A circular component for displaying user profile images or initials.
//! Shows a fallback (initials or placeholder) when no image is available.
//!
//! Reference: <https://ui.shadcn.com/docs/components/avatar>
//!
//! ## Features
//! - Circular shape with configurable size
//! - Fallback to initials when no image provided
//! - Multiple size variants
//! - Semantic color mapping from theme
//!
//! ## Usage
//! ```rust,ignore
//! use egui_shadcn::components::Avatar;
//!
//! // Display initials
//! ui.add(Avatar::new("John Doe"));
//!
//! // Custom size
//! ui.add(Avatar::new("AB").size(AvatarSize::Large));
//! ```

use egui::{Response, Ui, Vec2, Widget};

use crate::theme::ShadcnTheme;

/// Size variants for Avatar component
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarSize {
    /// Small avatar: 32px
    Small,
    /// Medium avatar: 40px (default)
    Medium,
    /// Large avatar: 48px
    Large,
    /// Extra large avatar: 64px
    ExtraLarge,
}

impl AvatarSize {
    /// Get the pixel size for this variant
    pub const fn pixels(&self) -> f32 {
        match self {
            AvatarSize::Small => 32.0,
            AvatarSize::Medium => 40.0,
            AvatarSize::Large => 48.0,
            AvatarSize::ExtraLarge => 64.0,
        }
    }
}

/// Avatar component for displaying user profile images or initials
///
/// A circular element that shows user representation. Currently supports
/// fallback to initials (image support can be added later with egui's
/// texture system).
///
/// ## Example
/// ```rust,ignore
/// use egui_shadcn::components::{Avatar, AvatarSize};
///
/// // Show initials for a user
/// ui.add(Avatar::new("John Doe"));
///
/// // Custom text
/// ui.add(Avatar::new("AB").size(AvatarSize::Small));
///
/// // Large avatar
/// ui.add(Avatar::new("Admin").size(AvatarSize::Large));
/// ```
#[derive(Debug, Clone)]
pub struct Avatar {
    text: String,
    size: AvatarSize,
}

impl Avatar {
    /// Create a new avatar with text fallback
    ///
    /// The text will be processed to show initials:
    /// - "John Doe" → "JD"
    /// - "Alice" → "A"
    /// - "AB" → "AB"
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            size: AvatarSize::Medium,
        }
    }

    /// Set the size of the avatar
    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    /// Extract initials from the text
    ///
    /// Takes the first letter of each word, up to 2 letters.
    fn get_initials(&self) -> String {
        let words: Vec<&str> = self.text.split_whitespace().collect();

        if words.is_empty() {
            return "?".to_string();
        }

        // If already short (1-2 chars), use as-is
        if self.text.len() <= 2 && !self.text.contains(' ') {
            return self.text.to_uppercase();
        }

        // Take first letter of first two words
        let initials: String = words
            .iter()
            .take(2)
            .filter_map(|word| word.chars().next())
            .collect::<String>()
            .to_uppercase();

        if initials.is_empty() {
            "?".to_string()
        } else {
            initials
        }
    }
}

impl Widget for Avatar {
    fn ui(self, ui: &mut Ui) -> Response {
        // Get theme from context or fall back to light mode
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let size = self.size.pixels();
        let initials = self.get_initials();

        // Avatar styling: circular with muted background
        // Use foreground color for initials to ensure sufficient contrast (4.5:1 minimum)
        let bg_color = theme.colors.muted;
        let text_color = theme.colors.foreground;
        let corner_radius = theme.radii.avatar(); // Full circle

        // Calculate font size based on avatar size
        let font_size = match self.size {
            AvatarSize::Small => theme.typography.small().size,
            AvatarSize::Medium => theme.typography.body().size,
            AvatarSize::Large => theme.typography.large().size,
            AvatarSize::ExtraLarge => theme.typography.h4().size,
        };

        // Reserve space for the avatar
        let (rect, response) = ui.allocate_exact_size(
            Vec2::splat(size),
            egui::Sense::hover(),
        );

        if ui.is_rect_visible(rect) {
            // Draw background circle
            ui.painter().rect_filled(
                rect,
                corner_radius,
                bg_color,
            );

            // Draw initials centered
            let font_id = egui::FontId::new(font_size, egui::FontFamily::Proportional);

            let galley = ui.painter().layout_no_wrap(
                initials,
                font_id,
                text_color,
            );

            let text_pos = rect.center() - galley.size() / 2.0;
            ui.painter().galley(text_pos, galley, text_color);
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_creation() {
        let avatar = Avatar::new("John Doe");
        assert_eq!(avatar.text, "John Doe");
        assert_eq!(avatar.size, AvatarSize::Medium);
    }

    #[test]
    fn test_avatar_size() {
        let avatar = Avatar::new("Test").size(AvatarSize::Large);
        assert_eq!(avatar.size, AvatarSize::Large);

        assert_eq!(AvatarSize::Small.pixels(), 32.0);
        assert_eq!(AvatarSize::Medium.pixels(), 40.0);
        assert_eq!(AvatarSize::Large.pixels(), 48.0);
        assert_eq!(AvatarSize::ExtraLarge.pixels(), 64.0);
    }

    #[test]
    fn test_initials_extraction() {
        let avatar = Avatar::new("John Doe");
        assert_eq!(avatar.get_initials(), "JD");

        let avatar = Avatar::new("Alice");
        assert_eq!(avatar.get_initials(), "A");

        let avatar = Avatar::new("Bob Charlie Delta");
        assert_eq!(avatar.get_initials(), "BC");

        let avatar = Avatar::new("AB");
        assert_eq!(avatar.get_initials(), "AB");

        let avatar = Avatar::new("");
        assert_eq!(avatar.get_initials(), "?");

        let avatar = Avatar::new("   ");
        assert_eq!(avatar.get_initials(), "?");
    }

    #[test]
    fn test_initials_case() {
        let avatar = Avatar::new("john doe");
        assert_eq!(avatar.get_initials(), "JD");

        let avatar = Avatar::new("alice");
        assert_eq!(avatar.get_initials(), "A");
    }
}
