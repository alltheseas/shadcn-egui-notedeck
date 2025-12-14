//! Collapsible component ported from shadcn/ui
//!
//! An interactive component that expands/collapses content.
//!
//! Reference: <https://ui.shadcn.com/docs/components/collapsible>

use egui::{Id, Ui, Sense, Vec2, Pos2};
use crate::theme::ShadcnTheme;

/// Collapsible component for expandable content sections
///
/// ## Example
/// ```rust,ignore
/// Collapsible::new("my_collapsible", &mut is_open)
///     .trigger(|ui, is_open| {
///         ui.horizontal(|ui| {
///             ui.label("Section Title");
///             let icon = if is_open { "▼" } else { "▶" };
///             ui.label(icon);
///         });
///     })
///     .content(|ui| {
///         ui.label("Hidden content here");
///     })
///     .show(ui);
/// ```
pub struct Collapsible<'a> {
    id: Id,
    open: &'a mut bool,
    trigger: Option<Box<dyn FnOnce(&mut Ui, bool) + 'a>>,
    content: Option<Box<dyn FnOnce(&mut Ui) + 'a>>,
    animate: bool,
}

impl<'a> Collapsible<'a> {
    /// Create a new collapsible
    pub fn new(id: impl std::hash::Hash, open: &'a mut bool) -> Self {
        Self {
            id: Id::new(id),
            open,
            trigger: None,
            content: None,
            animate: true,
        }
    }

    /// Set the trigger/header content
    ///
    /// The closure receives `(ui, is_open)` so you can adjust the trigger appearance.
    pub fn trigger(mut self, f: impl FnOnce(&mut Ui, bool) + 'a) -> Self {
        self.trigger = Some(Box::new(f));
        self
    }

    /// Set the collapsible content
    pub fn content(mut self, f: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.content = Some(Box::new(f));
        self
    }

    /// Disable animation (instant open/close)
    pub fn no_animation(mut self) -> Self {
        self.animate = false;
        self
    }

    /// Show the collapsible
    pub fn show(self, ui: &mut Ui) {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let is_open = *self.open;

        // Render trigger with clickable area
        if let Some(trigger_fn) = self.trigger {
            // Render the trigger content first to measure its size
            let trigger_response = ui.horizontal(|ui| {
                ui.set_min_height(44.0); // Apple HIG touch target
                trigger_fn(ui, is_open);
            });

            // Make the whole trigger area clickable by using interact()
            let click_response = ui.interact(
                trigger_response.response.rect,
                self.id.with("trigger"),
                egui::Sense::click(),
            );

            if click_response.clicked() {
                *self.open = !*self.open;
            }

            // Hover effect
            if click_response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
        } else {
            // Default trigger: text on left, double chevron on right (matching shadcn style)
            let response = ui.horizontal(|ui| {
                ui.set_min_height(44.0); // Apple HIG touch target

                // Text on left
                ui.label(
                    egui::RichText::new("Toggle")
                        .size(theme.typography.body().size)
                        .color(theme.colors.foreground),
                );

                // Double chevron on right - matching sidebar style exactly
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let chevron_size_vec = Vec2::new(16.0, 20.0);
                    let (rect, _) = ui.allocate_exact_size(chevron_size_vec, Sense::hover());

                    if ui.is_rect_visible(rect) {
                        let painter = ui.painter();
                        let cx = rect.center().x;
                        let stroke = egui::Stroke::new(1.2, theme.colors.muted_foreground);

                        // Up chevron (^)
                        let up_y = rect.center().y - 4.0;
                        painter.line_segment(
                            [Pos2::new(cx - 3.0, up_y + 2.0), Pos2::new(cx, up_y - 1.0)],
                            stroke,
                        );
                        painter.line_segment(
                            [Pos2::new(cx, up_y - 1.0), Pos2::new(cx + 3.0, up_y + 2.0)],
                            stroke,
                        );

                        // Down chevron (v)
                        let down_y = rect.center().y + 4.0;
                        painter.line_segment(
                            [Pos2::new(cx - 3.0, down_y - 2.0), Pos2::new(cx, down_y + 1.0)],
                            stroke,
                        );
                        painter.line_segment(
                            [Pos2::new(cx, down_y + 1.0), Pos2::new(cx + 3.0, down_y - 2.0)],
                            stroke,
                        );
                    }
                });
            });

            // Make the whole trigger area clickable
            let click_response = ui.interact(
                response.response.rect,
                self.id.with("default_trigger"),
                egui::Sense::click(),
            );

            if click_response.clicked() {
                *self.open = !*self.open;
            }
        }

        // Render content if open
        if is_open {
            if let Some(content_fn) = self.content {
                // Use CollapsingState for animation if enabled
                if self.animate {
                    let collapsing_id = self.id.with("content");

                    // Get or create animation state
                    let openness = ui.ctx().animate_bool_with_time(
                        collapsing_id,
                        true,
                        ui.style().animation_time,
                    );

                    if openness > 0.0 {
                        ui.scope(|ui| {
                            // Apply opacity animation
                            if openness < 1.0 {
                                ui.set_opacity(openness);
                            }
                            content_fn(ui);
                        });
                    }
                } else {
                    content_fn(ui);
                }
            }
        } else if self.animate {
            // Animate close
            let collapsing_id = self.id.with("content");
            let _openness = ui.ctx().animate_bool_with_time(
                collapsing_id,
                false,
                ui.style().animation_time,
            );
        }
    }
}

/// Simple collapsible header helper
///
/// Creates a clickable header with an expand/collapse chevron.
pub fn collapsible_trigger(
    ui: &mut Ui,
    title: &str,
    is_open: bool,
    on_click: impl FnOnce(),
) -> bool {
    let theme = ui.ctx().data(|d| {
        d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
            .unwrap_or_else(ShadcnTheme::light)
    });

    let response = ui.horizontal(|ui| {
        ui.set_min_height(44.0); // Apple HIG touch target

        ui.label(
            egui::RichText::new(title)
                .size(theme.typography.body().size)
                .strong()
                .color(theme.colors.foreground),
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Draw chevron using line segments
            let chevron_size_vec = Vec2::new(16.0, 16.0);
            let (chevron_rect, _) = ui.allocate_exact_size(chevron_size_vec, Sense::hover());

            if ui.is_rect_visible(chevron_rect) {
                let painter = ui.painter();
                let cx = chevron_rect.center().x;
                let stroke = egui::Stroke::new(1.2, theme.colors.muted_foreground);

                // Double chevron - matching sidebar style exactly
                // Up chevron (^)
                let up_y = chevron_rect.center().y - 4.0;
                painter.line_segment(
                    [Pos2::new(cx - 3.0, up_y + 2.0), Pos2::new(cx, up_y - 1.0)],
                    stroke,
                );
                painter.line_segment(
                    [Pos2::new(cx, up_y - 1.0), Pos2::new(cx + 3.0, up_y + 2.0)],
                    stroke,
                );

                // Down chevron (v)
                let down_y = chevron_rect.center().y + 4.0;
                painter.line_segment(
                    [Pos2::new(cx - 3.0, down_y - 2.0), Pos2::new(cx, down_y + 1.0)],
                    stroke,
                );
                painter.line_segment(
                    [Pos2::new(cx, down_y + 1.0), Pos2::new(cx + 3.0, down_y - 2.0)],
                    stroke,
                );
            }
        });
    });

    if response.response.clicked() {
        on_click();
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapsible_creation() {
        let mut open = false;
        let collapsible = Collapsible::new("test", &mut open)
            .no_animation();
        assert!(!collapsible.animate);
    }
}
