//! Context Menu component ported from shadcn/ui
//!
//! A menu that appears on right-click, providing contextual actions.
//!
//! Reference: <https://ui.shadcn.com/docs/components/context-menu>

use egui::{Id, Response, Ui, Sense, Pos2};
use crate::theme::ShadcnTheme;

/// Context Menu component for right-click menus
///
/// ## Example
/// ```rust,ignore
/// // Wrap content that should have a context menu
/// let response = ContextMenu::new("my_context")
///     .item("Copy", || println!("Copy"))
///     .item("Paste", || println!("Paste"))
///     .separator()
///     .item("Delete", || println!("Delete"))
///     .show(ui, |ui| {
///         ui.label("Right-click me!");
///     });
/// ```
pub struct ContextMenu<'a> {
    id: &'a str,
    items: Vec<ContextMenuItem>,
    width: f32,
}

/// A context menu item type
enum ContextMenuItem {
    Action {
        label: String,
        shortcut: Option<String>,
        enabled: bool,
        destructive: bool,
    },
    Separator,
    Label(String),
}

impl<'a> ContextMenu<'a> {
    /// Create a new context menu
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            items: Vec::new(),
            width: 180.0,
        }
    }

    /// Add a menu item
    pub fn item(mut self, label: impl Into<String>) -> Self {
        self.items.push(ContextMenuItem::Action {
            label: label.into(),
            shortcut: None,
            enabled: true,
            destructive: false,
        });
        self
    }

    /// Add a menu item with keyboard shortcut
    pub fn item_with_shortcut(mut self, label: impl Into<String>, shortcut: impl Into<String>) -> Self {
        self.items.push(ContextMenuItem::Action {
            label: label.into(),
            shortcut: Some(shortcut.into()),
            enabled: true,
            destructive: false,
        });
        self
    }

    /// Add a destructive (red) menu item
    pub fn destructive_item(mut self, label: impl Into<String>) -> Self {
        self.items.push(ContextMenuItem::Action {
            label: label.into(),
            shortcut: None,
            enabled: true,
            destructive: true,
        });
        self
    }

    /// Add a disabled menu item
    pub fn disabled_item(mut self, label: impl Into<String>) -> Self {
        self.items.push(ContextMenuItem::Action {
            label: label.into(),
            shortcut: None,
            enabled: false,
            destructive: false,
        });
        self
    }

    /// Add a separator
    pub fn separator(mut self) -> Self {
        self.items.push(ContextMenuItem::Separator);
        self
    }

    /// Add a label/header
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.items.push(ContextMenuItem::Label(text.into()));
        self
    }

    /// Set the menu width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Show the context menu for content
    ///
    /// Returns the response from the content area and the index of any clicked item.
    pub fn show<R>(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> ContextMenuResponse<R> {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let id = Id::new(self.id);
        let menu_state_id = id.with("state");

        // Get menu state (position where it was opened)
        let menu_pos: Option<Pos2> = ui.ctx().data(|d| d.get_temp(menu_state_id));

        // Render the content
        let content_response = ui.scope(content);
        let inner = content_response.inner;
        let response = content_response.response;
        let content_rect = response.rect;

        // Check for right-click to open menu using input state directly
        // This bypasses widget interaction issues
        let secondary_clicked = ui.input(|i| {
            i.pointer.secondary_clicked() &&
            i.pointer.interact_pos().map_or(false, |pos| content_rect.contains(pos))
        });

        if secondary_clicked {
            if let Some(pos) = ui.ctx().pointer_interact_pos() {
                ui.ctx().data_mut(|d| d.insert_temp(menu_state_id, pos));
            }
        }

        let mut clicked_item = None;

        // Show menu if open
        if let Some(pos) = menu_pos {
            let menu_id = id.with("menu");

            let area_response = egui::Area::new(menu_id)
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
                        ui.set_min_width(self.width);

                        let mut action_index = 0;

                        for item in &self.items {
                            match item {
                                ContextMenuItem::Action { label, shortcut, enabled, destructive } => {
                                    let current_index = action_index;
                                    action_index += 1;

                                    let item_response = ui.allocate_response(
                                        egui::vec2(ui.available_width(), 32.0),
                                        if *enabled { Sense::click() } else { Sense::hover() },
                                    );

                                    if item_response.clicked() && *enabled {
                                        clicked_item = Some(current_index);
                                        // Close menu
                                        ui.ctx().data_mut(|d| d.remove::<Pos2>(menu_state_id));
                                    }

                                    if ui.is_rect_visible(item_response.rect) {
                                        let hovered = item_response.hovered() && *enabled;

                                        // Hover background
                                        if hovered {
                                            ui.painter().rect_filled(
                                                item_response.rect,
                                                theme.radii.sm,
                                                if *destructive {
                                                    theme.colors.destructive.linear_multiply(0.1)
                                                } else {
                                                    theme.colors.accent
                                                },
                                            );
                                        }

                                        // Text color
                                        let text_color = if *destructive {
                                            if *enabled {
                                                theme.colors.destructive
                                            } else {
                                                theme.colors.destructive.linear_multiply(0.5)
                                            }
                                        } else if *enabled {
                                            if hovered {
                                                theme.colors.accent_foreground
                                            } else {
                                                theme.colors.popover_foreground
                                            }
                                        } else {
                                            theme.colors.popover_foreground.linear_multiply(0.5)
                                        };

                                        // Label
                                        ui.painter().text(
                                            egui::pos2(
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
                                                egui::pos2(
                                                    item_response.rect.max.x - 8.0,
                                                    item_response.rect.center().y,
                                                ),
                                                egui::Align2::RIGHT_CENTER,
                                                shortcut,
                                                egui::FontId::proportional(theme.typography.small().size - 1.0),
                                                theme.colors.muted_foreground,
                                            );
                                        }
                                    }
                                }
                                ContextMenuItem::Separator => {
                                    let separator_rect = ui.allocate_space(egui::vec2(ui.available_width(), 9.0)).1;
                                    ui.painter().line_segment(
                                        [
                                            egui::pos2(separator_rect.min.x + 4.0, separator_rect.center().y),
                                            egui::pos2(separator_rect.max.x - 4.0, separator_rect.center().y),
                                        ],
                                        egui::Stroke::new(1.0, theme.colors.border),
                                    );
                                }
                                ContextMenuItem::Label(text) => {
                                    let label_response = ui.allocate_response(
                                        egui::vec2(ui.available_width(), 24.0),
                                        Sense::hover(),
                                    );

                                    if ui.is_rect_visible(label_response.rect) {
                                        ui.painter().text(
                                            egui::pos2(
                                                label_response.rect.min.x + 8.0,
                                                label_response.rect.center().y,
                                            ),
                                            egui::Align2::LEFT_CENTER,
                                            text,
                                            egui::FontId::proportional(theme.typography.small().size - 1.0),
                                            theme.colors.muted_foreground,
                                        );
                                    }
                                }
                            }
                        }
                    });
                });

            // Close menu when clicking outside
            if area_response.response.clicked_elsewhere() {
                ui.ctx().data_mut(|d| d.remove::<Pos2>(menu_state_id));
            }

            // Close on escape
            if ui.ctx().input(|i| i.key_pressed(egui::Key::Escape)) {
                ui.ctx().data_mut(|d| d.remove::<Pos2>(menu_state_id));
            }
        }

        ContextMenuResponse {
            inner,
            response,
            clicked_item,
        }
    }
}

/// Response from showing a context menu
pub struct ContextMenuResponse<R> {
    /// The return value from the content closure
    pub inner: R,
    /// The response from the content area
    pub response: Response,
    /// Index of clicked menu item, if any (only counts Action items, not separators/labels)
    pub clicked_item: Option<usize>,
}

/// Extension trait for adding context menus to responses
pub trait ContextMenuExt {
    /// Show a context menu on right-click
    fn context_menu<R>(
        &self,
        ui: &mut Ui,
        id: impl Into<Id>,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R>;
}

impl ContextMenuExt for Response {
    fn context_menu<R>(
        &self,
        ui: &mut Ui,
        id: impl Into<Id>,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R> {
        let id = id.into();
        let menu_state_id = id.with("state");

        // Get menu state
        let menu_pos: Option<Pos2> = ui.ctx().data(|d| d.get_temp(menu_state_id));

        // Check for right-click to open menu
        if self.secondary_clicked() {
            if let Some(pos) = ui.ctx().pointer_interact_pos() {
                ui.ctx().data_mut(|d| d.insert_temp(menu_state_id, pos));
            }
        }

        // Show menu if open
        if let Some(pos) = menu_pos {
            let theme = ui.ctx().data(|d| {
                d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                    .unwrap_or_else(ShadcnTheme::light)
            });

            let menu_id = id.with("menu");
            let mut result = None;

            let area_response = egui::Area::new(menu_id)
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
                        result = Some(add_contents(ui));
                    });
                });

            // Close menu when clicking outside
            if area_response.response.clicked_elsewhere() {
                ui.ctx().data_mut(|d| d.remove::<Pos2>(menu_state_id));
            }

            // Close on escape
            if ui.ctx().input(|i| i.key_pressed(egui::Key::Escape)) {
                ui.ctx().data_mut(|d| d.remove::<Pos2>(menu_state_id));
            }

            return result;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_menu_creation() {
        let menu = ContextMenu::new("test")
            .item("Copy")
            .separator()
            .item("Paste")
            .destructive_item("Delete");

        assert_eq!(menu.items.len(), 4);
    }
}
