# Switch

A control that allows the user to toggle between on and off.

![Switch Light](../assets/screenshots/light/switch.png)

## Usage

```rust
use egui_shadcn::Switch;

let mut enabled = false;

// Basic switch
if Switch::new(&mut enabled).ui(ui).changed() {
    println!("Switch toggled: {}", enabled);
}

// With label
Switch::new(&mut enabled)
    .label("Enable notifications")
    .ui(ui);

// Disabled
Switch::new(&mut enabled)
    .enabled(false)
    .label("Feature unavailable")
    .ui(ui);
```

## Features

- **Smooth Animation**: Visual feedback on state change
- **Apple HIG Compliant**: 44px touch targets
- **Shadow Depth**: Thumb has subtle shadow for depth
- **Focus Ring**: Visible hover/focus indicator

## API

```rust
impl<'a> Switch<'a> {
    pub fn new(checked: &'a mut bool) -> Self;
    pub fn label(self, label: impl Into<String>) -> Self;
    pub fn enabled(self, enabled: bool) -> Self;
}

// Switch implements Widget trait
impl Widget for Switch<'_> {
    fn ui(self, ui: &mut Ui) -> Response;
}
```

## Examples

### Settings Panel

```rust
struct Settings {
    dark_mode: bool,
    notifications: bool,
    auto_save: bool,
    analytics: bool,
}

fn show_settings(ui: &mut Ui, settings: &mut Settings) {
    ui.vertical(|ui| {
        ui.heading("Appearance");
        Switch::new(&mut settings.dark_mode)
            .label("Dark mode")
            .ui(ui);

        ui.add_space(16.0);
        ui.heading("Notifications");
        Switch::new(&mut settings.notifications)
            .label("Push notifications")
            .ui(ui);

        ui.add_space(16.0);
        ui.heading("Privacy");
        Switch::new(&mut settings.auto_save)
            .label("Auto-save drafts")
            .ui(ui);
        Switch::new(&mut settings.analytics)
            .label("Usage analytics")
            .ui(ui);
    });
}
```

### Feature Toggles

```rust
let mut airplane_mode = false;
let mut wifi = true;
let mut bluetooth = true;

ui.vertical(|ui| {
    if Switch::new(&mut airplane_mode)
        .label("Airplane mode")
        .ui(ui)
        .changed()
    {
        if airplane_mode {
            wifi = false;
            bluetooth = false;
        }
    }

    ui.add_enabled_ui(!airplane_mode, |ui| {
        Switch::new(&mut wifi).label("Wi-Fi").ui(ui);
        Switch::new(&mut bluetooth).label("Bluetooth").ui(ui);
    });
});
```
