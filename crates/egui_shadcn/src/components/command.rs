//! Command component ported from shadcn/ui
//!
//! A command palette / quick action menu (Cmd+K style).
//!
//! Reference: <https://ui.shadcn.com/docs/components/command>

use egui::{Id, Ui, Sense, Vec2, Color32};
use crate::theme::ShadcnTheme;

/// Command palette component
///
/// ## Example
/// ```rust,ignore
/// let mut open = false;
/// let mut search = String::new();
///
/// // Toggle with keyboard shortcut
/// if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::K)) {
///     open = !open;
/// }
///
/// Command::new("cmd", &mut open, &mut search)
///     .group("Suggestions", |cmd| {
///         cmd.item("Calendar", || println!("Open calendar"))
///            .item("Search Emoji", || println!("Search emoji"))
///     })
///     .group("Settings", |cmd| {
///         cmd.item("Profile", || println!("Open profile"))
///            .item("Settings", || println!("Open settings"))
///     })
///     .show(ui);
/// ```
pub struct Command<'a> {
    id: Id,
    open: &'a mut bool,
    search: &'a mut String,
    placeholder: String,
    groups: Vec<CommandGroup>,
    width: f32,
    max_height: f32,
}

/// A group of command items
struct CommandGroup {
    label: String,
    items: Vec<CommandItem>,
}

/// A single command item
struct CommandItem {
    label: String,
    shortcut: Option<String>,
    icon: Option<String>,
}

impl<'a> Command<'a> {
    /// Create a new command palette
    pub fn new(id: impl Into<Id>, open: &'a mut bool, search: &'a mut String) -> Self {
        Self {
            id: id.into(),
            open,
            search,
            placeholder: "Type a command or search...".to_string(),
            groups: Vec::new(),
            width: 500.0,
            max_height: 400.0,
        }
    }

    /// Set the search placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Add a group of commands
    pub fn group(mut self, label: impl Into<String>, items: impl FnOnce(&mut CommandGroupBuilder)) -> Self {
        let mut builder = CommandGroupBuilder { items: Vec::new() };
        items(&mut builder);
        self.groups.push(CommandGroup {
            label: label.into(),
            items: builder.items,
        });
        self
    }

    /// Set the palette width (default: 500px)
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the maximum height (default: 400px)
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    /// Show the command palette
    ///
    /// Returns the index of the selected item as (group_index, item_index) if any
    pub fn show(self, ui: &mut Ui) -> Option<(usize, usize)> {
        if !*self.open {
            return None;
        }

        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        #[allow(deprecated)]
        let screen_rect = ui.ctx().screen_rect();
        let mut selected_item = None;

        // Draw backdrop
        let backdrop_layer = egui::LayerId::new(egui::Order::Middle, self.id.with("backdrop"));
        ui.ctx().layer_painter(backdrop_layer).rect_filled(
            screen_rect,
            0.0,
            Color32::from_black_alpha(100),
        );

        // Calculate position (centered, near top)
        let dialog_pos = egui::pos2(
            screen_rect.center().x - self.width / 2.0,
            screen_rect.min.y + 100.0,
        );

        // Draw command palette
        egui::Area::new(self.id.with("dialog"))
            .order(egui::Order::Foreground)
            .fixed_pos(dialog_pos)
            .show(ui.ctx(), |ui| {
                let frame = egui::Frame::NONE
                    .fill(theme.colors.popover)
                    .stroke(egui::Stroke::new(1.0, theme.colors.border))
                    .corner_radius(theme.radii.lg)
                    .shadow(theme.shadows.lg);

                frame.show(ui, |ui| {
                    ui.set_min_width(self.width);
                    ui.set_max_width(self.width);

                    // Search input
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.add_space(12.0);

                        // Search icon
                        ui.label(
                            egui::RichText::new("O")
                                .size(16.0)
                                .color(theme.colors.muted_foreground)
                        );

                        ui.add_space(8.0);

                        // Search input
                        let response = ui.add_sized(
                            Vec2::new(self.width - 60.0, 36.0),
                            egui::TextEdit::singleline(self.search)
                                .hint_text(&self.placeholder)
                                .frame(false)
                        );

                        // Auto-focus the search input
                        if response.gained_focus() || !response.has_focus() {
                            response.request_focus();
                        }

                        ui.add_space(12.0);
                    });

                    // Separator
                    ui.add_space(4.0);
                    let sep_rect = ui.available_rect_before_wrap();
                    ui.painter().line_segment(
                        [
                            egui::pos2(sep_rect.min.x, sep_rect.min.y),
                            egui::pos2(sep_rect.max.x, sep_rect.min.y),
                        ],
                        egui::Stroke::new(1.0, theme.colors.border),
                    );
                    ui.add_space(4.0);

                    // Command list
                    let search_lower = self.search.to_lowercase();

                    egui::ScrollArea::vertical()
                        .max_height(self.max_height - 60.0)
                        .show(ui, |ui| {
                            ui.add_space(4.0);

                            for (group_idx, group) in self.groups.iter().enumerate() {
                                // Filter items by search
                                let visible_items: Vec<(usize, &CommandItem)> = group.items.iter()
                                    .enumerate()
                                    .filter(|(_, item)| {
                                        search_lower.is_empty() ||
                                        item.label.to_lowercase().contains(&search_lower)
                                    })
                                    .collect();

                                if visible_items.is_empty() {
                                    continue;
                                }

                                // Group label
                                ui.add_space(4.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(12.0);
                                    ui.label(
                                        egui::RichText::new(&group.label)
                                            .size(theme.typography.small().size - 1.0)
                                            .color(theme.colors.muted_foreground)
                                    );
                                });
                                ui.add_space(4.0);

                                // Items
                                for (item_idx, item) in visible_items {
                                    let item_response = ui.allocate_response(
                                        Vec2::new(self.width - 8.0, 36.0),
                                        Sense::click(),
                                    );

                                    if ui.is_rect_visible(item_response.rect) {
                                        let hovered = item_response.hovered();

                                        // Hover background
                                        if hovered {
                                            ui.painter().rect_filled(
                                                item_response.rect.shrink2(egui::vec2(4.0, 0.0)),
                                                theme.radii.sm,
                                                theme.colors.accent,
                                            );
                                        }

                                        let text_color = if hovered {
                                            theme.colors.accent_foreground
                                        } else {
                                            theme.colors.popover_foreground
                                        };

                                        // Icon placeholder
                                        if let Some(ref icon) = item.icon {
                                            ui.painter().text(
                                                egui::pos2(
                                                    item_response.rect.min.x + 16.0,
                                                    item_response.rect.center().y,
                                                ),
                                                egui::Align2::LEFT_CENTER,
                                                icon,
                                                egui::FontId::proportional(14.0),
                                                text_color,
                                            );
                                        }

                                        // Label
                                        ui.painter().text(
                                            egui::pos2(
                                                item_response.rect.min.x + if item.icon.is_some() { 40.0 } else { 16.0 },
                                                item_response.rect.center().y,
                                            ),
                                            egui::Align2::LEFT_CENTER,
                                            &item.label,
                                            egui::FontId::proportional(theme.typography.small().size),
                                            text_color,
                                        );

                                        // Shortcut
                                        if let Some(ref shortcut) = item.shortcut {
                                            ui.painter().text(
                                                egui::pos2(
                                                    item_response.rect.max.x - 16.0,
                                                    item_response.rect.center().y,
                                                ),
                                                egui::Align2::RIGHT_CENTER,
                                                shortcut,
                                                egui::FontId::proportional(theme.typography.small().size - 2.0),
                                                theme.colors.muted_foreground,
                                            );
                                        }
                                    }

                                    if item_response.clicked() {
                                        selected_item = Some((group_idx, item_idx));
                                        *self.open = false;
                                    }
                                }
                            }

                            // Empty state
                            if self.groups.iter().all(|g| {
                                g.items.iter().all(|i| {
                                    !search_lower.is_empty() &&
                                    !i.label.to_lowercase().contains(&search_lower)
                                })
                            }) {
                                ui.add_space(20.0);
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        egui::RichText::new("No results found.")
                                            .size(theme.typography.small().size)
                                            .color(theme.colors.muted_foreground)
                                    );
                                });
                                ui.add_space(20.0);
                            }

                            ui.add_space(4.0);
                        });
                });
            });

        // Handle escape key
        if ui.ctx().input(|i| i.key_pressed(egui::Key::Escape)) {
            *self.open = false;
        }

        selected_item
    }
}

/// Builder for command group items
pub struct CommandGroupBuilder {
    items: Vec<CommandItem>,
}

impl CommandGroupBuilder {
    /// Add a command item
    pub fn item(&mut self, label: impl Into<String>) -> &mut Self {
        self.items.push(CommandItem {
            label: label.into(),
            shortcut: None,
            icon: None,
        });
        self
    }

    /// Add a command item with shortcut
    pub fn item_with_shortcut(&mut self, label: impl Into<String>, shortcut: impl Into<String>) -> &mut Self {
        self.items.push(CommandItem {
            label: label.into(),
            shortcut: Some(shortcut.into()),
            icon: None,
        });
        self
    }

    /// Add a command item with icon
    pub fn item_with_icon(&mut self, label: impl Into<String>, icon: impl Into<String>) -> &mut Self {
        self.items.push(CommandItem {
            label: label.into(),
            shortcut: None,
            icon: Some(icon.into()),
        });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let mut open = true;
        let mut search = String::new();

        let cmd = Command::new("test", &mut open, &mut search)
            .placeholder("Search...")
            .width(600.0);

        assert_eq!(cmd.placeholder, "Search...");
        assert_eq!(cmd.width, 600.0);
    }
}
