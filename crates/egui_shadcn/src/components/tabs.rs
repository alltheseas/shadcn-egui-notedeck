//! Tabs component ported from shadcn/ui
//!
//! A set of layered sections of content (tab panels) with tab buttons for navigation.
//!
//! Reference: <https://ui.shadcn.com/docs/components/tabs>

use egui::{Response, Ui};
use crate::theme::ShadcnTheme;

/// Tabs component for organizing content
///
/// ## Example
/// ```rust,ignore
/// Tabs::new(ui, "settings-tabs")
///     .tab("account", "Account", |ui| {
///         ui.label("Account settings");
///     })
///     .tab("password", "Password", |ui| {
///         ui.label("Password settings");
///     })
///     .show();
/// ```
pub struct Tabs<'a> {
    ui: &'a mut Ui,
    id: &'a str,
    tabs: Vec<(&'a str, &'a str, Box<dyn FnOnce(&mut Ui) + 'a>)>,
}

impl<'a> Tabs<'a> {
    /// Create new tabs
    pub fn new(ui: &'a mut Ui, id: &'a str) -> Self {
        Self {
            ui,
            id,
            tabs: Vec::new(),
        }
    }

    /// Add a tab
    pub fn tab(
        mut self,
        tab_id: &'a str,
        label: &'a str,
        content: impl FnOnce(&mut Ui) + 'a,
    ) -> Self {
        self.tabs.push((tab_id, label, Box::new(content)));
        self
    }

    /// Show the tabs
    pub fn show(self) -> Response {
        // Get theme from context or fall back to light mode
        let theme = self.ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });
        let id = egui::Id::new(self.id);

        // Get or create selected tab state
        let mut selected = self.ui.ctx().data_mut(|d| {
            d.get_temp::<usize>(id).unwrap_or(0)
        });

        // Render tab triggers with underline indicator (shadcn style)
        let mut response = self.ui.horizontal(|ui| {
            for (idx, (_, label, _)) in self.tabs.iter().enumerate() {
                let is_selected = idx == selected;

                // Use foreground for both, but with different opacity for inactive
                // This ensures good contrast in both light and dark modes
                let text_color = if is_selected {
                    theme.colors.foreground
                } else {
                    // Use foreground with reduced opacity for inactive tabs
                    // This maintains readability while showing inactive state
                    theme.colors.foreground.linear_multiply(0.6)
                };

                // Use transparent button for cleaner look
                // 44px minimum height for Apple HIG touch target
                let button = egui::Button::new(
                    egui::RichText::new(*label)
                        .color(text_color)
                        .size(theme.typography.body().size)
                )
                .fill(egui::Color32::TRANSPARENT)
                .stroke(egui::Stroke::NONE)
                .min_size(egui::vec2(0.0, 44.0));

                let button_response = ui.add(button);

                // Draw underline indicator for selected tab
                if is_selected {
                    let underline_y = button_response.rect.bottom();
                    ui.painter().line_segment(
                        [
                            egui::pos2(button_response.rect.left(), underline_y),
                            egui::pos2(button_response.rect.right(), underline_y),
                        ],
                        egui::Stroke::new(2.0, theme.colors.primary),
                    );
                }

                if button_response.clicked() {
                    selected = idx;
                    ui.ctx().data_mut(|d| d.insert_temp(id, selected));
                }
            }
        }).response;

        // Render selected tab content with visible border
        if let Some((_, _, content)) = self.tabs.into_iter().nth(selected) {
            self.ui.add_space(theme.spacing.md);

            // Use foreground at 30% for visible border while maintaining visual hierarchy
            let frame = egui::Frame::NONE
                .fill(theme.colors.background)
                .stroke(egui::Stroke::new(1.0, theme.colors.foreground.linear_multiply(0.3)))
                .corner_radius(theme.radii.md)
                .inner_margin(theme.spacing.md);

            response = frame.show(self.ui, |ui| {
                content(ui);
            }).response;
        }

        response
    }
}
