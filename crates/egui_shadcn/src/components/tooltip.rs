//! Tooltip component matching shadcn/ui
//!
//! Provides shadcn-styled tooltips for hover information.
//!
//! Reference: <https://ui.shadcn.com/docs/components/tooltip>

use egui::{Frame, Id, Response, Ui, WidgetText};
use crate::theme::ShadcnTheme;

/// Extension trait for adding shadcn-styled tooltips to responses
pub trait TooltipExt {
    /// Show a shadcn-styled tooltip on hover
    ///
    /// Unlike the default egui tooltip, this uses shadcn's inverted color scheme
    /// (dark background, light text).
    ///
    /// ## Example
    /// ```rust,ignore
    /// use egui_shadcn::TooltipExt;
    ///
    /// if ui.button("Hover me").shadcn_tooltip("This is a tooltip").clicked() {
    ///     // handle click
    /// }
    /// ```
    fn shadcn_tooltip(self, text: impl Into<WidgetText>) -> Self;

    /// Show a shadcn-styled tooltip with custom content on hover
    fn shadcn_tooltip_ui(self, add_contents: impl FnOnce(&mut Ui)) -> Self;
}

impl TooltipExt for Response {
    fn shadcn_tooltip(self, text: impl Into<WidgetText>) -> Self {
        let text = text.into();
        self.on_hover_ui(|ui| {
            style_tooltip_ui(ui);
            ui.label(text.clone());
        })
    }

    fn shadcn_tooltip_ui(self, add_contents: impl FnOnce(&mut Ui)) -> Self {
        self.on_hover_ui(|ui| {
            style_tooltip_ui(ui);
            add_contents(ui);
        })
    }
}

/// Apply shadcn tooltip styling to a UI (used within on_hover_ui)
///
/// This styles the existing tooltip frame rather than creating a new one.
fn style_tooltip_ui(ui: &mut Ui) {
    let theme = ui.ctx().data(|d| {
        d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
            .unwrap_or_else(ShadcnTheme::light)
    });

    // Apply text styling
    ui.style_mut().visuals.override_text_color = Some(theme.colors.popover_foreground);
    ui.style_mut().override_font_id = Some(egui::FontId::proportional(12.0));
}

/// Show a standalone tooltip frame with shadcn styling
///
/// Uses the theme's popover colors which are designed for floating UI elements.
fn show_shadcn_tooltip_frame(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    let theme = ui.ctx().data(|d| {
        d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
            .unwrap_or_else(ShadcnTheme::light)
    });

    Frame::NONE
        .fill(theme.colors.popover)
        .stroke(egui::Stroke::new(1.0, theme.colors.border))
        .corner_radius(theme.radii.sm)
        .shadow(theme.shadows.md)
        .inner_margin(egui::Margin::symmetric(10, 4))
        .show(ui, |ui| {
            ui.style_mut().visuals.override_text_color = Some(theme.colors.popover_foreground);
            ui.style_mut().override_font_id = Some(egui::FontId::proportional(12.0));
            add_contents(ui);
        });
}

/// Standalone tooltip component for more control
///
/// ## Example
/// ```rust,ignore
/// let response = ui.button("Click me");
/// if response.hovered() {
///     Tooltip::show(ui, "Button tooltip");
/// }
/// ```
pub struct Tooltip;

impl Tooltip {
    /// Show a simple text tooltip at the current hover position
    ///
    /// Call this when you want to show a tooltip for a hovered widget.
    pub fn show(ui: &mut Ui, text: impl Into<String>) {
        show_shadcn_tooltip_frame(ui, |ui| {
            ui.label(text.into());
        });
    }

    /// Show a tooltip with custom UI content
    pub fn show_ui(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
        show_shadcn_tooltip_frame(ui, add_contents);
    }
}

/// Helper function to show a simple shadcn-styled tooltip for a response
///
/// ## Example
/// ```rust,ignore
/// let response = ui.button("Click me");
/// shadcn_tooltip_for(&response, "Button tooltip");
/// ```
pub fn shadcn_tooltip_for(response: &Response, text: impl Into<String>) -> Response {
    let text = text.into();
    response.clone().on_hover_ui(|ui| {
        style_tooltip_ui(ui);
        ui.label(&text);
    })
}

#[cfg(test)]
mod tests {
    // Tooltip is a zero-sized type with only associated functions
    // No state to test
}
