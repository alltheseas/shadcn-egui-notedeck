//! Menubar component ported from shadcn/ui
//!
//! A horizontal menu bar with dropdown menus (File, Edit, View style).
//!
//! Reference: <https://ui.shadcn.com/docs/components/menubar>

use egui::{Id, Ui, Sense, Vec2, Pos2};
use crate::theme::ShadcnTheme;

/// Menubar component for application menus
///
/// ## Example
/// ```rust,ignore
/// Menubar::new("app_menu")
///     .menu("File", |menu| {
///         menu.item("New", || println!("New"))
///             .item_with_shortcut("Open", "Ctrl+O", || println!("Open"))
///             .separator()
///             .item("Exit", || println!("Exit"));
///     })
///     .menu("Edit", |menu| {
///         menu.item_with_shortcut("Undo", "Ctrl+Z", || println!("Undo"))
///             .item_with_shortcut("Redo", "Ctrl+Y", || println!("Redo"));
///     })
///     .show(ui);
/// ```
pub struct Menubar<'a> {
    id: &'a str,
    menus: Vec<MenubarMenu>,
}

struct MenubarMenu {
    label: String,
    items: Vec<MenubarItem>,
}

enum MenubarItem {
    Action {
        label: String,
        shortcut: Option<String>,
        enabled: bool,
    },
    Separator,
    Submenu {
        label: String,
        items: Vec<MenubarItem>,
    },
}

impl<'a> Menubar<'a> {
    /// Create a new menubar
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            menus: Vec::new(),
        }
    }

    /// Add a menu to the menubar
    pub fn menu(mut self, label: impl Into<String>, build: impl FnOnce(&mut MenuBuilder)) -> Self {
        let mut builder = MenuBuilder { items: Vec::new() };
        build(&mut builder);
        self.menus.push(MenubarMenu {
            label: label.into(),
            items: builder.items,
        });
        self
    }

    /// Show the menubar
    pub fn show(self, ui: &mut Ui) -> MenubarResponse {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let id = Id::new(self.id);
        let open_menu_id = id.with("open_menu");

        // Get currently open menu index
        let open_menu: Option<usize> = ui.ctx().data(|d| d.get_temp(open_menu_id));

        let mut clicked_item: Option<(usize, usize)> = None;
        let mut new_open_menu = open_menu;

        // Menubar container - subtle rounded border like shadcn
        let menubar_response = egui::Frame::NONE
            .fill(theme.colors.background)
            .stroke(egui::Stroke::new(1.0, theme.colors.border))
            .corner_radius(theme.radii.md)
            .inner_margin(egui::Margin::symmetric(4, 4))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;

                    for (menu_idx, menu) in self.menus.iter().enumerate() {
                        let is_open = open_menu == Some(menu_idx);

                        // Menu trigger button
                        let button_response = self.draw_menu_trigger(
                            ui, &theme, &menu.label, is_open, menu_idx
                        );

                        // Open menu on click or hover when another menu is open
                        if button_response.clicked() {
                            new_open_menu = if is_open { None } else { Some(menu_idx) };
                        } else if button_response.hovered() && open_menu.is_some() && !is_open {
                            new_open_menu = Some(menu_idx);
                        }

                        // Show dropdown if open
                        if is_open {
                            let dropdown_pos = Pos2::new(
                                button_response.rect.min.x,
                                button_response.rect.max.y + 2.0,
                            );

                            if let Some(item_idx) = self.show_dropdown(
                                ui, &theme, id, menu_idx, &menu.items, dropdown_pos
                            ) {
                                clicked_item = Some((menu_idx, item_idx));
                                new_open_menu = None;
                            }
                        }
                    }
                });
            });

        // Update open menu state
        if new_open_menu != open_menu {
            ui.ctx().data_mut(|d| {
                if let Some(idx) = new_open_menu {
                    d.insert_temp(open_menu_id, idx);
                } else {
                    d.remove::<usize>(open_menu_id);
                }
            });
        }

        // Close menu when clicking outside
        if open_menu.is_some() {
            let clicked_outside = ui.input(|i| {
                i.pointer.any_click() &&
                !menubar_response.response.rect.contains(
                    i.pointer.interact_pos().unwrap_or(Pos2::ZERO)
                )
            });
            if clicked_outside {
                ui.ctx().data_mut(|d| d.remove::<usize>(open_menu_id));
            }
        }

        // Close on escape
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            ui.ctx().data_mut(|d| d.remove::<usize>(open_menu_id));
        }

        MenubarResponse { clicked_item }
    }

    fn draw_menu_trigger(
        &self,
        ui: &mut Ui,
        theme: &ShadcnTheme,
        label: &str,
        is_open: bool,
        _menu_idx: usize,
    ) -> egui::Response {
        let font_id = egui::FontId::proportional(theme.typography.small().size);
        let padding = Vec2::new(12.0, 8.0);
        // Approximate text width
        let approx_char_width = theme.typography.small().size * 0.55;
        let text_size = approx_char_width * label.len() as f32;
        let size = Vec2::new(text_size.max(40.0) + padding.x * 2.0, 36.0);

        let (rect, response) = ui.allocate_exact_size(size, Sense::click());

        if ui.is_rect_visible(rect) {
            let hovered = response.hovered();

            // Background - subtle gray like shadcn, not strong accent
            if is_open || hovered {
                ui.painter().rect_filled(
                    rect,
                    theme.radii.sm,
                    theme.colors.accent, // accent is already subtle in shadcn theme
                );
            }

            // Text - keep foreground color, shadcn doesn't change text color on hover
            let text_color = theme.colors.foreground;

            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                label,
                font_id,
                text_color,
            );
        }

        response
    }

    fn show_dropdown(
        &self,
        ui: &mut Ui,
        theme: &ShadcnTheme,
        id: Id,
        menu_idx: usize,
        items: &[MenubarItem],
        pos: Pos2,
    ) -> Option<usize> {
        let mut clicked_item = None;
        let dropdown_id = id.with(("dropdown", menu_idx));

        egui::Area::new(dropdown_id)
            .order(egui::Order::Foreground)
            .fixed_pos(pos)
            .show(ui.ctx(), |ui| {
                let frame = egui::Frame::NONE
                    .fill(theme.colors.popover)
                    .stroke(egui::Stroke::new(1.0, theme.colors.border))
                    .corner_radius(theme.radii.md)
                    .shadow(theme.shadows.md)
                    .inner_margin(4.0);

                frame.show(ui, |ui| {
                    ui.set_min_width(180.0);

                    let mut action_idx = 0;
                    for item in items {
                        match item {
                            MenubarItem::Action { label, shortcut, enabled } => {
                                let current_idx = action_idx;
                                action_idx += 1;

                                let item_response = ui.allocate_response(
                                    Vec2::new(ui.available_width(), 32.0),
                                    if *enabled { Sense::click() } else { Sense::hover() },
                                );

                                if item_response.clicked() && *enabled {
                                    clicked_item = Some(current_idx);
                                }

                                if ui.is_rect_visible(item_response.rect) {
                                    let hovered = item_response.hovered() && *enabled;

                                    if hovered {
                                        ui.painter().rect_filled(
                                            item_response.rect,
                                            theme.radii.sm,
                                            theme.colors.accent,
                                        );
                                    }

                                    let text_color = if *enabled {
                                        if hovered {
                                            theme.colors.accent_foreground
                                        } else {
                                            theme.colors.popover_foreground
                                        }
                                    } else {
                                        theme.colors.muted_foreground
                                    };

                                    // Label
                                    ui.painter().text(
                                        Pos2::new(
                                            item_response.rect.min.x + 8.0,
                                            item_response.rect.center().y,
                                        ),
                                        egui::Align2::LEFT_CENTER,
                                        label,
                                        egui::FontId::proportional(theme.typography.small().size),
                                        text_color,
                                    );

                                    // Shortcut
                                    if let Some(shortcut) = shortcut {
                                        ui.painter().text(
                                            Pos2::new(
                                                item_response.rect.max.x - 8.0,
                                                item_response.rect.center().y,
                                            ),
                                            egui::Align2::RIGHT_CENTER,
                                            shortcut,
                                            egui::FontId::proportional(theme.typography.small().size - 2.0),
                                            theme.colors.muted_foreground,
                                        );
                                    }
                                }
                            }
                            MenubarItem::Separator => {
                                let sep_rect = ui.allocate_space(Vec2::new(ui.available_width(), 9.0)).1;
                                ui.painter().line_segment(
                                    [
                                        Pos2::new(sep_rect.min.x + 4.0, sep_rect.center().y),
                                        Pos2::new(sep_rect.max.x - 4.0, sep_rect.center().y),
                                    ],
                                    egui::Stroke::new(1.0, theme.colors.border),
                                );
                            }
                            MenubarItem::Submenu { .. } => {
                                // TODO: Implement submenus if needed
                            }
                        }
                    }
                });
            });

        clicked_item
    }
}

/// Builder for menu items
pub struct MenuBuilder {
    items: Vec<MenubarItem>,
}

impl MenuBuilder {
    /// Add a menu item
    pub fn item(&mut self, label: impl Into<String>) -> &mut Self {
        self.items.push(MenubarItem::Action {
            label: label.into(),
            shortcut: None,
            enabled: true,
        });
        self
    }

    /// Add a menu item with keyboard shortcut
    pub fn item_with_shortcut(&mut self, label: impl Into<String>, shortcut: impl Into<String>) -> &mut Self {
        self.items.push(MenubarItem::Action {
            label: label.into(),
            shortcut: Some(shortcut.into()),
            enabled: true,
        });
        self
    }

    /// Add a disabled menu item
    pub fn disabled_item(&mut self, label: impl Into<String>) -> &mut Self {
        self.items.push(MenubarItem::Action {
            label: label.into(),
            shortcut: None,
            enabled: false,
        });
        self
    }

    /// Add a separator
    pub fn separator(&mut self) -> &mut Self {
        self.items.push(MenubarItem::Separator);
        self
    }
}

/// Response from showing a menubar
pub struct MenubarResponse {
    /// The clicked item as (menu_index, item_index), if any
    pub clicked_item: Option<(usize, usize)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menubar_creation() {
        let menubar = Menubar::new("test")
            .menu("File", |m| {
                m.item("New").separator().item("Exit");
            });

        assert_eq!(menubar.menus.len(), 1);
        assert_eq!(menubar.menus[0].items.len(), 3);
    }
}
