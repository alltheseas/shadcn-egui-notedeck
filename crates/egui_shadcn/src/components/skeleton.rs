//! Skeleton component ported from shadcn/ui
//!
//! Displays a placeholder for loading states.
//!
//! Reference: <https://ui.shadcn.com/docs/components/skeleton>

use egui::{Response, Ui, Vec2, Widget};
use crate::theme::ShadcnTheme;

/// Skeleton component for loading states
///
/// ## Example
/// ```rust,ignore
/// ui.add(Skeleton::new(Vec2::new(200.0, 20.0)));
/// ui.add(Skeleton::circle(40.0));
/// ```
pub struct Skeleton {
    size: Vec2,
    is_circle: bool,
}

impl Skeleton {
    /// Create a rectangular skeleton with the given size
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            is_circle: false,
        }
    }

    /// Create a circular skeleton (for avatars, etc.)
    pub fn circle(diameter: f32) -> Self {
        Self {
            size: Vec2::splat(diameter),
            is_circle: true,
        }
    }
}

impl Widget for Skeleton {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let (rect, response) = ui.allocate_exact_size(
            self.size,
            egui::Sense::hover(),
        );

        if ui.is_rect_visible(rect) {
            let corner_radius = if self.is_circle {
                theme.radii.avatar()
            } else {
                theme.radii.uniform_md()
            };

            // Animated pulse effect (shadcn/ui style)
            // Get time for animation
            let time = ui.input(|i| i.time);

            // Pulse animation: 2 second cycle (1s fade in, 1s fade out)
            let cycle = (time % 2.0) as f32;
            let pulse = if cycle < 1.0 {
                cycle // 0.0 -> 1.0
            } else {
                2.0 - cycle // 1.0 -> 0.0
            };

            // Detect light vs dark mode for better contrast
            let is_dark_mode = theme.colors.background.r() < 128;

            // In light mode, use a darker base color for better visibility
            // and create more dramatic animation range
            let (dark_color, light_color) = if is_dark_mode {
                // Dark mode: muted is already visible
                let base = theme.colors.muted;
                let lighter = lighten_color(base, 0.3);
                (base, lighter)
            } else {
                // Light mode: create high contrast range for visibility
                let dark = darken_color(theme.colors.muted, 0.4);
                // Reduced lighter side by 20% (from 0.1 to 0.0 - just use muted, then darken slightly)
                let light = darken_color(theme.colors.muted, 0.08);
                (dark, light)
            };

            // Interpolate across the full range (pulse goes 0.0 to 1.0)
            let animated_color = blend_colors(dark_color, light_color, pulse);

            ui.painter().rect_filled(
                rect,
                corner_radius,
                animated_color,
            );

            // Request repaint to continue animation
            ui.ctx().request_repaint();
        }

        response
    }
}

/// Darken a color by a given factor (0.0 = no change, 1.0 = black)
fn darken_color(color: egui::Color32, factor: f32) -> egui::Color32 {
    let r = (color.r() as f32 * (1.0 - factor)).max(0.0) as u8;
    let g = (color.g() as f32 * (1.0 - factor)).max(0.0) as u8;
    let b = (color.b() as f32 * (1.0 - factor)).max(0.0) as u8;
    egui::Color32::from_rgb(r, g, b)
}

/// Lighten a color by a given factor (0.0 = no change, 1.0 = white)
fn lighten_color(color: egui::Color32, factor: f32) -> egui::Color32 {
    let r = color.r() as f32 + (255.0 - color.r() as f32) * factor;
    let g = color.g() as f32 + (255.0 - color.g() as f32) * factor;
    let b = color.b() as f32 + (255.0 - color.b() as f32) * factor;
    egui::Color32::from_rgb(r as u8, g as u8, b as u8)
}

/// Blend two colors together (t=0 is color1, t=1 is color2)
fn blend_colors(color1: egui::Color32, color2: egui::Color32, t: f32) -> egui::Color32 {
    let t = t.clamp(0.0, 1.0);
    let r = color1.r() as f32 * (1.0 - t) + color2.r() as f32 * t;
    let g = color1.g() as f32 * (1.0 - t) + color2.g() as f32 * t;
    let b = color1.b() as f32 * (1.0 - t) + color2.b() as f32 * t;
    egui::Color32::from_rgb(r as u8, g as u8, b as u8)
}
