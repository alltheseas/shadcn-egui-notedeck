//! Toggle Group component ported from shadcn/ui
//!
//! A set of toggle buttons that can be switched on or off.
//!
//! Reference: <https://ui.shadcn.com/docs/components/toggle-group>

use egui::{Id, Response, Ui, Sense};
use crate::theme::ShadcnTheme;

/// Toggle group type determining selection behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleGroupType {
    /// Only one item can be selected at a time
    Single,
    /// Multiple items can be selected
    Multiple,
}

/// Toggle group variant for styling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleGroupVariant {
    /// Default style with background on selection
    Default,
    /// Outline style with border
    Outline,
}

/// Toggle Group component
///
/// ## Example
/// ```rust,ignore
/// ToggleGroup::new("alignment")
///     .group_type(ToggleGroupType::Single)
///     .item("left", "Left")
///     .item("center", "Center")
///     .item("right", "Right")
///     .show(ui);
/// ```
pub struct ToggleGroup<'a> {
    id: Id,
    group_type: ToggleGroupType,
    variant: ToggleGroupVariant,
    items: Vec<ToggleGroupItem>,
    selected: &'a mut Vec<String>,
}

struct ToggleGroupItem {
    value: String,
    label: String,
}

impl<'a> ToggleGroup<'a> {
    /// Create a new toggle group bound to a selection vector
    pub fn new(id: impl std::hash::Hash, selected: &'a mut Vec<String>) -> Self {
        Self {
            id: Id::new(id),
            group_type: ToggleGroupType::Single,
            variant: ToggleGroupVariant::Default,
            items: Vec::new(),
            selected,
        }
    }

    /// Set the toggle group type (single or multiple selection)
    pub fn group_type(mut self, group_type: ToggleGroupType) -> Self {
        self.group_type = group_type;
        self
    }

    /// Set the variant style
    pub fn variant(mut self, variant: ToggleGroupVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Add an item to the group
    pub fn item(mut self, value: impl Into<String>, label: impl Into<String>) -> Self {
        self.items.push(ToggleGroupItem {
            value: value.into(),
            label: label.into(),
        });
        self
    }

    /// Show the toggle group
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let response = ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0; // No gap between items for connected look

            let item_count = self.items.len();

            for (idx, item) in self.items.iter().enumerate() {
                let is_selected = self.selected.contains(&item.value);
                let is_first = idx == 0;
                let is_last = idx == item_count - 1;

                // Calculate corner radius for connected buttons
                let r = theme.radii.md;
                let corner_radius = if item_count == 1 {
                    egui::CornerRadius { nw: r, sw: r, ne: r, se: r }
                } else if is_first {
                    egui::CornerRadius { nw: r, sw: r, ne: 0, se: 0 }
                } else if is_last {
                    egui::CornerRadius { nw: 0, sw: 0, ne: r, se: r }
                } else {
                    egui::CornerRadius::ZERO
                };

                // Calculate button size
                let padding = egui::vec2(theme.spacing.md, 10.0);
                let text_galley = ui.painter().layout_no_wrap(
                    item.label.clone(),
                    egui::FontId::proportional(theme.typography.small().size),
                    egui::Color32::WHITE,
                );
                let button_size = egui::vec2(
                    text_galley.size().x + padding.x * 2.0,
                    44.0, // Apple HIG minimum
                );

                // Allocate button
                let (rect, response) = ui.allocate_exact_size(button_size, Sense::click());

                // Handle click
                if response.clicked() {
                    match self.group_type {
                        ToggleGroupType::Single => {
                            self.selected.clear();
                            self.selected.push(item.value.clone());
                        }
                        ToggleGroupType::Multiple => {
                            if is_selected {
                                self.selected.retain(|v| v != &item.value);
                            } else {
                                self.selected.push(item.value.clone());
                            }
                        }
                    }
                }

                if ui.is_rect_visible(rect) {
                    let hovered = response.hovered();

                    // Colors based on state and variant
                    let (bg_color, text_color, border_stroke) = match self.variant {
                        ToggleGroupVariant::Default => {
                            if is_selected {
                                (
                                    theme.colors.accent,
                                    theme.colors.accent_foreground,
                                    egui::Stroke::NONE,
                                )
                            } else if hovered {
                                (
                                    theme.colors.muted,
                                    theme.colors.foreground,
                                    egui::Stroke::NONE,
                                )
                            } else {
                                (
                                    egui::Color32::TRANSPARENT,
                                    theme.colors.foreground,
                                    egui::Stroke::NONE,
                                )
                            }
                        }
                        ToggleGroupVariant::Outline => {
                            let border = egui::Stroke::new(1.0, theme.colors.border);
                            if is_selected {
                                (
                                    theme.colors.accent,
                                    theme.colors.accent_foreground,
                                    border,
                                )
                            } else if hovered {
                                (
                                    theme.colors.muted,
                                    theme.colors.foreground,
                                    border,
                                )
                            } else {
                                (
                                    egui::Color32::TRANSPARENT,
                                    theme.colors.foreground,
                                    border,
                                )
                            }
                        }
                    };

                    // Draw background
                    ui.painter().rect_filled(rect, corner_radius, bg_color);

                    // Draw border for outline variant
                    if border_stroke != egui::Stroke::NONE {
                        ui.painter().rect_stroke(
                            rect,
                            corner_radius,
                            border_stroke,
                            egui::StrokeKind::Inside,
                        );
                    }

                    // Draw text
                    let text_galley = ui.painter().layout_no_wrap(
                        item.label.clone(),
                        egui::FontId::proportional(theme.typography.small().size),
                        text_color,
                    );
                    let text_pos = rect.center() - text_galley.size() / 2.0;
                    ui.painter().galley(text_pos, text_galley, text_color);
                }
            }
        });

        response.response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_group_creation() {
        let mut selected = vec![];
        let group = ToggleGroup::new("test", &mut selected)
            .group_type(ToggleGroupType::Multiple)
            .variant(ToggleGroupVariant::Outline)
            .item("a", "A")
            .item("b", "B");

        assert_eq!(group.group_type, ToggleGroupType::Multiple);
        assert_eq!(group.variant, ToggleGroupVariant::Outline);
        assert_eq!(group.items.len(), 2);
    }
}
