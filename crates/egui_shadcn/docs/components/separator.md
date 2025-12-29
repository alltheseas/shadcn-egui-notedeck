# Separator

Visually or semantically separates content.

![Separator Light](../assets/screenshots/light/separator.png)

## Usage

```rust
use egui_shadcn::Separator;

// Horizontal separator (default)
ui.add(Separator::horizontal());

// Vertical separator
ui.add(Separator::vertical());
```

## Features

- **Horizontal**: Full-width divider line
- **Vertical**: Full-height divider line
- **Theme Integration**: Uses theme border color

## API

```rust
impl Separator {
    pub fn horizontal() -> Self;
    pub fn vertical() -> Self;
}

// Separator implements Widget trait
impl Widget for Separator {
    fn ui(self, ui: &mut Ui) -> Response;
}
```

## Examples

### Section Dividers

```rust
fn show_profile_sections(ui: &mut Ui) {
    ui.heading("Profile");
    ui.label("Your public profile information");

    ui.add_space(16.0);
    ui.add(Separator::horizontal());
    ui.add_space(16.0);

    ui.heading("Account");
    ui.label("Manage your account settings");

    ui.add_space(16.0);
    ui.add(Separator::horizontal());
    ui.add_space(16.0);

    ui.heading("Notifications");
    ui.label("Configure notification preferences");
}
```

### Menu Dividers

```rust
fn show_menu(ui: &mut Ui) {
    if Button::new("New File").variant(ButtonVariant::Ghost).show(ui).clicked() {
        // Handle new file
    }
    if Button::new("Open").variant(ButtonVariant::Ghost).show(ui).clicked() {
        // Handle open
    }

    ui.add(Separator::horizontal());

    if Button::new("Save").variant(ButtonVariant::Ghost).show(ui).clicked() {
        // Handle save
    }
    if Button::new("Save As").variant(ButtonVariant::Ghost).show(ui).clicked() {
        // Handle save as
    }

    ui.add(Separator::horizontal());

    if Button::new("Exit").variant(ButtonVariant::Ghost).show(ui).clicked() {
        // Handle exit
    }
}
```

### Toolbar with Vertical Separators

```rust
fn show_toolbar(ui: &mut Ui) {
    ui.horizontal(|ui| {
        Button::new("Bold").size(ButtonSize::Icon).show(ui);
        Button::new("Italic").size(ButtonSize::Icon).show(ui);
        Button::new("Underline").size(ButtonSize::Icon).show(ui);

        ui.add(Separator::vertical());

        Button::new("Left").size(ButtonSize::Icon).show(ui);
        Button::new("Center").size(ButtonSize::Icon).show(ui);
        Button::new("Right").size(ButtonSize::Icon).show(ui);

        ui.add(Separator::vertical());

        Button::new("Link").size(ButtonSize::Icon).show(ui);
        Button::new("Image").size(ButtonSize::Icon).show(ui);
    });
}
```

### List with Dividers

```rust
fn show_list_with_dividers(ui: &mut Ui, items: &[String]) {
    for (i, item) in items.iter().enumerate() {
        ui.label(item);

        if i < items.len() - 1 {
            ui.add(Separator::horizontal());
        }
    }
}
```
