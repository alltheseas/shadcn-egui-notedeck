# Tabs

A set of layered sections of content—known as tab panels—that display one panel at a time.

![Tabs Light](../assets/screenshots/light/tabs.png)

## Usage

```rust
use egui_shadcn::Tabs;

// Basic tabs
Tabs::new(ui, "my-tabs")
    .tab("account", "Account", |ui| {
        ui.label("Account settings go here");
    })
    .tab("password", "Password", |ui| {
        ui.label("Password settings go here");
    })
    .tab("notifications", "Notifications", |ui| {
        ui.label("Notification preferences");
    })
    .show();
```

## Features

- **State Persistence**: Selected tab saved across frames
- **Underline Indicator**: Active tab has underline highlight
- **Apple HIG Compliant**: 44px touch targets
- **Content Border**: Tab content has subtle border

## API

```rust
impl<'a> Tabs<'a> {
    pub fn new(ui: &'a mut Ui, id: &'a str) -> Self;
    pub fn tab(
        self,
        tab_id: &'a str,
        label: &'a str,
        content: impl FnOnce(&mut Ui) + 'a,
    ) -> Self;
    pub fn show(self) -> Response;
}
```

## Examples

### Settings Tabs

```rust
Tabs::new(ui, "settings")
    .tab("profile", "Profile", |ui| {
        form_label(ui, "Display Name");
        shadcn_input(ui, &mut name, "Enter name");

        ui.add_space(12.0);
        form_label(ui, "Bio");
        shadcn_textarea(ui, &mut bio, "Tell us about yourself");
    })
    .tab("account", "Account", |ui| {
        form_label(ui, "Email");
        shadcn_input(ui, &mut email, "email@example.com");

        ui.add_space(12.0);
        form_label(ui, "Password");
        shadcn_input(ui, &mut password, "••••••••");
    })
    .tab("notifications", "Notifications", |ui| {
        Switch::new(&mut email_notifs)
            .label("Email notifications")
            .ui(ui);
        Switch::new(&mut push_notifs)
            .label("Push notifications")
            .ui(ui);
    })
    .show();
```

### Dashboard Tabs

```rust
Tabs::new(ui, "dashboard")
    .tab("overview", "Overview", |ui| {
        ui.horizontal(|ui| {
            Card::new(ui)
                .content(|ui| {
                    ui.heading("1,234");
                    ui.label("Total Users");
                })
                .show();

            Card::new(ui)
                .content(|ui| {
                    ui.heading("$45,231");
                    ui.label("Revenue");
                })
                .show();
        });
    })
    .tab("analytics", "Analytics", |ui| {
        ui.label("Charts and graphs would go here");
    })
    .tab("reports", "Reports", |ui| {
        ui.label("Report generation interface");
    })
    .show();
```

### Code Editor Tabs

```rust
fn show_editor_tabs(ui: &mut Ui, files: &mut Vec<File>) {
    let tab_id = "editor-files";

    // Build tabs dynamically from open files
    let mut tabs = Tabs::new(ui, tab_id);

    for (idx, file) in files.iter_mut().enumerate() {
        let id = format!("file-{}", idx);
        tabs = tabs.tab(
            Box::leak(id.into_boxed_str()),
            Box::leak(file.name.clone().into_boxed_str()),
            |ui| {
                shadcn_textarea(ui, &mut file.content, "");
            }
        );
    }

    tabs.show();
}
```

### Preview Tabs

```rust
Tabs::new(ui, "preview")
    .tab("preview", "Preview", |ui| {
        // Rendered preview
        ui.label("This is how your content will appear");
    })
    .tab("code", "Code", |ui| {
        // Raw code view
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.code(code_content);
        });
    })
    .show();
```
