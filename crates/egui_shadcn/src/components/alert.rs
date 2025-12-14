//! Alert component ported from shadcn/ui
//!
//! Displays a callout for user attention with title and description.
//! Supports default and destructive variants.
//!
//! Reference: <https://ui.shadcn.com/docs/components/alert>

use egui::{Response, Ui};
use crate::theme::ShadcnTheme;

/// Visual style variants for Alert component
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertVariant {
    /// Default informational alert
    Default,
    /// Destructive/error alert
    Destructive,
}

/// Alert component for displaying notifications and messages
///
/// ## Example
/// ```rust,ignore
/// Alert::new(ui, AlertVariant::Default)
///     .title("Heads up!")
///     .description("This is an informational message.")
///     .show();
/// ```
pub struct Alert<'a> {
    ui: &'a mut Ui,
    variant: AlertVariant,
    title: Option<String>,
    description: Option<String>,
}

impl<'a> Alert<'a> {
    /// Create a new alert
    pub fn new(ui: &'a mut Ui, variant: AlertVariant) -> Self {
        Self {
            ui,
            variant,
            title: None,
            description: None,
        }
    }

    /// Set the alert title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the alert description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Render the alert
    pub fn show(self) -> Response {
        let theme = self.ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let (bg_color, border_color, title_color, desc_color) = match self.variant {
            AlertVariant::Default => (
                theme.colors.background,
                theme.colors.border,
                theme.colors.foreground,
                // Use 70% foreground for sufficient contrast (WCAG AA)
                theme.colors.foreground.linear_multiply(0.7),
            ),
            AlertVariant::Destructive => {
                // Use theme-aware destructive colors
                // For light mode: very light red background with dark red text
                // For dark mode: dark red background with light red/white text
                let is_dark_mode = theme.colors.background.r() < 128;

                let (bg, title_fg, desc_fg) = if is_dark_mode {
                    // Dark mode: dark red background (#3A1515 = rgb(58, 21, 21))
                    (
                        egui::Color32::from_rgb(58, 21, 21),
                        egui::Color32::from_rgb(255, 150, 150), // Light red for title
                        egui::Color32::from_rgb(230, 230, 230), // Light gray for description
                    )
                } else {
                    // Light mode: very light red background
                    (
                        egui::Color32::from_rgb(254, 242, 242),
                        theme.colors.destructive, // Dark red for title
                        egui::Color32::from_rgb(120, 50, 65), // Darker muted red for description
                    )
                };

                (bg, theme.colors.destructive, title_fg, desc_fg)
            }
        };

        let frame = egui::Frame::NONE
            .fill(bg_color)
            .stroke(egui::Stroke::new(1.0, border_color))
            .corner_radius(theme.radii.alert())
            .inner_margin(theme.spacing.vec2(4)); // 16px

        frame.show(self.ui, |ui| {
            if let Some(title) = self.title {
                ui.label(
                    egui::RichText::new(title)
                        .size(theme.typography.body().size)
                        .strong()
                        .color(title_color),
                );
            }

            if let Some(description) = self.description {
                ui.label(
                    egui::RichText::new(description)
                        .size(theme.typography.small().size)
                        .color(desc_color),
                );
            }
        }).response
    }
}
