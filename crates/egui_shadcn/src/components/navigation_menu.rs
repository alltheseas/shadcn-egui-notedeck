//! Navigation Menu component ported from shadcn/ui
//!
//! A collection of links for navigating websites with dropdown support.
//!
//! Reference: <https://ui.shadcn.com/docs/components/navigation-menu>

use egui::{Id, Ui, Sense, Vec2, Pos2};
use crate::theme::ShadcnTheme;

/// Navigation Menu component for website navigation
///
/// ## Example
/// ```rust,ignore
/// NavigationMenu::new("main_nav")
///     .item("Home", || println!("Home"))
///     .dropdown("Products", |dropdown| {
///         dropdown
///             .item("All Products", "Browse our catalog")
///             .item("New Arrivals", "Latest additions")
///             .separator()
///             .item("Sale", "Discounted items");
///     })
///     .item("About", || println!("About"))
///     .show(ui);
/// ```
pub struct NavigationMenu<'a> {
    id: &'a str,
    items: Vec<NavItem>,
}

enum NavItem {
    Link {
        label: String,
    },
    Dropdown {
        label: String,
        items: Vec<DropdownItem>,
    },
}

struct DropdownItem {
    label: String,
    description: Option<String>,
}

impl<'a> NavigationMenu<'a> {
    /// Create a new navigation menu
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            items: Vec::new(),
        }
    }

    /// Add a simple navigation link
    pub fn item(mut self, label: impl Into<String>) -> Self {
        self.items.push(NavItem::Link {
            label: label.into(),
        });
        self
    }

    /// Add a dropdown menu
    pub fn dropdown(mut self, label: impl Into<String>, build: impl FnOnce(&mut NavDropdownBuilder)) -> Self {
        let mut builder = NavDropdownBuilder { items: Vec::new() };
        build(&mut builder);
        self.items.push(NavItem::Dropdown {
            label: label.into(),
            items: builder.items,
        });
        self
    }

    /// Show the navigation menu
    pub fn show(self, ui: &mut Ui) -> NavigationMenuResponse {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let id = Id::new(self.id);
        let open_dropdown_id = id.with("open_dropdown");

        // Get currently open dropdown index
        let open_dropdown: Option<usize> = ui.ctx().data(|d| d.get_temp(open_dropdown_id));

        let mut clicked_item: Option<(usize, Option<usize>)> = None;
        let mut new_open_dropdown = open_dropdown;

        // Navigation container
        egui::Frame::NONE
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;

                    for (item_idx, item) in self.items.iter().enumerate() {
                        match item {
                            NavItem::Link { label } => {
                                let response = self.draw_nav_link(ui, &theme, label, false, false);
                                if response.clicked() {
                                    clicked_item = Some((item_idx, None));
                                }
                            }
                            NavItem::Dropdown { label, items } => {
                                let is_open = open_dropdown == Some(item_idx);
                                let response = self.draw_nav_link(ui, &theme, label, is_open, true);

                                // Toggle on click, open on hover if another is open
                                if response.clicked() {
                                    new_open_dropdown = if is_open { None } else { Some(item_idx) };
                                } else if response.hovered() && open_dropdown.is_some() && !is_open {
                                    new_open_dropdown = Some(item_idx);
                                }

                                // Show dropdown content
                                if is_open {
                                    let dropdown_pos = Pos2::new(
                                        response.rect.min.x,
                                        response.rect.max.y + 4.0,
                                    );

                                    if let Some(sub_idx) = self.show_dropdown_content(
                                        ui, &theme, id, item_idx, items, dropdown_pos
                                    ) {
                                        clicked_item = Some((item_idx, Some(sub_idx)));
                                        new_open_dropdown = None;
                                    }
                                }
                            }
                        }
                    }
                });
            });

        // Update open dropdown state
        if new_open_dropdown != open_dropdown {
            ui.ctx().data_mut(|d| {
                if let Some(idx) = new_open_dropdown {
                    d.insert_temp(open_dropdown_id, idx);
                } else {
                    d.remove::<usize>(open_dropdown_id);
                }
            });
        }

        // Close on escape
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            ui.ctx().data_mut(|d| d.remove::<usize>(open_dropdown_id));
        }

        NavigationMenuResponse { clicked_item }
    }

    fn draw_nav_link(&self, ui: &mut Ui, theme: &ShadcnTheme, label: &str, is_open: bool, _has_dropdown: bool) -> egui::Response {
        let font_id = egui::FontId::proportional(theme.typography.small().size);
        let padding = Vec2::new(16.0, 10.0);
        let approx_width = theme.typography.small().size * 0.55 * label.len() as f32;
        let size = Vec2::new(approx_width + padding.x * 2.0, 40.0);

        let (rect, response) = ui.allocate_exact_size(size, Sense::click());

        if ui.is_rect_visible(rect) {
            let hovered = response.hovered();

            // Background on hover/open
            if is_open || hovered {
                ui.painter().rect_filled(
                    rect,
                    theme.radii.md,
                    theme.colors.accent,
                );
            }

            // Text
            let text_color = if is_open || hovered {
                theme.colors.accent_foreground
            } else {
                theme.colors.foreground
            };

            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                label,
                font_id.clone(),
                text_color,
            );
        }

        response
    }

    fn show_dropdown_content(
        &self,
        ui: &mut Ui,
        theme: &ShadcnTheme,
        id: Id,
        item_idx: usize,
        items: &[DropdownItem],
        pos: Pos2,
    ) -> Option<usize> {
        let mut clicked_idx = None;
        let dropdown_id = id.with(("dropdown_content", item_idx));

        let area_response = egui::Area::new(dropdown_id)
            .order(egui::Order::Foreground)
            .fixed_pos(pos)
            .show(ui.ctx(), |ui| {
                let frame = egui::Frame::NONE
                    .fill(theme.colors.popover)
                    .stroke(egui::Stroke::new(1.0, theme.colors.border))
                    .corner_radius(theme.radii.md)
                    .shadow(theme.shadows.md)
                    .inner_margin(8.0);

                frame.show(ui, |ui| {
                    ui.set_min_width(220.0);

                    for (idx, item) in items.iter().enumerate() {
                        let item_height = if item.description.is_some() { 56.0 } else { 40.0 };
                        let item_response = ui.allocate_response(
                            Vec2::new(ui.available_width(), item_height),
                            Sense::click(),
                        );

                        if item_response.clicked() {
                            clicked_idx = Some(idx);
                        }

                        if ui.is_rect_visible(item_response.rect) {
                            let hovered = item_response.hovered();

                            if hovered {
                                ui.painter().rect_filled(
                                    item_response.rect,
                                    theme.radii.sm,
                                    theme.colors.accent,
                                );
                            }

                            let text_color = if hovered {
                                theme.colors.accent_foreground
                            } else {
                                theme.colors.popover_foreground
                            };

                            // Label
                            let label_y = if item.description.is_some() {
                                item_response.rect.min.y + 14.0
                            } else {
                                item_response.rect.center().y
                            };

                            ui.painter().text(
                                Pos2::new(item_response.rect.min.x + 12.0, label_y),
                                egui::Align2::LEFT_CENTER,
                                &item.label,
                                egui::FontId::proportional(theme.typography.small().size),
                                text_color,
                            );

                            // Description
                            if let Some(ref desc) = item.description {
                                ui.painter().text(
                                    Pos2::new(
                                        item_response.rect.min.x + 12.0,
                                        item_response.rect.min.y + 34.0,
                                    ),
                                    egui::Align2::LEFT_CENTER,
                                    desc,
                                    egui::FontId::proportional(theme.typography.small().size - 2.0),
                                    theme.colors.muted_foreground,
                                );
                            }
                        }
                    }
                });
            });

        // Close when clicking outside
        if area_response.response.clicked_elsewhere() {
            ui.ctx().data_mut(|d| d.remove::<usize>(id.with("open_dropdown")));
        }

        clicked_idx
    }
}

/// Builder for dropdown items
pub struct NavDropdownBuilder {
    items: Vec<DropdownItem>,
}

impl NavDropdownBuilder {
    /// Add a dropdown item
    pub fn item(&mut self, label: impl Into<String>, description: impl Into<String>) -> &mut Self {
        self.items.push(DropdownItem {
            label: label.into(),
            description: Some(description.into()),
        });
        self
    }

    /// Add a dropdown item without description
    pub fn item_simple(&mut self, label: impl Into<String>) -> &mut Self {
        self.items.push(DropdownItem {
            label: label.into(),
            description: None,
        });
        self
    }
}

/// Response from showing a navigation menu
pub struct NavigationMenuResponse {
    /// Clicked item as (item_index, sub_item_index if dropdown)
    pub clicked_item: Option<(usize, Option<usize>)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_menu_creation() {
        let nav = NavigationMenu::new("test")
            .item("Home")
            .dropdown("Products", |d| {
                d.item("Item 1", "Description 1");
            });

        assert_eq!(nav.items.len(), 2);
    }
}
