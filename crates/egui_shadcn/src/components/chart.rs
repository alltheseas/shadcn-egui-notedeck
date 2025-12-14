//! Chart component ported from shadcn/ui
//!
//! Simple charting components for data visualization.
//!
//! Reference: <https://ui.shadcn.com/docs/components/chart>

use egui::{Id, Response, Ui, Vec2, Pos2, Rect};
use crate::theme::ShadcnTheme;

/// Chart type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChartType {
    /// Bar chart
    #[default]
    Bar,
    /// Line chart
    Line,
    /// Area chart (filled line)
    Area,
}

/// A single data point
#[derive(Debug, Clone)]
pub struct DataPoint {
    /// Label for the x-axis
    pub label: String,
    /// Value for the y-axis
    pub value: f64,
}

impl DataPoint {
    /// Create a new data point
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

/// Chart component for data visualization
///
/// ## Example
/// ```rust,ignore
/// let data = vec![
///     DataPoint::new("Jan", 100.0),
///     DataPoint::new("Feb", 150.0),
///     DataPoint::new("Mar", 120.0),
/// ];
///
/// Chart::new("my_chart", &data)
///     .chart_type(ChartType::Bar)
///     .size(Vec2::new(400.0, 200.0))
///     .show(ui);
/// ```
pub struct Chart<'a> {
    id: Id,
    data: &'a [DataPoint],
    chart_type: ChartType,
    size: Vec2,
    show_labels: bool,
    show_grid: bool,
    color: Option<egui::Color32>,
}

impl<'a> Chart<'a> {
    /// Create a new chart
    pub fn new(id: impl std::hash::Hash, data: &'a [DataPoint]) -> Self {
        Self {
            id: Id::new(id),
            data,
            chart_type: ChartType::Bar,
            size: Vec2::new(400.0, 200.0),
            show_labels: true,
            show_grid: true,
            color: None,
        }
    }

    /// Set the chart type
    pub fn chart_type(mut self, chart_type: ChartType) -> Self {
        self.chart_type = chart_type;
        self
    }

    /// Set the chart size
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    /// Show/hide x-axis labels
    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Show/hide grid lines
    pub fn show_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Set custom color for the chart
    pub fn color(mut self, color: egui::Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Show the chart
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let label_height = if self.show_labels { 24.0 } else { 0.0 };
        let padding = 8.0;
        let total_size = Vec2::new(self.size.x, self.size.y + label_height);

        let (rect, response) = ui.allocate_exact_size(total_size, egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            let chart_color = self.color.unwrap_or(theme.colors.primary);

            // Chart area (excluding labels)
            let chart_rect = Rect::from_min_max(
                rect.min + Vec2::new(padding, padding),
                Pos2::new(rect.max.x - padding, rect.max.y - label_height - padding),
            );

            // Draw background
            ui.painter().rect_filled(rect, theme.radii.lg, theme.colors.card);
            ui.painter().rect_stroke(
                rect,
                theme.radii.lg,
                egui::Stroke::new(1.0, theme.colors.border),
                egui::StrokeKind::Inside,
            );

            if self.data.is_empty() {
                // Show empty state
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "No data",
                    egui::FontId::proportional(14.0),
                    theme.colors.muted_foreground,
                );
                return response;
            }

            // Find min/max values
            let max_value = self.data.iter().map(|d| d.value).fold(0.0_f64, f64::max);
            let min_value = 0.0_f64; // Always start from 0

            // Draw grid lines
            if self.show_grid {
                let grid_lines = 4;
                for i in 0..=grid_lines {
                    let y = chart_rect.min.y + (chart_rect.height() * i as f32 / grid_lines as f32);
                    ui.painter().line_segment(
                        [Pos2::new(chart_rect.min.x, y), Pos2::new(chart_rect.max.x, y)],
                        egui::Stroke::new(1.0, theme.colors.border.gamma_multiply(0.5)),
                    );
                }
            }

            match self.chart_type {
                ChartType::Bar => self.draw_bar_chart(ui, &theme, chart_rect, max_value, chart_color),
                ChartType::Line => self.draw_line_chart(ui, &theme, chart_rect, max_value, chart_color, false),
                ChartType::Area => self.draw_line_chart(ui, &theme, chart_rect, max_value, chart_color, true),
            }

            // Draw x-axis labels
            if self.show_labels {
                let bar_width = chart_rect.width() / self.data.len() as f32;
                for (i, point) in self.data.iter().enumerate() {
                    let x = chart_rect.min.x + bar_width * (i as f32 + 0.5);
                    let y = rect.max.y - label_height / 2.0;

                    ui.painter().text(
                        Pos2::new(x, y),
                        egui::Align2::CENTER_CENTER,
                        &point.label,
                        egui::FontId::proportional(11.0),
                        theme.colors.muted_foreground,
                    );
                }
            }
        }

        response
    }

    fn draw_bar_chart(
        &self,
        ui: &mut Ui,
        theme: &ShadcnTheme,
        chart_rect: Rect,
        max_value: f64,
        color: egui::Color32,
    ) {
        let bar_count = self.data.len();
        let bar_spacing = 8.0;
        let available_width = chart_rect.width() - (bar_spacing * (bar_count + 1) as f32);
        let bar_width = available_width / bar_count as f32;

        for (i, point) in self.data.iter().enumerate() {
            let normalized = if max_value > 0.0 {
                (point.value / max_value) as f32
            } else {
                0.0
            };

            let bar_height = normalized * chart_rect.height();
            let x = chart_rect.min.x + bar_spacing + (bar_width + bar_spacing) * i as f32;
            let y = chart_rect.max.y - bar_height;

            let bar_rect = Rect::from_min_size(
                Pos2::new(x, y),
                Vec2::new(bar_width, bar_height),
            );

            ui.painter().rect_filled(bar_rect, theme.radii.sm, color);
        }
    }

    fn draw_line_chart(
        &self,
        ui: &mut Ui,
        theme: &ShadcnTheme,
        chart_rect: Rect,
        max_value: f64,
        color: egui::Color32,
        fill: bool,
    ) {
        if self.data.len() < 2 {
            return;
        }

        let point_count = self.data.len();
        let x_step = chart_rect.width() / (point_count - 1) as f32;

        // Calculate points
        let points: Vec<Pos2> = self.data.iter().enumerate().map(|(i, point)| {
            let normalized = if max_value > 0.0 {
                (point.value / max_value) as f32
            } else {
                0.0
            };

            let x = chart_rect.min.x + x_step * i as f32;
            let y = chart_rect.max.y - normalized * chart_rect.height();
            Pos2::new(x, y)
        }).collect();

        // Draw filled area if requested
        if fill {
            // Build a closed path: data points left-to-right, then bottom-right to bottom-left
            let fill_color = color.gamma_multiply(0.3);
            let mut path = egui::epaint::PathShape::closed_line(vec![], egui::Stroke::NONE);
            path.fill = fill_color;

            // Start from bottom-left, go up to first data point
            path.points.push(Pos2::new(chart_rect.min.x, chart_rect.max.y));
            // Add all data points
            for p in &points {
                path.points.push(*p);
            }
            // Go down to bottom-right and close
            path.points.push(Pos2::new(chart_rect.max.x, chart_rect.max.y));

            ui.painter().add(egui::Shape::Path(path));
        }

        // Draw line segments
        for window in points.windows(2) {
            ui.painter().line_segment(
                [window[0], window[1]],
                egui::Stroke::new(2.0, color),
            );
        }

        // Draw data points
        for point in &points {
            ui.painter().circle_filled(*point, 4.0, color);
            ui.painter().circle_stroke(*point, 4.0, egui::Stroke::new(2.0, theme.colors.background));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_point() {
        let point = DataPoint::new("Test", 42.0);
        assert_eq!(point.label, "Test");
        assert_eq!(point.value, 42.0);
    }

    #[test]
    fn test_chart_creation() {
        let data = vec![
            DataPoint::new("A", 10.0),
            DataPoint::new("B", 20.0),
        ];
        let chart = Chart::new("test", &data)
            .chart_type(ChartType::Line)
            .size(Vec2::new(300.0, 150.0));

        assert_eq!(chart.chart_type, ChartType::Line);
    }
}
