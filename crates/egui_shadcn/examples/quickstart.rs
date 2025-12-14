//! Quickstart example for notedeck app developers
//!
//! This is a minimal example showing how to integrate egui_shadcn
//! into a new notedeck app with just a few lines of code.
//!
//! Run with: cargo run -p egui_shadcn --example quickstart

use egui_shadcn::{*, notedeck::patterns};

/// Minimal notedeck app with shadcn integration
struct QuickstartApp {
    dark_mode: bool,
    username: String,
    email: String,
    message: String,
}

impl Default for QuickstartApp {
    fn default() -> Self {
        Self {
            dark_mode: false,
            username: String::new(),
            email: String::new(),
            message: String::new(),
        }
    }
}

impl QuickstartApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for QuickstartApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // STEP 1: Apply notedeck theme (one line!)
        NotedeckTheme::apply(ctx, self.dark_mode);

        // STEP 2: Build your UI with shadcn components
        egui::CentralPanel::default().show(ctx, |ui| {
            // Add scroll area for all content
            egui::ScrollArea::vertical().show(ui, |ui| {
            // Get theme for spacing
            let theme = NotedeckTheme::get(self.dark_mode);

            // Header with notedeck pattern
            patterns::header(ui, "My Notedeck App", Some("v1.0"));

            ui.add_space(theme.spacing.lg);

            // Theme toggle
            ui.horizontal(|ui| {
                ui.label("Theme:");
                if ui.button(if self.dark_mode { "🌙 Dark" } else { "☀ Light" }).clicked() {
                    self.dark_mode = !self.dark_mode;
                }
            });

            ui.add_space(theme.spacing.lg);

            // Use pre-built patterns for consistency
            patterns::success_message(ui, "Successfully connected to relay!");

            ui.add_space(theme.spacing.md);

            // Form using patterns
            Card::new(ui)
                .header(|ui| {
                    card_title(ui, "User Profile");
                    card_description(ui, "Update your information");
                })
                .content(|ui| {
                    // Easy form fields with the pattern helper
                    patterns::form_field(
                        ui,
                        "Username",
                        &mut self.username,
                        "Enter username...",
                        Some("This will be visible to others")
                    );

                    patterns::form_field(
                        ui,
                        "Email",
                        &mut self.email,
                        "you@example.com",
                        None
                    );

                    form_label(ui, "Bio");
                    shadcn_textarea(ui, &mut self.message, "Tell us about yourself...");
                })
                .footer(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            // Save logic here
                        }
                        if ui.button("Cancel").clicked() {
                            // Cancel logic here
                        }
                    });
                })
                .show();

            ui.add_space(theme.spacing.lg);

            // Use individual components
            ui.label("Status Badges:");
            ui.horizontal(|ui| {
                ui.add(Badge::new("Online").variant(BadgeVariant::Secondary));
                ui.add(Badge::new("Premium").variant(BadgeVariant::Default));
                ui.add(Badge::new("New").variant(BadgeVariant::Outline));
            });

            ui.add_space(theme.spacing.lg);

            // Use user card pattern
            patterns::user_card(
                ui,
                "Alice Johnson",
                "alice@nostr.com",
                Some("Active")
            );

            ui.add_space(theme.spacing.lg);

            // Tabs example
            Tabs::new(ui, "quickstart-tabs")
                .tab("overview", "Overview", |ui| {
                    ui.label("This is the overview tab");
                    patterns::success_message(ui, "Everything is working!");
                })
                .tab("settings", "Settings", |ui| {
                    ui.label("Settings configuration");
                    patterns::setting_row(ui, "Notifications", |ui| {
                        let mut enabled = true;
                        ui.checkbox(&mut enabled, "");
                    });
                    patterns::setting_row(ui, "Dark Mode", |ui| {
                        ui.checkbox(&mut self.dark_mode, "");
                    });
                })
                .tab("about", "About", |ui| {
                    ui.label("About this app");
                    ui.label("Built with egui_shadcn for notedeck");
                })
                .show();
            });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("Notedeck Quickstart - egui_shadcn"),
        ..Default::default()
    };

    eframe::run_native(
        "Notedeck Quickstart",
        options,
        Box::new(|cc| Ok(Box::new(QuickstartApp::new(cc)))),
    )
}
