//! Sheet component ported from shadcn/ui
//!
//! A slide-out panel that appears from the edge of the screen.
//!
//! Reference: <https://ui.shadcn.com/docs/components/sheet>

use egui::{Id, Ui, Sense, Color32, Rect, Pos2, Vec2};
use crate::theme::ShadcnTheme;

/// Side from which the sheet slides in
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SheetSide {
    /// Slide in from the top
    Top,
    /// Slide in from the right (default)
    #[default]
    Right,
    /// Slide in from the bottom
    Bottom,
    /// Slide in from the left
    Left,
}

/// Sheet component - a slide-out panel from screen edge
///
/// ## Example
/// ```rust,ignore
/// if ui.button("Open Sheet").clicked() {
///     sheet_open = true;
/// }
///
/// Sheet::new("my_sheet", &mut sheet_open)
///     .side(SheetSide::Right)
///     .title("Edit Profile")
///     .description("Make changes to your profile here.")
///     .show(ui, |ui| {
///         ui.label("Sheet content goes here");
///     });
/// ```
pub struct Sheet<'a> {
    id: Id,
    open: &'a mut bool,
    side: SheetSide,
    title: Option<String>,
    description: Option<String>,
    width: Option<f32>,
    height: Option<f32>,
}

impl<'a> Sheet<'a> {
    /// Create a new sheet
    pub fn new(id: impl std::hash::Hash, open: &'a mut bool) -> Self {
        Self {
            id: Id::new(id),
            open,
            side: SheetSide::Right,
            title: None,
            description: None,
            width: None,
            height: None,
        }
    }

    /// Set the side from which the sheet slides in
    pub fn side(mut self, side: SheetSide) -> Self {
        self.side = side;
        self
    }

    /// Set the sheet title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the sheet description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set custom width (for Left/Right sheets)
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set custom height (for Top/Bottom sheets)
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Show the sheet
    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> Option<R> {
        if !*self.open {
            return None;
        }

        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        #[allow(deprecated)]
        let screen_rect = ui.ctx().screen_rect();
        let mut result = None;

        // Calculate sheet dimensions and position first
        let default_width = 400.0;
        let default_height = 300.0;

        let r = theme.radii.lg;
        let (sheet_rect, corner_radius) = match self.side {
            SheetSide::Right => {
                let width = self.width.unwrap_or(default_width);
                let rect = Rect::from_min_size(
                    Pos2::new(screen_rect.right() - width, screen_rect.top()),
                    Vec2::new(width, screen_rect.height()),
                );
                let radius = egui::CornerRadius { nw: r, sw: r, ne: 0, se: 0 };
                (rect, radius)
            }
            SheetSide::Left => {
                let width = self.width.unwrap_or(default_width);
                let rect = Rect::from_min_size(
                    screen_rect.left_top(),
                    Vec2::new(width, screen_rect.height()),
                );
                let radius = egui::CornerRadius { nw: 0, sw: 0, ne: r, se: r };
                (rect, radius)
            }
            SheetSide::Top => {
                let height = self.height.unwrap_or(default_height);
                let rect = Rect::from_min_size(
                    screen_rect.left_top(),
                    Vec2::new(screen_rect.width(), height),
                );
                let radius = egui::CornerRadius { nw: 0, ne: 0, sw: r, se: r };
                (rect, radius)
            }
            SheetSide::Bottom => {
                let height = self.height.unwrap_or(default_height);
                let rect = Rect::from_min_size(
                    Pos2::new(screen_rect.left(), screen_rect.bottom() - height),
                    Vec2::new(screen_rect.width(), height),
                );
                let radius = egui::CornerRadius { nw: r, ne: r, sw: 0, se: 0 };
                (rect, radius)
            }
        };

        // Draw backdrop overlay - just visual, no interaction capture
        let backdrop_layer = egui::LayerId::new(egui::Order::Middle, self.id.with("backdrop_layer"));
        ui.ctx().layer_painter(backdrop_layer).rect_filled(
            screen_rect,
            0.0,
            Color32::from_black_alpha(128),
        );

        // Track when sheet was opened to avoid closing immediately
        let opened_time_id = self.id.with("opened_time");
        let current_time = ui.ctx().input(|i| i.time);
        let opened_time: f64 = ui.ctx().data(|d| d.get_temp(opened_time_id).unwrap_or(current_time));

        // Store current time as opened time if this is first frame
        if (current_time - opened_time).abs() < 0.001 {
            ui.ctx().data_mut(|d| d.insert_temp(opened_time_id, current_time));
        }

        // Check for clicks outside the sheet panel - only after 100ms delay
        let time_open = current_time - opened_time;
        if time_open > 0.1 {
            let pointer_pos = ui.ctx().input(|i| i.pointer.interact_pos());
            let primary_released = ui.ctx().input(|i| i.pointer.primary_released());
            let clicked_outside = primary_released
                && pointer_pos.map(|p| !sheet_rect.contains(p)).unwrap_or(false);

            if clicked_outside {
                // Clear the opened time tracking
                ui.ctx().data_mut(|d| d.remove::<f64>(opened_time_id));
                *self.open = false;
                return None;
            }
        }

        // Draw the sheet panel
        let sheet_id = self.id.with("panel");
        egui::Area::new(sheet_id)
            .order(egui::Order::Foreground)
            .fixed_pos(sheet_rect.left_top())
            .interactable(true)
            .show(ui.ctx(), |ui| {
                let frame = egui::Frame::NONE
                    .fill(theme.colors.background)
                    .stroke(egui::Stroke::new(1.0, theme.colors.border))
                    .corner_radius(corner_radius)
                    .inner_margin(theme.spacing.lg);

                frame.show(ui, |ui| {
                    ui.set_min_size(sheet_rect.size() - Vec2::splat(theme.spacing.lg * 2.0));
                    ui.set_max_size(sheet_rect.size() - Vec2::splat(theme.spacing.lg * 2.0));

                    // Header with close button
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            // Close button - draw X manually for reliability
                            let btn_size = Vec2::splat(24.0);
                            let (btn_rect, btn_response) = ui.allocate_exact_size(btn_size, Sense::click());

                            let btn_color = if btn_response.hovered() {
                                theme.colors.foreground
                            } else {
                                theme.colors.muted_foreground
                            };

                            // Draw X
                            let painter = ui.painter();
                            let center = btn_rect.center();
                            let half = 6.0;
                            let stroke = egui::Stroke::new(2.0, btn_color);
                            painter.line_segment(
                                [center + Vec2::new(-half, -half), center + Vec2::new(half, half)],
                                stroke,
                            );
                            painter.line_segment(
                                [center + Vec2::new(half, -half), center + Vec2::new(-half, half)],
                                stroke,
                            );

                            if btn_response.clicked() {
                                *self.open = false;
                            }
                            if btn_response.hovered() {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                            }
                        });
                    });

                    // Title
                    if let Some(title) = &self.title {
                        ui.label(
                            egui::RichText::new(title)
                                .size(theme.typography.large().size)
                                .strong()
                                .color(theme.colors.foreground)
                        );
                    }

                    // Description
                    if let Some(description) = &self.description {
                        ui.label(
                            egui::RichText::new(description)
                                .size(theme.typography.small().size)
                                .color(theme.colors.muted_foreground)
                        );
                        ui.add_space(theme.spacing.md);
                    }

                    // Content
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            result = Some(content(ui));
                        });
                });
            });

        // Handle escape key to close
        if ui.ctx().input(|i| i.key_pressed(egui::Key::Escape)) {
            *self.open = false;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheet_creation() {
        let mut open = true;
        let sheet = Sheet::new("test", &mut open)
            .side(SheetSide::Left)
            .title("Test")
            .description("Test desc")
            .width(500.0);

        assert_eq!(sheet.side, SheetSide::Left);
        assert_eq!(sheet.title, Some("Test".to_string()));
        assert_eq!(sheet.description, Some("Test desc".to_string()));
        assert_eq!(sheet.width, Some(500.0));
    }
}
