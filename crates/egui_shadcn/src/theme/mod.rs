//! Theme system for egui_shadcn
//!
//! This module contains the design system foundation ported from shadcn/ui:
//! - Colors (semantic color tokens) ✓
//! - Spacing (consistent spacing scale) ✓
//! - Typography (font sizes and weights) ✓
//! - Corner radii (border radius values) ✓
//! - Shadows (elevation system) ✓
//!
//! The theme can be applied to an egui context to style all components consistently.

pub mod colors;
pub mod spacing;
pub mod typography;
pub mod radii;
pub mod shadows;

pub use colors::ShadcnColors;
pub use spacing::ShadcnSpacing;
pub use typography::ShadcnTypography;
pub use radii::ShadcnRadii;
pub use shadows::ShadcnShadows;

/// The main shadcn theme struct that aggregates all design tokens
///
/// Complete design system including:
/// - Colors: Complete semantic color system with light/dark modes
/// - Spacing: Tailwind-based spacing scale for consistent margins/padding
/// - Typography: Tailwind font size scale with semantic names
/// - Corner radii: Tailwind border-radius scale for rounded corners
/// - Shadows: Tailwind box-shadow scale for elevation and depth
#[derive(Debug, Clone)]
pub struct ShadcnTheme {
    /// Semantic color palette
    pub colors: ShadcnColors,
    /// Spacing scale based on Tailwind (4px base unit)
    pub spacing: ShadcnSpacing,
    /// Typography scale based on Tailwind font sizes
    pub typography: ShadcnTypography,
    /// Corner radius scale based on Tailwind (2px-32px)
    pub radii: ShadcnRadii,
    /// Shadow/elevation scale based on Tailwind box-shadows
    pub shadows: ShadcnShadows,
}

impl Default for ShadcnTheme {
    fn default() -> Self {
        Self::light()
    }
}

impl ShadcnTheme {
    /// Create a new shadcn theme with light mode colors
    pub fn new() -> Self {
        Self::light()
    }

    /// Create a light mode theme
    pub fn light() -> Self {
        Self {
            colors: ShadcnColors::light(),
            spacing: ShadcnSpacing::new(),
            typography: ShadcnTypography::new(),
            radii: ShadcnRadii::new(),
            shadows: ShadcnShadows::light(),
        }
    }

    /// Create a dark mode theme
    pub fn dark() -> Self {
        Self {
            colors: ShadcnColors::dark(),
            spacing: ShadcnSpacing::new(),
            typography: ShadcnTypography::new(),
            radii: ShadcnRadii::new(),
            shadows: ShadcnShadows::dark(),
        }
    }

    /// Lighten a color by a factor (0.0 to 1.0)
    fn lighten_color(&self, color: egui::Color32, factor: f32) -> egui::Color32 {
        let [r, g, b, a] = color.to_array();
        let r = (r as f32 + (255.0 - r as f32) * factor).min(255.0) as u8;
        let g = (g as f32 + (255.0 - g as f32) * factor).min(255.0) as u8;
        let b = (b as f32 + (255.0 - b as f32) * factor).min(255.0) as u8;
        egui::Color32::from_rgba_premultiplied(r, g, b, a)
    }

    /// Darken a color by a factor (0.0 to 1.0)
    fn darken_color(&self, color: egui::Color32, factor: f32) -> egui::Color32 {
        let [r, g, b, a] = color.to_array();
        let r = (r as f32 * (1.0 - factor)).max(0.0) as u8;
        let g = (g as f32 * (1.0 - factor)).max(0.0) as u8;
        let b = (b as f32 * (1.0 - factor)).max(0.0) as u8;
        egui::Color32::from_rgba_premultiplied(r, g, b, a)
    }

    /// Draw a focus ring around a widget (shadcn style)
    ///
    /// shadcn focus rings are:
    /// - 2-3px wide ring
    /// - Uses ring color (semi-transparent primary)
    /// - Only shown on keyboard focus
    /// Note: Ring is drawn inside the element bounds to avoid off-screen rendering issues
    pub fn draw_focus_ring(
        &self,
        painter: &egui::Painter,
        rect: egui::Rect,
        corner_radius: impl Into<egui::CornerRadius>,
        has_focus: bool,
    ) {
        if !has_focus {
            return;
        }

        let corner_radius = corner_radius.into();
        let ring_width = 2.0;

        // Draw focus ring inside the element bounds to avoid off-screen issues
        // Use a slightly smaller rect and draw inside stroke
        let ring_rect = rect.shrink(1.0);

        painter.rect_stroke(
            ring_rect,
            corner_radius,
            egui::Stroke::new(ring_width, self.colors.ring),
            egui::StrokeKind::Inside,
        );
    }

    /// Apply this theme to an egui context
    ///
    /// Maps shadcn color tokens to egui's Visuals system, applies
    /// spacing values to Style, configures typography, sets corner radii,
    /// and applies shadows. This provides consistent styling across all egui widgets.
    pub fn apply(&self, ctx: &egui::Context) {
        // Store theme in context for component access
        ctx.data_mut(|d| d.insert_temp(egui::Id::new("shadcn_theme"), self.clone()));

        let mut style = (*ctx.style()).clone();
        let visuals = &mut style.visuals;

        // Apply shadcn typography to egui text styles
        style.text_styles = typography::configure_text_styles(&self.typography);

        // Apply shadcn spacing to egui style
        // Item spacing = space between widgets in layouts
        style.spacing.item_spacing = self.spacing.vec2(2); // 8px
        // Button padding
        style.spacing.button_padding = self.spacing.vec2_xy(4, 2); // 16px x 8px
        // Window padding (convert f32 to i8 for Margin)
        style.spacing.window_margin = egui::Margin::same(self.spacing.window_padding() as i8);
        // Indent for collapsing sections
        style.spacing.indent = self.spacing.md_lg; // 20px

        // Apply shadcn corner radii to egui visuals
        // Widget corner radius for buttons, inputs, etc.
        visuals.widgets.noninteractive.corner_radius = self.radii.input();
        visuals.widgets.inactive.corner_radius = self.radii.button();
        visuals.widgets.hovered.corner_radius = self.radii.button();
        visuals.widgets.active.corner_radius = self.radii.button();
        visuals.widgets.open.corner_radius = self.radii.button();

        // Apply shadcn shadows to egui visuals
        // Window shadow (subtle card shadow)
        visuals.window_shadow = self.shadows.card();
        // Popup shadow (moderate elevation for dropdowns/menus)
        visuals.popup_shadow = self.shadows.popover();

        // Apply shadcn colors to egui visuals

        // Set dark_mode based on actual theme
        let is_light_mode = self.colors.background == egui::Color32::WHITE;
        visuals.dark_mode = !is_light_mode;

        // Window/panel background
        visuals.window_fill = self.colors.background;
        visuals.panel_fill = self.colors.background;

        // Extreme background (used for scrollbar track background)
        // Must be visible against the background color
        visuals.extreme_bg_color = if is_light_mode {
            egui::Color32::from_rgb(240, 240, 240) // Light gray for light mode
        } else {
            egui::Color32::from_rgb(40, 40, 40) // Dark gray for dark mode
        };

        // Configure scroll style to use bg_fill (primary color) for handles
        // instead of fg_stroke.color (which is white in light mode)
        style.spacing.scroll.foreground_color = false;

        // Foreground/text colors
        // Set override to apply consistent text color across all widgets
        visuals.override_text_color = Some(self.colors.foreground);

        // Widget colors - shadcn styling

        // Noninteractive (labels, text, disabled elements)
        visuals.widgets.noninteractive.bg_fill = self.colors.background;
        // Use visible gray for scrollbar track background
        visuals.widgets.noninteractive.weak_bg_fill = egui::Color32::from_rgb(230, 230, 230);
        visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, self.colors.border);
        visuals.widgets.noninteractive.fg_stroke.color = self.colors.muted_foreground;

        // Inactive (default button/input state) - PRIMARY PURPLE
        visuals.widgets.inactive.bg_fill = self.colors.primary;
        // Use a visible gray for scrollbar handles
        visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(150, 150, 150);
        visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
        visuals.widgets.inactive.fg_stroke.color = self.colors.primary_foreground;
        // Scrollbar handle expansion (make it more visible)
        visuals.widgets.inactive.expansion = 0.0;

        // Hovered - Slightly lighter/darker primary
        let hovered_primary = if self.colors.background == egui::Color32::WHITE {
            // Light mode - darken on hover
            self.darken_color(self.colors.primary, 0.1)
        } else {
            // Dark mode - lighten on hover
            self.lighten_color(self.colors.primary, 0.1)
        };
        visuals.widgets.hovered.bg_fill = hovered_primary;
        // Use ring color for hovered scrollbars (darker than border)
        visuals.widgets.hovered.weak_bg_fill = self.colors.ring;
        visuals.widgets.hovered.bg_stroke = egui::Stroke::new(2.0, self.colors.ring);
        visuals.widgets.hovered.fg_stroke.color = self.colors.primary_foreground;

        // Active (clicked/pressed) - Even darker/lighter
        let active_primary = if self.colors.background == egui::Color32::WHITE {
            self.darken_color(self.colors.primary, 0.2)
        } else {
            self.lighten_color(self.colors.primary, 0.2)
        };
        visuals.widgets.active.bg_fill = active_primary;
        visuals.widgets.active.weak_bg_fill = self.colors.primary;
        visuals.widgets.active.bg_stroke = egui::Stroke::NONE;
        visuals.widgets.active.fg_stroke.color = self.colors.primary_foreground;

        // Open (for dropdowns/menus)
        visuals.widgets.open.bg_fill = self.colors.primary;
        visuals.widgets.open.weak_bg_fill = self.colors.primary;
        visuals.widgets.open.bg_stroke = egui::Stroke::new(2.0, self.colors.ring);
        visuals.widgets.open.fg_stroke.color = self.colors.primary_foreground;

        // Selection colors
        visuals.selection.bg_fill = self.colors.accent;
        visuals.selection.stroke.color = self.colors.accent_foreground;

        // Hyperlink color
        visuals.hyperlink_color = self.colors.primary;

        // Error/warning colors (use destructive)
        visuals.error_fg_color = self.colors.destructive;
        visuals.warn_fg_color = self.colors.destructive;

        // Apply the updated style
        ctx.set_style(style);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let light = ShadcnTheme::light();
        let dark = ShadcnTheme::dark();

        // Verify light and dark themes have different backgrounds
        assert_ne!(light.colors.background, dark.colors.background);
    }

    #[test]
    fn test_default_is_light() {
        let default = ShadcnTheme::default();
        let light = ShadcnTheme::light();

        assert_eq!(default.colors.background, light.colors.background);
    }
}
