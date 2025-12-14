//! Card component ported from shadcn/ui
//!
//! A container component for grouping related content with header, body, and footer.
//! Provides a structured layout with consistent styling.
//!
//! Reference: <https://ui.shadcn.com/docs/components/card>
//!
//! ## Features
//! - Structured layout: header, content, footer
//! - Rounded corners and subtle shadow from theme
//! - Border and background colors from theme
//! - Flexible content organization
//!
//! ## Usage
//! ```rust,ignore
//! use egui_shadcn::components::Card;
//!
//! Card::new(ui)
//!     .header(|ui| {
//!         ui.heading("Card Title");
//!         ui.label("Description text");
//!     })
//!     .content(|ui| {
//!         ui.label("Main content goes here");
//!     })
//!     .footer(|ui| {
//!         ui.button("Action");
//!     })
//!     .show();
//! ```

use egui::{Response, Ui};

use crate::theme::ShadcnTheme;

/// Card component for grouping related content
///
/// Provides a container with optional header, content, and footer sections.
/// Each section is rendered in order with appropriate spacing and styling.
///
/// ## Example
/// ```rust,ignore
/// use egui_shadcn::components::Card;
///
/// // Simple card with just content
/// Card::new(ui)
///     .content(|ui| {
///         ui.label("Hello, card!");
///     })
///     .show();
///
/// // Full card with all sections
/// Card::new(ui)
///     .header(|ui| {
///         ui.heading("User Profile");
///         ui.label("Manage your account settings");
///     })
///     .content(|ui| {
///         ui.label("Name: John Doe");
///         ui.label("Email: john@example.com");
///     })
///     .footer(|ui| {
///         if ui.button("Save Changes").clicked() {
///             // Handle save
///         }
///     })
///     .show();
/// ```
pub struct Card<'a> {
    ui: &'a mut Ui,
    header: Option<Box<dyn FnOnce(&mut Ui) + 'a>>,
    content: Option<Box<dyn FnOnce(&mut Ui) + 'a>>,
    footer: Option<Box<dyn FnOnce(&mut Ui) + 'a>>,
    hoverable: bool,
}

impl<'a> Card<'a> {
    /// Create a new card in the given UI context
    pub fn new(ui: &'a mut Ui) -> Self {
        Self {
            ui,
            header: None,
            content: None,
            footer: None,
            hoverable: true,
        }
    }

    /// Enable/disable hover effect (default: true)
    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }

    /// Add a header section to the card
    ///
    /// The header typically contains a title and optional description.
    pub fn header(mut self, header: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.header = Some(Box::new(header));
        self
    }

    /// Add a content section to the card
    ///
    /// This is the main body of the card where primary content goes.
    pub fn content(mut self, content: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.content = Some(Box::new(content));
        self
    }

    /// Add a footer section to the card
    ///
    /// The footer typically contains action buttons or additional info.
    pub fn footer(mut self, footer: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.footer = Some(Box::new(footer));
        self
    }

    /// Render the card with all its sections
    pub fn show(self) -> Response {
        let theme = self.ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        // Card styling
        let corner_radius = theme.radii.card();
        let bg_color = theme.colors.card;
        let shadow = theme.shadows.card();
        let hoverable = self.hoverable;

        // Base border color
        let border_color = theme.colors.border;

        let frame = egui::Frame::NONE
            .fill(bg_color)
            .stroke(egui::Stroke::new(1.0, border_color))
            .corner_radius(corner_radius)
            .shadow(shadow)
            .inner_margin(0.0); // We'll add padding per section

        let response = frame.show(self.ui, |ui| {
            // Card uses consistent padding throughout - no internal dividers (shadcn style)
            let card_padding = theme.spacing.vec2_xy(6, 6); // 24px

            egui::Frame::NONE
                .inner_margin(card_padding)
                .show(ui, |ui| {
                    // Header section
                    if let Some(header_fn) = self.header {
                        header_fn(ui);
                        // Add spacing after header if there's content or footer
                        if self.content.is_some() || self.footer.is_some() {
                            ui.add_space(theme.spacing.md); // 16px gap
                        }
                    }

                    // Content section
                    if let Some(content_fn) = self.content {
                        content_fn(ui);
                        // Add spacing after content if there's a footer
                        if self.footer.is_some() {
                            ui.add_space(theme.spacing.md); // 16px gap
                        }
                    }

                    // Footer section
                    if let Some(footer_fn) = self.footer {
                        footer_fn(ui);
                    }
                });
        });

        // Draw hover effect overlay if hovered
        if hoverable && response.response.hovered() {
            let rect = response.response.rect;
            // Subtle border highlight on hover
            self.ui.painter().rect_stroke(
                rect,
                corner_radius,
                egui::Stroke::new(1.0, theme.colors.foreground.linear_multiply(0.2)),
                egui::StrokeKind::Outside,
            );
        }

        response.response
    }
}

// Helper functions for common card patterns

/// Helper to create a card title
///
/// Renders text in a larger, bold font suitable for card headers.
pub fn card_title(ui: &mut Ui, text: impl Into<String>) {
    let theme = ui.ctx().data(|d| {
        d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
            .unwrap_or_else(ShadcnTheme::light)
    });
    ui.label(
        egui::RichText::new(text.into())
            .size(theme.typography.h4().size)
            .color(theme.colors.foreground),
    );
}

/// Helper to create a card description
///
/// Renders secondary text suitable for card subtitles.
/// Uses foreground at 70% opacity to ensure sufficient contrast (4.5:1 minimum).
pub fn card_description(ui: &mut Ui, text: impl Into<String>) {
    let theme = ui.ctx().data(|d| {
        d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
            .unwrap_or_else(ShadcnTheme::light)
    });
    // Use foreground at 70% opacity for sufficient contrast while showing hierarchy
    let description_color = theme.colors.foreground.linear_multiply(0.7);
    ui.label(
        egui::RichText::new(text.into())
            .size(theme.typography.small().size)
            .color(description_color),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Full UI tests require egui context, so we test the builder pattern
    #[test]
    fn test_card_builder() {
        // Test that Card can be constructed
        // Actual rendering would need an egui context
        struct TestUi;
        // Card builder pattern is tested in integration/showcase
    }
}
