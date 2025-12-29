//! Carousel component ported from shadcn/ui
//!
//! A carousel with motion and swipe gestures for cycling through content.
//!
//! Reference: <https://ui.shadcn.com/docs/components/carousel>

use egui::{Id, Response, Ui, Sense, Vec2, Pos2};
use crate::theme::ShadcnTheme;

/// Carousel orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CarouselOrientation {
    /// Horizontal carousel (default)
    #[default]
    Horizontal,
    /// Vertical carousel
    Vertical,
}

/// Carousel component for cycling through content
///
/// ## Example
/// ```rust,ignore
/// let items = vec!["Item 1", "Item 2", "Item 3"];
/// let mut current = 0;
///
/// Carousel::new("my_carousel", &mut current, items.len())
///     .item_width(200.0)
///     .show(ui, |ui, index| {
///         ui.label(&items[index]);
///     });
/// ```
pub struct Carousel<'a> {
    id: Id,
    current: &'a mut usize,
    item_count: usize,
    orientation: CarouselOrientation,
    item_size: Vec2,
    show_buttons: bool,
    show_dots: bool,
    loop_items: bool,
}

impl<'a> Carousel<'a> {
    /// Create a new carousel
    pub fn new(id: impl std::hash::Hash, current: &'a mut usize, item_count: usize) -> Self {
        Self {
            id: Id::new(id),
            current,
            item_count,
            orientation: CarouselOrientation::Horizontal,
            item_size: Vec2::new(280.0, 200.0),
            show_buttons: true,
            show_dots: true,
            loop_items: false,
        }
    }

    /// Set the carousel orientation
    pub fn orientation(mut self, orientation: CarouselOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the item size
    pub fn item_size(mut self, size: Vec2) -> Self {
        self.item_size = size;
        self
    }

    /// Set the item width (keeps default height)
    pub fn item_width(mut self, width: f32) -> Self {
        self.item_size.x = width;
        self
    }

    /// Set the item height (keeps default width)
    pub fn item_height(mut self, height: f32) -> Self {
        self.item_size.y = height;
        self
    }

    /// Show/hide navigation buttons
    pub fn show_buttons(mut self, show: bool) -> Self {
        self.show_buttons = show;
        self
    }

    /// Show/hide dot indicators
    pub fn show_dots(mut self, show: bool) -> Self {
        self.show_dots = show;
        self
    }

    /// Enable looping (wrap from last to first)
    pub fn loop_items(mut self, loop_items: bool) -> Self {
        self.loop_items = loop_items;
        self
    }

    /// Show the carousel with a content builder
    pub fn show<F>(self, ui: &mut Ui, mut content: F) -> Response
    where
        F: FnMut(&mut Ui, usize),
    {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let button_size = 40.0;
        let button_margin = 8.0;

        // Calculate total size
        let total_width = if self.show_buttons {
            self.item_size.x + (button_size + button_margin) * 2.0
        } else {
            self.item_size.x
        };

        let dots_height = if self.show_dots { 24.0 } else { 0.0 };
        let total_height = self.item_size.y + dots_height;

        let response = ui.allocate_ui(Vec2::new(total_width, total_height), |ui| {
            match self.orientation {
                CarouselOrientation::Horizontal => {
                    ui.horizontal(|ui| {
                        // Previous button - vertically centered with content
                        if self.show_buttons {
                            ui.allocate_ui_with_layout(
                                Vec2::new(button_size, self.item_size.y),
                                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                                |ui| {
                                    let can_go_prev = self.loop_items || *self.current > 0;
                                    let prev_response = self.draw_nav_button(
                                        ui, &theme, button_size, true, can_go_prev
                                    );
                                    if prev_response.clicked() && can_go_prev {
                                        if *self.current > 0 {
                                            *self.current -= 1;
                                        } else if self.loop_items && self.item_count > 0 {
                                            *self.current = self.item_count - 1;
                                        }
                                    }
                                }
                            );
                            ui.add_space(button_margin);
                        }

                        // Content area
                        ui.vertical(|ui| {
                            // Item frame
                            egui::Frame::NONE
                                .stroke(egui::Stroke::new(1.0, theme.colors.border))
                                .corner_radius(theme.radii.lg)
                                .inner_margin(egui::Margin::same(16))
                                .show(ui, |ui| {
                                    ui.set_min_size(self.item_size - Vec2::splat(32.0));
                                    if self.item_count > 0 && *self.current < self.item_count {
                                        content(ui, *self.current);
                                    }
                                });

                            // Dot indicators
                            if self.show_dots && self.item_count > 1 {
                                ui.add_space(8.0);
                                ui.horizontal(|ui| {
                                    ui.add_space((self.item_size.x - (self.item_count as f32 * 12.0)) / 2.0);
                                    for i in 0..self.item_count {
                                        let is_active = i == *self.current;
                                        let dot_response = self.draw_dot(ui, &theme, is_active);
                                        if dot_response.clicked() {
                                            *self.current = i;
                                        }
                                    }
                                });
                            }
                        });

                        // Next button - vertically centered with content
                        if self.show_buttons {
                            ui.add_space(button_margin);
                            ui.allocate_ui_with_layout(
                                Vec2::new(button_size, self.item_size.y),
                                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                                |ui| {
                                    let can_go_next = self.loop_items || *self.current < self.item_count.saturating_sub(1);
                                    let next_response = self.draw_nav_button(
                                        ui, &theme, button_size, false, can_go_next
                                    );
                                    if next_response.clicked() && can_go_next {
                                        if *self.current < self.item_count - 1 {
                                            *self.current += 1;
                                        } else if self.loop_items {
                                            *self.current = 0;
                                        }
                                    }
                                }
                            );
                        }
                    });
                }
                CarouselOrientation::Vertical => {
                    ui.vertical(|ui| {
                        // Previous button (up)
                        if self.show_buttons {
                            let can_go_prev = self.loop_items || *self.current > 0;
                            ui.horizontal(|ui| {
                                ui.add_space((self.item_size.x - button_size) / 2.0);
                                let prev_response = self.draw_nav_button_vertical(
                                    ui, &theme, button_size, true, can_go_prev
                                );
                                if prev_response.clicked() && can_go_prev {
                                    if *self.current > 0 {
                                        *self.current -= 1;
                                    } else if self.loop_items && self.item_count > 0 {
                                        *self.current = self.item_count - 1;
                                    }
                                }
                            });
                            ui.add_space(button_margin);
                        }

                        // Content area
                        egui::Frame::NONE
                            .stroke(egui::Stroke::new(1.0, theme.colors.border))
                            .corner_radius(theme.radii.lg)
                            .inner_margin(egui::Margin::same(16))
                            .show(ui, |ui| {
                                ui.set_min_size(self.item_size - Vec2::splat(32.0));
                                if self.item_count > 0 && *self.current < self.item_count {
                                    content(ui, *self.current);
                                }
                            });

                        // Next button (down)
                        if self.show_buttons {
                            ui.add_space(button_margin);
                            let can_go_next = self.loop_items || *self.current < self.item_count.saturating_sub(1);
                            ui.horizontal(|ui| {
                                ui.add_space((self.item_size.x - button_size) / 2.0);
                                let next_response = self.draw_nav_button_vertical(
                                    ui, &theme, button_size, false, can_go_next
                                );
                                if next_response.clicked() && can_go_next {
                                    if *self.current < self.item_count - 1 {
                                        *self.current += 1;
                                    } else if self.loop_items {
                                        *self.current = 0;
                                    }
                                }
                            });
                        }
                    });
                }
            }
        });

        response.response
    }

    fn draw_nav_button(
        &self,
        ui: &mut Ui,
        theme: &ShadcnTheme,
        size: f32,
        is_prev: bool,
        enabled: bool,
    ) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());

        if ui.is_rect_visible(rect) {
            let is_hovered = response.hovered() && enabled;

            // Background (outline style)
            let bg_color = if is_hovered {
                theme.colors.accent
            } else {
                egui::Color32::TRANSPARENT
            };

            ui.painter().rect(
                rect,
                theme.radii.full(), // Circular button
                bg_color,
                egui::Stroke::new(1.0, theme.colors.border),
                egui::StrokeKind::Inside,
            );

            // Chevron
            let cx = rect.center().x;
            let cy = rect.center().y;
            let chevron_size = 5.0;
            let color = if enabled {
                if is_hovered {
                    theme.colors.accent_foreground
                } else {
                    theme.colors.foreground
                }
            } else {
                theme.colors.muted_foreground.gamma_multiply(0.5)
            };
            let stroke = egui::Stroke::new(2.0, color);

            if is_prev {
                // Left chevron (<)
                ui.painter().line_segment(
                    [Pos2::new(cx + chevron_size * 0.3, cy - chevron_size),
                     Pos2::new(cx - chevron_size * 0.5, cy)],
                    stroke,
                );
                ui.painter().line_segment(
                    [Pos2::new(cx - chevron_size * 0.5, cy),
                     Pos2::new(cx + chevron_size * 0.3, cy + chevron_size)],
                    stroke,
                );
            } else {
                // Right chevron (>)
                ui.painter().line_segment(
                    [Pos2::new(cx - chevron_size * 0.3, cy - chevron_size),
                     Pos2::new(cx + chevron_size * 0.5, cy)],
                    stroke,
                );
                ui.painter().line_segment(
                    [Pos2::new(cx + chevron_size * 0.5, cy),
                     Pos2::new(cx - chevron_size * 0.3, cy + chevron_size)],
                    stroke,
                );
            }
        }

        if response.hovered() && enabled {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        response
    }

    fn draw_nav_button_vertical(
        &self,
        ui: &mut Ui,
        theme: &ShadcnTheme,
        size: f32,
        is_up: bool,
        enabled: bool,
    ) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());

        if ui.is_rect_visible(rect) {
            let is_hovered = response.hovered() && enabled;

            let bg_color = if is_hovered {
                theme.colors.accent
            } else {
                egui::Color32::TRANSPARENT
            };

            ui.painter().rect(
                rect,
                theme.radii.full(),
                bg_color,
                egui::Stroke::new(1.0, theme.colors.border),
                egui::StrokeKind::Inside,
            );

            let cx = rect.center().x;
            let cy = rect.center().y;
            let chevron_size = 5.0;
            let color = if enabled {
                if is_hovered {
                    theme.colors.accent_foreground
                } else {
                    theme.colors.foreground
                }
            } else {
                theme.colors.muted_foreground.gamma_multiply(0.5)
            };
            let stroke = egui::Stroke::new(2.0, color);

            if is_up {
                // Up chevron (^)
                ui.painter().line_segment(
                    [Pos2::new(cx - chevron_size, cy + chevron_size * 0.3),
                     Pos2::new(cx, cy - chevron_size * 0.5)],
                    stroke,
                );
                ui.painter().line_segment(
                    [Pos2::new(cx, cy - chevron_size * 0.5),
                     Pos2::new(cx + chevron_size, cy + chevron_size * 0.3)],
                    stroke,
                );
            } else {
                // Down chevron (v)
                ui.painter().line_segment(
                    [Pos2::new(cx - chevron_size, cy - chevron_size * 0.3),
                     Pos2::new(cx, cy + chevron_size * 0.5)],
                    stroke,
                );
                ui.painter().line_segment(
                    [Pos2::new(cx, cy + chevron_size * 0.5),
                     Pos2::new(cx + chevron_size, cy - chevron_size * 0.3)],
                    stroke,
                );
            }
        }

        if response.hovered() && enabled {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        response
    }

    fn draw_dot(&self, ui: &mut Ui, theme: &ShadcnTheme, is_active: bool) -> Response {
        let size = 8.0;
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(size + 4.0), Sense::click());

        if ui.is_rect_visible(rect) {
            let color = if is_active {
                theme.colors.primary
            } else if response.hovered() {
                theme.colors.muted_foreground
            } else {
                theme.colors.muted_foreground.gamma_multiply(0.5)
            };

            let dot_rect = egui::Rect::from_center_size(rect.center(), Vec2::splat(size));
            ui.painter().rect_filled(dot_rect, size / 2.0, color);
        }

        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carousel_creation() {
        let mut current = 0;
        let carousel = Carousel::new("test", &mut current, 5)
            .item_width(300.0)
            .loop_items(true);

        assert_eq!(carousel.item_count, 5);
        assert!(carousel.loop_items);
    }
}
