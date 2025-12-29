# Dialog

A modal dialog that interrupts the user with important content and expects a response.

![Dialog Light](../assets/screenshots/light/dialog.png)

## Usage

```rust
use egui_shadcn::{Dialog, confirm_dialog, ConfirmResult};

let mut dialog_open = false;

// Button to open dialog
if Button::new("Open Dialog").show(ui).clicked() {
    dialog_open = true;
}

// Basic dialog
Dialog::new("my_dialog")
    .title("Edit Profile")
    .description("Make changes to your profile here.")
    .show(ui.ctx(), &mut dialog_open, |ui| {
        form_label(ui, "Name");
        shadcn_input(ui, &mut name, "Enter name");
    });

// Dialog with footer
Dialog::new("confirm")
    .title("Save Changes")
    .description("Your changes will be saved.")
    .show_with_footer(
        ui.ctx(),
        &mut dialog_open,
        |ui| {
            ui.label("Are you sure you want to save?");
        },
        |ui| {
            if Button::new("Save").show(ui).clicked() {
                // Handle save
            }
            if Button::new("Cancel")
                .variant(ButtonVariant::Outline)
                .show(ui)
                .clicked()
            {
                dialog_open = false;
            }
        },
    );

// Simple confirmation dialog
let result = confirm_dialog(
    ui.ctx(),
    "delete_confirm",
    &mut confirm_open,
    "Delete Item",
    "Are you sure you want to delete this item?",
);

match result {
    ConfirmResult::Confirmed => { /* Handle delete */ }
    ConfirmResult::Cancelled => { /* User cancelled */ }
    ConfirmResult::Pending => { /* Dialog still open */ }
}
```

## Features

- **Modal Overlay**: Blocks interaction with background
- **Backdrop**: 50% opacity dark backdrop
- **Close Button**: Optional X button in header
- **Escape Key**: Close on escape press
- **Backdrop Click**: Close when clicking outside

## API

```rust
impl Dialog {
    pub fn new(id: impl Into<Id>) -> Self;
    pub fn title(self, title: impl Into<String>) -> Self;
    pub fn description(self, description: impl Into<String>) -> Self;
    pub fn max_width(self, width: f32) -> Self;
    pub fn closable(self, closable: bool) -> Self;

    pub fn show<R>(
        self,
        ctx: &egui::Context,
        open: &mut bool,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R>;

    pub fn show_with_footer<R>(
        self,
        ctx: &egui::Context,
        open: &mut bool,
        content: impl FnOnce(&mut Ui) -> R,
        footer: impl FnOnce(&mut Ui),
    ) -> Option<R>;
}

// Confirmation helper
pub fn confirm_dialog(
    ctx: &egui::Context,
    id: impl Into<Id>,
    open: &mut bool,
    title: &str,
    message: &str,
) -> ConfirmResult;

pub enum ConfirmResult {
    Confirmed,
    Cancelled,
    Pending,
}
```

## Examples

### Edit Profile Dialog

```rust
let mut dialog_open = false;
let mut name = String::from("John Doe");
let mut email = String::from("john@example.com");

if Button::new("Edit Profile").show(ui).clicked() {
    dialog_open = true;
}

Dialog::new("edit_profile")
    .title("Edit Profile")
    .description("Make changes to your profile here. Click save when you're done.")
    .show_with_footer(
        ui.ctx(),
        &mut dialog_open,
        |ui| {
            form_label(ui, "Name");
            shadcn_input(ui, &mut name, "Enter name");

            ui.add_space(12.0);
            form_label(ui, "Email");
            shadcn_input(ui, &mut email, "Enter email");
        },
        |ui| {
            if Button::new("Save Changes").show(ui).clicked() {
                // Save profile
                dialog_open = false;
            }
            if Button::new("Cancel")
                .variant(ButtonVariant::Ghost)
                .show(ui)
                .clicked()
            {
                dialog_open = false;
            }
        },
    );
```

### Delete Confirmation

```rust
let mut show_delete_confirm = false;

if Button::new("Delete Account")
    .variant(ButtonVariant::Destructive)
    .show(ui)
    .clicked()
{
    show_delete_confirm = true;
}

match confirm_dialog(
    ui.ctx(),
    "delete_account",
    &mut show_delete_confirm,
    "Are you absolutely sure?",
    "This action cannot be undone. This will permanently delete your account and remove your data from our servers.",
) {
    ConfirmResult::Confirmed => {
        // Delete account
    }
    ConfirmResult::Cancelled | ConfirmResult::Pending => {}
}
```

### Share Dialog

```rust
Dialog::new("share")
    .title("Share")
    .description("Anyone with the link can view this document.")
    .show(ui.ctx(), &mut share_open, |ui| {
        ui.horizontal(|ui| {
            shadcn_input(ui, &mut share_link, "");
            if Button::new("Copy")
                .variant(ButtonVariant::Secondary)
                .show(ui)
                .clicked()
            {
                ui.output_mut(|o| o.copied_text = share_link.clone());
            }
        });
    });
```

### Settings Dialog

```rust
Dialog::new("settings")
    .title("Settings")
    .max_width(600.0)
    .show(ui.ctx(), &mut settings_open, |ui| {
        Tabs::new(ui, "settings_tabs")
            .tab("general", "General", |ui| {
                Switch::new(&mut dark_mode)
                    .label("Dark mode")
                    .ui(ui);
            })
            .tab("notifications", "Notifications", |ui| {
                Switch::new(&mut email_notifs)
                    .label("Email notifications")
                    .ui(ui);
            })
            .show();
    });
```
