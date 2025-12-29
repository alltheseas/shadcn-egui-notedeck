# Checkbox

A control that allows the user to toggle between checked and not checked.

![Checkbox Light](../assets/screenshots/light/checkbox.png)

## Usage

```rust
use egui_shadcn::Checkbox;

let mut checked = false;

// Basic checkbox
if Checkbox::new(&mut checked).ui(ui).changed() {
    println!("Checkbox toggled: {}", checked);
}

// With label
Checkbox::new(&mut checked)
    .label("Accept terms and conditions")
    .ui(ui);

// With label and description
Checkbox::new(&mut checked)
    .label("Marketing emails")
    .description("Receive emails about new products and features")
    .ui(ui);

// Indeterminate state (for "select all" scenarios)
Checkbox::new(&mut checked)
    .indeterminate(true)
    .label("Select all")
    .ui(ui);

// Disabled
Checkbox::new(&mut checked)
    .enabled(false)
    .label("Disabled option")
    .ui(ui);
```

## Features

- **Apple HIG Compliant**: 44px touch targets for mobile
- **Indeterminate State**: For partial selections
- **Labels & Descriptions**: Built-in text support
- **Focus Ring**: Visible keyboard focus indicator

## API

```rust
impl<'a> Checkbox<'a> {
    pub fn new(checked: &'a mut bool) -> Self;
    pub fn label(self, label: impl Into<String>) -> Self;
    pub fn description(self, description: impl Into<String>) -> Self;
    pub fn indeterminate(self, indeterminate: bool) -> Self;
    pub fn enabled(self, enabled: bool) -> Self;
}

// Checkbox implements Widget trait
impl Widget for Checkbox<'_> {
    fn ui(self, ui: &mut Ui) -> Response;
}
```

## Examples

### Form with Multiple Options

```rust
struct FormState {
    receive_emails: bool,
    receive_sms: bool,
    receive_push: bool,
}

fn show_notifications_form(ui: &mut Ui, state: &mut FormState) {
    ui.vertical(|ui| {
        ui.label("Notification Preferences");

        Checkbox::new(&mut state.receive_emails)
            .label("Email notifications")
            .description("Get notified about important updates")
            .ui(ui);

        Checkbox::new(&mut state.receive_sms)
            .label("SMS notifications")
            .description("Receive text messages for urgent alerts")
            .ui(ui);

        Checkbox::new(&mut state.receive_push)
            .label("Push notifications")
            .description("Browser and mobile push notifications")
            .ui(ui);
    });
}
```

### Terms Acceptance

```rust
let mut terms_accepted = false;

ui.vertical(|ui| {
    Checkbox::new(&mut terms_accepted)
        .label("I agree to the Terms of Service")
        .ui(ui);

    ui.add_enabled_ui(terms_accepted, |ui| {
        if Button::new("Continue").show(ui).clicked() {
            // Proceed
        }
    });
});
```
