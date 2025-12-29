# Skeleton

Use to show a placeholder while content is loading.

![Skeleton Light](../assets/screenshots/light/skeleton.png)

## Usage

```rust
use egui_shadcn::Skeleton;
use egui::Vec2;

// Rectangular skeleton
ui.add(Skeleton::new(Vec2::new(200.0, 20.0)));

// Full width skeleton
ui.add(Skeleton::new(Vec2::new(ui.available_width(), 16.0)));

// Circular skeleton (for avatars)
ui.add(Skeleton::circle(40.0));
```

## Features

- **Pulse Animation**: Smooth 2-second pulse cycle
- **Light/Dark Mode**: Adaptive colors for both themes
- **Circular Option**: Perfect for avatar placeholders
- **Custom Sizing**: Any dimensions supported

## API

```rust
impl Skeleton {
    /// Create a rectangular skeleton with the given size
    pub fn new(size: Vec2) -> Self;

    /// Create a circular skeleton (for avatars, etc.)
    pub fn circle(diameter: f32) -> Self;
}

// Skeleton implements Widget trait
impl Widget for Skeleton {
    fn ui(self, ui: &mut Ui) -> Response;
}
```

## Examples

### Card Loading State

```rust
fn show_loading_card(ui: &mut Ui) {
    Card::new()
        .content(|ui| {
            ui.horizontal(|ui| {
                // Avatar placeholder
                ui.add(Skeleton::circle(48.0));

                ui.vertical(|ui| {
                    // Name placeholder
                    ui.add(Skeleton::new(Vec2::new(120.0, 16.0)));
                    ui.add_space(4.0);
                    // Username placeholder
                    ui.add(Skeleton::new(Vec2::new(80.0, 14.0)));
                });
            });

            ui.add_space(12.0);

            // Content lines
            ui.add(Skeleton::new(Vec2::new(ui.available_width(), 14.0)));
            ui.add_space(4.0);
            ui.add(Skeleton::new(Vec2::new(ui.available_width() * 0.8, 14.0)));
            ui.add_space(4.0);
            ui.add(Skeleton::new(Vec2::new(ui.available_width() * 0.6, 14.0)));
        })
        .show(ui);
}
```

### List Loading State

```rust
fn show_loading_list(ui: &mut Ui, count: usize) {
    for _ in 0..count {
        ui.horizontal(|ui| {
            ui.add(Skeleton::circle(32.0));
            ui.add_space(8.0);
            ui.vertical(|ui| {
                ui.add(Skeleton::new(Vec2::new(150.0, 14.0)));
                ui.add_space(4.0);
                ui.add(Skeleton::new(Vec2::new(100.0, 12.0)));
            });
        });
        ui.add_space(8.0);
    }
}
```

### Profile Loading

```rust
fn show_loading_profile(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        // Large avatar
        ui.add(Skeleton::circle(96.0));
        ui.add_space(16.0);

        // Name
        ui.add(Skeleton::new(Vec2::new(180.0, 24.0)));
        ui.add_space(8.0);

        // Bio
        ui.add(Skeleton::new(Vec2::new(240.0, 14.0)));
        ui.add_space(4.0);
        ui.add(Skeleton::new(Vec2::new(200.0, 14.0)));

        ui.add_space(16.0);

        // Stats row
        ui.horizontal(|ui| {
            for _ in 0..3 {
                ui.add(Skeleton::new(Vec2::new(60.0, 32.0)));
                ui.add_space(16.0);
            }
        });
    });
}
```

### Conditional Rendering

```rust
fn show_user_or_skeleton(ui: &mut Ui, user: Option<&User>) {
    ui.horizontal(|ui| {
        if let Some(user) = user {
            ui.add(Avatar::new().name(&user.name).size(AvatarSize::Medium));
            ui.label(&user.name);
        } else {
            ui.add(Skeleton::circle(40.0));
            ui.add(Skeleton::new(Vec2::new(100.0, 16.0)));
        }
    });
}
```
