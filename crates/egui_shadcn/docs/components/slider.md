# Slider

An input where the user selects a value from within a given range.

![Slider Light](../assets/screenshots/light/slider.png)

## Usage

```rust
use egui_shadcn::Slider;

let mut volume = 50.0;

// Basic slider (0 to 100)
ui.add(Slider::new(&mut volume, 0.0..=100.0));

// With step increments
ui.add(Slider::new(&mut volume, 0.0..=100.0).step(10.0));

// Custom range
let mut temperature = 20.0;
ui.add(Slider::new(&mut temperature, 15.0..=30.0).step(0.5));

// Disabled
ui.add(Slider::new(&mut volume, 0.0..=100.0).enabled(false));
```

## Features

- **Apple HIG Compliant**: 44px touch target height
- **Click and Drag**: Both click-to-set and drag interactions
- **Step Support**: Optional discrete steps
- **Visual Feedback**: Hover state on thumb

## API

```rust
impl<'a> Slider<'a> {
    pub fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self;
    pub fn step(self, step: f32) -> Self;
    pub fn enabled(self, enabled: bool) -> Self;
}

// Slider implements Widget trait
impl Widget for Slider<'_> {
    fn ui(self, ui: &mut Ui) -> Response;
}
```

## Examples

### Volume Control

```rust
fn show_volume_control(ui: &mut Ui, volume: &mut f32) {
    ui.horizontal(|ui| {
        ui.label("🔊");
        ui.add(Slider::new(volume, 0.0..=100.0));
        ui.label(format!("{}%", *volume as i32));
    });
}
```

### Settings Panel

```rust
struct Settings {
    brightness: f32,
    contrast: f32,
    saturation: f32,
}

fn show_display_settings(ui: &mut Ui, settings: &mut Settings) {
    ui.vertical(|ui| {
        ui.label("Brightness");
        ui.add(Slider::new(&mut settings.brightness, 0.0..=100.0));

        ui.add_space(12.0);
        ui.label("Contrast");
        ui.add(Slider::new(&mut settings.contrast, 0.0..=200.0));

        ui.add_space(12.0);
        ui.label("Saturation");
        ui.add(Slider::new(&mut settings.saturation, 0.0..=200.0));
    });
}
```

### Price Range Filter

```rust
fn show_price_filter(ui: &mut Ui, min_price: &mut f32, max_price: &mut f32) {
    ui.vertical(|ui| {
        ui.label("Price Range");

        ui.horizontal(|ui| {
            ui.label("Min:");
            ui.add(Slider::new(min_price, 0.0..=1000.0).step(10.0));
            ui.label(format!("${}", *min_price as i32));
        });

        ui.horizontal(|ui| {
            ui.label("Max:");
            ui.add(Slider::new(max_price, 0.0..=1000.0).step(10.0));
            ui.label(format!("${}", *max_price as i32));
        });
    });
}
```

### Temperature Control

```rust
fn show_thermostat(ui: &mut Ui, temp: &mut f32) {
    ui.vertical(|ui| {
        ui.heading(format!("{}°C", *temp as i32));
        ui.add(Slider::new(temp, 15.0..=30.0).step(0.5));

        ui.horizontal(|ui| {
            if Button::new("-")
                .size(ButtonSize::Icon)
                .show(ui)
                .clicked()
            {
                *temp = (*temp - 0.5).max(15.0);
            }
            if Button::new("+")
                .size(ButtonSize::Icon)
                .show(ui)
                .clicked()
            {
                *temp = (*temp + 0.5).min(30.0);
            }
        });
    });
}
```
