//! Table component ported from shadcn/ui
//!
//! A responsive table component for displaying tabular data.
//!
//! Reference: <https://ui.shadcn.com/docs/components/table>

use egui::{Ui, Sense, Vec2, Pos2};
use crate::theme::ShadcnTheme;

/// Table component for displaying data
///
/// ## Example
/// ```rust,ignore
/// Table::new("users")
///     .striped(true)
///     .header(|ui| {
///         ui.label("Name");
///         ui.label("Email");
///         ui.label("Status");
///     })
///     .body(|body| {
///         for user in &users {
///             body.row(|ui| {
///                 ui.label(&user.name);
///                 ui.label(&user.email);
///                 ui.label(&user.status);
///             });
///         }
///     })
///     .show(ui);
/// ```
pub struct Table<'a> {
    id: &'a str,
    striped: bool,
    hoverable: bool,
}

impl<'a> Table<'a> {
    /// Create a new table
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            striped: true,
            hoverable: true,
        }
    }

    /// Enable/disable striped rows (default: true)
    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    /// Enable/disable row hover effect (default: true)
    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }

    /// Show the table with header and body
    pub fn show<H, B>(self, ui: &mut Ui, header: H, body: B)
    where
        H: FnOnce(&mut Ui),
        B: FnOnce(&mut TableBody<'_>),
    {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let _id = egui::Id::new(self.id);

        // Table container with subtle border
        egui::Frame::NONE
            .stroke(egui::Stroke::new(1.0, theme.colors.border.linear_multiply(0.5)))
            .corner_radius(theme.radii.md)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());

                // Header
                egui::Frame::NONE
                    .fill(theme.colors.muted)
                    .inner_margin(egui::Margin::symmetric(12, 10))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.style_mut().visuals.override_text_color = Some(theme.colors.muted_foreground);
                            header(ui);
                        });
                    });

                // Separator (subtle)
                let sep_rect = ui.available_rect_before_wrap();
                ui.painter().line_segment(
                    [
                        Pos2::new(sep_rect.min.x, sep_rect.min.y),
                        Pos2::new(sep_rect.max.x, sep_rect.min.y),
                    ],
                    egui::Stroke::new(1.0, theme.colors.border.linear_multiply(0.5)),
                );

                // Body
                let mut table_body = TableBody {
                    row_index: 0,
                    rows: Vec::new(),
                };
                body(&mut table_body);

                // Render rows
                let row_count = table_body.rows.len();
                for (idx, row_fn) in table_body.rows.into_iter().enumerate() {
                    let is_striped = self.striped && idx % 2 == 1;

                    egui::Frame::NONE
                        .fill(if is_striped {
                            theme.colors.muted.linear_multiply(0.3)
                        } else {
                            egui::Color32::TRANSPARENT
                        })
                        .inner_margin(egui::Margin::symmetric(12, 10))
                        .show(ui, |ui| {
                            // Detect hover
                            let row_rect = ui.available_rect_before_wrap();
                            let response = ui.interact(row_rect, egui::Id::new(("table_row", idx)), Sense::hover());

                            if self.hoverable && response.hovered() {
                                ui.painter().rect_filled(row_rect, 0.0, theme.colors.muted.linear_multiply(0.5));
                            }

                            ui.horizontal(|ui| {
                                row_fn(ui);
                            });
                        });

                    // Row separator (except last) - very subtle
                    if idx < row_count.saturating_sub(1) {
                        let sep_rect = ui.available_rect_before_wrap();
                        ui.painter().line_segment(
                            [
                                Pos2::new(sep_rect.min.x, sep_rect.min.y),
                                Pos2::new(sep_rect.max.x, sep_rect.min.y),
                            ],
                            egui::Stroke::new(1.0, theme.colors.border.linear_multiply(0.3)),
                        );
                    }
                }
            });
    }
}

/// Table body builder
pub struct TableBody<'a> {
    #[allow(dead_code)]
    row_index: usize,
    rows: Vec<Box<dyn FnOnce(&mut Ui) + 'a>>,
}

impl<'a> TableBody<'a> {
    /// Add a row to the table
    pub fn row(&mut self, content: impl FnOnce(&mut Ui) + 'a) {
        self.rows.push(Box::new(content));
        self.row_index += 1;
    }
}

/// Simple table helper for basic use cases
///
/// ## Example
/// ```rust,ignore
/// simple_table(ui, &["Name", "Age", "City"], &[
///     &["Alice", "30", "NYC"],
///     &["Bob", "25", "LA"],
/// ]);
/// ```
pub fn simple_table(ui: &mut Ui, headers: &[&str], rows: &[&[&str]]) {
    let theme = ui.ctx().data(|d| {
        d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
            .unwrap_or_else(ShadcnTheme::light)
    });

    let num_cols = headers.len().max(1);
    let table_width = ui.available_width();
    let col_width = table_width / num_cols as f32;
    let row_height = 44.0;

    egui::Frame::NONE
        .stroke(egui::Stroke::new(1.0, theme.colors.border.linear_multiply(0.5)))
        .corner_radius(theme.radii.md)
        .show(ui, |ui| {
            ui.set_min_width(table_width);

            // Header row
            let (header_rect, _) = ui.allocate_exact_size(
                Vec2::new(table_width, row_height),
                Sense::hover(),
            );

            if ui.is_rect_visible(header_rect) {
                // Header background (no rounded corners, frame handles that)
                ui.painter().rect_filled(
                    header_rect,
                    0.0,
                    theme.colors.muted.linear_multiply(0.5),
                );

                // Header text
                for (i, header) in headers.iter().enumerate() {
                    let is_last = i == num_cols - 1;
                    let cell_rect = egui::Rect::from_min_size(
                        Pos2::new(header_rect.min.x + (i as f32 * col_width), header_rect.min.y),
                        Vec2::new(col_width, row_height),
                    );

                    // Right-align last column (typically Amount), left-align others
                    let (align, text_pos) = if is_last {
                        (egui::Align2::RIGHT_CENTER, Pos2::new(cell_rect.max.x - 16.0, cell_rect.center().y))
                    } else {
                        (egui::Align2::LEFT_CENTER, Pos2::new(cell_rect.min.x + 16.0, cell_rect.center().y))
                    };

                    ui.painter().text(
                        text_pos,
                        align,
                        *header,
                        egui::FontId::proportional(theme.typography.small().size),
                        theme.colors.muted_foreground,
                    );
                }
            }

            // Header separator (subtle)
            ui.painter().line_segment(
                [
                    Pos2::new(header_rect.min.x, header_rect.max.y),
                    Pos2::new(header_rect.max.x, header_rect.max.y),
                ],
                egui::Stroke::new(1.0, theme.colors.border.linear_multiply(0.5)),
            );

            // Data rows
            for (row_idx, row) in rows.iter().enumerate() {
                let (row_rect, _) = ui.allocate_exact_size(
                    Vec2::new(table_width, row_height),
                    Sense::hover(),
                );

                if ui.is_rect_visible(row_rect) {
                    // Alternating row background (subtle)
                    if row_idx % 2 == 1 {
                        ui.painter().rect_filled(
                            row_rect,
                            0.0,
                            theme.colors.muted.linear_multiply(0.3),
                        );
                    }

                    // Cell text
                    for (col_idx, cell) in row.iter().enumerate() {
                        let is_last = col_idx == num_cols - 1;
                        let cell_rect = egui::Rect::from_min_size(
                            Pos2::new(row_rect.min.x + (col_idx as f32 * col_width), row_rect.min.y),
                            Vec2::new(col_width, row_height),
                        );

                        // Right-align last column (typically Amount), left-align others
                        let (align, text_pos) = if is_last {
                            (egui::Align2::RIGHT_CENTER, Pos2::new(cell_rect.max.x - 16.0, cell_rect.center().y))
                        } else {
                            (egui::Align2::LEFT_CENTER, Pos2::new(cell_rect.min.x + 16.0, cell_rect.center().y))
                        };

                        ui.painter().text(
                            text_pos,
                            align,
                            *cell,
                            egui::FontId::proportional(theme.typography.small().size),
                            theme.colors.foreground,
                        );
                    }
                }

                // Row separator (except last) - very subtle
                if row_idx < rows.len() - 1 {
                    ui.painter().line_segment(
                        [
                            Pos2::new(row_rect.min.x, row_rect.max.y),
                            Pos2::new(row_rect.max.x, row_rect.max.y),
                        ],
                        egui::Stroke::new(1.0, theme.colors.border.linear_multiply(0.3)),
                    );
                }
            }
        });
}

/// Table row response
pub struct TableResponse {
    /// Index of clicked row, if any
    pub clicked_row: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let table = Table::new("test")
            .striped(false)
            .hoverable(true);

        assert!(!table.striped);
        assert!(table.hoverable);
    }
}
