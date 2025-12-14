//! Breadcrumb component ported from shadcn/ui
//!
//! A navigation component showing the current page's location in a hierarchy.
//!
//! Reference: <https://ui.shadcn.com/docs/components/breadcrumb>

use egui::{Response, Ui, Sense, Vec2, Pos2};
use crate::theme::ShadcnTheme;

/// Breadcrumb separator style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreadcrumbSeparator {
    /// Chevron separator (>)
    Chevron,
    /// Slash separator (/)
    Slash,
}

/// Breadcrumb component for navigation hierarchy
///
/// ## Example
/// ```rust,ignore
/// Breadcrumb::new()
///     .item("Home", || { /* navigate home */ })
///     .item("Products", || { /* navigate to products */ })
///     .item("Electronics", || { /* navigate to electronics */ })
///     .current("Laptop")
///     .show(ui);
/// ```
pub struct Breadcrumb<'a> {
    items: Vec<BreadcrumbItem<'a>>,
    current: Option<String>,
    separator: BreadcrumbSeparator,
}

struct BreadcrumbItem<'a> {
    label: String,
    on_click: Option<Box<dyn FnOnce() + 'a>>,
}

impl<'a> Breadcrumb<'a> {
    /// Create a new breadcrumb
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            current: None,
            separator: BreadcrumbSeparator::Chevron,
        }
    }

    /// Set the separator style
    pub fn separator(mut self, separator: BreadcrumbSeparator) -> Self {
        self.separator = separator;
        self
    }

    /// Add a breadcrumb item with click handler
    pub fn item(mut self, label: impl Into<String>, on_click: impl FnOnce() + 'a) -> Self {
        self.items.push(BreadcrumbItem {
            label: label.into(),
            on_click: Some(Box::new(on_click)),
        });
        self
    }

    /// Add a breadcrumb item without click handler (just text)
    pub fn item_text(mut self, label: impl Into<String>) -> Self {
        self.items.push(BreadcrumbItem {
            label: label.into(),
            on_click: None,
        });
        self
    }

    /// Set the current page (shown at the end, not clickable)
    pub fn current(mut self, label: impl Into<String>) -> Self {
        self.current = Some(label.into());
        self
    }

    /// Show the breadcrumb
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let response = ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = theme.spacing.sm;

            let item_count = self.items.len();

            for (idx, item) in self.items.into_iter().enumerate() {
                // Render item
                if item.on_click.is_some() {
                    // Clickable link
                    let link_response = ui.add(
                        egui::Label::new(
                            egui::RichText::new(&item.label)
                                .size(theme.typography.small().size)
                                .color(theme.colors.muted_foreground)
                        ).sense(Sense::click())
                    );

                    if link_response.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        // Draw underline on hover
                        let rect = link_response.rect;
                        ui.painter().line_segment(
                            [
                                egui::pos2(rect.min.x, rect.max.y),
                                egui::pos2(rect.max.x, rect.max.y),
                            ],
                            egui::Stroke::new(1.0, theme.colors.muted_foreground),
                        );
                    }

                    if link_response.clicked() {
                        if let Some(on_click) = item.on_click {
                            on_click();
                        }
                    }
                } else {
                    // Non-clickable text
                    ui.label(
                        egui::RichText::new(&item.label)
                            .size(theme.typography.small().size)
                            .color(theme.colors.muted_foreground)
                    );
                }

                // Separator (if not last item or if there's a current page)
                let has_more = idx < item_count - 1 || self.current.is_some();
                if has_more {
                    match self.separator {
                        BreadcrumbSeparator::Chevron => {
                            // Draw a proper chevron using line segments
                            let sep_size = Vec2::new(12.0, 16.0);
                            let (sep_rect, _) = ui.allocate_exact_size(sep_size, Sense::hover());

                            if ui.is_rect_visible(sep_rect) {
                                let chevron_x = sep_rect.center().x;
                                let chevron_y = sep_rect.center().y;
                                let chevron_size = 3.5;
                                let stroke = egui::Stroke::new(1.5, theme.colors.muted_foreground);

                                // Right chevron (>)
                                ui.painter().line_segment(
                                    [Pos2::new(chevron_x - chevron_size * 0.5, chevron_y - chevron_size),
                                     Pos2::new(chevron_x + chevron_size * 0.5, chevron_y)],
                                    stroke,
                                );
                                ui.painter().line_segment(
                                    [Pos2::new(chevron_x + chevron_size * 0.5, chevron_y),
                                     Pos2::new(chevron_x - chevron_size * 0.5, chevron_y + chevron_size)],
                                    stroke,
                                );
                            }
                        }
                        BreadcrumbSeparator::Slash => {
                            ui.label(
                                egui::RichText::new("/")
                                    .size(theme.typography.small().size)
                                    .color(theme.colors.muted_foreground)
                            );
                        }
                    }
                }
            }

            // Current page (shown at the end)
            if let Some(current) = self.current {
                ui.label(
                    egui::RichText::new(current)
                        .size(theme.typography.small().size)
                        .color(theme.colors.foreground)
                );
            }
        });

        response.response
    }
}

impl Default for Breadcrumb<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumb_creation() {
        let breadcrumb = Breadcrumb::new()
            .separator(BreadcrumbSeparator::Slash)
            .current("Current Page");

        assert_eq!(breadcrumb.separator, BreadcrumbSeparator::Slash);
        assert_eq!(breadcrumb.current, Some("Current Page".to_string()));
    }
}
