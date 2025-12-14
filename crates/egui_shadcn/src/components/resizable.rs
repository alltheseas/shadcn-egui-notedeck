//! Resizable panels component ported from shadcn/ui
//!
//! A component for creating resizable panel layouts.
//!
//! Reference: <https://ui.shadcn.com/docs/components/resizable>

use egui::{Id, Response, Ui, Sense, Vec2, Pos2, Rect};
use crate::theme::ShadcnTheme;

/// Resizable panel direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResizableDirection {
    /// Horizontal panels (side by side)
    #[default]
    Horizontal,
    /// Vertical panels (stacked)
    Vertical,
}

/// A resizable panel group for split pane layouts
///
/// ## Example
/// ```rust,ignore
/// let mut split = 0.5; // 50% split
///
/// ResizablePanelGroup::new("my_panels", &mut split)
///     .direction(ResizableDirection::Horizontal)
///     .min_size(100.0)
///     .show(ui, |ui, panel| {
///         match panel {
///             0 => ui.label("Left panel"),
///             1 => ui.label("Right panel"),
///             _ => {}
///         }
///     });
/// ```
pub struct ResizablePanelGroup<'a> {
    id: Id,
    split: &'a mut f32,
    direction: ResizableDirection,
    min_size: f32,
    handle_size: f32,
    show_handle: bool,
}

impl<'a> ResizablePanelGroup<'a> {
    /// Create a new resizable panel group
    ///
    /// - `id`: Unique identifier
    /// - `split`: Ratio of first panel (0.0 to 1.0)
    pub fn new(id: impl std::hash::Hash, split: &'a mut f32) -> Self {
        Self {
            id: Id::new(id),
            split,
            direction: ResizableDirection::Horizontal,
            min_size: 50.0,
            handle_size: 8.0,
            show_handle: true,
        }
    }

    /// Set the panel direction
    pub fn direction(mut self, direction: ResizableDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set the minimum panel size in pixels
    pub fn min_size(mut self, min_size: f32) -> Self {
        self.min_size = min_size;
        self
    }

    /// Set the handle/divider size
    pub fn handle_size(mut self, size: f32) -> Self {
        self.handle_size = size;
        self
    }

    /// Show/hide the resize handle visual
    pub fn show_handle(mut self, show: bool) -> Self {
        self.show_handle = show;
        self
    }

    /// Show the resizable panels with a content builder
    ///
    /// The content builder receives `(ui, panel_index)` where:
    /// - `panel_index = 0` for the first panel
    /// - `panel_index = 1` for the second panel
    pub fn show<F>(self, ui: &mut Ui, mut content: F) -> Response
    where
        F: FnMut(&mut Ui, usize),
    {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let available = ui.available_size();
        let (response, handle_response) = match self.direction {
            ResizableDirection::Horizontal => {
                self.show_horizontal(ui, &theme, available, &mut content)
            }
            ResizableDirection::Vertical => {
                self.show_vertical(ui, &theme, available, &mut content)
            }
        };

        // Handle dragging
        if let Some(handle) = handle_response {
            if handle.dragged() {
                if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                    let (total_size, pos_component) = match self.direction {
                        ResizableDirection::Horizontal => (available.x, pointer_pos.x - handle.rect.center().x + available.x * *self.split),
                        ResizableDirection::Vertical => (available.y, pointer_pos.y - handle.rect.center().y + available.y * *self.split),
                    };

                    // Calculate new split ratio
                    let new_split = pos_component / total_size;

                    // Clamp to min sizes
                    let min_ratio = self.min_size / total_size;
                    *self.split = new_split.clamp(min_ratio, 1.0 - min_ratio);
                }
            }

            // Update cursor
            if handle.hovered() || handle.dragged() {
                let cursor = match self.direction {
                    ResizableDirection::Horizontal => egui::CursorIcon::ResizeHorizontal,
                    ResizableDirection::Vertical => egui::CursorIcon::ResizeVertical,
                };
                ui.ctx().set_cursor_icon(cursor);
            }
        }

        response
    }

    fn show_horizontal<F>(
        &self,
        ui: &mut Ui,
        theme: &ShadcnTheme,
        available: Vec2,
        content: &mut F,
    ) -> (Response, Option<Response>)
    where
        F: FnMut(&mut Ui, usize),
    {
        let first_width = (available.x - self.handle_size) * *self.split;
        let second_width = available.x - first_width - self.handle_size;

        let mut handle_response = None;

        let response = ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            // First panel
            ui.allocate_ui(Vec2::new(first_width, available.y), |ui| {
                egui::Frame::NONE
                    .show(ui, |ui| {
                        content(ui, 0);
                    });
            });

            // Handle
            let (handle_rect, handle_resp) = ui.allocate_exact_size(
                Vec2::new(self.handle_size, available.y),
                Sense::drag(),
            );

            if ui.is_rect_visible(handle_rect) && self.show_handle {
                let is_active = handle_resp.hovered() || handle_resp.dragged();
                let handle_color = if is_active {
                    theme.colors.primary
                } else {
                    theme.colors.border
                };

                // Draw handle line
                let center_x = handle_rect.center().x;
                ui.painter().line_segment(
                    [
                        Pos2::new(center_x, handle_rect.min.y + 4.0),
                        Pos2::new(center_x, handle_rect.max.y - 4.0),
                    ],
                    egui::Stroke::new(2.0, handle_color),
                );

                // Draw grip dots
                let grip_y = handle_rect.center().y;
                for offset in [-8.0, 0.0, 8.0] {
                    ui.painter().circle_filled(
                        Pos2::new(center_x, grip_y + offset),
                        2.0,
                        handle_color,
                    );
                }
            }

            handle_response = Some(handle_resp);

            // Second panel
            ui.allocate_ui(Vec2::new(second_width, available.y), |ui| {
                egui::Frame::NONE
                    .show(ui, |ui| {
                        content(ui, 1);
                    });
            });
        });

        (response.response, handle_response)
    }

    fn show_vertical<F>(
        &self,
        ui: &mut Ui,
        theme: &ShadcnTheme,
        available: Vec2,
        content: &mut F,
    ) -> (Response, Option<Response>)
    where
        F: FnMut(&mut Ui, usize),
    {
        let first_height = (available.y - self.handle_size) * *self.split;
        let second_height = available.y - first_height - self.handle_size;

        let mut handle_response = None;

        let response = ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;

            // First panel
            ui.allocate_ui(Vec2::new(available.x, first_height), |ui| {
                egui::Frame::NONE
                    .show(ui, |ui| {
                        content(ui, 0);
                    });
            });

            // Handle
            let (handle_rect, handle_resp) = ui.allocate_exact_size(
                Vec2::new(available.x, self.handle_size),
                Sense::drag(),
            );

            if ui.is_rect_visible(handle_rect) && self.show_handle {
                let is_active = handle_resp.hovered() || handle_resp.dragged();
                let handle_color = if is_active {
                    theme.colors.primary
                } else {
                    theme.colors.border
                };

                // Draw handle line
                let center_y = handle_rect.center().y;
                ui.painter().line_segment(
                    [
                        Pos2::new(handle_rect.min.x + 4.0, center_y),
                        Pos2::new(handle_rect.max.x - 4.0, center_y),
                    ],
                    egui::Stroke::new(2.0, handle_color),
                );

                // Draw grip dots
                let grip_x = handle_rect.center().x;
                for offset in [-8.0, 0.0, 8.0] {
                    ui.painter().circle_filled(
                        Pos2::new(grip_x + offset, center_y),
                        2.0,
                        handle_color,
                    );
                }
            }

            handle_response = Some(handle_resp);

            // Second panel
            ui.allocate_ui(Vec2::new(available.x, second_height), |ui| {
                egui::Frame::NONE
                    .show(ui, |ui| {
                        content(ui, 1);
                    });
            });
        });

        (response.response, handle_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resizable_creation() {
        let mut split = 0.5;
        let panels = ResizablePanelGroup::new("test", &mut split)
            .direction(ResizableDirection::Vertical)
            .min_size(100.0);

        assert_eq!(panels.direction, ResizableDirection::Vertical);
        assert_eq!(panels.min_size, 100.0);
    }
}
