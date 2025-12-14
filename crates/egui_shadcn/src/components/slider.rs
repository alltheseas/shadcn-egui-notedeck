//! Slider component matching shadcn/ui exactly
//!
//! Provides a slider for numeric value selection.
//!
//! Reference: <https://ui.shadcn.com/docs/components/slider>

use egui::{Response, Sense, Ui, Widget};
use crate::theme::ShadcnTheme;

/// Slider widget matching shadcn/ui design
///
/// ## Example
/// ```rust,ignore
/// let mut value = 50.0;
/// Slider::new(&mut value, 0.0..=100.0).ui(ui);
/// ```
pub struct Slider<'a> {
    value: &'a mut f32,
    range: std::ops::RangeInclusive<f32>,
    step: Option<f32>,
    enabled: bool,
}

impl<'a> Slider<'a> {
    /// Create a new slider
    pub fn new(value: &'a mut f32, range: std::ops::RangeInclusive<f32>) -> Self {
        Self {
            value,
            range,
            step: None,
            enabled: true,
        }
    }

    /// Set the step size
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Set whether enabled
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl<'a> Widget for Slider<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let touch_target = 44.0; // Apple HIG minimum touch target
        let track_height = 4.0;
        let thumb_radius = 10.0; // Visual thumb radius (20px diameter)

        let desired_width = ui.available_width().max(200.0);
        let desired_size = egui::vec2(desired_width, touch_target);

        let (rect, mut response) = ui.allocate_exact_size(
            desired_size,
            if self.enabled { Sense::click_and_drag() } else { Sense::hover() },
        );

        // Handle interaction
        if self.enabled && (response.dragged() || response.clicked()) {
            if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                let normalized = ((pointer_pos.x - rect.min.x) / rect.width()).clamp(0.0, 1.0);
                let range_size = self.range.end() - self.range.start();
                let mut new_value = self.range.start() + normalized * range_size;

                if let Some(step) = self.step {
                    new_value = (new_value / step).round() * step;
                }

                *self.value = new_value.clamp(*self.range.start(), *self.range.end());
                response.mark_changed();
            }
        }

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Calculate thumb position
            let range_size = self.range.end() - self.range.start();
            let normalized = (*self.value - self.range.start()) / range_size;
            let thumb_x = rect.min.x + normalized * rect.width();

            // Track rect (centered vertically)
            let track_rect = egui::Rect::from_min_size(
                egui::pos2(rect.min.x, rect.center().y - track_height / 2.0),
                egui::vec2(rect.width(), track_height),
            );

            // Draw background track
            painter.rect_filled(
                track_rect,
                theme.radii.progress(),
                theme.colors.secondary,
            );

            // Draw filled track (up to thumb)
            let filled_rect = egui::Rect::from_min_size(
                track_rect.min,
                egui::vec2((thumb_x - rect.min.x).max(0.0), track_height),
            );
            painter.rect_filled(
                filled_rect,
                theme.radii.progress(),
                theme.colors.primary,
            );

            // Draw thumb
            let thumb_center = egui::pos2(thumb_x, rect.center().y);
            let hovered = response.hovered();

            painter.circle(
                thumb_center,
                thumb_radius,
                theme.colors.background,
                egui::Stroke::new(
                    2.0,
                    if hovered {
                        theme.colors.ring
                    } else {
                        theme.colors.primary
                    },
                ),
            );
        }

        if !self.enabled {
            response.on_disabled_hover_text("Disabled")
        } else {
            response
        }
    }
}
