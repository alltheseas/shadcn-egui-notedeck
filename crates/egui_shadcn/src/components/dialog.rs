//! Dialog component matching shadcn/ui
//!
//! A modal dialog that overlays the page content.
//!
//! Reference: <https://ui.shadcn.com/docs/components/dialog>

use egui::{Color32, Frame, Id, Modal, Ui};
use crate::theme::ShadcnTheme;

/// Dialog component for modal overlays
///
/// ## Example
/// ```rust,ignore
/// let mut open = false;
///
/// if ui.button("Open Dialog").clicked() {
///     open = true;
/// }
///
/// Dialog::new("my_dialog")
///     .title("Edit Profile")
///     .description("Make changes to your profile here.")
///     .show(ui.ctx(), &mut open, |ui| {
///         ui.label("Dialog content goes here");
///     });
/// ```
pub struct Dialog {
    id: Id,
    title: Option<String>,
    description: Option<String>,
    max_width: f32,
    closable: bool,
}

impl Dialog {
    /// Create a new dialog with the given ID
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            title: None,
            description: None,
            max_width: 425.0,
            closable: true,
        }
    }

    /// Set the dialog title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the dialog description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the maximum width of the dialog (default: 425px)
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Set whether the dialog shows a close button (default: true)
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Show the dialog with custom content
    ///
    /// Returns the inner value from the content closure if the dialog is open
    pub fn show<R>(
        self,
        ctx: &egui::Context,
        open: &mut bool,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R> {
        if !*open {
            return None;
        }

        let theme = ctx.data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        // Create frame matching shadcn style
        let frame = Frame::NONE
            .fill(theme.colors.background)
            .stroke(egui::Stroke::new(1.0, theme.colors.border))
            .corner_radius(theme.radii.lg)
            .shadow(theme.shadows.lg)
            .inner_margin(theme.spacing.lg);

        // Semi-transparent backdrop (50% opacity like shadcn)
        let backdrop_color = Color32::from_rgba_unmultiplied(0, 0, 0, 128);

        let modal = Modal::new(self.id)
            .backdrop_color(backdrop_color)
            .frame(frame);

        let modal_response = modal.show(ctx, |ui| {
            ui.set_max_width(self.max_width);
            ui.set_min_width(200.0);

            // Header section with title, description, and close button
            if self.title.is_some() || self.description.is_some() || self.closable {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        // Title
                        if let Some(title) = &self.title {
                            ui.add(egui::Label::new(
                                egui::RichText::new(title)
                                    .size(theme.typography.large().size)
                                    .strong()
                                    .color(theme.colors.foreground)
                            ));
                        }

                        // Description - use 70% foreground for sufficient contrast
                        if let Some(desc) = &self.description {
                            ui.add(egui::Label::new(
                                egui::RichText::new(desc)
                                    .size(theme.typography.small().size)
                                    .color(theme.colors.foreground.linear_multiply(0.7))
                            ));
                        }
                    });

                    // Close button (positioned to the right)
                    // Use 60% foreground for sufficient contrast on interactive element
                    // 44px minimum touch target per Apple HIG
                    if self.closable {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            let close_btn = ui.add_sized(
                                egui::vec2(44.0, 44.0),
                                egui::Button::new(
                                    egui::RichText::new("X")
                                        .size(14.0)
                                        .strong()
                                        .color(theme.colors.foreground.linear_multiply(0.6))
                                )
                                .fill(Color32::TRANSPARENT)
                                .stroke(egui::Stroke::NONE)
                                .corner_radius(theme.radii.sm)
                            );
                            if close_btn.clicked() {
                                ui.close();
                            }
                            if close_btn.hovered() {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                            }
                        });
                    }
                });

                ui.add_space(theme.spacing.md);
            }

            // Content
            content(ui)
        });

        // Handle closing
        if modal_response.should_close() {
            *open = false;
        }

        Some(modal_response.inner)
    }

    /// Show the dialog with content and footer buttons
    pub fn show_with_footer<R>(
        self,
        ctx: &egui::Context,
        open: &mut bool,
        content: impl FnOnce(&mut Ui) -> R,
        footer: impl FnOnce(&mut Ui),
    ) -> Option<R> {
        if !*open {
            return None;
        }

        let theme = ctx.data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let frame = Frame::NONE
            .fill(theme.colors.background)
            .stroke(egui::Stroke::new(1.0, theme.colors.border))
            .corner_radius(theme.radii.lg)
            .shadow(theme.shadows.lg)
            .inner_margin(theme.spacing.lg);

        let backdrop_color = Color32::from_rgba_unmultiplied(0, 0, 0, 128);

        let modal = Modal::new(self.id)
            .backdrop_color(backdrop_color)
            .frame(frame);

        let modal_response = modal.show(ctx, |ui| {
            ui.set_max_width(self.max_width);
            ui.set_min_width(200.0);

            // Header section
            if self.title.is_some() || self.description.is_some() || self.closable {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        if let Some(title) = &self.title {
                            ui.add(egui::Label::new(
                                egui::RichText::new(title)
                                    .size(theme.typography.large().size)
                                    .strong()
                                    .color(theme.colors.foreground)
                            ));
                        }
                        // Description - use 70% foreground for sufficient contrast
                        if let Some(desc) = &self.description {
                            ui.add(egui::Label::new(
                                egui::RichText::new(desc)
                                    .size(theme.typography.small().size)
                                    .color(theme.colors.foreground.linear_multiply(0.7))
                            ));
                        }
                    });

                    // Close button - use 60% foreground for sufficient contrast
                    // 44px minimum touch target per Apple HIG
                    if self.closable {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            let close_btn = ui.add_sized(
                                egui::vec2(44.0, 44.0),
                                egui::Button::new(
                                    egui::RichText::new("X")
                                        .size(14.0)
                                        .strong()
                                        .color(theme.colors.foreground.linear_multiply(0.6))
                                )
                                .fill(Color32::TRANSPARENT)
                                .stroke(egui::Stroke::NONE)
                                .corner_radius(theme.radii.sm)
                            );
                            if close_btn.clicked() {
                                ui.close();
                            }
                        });
                    }
                });
                ui.add_space(theme.spacing.md);
            }

            // Content
            let result = content(ui);

            // Footer
            ui.add_space(theme.spacing.md);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                footer(ui);
            });

            result
        });

        if modal_response.should_close() {
            *open = false;
        }

        Some(modal_response.inner)
    }
}

/// Response from a confirmation dialog
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmResult {
    /// User confirmed
    Confirmed,
    /// User cancelled
    Cancelled,
    /// Dialog is still open (no decision yet)
    Pending,
}

/// Helper to create a simple confirmation dialog
///
/// Returns the user's choice when they click a button, or `Pending` if still open.
pub fn confirm_dialog(
    ctx: &egui::Context,
    id: impl Into<Id>,
    open: &mut bool,
    title: &str,
    message: &str,
) -> ConfirmResult {
    if !*open {
        return ConfirmResult::Pending;
    }

    let mut result = ConfirmResult::Pending;
    let id = id.into();

    let theme = ctx.data(|d| {
        d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
            .unwrap_or_else(ShadcnTheme::light)
    });

    let frame = Frame::NONE
        .fill(theme.colors.background)
        .stroke(egui::Stroke::new(1.0, theme.colors.border))
        .corner_radius(theme.radii.lg)
        .shadow(theme.shadows.lg)
        .inner_margin(theme.spacing.lg);

    let backdrop_color = Color32::from_rgba_unmultiplied(0, 0, 0, 128);

    let modal = Modal::new(id)
        .backdrop_color(backdrop_color)
        .frame(frame);

    let modal_response = modal.show(ctx, |ui| {
        ui.set_max_width(425.0);
        ui.set_min_width(200.0);

        // Title
        ui.add(egui::Label::new(
            egui::RichText::new(title)
                .size(theme.typography.large().size)
                .strong()
                .color(theme.colors.foreground)
        ));

        // Message - use 70% foreground for sufficient contrast (WCAG AA)
        ui.add(egui::Label::new(
            egui::RichText::new(message)
                .size(theme.typography.small().size)
                .color(theme.colors.foreground.linear_multiply(0.7))
        ));

        ui.add_space(theme.spacing.lg);

        // Footer buttons (right-aligned)
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Confirm button (primary)
            if ui.add(
                egui::Button::new("Confirm")
                    .fill(theme.colors.primary)
            ).clicked() {
                result = ConfirmResult::Confirmed;
            }

            ui.add_space(theme.spacing.sm);

            // Cancel button (outline style)
            if ui.add(
                egui::Button::new("Cancel")
                    .fill(Color32::TRANSPARENT)
                    .stroke(egui::Stroke::new(1.0, theme.colors.border))
            ).clicked() {
                result = ConfirmResult::Cancelled;
            }
        });
    });

    // Close on backdrop click or escape
    if modal_response.should_close() {
        result = ConfirmResult::Cancelled;
    }

    // Close dialog if user made a choice
    if result != ConfirmResult::Pending {
        *open = false;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_builder() {
        let dialog = Dialog::new("test")
            .title("Test Title")
            .description("Test Description")
            .max_width(500.0)
            .closable(false);

        assert!(dialog.title.as_ref().unwrap() == "Test Title");
        assert!(dialog.description.as_ref().unwrap() == "Test Description");
        assert!(dialog.max_width == 500.0);
        assert!(!dialog.closable);
    }

    #[test]
    fn test_confirm_result() {
        assert_ne!(ConfirmResult::Confirmed, ConfirmResult::Cancelled);
        assert_ne!(ConfirmResult::Confirmed, ConfirmResult::Pending);
        assert_ne!(ConfirmResult::Cancelled, ConfirmResult::Pending);
    }
}
