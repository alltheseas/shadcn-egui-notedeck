//! Notedeck-specific helpers and presets
//!
//! This module provides utilities specifically for notedeck apps to make
//! integration seamless and ensure design consistency across all apps.

use crate::theme::ShadcnTheme;
use egui::Context;

/// Notedeck theme preset
///
/// A pre-configured theme that matches notedeck's design language.
/// Use this for instant design consistency.
pub struct NotedeckTheme;

impl NotedeckTheme {
    /// Apply notedeck light theme to the context
    ///
    /// Call this once per frame in your app's update() method.
    ///
    /// ## Example
    /// ```ignore
    /// impl eframe::App for MyApp {
    ///     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    ///         NotedeckTheme::apply_light(ctx);
    ///         // Your UI code here
    ///     }
    /// }
    /// ```
    pub fn apply_light(ctx: &Context) {
        ShadcnTheme::light().apply(ctx);
    }

    /// Apply notedeck dark theme to the context
    pub fn apply_dark(ctx: &Context) {
        ShadcnTheme::dark().apply(ctx);
    }

    /// Apply theme based on a boolean flag
    ///
    /// ## Example
    /// ```ignore
    /// NotedeckTheme::apply(ctx, self.settings.dark_mode);
    /// ```
    pub fn apply(ctx: &Context, dark_mode: bool) {
        if dark_mode {
            Self::apply_dark(ctx);
        } else {
            Self::apply_light(ctx);
        }
    }

    /// Get the light theme without applying it
    ///
    /// Useful if you want to access theme tokens without applying to context.
    pub fn light() -> ShadcnTheme {
        ShadcnTheme::light()
    }

    /// Get the dark theme without applying it
    pub fn dark() -> ShadcnTheme {
        ShadcnTheme::dark()
    }

    /// Get theme based on dark_mode flag without applying it
    pub fn get(dark_mode: bool) -> ShadcnTheme {
        if dark_mode {
            Self::dark()
        } else {
            Self::light()
        }
    }
}

/// Helper trait for egui Context to make theme application even easier
pub trait NotedeckContextExt {
    /// Apply notedeck theme in one call
    ///
    /// ## Example
    /// ```ignore
    /// ctx.apply_notedeck_theme(self.dark_mode);
    /// ```
    fn apply_notedeck_theme(&self, dark_mode: bool);
}

impl NotedeckContextExt for Context {
    fn apply_notedeck_theme(&self, dark_mode: bool) {
        NotedeckTheme::apply(self, dark_mode);
    }
}

/// Common notedeck UI patterns
pub mod patterns {
    use crate::*;
    use egui::Ui;

    /// Standard notedeck header with title and optional badge
    ///
    /// ## Example
    /// ```ignore
    /// patterns::header(ui, "Timeline", Some("Live"));
    /// ```
    pub fn header(ui: &mut Ui, title: &str, badge: Option<&str>) {
        ui.horizontal(|ui| {
            let theme = crate::ShadcnTheme::light();
            ui.label(
                egui::RichText::new(title)
                    .size(theme.typography.h3().size)
                    .color(theme.colors.foreground)
            );

            if let Some(badge_text) = badge {
                ui.add(Badge::new(badge_text).variant(BadgeVariant::Secondary));
            }
        });
    }

    /// Standard notedeck settings row (label + control)
    ///
    /// ## Example
    /// ```ignore
    /// patterns::setting_row(ui, "Dark Mode", |ui| {
    ///     ui.checkbox(&mut self.dark_mode, "");
    /// });
    /// ```
    pub fn setting_row(ui: &mut Ui, label: &str, add_control: impl FnOnce(&mut Ui)) {
        ui.horizontal(|ui| {
            ui.label(label);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                add_control(ui);
            });
        });
    }

    /// Standard notedeck form field (label + input + optional helper)
    ///
    /// ## Example
    /// ```ignore
    /// patterns::form_field(
    ///     ui,
    ///     "Email",
    ///     &mut self.email,
    ///     "you@example.com",
    ///     Some("We'll never share your email")
    /// );
    /// ```
    pub fn form_field(
        ui: &mut Ui,
        label: &str,
        text: &mut String,
        placeholder: &str,
        helper: Option<&str>,
    ) {
        let theme = crate::ShadcnTheme::light();

        form_label(ui, label);
        shadcn_input(ui, text, placeholder);

        if let Some(helper_text) = helper {
            ui.add_space(theme.spacing.xs);
            form_helper(ui, helper_text);
        }

        ui.add_space(theme.spacing.sm);
    }

    /// Standard notedeck error message
    pub fn error_message(ui: &mut Ui, message: &str) {
        Alert::new(ui, AlertVariant::Destructive)
            .title("Error")
            .description(message)
            .show();
    }

    /// Standard notedeck success message
    pub fn success_message(ui: &mut Ui, message: &str) {
        Alert::new(ui, AlertVariant::Default)
            .title("Success")
            .description(message)
            .show();
    }

    /// Standard notedeck user profile card
    ///
    /// ## Example
    /// ```ignore
    /// patterns::user_card(ui, "Alice", "alice@nostr.com", Some("Active"));
    /// ```
    pub fn user_card(ui: &mut Ui, name: &str, handle: &str, status: Option<&str>) {
        Card::new(ui)
            .header(|ui| {
                ui.horizontal(|ui| {
                    ui.add(Avatar::new(name));
                    ui.vertical(|ui| {
                        card_title(ui, name);
                        card_description(ui, handle);
                    });
                });
            })
            .content(|ui| {
                if let Some(status_text) = status {
                    ui.horizontal(|ui| {
                        ui.label("Status:");
                        ui.add(Badge::new(status_text).variant(BadgeVariant::Secondary));
                    });
                }
            })
            .show();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notedeck_theme_get() {
        let light = NotedeckTheme::get(false);
        let dark = NotedeckTheme::get(true);

        // Verify they're different
        assert_ne!(light.colors.background, dark.colors.background);
    }
}
