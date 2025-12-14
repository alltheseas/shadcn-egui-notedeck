//! Pagination component ported from shadcn/ui
//!
//! Navigation for paginated content with page numbers and prev/next buttons.
//!
//! Reference: <https://ui.shadcn.com/docs/components/pagination>

use egui::{Response, Ui, Sense, Vec2, Pos2};
use crate::theme::ShadcnTheme;

/// Pagination component for navigating pages
///
/// ## Example
/// ```rust,ignore
/// let mut current_page = 1;
/// let total_pages = 10;
///
/// if Pagination::new(&mut current_page, total_pages)
///     .show(ui)
///     .changed()
/// {
///     // Page changed, reload data
/// }
/// ```
pub struct Pagination<'a> {
    current_page: &'a mut usize,
    total_pages: usize,
    siblings: usize,
    show_edges: bool,
}

impl<'a> Pagination<'a> {
    /// Create a new pagination component
    ///
    /// - `current_page`: 1-indexed current page number
    /// - `total_pages`: Total number of pages
    pub fn new(current_page: &'a mut usize, total_pages: usize) -> Self {
        Self {
            current_page,
            total_pages,
            siblings: 1,
            show_edges: true,
        }
    }

    /// Set how many sibling pages to show on each side of current (default: 1)
    pub fn siblings(mut self, siblings: usize) -> Self {
        self.siblings = siblings;
        self
    }

    /// Whether to always show first and last page (default: true)
    pub fn show_edges(mut self, show: bool) -> Self {
        self.show_edges = show;
        self
    }

    /// Show the pagination
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let page_button_size = Vec2::splat(36.0);
        let spacing = 4.0;
        let mut changed = false;

        let response = ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = spacing;

            // Previous button - shadcn style with chevron icon + text
            let can_go_prev = *self.current_page > 1;
            let prev_response = self.draw_nav_button(ui, &theme, "Previous", can_go_prev, true);
            if prev_response.clicked() && can_go_prev {
                *self.current_page -= 1;
                changed = true;
            }

            ui.add_space(8.0);

            // Calculate which pages to show
            let pages = self.calculate_pages();

            for page_item in pages {
                match page_item {
                    PageItem::Page(num) => {
                        let is_current = num == *self.current_page;
                        let page_response = self.draw_page_button(
                            ui, &theme, num, is_current, page_button_size
                        );
                        if page_response.clicked() && !is_current {
                            *self.current_page = num;
                            changed = true;
                        }
                    }
                    PageItem::Ellipsis => {
                        self.draw_ellipsis(ui, &theme, page_button_size);
                    }
                }
            }

            ui.add_space(8.0);

            // Next button - shadcn style with text + chevron icon
            let can_go_next = *self.current_page < self.total_pages;
            let next_response = self.draw_nav_button(ui, &theme, "Next", can_go_next, false);
            if next_response.clicked() && can_go_next {
                *self.current_page += 1;
                changed = true;
            }
        });

        let mut response = response.response;
        if changed {
            response.mark_changed();
        }
        response
    }

    /// Calculate which pages to display
    fn calculate_pages(&self) -> Vec<PageItem> {
        let mut pages = Vec::new();
        let current = *self.current_page;
        let total = self.total_pages;

        if total <= 7 {
            // Show all pages if total is small
            for i in 1..=total {
                pages.push(PageItem::Page(i));
            }
            return pages;
        }

        // Always show first page if show_edges
        if self.show_edges {
            pages.push(PageItem::Page(1));
        }

        // Calculate range around current page
        let start = (current.saturating_sub(self.siblings)).max(2);
        let end = (current + self.siblings).min(total - 1);

        // Add ellipsis after first page if needed
        if start > 2 {
            pages.push(PageItem::Ellipsis);
        } else if !self.show_edges {
            // Show page 1 if we're not showing edges but start is 2
            pages.push(PageItem::Page(1));
        }

        // Add pages around current
        for i in start..=end {
            pages.push(PageItem::Page(i));
        }

        // Add ellipsis before last page if needed
        if end < total - 1 {
            pages.push(PageItem::Ellipsis);
        }

        // Always show last page if show_edges
        if self.show_edges && total > 1 {
            pages.push(PageItem::Page(total));
        }

        pages
    }

    /// Draw a navigation button (Previous/Next) with chevron icon - shadcn style
    fn draw_nav_button(
        &self,
        ui: &mut Ui,
        theme: &ShadcnTheme,
        text: &str,
        enabled: bool,
        is_prev: bool,
    ) -> Response {
        let font_id = egui::FontId::proportional(theme.typography.small().size);
        // Approximate text width based on character count
        let approx_char_width = theme.typography.small().size * 0.55;
        let text_width = approx_char_width * text.len() as f32;
        // Add space for chevron icon + gap + padding on both sides
        let chevron_space = 16.0;
        let gap = 4.0;
        let padding = 12.0;
        let size = Vec2::new(padding + chevron_space + gap + text_width + padding, 36.0);

        let (rect, response) = ui.allocate_exact_size(
            size,
            if enabled { Sense::click() } else { Sense::hover() },
        );

        if ui.is_rect_visible(rect) {
            let hovered = response.hovered() && enabled;

            // Background - only on hover, subtle
            if hovered {
                ui.painter().rect_filled(rect, theme.radii.md, theme.colors.accent);
            }

            // Color for both text and chevron
            let color = if enabled {
                if hovered {
                    theme.colors.accent_foreground
                } else {
                    theme.colors.foreground
                }
            } else {
                theme.colors.muted_foreground
            };

            let chevron_size = 4.0;
            let stroke = egui::Stroke::new(1.5, color);

            if is_prev {
                // Previous: chevron on left, text on right
                let chevron_x = rect.min.x + padding + 4.0;
                let chevron_y = rect.center().y;

                // Left chevron (<)
                ui.painter().line_segment(
                    [Pos2::new(chevron_x + chevron_size * 0.5, chevron_y - chevron_size),
                     Pos2::new(chevron_x - chevron_size * 0.5, chevron_y)],
                    stroke,
                );
                ui.painter().line_segment(
                    [Pos2::new(chevron_x - chevron_size * 0.5, chevron_y),
                     Pos2::new(chevron_x + chevron_size * 0.5, chevron_y + chevron_size)],
                    stroke,
                );

                // Text - positioned after chevron with gap
                ui.painter().text(
                    Pos2::new(chevron_x + chevron_size + gap + 4.0, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    text,
                    font_id,
                    color,
                );
            } else {
                // Next: text on left, chevron on right
                let chevron_x = rect.max.x - padding - 4.0;
                let chevron_y = rect.center().y;

                // Text - positioned before chevron
                ui.painter().text(
                    Pos2::new(chevron_x - chevron_size - gap - 4.0, rect.center().y),
                    egui::Align2::RIGHT_CENTER,
                    text,
                    font_id,
                    color,
                );

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

        response
    }

    /// Draw a page number button - shadcn minimal style
    fn draw_page_button(
        &self,
        ui: &mut Ui,
        theme: &ShadcnTheme,
        page: usize,
        is_current: bool,
        size: Vec2,
    ) -> Response {
        let (rect, response) = ui.allocate_exact_size(size, Sense::click());

        if ui.is_rect_visible(rect) {
            let hovered = response.hovered() && !is_current;

            // Background - subtle for selected, hover for others
            // shadcn uses a light border/background for selected, not filled primary
            if is_current {
                // Selected: subtle border with slight background
                ui.painter().rect_stroke(
                    rect,
                    theme.radii.md,
                    egui::Stroke::new(1.0, theme.colors.border),
                    egui::StrokeKind::Inside,
                );
            } else if hovered {
                ui.painter().rect_filled(rect, theme.radii.md, theme.colors.accent);
            }
            // No border for non-selected, non-hovered - clean minimal look

            // Text
            let text_color = if hovered {
                theme.colors.accent_foreground
            } else {
                theme.colors.foreground
            };

            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                &page.to_string(),
                egui::FontId::proportional(theme.typography.small().size),
                text_color,
            );
        }

        response
    }

    /// Draw ellipsis indicator
    fn draw_ellipsis(&self, ui: &mut Ui, theme: &ShadcnTheme, size: Vec2) {
        let (rect, _) = ui.allocate_exact_size(size, Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "...",
                egui::FontId::proportional(theme.typography.body().size),
                theme.colors.muted_foreground,
            );
        }
    }
}

/// Internal enum for page items
enum PageItem {
    Page(usize),
    Ellipsis,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_creation() {
        let mut page = 1;
        let pagination = Pagination::new(&mut page, 10);
        assert_eq!(pagination.total_pages, 10);
        assert_eq!(pagination.siblings, 1);
    }

    #[test]
    fn test_page_calculation_small() {
        let mut page = 3;
        let pagination = Pagination::new(&mut page, 5);
        let pages = pagination.calculate_pages();
        assert_eq!(pages.len(), 5); // All pages shown
    }
}
