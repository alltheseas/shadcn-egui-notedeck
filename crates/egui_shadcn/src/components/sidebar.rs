//! Sidebar component ported from shadcn/ui
//!
//! A collapsible side navigation component.
//!
//! Reference: <https://ui.shadcn.com/docs/components/sidebar>

use egui::{Id, Ui, Sense, Vec2, Pos2};
use crate::theme::ShadcnTheme;

/// Sidebar component for side navigation
///
/// ## Example
/// ```rust,ignore
/// let mut sidebar_open = true;
///
/// Sidebar::new("main_sidebar", &mut sidebar_open)
///     .width(240.0)
///     .collapsible(true)
///     .header(|ui| {
///         ui.label("My App");
///     })
///     .item("Dashboard", "D", true, || println!("Dashboard"))
///     .item("Settings", "S", false, || println!("Settings"))
///     .separator()
///     .item("Logout", "L", false, || println!("Logout"))
///     .show(ui);
/// ```
pub struct Sidebar<'a> {
    id: &'a str,
    open: &'a mut bool,
    width: f32,
    collapsed_width: f32,
    collapsible: bool,
    items: Vec<SidebarItem>,
    header: Option<Box<dyn FnOnce(&mut Ui) + 'a>>,
    footer: Option<Box<dyn FnOnce(&mut Ui) + 'a>>,
}

enum SidebarItem {
    Link {
        label: String,
        icon: Option<String>,
        active: bool,
    },
    /// Link with a context menu (three dots) - for project items
    LinkWithMenu {
        label: String,
        icon: Option<String>,
        active: bool,
        menu_items: Vec<String>,
    },
    Separator,
    Section {
        label: String,
        items: Vec<SidebarItem>,
    },
    /// Collapsible menu item with sub-items (like Playground > History, Starred, Settings)
    Collapsible {
        label: String,
        icon: Option<String>,
        items: Vec<SidebarItem>,
    },
}

impl<'a> Sidebar<'a> {
    /// Create a new sidebar
    pub fn new(id: &'a str, open: &'a mut bool) -> Self {
        Self {
            id,
            open,
            width: 240.0,
            collapsed_width: 60.0,
            collapsible: true,
            items: Vec::new(),
            header: None,
            footer: None,
        }
    }

    /// Set the expanded width (default: 240px)
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the collapsed width (default: 60px)
    pub fn collapsed_width(mut self, width: f32) -> Self {
        self.collapsed_width = width;
        self
    }

    /// Enable/disable collapsible behavior (default: true)
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Add a header section
    pub fn header(mut self, header: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.header = Some(Box::new(header));
        self
    }

    /// Add a footer section
    pub fn footer(mut self, footer: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.footer = Some(Box::new(footer));
        self
    }

    /// Add a navigation item
    pub fn item(mut self, label: impl Into<String>, icon: impl Into<String>, active: bool) -> Self {
        self.items.push(SidebarItem::Link {
            label: label.into(),
            icon: Some(icon.into()),
            active,
        });
        self
    }

    /// Add a navigation item without icon
    pub fn item_no_icon(mut self, label: impl Into<String>, active: bool) -> Self {
        self.items.push(SidebarItem::Link {
            label: label.into(),
            icon: None,
            active,
        });
        self
    }

    /// Add a project item with context menu (three dots)
    /// Menu items will appear when the three dots are clicked
    pub fn project_item(mut self, label: impl Into<String>, icon: impl Into<String>, active: bool, menu_items: Vec<&str>) -> Self {
        self.items.push(SidebarItem::LinkWithMenu {
            label: label.into(),
            icon: Some(icon.into()),
            active,
            menu_items: menu_items.into_iter().map(|s| s.to_string()).collect(),
        });
        self
    }

    /// Add a separator
    pub fn separator(mut self) -> Self {
        self.items.push(SidebarItem::Separator);
        self
    }

    /// Add a section with label
    pub fn section(mut self, label: impl Into<String>, build: impl FnOnce(&mut SidebarBuilder)) -> Self {
        let mut builder = SidebarBuilder { items: Vec::new() };
        build(&mut builder);
        self.items.push(SidebarItem::Section {
            label: label.into(),
            items: builder.items,
        });
        self
    }

    /// Add a collapsible menu item with sub-items (like Playground > History, Starred, Settings)
    pub fn menu(mut self, label: impl Into<String>, icon: impl Into<String>, build: impl FnOnce(&mut SidebarBuilder)) -> Self {
        let mut builder = SidebarBuilder { items: Vec::new() };
        build(&mut builder);
        self.items.push(SidebarItem::Collapsible {
            label: label.into(),
            icon: Some(icon.into()),
            items: builder.items,
        });
        self
    }

    /// Show the sidebar
    pub fn show(self, ui: &mut Ui) -> SidebarResponse {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let current_width = if *self.open { self.width } else { self.collapsed_width };
        let is_collapsed = !*self.open;

        // Extract header and footer before entering closures
        let header = self.header;
        let footer = self.footer;
        let items = self.items;
        let collapsible = self.collapsible;

        let mut clicked_item: Option<usize> = None;
        let mut toggle_clicked = false;

        egui::Frame::NONE
            .fill(theme.colors.sidebar)
            .stroke(egui::Stroke::new(1.0, theme.colors.sidebar_border))
            .show(ui, |ui| {
                ui.set_min_width(current_width);
                ui.set_max_width(current_width);
                ui.set_min_height(ui.available_height());

                ui.vertical(|ui| {
                    // Header
                    if let Some(header) = header {
                        egui::Frame::NONE
                            .inner_margin(egui::Margin::symmetric(12, 12))
                            .show(ui, |ui| {
                                header(ui);
                            });

                        // Header separator
                        let sep_rect = ui.available_rect_before_wrap();
                        ui.painter().line_segment(
                            [
                                Pos2::new(sep_rect.min.x, sep_rect.min.y),
                                Pos2::new(sep_rect.max.x, sep_rect.min.y),
                            ],
                            egui::Stroke::new(1.0, theme.colors.sidebar_border),
                        );
                    }

                    // Toggle button (if collapsible)
                    if collapsible {
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.add_space(8.0);
                            let toggle_response = draw_toggle_button(ui, &theme, is_collapsed);
                            if toggle_response.clicked() {
                                toggle_clicked = true;
                            }
                        });
                        ui.add_space(8.0);
                    }

                    // Navigation items
                    ui.add_space(4.0);
                    let mut item_idx = 0;
                    for item in &items {
                        match item {
                            SidebarItem::Link { label, icon, active } => {
                                let current_idx = item_idx;
                                item_idx += 1;

                                let response = draw_nav_item(
                                    ui, &theme, label, icon.as_deref(), *active, is_collapsed
                                );
                                if response.clicked() {
                                    clicked_item = Some(current_idx);
                                }
                            }
                            SidebarItem::LinkWithMenu { label, icon, active, menu_items } => {
                                let current_idx = item_idx;
                                item_idx += 1;

                                let response = draw_nav_item_with_menu(
                                    ui, &theme, label, icon.as_deref(), *active, is_collapsed, menu_items, current_idx
                                );
                                if response.clicked() {
                                    clicked_item = Some(current_idx);
                                }
                            }
                            SidebarItem::Separator => {
                                ui.add_space(8.0);
                                let sep_rect = ui.available_rect_before_wrap();
                                ui.painter().line_segment(
                                    [
                                        Pos2::new(sep_rect.min.x + 12.0, sep_rect.min.y),
                                        Pos2::new(sep_rect.max.x - 12.0, sep_rect.min.y),
                                    ],
                                    egui::Stroke::new(1.0, theme.colors.sidebar_border),
                                );
                                ui.add_space(8.0);
                            }
                            SidebarItem::Section { label, items: sub_items } => {
                                if !is_collapsed {
                                    ui.add_space(12.0);
                                    ui.horizontal(|ui| {
                                        ui.add_space(16.0);
                                        ui.label(
                                            egui::RichText::new(label)
                                                .size(theme.typography.small().size - 2.0)
                                                .color(theme.colors.sidebar_foreground.linear_multiply(0.6))
                                        );
                                    });
                                    ui.add_space(4.0);
                                }

                                for sub_item in sub_items {
                                    if let SidebarItem::Link { label, icon, active } = sub_item {
                                        let current_idx = item_idx;
                                        item_idx += 1;

                                        let response = draw_nav_item(
                                            ui, &theme, label, icon.as_deref(), *active, is_collapsed
                                        );
                                        if response.clicked() {
                                            clicked_item = Some(current_idx);
                                        }
                                    }
                                }
                            }
                            SidebarItem::Collapsible { label, icon, items: sub_items } => {
                                // Get/set expanded state from egui temp storage
                                let expanded_id = Id::new(("sidebar_expanded", label.as_str()));
                                let is_expanded = ui.ctx().data(|d| d.get_temp::<bool>(expanded_id).unwrap_or(false));

                                // Draw the collapsible header item with chevron
                                let response = draw_collapsible_item(
                                    ui, &theme, label, icon.as_deref(), is_expanded, is_collapsed
                                );

                                if response.clicked() {
                                    // Toggle expanded state
                                    ui.ctx().data_mut(|d| d.insert_temp(expanded_id, !is_expanded));
                                }

                                // Show sub-items if expanded and not collapsed
                                if is_expanded && !is_collapsed {
                                    for sub_item in sub_items {
                                        if let SidebarItem::Link { label, icon, active } = sub_item {
                                            let current_idx = item_idx;
                                            item_idx += 1;

                                            let response = draw_sub_item(
                                                ui, &theme, label, *active
                                            );
                                            if response.clicked() {
                                                clicked_item = Some(current_idx);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Spacer to push footer to bottom
                    ui.add_space(ui.available_height().max(60.0) - 60.0);

                    // Footer
                    if let Some(footer) = footer {
                        let sep_rect = ui.available_rect_before_wrap();
                        ui.painter().line_segment(
                            [
                                Pos2::new(sep_rect.min.x, sep_rect.min.y),
                                Pos2::new(sep_rect.max.x, sep_rect.min.y),
                            ],
                            egui::Stroke::new(1.0, theme.colors.sidebar_border),
                        );

                        egui::Frame::NONE
                            .inner_margin(egui::Margin::symmetric(12, 12))
                            .show(ui, |ui| {
                                footer(ui);
                            });
                    }
                });
            });

        // Toggle the sidebar open state when toggle button was clicked
        if toggle_clicked {
            *self.open = !*self.open;
        }

        SidebarResponse {
            clicked_item,
            toggle_clicked,
        }
    }
}

fn draw_toggle_button(ui: &mut Ui, theme: &ShadcnTheme, _is_collapsed: bool) -> egui::Response {
    let size = Vec2::splat(32.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();

        if hovered {
            ui.painter().rect_filled(rect, theme.radii.sm, theme.colors.sidebar_accent);
        }

        // Draw panel icon - shadcn style (rectangle with sidebar indicator on left)
        let icon_size = 16.0;
        let icon_rect = egui::Rect::from_center_size(rect.center(), Vec2::splat(icon_size));
        let stroke = egui::Stroke::new(1.5, theme.colors.sidebar_foreground);

        // Outer rectangle (panel) with rounded corners
        ui.painter().rect_stroke(icon_rect, 2.0, stroke, egui::StrokeKind::Inside);

        // Vertical line on left side (sidebar indicator) - slightly inset
        let line_x = icon_rect.min.x + icon_size * 0.33;
        ui.painter().line_segment(
            [
                Pos2::new(line_x, icon_rect.min.y + 3.0),
                Pos2::new(line_x, icon_rect.max.y - 3.0),
            ],
            stroke,
        );
    }

    response
}

fn draw_nav_item(
    ui: &mut Ui,
    theme: &ShadcnTheme,
    label: &str,
    icon: Option<&str>,
    active: bool,
    is_collapsed: bool,
) -> egui::Response {
    let height = 36.0; // Slightly smaller for lighter feel
    let width = ui.available_width() - 16.0;

    ui.horizontal(|ui| {
        ui.add_space(8.0);

        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(width, height),
            Sense::click(),
        );

        if ui.is_rect_visible(rect) {
            let hovered = response.hovered();

            // Background - lighter styling like shadcn
            let bg_color = if active {
                // Active: subtle accent background
                theme.colors.sidebar_accent
            } else if hovered {
                // Hover: visible but not heavy
                theme.colors.foreground.linear_multiply(0.08)
            } else {
                egui::Color32::TRANSPARENT
            };

            ui.painter().rect_filled(rect, theme.radii.sm, bg_color);

            // Subtle left border indicator for active item
            if active {
                let indicator_rect = egui::Rect::from_min_size(
                    rect.min,
                    egui::vec2(3.0, rect.height()),
                );
                ui.painter().rect_filled(indicator_rect, 1.5, theme.colors.primary);
            }

            // Icon
            if let Some(icon) = icon {
                ui.painter().text(
                    Pos2::new(rect.min.x + 12.0, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    icon,
                    egui::FontId::proportional(16.0),
                    if active {
                        theme.colors.sidebar_accent_foreground
                    } else {
                        theme.colors.sidebar_foreground
                    },
                );
            }

            // Label (only when expanded)
            if !is_collapsed {
                let text_x = if icon.is_some() { rect.min.x + 36.0 } else { rect.min.x + 12.0 };
                ui.painter().text(
                    Pos2::new(text_x, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    label,
                    egui::FontId::proportional(theme.typography.small().size),
                    if active {
                        theme.colors.sidebar_accent_foreground
                    } else {
                        theme.colors.sidebar_foreground
                    },
                );
            }
        }

        response
    }).inner
}

/// Draw a navigation item with three dots menu
fn draw_nav_item_with_menu(
    ui: &mut Ui,
    theme: &ShadcnTheme,
    label: &str,
    icon: Option<&str>,
    active: bool,
    is_collapsed: bool,
    menu_items: &[String],
    item_idx: usize,
) -> egui::Response {
    let height = 36.0; // Slightly smaller for lighter feel
    let width = ui.available_width() - 16.0;

    ui.horizontal(|ui| {
        ui.add_space(8.0);

        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(width, height),
            Sense::click(),
        );

        let hovered = response.hovered() || ui.rect_contains_pointer(rect);
        let popup_id = Id::new(("sidebar_item_menu", item_idx));
        let popup_open = ui.memory(|mem| mem.is_popup_open(popup_id));

        if ui.is_rect_visible(rect) {
            // Background - lighter styling like shadcn
            let bg_color = if active {
                theme.colors.sidebar_accent
            } else if hovered || popup_open {
                theme.colors.foreground.linear_multiply(0.08)
            } else {
                egui::Color32::TRANSPARENT
            };

            ui.painter().rect_filled(rect, theme.radii.sm, bg_color);

            // Subtle left border indicator for active item
            if active {
                let indicator_rect = egui::Rect::from_min_size(
                    rect.min,
                    egui::vec2(3.0, rect.height()),
                );
                ui.painter().rect_filled(indicator_rect, 1.5, theme.colors.primary);
            }

            // Icon
            if let Some(icon) = icon {
                ui.painter().text(
                    Pos2::new(rect.min.x + 12.0, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    icon,
                    egui::FontId::proportional(16.0),
                    if active {
                        theme.colors.sidebar_accent_foreground
                    } else {
                        theme.colors.sidebar_foreground
                    },
                );
            }

            // Label (only when expanded)
            if !is_collapsed {
                let text_x = if icon.is_some() { rect.min.x + 36.0 } else { rect.min.x + 12.0 };
                ui.painter().text(
                    Pos2::new(text_x, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    label,
                    egui::FontId::proportional(theme.typography.small().size),
                    if active {
                        theme.colors.sidebar_accent_foreground
                    } else {
                        theme.colors.sidebar_foreground
                    },
                );

                // Three dots on the right (show on hover or when popup open)
                if hovered || popup_open {
                    let dots_rect = egui::Rect::from_min_size(
                        Pos2::new(rect.max.x - 28.0, rect.min.y),
                        Vec2::new(24.0, height),
                    );

                    // Draw three dots vertically
                    let dot_x = dots_rect.center().x;
                    let dot_y = dots_rect.center().y;
                    let dot_spacing = 4.0;
                    let dot_radius = 1.5;
                    let dot_color = theme.colors.sidebar_foreground.linear_multiply(0.7);

                    ui.painter().circle_filled(Pos2::new(dot_x, dot_y - dot_spacing), dot_radius, dot_color);
                    ui.painter().circle_filled(Pos2::new(dot_x, dot_y), dot_radius, dot_color);
                    ui.painter().circle_filled(Pos2::new(dot_x, dot_y + dot_spacing), dot_radius, dot_color);

                    // Make dots clickable
                    let dots_response = ui.interact(dots_rect, popup_id.with("dots"), Sense::click());
                    if dots_response.clicked() {
                        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                    }

                    // Show popup menu
                    egui::popup_below_widget(ui, popup_id, &dots_response, egui::PopupCloseBehavior::CloseOnClickOutside, |ui| {
                        ui.set_min_width(160.0);
                        egui::Frame::NONE
                            .fill(theme.colors.popover)
                            .stroke(egui::Stroke::new(1.0, theme.colors.border))
                            .corner_radius(theme.radii.md)
                            .shadow(theme.shadows.md)
                            .inner_margin(4.0)
                            .show(ui, |ui| {
                                for menu_item in menu_items {
                                    let item_response = ui.allocate_response(
                                        Vec2::new(ui.available_width(), 32.0),
                                        Sense::click(),
                                    );

                                    if ui.is_rect_visible(item_response.rect) {
                                        if item_response.hovered() {
                                            ui.painter().rect_filled(
                                                item_response.rect,
                                                theme.radii.sm,
                                                theme.colors.accent,
                                            );
                                        }

                                        let text_color = if item_response.hovered() {
                                            theme.colors.accent_foreground
                                        } else {
                                            theme.colors.popover_foreground
                                        };

                                        ui.painter().text(
                                            Pos2::new(item_response.rect.min.x + 12.0, item_response.rect.center().y),
                                            egui::Align2::LEFT_CENTER,
                                            menu_item,
                                            egui::FontId::proportional(theme.typography.small().size),
                                            text_color,
                                        );
                                    }

                                    if item_response.clicked() {
                                        ui.memory_mut(|mem| mem.close_popup(popup_id));
                                    }
                                }
                            });
                    });
                }
            }
        }

        response
    }).inner
}

/// Draw a collapsible menu item with chevron
fn draw_collapsible_item(
    ui: &mut Ui,
    theme: &ShadcnTheme,
    label: &str,
    icon: Option<&str>,
    is_expanded: bool,
    is_collapsed: bool,
) -> egui::Response {
    let height = 40.0;
    let width = ui.available_width() - 16.0;

    ui.horizontal(|ui| {
        ui.add_space(8.0);

        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(width, height),
            Sense::click(),
        );

        if ui.is_rect_visible(rect) {
            let hovered = response.hovered();

            // Background on hover
            if hovered {
                ui.painter().rect_filled(rect, theme.radii.md, theme.colors.sidebar_accent.linear_multiply(0.5));
            }

            // Icon
            if let Some(icon) = icon {
                ui.painter().text(
                    Pos2::new(rect.min.x + 12.0, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    icon,
                    egui::FontId::proportional(16.0),
                    theme.colors.sidebar_foreground,
                );
            }

            // Label (only when expanded)
            if !is_collapsed {
                let text_x = if icon.is_some() { rect.min.x + 36.0 } else { rect.min.x + 12.0 };
                ui.painter().text(
                    Pos2::new(text_x, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    label,
                    egui::FontId::proportional(theme.typography.small().size),
                    theme.colors.sidebar_foreground,
                );

                // Chevron on the right - draw a proper chevron shape
                let chevron_x = rect.max.x - 16.0;
                let chevron_y = rect.center().y;
                let chevron_size = 4.0;
                let chevron_color = theme.colors.sidebar_foreground.linear_multiply(0.5);
                let stroke = egui::Stroke::new(1.5, chevron_color);

                if is_expanded {
                    // Down chevron (v shape)
                    ui.painter().line_segment(
                        [Pos2::new(chevron_x - chevron_size, chevron_y - chevron_size * 0.5),
                         Pos2::new(chevron_x, chevron_y + chevron_size * 0.5)],
                        stroke,
                    );
                    ui.painter().line_segment(
                        [Pos2::new(chevron_x, chevron_y + chevron_size * 0.5),
                         Pos2::new(chevron_x + chevron_size, chevron_y - chevron_size * 0.5)],
                        stroke,
                    );
                } else {
                    // Right chevron (> shape)
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
        }

        response
    }).inner
}

/// Draw an indented sub-item (child of collapsible)
fn draw_sub_item(
    ui: &mut Ui,
    theme: &ShadcnTheme,
    label: &str,
    active: bool,
) -> egui::Response {
    let height = 32.0;
    let width = ui.available_width() - 16.0;

    ui.horizontal(|ui| {
        ui.add_space(8.0);

        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(width, height),
            Sense::click(),
        );

        if ui.is_rect_visible(rect) {
            let hovered = response.hovered();

            // Background - lighter styling like shadcn
            let bg_color = if active {
                theme.colors.sidebar_accent
            } else if hovered {
                theme.colors.foreground.linear_multiply(0.08)
            } else {
                egui::Color32::TRANSPARENT
            };

            ui.painter().rect_filled(rect, theme.radii.sm, bg_color);

            // Subtle left border indicator for active sub-item
            if active {
                let indicator_rect = egui::Rect::from_min_size(
                    rect.min,
                    egui::vec2(3.0, rect.height()),
                );
                ui.painter().rect_filled(indicator_rect, 1.5, theme.colors.primary);
            }

            // Indented label (extra left padding for sub-items)
            ui.painter().text(
                Pos2::new(rect.min.x + 44.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                label,
                egui::FontId::proportional(theme.typography.small().size - 1.0),
                if active {
                    theme.colors.sidebar_accent_foreground
                } else {
                    theme.colors.sidebar_foreground
                },
            );
        }

        response
    }).inner
}

/// Builder for sidebar sections
pub struct SidebarBuilder {
    items: Vec<SidebarItem>,
}

impl SidebarBuilder {
    /// Add a navigation item with icon
    pub fn item(&mut self, label: impl Into<String>, icon: impl Into<String>, active: bool) -> &mut Self {
        self.items.push(SidebarItem::Link {
            label: label.into(),
            icon: Some(icon.into()),
            active,
        });
        self
    }

    /// Add a sub-item (no icon, used inside collapsible sections)
    pub fn sub_item(&mut self, label: impl Into<String>, active: bool) -> &mut Self {
        self.items.push(SidebarItem::Link {
            label: label.into(),
            icon: None,
            active,
        });
        self
    }
}

/// Response from showing a sidebar
pub struct SidebarResponse {
    /// Index of clicked navigation item, if any
    pub clicked_item: Option<usize>,
    /// Whether the toggle button was clicked
    pub toggle_clicked: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sidebar_creation() {
        let mut open = true;
        let sidebar = Sidebar::new("test", &mut open)
            .width(280.0)
            .item("Home", "H", true)
            .separator()
            .item("Settings", "S", false);

        assert_eq!(sidebar.width, 280.0);
        assert_eq!(sidebar.items.len(), 3);
    }
}
