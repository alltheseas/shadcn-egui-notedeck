# Kbd

A keyboard input component that displays keyboard shortcuts.

![Kbd Light](../assets/screenshots/light/kbd.png)

## Usage

```rust
use egui_shadcn::Kbd;

// Single key
ui.add(Kbd::new("⌘"));

// Key combination
ui.horizontal(|ui| {
    ui.add(Kbd::new("⌘"));
    ui.add(Kbd::new("K"));
});

// Multiple modifiers
ui.horizontal(|ui| {
    ui.add(Kbd::new("⌘"));
    ui.add(Kbd::new("⇧"));
    ui.add(Kbd::new("P"));
});
```

## Common Keys

| Symbol | Key |
|--------|-----|
| `⌘` | Command (Mac) |
| `⌃` | Control |
| `⇧` | Shift |
| `⌥` | Option/Alt |
| `↵` | Enter/Return |
| `⌫` | Backspace |
| `⇥` | Tab |
| `⎋` | Escape |

## API

```rust
impl Kbd {
    pub fn new(key: impl Into<String>) -> Self;
}

// Kbd implements Widget trait
impl Widget for Kbd {
    fn ui(self, ui: &mut Ui) -> Response;
}
```

## Examples

### Shortcut Hints

```rust
fn show_shortcut_hints(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Save:");
        ui.add(Kbd::new("⌘"));
        ui.add(Kbd::new("S"));
    });

    ui.horizontal(|ui| {
        ui.label("Undo:");
        ui.add(Kbd::new("⌘"));
        ui.add(Kbd::new("Z"));
    });

    ui.horizontal(|ui| {
        ui.label("Search:");
        ui.add(Kbd::new("⌘"));
        ui.add(Kbd::new("K"));
    });
}
```

### Menu Items with Shortcuts

```rust
fn show_menu_with_shortcuts(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("New File");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add(Kbd::new("N"));
            ui.add(Kbd::new("⌘"));
        });
    });

    ui.horizontal(|ui| {
        ui.label("Open File");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add(Kbd::new("O"));
            ui.add(Kbd::new("⌘"));
        });
    });
}
```

### Command Palette

```rust
fn show_command_item(ui: &mut Ui, label: &str, keys: &[&str]) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            for key in keys.iter().rev() {
                ui.add(Kbd::new(*key));
            }
        });
    });
}

// Usage
show_command_item(ui, "Toggle Dark Mode", &["⌘", "⇧", "D"]);
show_command_item(ui, "Open Settings", &["⌘", ","]);
show_command_item(ui, "Quick Search", &["⌘", "P"]);
```

### Inline Help

```rust
fn show_inline_help(ui: &mut Ui) {
    ui.horizontal_wrapped(|ui| {
        ui.label("Press");
        ui.add(Kbd::new("⌘"));
        ui.add(Kbd::new("K"));
        ui.label("to open the command palette, or");
        ui.add(Kbd::new("?"));
        ui.label("for help.");
    });
}
```
