//! Popover component ported from shadcn/ui
//!
//! A floating panel with rich content, positioned relative to a trigger.
//! Unlike tooltips, popovers stay open until explicitly dismissed.
//!
//! Reference: <https://ui.shadcn.com/docs/components/popover>

use egui::{Id, Response, Ui};
use crate::theme::ShadcnTheme;

/// Popover component for displaying rich content
///
/// ## Example
/// ```rust,ignore
/// let trigger = ui.button("Open Popover");
/// Popover::new("my_popover")
///     .show(ui, &trigger, |ui| {
///         ui.label("Popover content");
///         ui.text_edit_singleline(&mut my_text);
///         if ui.button("Close").clicked() {
///             ui.close();
///         }
///     });
/// ```
pub struct Popover {
    id: Id,
    width: Option<f32>,
    offset: f32,
}

impl Popover {
    /// Create a new popover
    pub fn new(id: impl std::hash::Hash) -> Self {
        Self {
            id: Id::new(id),
            width: None,
            offset: 4.0,
        }
    }

    /// Set the popover width (default: auto-sized)
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the offset from the trigger (default: 4.0)
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    /// Show the popover
    ///
    /// The popover opens when the trigger is clicked and closes when
    /// clicking outside or calling `ui.close()` from within.
    pub fn show<R>(
        self,
        ui: &mut Ui,
        trigger: &Response,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R> {
        // Use simple boolean state in memory
        let open_id = self.id.with("open");
        let is_open = ui.ctx().data(|d| d.get_temp::<bool>(open_id).unwrap_or(false));

        // Toggle on click
        let mut new_open_state = is_open;
        if trigger.clicked() {
            new_open_state = !is_open;
        }

        if !new_open_state {
            // Save state and return early
            ui.ctx().data_mut(|d| d.insert_temp(open_id, new_open_state));
            return None;
        }

        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        // Position below the trigger
        let pos = trigger.rect.left_bottom() + egui::vec2(0.0, self.offset);

        let mut result = None;

        let popup_area_id = self.id.with("area");
        let area_response = egui::Area::new(popup_area_id)
            .order(egui::Order::Foreground)
            .fixed_pos(pos)
            .show(ui.ctx(), |ui| {
                let frame = egui::Frame::NONE
                    .fill(theme.colors.popover)
                    .stroke(egui::Stroke::new(1.0, theme.colors.border))
                    .corner_radius(theme.radii.md)
                    .shadow(theme.shadows.md)
                    .inner_margin(theme.spacing.lg);

                frame.show(ui, |ui| {
                    if let Some(width) = self.width {
                        ui.set_min_width(width);
                        ui.set_max_width(width);
                    }

                    result = Some(content(ui));
                });
            });

        // Close when clicking outside (but not on the trigger itself)
        // Only check if we were already open (not just opened this frame)
        if is_open {
            if area_response.response.clicked_elsewhere() && !trigger.clicked() {
                new_open_state = false;
            }
        }

        // Save the open state for next frame
        ui.ctx().data_mut(|d| d.insert_temp(open_id, new_open_state));

        result
    }
}

/// Extension trait for showing popover on any response
pub trait PopoverExt {
    /// Show a popover when this element is clicked
    fn popover<R>(
        &self,
        ui: &mut Ui,
        id: impl std::hash::Hash,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R>;
}

impl PopoverExt for Response {
    fn popover<R>(
        &self,
        ui: &mut Ui,
        id: impl std::hash::Hash,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R> {
        Popover::new(id).show(ui, self, content)
    }
}

/// Builder for showing a popover with trigger
pub struct PopoverTrigger<'a> {
    id: &'a str,
    width: Option<f32>,
}

impl<'a> PopoverTrigger<'a> {
    /// Create a new popover trigger builder
    pub fn new(id: &'a str) -> Self {
        Self { id, width: None }
    }

    /// Set the popover width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Show with a button trigger and content
    pub fn button<R>(
        self,
        ui: &mut Ui,
        button_text: impl Into<String>,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> (Response, Option<R>) {
        // Create styled trigger button
        let trigger = ui.add(
            egui::Button::new(button_text.into())
                .min_size(egui::vec2(0.0, 44.0)), // Apple HIG touch target
        );

        let popover = Popover {
            id: Id::new(self.id),
            width: self.width,
            offset: 4.0,
        };

        let result = popover.show(ui, &trigger, content);

        (trigger, result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popover_creation() {
        let popover = Popover::new("test")
            .width(300.0)
            .offset(8.0);
        assert_eq!(popover.width, Some(300.0));
        assert_eq!(popover.offset, 8.0);
    }
}
