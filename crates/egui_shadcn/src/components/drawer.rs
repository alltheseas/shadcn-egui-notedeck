//! Drawer component ported from shadcn/ui
//!
//! A drawer panel that slides in from an edge with smooth animations,
//! typically used for mobile navigation or additional content panels.
//!
//! Reference: <https://ui.shadcn.com/docs/components/drawer>

use egui::{Id, Ui, Color32, Rect, Pos2, Vec2, Sense};
use crate::animation::SlideAnimation;
use crate::theme::ShadcnTheme;

/// Side from which the drawer slides in
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerSide {
    /// Slide in from the top
    Top,
    /// Slide in from the right
    Right,
    /// Slide in from the bottom (default)
    #[default]
    Bottom,
    /// Slide in from the left
    Left,
}

/// Drawer component - a slide-in panel with handle
///
/// ## Example
/// ```rust,ignore
/// if ui.button("Open Drawer").clicked() {
///     drawer_open = true;
/// }
///
/// Drawer::new("my_drawer", &mut drawer_open)
///     .side(DrawerSide::Bottom)
///     .title("Settings")
///     .show(ui, |ui| {
///         ui.label("Drawer content");
///     });
/// ```
pub struct Drawer<'a> {
    id: Id,
    open: &'a mut bool,
    side: DrawerSide,
    title: Option<String>,
    description: Option<String>,
    show_handle: bool,
    size: Option<f32>,
}

impl<'a> Drawer<'a> {
    /// Create a new drawer
    pub fn new(id: impl std::hash::Hash, open: &'a mut bool) -> Self {
        Self {
            id: Id::new(id),
            open,
            side: DrawerSide::Bottom,
            title: None,
            description: None,
            show_handle: true,
            size: None,
        }
    }

    /// Set the side from which the drawer slides in
    pub fn side(mut self, side: DrawerSide) -> Self {
        self.side = side;
        self
    }

    /// Set the drawer title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the drawer description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Whether to show the drag handle (default: true)
    pub fn show_handle(mut self, show: bool) -> Self {
        self.show_handle = show;
        self
    }

    /// Set custom size (width for Left/Right, height for Top/Bottom)
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Show the drawer
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

        // Calculate drawer dimensions
        let default_size = match self.side {
            DrawerSide::Top | DrawerSide::Bottom => 340.0,
            DrawerSide::Left | DrawerSide::Right => 380.0,
        };
        let size = self.size.unwrap_or(default_size);
        let r = theme.radii.xl;

        // Calculate the offset translation based on animation progress
        let offset_amount = anim.offset;

        let (drawer_rect, corner_radius) = match self.side {
            DrawerSide::Bottom => {
                let base_y = screen_rect.bottom() - size;
                let translate_y = size * offset_amount;
                let rect = Rect::from_min_size(
                    Pos2::new(screen_rect.left(), base_y + translate_y),
                    Vec2::new(screen_rect.width(), size),
                );
                (rect, egui::CornerRadius { nw: r, ne: r, sw: 0, se: 0 })
            }
            DrawerSide::Top => {
                let translate_y = -size * offset_amount;
                let rect = Rect::from_min_size(
                    Pos2::new(screen_rect.left(), screen_rect.top() + translate_y),
                    Vec2::new(screen_rect.width(), size),
                );
                (rect, egui::CornerRadius { nw: 0, ne: 0, sw: r, se: r })
            }
            DrawerSide::Left => {
                let translate_x = -size * offset_amount;
                let rect = Rect::from_min_size(
                    Pos2::new(screen_rect.left() + translate_x, screen_rect.top()),
                    Vec2::new(size, screen_rect.height()),
                );
                (rect, egui::CornerRadius { nw: 0, ne: r, sw: 0, se: r })
            }
            DrawerSide::Right => {
                let base_x = screen_rect.right() - size;
                let translate_x = size * offset_amount;
                let rect = Rect::from_min_size(
                    Pos2::new(base_x + translate_x, screen_rect.top()),
                    Vec2::new(size, screen_rect.height()),
                );
                (rect, egui::CornerRadius { nw: r, ne: 0, sw: r, se: 0 })
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

        // Check for clicks outside the drawer panel (only when mostly open)
        if offset_amount < 0.3 {
            let pointer_pos = ui.ctx().input(|i| i.pointer.latest_pos());
            let primary_pressed = ui.ctx().input(|i| i.pointer.primary_pressed());

            // Calculate the visible drawer rect (accounting for animation)
            let visible_rect = match self.side {
                DrawerSide::Bottom => Rect::from_min_size(
                    Pos2::new(screen_rect.left(), screen_rect.bottom() - size * (1.0 - offset_amount)),
                    Vec2::new(screen_rect.width(), size),
                ),
                DrawerSide::Top => Rect::from_min_size(
                    screen_rect.left_top(),
                    Vec2::new(screen_rect.width(), size * (1.0 - offset_amount)),
                ),
                DrawerSide::Left => Rect::from_min_size(
                    screen_rect.left_top(),
                    Vec2::new(size * (1.0 - offset_amount), screen_rect.height()),
                ),
                DrawerSide::Right => Rect::from_min_size(
                    Pos2::new(screen_rect.right() - size * (1.0 - offset_amount), screen_rect.top()),
                    Vec2::new(size, screen_rect.height()),
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

        // Draw the drawer panel
        let drawer_id = self.id.with("panel");
        egui::Area::new(drawer_id)
            .order(egui::Order::Foreground)
            .fixed_pos(drawer_rect.left_top())
            .interactable(true)
            .show(ui.ctx(), |ui| {
                let frame = egui::Frame::NONE
                    .fill(theme.colors.background)
                    .stroke(egui::Stroke::new(1.0, theme.colors.border))
                    .corner_radius(corner_radius);

                frame.show(ui, |ui| {
                    ui.set_min_size(drawer_rect.size());
                    ui.set_max_size(drawer_rect.size());

                    ui.vertical(|ui| {
                        // Draw handle for bottom/top drawers
                        if self.show_handle && matches!(self.side, DrawerSide::Bottom | DrawerSide::Top) {
                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                let handle_width = 48.0;
                                let handle_height = 4.0;
                                let available_width = ui.available_width();
                                ui.add_space((available_width - handle_width) / 2.0);

                                let (handle_rect, handle_response) = ui.allocate_exact_size(
                                    Vec2::new(handle_width, handle_height),
                                    Sense::click(),
                                );

                                let handle_color = if handle_response.hovered() {
                                    theme.colors.muted_foreground
                                } else {
                                    theme.colors.border
                                };

                                ui.painter().rect_filled(
                                    handle_rect,
                                    handle_height / 2.0,
                                    handle_color,
                                );

                                if handle_response.clicked() {
                                    *self.open = false;
                                    ui.ctx().request_repaint();
                                }
                                if handle_response.hovered() {
                                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                }
                            });
                            ui.add_space(8.0);
                        }

                        // Content area with padding
                        ui.add_space(theme.spacing.md);
                        ui.horizontal(|ui| {
                            ui.add_space(theme.spacing.lg);
                            ui.vertical(|ui| {
                                let content_width = drawer_rect.width() - theme.spacing.lg * 2.0;
                                ui.set_max_width(content_width);

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
                                result = Some(content(ui));
                            });
                            ui.add_space(theme.spacing.lg);
                        });
                    });
                });
            });

        // Handle escape key
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
    fn test_drawer_creation() {
        let mut open = true;
        let drawer = Drawer::new("test", &mut open)
            .side(DrawerSide::Bottom)
            .title("Test")
            .show_handle(false)
            .size(400.0);

        assert_eq!(drawer.side, DrawerSide::Bottom);
        assert_eq!(drawer.title, Some("Test".to_string()));
        assert!(!drawer.show_handle);
        assert_eq!(drawer.size, Some(400.0));
    }
}
