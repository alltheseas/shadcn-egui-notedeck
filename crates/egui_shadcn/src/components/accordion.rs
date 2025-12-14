//! Accordion component ported from shadcn/ui
//!
//! A vertically stacked set of interactive headings that reveal content.
//!
//! Reference: <https://ui.shadcn.com/docs/components/accordion>

use egui::{Id, Ui, Sense, Vec2, Pos2};
use crate::theme::ShadcnTheme;

/// Accordion type determining expansion behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccordionType {
    /// Only one item can be expanded at a time
    Single,
    /// Multiple items can be expanded simultaneously
    Multiple,
}

/// Accordion component for expandable content sections
///
/// ## Example
/// ```rust,ignore
/// Accordion::new("my_accordion")
///     .accordion_type(AccordionType::Single)
///     .collapsible(true)
///     .item("item-1", "Is it accessible?", |ui| {
///         ui.label("Yes. It adheres to the WAI-ARIA design pattern.");
///     })
///     .item("item-2", "Is it styled?", |ui| {
///         ui.label("Yes. It comes with default styles from shadcn.");
///     })
///     .show(ui);
/// ```
pub struct Accordion<'a> {
    id: Id,
    accordion_type: AccordionType,
    collapsible: bool,
    default_value: Option<String>,
    items: Vec<AccordionItem<'a>>,
}

struct AccordionItem<'a> {
    value: String,
    trigger: String,
    content: Box<dyn FnOnce(&mut Ui) + 'a>,
}

impl<'a> Accordion<'a> {
    /// Create a new accordion
    pub fn new(id: impl std::hash::Hash) -> Self {
        Self {
            id: Id::new(id),
            accordion_type: AccordionType::Single,
            collapsible: true,
            default_value: None,
            items: Vec::new(),
        }
    }

    /// Set the accordion type (single or multiple expansion)
    pub fn accordion_type(mut self, accordion_type: AccordionType) -> Self {
        self.accordion_type = accordion_type;
        self
    }

    /// Set whether all items can be collapsed (only for Single type)
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Set the default expanded item value
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Add an accordion item
    pub fn item(
        mut self,
        value: impl Into<String>,
        trigger: impl Into<String>,
        content: impl FnOnce(&mut Ui) + 'a,
    ) -> Self {
        self.items.push(AccordionItem {
            value: value.into(),
            trigger: trigger.into(),
            content: Box::new(content),
        });
        self
    }

    /// Show the accordion
    pub fn show(self, ui: &mut Ui) {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        // Get or initialize expanded state from memory
        let state_id = self.id.with("state");
        let mut expanded: Vec<String> = ui.ctx().data(|d| {
            d.get_temp::<Vec<String>>(state_id).unwrap_or_else(|| {
                // Initialize with default value if provided
                self.default_value.clone().map(|v| vec![v]).unwrap_or_default()
            })
        });

        let item_count = self.items.len();

        for (idx, item) in self.items.into_iter().enumerate() {
            let is_last = idx == item_count - 1;
            let is_expanded = expanded.contains(&item.value);
            let item_id = self.id.with(&item.value);

            // Draw trigger with chevron at fixed position for alignment
            let row_start_x = ui.cursor().min.x;
            let chevron_x_offset = 180.0; // Fixed position for all chevrons

            let trigger_response = ui.horizontal(|ui| {
                ui.set_min_height(44.0); // Apple HIG touch target

                // Trigger text
                ui.label(
                    egui::RichText::new(&item.trigger)
                        .size(theme.typography.body().size)
                        .color(theme.colors.foreground),
                );
            });

            // Draw chevron at fixed x position (outside the horizontal layout)
            let rotation = ui.ctx().animate_bool_with_time(
                item_id.with("chevron"),
                is_expanded,
                ui.style().animation_time,
            );

            let row_rect = trigger_response.response.rect;
            let cx = row_start_x + chevron_x_offset;
            let cy = row_rect.center().y;
            let chevron_size = 4.0;
            let stroke = egui::Stroke::new(1.5, theme.colors.muted_foreground);

            if rotation > 0.5 {
                // Up chevron (^ shape) when expanded
                ui.painter().line_segment(
                    [Pos2::new(cx - chevron_size, cy + chevron_size * 0.5),
                     Pos2::new(cx, cy - chevron_size * 0.5)],
                    stroke,
                );
                ui.painter().line_segment(
                    [Pos2::new(cx, cy - chevron_size * 0.5),
                     Pos2::new(cx + chevron_size, cy + chevron_size * 0.5)],
                    stroke,
                );
            } else {
                // Down chevron (v shape) when collapsed
                ui.painter().line_segment(
                    [Pos2::new(cx - chevron_size, cy - chevron_size * 0.5),
                     Pos2::new(cx, cy + chevron_size * 0.5)],
                    stroke,
                );
                ui.painter().line_segment(
                    [Pos2::new(cx, cy + chevron_size * 0.5),
                     Pos2::new(cx + chevron_size, cy - chevron_size * 0.5)],
                    stroke,
                );
            }

            // Make entire row (text + chevron area) clickable
            let clickable_rect = egui::Rect::from_min_max(
                row_rect.min,
                Pos2::new(cx + chevron_size + 8.0, row_rect.max.y),
            );
            let click_response = ui.interact(
                clickable_rect,
                item_id.with("trigger"),
                Sense::click(),
            );

            if click_response.clicked() {
                match self.accordion_type {
                    AccordionType::Single => {
                        if is_expanded {
                            if self.collapsible {
                                expanded.clear();
                            }
                        } else {
                            expanded.clear();
                            expanded.push(item.value.clone());
                        }
                    }
                    AccordionType::Multiple => {
                        if is_expanded {
                            expanded.retain(|v| v != &item.value);
                        } else {
                            expanded.push(item.value.clone());
                        }
                    }
                }
            }

            // Hover effect
            if click_response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }

            // Draw content with animation
            let openness = ui.ctx().animate_bool_with_time(
                item_id.with("content"),
                is_expanded,
                ui.style().animation_time,
            );

            if openness > 0.0 {
                ui.scope(|ui| {
                    // Apply opacity animation
                    if openness < 1.0 {
                        ui.set_opacity(openness);
                    }

                    // Content padding
                    ui.add_space(theme.spacing.xs);
                    egui::Frame::NONE
                        .inner_margin(egui::Margin {
                            left: 0,
                            right: 0,
                            top: 0,
                            bottom: theme.spacing.md as i8,
                        })
                        .show(ui, |ui| {
                            (item.content)(ui);
                        });
                });
            }

            // Draw separator (except for last item)
            if !is_last {
                ui.add_space(1.0);
                let rect = ui.available_rect_before_wrap();
                let separator_rect = egui::Rect::from_min_size(
                    rect.min,
                    egui::vec2(ui.available_width(), 1.0),
                );
                ui.painter().rect_filled(
                    separator_rect,
                    0.0,
                    theme.colors.border,
                );
                ui.add_space(1.0);
            }
        }

        // Save expanded state
        ui.ctx().data_mut(|d| d.insert_temp(state_id, expanded));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accordion_creation() {
        let accordion = Accordion::new("test")
            .accordion_type(AccordionType::Multiple)
            .collapsible(false)
            .default_value("item-1");

        assert_eq!(accordion.accordion_type, AccordionType::Multiple);
        assert!(!accordion.collapsible);
        assert_eq!(accordion.default_value, Some("item-1".to_string()));
    }
}
