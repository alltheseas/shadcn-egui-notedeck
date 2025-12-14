# egui_shadcn Integration Guide for Notedeck Apps

This guide shows you how to integrate the egui_shadcn design system into any notedeck app (notedeck, dave, or your custom app) for instant design consistency.

## Quick Start (3 Steps)

### 1. Add Dependency

Add to your app's `Cargo.toml`:

```toml
[dependencies]
egui_shadcn = { path = "../../egui/crates/egui_shadcn" }
```

### 2. Apply Theme on Startup

In your app's initialization (typically in `eframe::App::new` or similar):

```rust
use egui_shadcn::ShadcnTheme;

impl eframe::App for YourApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme - do this once per frame or cache the theme
        let theme = if self.dark_mode {
            ShadcnTheme::dark()
        } else {
            ShadcnTheme::light()
        };
        theme.apply(ctx);

        // Now build your UI - all widgets will use shadcn styling automatically
        egui::CentralPanel::default().show(ctx, |ui| {
            // Your UI code here
        });
    }
}
```

### 3. Use Components

Import and use shadcn components:

```rust
use egui_shadcn::{Badge, BadgeVariant, Card, Alert, AlertVariant};

// In your UI code:
ui.add(Badge::new("New").variant(BadgeVariant::Destructive));

Card::new(ui)
    .header(|ui| {
        card_title(ui, "User Profile");
    })
    .content(|ui| {
        ui.label("Your content here");
    })
    .show();
```

## Available Components

### Phase 2: Core Components
- **Badge**: Labels and tags with 4 variants (Default, Secondary, Destructive, Outline)
- **Avatar**: Circular user avatars with automatic initials extraction
- **Card**: Structured containers with header/content/footer
- **Alert**: Notification boxes (Default, Destructive)
- **Skeleton**: Loading state placeholders
- **Kbd**: Keyboard shortcut displays

### Phase 3: Form Components
- **shadcn_input**: Single-line text input with placeholder
- **shadcn_textarea**: Multi-line text area
- **form_label**: Styled form labels
- **form_helper**: Helper text for forms

### Phase 4: Navigation & Layout
- **Separator**: Horizontal/vertical dividers
- **Tabs**: Stateful tabbed interface

## Design Tokens

### Colors
The theme uses notedeck's purple palette by default:
- **Primary**: `#CC43C5` (notedeck purple)
- **Accent**: `#8256DD` (purple alt)
- **Destructive**: `#C7375A` (red)
- **Secondary/Muted**: Grays

Access colors directly:
```rust
let theme = ShadcnTheme::light();
let purple = theme.colors.primary;
let red = theme.colors.destructive;
```

### Spacing
Consistent spacing scale:
```rust
let theme = ShadcnTheme::light();
ui.add_space(theme.spacing.sm);  // 8px
ui.add_space(theme.spacing.md);  // 16px
ui.add_space(theme.spacing.lg);  // 24px
```

### Typography
Semantic font sizes:
```rust
let theme = ShadcnTheme::light();
ui.label(egui::RichText::new("Heading").size(theme.typography.h2().size));
ui.label(egui::RichText::new("Body").size(theme.typography.body().size));
ui.label(egui::RichText::new("Small").size(theme.typography.small().size));
```

### Corner Radii
```rust
let theme = ShadcnTheme::light();
egui::Frame::NONE
    .corner_radius(theme.radii.md)
    .show(ui, |ui| { /* ... */ });
```

### Shadows
```rust
let theme = ShadcnTheme::light();
egui::Frame::NONE
    .shadow(theme.shadows.lg)
    .show(ui, |ui| { /* ... */ });
```

## Examples

### Complete App Template

```rust
use eframe::egui;
use egui_shadcn::*;

struct MyApp {
    dark_mode: bool,
    name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            dark_mode: false,
            name: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply shadcn theme
        let theme = if self.dark_mode {
            ShadcnTheme::dark()
        } else {
            ShadcnTheme::light()
        };
        theme.apply(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Notedeck App");

            // Theme toggle
            if ui.button(if self.dark_mode { "🌙 Dark" } else { "☀ Light" }).clicked() {
                self.dark_mode = !self.dark_mode;
            }

            // Use shadcn components
            Card::new(ui)
                .header(|ui| {
                    card_title(ui, "Welcome");
                    card_description(ui, "Get started with shadcn components");
                })
                .content(|ui| {
                    form_label(ui, "Your Name");
                    shadcn_input(ui, &mut self.name, "Enter your name...");

                    ui.add_space(theme.spacing.md);

                    ui.horizontal(|ui| {
                        ui.label("Status:");
                        ui.add(Badge::new("Active").variant(BadgeVariant::Secondary));
                    });
                })
                .footer(|ui| {
                    if ui.button("Submit").clicked() {
                        // Handle submission
                    }
                })
                .show();
        });
    }
}
```

### Form Example

```rust
Card::new(ui)
    .header(|ui| {
        card_title(ui, "Contact Form");
    })
    .content(|ui| {
        form_label(ui, "Email");
        shadcn_input(ui, &mut self.email, "you@example.com");
        form_helper(ui, "We'll never share your email.");

        ui.add_space(theme.spacing.sm);

        form_label(ui, "Message");
        shadcn_textarea(ui, &mut self.message, "Type your message...");
    })
    .footer(|ui| {
        if ui.button("Send").clicked() {
            // Send logic
        }
    })
    .show();
```

### Notification Example

```rust
Alert::new(ui, AlertVariant::Default)
    .title("Success")
    .description("Your changes have been saved.")
    .show();

Alert::new(ui, AlertVariant::Destructive)
    .title("Error")
    .description("Failed to connect to server.")
    .show();
```

### Tabs Example

```rust
Tabs::new(ui, "settings-tabs")
    .tab("general", "General", |ui| {
        ui.label("General settings");
        shadcn_input(ui, &mut self.username, "Username");
    })
    .tab("security", "Security", |ui| {
        ui.label("Security settings");
        shadcn_input(ui, &mut self.password, "Password");
    })
    .show();
```

## Design Consistency Checklist

When building your app, ensure:

- [ ] Apply `ShadcnTheme` in every frame update
- [ ] Use `theme.spacing.*` for all spacing/margins
- [ ] Use `theme.typography.*` for text sizes
- [ ] Use shadcn components instead of raw egui widgets where possible
- [ ] Use `theme.colors.*` for custom colored elements
- [ ] Use `theme.radii.*` for rounded corners
- [ ] Use `theme.shadows.*` for elevation/depth
- [ ] Support both light and dark modes

## Testing Your Integration

Run the showcase example to see all components:
```bash
cargo run -p egui_shadcn --example showcase
```

## Migration from Existing Code

### Before (raw egui):
```rust
ui.label("Hello");
ui.button("Click me");
egui::Frame::none()
    .fill(egui::Color32::GRAY)
    .show(ui, |ui| {
        ui.label("Content");
    });
```

### After (with shadcn):
```rust
let theme = ShadcnTheme::light();

ui.label(egui::RichText::new("Hello").size(theme.typography.body().size));
ui.button("Click me");  // Automatically styled by theme.apply()

Card::new(ui)
    .content(|ui| {
        ui.label("Content");
    })
    .show();
```

## Getting Help

- See `examples/showcase.rs` for comprehensive examples
- Check component source in `src/components/`
- Review theme tokens in `src/theme/`

## Contributing New Components

To add a new component:

1. Create `src/components/your_component.rs`
2. Implement using `egui::Widget` trait or builder pattern
3. Use `theme.colors.*`, `theme.spacing.*`, etc.
4. Export from `src/components/mod.rs`
5. Add example to `examples/showcase.rs`
6. Update this guide with usage examples
