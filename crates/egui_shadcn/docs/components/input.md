# Input

Displays a form input field.

![Input Light](../assets/screenshots/light/input.png)

## Usage

```rust
use egui_shadcn::{shadcn_input, shadcn_input_with_error, shadcn_textarea, form_label, form_helper};

let mut email = String::new();
let mut bio = String::new();
let has_error = false;

// Basic input
shadcn_input(ui, &mut email, "Enter your email...");

// Input with error state
shadcn_input_with_error(ui, &mut email, "Email", has_error);

// Multi-line textarea
shadcn_textarea(ui, &mut bio, "Tell us about yourself...");

// With form labels
form_label(ui, "Email Address");
shadcn_input(ui, &mut email, "name@example.com");
form_helper(ui, "We'll never share your email.");
```

## Features

- **Apple HIG Compliant**: 44px minimum touch targets
- **State Styling**: Default, hover, focus, and error states
- **Focus Ring**: Visible focus indicator using theme ring color
- **Placeholder Support**: Hint text for empty fields

## API

```rust
/// Single-line text input
pub fn shadcn_input(
    ui: &mut Ui,
    text: &mut String,
    placeholder: &str
) -> Response;

/// Single-line input with error state
pub fn shadcn_input_with_error(
    ui: &mut Ui,
    text: &mut String,
    placeholder: &str,
    has_error: bool
) -> Response;

/// Multi-line textarea
pub fn shadcn_textarea(
    ui: &mut Ui,
    text: &mut String,
    placeholder: &str
) -> Response;

/// Form label styling
pub fn form_label(ui: &mut Ui, text: &str);

/// Form helper text
pub fn form_helper(ui: &mut Ui, text: &str);
```

## Examples

### Login Form

```rust
fn show_login_form(ui: &mut Ui, state: &mut LoginState) {
    ui.vertical(|ui| {
        form_label(ui, "Email");
        shadcn_input_with_error(
            ui,
            &mut state.email,
            "name@example.com",
            state.email_error.is_some()
        );
        if let Some(error) = &state.email_error {
            ui.colored_label(egui::Color32::RED, error);
        }

        ui.add_space(16.0);

        form_label(ui, "Password");
        shadcn_input(ui, &mut state.password, "Enter password");

        ui.add_space(24.0);

        if Button::new("Sign In").show(ui).clicked() {
            // Handle login
        }
    });
}
```

### Contact Form

```rust
fn show_contact_form(ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                form_label(ui, "First Name");
                shadcn_input(ui, &mut first_name, "John");
            });
            ui.vertical(|ui| {
                form_label(ui, "Last Name");
                shadcn_input(ui, &mut last_name, "Doe");
            });
        });

        ui.add_space(12.0);
        form_label(ui, "Email");
        shadcn_input(ui, &mut email, "john@example.com");

        ui.add_space(12.0);
        form_label(ui, "Message");
        shadcn_textarea(ui, &mut message, "How can we help you?");
        form_helper(ui, "Your message will be reviewed within 24 hours.");

        ui.add_space(24.0);
        Button::new("Send Message").show(ui);
    });
}
```

### Search Input

```rust
fn show_search(ui: &mut Ui, query: &mut String) {
    ui.horizontal(|ui| {
        shadcn_input(ui, query, "Search...");
        if Button::new("Search")
            .variant(ButtonVariant::Outline)
            .show(ui)
            .clicked()
        {
            // Perform search
        }
    });
}
```

### Form Validation

```rust
fn show_validated_input(ui: &mut Ui, email: &mut String) -> bool {
    let is_valid = email.contains('@') && email.contains('.');
    let has_error = !email.is_empty() && !is_valid;

    form_label(ui, "Email");
    shadcn_input_with_error(ui, email, "name@example.com", has_error);

    if has_error {
        ui.colored_label(egui::Color32::RED, "Please enter a valid email address");
    }

    is_valid
}
```
