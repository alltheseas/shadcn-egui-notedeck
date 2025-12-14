//! Input helpers for form components with shadcn styling
//!
//! Provides styled wrappers around egui's built-in TextEdit widgets.
//!
//! Reference: <https://ui.shadcn.com/docs/components/input>

use egui::{Response, TextEdit, Ui};
use crate::theme::ShadcnTheme;

/// Styled single-line text input
///
/// Features shadcn-style border with hover and focus states:
/// - Default: soft border color
/// - Hover: slightly darker/lighter border
/// - Focus: ring color border + focus ring outline
///
/// ## Example
/// ```rust,ignore
/// let mut text = String::new();
/// shadcn_input(ui, &mut text, "Enter your email...");
/// ```
pub fn shadcn_input(ui: &mut Ui, text: &mut String, placeholder: &str) -> Response {
    shadcn_input_with_error(ui, text, placeholder, false)
}

/// Styled single-line text input with optional error state
///
/// When `has_error` is true, the border will be red/destructive colored.
pub fn shadcn_input_with_error(ui: &mut Ui, text: &mut String, placeholder: &str, has_error: bool) -> Response {
    let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

    // Allocate space first to detect hover
    let desired_size = egui::vec2(ui.available_width(), 44.0); // Apple HIG min touch target
    let (rect, _) = ui.allocate_at_least(desired_size, egui::Sense::hover());
    let is_hovered = ui.rect_contains_pointer(rect);

    // Draw background
    ui.painter().rect_filled(rect, theme.radii.md, theme.colors.background);

    // We'll detect focus after adding the text edit
    let mut response: Option<Response> = None;

    // Draw input inside the rect
    ui.scope_builder(
        egui::UiBuilder::new().max_rect(rect.shrink2(egui::vec2(12.0, 8.0))),
        |ui| {
            let text_edit = TextEdit::singleline(text)
                .hint_text(placeholder)
                .frame(false)  // Disable default frame, we provide our own
                .desired_width(ui.available_width())
                .vertical_align(egui::Align::Center);

            response = Some(ui.add(text_edit));
        },
    );

    let response = response.unwrap();
    let has_focus = response.has_focus();

    // Determine border color based on state
    // Error state takes priority - show destructive color
    let border_color = if has_error {
        theme.colors.destructive
    } else if has_focus {
        theme.colors.ring
    } else if is_hovered {
        // Slightly more prominent border on hover
        theme.colors.foreground.linear_multiply(0.3)
    } else {
        theme.colors.border
    };

    // Draw border (thicker when focused or error for emphasis)
    let border_width = if has_focus || has_error { 2.0 } else { 1.0 };
    ui.painter().rect_stroke(
        rect,
        theme.radii.md,
        egui::Stroke::new(border_width, border_color),
        egui::StrokeKind::Inside,
    );

    response
}

/// Styled multi-line text area
///
/// Features shadcn-style border with hover and focus states:
/// - Default: soft border color
/// - Hover: slightly darker/lighter border
/// - Focus: ring color border + focus ring outline
///
/// ## Example
/// ```rust,ignore
/// let mut text = String::new();
/// shadcn_textarea(ui, &mut text, "Enter description...");
/// ```
pub fn shadcn_textarea(ui: &mut Ui, text: &mut String, placeholder: &str) -> Response {
    let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

    // Calculate min height for 4 rows + padding
    let line_height = theme.typography.body().size * 1.4;
    let min_height = (line_height * 4.0 + 16.0).max(88.0); // At least 4 rows, min 88px for touch

    // Allocate space first to detect hover
    let desired_size = egui::vec2(ui.available_width(), min_height);
    let (rect, _) = ui.allocate_at_least(desired_size, egui::Sense::hover());
    let is_hovered = ui.rect_contains_pointer(rect);

    // Draw background
    ui.painter().rect_filled(rect, theme.radii.md, theme.colors.background);

    // We'll detect focus after adding the text edit
    let mut response: Option<Response> = None;

    // Draw textarea inside the rect
    ui.scope_builder(
        egui::UiBuilder::new().max_rect(rect.shrink2(egui::vec2(12.0, 8.0))),
        |ui| {
            let text_edit = TextEdit::multiline(text)
                .hint_text(placeholder)
                .frame(false)  // Disable default frame, we provide our own
                .desired_width(ui.available_width())
                .desired_rows(4);

            response = Some(ui.add(text_edit));
        },
    );

    let response = response.unwrap();
    let has_focus = response.has_focus();

    // Determine border color based on state
    // On focus: use ring color for border (no separate focus ring to avoid double border)
    let border_color = if has_focus {
        theme.colors.ring
    } else if is_hovered {
        // Slightly more prominent border on hover
        theme.colors.foreground.linear_multiply(0.3)
    } else {
        theme.colors.border
    };

    // Draw border (thicker when focused for emphasis)
    let border_width = if has_focus { 2.0 } else { 1.0 };
    ui.painter().rect_stroke(
        rect,
        theme.radii.md,
        egui::Stroke::new(border_width, border_color),
        egui::StrokeKind::Inside,
    );

    response
}

/// Form label with shadcn styling
///
/// ## Example
/// ```rust,ignore
/// form_label(ui, "Email Address");
/// shadcn_input(ui, &mut email, "Enter email...");
/// ```
pub fn form_label(ui: &mut Ui, text: &str) {
    let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });
    ui.label(
        egui::RichText::new(text)
            .size(theme.typography.small().size)
            .color(theme.colors.foreground)
    );
}

/// Form field helper text
/// Uses 70% foreground opacity to ensure sufficient contrast while showing hierarchy.
pub fn form_helper(ui: &mut Ui, text: &str) {
    let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });
    ui.label(
        egui::RichText::new(text)
            .size(theme.typography.small().size)
            .color(theme.colors.foreground.linear_multiply(0.7))
    );
}
