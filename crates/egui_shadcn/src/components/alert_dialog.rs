//! Alert Dialog component ported from shadcn/ui
//!
//! A modal dialog that interrupts the user with important content and expects a response.
//! Unlike regular Dialog, AlertDialog is specifically for confirmations and destructive actions.
//!
//! Reference: <https://ui.shadcn.com/docs/components/alert-dialog>

use egui::{Color32, Id, Vec2};
use crate::theme::ShadcnTheme;

/// Alert Dialog component for confirmations and destructive actions
///
/// ## Example
/// ```rust,ignore
/// let mut show_alert = false;
///
/// if ui.button("Delete Account").clicked() {
///     show_alert = true;
/// }
///
/// match AlertDialog::new("delete_confirm")
///     .title("Are you absolutely sure?")
///     .description("This action cannot be undone. This will permanently delete your account.")
///     .cancel_text("Cancel")
///     .action_text("Delete")
///     .destructive(true)
///     .show(ui.ctx(), &mut show_alert)
/// {
///     AlertDialogResult::Action => {
///         // User confirmed deletion
///     }
///     AlertDialogResult::Cancel => {
///         // User cancelled
///     }
///     AlertDialogResult::Pending => {
///         // Dialog still open
///     }
/// }
/// ```
pub struct AlertDialog {
    id: Id,
    title: String,
    description: Option<String>,
    cancel_text: String,
    action_text: String,
    destructive: bool,
    max_width: f32,
}

/// Result from showing an alert dialog
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertDialogResult {
    /// User clicked the action button
    Action,
    /// User clicked cancel or dismissed
    Cancel,
    /// Dialog is still open (no decision yet)
    Pending,
}

impl AlertDialog {
    /// Create a new alert dialog
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            title: "Are you sure?".to_string(),
            description: None,
            cancel_text: "Cancel".to_string(),
            action_text: "Continue".to_string(),
            destructive: false,
            max_width: 500.0,
        }
    }

    /// Set the dialog title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the dialog description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the cancel button text (default: "Cancel")
    pub fn cancel_text(mut self, text: impl Into<String>) -> Self {
        self.cancel_text = text.into();
        self
    }

    /// Set the action button text (default: "Continue")
    pub fn action_text(mut self, text: impl Into<String>) -> Self {
        self.action_text = text.into();
        self
    }

    /// Set whether the action is destructive (shows red button)
    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    /// Set the maximum width (default: 500px)
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Show the alert dialog
    pub fn show(self, ctx: &egui::Context, open: &mut bool) -> AlertDialogResult {
        if !*open {
            return AlertDialogResult::Pending;
        }

        let theme = ctx.data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let mut result = AlertDialogResult::Pending;

        // Draw backdrop
        #[allow(deprecated)]
        let screen_rect = ctx.screen_rect();
        let backdrop_layer = egui::LayerId::new(egui::Order::Middle, self.id.with("backdrop"));
        ctx.layer_painter(backdrop_layer).rect_filled(
            screen_rect,
            0.0,
            Color32::from_black_alpha(128),
        );

        // Calculate dialog position (centered)
        let dialog_width = self.max_width.min(screen_rect.width() - 32.0);
        let dialog_height = if self.description.is_some() { 200.0 } else { 160.0 };
        let dialog_pos = screen_rect.center() - Vec2::new(dialog_width / 2.0, dialog_height / 2.0);

        // Draw dialog
        egui::Area::new(self.id.with("dialog"))
            .order(egui::Order::Foreground)
            .fixed_pos(dialog_pos)
            .show(ctx, |ui| {
                let frame = egui::Frame::NONE
                    .fill(theme.colors.background)
                    .stroke(egui::Stroke::new(1.0, theme.colors.border))
                    .corner_radius(theme.radii.lg)
                    .shadow(theme.shadows.lg)
                    .inner_margin(theme.spacing.lg);

                frame.show(ui, |ui| {
                    ui.set_min_width(dialog_width - theme.spacing.lg * 2.0);
                    ui.set_max_width(dialog_width - theme.spacing.lg * 2.0);

                    // Title
                    ui.label(
                        egui::RichText::new(&self.title)
                            .size(theme.typography.large().size)
                            .strong()
                            .color(theme.colors.foreground)
                    );

                    // Description
                    if let Some(ref desc) = self.description {
                        ui.add_space(theme.spacing.sm);
                        ui.label(
                            egui::RichText::new(desc)
                                .size(theme.typography.body().size)
                                .color(theme.colors.muted_foreground)
                        );
                    }

                    ui.add_space(theme.spacing.lg);

                    // Buttons (right-aligned)
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Action button
                        let action_btn_size = Vec2::new(100.0, 44.0);
                        let (action_rect, action_response) = ui.allocate_exact_size(
                            action_btn_size,
                            egui::Sense::click(),
                        );

                        if ui.is_rect_visible(action_rect) {
                            let hovered = action_response.hovered();
                            let bg_color = if self.destructive {
                                if hovered {
                                    darken(theme.colors.destructive, 0.1)
                                } else {
                                    theme.colors.destructive
                                }
                            } else {
                                if hovered {
                                    darken(theme.colors.primary, 0.1)
                                } else {
                                    theme.colors.primary
                                }
                            };

                            ui.painter().rect_filled(action_rect, theme.radii.md, bg_color);

                            let text_color = if self.destructive {
                                theme.colors.destructive_foreground
                            } else {
                                theme.colors.primary_foreground
                            };

                            ui.painter().text(
                                action_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                &self.action_text,
                                egui::FontId::proportional(theme.typography.body().size),
                                text_color,
                            );
                        }

                        if action_response.clicked() {
                            result = AlertDialogResult::Action;
                        }

                        ui.add_space(theme.spacing.sm);

                        // Cancel button (outline style)
                        let cancel_btn_size = Vec2::new(80.0, 44.0);
                        let (cancel_rect, cancel_response) = ui.allocate_exact_size(
                            cancel_btn_size,
                            egui::Sense::click(),
                        );

                        if ui.is_rect_visible(cancel_rect) {
                            let hovered = cancel_response.hovered();
                            let bg_color = if hovered {
                                theme.colors.secondary
                            } else {
                                Color32::TRANSPARENT
                            };

                            ui.painter().rect_filled(cancel_rect, theme.radii.md, bg_color);
                            ui.painter().rect_stroke(
                                cancel_rect,
                                theme.radii.md,
                                egui::Stroke::new(1.0, theme.colors.border),
                                egui::StrokeKind::Inside,
                            );

                            ui.painter().text(
                                cancel_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                &self.cancel_text,
                                egui::FontId::proportional(theme.typography.body().size),
                                theme.colors.foreground,
                            );
                        }

                        if cancel_response.clicked() {
                            result = AlertDialogResult::Cancel;
                        }
                    });
                });
            });

        // Handle escape key
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            result = AlertDialogResult::Cancel;
        }

        // Close dialog if user made a choice
        if result != AlertDialogResult::Pending {
            *open = false;
        }

        result
    }
}

/// Darken a color by a factor (0.0 to 1.0)
fn darken(color: Color32, factor: f32) -> Color32 {
    let [r, g, b, a] = color.to_array();
    let r = (r as f32 * (1.0 - factor)).max(0.0) as u8;
    let g = (g as f32 * (1.0 - factor)).max(0.0) as u8;
    let b = (b as f32 * (1.0 - factor)).max(0.0) as u8;
    Color32::from_rgba_premultiplied(r, g, b, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_dialog_creation() {
        let dialog = AlertDialog::new("test")
            .title("Delete?")
            .description("This cannot be undone")
            .cancel_text("No")
            .action_text("Yes, delete")
            .destructive(true);

        assert_eq!(dialog.title, "Delete?");
        assert!(dialog.destructive);
    }

    #[test]
    fn test_alert_dialog_result() {
        assert_ne!(AlertDialogResult::Action, AlertDialogResult::Cancel);
        assert_ne!(AlertDialogResult::Action, AlertDialogResult::Pending);
    }
}
