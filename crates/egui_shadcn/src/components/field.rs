//! Field wrapper component ported from shadcn/ui
//!
//! A wrapper that provides label, description, and error message for form inputs.
//!
//! Reference: <https://ui.shadcn.com/docs/components/form>

use egui::Ui;
use crate::theme::ShadcnTheme;

/// Field wrapper for form inputs
///
/// Provides consistent styling for:
/// - Label (above input)
/// - Description (below label, optional)
/// - Error message (below input, optional)
///
/// ## Example
/// ```rust,ignore
/// let mut email = String::new();
/// let email_error = if email.is_empty() { Some("Email is required") } else { None };
///
/// Field::new("email")
///     .label("Email")
///     .description("We'll never share your email.")
///     .error(email_error)
///     .show(ui, |ui| {
///         shadcn_input(ui, &mut email, "Enter your email...");
///     });
/// ```
pub struct Field<'a> {
    id: &'a str,
    label: Option<String>,
    description: Option<String>,
    error: Option<String>,
    required: bool,
}

impl<'a> Field<'a> {
    /// Create a new field wrapper
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            label: None,
            description: None,
            error: None,
            required: false,
        }
    }

    /// Set the field label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the description text (shown below label)
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the error message (shown below input)
    pub fn error(mut self, error: Option<impl Into<String>>) -> Self {
        self.error = error.map(|e| e.into());
        self
    }

    /// Mark the field as required (shows asterisk)
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Show the field with content
    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> FieldResponse<R> {
        let theme = ui.ctx().data(|d| {
            d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let has_error = self.error.is_some();

        ui.vertical(|ui| {
            // Label
            if let Some(ref label) = self.label {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(label)
                            .size(theme.typography.small().size)
                            .color(if has_error {
                                theme.colors.destructive
                            } else {
                                theme.colors.foreground
                            })
                    );

                    if self.required {
                        ui.label(
                            egui::RichText::new("*")
                                .size(theme.typography.small().size)
                                .color(theme.colors.destructive)
                        );
                    }
                });
            }

            // Description
            if let Some(ref desc) = self.description {
                ui.add_space(2.0);
                ui.label(
                    egui::RichText::new(desc)
                        .size(theme.typography.small().size - 2.0)
                        .color(theme.colors.muted_foreground)
                );
                ui.add_space(8.0);
            } else if self.label.is_some() {
                ui.add_space(6.0);
            }

            // Content (the actual input)
            let inner = content(ui);

            // Error message
            if let Some(ref error) = self.error {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    // Error icon placeholder
                    ui.label(
                        egui::RichText::new("!")
                            .size(theme.typography.small().size - 2.0)
                            .color(theme.colors.destructive)
                    );
                    ui.label(
                        egui::RichText::new(error)
                            .size(theme.typography.small().size - 2.0)
                            .color(theme.colors.destructive)
                    );
                });
            }

            FieldResponse {
                inner,
                has_error,
            }
        }).inner
    }
}

/// Response from showing a field
pub struct FieldResponse<R> {
    /// The return value from the content closure
    pub inner: R,
    /// Whether the field has an error
    pub has_error: bool,
}

/// Convenience function for a simple labeled input
pub fn labeled_input(ui: &mut Ui, label: &str, value: &mut String, placeholder: &str) {
    Field::new(label)
        .label(label)
        .show(ui, |ui| {
            crate::components::input::shadcn_input(ui, value, placeholder);
        });
}

/// Convenience function for a required labeled input with error
pub fn required_input(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    placeholder: &str,
    error: Option<&str>,
) {
    Field::new(label)
        .label(label)
        .required(true)
        .error(error)
        .show(ui, |ui| {
            crate::components::input::shadcn_input(ui, value, placeholder);
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_creation() {
        let field = Field::new("test")
            .label("Test Label")
            .description("Test description")
            .required(true)
            .error(Some("Test error"));

        assert_eq!(field.label, Some("Test Label".to_string()));
        assert!(field.required);
        assert!(field.error.is_some());
    }
}
