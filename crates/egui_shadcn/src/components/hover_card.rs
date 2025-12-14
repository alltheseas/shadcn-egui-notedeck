//! Hover Card component ported from shadcn/ui
//!
//! A card that appears on hover, typically used to show preview content.
//!
//! Reference: <https://ui.shadcn.com/docs/components/hover-card>

use egui::{Id, Response, Ui};
use crate::theme::ShadcnTheme;

/// Hover Card component for preview content on hover
///
/// ## Example
/// ```rust,ignore
/// let trigger = ui.link("@username");
/// HoverCard::new("user_preview")
///     .show(ui, &trigger, |ui| {
///         ui.label("User Profile");
///         ui.label("Joined in 2020");
///     });
/// ```
pub struct HoverCard {
    id: Id,
    open_delay_ms: u64,
    close_delay_ms: u64,
    width: Option<f32>,
}

impl HoverCard {
    /// Create a new hover card
    pub fn new(id: impl std::hash::Hash) -> Self {
        Self {
            id: Id::new(id),
            open_delay_ms: 200,
            close_delay_ms: 100,
            width: Some(300.0),
        }
    }

    /// Set the delay before opening (default: 200ms)
    pub fn open_delay(mut self, ms: u64) -> Self {
        self.open_delay_ms = ms;
        self
    }

    /// Set the delay before closing (default: 100ms)
    pub fn close_delay(mut self, ms: u64) -> Self {
        self.close_delay_ms = ms;
        self
    }

    /// Set the card width (default: 300.0)
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Show the hover card
    pub fn show<R>(
        self,
        ui: &mut Ui,
        trigger: &Response,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R> {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        // Track hover state with timing
        let hover_start_id = self.id.with("hover_start");
        let is_visible_id = self.id.with("visible");

        let now = ui.ctx().input(|i| i.time);
        let trigger_hovered = trigger.hovered();

        // Get previous hover start time
        let hover_start: Option<f64> = ui.ctx().data(|d| d.get_temp(hover_start_id));
        let was_visible: bool = ui.ctx().data(|d| d.get_temp(is_visible_id).unwrap_or(false));

        // Determine if we should show the card
        let should_show = if trigger_hovered {
            match hover_start {
                Some(start) => {
                    let elapsed_ms = ((now - start) * 1000.0) as u64;
                    elapsed_ms >= self.open_delay_ms
                }
                None => {
                    // Start tracking hover
                    ui.ctx().data_mut(|d| d.insert_temp(hover_start_id, now));
                    false
                }
            }
        } else if was_visible {
            // Check if we should close with delay
            match hover_start {
                Some(start) if start < 0.0 => {
                    // Negative start means we're in close delay
                    let close_start = -start;
                    let elapsed_ms = ((now - close_start) * 1000.0) as u64;
                    elapsed_ms < self.close_delay_ms
                }
                _ => {
                    // Start close delay
                    ui.ctx().data_mut(|d| d.insert_temp(hover_start_id, -now));
                    true
                }
            }
        } else {
            // Clear hover start when not hovering
            ui.ctx().data_mut(|d| d.remove::<f64>(hover_start_id));
            false
        };

        // Update visibility state
        ui.ctx().data_mut(|d| d.insert_temp(is_visible_id, should_show));

        if !should_show {
            return None;
        }

        // Position below the trigger
        let pos = trigger.rect.left_bottom() + egui::vec2(0.0, 4.0);

        let mut result = None;

        let card_id = self.id.with("card");
        let area_response = egui::Area::new(card_id)
            .order(egui::Order::Foreground)
            .fixed_pos(pos)
            .show(ui.ctx(), |ui| {
                let frame = egui::Frame::NONE
                    .fill(theme.colors.popover)
                    .stroke(egui::Stroke::new(1.0, theme.colors.border))
                    .corner_radius(theme.radii.lg)
                    .shadow(theme.shadows.lg)
                    .inner_margin(theme.spacing.lg);

                frame.show(ui, |ui| {
                    if let Some(width) = self.width {
                        ui.set_min_width(width);
                        ui.set_max_width(width);
                    }

                    result = Some(content(ui));
                });
            });

        // Keep visible if hovering over the card
        let card_hovered = area_response.response.hovered();
        if card_hovered && !trigger_hovered {
            // Reset close timer when hovering over card
            ui.ctx().data_mut(|d| d.insert_temp(hover_start_id, now));
        }

        result
    }
}

/// Extension trait for showing hover card on any response
pub trait HoverCardExt {
    /// Show a hover card when this element is hovered
    fn hover_card<R>(
        &self,
        ui: &mut Ui,
        id: impl std::hash::Hash,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R>;
}

impl HoverCardExt for Response {
    fn hover_card<R>(
        &self,
        ui: &mut Ui,
        id: impl std::hash::Hash,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R> {
        HoverCard::new(id).show(ui, self, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hover_card_creation() {
        let card = HoverCard::new("test")
            .open_delay(300)
            .close_delay(150)
            .width(400.0);

        assert_eq!(card.open_delay_ms, 300);
        assert_eq!(card.close_delay_ms, 150);
        assert_eq!(card.width, Some(400.0));
    }
}
