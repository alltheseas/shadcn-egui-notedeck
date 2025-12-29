//! Toast notification component ported from shadcn/ui
//!
//! Displays temporary notification messages.
//!
//! Reference: <https://ui.shadcn.com/docs/components/toast>

use egui::{Context, Id, Pos2};
use crate::theme::ShadcnTheme;
use std::time::{Duration, Instant};

/// Toast variant for styling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastVariant {
    /// Default informational toast
    Default,
    /// Success toast
    Success,
    /// Destructive/error toast
    Destructive,
}

/// A single toast notification
#[derive(Debug, Clone)]
pub struct Toast {
    /// Unique ID for this toast
    pub id: u64,
    /// Title text
    pub title: String,
    /// Optional description
    pub description: Option<String>,
    /// Visual variant
    pub variant: ToastVariant,
    /// When toast was created
    pub created_at: Instant,
    /// How long to show the toast
    pub duration: Duration,
    /// Whether the toast has been dismissed
    pub dismissed: bool,
}

impl Toast {
    /// Create a new toast with default settings
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: rand_id(),
            title: title.into(),
            description: None,
            variant: ToastVariant::Default,
            created_at: Instant::now(),
            duration: Duration::from_secs(5),
            dismissed: false,
        }
    }

    /// Set the description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the variant
    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set duration before auto-dismiss
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Check if the toast should be removed
    pub fn is_expired(&self) -> bool {
        self.dismissed || self.created_at.elapsed() >= self.duration
    }
}

/// Simple pseudo-random ID generator
fn rand_id() -> u64 {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    now.as_nanos() as u64
}

/// Toast manager for handling multiple toasts
///
/// Store this in your app state and call `show()` each frame.
///
/// ## Example
/// ```rust,ignore
/// struct MyApp {
///     toasts: Toaster,
/// }
///
/// impl MyApp {
///     fn update(&mut self, ctx: &egui::Context) {
///         // Add a toast
///         if some_action_completed {
///             self.toasts.add(Toast::new("Action completed!"));
///         }
///
///         // Render toasts
///         self.toasts.show(ctx);
///     }
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct Toaster {
    toasts: Vec<Toast>,
}

impl Toaster {
    /// Create a new toaster
    pub fn new() -> Self {
        Self { toasts: Vec::new() }
    }

    /// Add a toast notification
    pub fn add(&mut self, toast: Toast) {
        self.toasts.push(toast);
    }

    /// Add a simple success toast
    pub fn success(&mut self, title: impl Into<String>) {
        self.add(Toast::new(title).variant(ToastVariant::Success));
    }

    /// Add a simple error toast
    pub fn error(&mut self, title: impl Into<String>) {
        self.add(Toast::new(title).variant(ToastVariant::Destructive));
    }

    /// Add a simple info toast
    pub fn info(&mut self, title: impl Into<String>) {
        self.add(Toast::new(title).variant(ToastVariant::Default));
    }

    /// Dismiss a toast by ID
    pub fn dismiss(&mut self, id: u64) {
        if let Some(toast) = self.toasts.iter_mut().find(|t| t.id == id) {
            toast.dismissed = true;
        }
    }

    /// Show all active toasts
    ///
    /// Call this once per frame in your update function.
    pub fn show(&mut self, ctx: &Context) {
        // Remove expired toasts
        self.toasts.retain(|t| !t.is_expired());

        if self.toasts.is_empty() {
            return;
        }

        let theme = ctx.data(|d| {
            d.get_temp::<ShadcnTheme>(Id::new("shadcn_theme"))
                .unwrap_or_else(ShadcnTheme::light)
        });

        let screen_rect = ctx.input(|i| i.raw.screen_rect.unwrap_or(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0))));
        let toast_width = 360.0;
        let toast_spacing = 8.0;
        let margin = 16.0;

        // Position toasts in bottom-right corner
        let base_x = screen_rect.max.x - toast_width - margin;
        let mut current_y = screen_rect.max.y - margin;

        // Collect IDs to dismiss (can't mutate while iterating)
        let mut to_dismiss = Vec::new();

        for toast in self.toasts.iter().rev() {
            let toast_id = Id::new("toast").with(toast.id);

            // Calculate toast height based on content
            let has_description = toast.description.is_some();
            let toast_height = if has_description { 80.0 } else { 56.0 };

            current_y -= toast_height + toast_spacing;

            let toast_pos = Pos2::new(base_x, current_y);

            egui::Area::new(toast_id)
                .order(egui::Order::Foreground)
                .fixed_pos(toast_pos)
                .show(ctx, |ui| {
                    // Colors based on variant
                    let (bg_color, border_color, title_color, desc_color) = match toast.variant {
                        ToastVariant::Default => (
                            theme.colors.background,
                            theme.colors.border,
                            theme.colors.foreground,
                            theme.colors.muted_foreground,
                        ),
                        ToastVariant::Success => (
                            theme.colors.background,
                            egui::Color32::from_rgb(34, 197, 94), // Green
                            theme.colors.foreground,
                            theme.colors.muted_foreground,
                        ),
                        ToastVariant::Destructive => (
                            theme.colors.destructive,
                            theme.colors.destructive,
                            theme.colors.destructive_foreground,
                            theme.colors.destructive_foreground.linear_multiply(0.85),
                        ),
                    };

                    let frame = egui::Frame::NONE
                        .fill(bg_color)
                        .stroke(egui::Stroke::new(1.0, border_color))
                        .corner_radius(theme.radii.lg)
                        .shadow(theme.shadows.lg)
                        .inner_margin(egui::Margin::symmetric(16, 12));

                    frame.show(ui, |ui| {
                        ui.set_min_width(toast_width - 32.0);
                        ui.set_max_width(toast_width - 32.0);

                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                // Title
                                ui.label(
                                    egui::RichText::new(&toast.title)
                                        .size(theme.typography.body().size)
                                        .strong()
                                        .color(title_color),
                                );

                                // Description
                                if let Some(ref desc) = toast.description {
                                    ui.label(
                                        egui::RichText::new(desc)
                                            .size(theme.typography.small().size)
                                            .color(desc_color),
                                    );
                                }
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                // Close button with 44px touch target
                                let close_btn = ui.add_sized(
                                    egui::vec2(32.0, 32.0),
                                    egui::Button::new(
                                        egui::RichText::new("X")
                                            .size(12.0)
                                            .color(title_color.linear_multiply(0.7)),
                                    )
                                    .fill(egui::Color32::TRANSPARENT)
                                    .stroke(egui::Stroke::NONE),
                                );

                                if close_btn.clicked() {
                                    to_dismiss.push(toast.id);
                                }
                            });
                        });

                        // Progress bar showing time remaining
                        let elapsed = toast.created_at.elapsed();
                        let progress = 1.0 - (elapsed.as_secs_f32() / toast.duration.as_secs_f32()).min(1.0);

                        if progress > 0.0 {
                            let bar_rect = egui::Rect::from_min_size(
                                egui::pos2(ui.min_rect().min.x, ui.min_rect().max.y + 4.0),
                                egui::vec2((toast_width - 32.0) * progress, 2.0),
                            );
                            ui.painter().rect_filled(
                                bar_rect,
                                1.0,
                                border_color.linear_multiply(0.5),
                            );
                        }
                    });
                });

            // Request repaint to animate progress bar
            ctx.request_repaint();
        }

        // Dismiss clicked toasts
        for id in to_dismiss {
            self.dismiss(id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toast_creation() {
        let toast = Toast::new("Test")
            .description("Description")
            .variant(ToastVariant::Success);

        assert_eq!(toast.title, "Test");
        assert!(toast.description.is_some());
        assert_eq!(toast.variant, ToastVariant::Success);
    }

    #[test]
    fn test_toaster() {
        let mut toaster = Toaster::new();
        toaster.add(Toast::new("Test 1"));
        toaster.add(Toast::new("Test 2"));

        assert_eq!(toaster.toasts.len(), 2);
    }
}
