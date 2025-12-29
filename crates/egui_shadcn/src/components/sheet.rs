//! Sheet component ported from shadcn/ui
//!
//! A slide-out panel that appears from the edge of the screen with smooth animations.
//!
//! Reference: <https://ui.shadcn.com/docs/components/sheet>

use egui::{Id, Ui, Sense, Color32, Rect, Pos2, Vec2};
use crate::animation::SlideAnimation;
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
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        // Load animation state
        let anim_id = self.id.with("anim");
        let mut anim = SlideAnimation::load(ui.ctx(), anim_id);

        // Determine target: 0.0 = fully visible, 1.0 = fully hidden
        let target = if *self.open { 0.0 } else { 1.0 };

        // Start animation if we're not at target
        if (anim.offset - target).abs() > 0.001 {
            anim.opening = *self.open;
            anim.animating = true;
        }

        // Update animation
        anim.update(ui.ctx());

        // Store animation state
        anim.store(ui.ctx(), anim_id);

        // Don't render if fully closed
        if !*self.open && anim.offset >= 0.999 {
            return None;
        }

        #[allow(deprecated)]
        let screen_rect = ui.ctx().screen_rect();
        let mut result = None;

        // Calculate sheet dimensions
        let default_width = 400.0;
        let default_height = 300.0;

        let r = theme.radii.lg;
        let sheet_width = self.width.unwrap_or(default_width);
        let sheet_height = self.height.unwrap_or(default_height);

        // Calculate the offset translation based on animation progress
        let offset_amount = anim.offset;

        let (sheet_rect, corner_radius, translate) = match self.side {
            SheetSide::Right => {
                let base_x = screen_rect.right() - sheet_width;
                let translate_x = sheet_width * offset_amount;
                let rect = Rect::from_min_size(
                    Pos2::new(base_x + translate_x, screen_rect.top()),
                    Vec2::new(sheet_width, screen_rect.height()),
                );
                let radius = egui::CornerRadius { nw: r, sw: r, ne: 0, se: 0 };
                (rect, radius, Vec2::new(translate_x, 0.0))
            }
            SheetSide::Left => {
                let translate_x = -sheet_width * offset_amount;
                let rect = Rect::from_min_size(
                    Pos2::new(screen_rect.left() + translate_x, screen_rect.top()),
                    Vec2::new(sheet_width, screen_rect.height()),
                );
                let radius = egui::CornerRadius { nw: 0, sw: 0, ne: r, se: r };
                (rect, radius, Vec2::new(translate_x, 0.0))
            }
            SheetSide::Top => {
                let translate_y = -sheet_height * offset_amount;
                let rect = Rect::from_min_size(
                    Pos2::new(screen_rect.left(), screen_rect.top() + translate_y),
                    Vec2::new(screen_rect.width(), sheet_height),
                );
                let radius = egui::CornerRadius { nw: 0, ne: 0, sw: r, se: r };
                (rect, radius, Vec2::new(0.0, translate_y))
            }
            SheetSide::Bottom => {
                let base_y = screen_rect.bottom() - sheet_height;
                let translate_y = sheet_height * offset_amount;
                let rect = Rect::from_min_size(
                    Pos2::new(screen_rect.left(), base_y + translate_y),
                    Vec2::new(screen_rect.width(), sheet_height),
                );
                let radius = egui::CornerRadius { nw: r, ne: r, sw: 0, se: 0 };
                (rect, radius, Vec2::new(0.0, translate_y))
            }
        };

        // Draw backdrop overlay with animated alpha
        let backdrop_alpha = ((1.0 - offset_amount) * 128.0) as u8;
        let backdrop_layer = egui::LayerId::new(egui::Order::Middle, self.id.with("backdrop_layer"));
        ui.ctx().layer_painter(backdrop_layer).rect_filled(
            screen_rect,
            0.0,
            Color32::from_black_alpha(backdrop_alpha),
        );

        // Check for clicks outside the sheet panel (only when mostly open)
        if offset_amount < 0.3 {
            let pointer_pos = ui.ctx().input(|i| i.pointer.latest_pos());
            let primary_pressed = ui.ctx().input(|i| i.pointer.primary_pressed());

            // Calculate the visible sheet rect (accounting for animation)
            let visible_rect = match self.side {
                SheetSide::Right => Rect::from_min_size(
                    Pos2::new(screen_rect.right() - sheet_width * (1.0 - offset_amount), screen_rect.top()),
                    Vec2::new(sheet_width, screen_rect.height()),
                ),
                SheetSide::Left => Rect::from_min_size(
                    screen_rect.left_top(),
                    Vec2::new(sheet_width * (1.0 - offset_amount), screen_rect.height()),
                ),
                SheetSide::Top => Rect::from_min_size(
                    screen_rect.left_top(),
                    Vec2::new(screen_rect.width(), sheet_height * (1.0 - offset_amount)),
                ),
                SheetSide::Bottom => Rect::from_min_size(
                    Pos2::new(screen_rect.left(), screen_rect.bottom() - sheet_height * (1.0 - offset_amount)),
                    Vec2::new(screen_rect.width(), sheet_height),
                ),
            };

            let clicked_outside = primary_pressed
                && pointer_pos.map(|p| !visible_rect.contains(p)).unwrap_or(false);

            if clicked_outside {
                *self.open = false;
                // Re-store animation state to start closing immediately
                anim.opening = false;
                anim.animating = true;
                anim.store(ui.ctx(), anim_id);
                ui.ctx().request_repaint();
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
                    ui.set_min_size(Vec2::new(
                        sheet_width - theme.spacing.lg * 2.0,
                        sheet_rect.height() - theme.spacing.lg * 2.0,
                    ));
                    ui.set_max_size(Vec2::new(
                        sheet_width - theme.spacing.lg * 2.0,
                        sheet_rect.height() - theme.spacing.lg * 2.0,
                    ));

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
                                ui.ctx().request_repaint();
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
            ui.ctx().request_repaint();
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
