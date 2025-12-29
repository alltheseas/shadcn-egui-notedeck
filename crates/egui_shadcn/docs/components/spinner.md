# Spinner

An animated loading indicator.

![Spinner Light](../assets/screenshots/light/spinner.png)

## Usage

```rust
use egui_shadcn::{Spinner, SpinnerSize};

// Default spinner (24px)
ui.add(Spinner::new());

// Different sizes
ui.add(Spinner::new().size(SpinnerSize::Small));   // 16px
ui.add(Spinner::new().size(SpinnerSize::Medium));  // 24px (default)
ui.add(Spinner::new().size(SpinnerSize::Large));   // 32px
ui.add(Spinner::new().size(SpinnerSize::XLarge));  // 40px
ui.add(Spinner::new().size(SpinnerSize::XXLarge)); // 48px
ui.add(Spinner::new().size(SpinnerSize::XXXLarge)); // 56px
```

## Sizes

| Size | Pixels | Stroke | Use Case |
|------|--------|--------|----------|
| `Small` | 16px | 2.0px | Inline loading, buttons |
| `Medium` | 24px | 2.5px | Default, card loading |
| `Large` | 32px | 3.0px | Section loading |
| `XLarge` | 40px | 3.5px | Page loading |
| `XXLarge` | 48px | 4.0px | Full screen loading |
| `XXXLarge` | 56px | 4.5px | Hero loading states |

## API

```rust
impl Spinner {
    pub fn new() -> Self;
    pub fn size(self, size: SpinnerSize) -> Self;
}

impl SpinnerSize {
    pub const fn pixels(&self) -> f32;
    pub const fn stroke_width(&self) -> f32;
}

// Spinner implements Widget trait
impl Widget for Spinner {
    fn ui(self, ui: &mut Ui) -> Response;
}
```

## Examples

### Loading Button

```rust
let is_loading = true;

ui.horizontal(|ui| {
    if is_loading {
        ui.add(Spinner::new().size(SpinnerSize::Small));
        ui.label("Loading...");
    } else {
        if Button::new("Submit").show(ui).clicked() {
            // Handle submit
        }
    }
});
```

### Loading Card

```rust
Card::new()
    .header("Data Loading")
    .content(|ui| {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.add(Spinner::new().size(SpinnerSize::Large));
                ui.add_space(8.0);
                ui.label("Fetching data...");
            });
        });
    })
    .show(ui);
```

### Full Page Loading

```rust
fn show_loading_screen(ui: &mut Ui) {
    ui.centered_and_justified(|ui| {
        ui.vertical_centered(|ui| {
            ui.add(Spinner::new().size(SpinnerSize::XXLarge));
            ui.add_space(16.0);
            ui.heading("Loading your content");
            ui.label("This may take a moment...");
        });
    });
}
```

### Conditional Loading State

```rust
enum LoadState<T> {
    Loading,
    Loaded(T),
    Error(String),
}

fn show_content(ui: &mut Ui, state: &LoadState<String>) {
    match state {
        LoadState::Loading => {
            ui.horizontal(|ui| {
                ui.add(Spinner::new());
                ui.label("Loading...");
            });
        }
        LoadState::Loaded(content) => {
            ui.label(content);
        }
        LoadState::Error(msg) => {
            ui.colored_label(egui::Color32::RED, msg);
        }
    }
}
```
