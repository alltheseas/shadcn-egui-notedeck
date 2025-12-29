//! Showcase example for egui_shadcn
//!
//! This example demonstrates the shadcn theme and components.
//! Run with: cargo run -p egui_shadcn --example showcase

use egui_shadcn::{
    ShadcnTheme,
    Badge, BadgeVariant,
    Avatar, AvatarSize,
    Card, card_title, card_description,
    Alert, AlertVariant,
    Skeleton,
    Spinner, SpinnerSize,
    Kbd,
    Button, ButtonVariant, ButtonSize,
    shadcn_input, shadcn_input_with_error, shadcn_textarea, form_label, form_helper,
    Checkbox,
    Switch,
    Slider,
    Progress,
    Separator,
    Tabs,
    Toggle, ToggleVariant, ToggleSize,
    RadioGroup,
    Select,
    DropdownMenu,
    Combobox, ComboboxOption,
    Toast, ToastVariant, Toaster,
    Dialog, confirm_dialog, ConfirmResult,
    TooltipExt,
    Popover,
    Collapsible,
    Accordion, AccordionType,
    Breadcrumb, BreadcrumbSeparator,
    ToggleGroup, ToggleGroupType,
    HoverCardExt,
    Sheet, SheetSide,
    Drawer, DrawerSide,
    // Phase 6: Data Display & Advanced
    AlertDialog, AlertDialogResult,
    ContextMenu,
    Pagination,
    AspectRatio, AspectRatioPreset,
    simple_table,
    Command,
    Calendar, CalendarSelection,
    DatePicker,
    Carousel,
    Chart, ChartType, DataPoint,
    ResizablePanelGroup, ResizableDirection,
    // Phase 7: Navigation & Forms
    Menubar,
    NavigationMenu,
    Sidebar,
    Field,
};

/// App state for the showcase
struct ShowcaseApp {
    dark_mode: bool,
    email: String,
    message: String,
    checkbox_checked: bool,
    switch_enabled: bool,
    slider_value: f32,
    progress_value: f32,
    // Toggle state
    toggle_bold: bool,
    toggle_italic: bool,
    toggle_underline: bool,
    // Toggle Group state
    toggle_group_selected: Vec<String>,
    // Dialog state
    dialog_open: bool,
    confirm_dialog_open: bool,
    confirm_result: Option<ConfirmResult>,
    // Radio state
    radio_selected: usize,
    // Select state
    select_fruit: usize,
    // Combobox state
    combobox_framework: Option<String>,
    // Toaster
    toaster: Toaster,
    // Popover state
    popover_width: f32,
    // Collapsible state
    collapsible_open: bool,
    // Sheet state
    sheet_open: bool,
    // Drawer state
    drawer_open: bool,
    // Phase 6: Data Display & Advanced
    alert_dialog_open: bool,
    alert_dialog_result: Option<AlertDialogResult>,
    current_page: usize,
    command_open: bool,
    command_search: String,
    calendar_selection: CalendarSelection,
    calendar_view_date: chrono::NaiveDate,
    datepicker_date: Option<chrono::NaiveDate>,
    carousel_index: usize,
    resizable_split: f32,
    resizable_vertical_split: f32,
    // Phase 7: Navigation & Forms
    field_username: String,
    field_email: String,
    field_email_error: Option<String>,
    sidebar_open: bool,
    sidebar_selected: usize,
    account_menu_open: bool,
}

impl Default for ShowcaseApp {
    fn default() -> Self {
        Self {
            dark_mode: false,
            email: String::new(),
            message: String::new(),
            checkbox_checked: false,
            switch_enabled: false,
            slider_value: 50.0,
            progress_value: 0.65,
            toggle_bold: false,
            toggle_italic: false,
            toggle_underline: false,
            toggle_group_selected: vec!["center".to_string()],
            dialog_open: false,
            confirm_dialog_open: false,
            confirm_result: None,
            radio_selected: 0,
            select_fruit: 0,
            combobox_framework: None,
            toaster: Toaster::new(),
            popover_width: 200.0,
            collapsible_open: false,
            sheet_open: false,
            drawer_open: false,
            // Phase 6
            alert_dialog_open: false,
            alert_dialog_result: None,
            current_page: 1,
            command_open: false,
            command_search: String::new(),
            calendar_selection: CalendarSelection::None,
            calendar_view_date: chrono::Local::now().date_naive(),
            datepicker_date: None,
            carousel_index: 0,
            resizable_split: 0.5,
            resizable_vertical_split: 0.6,
            // Phase 7
            field_username: String::new(),
            field_email: String::new(),
            field_email_error: None,
            sidebar_open: true,
            sidebar_selected: 0,
            account_menu_open: false,
        }
    }
}

impl ShowcaseApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for ShowcaseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply the appropriate theme based on dark_mode toggle
        let theme = if self.dark_mode {
            ShadcnTheme::dark()
        } else {
            ShadcnTheme::light()
        };
        theme.apply(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {
            ui.heading("egui_shadcn Showcase");

            ui.horizontal(|ui| {
                ui.label("Theme:");
                if ui.button(if self.dark_mode { "🌙 Dark" } else { "☀ Light" }).clicked() {
                    self.dark_mode = !self.dark_mode;
                }
            });

            ui.separator();

            ui.label("This demonstrates the shadcn/ui design system ported to egui.");

            ui.add_space(16.0);

            ui.group(|ui| {
                ui.label("✅ Phase 1: Design System");
                ui.label("  ✓ Color system (light & dark modes)");
                ui.label("  ✓ Spacing system");
                ui.label("  ✓ Typography scale");
                ui.label("  ✓ Corner radii");
                ui.label("  ✓ Shadow/elevation");
            });

            ui.add_space(8.0);

            ui.group(|ui| {
                ui.label("✅ Phase 2: Core Components");
                ui.label("  ✓ Button (all variants & sizes)");
                ui.label("  ✓ Badge");
                ui.label("  ✓ Avatar");
                ui.label("  ✓ Card");
                ui.label("  ✓ Alert");
                ui.label("  ✓ Skeleton");
                ui.label("  ✓ Kbd");
            });

            ui.add_space(16.0);

            ui.heading("Phase 1: Design System Demo");

            ui.label("Spacing Scale:");
            ui.horizontal(|ui| {
                // Visual demonstration of spacing sizes with colored boxes
                let spacing_demo = |ui: &mut egui::Ui, label: &str, size: f32, color: egui::Color32| {
                    ui.vertical(|ui| {
                        ui.label(format!("{}: {}", label, size));
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(size, size),
                            egui::Sense::hover(),
                        );
                        ui.painter().rect_filled(rect, 2.0, color);
                    });
                };

                spacing_demo(ui, "xs", theme.spacing.xs, theme.colors.primary.linear_multiply(0.3));
                ui.add_space(4.0);
                spacing_demo(ui, "sm", theme.spacing.sm, theme.colors.primary.linear_multiply(0.5));
                ui.add_space(4.0);
                spacing_demo(ui, "md", theme.spacing.md, theme.colors.primary.linear_multiply(0.7));
                ui.add_space(4.0);
                spacing_demo(ui, "lg", theme.spacing.lg, theme.colors.primary.linear_multiply(0.85));
                ui.add_space(4.0);
                spacing_demo(ui, "xl", theme.spacing.xl, theme.colors.primary);
            });

            ui.add_space(8.0);

            ui.label("Typography Scale:");
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Small Text (14px)")
                    .size(theme.typography.small().size));
                ui.label(egui::RichText::new("Body Text (16px)")
                    .size(theme.typography.body().size));
                ui.label(egui::RichText::new("Large Text (18px)")
                    .size(theme.typography.large().size));
                ui.label(egui::RichText::new("Lead Text (20px)")
                    .size(theme.typography.lead().size));
                ui.label(egui::RichText::new("Heading 4 (24px)")
                    .size(theme.typography.h4().size));
                ui.label(egui::RichText::new("Heading 3 (30px)")
                    .size(theme.typography.h3().size));
                ui.label(egui::RichText::new("Heading 2 (36px)")
                    .size(theme.typography.h2().size));
            });

            ui.add_space(8.0);

            ui.label("Corner Radii:");
            ui.horizontal(|ui| {
                for (label, radius) in [
                    ("sm", theme.radii.sm),
                    ("md", theme.radii.md),
                    ("lg", theme.radii.lg),
                    ("xl", theme.radii.xl),
                ] {
                    egui::Frame::NONE
                        .fill(theme.colors.primary)
                        .inner_margin(egui::Margin::same(8))
                        .corner_radius(radius)
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new(label).color(theme.colors.primary_foreground));
                        });
                }
            });

            ui.add_space(8.0);

            ui.label("Shadow/Elevation:");
            ui.horizontal(|ui| {
                for (label, shadow) in [
                    ("sm", theme.shadows.sm),
                    ("md", theme.shadows.md),
                    ("lg", theme.shadows.lg),
                    ("xl", theme.shadows.xl),
                ] {
                    egui::Frame::NONE
                        .fill(theme.colors.card)
                        .inner_margin(egui::Margin::same(12))
                        .shadow(shadow)
                        .show(ui, |ui| {
                            ui.label(label);
                        });
                }
            });

            ui.add_space(16.0);

            ui.heading("Color System Demo");
            ui.label("All shadcn semantic color tokens:");
            ui.add_space(8.0);

            // Color swatches
            ui.horizontal_wrapped(|ui| {
                // Primary
                egui::Frame::NONE
                    .fill(theme.colors.primary)
                    .inner_margin(egui::Margin::same(12))
                    .corner_radius(theme.radii.md)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Primary").color(theme.colors.primary_foreground));
                    });

                // Secondary
                egui::Frame::NONE
                    .fill(theme.colors.secondary)
                    .inner_margin(egui::Margin::same(12))
                    .corner_radius(theme.radii.md)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Secondary").color(theme.colors.secondary_foreground));
                    });

                // Destructive
                egui::Frame::NONE
                    .fill(theme.colors.destructive)
                    .inner_margin(egui::Margin::same(12))
                    .corner_radius(theme.radii.md)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Destructive").color(theme.colors.destructive_foreground));
                    });

                // Muted
                egui::Frame::NONE
                    .fill(theme.colors.muted)
                    .inner_margin(egui::Margin::same(12))
                    .corner_radius(theme.radii.md)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Muted").color(theme.colors.muted_foreground));
                    });

                // Accent
                egui::Frame::NONE
                    .fill(theme.colors.accent)
                    .inner_margin(egui::Margin::same(12))
                    .corner_radius(theme.radii.md)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Accent").color(theme.colors.accent_foreground));
                    });

                // Card
                egui::Frame::NONE
                    .fill(theme.colors.card)
                    .stroke(egui::Stroke::new(1.0, theme.colors.border))
                    .inner_margin(egui::Margin::same(12))
                    .corner_radius(theme.radii.md)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Card").color(theme.colors.card_foreground));
                    });

                // Popover
                egui::Frame::NONE
                    .fill(theme.colors.popover)
                    .stroke(egui::Stroke::new(1.0, theme.colors.border))
                    .inner_margin(egui::Margin::same(12))
                    .corner_radius(theme.radii.md)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Popover").color(theme.colors.popover_foreground));
                    });
            });

            ui.add_space(8.0);

            ui.label("Interactive elements:");
            ui.horizontal(|ui| {
                ui.label("Primary button:");
                ui.add(Button::new("Click me"));
            });

            ui.horizontal(|ui| {
                ui.label("Secondary (checkbox):");
                ui.add(Checkbox::new(&mut self.checkbox_checked).label("Check me"));
            });

            ui.horizontal(|ui| {
                ui.label("Text input:");
                ui.set_max_width(200.0); // Constrain width on desktop
                shadcn_input(ui, &mut self.email, "Type here");
            });

            ui.horizontal(|ui| {
                ui.label("Hyperlink:");
                ui.hyperlink_to("shadcn/ui", "https://ui.shadcn.com");
            });

            ui.add_space(16.0);

            ui.heading("Phase 2: Core Components Demo");
            ui.add_space(8.0);

            ui.label("Badges:");
            ui.horizontal(|ui| {
                ui.add(Badge::new("Default"));
                ui.add(Badge::new("Secondary").variant(BadgeVariant::Secondary));
                ui.add(Badge::new("Destructive").variant(BadgeVariant::Destructive));
                ui.add(Badge::new("Outline").variant(BadgeVariant::Outline));
            });

            ui.horizontal(|ui| {
                ui.label("Notification count:");
                ui.add(Badge::new("99+").variant(BadgeVariant::Destructive));
            });

            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.add(Badge::new("Beta").variant(BadgeVariant::Secondary));
                ui.add(Badge::new("New").variant(BadgeVariant::Default));
            });

            ui.add_space(16.0);

            ui.label("Buttons:");
            ui.label("All shadcn button variants:");
            ui.horizontal(|ui| {
                if ui.add(Button::new("Default").variant(ButtonVariant::Default)).clicked() {
                    // Handle click
                }
                if ui.add(Button::new("Secondary").variant(ButtonVariant::Secondary)).clicked() {
                    // Handle click
                }
                if ui.add(Button::new("Outline").variant(ButtonVariant::Outline)).clicked() {
                    // Handle click
                }
                if ui.add(Button::new("Ghost").variant(ButtonVariant::Ghost)).clicked() {
                    // Handle click
                }
                if ui.add(Button::new("Destructive").variant(ButtonVariant::Destructive)).clicked() {
                    // Handle click
                }
                if ui.add(Button::new("Link").variant(ButtonVariant::Link)).clicked() {
                    // Handle click
                }
            });

            ui.add_space(8.0);
            ui.label("Button sizes:");
            ui.horizontal(|ui| {
                ui.add(Button::new("Small").size(ButtonSize::Small));
                ui.add(Button::new("Default").size(ButtonSize::Default));
                ui.add(Button::new("Large").size(ButtonSize::Large));
                ui.add(Button::new("🔔").size(ButtonSize::Icon));
            });

            ui.add_space(8.0);
            ui.label("Disabled state:");
            ui.horizontal(|ui| {
                ui.add(Button::new("Enabled").variant(ButtonVariant::Default));
                ui.add(Button::new("Disabled").variant(ButtonVariant::Default).enabled(false));
            });

            ui.add_space(16.0);

            ui.label("Avatars:");
            ui.horizontal(|ui| {
                ui.add(Avatar::new("John Doe"));
                ui.add(Avatar::new("Alice Smith"));
                ui.add(Avatar::new("Bob"));
                ui.add(Avatar::new("CD"));
            });

            ui.horizontal(|ui| {
                ui.label("Sizes:");
                ui.add(Avatar::new("S").size(AvatarSize::Small));
                ui.add(Avatar::new("M").size(AvatarSize::Medium));
                ui.add(Avatar::new("L").size(AvatarSize::Large));
                ui.add(Avatar::new("XL").size(AvatarSize::ExtraLarge));
            });

            ui.add_space(16.0);

            ui.label("Cards:");

            // Constrain card width for better aesthetics on wide screens
            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_max_width(600.0);

                // Simple card
                Card::new(ui)
                    .header(|ui| {
                        card_title(ui, "Simple Card");
                        card_description(ui, "This is a basic card example");
                    })
                    .content(|ui| {
                        ui.label("Card content goes here.");
                        ui.label("You can add any widgets.");
                    })
                    .show();

                ui.add_space(8.0);

                // Card with footer
                Card::new(ui)
                    .header(|ui| {
                        card_title(ui, "User Profile");
                        card_description(ui, "Manage your account");
                    })
                    .content(|ui| {
                        ui.label("Name: John Doe");
                        ui.label("Email: john@example.com");
                        ui.horizontal(|ui| {
                            ui.label("Status:");
                            ui.add(Badge::new("Active").variant(BadgeVariant::Secondary));
                        });
                    })
                    .footer(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(Button::new("Save"));
                            ui.add(Button::new("Cancel").variant(ButtonVariant::Outline));
                        });
                    })
                    .show();
            });

            ui.add_space(16.0);

            ui.label("Alerts:");
            Alert::new(ui, AlertVariant::Default)
                .title("Informational")
                .description("This is a default alert with helpful information.")
                .show();

            ui.add_space(8.0);

            Alert::new(ui, AlertVariant::Destructive)
                .title("Error")
                .description("Something went wrong. Please try again.")
                .show();

            ui.add_space(16.0);

            ui.label("Skeleton (Loading States):");
            ui.horizontal(|ui| {
                ui.add(Skeleton::circle(40.0));
                ui.vertical(|ui| {
                    ui.add(Skeleton::new(egui::Vec2::new(200.0, 16.0)));
                    ui.add(Skeleton::new(egui::Vec2::new(150.0, 16.0)));
                });
            });

            ui.add_space(16.0);

            ui.label("Spinner (Loading):");
            ui.horizontal(|ui| {
                ui.add(Spinner::new().size(SpinnerSize::Small));
                ui.add(Spinner::new().size(SpinnerSize::Medium));
                ui.add(Spinner::new().size(SpinnerSize::Large));
                ui.add(Spinner::new().size(SpinnerSize::XLarge));
                ui.add(Spinner::new().size(SpinnerSize::XXLarge));
                ui.add(Spinner::new().size(SpinnerSize::XXXLarge));
            });

            ui.add_space(16.0);

            ui.label("Keyboard Shortcuts:");
            ui.horizontal(|ui| {
                ui.label("Press");
                ui.add(Kbd::new("Ctrl"));
                ui.label("+");
                ui.add(Kbd::new("K"));
                ui.label("to search");
            });

            ui.add_space(16.0);

            ui.heading("Phase 3: Form Components");
            ui.add_space(8.0);

            ui.label("Checkbox:");
            ui.add(Checkbox::new(&mut self.checkbox_checked).label("Accept terms and conditions"));
            ui.add(Checkbox::new(&mut self.checkbox_checked)
                .label("Accept terms and conditions")
                .description("By clicking this checkbox, you agree to the terms and conditions."));

            ui.add_space(8.0);

            ui.label("Switch:");
            ui.add(Switch::new(&mut self.switch_enabled).label("Enable notifications"));

            ui.add_space(8.0);

            ui.label("Radio Group:");
            RadioGroup::new("notification_type", &mut self.radio_selected)
                .option("All notifications")
                .option("Mentions only")
                .option("None")
                .show(ui);

            ui.add_space(8.0);

            ui.label("Select:");
            Select::new("fruit_select", &mut self.select_fruit)
                .placeholder("Select a fruit...")
                .option("Apple")
                .option("Banana")
                .option("Cherry")
                .option("Date")
                .option("Elderberry")
                .show(ui);

            ui.add_space(8.0);

            ui.label("Combobox (searchable select):");
            let frameworks = vec![
                ComboboxOption::with_label("next", "Next.js"),
                ComboboxOption::with_label("svelte", "SvelteKit"),
                ComboboxOption::with_label("nuxt", "Nuxt.js"),
                ComboboxOption::with_label("remix", "Remix"),
                ComboboxOption::with_label("astro", "Astro"),
            ];
            Combobox::new("framework_combo", &frameworks, &mut self.combobox_framework)
                .placeholder("Select framework...")
                .search_placeholder("Search frameworks...")
                .width(200.0)
                .show(ui);

            ui.add_space(8.0);

            ui.label("Dropdown Menu:");
            ui.horizontal(|ui| {
                let menu = DropdownMenu::new("actions_menu")
                    .trigger_text("Actions")
                    .item_with_shortcut("Copy", "Ctrl+C")
                    .item_with_shortcut("Paste", "Ctrl+V")
                    .separator()
                    .label("Danger Zone")
                    .destructive_item("Delete")
                    .show(ui);

                if let Some(idx) = menu.clicked_item {
                    ui.label(format!("Clicked item {}", idx));
                }
            });

            ui.add_space(8.0);

            ui.label("Slider:");
            ui.horizontal(|ui| {
                ui.add_sized([300.0, 20.0], Slider::new(&mut self.slider_value, 0.0..=100.0));
                ui.label(format!("Value: {:.0}", self.slider_value));
            });

            ui.add_space(8.0);

            ui.label("Progress:");
            ui.add_sized([300.0, 20.0], Progress::new(self.progress_value));
            ui.horizontal(|ui| {
                if ui.button("-").clicked() {
                    self.progress_value = (self.progress_value - 0.1).max(0.0);
                }
                if ui.button("+").clicked() {
                    self.progress_value = (self.progress_value + 0.1).min(1.0);
                }
                ui.label(format!("{:.0}%", self.progress_value * 100.0));
            });

            ui.add_space(16.0);

            // Constrain form width for better aesthetics on wide screens
            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_max_width(600.0);

                Card::new(ui)
                    .header(|ui| {
                        card_title(ui, "Contact Form");
                        card_description(ui, "Fill in your details");
                    })
                    .content(|ui| {
                        form_label(ui, "Email");
                        shadcn_input(ui, &mut self.email, "Enter your email...");
                        ui.add_space(8.0);

                        form_label(ui, "Message");
                        shadcn_textarea(ui, &mut self.message, "Type your message...");
                        ui.add_space(16.0);
                        form_helper(ui, "We'll never share your email.");
                    })
                    .footer(|ui| {
                        ui.add(Button::new("Submit"));
                    })
                    .show();
            });

            ui.add_space(16.0);

            ui.heading("Phase 4: Navigation & Layout");
            ui.add_space(8.0);

            ui.label("Separator:");
            ui.label("Content above");
            Separator::horizontal().show(ui);
            ui.label("Content below");

            ui.add_space(16.0);

            ui.label("Tabs:");
            Tabs::new(ui, "demo-tabs")
                .tab("overview", "Overview", |ui| {
                    ui.label("Overview content goes here");
                    ui.label("This is the first tab");
                })
                .tab("settings", "Settings", |ui| {
                    ui.label("Settings content");
                    ui.checkbox(&mut self.dark_mode, "Enable dark mode");
                })
                .tab("about", "About", |ui| {
                    ui.label("About this showcase");
                    ui.label("Built with egui_shadcn");
                })
                .show();

            ui.add_space(16.0);

            ui.label("Collapsible:");
            Collapsible::new("demo_collapsible", &mut self.collapsible_open)
                .trigger(|ui, _is_open| {
                    // Shadcn style: semibold text with double chevron button right after
                    ui.horizontal(|ui| {
                        // Semibold text
                        ui.label(egui::RichText::new("@peduarte starred 3 repositories")
                            .size(14.0)
                            .strong());

                        ui.add_space(8.0); // gap-4 in tailwind = 16px, but 8px looks cleaner

                        // Double chevron icon (ChevronsUpDown) as ghost button style
                        let chevron_size = egui::vec2(32.0, 32.0);
                        let (rect, response) = ui.allocate_exact_size(chevron_size, egui::Sense::hover());

                        if ui.is_rect_visible(rect) {
                            // Draw hover background if hovered
                            if response.hovered() {
                                ui.painter().rect_filled(
                                    rect,
                                    egui::CornerRadius::same(6),
                                    ui.visuals().widgets.hovered.bg_fill.gamma_multiply(0.5),
                                );
                            }

                            let painter = ui.painter();
                            let cx = rect.center().x;
                            let color = ui.visuals().weak_text_color();
                            let stroke = egui::Stroke::new(1.2, color);

                            // Up chevron (^)
                            let up_y = rect.center().y - 4.0;
                            painter.line_segment(
                                [egui::pos2(cx - 3.0, up_y + 2.0), egui::pos2(cx, up_y - 1.0)],
                                stroke,
                            );
                            painter.line_segment(
                                [egui::pos2(cx, up_y - 1.0), egui::pos2(cx + 3.0, up_y + 2.0)],
                                stroke,
                            );

                            // Down chevron (v)
                            let down_y = rect.center().y + 4.0;
                            painter.line_segment(
                                [egui::pos2(cx - 3.0, down_y - 2.0), egui::pos2(cx, down_y + 1.0)],
                                stroke,
                            );
                            painter.line_segment(
                                [egui::pos2(cx, down_y + 1.0), egui::pos2(cx + 3.0, down_y - 2.0)],
                                stroke,
                            );
                        }
                    });
                })
                .content(|ui| {
                    // Shadcn style: items in rounded outline boxes
                    let items = ["@radix-ui/primitives", "@radix-ui/colors", "@stitches/react"];
                    for item in items {
                        egui::Frame::NONE
                            .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
                            .corner_radius(egui::CornerRadius::same(6))
                            .inner_margin(egui::Margin::symmetric(12, 8))
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new(item).size(14.0));
                            });
                        ui.add_space(4.0);
                    }
                })
                .show(ui);

            ui.add_space(16.0);

            ui.label("Accordion:");
            Accordion::new("demo_accordion")
                .accordion_type(AccordionType::Single)
                .collapsible(true)
                .item("item-1", "Is it accessible?", |ui| {
                    ui.label("Yes. It adheres to the WAI-ARIA design pattern.");
                })
                .item("item-2", "Is it styled?", |ui| {
                    ui.label("Yes. It comes with default styles matching shadcn/ui.");
                })
                .item("item-3", "Is it animated?", |ui| {
                    ui.label("Yes. It uses smooth animations for expanding and collapsing.");
                })
                .show(ui);

            ui.add_space(16.0);

            ui.label("Breadcrumb:");
            Breadcrumb::new()
                .item("Home", || {})
                .item("Products", || {})
                .item("Electronics", || {})
                .current("Laptop")
                .show(ui);

            ui.add_space(8.0);
            ui.label("Breadcrumb (slash separator):");
            Breadcrumb::new()
                .separator(BreadcrumbSeparator::Slash)
                .item("docs", || {})
                .item("components", || {})
                .current("breadcrumb")
                .show(ui);

            ui.add_space(16.0);

            ui.heading("Phase 5: Overlays & Interactions");
            ui.add_space(8.0);

            ui.label("Toggle (toolbar button style):");
            ui.horizontal(|ui| {
                ui.add(Toggle::new(&mut self.toggle_bold, "B").size(ToggleSize::Default));
                ui.add(Toggle::new(&mut self.toggle_italic, "I").size(ToggleSize::Default));
                ui.add(Toggle::new(&mut self.toggle_underline, "U").variant(ToggleVariant::Outline));
            });
            ui.label(format!(
                "Active: Bold={}, Italic={}, Underline={}",
                self.toggle_bold, self.toggle_italic, self.toggle_underline
            ));

            ui.add_space(16.0);

            ui.label("Toggle Group (single selection):");
            ToggleGroup::new("alignment", &mut self.toggle_group_selected)
                .group_type(ToggleGroupType::Single)
                .item("left", "Left")
                .item("center", "Center")
                .item("right", "Right")
                .show(ui);
            ui.label(format!("Selected: {:?}", self.toggle_group_selected));

            ui.add_space(16.0);

            ui.label("Tooltip (hover over buttons):");
            ui.horizontal(|ui| {
                ui.add(Button::new("Hover me")).shadcn_tooltip("This is a shadcn-styled tooltip!");
                ui.add(Button::new("Another").variant(ButtonVariant::Secondary))
                    .shadcn_tooltip("Tooltips use inverted colors");
                ui.add(Button::new("Custom").variant(ButtonVariant::Outline))
                    .shadcn_tooltip_ui(|ui| {
                        ui.label("Custom tooltip content");
                        ui.label("With multiple lines!");
                    });
            });

            ui.add_space(16.0);

            ui.label("Hover Card (hover with delay):");
            ui.horizontal(|ui| {
                let link = ui.link("@shadcn");
                link.hover_card(ui, "user_hover_card", |ui| {
                    ui.label(egui::RichText::new("@shadcn").strong());
                    ui.label("The creator of shadcn/ui");
                    ui.add_space(8.0);
                    ui.label("Building beautiful components for the web.");
                });
            });

            ui.add_space(16.0);

            ui.label("Popover (click to open):");
            ui.horizontal(|ui| {
                let trigger = ui.add(Button::new("Open Popover"));
                Popover::new("demo_popover")
                    .width(280.0)
                    .show(ui, &trigger, |ui| {
                        ui.label("Dimensions");
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.label("Width:");
                            ui.add(egui::DragValue::new(&mut self.popover_width).range(100.0..=400.0));
                        });
                        ui.add_space(8.0);
                        ui.label("Click outside to close");
                    });
            });

            ui.add_space(16.0);

            ui.label("Sheet (slide-out panel):");
            ui.horizontal(|ui| {
                if ui.add(Button::new("Open Sheet")).clicked() {
                    self.sheet_open = true;
                }
            });
            Sheet::new("demo_sheet", &mut self.sheet_open)
                .side(SheetSide::Right)
                .title("Edit Profile")
                .description("Make changes to your profile here. Click outside or press Escape to close.")
                .show(ui, |ui| {
                    form_label(ui, "Name");
                    shadcn_input(ui, &mut self.email, "Enter name...");
                    ui.add_space(8.0);
                    form_label(ui, "Bio");
                    shadcn_textarea(ui, &mut self.message, "Tell us about yourself...");
                    ui.add_space(16.0);
                    ui.add(Button::new("Save changes"));
                });

            ui.add_space(16.0);

            ui.label("Drawer (bottom panel):");
            ui.horizontal(|ui| {
                if ui.add(Button::new("Open Drawer").variant(ButtonVariant::Secondary)).clicked() {
                    self.drawer_open = true;
                }
            });
            let mut close_drawer = false;
            Drawer::new("demo_drawer", &mut self.drawer_open)
                .side(DrawerSide::Bottom)
                .title("Move Goal")
                .description("Set your daily activity goal.")
                .show(ui, |ui| {
                    ui.add_space(8.0);
                    form_label(ui, "Goal (calories)");
                    ui.add(Slider::new(&mut self.slider_value, 0.0..=500.0));
                    ui.label(format!("{:.0} cal", self.slider_value));
                    ui.add_space(16.0);
                    if ui.add(Button::new("Submit")).clicked() {
                        close_drawer = true;
                    }
                });
            if close_drawer {
                self.drawer_open = false;
            }

            ui.add_space(16.0);

            ui.label("Toast notifications:");
            ui.horizontal(|ui| {
                if ui.add(Button::new("Info Toast")).clicked() {
                    self.toaster.info("This is an info message");
                }
                if ui.add(Button::new("Success Toast").variant(ButtonVariant::Secondary)).clicked() {
                    self.toaster.add(
                        Toast::new("Success!")
                            .description("Your action was completed successfully.")
                            .variant(ToastVariant::Success)
                    );
                }
                if ui.add(Button::new("Error Toast").variant(ButtonVariant::Destructive)).clicked() {
                    self.toaster.add(
                        Toast::new("Error occurred")
                            .description("Something went wrong. Please try again.")
                            .variant(ToastVariant::Destructive)
                    );
                }
            });

            ui.add_space(16.0);

            ui.label("Dialog:");
            ui.horizontal(|ui| {
                if ui.add(Button::new("Open Dialog")).clicked() {
                    self.dialog_open = true;
                }
                if ui.add(Button::new("Confirm Dialog").variant(ButtonVariant::Secondary)).clicked() {
                    self.confirm_dialog_open = true;
                    self.confirm_result = None;
                }
            });

            if let Some(result) = self.confirm_result {
                ui.label(format!("Last confirm result: {:?}", result));
            }

            // Show dialogs
            Dialog::new("example_dialog")
                .title("Edit Profile")
                .description("Make changes to your profile here.")
                .show(ctx, &mut self.dialog_open, |ui| {
                    form_label(ui, "Name");
                    shadcn_input(ui, &mut self.email, "Enter your name...");
                    ui.add_space(12.0);
                    if ui.add(Button::new("Save Changes")).clicked() {
                        ui.close();
                    }
                });

            let result = confirm_dialog(
                ctx,
                "confirm_example",
                &mut self.confirm_dialog_open,
                "Are you sure?",
                "This action cannot be undone.",
            );
            if result != ConfirmResult::Pending {
                self.confirm_result = Some(result);
            }

            ui.add_space(16.0);

            ui.heading("Phase 6: Data Display & Advanced");
            ui.add_space(8.0);

            ui.label("Alert Dialog (destructive confirmation):");
            ui.horizontal(|ui| {
                if ui.add(Button::new("Delete Account").variant(ButtonVariant::Destructive)).clicked() {
                    self.alert_dialog_open = true;
                }
                if let Some(result) = self.alert_dialog_result {
                    ui.label(format!("Result: {:?}", result));
                }
            });

            // Show alert dialog
            let alert_result = AlertDialog::new("delete_account")
                .title("Are you absolutely sure?")
                .description("This action cannot be undone. This will permanently delete your account and remove your data from our servers.")
                .cancel_text("Cancel")
                .action_text("Delete")
                .destructive(true)
                .show(ctx, &mut self.alert_dialog_open);
            if alert_result != AlertDialogResult::Pending {
                self.alert_dialog_result = Some(alert_result);
            }

            ui.add_space(16.0);

            ui.label("Context Menu (right-click the box below):");
            let menu_response = ContextMenu::new("demo_context")
                .item("Edit")
                .item("Duplicate")
                .separator()
                .item("Share")
                .separator()
                .destructive_item("Delete")
                .show(ui, |ui| {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(200.0, 60.0),
                        egui::Sense::click(),
                    );
                    if ui.is_rect_visible(rect) {
                        ui.painter().rect_filled(
                            rect,
                            theme.radii.md,
                            theme.colors.muted,
                        );
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "Right-click me",
                            egui::FontId::proportional(14.0),
                            theme.colors.muted_foreground,
                        );
                    }
                });

            if let Some(idx) = menu_response.clicked_item {
                self.toaster.info(format!("Context menu item {} clicked", idx));
            }

            ui.add_space(16.0);

            ui.label("Pagination:");
            ui.horizontal(|ui| {
                Pagination::new(&mut self.current_page, 10).show(ui);
                ui.label(format!("Current page: {}", self.current_page));
            });

            ui.add_space(16.0);

            ui.label("Aspect Ratio containers:");
            ui.horizontal(|ui| {
                // 1:1 Square
                AspectRatio::new(AspectRatioPreset::Square)
                    .max_width(100.0)
                    .show(ui, |ui| {
                        let rect = ui.available_rect_before_wrap();
                        ui.painter().rect_filled(rect, 4.0, theme.colors.primary);
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "1:1",
                            egui::FontId::proportional(14.0),
                            theme.colors.primary_foreground,
                        );
                    });

                ui.add_space(8.0);

                // 16:9 Video
                AspectRatio::new(AspectRatioPreset::Video)
                    .max_width(160.0)
                    .show(ui, |ui| {
                        let rect = ui.available_rect_before_wrap();
                        ui.painter().rect_filled(rect, 4.0, theme.colors.secondary);
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "16:9",
                            egui::FontId::proportional(14.0),
                            theme.colors.secondary_foreground,
                        );
                    });

                ui.add_space(8.0);

                // 4:3 Standard
                AspectRatio::new(AspectRatioPreset::Standard)
                    .max_width(120.0)
                    .show(ui, |ui| {
                        let rect = ui.available_rect_before_wrap();
                        ui.painter().rect_filled(rect, 4.0, theme.colors.accent);
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "4:3",
                            egui::FontId::proportional(14.0),
                            theme.colors.accent_foreground,
                        );
                    });
            });

            ui.add_space(16.0);

            ui.label("Table:");
            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_max_width(500.0);
                simple_table(ui, &["Invoice", "Status", "Method", "Amount"], &[
                    &["INV001", "Paid", "Credit Card", "$250.00"],
                    &["INV002", "Pending", "PayPal", "$150.00"],
                    &["INV003", "Unpaid", "Bank Transfer", "$350.00"],
                    &["INV004", "Paid", "Credit Card", "$450.00"],
                ]);
            });

            ui.add_space(16.0);

            ui.label("Command Palette (press Ctrl+K or click button):");
            ui.horizontal(|ui| {
                if ui.add(Button::new("Open Command Palette")).clicked() {
                    self.command_open = true;
                }
                ui.add(Kbd::new("Ctrl"));
                ui.label("+");
                ui.add(Kbd::new("K"));
            });

            // Check for Ctrl+K shortcut
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::K)) {
                self.command_open = !self.command_open;
            }

            // Show command palette
            if let Some((group, item)) = Command::new("demo_command", &mut self.command_open, &mut self.command_search)
                .placeholder("Type a command or search...")
                .group("Suggestions", |cmd| {
                    cmd.item("Calendar");
                    cmd.item("Search Emoji");
                    cmd.item("Calculator");
                })
                .group("Settings", |cmd| {
                    cmd.item_with_shortcut("Profile", "Ctrl+P");
                    cmd.item_with_shortcut("Billing", "Ctrl+B");
                    cmd.item_with_shortcut("Settings", "Ctrl+S");
                })
                .show(ui)
            {
                self.toaster.info(format!("Selected: group {}, item {}", group, item));
                self.command_search.clear();
            }

            ui.add_space(16.0);

            ui.label("Calendar:");
            ui.horizontal(|ui| {
                Calendar::new("demo_calendar", &mut self.calendar_selection, &mut self.calendar_view_date)
                    .show(ui);

                ui.add_space(16.0);

                // Show selected date
                ui.vertical(|ui| {
                    ui.label("Selected:");
                    match &self.calendar_selection {
                        CalendarSelection::None => {
                            ui.label(egui::RichText::new("None").color(ui.visuals().weak_text_color()));
                        }
                        CalendarSelection::Single(date) => {
                            ui.label(date.format("%B %d, %Y").to_string());
                        }
                        CalendarSelection::Range(start, end) => {
                            ui.label(format!("{} - {}", start.format("%b %d"), end.format("%b %d, %Y")));
                        }
                    }
                });
            });

            ui.add_space(16.0);

            ui.label("DatePicker (Calendar in popover):");
            ui.horizontal(|ui| {
                DatePicker::new("demo_datepicker", &mut self.datepicker_date)
                    .placeholder("Pick a date")
                    .show(ui);

                if let Some(date) = self.datepicker_date {
                    ui.add_space(8.0);
                    ui.label(format!("Selected: {}", date.format("%Y-%m-%d")));
                }
            });

            ui.add_space(16.0);

            ui.label("Carousel:");
            let carousel_items = ["Slide 1", "Slide 2", "Slide 3", "Slide 4", "Slide 5"];
            Carousel::new("demo_carousel", &mut self.carousel_index, carousel_items.len())
                .item_size(egui::vec2(280.0, 160.0))
                .show(ui, |ui, index| {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            egui::RichText::new(carousel_items[index])
                                .size(32.0)
                                .strong()
                        );
                    });
                });

            ui.add_space(16.0);

            ui.label("Chart (Bar):");
            let chart_data = vec![
                DataPoint::new("Jan", 186.0),
                DataPoint::new("Feb", 305.0),
                DataPoint::new("Mar", 237.0),
                DataPoint::new("Apr", 273.0),
                DataPoint::new("May", 209.0),
                DataPoint::new("Jun", 214.0),
            ];
            Chart::new("demo_bar_chart", &chart_data)
                .chart_type(ChartType::Bar)
                .size(egui::vec2(350.0, 180.0))
                .show(ui);

            ui.add_space(16.0);

            ui.label("Chart (Line):");
            Chart::new("demo_line_chart", &chart_data)
                .chart_type(ChartType::Line)
                .size(egui::vec2(350.0, 180.0))
                .show(ui);

            ui.add_space(16.0);

            ui.label("Chart (Area):");
            Chart::new("demo_area_chart", &chart_data)
                .chart_type(ChartType::Area)
                .size(egui::vec2(350.0, 180.0))
                .show(ui);

            ui.add_space(16.0);

            ui.label("Resizable Panels (drag the handles):");
            let h_split_value = self.resizable_split; // Copy for display
            let v_split_value = self.resizable_vertical_split;
            egui::Frame::NONE
                .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
                .corner_radius(egui::CornerRadius::same(8))
                .show(ui, |ui| {
                    ui.set_min_size(egui::vec2(400.0, 200.0));

                    // Outer vertical split: top section (horizontal panels) / bottom panel
                    ResizablePanelGroup::new("demo_resizable_v", &mut self.resizable_vertical_split)
                        .direction(ResizableDirection::Vertical)
                        .min_size(60.0)
                        .show(ui, |ui, vpanel| {
                            match vpanel {
                                0 => {
                                    // Top section: horizontal split
                                    ResizablePanelGroup::new("demo_resizable_h", &mut self.resizable_split)
                                        .min_size(80.0)
                                        .show(ui, |ui, hpanel| {
                                            ui.vertical_centered(|ui| {
                                                ui.add_space(20.0);
                                                match hpanel {
                                                    0 => {
                                                        ui.label(egui::RichText::new("Panel One").strong());
                                                        ui.label(format!("{:.0}%", h_split_value * 100.0));
                                                    }
                                                    1 => {
                                                        ui.label(egui::RichText::new("Panel Two").strong());
                                                        ui.label(format!("{:.0}%", (1.0 - h_split_value) * 100.0));
                                                    }
                                                    _ => {}
                                                }
                                            });
                                        });
                                }
                                1 => {
                                    // Bottom panel (full width)
                                    ui.vertical_centered(|ui| {
                                        ui.add_space(15.0);
                                        ui.label(egui::RichText::new("Panel Three").strong());
                                        ui.label("(spans full width, vertical resize only)");
                                    });
                                }
                                _ => {}
                            }
                        });
                });

            ui.add_space(16.0);

            ui.heading("Phase 7: Navigation & Forms");
            ui.add_space(8.0);

            ui.label("Menubar (application-style menu):");
            Menubar::new("demo_menubar")
                .menu("File", |m| {
                    m.item("New");
                    m.item_with_shortcut("Open", "Ctrl+O");
                    m.item_with_shortcut("Save", "Ctrl+S");
                    m.separator();
                    m.item("Exit");
                })
                .menu("Edit", |m| {
                    m.item_with_shortcut("Undo", "Ctrl+Z");
                    m.item_with_shortcut("Redo", "Ctrl+Y");
                    m.separator();
                    m.item_with_shortcut("Cut", "Ctrl+X");
                    m.item_with_shortcut("Copy", "Ctrl+C");
                    m.item_with_shortcut("Paste", "Ctrl+V");
                })
                .menu("View", |m| {
                    m.item("Zoom In");
                    m.item("Zoom Out");
                    m.separator();
                    m.item("Full Screen");
                })
                .menu("Help", |m| {
                    m.item("Documentation");
                    m.item("About");
                })
                .show(ui);

            ui.add_space(16.0);

            ui.label("Navigation Menu (website-style navigation):");
            let nav_response = NavigationMenu::new("demo_nav")
                .item("Home")
                .dropdown("Getting Started", |dd| {
                    dd.item("Introduction", "Re-usable components built with egui and shadcn design.");
                    dd.item("Installation", "How to install dependencies and configure your project.");
                    dd.item("Typography", "Styles for headings, paragraphs, lists and more.");
                })
                .dropdown("Components", |dd| {
                    dd.item("Alert Dialog", "A modal dialog that interrupts user workflow.");
                    dd.item("Button", "Displays a button or a component that looks like a button.");
                    dd.item("Card", "Displays a card with header, content, and footer.");
                })
                .item("Documentation")
                .show(ui);

            if let Some((menu_idx, sub_idx)) = nav_response.clicked_item {
                let msg = if let Some(sub) = sub_idx {
                    format!("Nav clicked: menu {}, item {}", menu_idx, sub)
                } else {
                    format!("Nav link clicked: {}", menu_idx)
                };
                self.toaster.info(msg);
            }

            ui.add_space(16.0);

            ui.label("Sidebar (collapsible side navigation):");
            ui.label("The sidebar shows icons when collapsed and full text when expanded.");
            ui.add_space(8.0);

            // Show sidebar in a constrained area
            ui.horizontal(|ui| {
                // Sidebar on left
                // Store sidebar open state for header/footer to check
                let is_sidebar_open = self.sidebar_open;

                // Get theme for sidebar colors
                let theme = ui.ctx().data(|d| {
                    d.get_temp::<egui_shadcn::ShadcnTheme>(egui::Id::new("shadcn_theme"))
                        .unwrap_or_else(egui_shadcn::ShadcnTheme::light)
                });
                let sidebar_fg = theme.colors.sidebar_foreground;
                let sidebar_muted = theme.colors.sidebar_foreground.linear_multiply(0.7);

                let sidebar_response = Sidebar::new("demo_sidebar", &mut self.sidebar_open)
                    .width(220.0)
                    .collapsed_width(56.0)
                    .header(move |ui| {
                        ui.horizontal(|ui| {
                            // Logo icon (simple box)
                            ui.label(egui::RichText::new("[A]").size(16.0).strong().color(sidebar_fg));
                            if is_sidebar_open {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new("Acme Inc").strong().color(sidebar_fg));
                                    ui.label(egui::RichText::new("Enterprise").size(11.0).color(sidebar_muted));
                                });
                                // Add chevrons for team/workspace dropdown
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let chevron_size = egui::vec2(16.0, 20.0);
                                    let (rect, response) = ui.allocate_exact_size(chevron_size, egui::Sense::click());

                                    if ui.is_rect_visible(rect) {
                                        let painter = ui.painter();
                                        let color = sidebar_muted;
                                        let stroke = egui::Stroke::new(1.2, color);
                                        let cx = rect.center().x;

                                        // Up chevron (^)
                                        let up_y = rect.center().y - 4.0;
                                        painter.line_segment(
                                            [egui::pos2(cx - 3.0, up_y + 2.0), egui::pos2(cx, up_y - 1.0)],
                                            stroke,
                                        );
                                        painter.line_segment(
                                            [egui::pos2(cx, up_y - 1.0), egui::pos2(cx + 3.0, up_y + 2.0)],
                                            stroke,
                                        );

                                        // Down chevron (v)
                                        let down_y = rect.center().y + 4.0;
                                        painter.line_segment(
                                            [egui::pos2(cx - 3.0, down_y - 2.0), egui::pos2(cx, down_y + 1.0)],
                                            stroke,
                                        );
                                        painter.line_segment(
                                            [egui::pos2(cx, down_y + 1.0), egui::pos2(cx + 3.0, down_y - 2.0)],
                                            stroke,
                                        );
                                    }

                                    let popup_id = ui.make_persistent_id("team_menu_popup");
                                    if response.clicked() {
                                        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                                    }

                                    egui::popup_below_widget(ui, popup_id, &response, egui::PopupCloseBehavior::CloseOnClickOutside, |ui| {
                                        ui.set_min_width(200.0);

                                        // Get theme for proper text colors
                                        let popup_theme = ui.ctx().data(|d| {
                                            d.get_temp::<egui_shadcn::ShadcnTheme>(egui::Id::new("shadcn_theme"))
                                                .unwrap_or_else(egui_shadcn::ShadcnTheme::light)
                                        });
                                        let popup_fg = popup_theme.colors.popover_foreground;
                                        let popup_muted = popup_theme.colors.muted_foreground;

                                        // Team header
                                        ui.label(egui::RichText::new("Teams").size(11.0).color(popup_muted));
                                        ui.add_space(4.0);

                                        // Team items with explicit colors
                                        ui.label(egui::RichText::new("[A]  Acme Inc").strong().color(popup_fg));
                                        ui.label(egui::RichText::new("[M]  Acme Corp.").color(popup_fg));
                                        ui.label(egui::RichText::new("[E]  Evil Corp.").color(popup_fg));
                                        ui.separator();
                                        ui.label(egui::RichText::new("+  Add team").color(popup_fg));
                                    });
                                });
                            }
                        });
                    })
                    .section("Platform", |s| {
                        // Regular items with simple ASCII icons
                        s.item("Documentation", "D", self.sidebar_selected == 0);
                        s.item("Settings", "S", self.sidebar_selected == 1);
                    })
                    // Collapsible menu items with sub-items
                    .menu("Playground", "P", |s| {
                        s.sub_item("History", self.sidebar_selected == 2);
                        s.sub_item("Starred", self.sidebar_selected == 3);
                        s.sub_item("Settings", self.sidebar_selected == 4);
                    })
                    .menu("Models", "M", |s| {
                        s.sub_item("Genesis", self.sidebar_selected == 5);
                        s.sub_item("Explorer", self.sidebar_selected == 6);
                        s.sub_item("Quantum", self.sidebar_selected == 7);
                    })
                    .separator()
                    // Projects with context menu (three dots -> View/Share/Delete)
                    .project_item("Design Engineering", "E", self.sidebar_selected == 8, vec!["View Project", "Share Project", "Delete Project"])
                    .project_item("Sales & Marketing", "M", self.sidebar_selected == 9, vec!["View Project", "Share Project", "Delete Project"])
                    .footer(move |ui| {
                        ui.horizontal(|ui| {
                            ui.add(egui_shadcn::Avatar::new("SC").size(egui_shadcn::AvatarSize::Small));
                            if is_sidebar_open {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new("shadcn").color(sidebar_fg));
                                    ui.label(egui::RichText::new("m@example.com").size(11.0).color(sidebar_muted));
                                });
                                // Up/down chevron for account menu - draw custom chevrons
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let chevron_size = egui::vec2(16.0, 20.0);
                                    let (rect, response) = ui.allocate_exact_size(chevron_size, egui::Sense::click());

                                    if ui.is_rect_visible(rect) {
                                        let painter = ui.painter();
                                        let stroke = egui::Stroke::new(1.2, sidebar_muted);
                                        let cx = rect.center().x;

                                        // Up chevron (^)
                                        let up_y = rect.center().y - 4.0;
                                        painter.line_segment(
                                            [egui::pos2(cx - 3.0, up_y + 2.0), egui::pos2(cx, up_y - 1.0)],
                                            stroke,
                                        );
                                        painter.line_segment(
                                            [egui::pos2(cx, up_y - 1.0), egui::pos2(cx + 3.0, up_y + 2.0)],
                                            stroke,
                                        );

                                        // Down chevron (v)
                                        let down_y = rect.center().y + 4.0;
                                        painter.line_segment(
                                            [egui::pos2(cx - 3.0, down_y - 2.0), egui::pos2(cx, down_y + 1.0)],
                                            stroke,
                                        );
                                        painter.line_segment(
                                            [egui::pos2(cx, down_y + 1.0), egui::pos2(cx + 3.0, down_y - 2.0)],
                                            stroke,
                                        );
                                    }

                                    let popup_id = ui.make_persistent_id("account_menu_popup");
                                    if response.clicked() {
                                        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                                    }

                                    egui::popup_below_widget(ui, popup_id, &response, egui::PopupCloseBehavior::CloseOnClickOutside, |ui| {
                                        ui.set_min_width(200.0);

                                        // Get theme for proper text colors
                                        let popup_theme = ui.ctx().data(|d| {
                                            d.get_temp::<egui_shadcn::ShadcnTheme>(egui::Id::new("shadcn_theme"))
                                                .unwrap_or_else(egui_shadcn::ShadcnTheme::light)
                                        });
                                        let popup_fg = popup_theme.colors.popover_foreground;
                                        let popup_muted = popup_theme.colors.muted_foreground;

                                        // User info header
                                        ui.horizontal(|ui| {
                                            ui.add(egui_shadcn::Avatar::new("SC").size(egui_shadcn::AvatarSize::Small));
                                            ui.vertical(|ui| {
                                                ui.label(egui::RichText::new("shadcn").strong().color(popup_fg));
                                                ui.label(egui::RichText::new("m@example.com").size(11.0).color(popup_muted));
                                            });
                                        });

                                        ui.separator();

                                        // Menu items with explicit colors
                                        ui.label(egui::RichText::new("Upgrade to Pro").color(popup_fg));
                                        ui.separator();
                                        ui.label(egui::RichText::new("Account").color(popup_fg));
                                        ui.label(egui::RichText::new("Billing").color(popup_fg));
                                        ui.label(egui::RichText::new("Notifications").color(popup_fg));
                                        ui.separator();
                                        ui.label(egui::RichText::new("Log out").color(popup_fg));
                                    });
                                });
                            }
                        });
                    })
                    .show(ui);

                if let Some(idx) = sidebar_response.clicked_item {
                    self.sidebar_selected = idx;
                }

                // Content area next to sidebar
                egui::Frame::NONE
                    .inner_margin(16.0)
                    .show(ui, |ui| {
                        ui.label("Content area");
                        ui.label(format!("Selected item: {}", self.sidebar_selected));
                        if ui.button("Toggle Sidebar").clicked() {
                            self.sidebar_open = !self.sidebar_open;
                        }
                    });
            });

            ui.add_space(16.0);

            ui.label("Field wrapper (form input with label, description, and error):");
            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_max_width(400.0);

                Field::new("username")
                    .label("Username")
                    .description("This is your public display name.")
                    .show(ui, |ui| {
                        shadcn_input(ui, &mut self.field_username, "Enter username...");
                    });

                ui.add_space(12.0);

                // Validate email on change
                let email_has_error = !self.field_email.is_empty() && !self.field_email.contains('@');
                if email_has_error {
                    self.field_email_error = Some("Please enter a valid email address".to_string());
                } else {
                    self.field_email_error = None;
                }

                Field::new("email")
                    .label("Email")
                    .required(true)
                    .error(self.field_email_error.as_deref())
                    .show(ui, |ui| {
                        // Use error-aware input that shows red border on error
                        shadcn_input_with_error(ui, &mut self.field_email, "Enter email...", email_has_error);
                    });
            });

            ui.add_space(16.0);

            ui.label("Toggle between light and dark modes to see the color system in action!");
            });
        });

        // Show toast notifications
        self.toaster.show(ctx);
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("egui_shadcn Showcase"),
        ..Default::default()
    };

    eframe::run_native(
        "egui_shadcn Showcase",
        options,
        Box::new(|cc| Ok(Box::new(ShowcaseApp::new(cc)))),
    )
}
