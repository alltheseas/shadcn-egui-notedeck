//! Drawer component ported from shadcn/ui
//!
//! A drawer panel that slides in from an edge, typically used for
//! mobile navigation or additional content panels.
//!
//! Reference: <https://ui.shadcn.com/docs/components/drawer>

use egui::{Id, Ui, Color32, Rect, Pos2, Vec2};
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

        // Calculate drawer dimensions first
        let default_size = match self.side {
            DrawerSide::Top | DrawerSide::Bottom => 340.0,
            DrawerSide::Left | DrawerSide::Right => 380.0,
        };
        let size = self.size.unwrap_or(default_size);
        let r = theme.radii.xl;

        let (drawer_rect, corner_radius) = match self.side {
            DrawerSide::Bottom => {
                let rect = Rect::from_min_size(
                    Pos2::new(screen_rect.left(), screen_rect.bottom() - size),
                    Vec2::new(screen_rect.width(), size),
                );
                (rect, egui::CornerRadius { nw: r, ne: r, sw: 0, se: 0 })
            }
            DrawerSide::Top => {
                let rect = Rect::from_min_size(
                    screen_rect.left_top(),
                    Vec2::new(screen_rect.width(), size),
                );
                (rect, egui::CornerRadius { nw: 0, ne: 0, sw: r, se: r })
            }
            DrawerSide::Left => {
                let rect = Rect::from_min_size(
                    screen_rect.left_top(),
                    Vec2::new(size, screen_rect.height()),
                );
                (rect, egui::CornerRadius { nw: 0, ne: r, sw: 0, se: r })
            }
            DrawerSide::Right => {
                let rect = Rect::from_min_size(
                    Pos2::new(screen_rect.right() - size, screen_rect.top()),
                    Vec2::new(size, screen_rect.height()),
                );
                (rect, egui::CornerRadius { nw: r, ne: 0, sw: r, se: 0 })
            }
        };

        // Draw backdrop overlay - just visual, no interaction capture
        let backdrop_layer = egui::LayerId::new(egui::Order::Middle, self.id.with("backdrop_layer"));
        ui.ctx().layer_painter(backdrop_layer).rect_filled(
            screen_rect,
            0.0,
            Color32::from_black_alpha(128),
        );

        // Check for clicks outside the drawer panel - only on primary button release
        let pointer_pos = ui.ctx().input(|i| i.pointer.interact_pos());
        let primary_released = ui.ctx().input(|i| i.pointer.primary_released());
        let clicked_outside = primary_released
            && pointer_pos.map(|p| !drawer_rect.contains(p)).unwrap_or(false);

        if clicked_outside {
            *self.open = false;
            return None;
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
