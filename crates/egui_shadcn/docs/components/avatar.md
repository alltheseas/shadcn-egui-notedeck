# Avatar

An image element with a fallback for representing the user.

![Avatar Light](../assets/screenshots/light/avatar.png)

## Usage

```rust
use egui_shadcn::{Avatar, AvatarSize};

// Display initials from name
ui.add(Avatar::new("John Doe")); // Shows "JD"

// Single name
ui.add(Avatar::new("Alice")); // Shows "A"

// Custom initials
ui.add(Avatar::new("AB")); // Shows "AB"

// Different sizes
ui.add(Avatar::new("John Doe").size(AvatarSize::Small));     // 32px
ui.add(Avatar::new("John Doe").size(AvatarSize::Medium));    // 40px (default)
ui.add(Avatar::new("John Doe").size(AvatarSize::Large));     // 48px
ui.add(Avatar::new("John Doe").size(AvatarSize::ExtraLarge)); // 64px
```

## Sizes

| Size | Pixels | Use Case |
|------|--------|----------|
| `Small` | 32px | Compact lists, comments |
| `Medium` | 40px | Default, navigation |
| `Large` | 48px | Profile headers |
| `ExtraLarge` | 64px | Profile pages, hero sections |

## API

```rust
impl Avatar {
    pub fn new(text: impl Into<String>) -> Self;
    pub fn size(self, size: AvatarSize) -> Self;
}

impl AvatarSize {
    pub const fn pixels(&self) -> f32;
}

// Avatar implements Widget trait
impl Widget for Avatar {
    fn ui(self, ui: &mut Ui) -> Response;
}
```

## Initials Extraction

The avatar automatically extracts initials from the provided text:

| Input | Output |
|-------|--------|
| `"John Doe"` | `"JD"` |
| `"Alice"` | `"A"` |
| `"Bob Charlie Delta"` | `"BC"` (first 2 words) |
| `"AB"` | `"AB"` |
| `"john doe"` | `"JD"` (uppercase) |
| `""` | `"?"` |

## Examples

### User List

```rust
fn show_user_list(ui: &mut Ui, users: &[User]) {
    for user in users {
        ui.horizontal(|ui| {
            ui.add(Avatar::new(&user.name).size(AvatarSize::Small));
            ui.vertical(|ui| {
                ui.label(&user.name);
                ui.small(&user.email);
            });
        });
        ui.add_space(8.0);
    }
}
```

### Profile Header

```rust
fn show_profile_header(ui: &mut Ui, user: &User) {
    ui.horizontal(|ui| {
        ui.add(Avatar::new(&user.name).size(AvatarSize::ExtraLarge));
        ui.vertical(|ui| {
            ui.heading(&user.name);
            ui.label(format!("@{}", user.username));
            ui.label(&user.bio);
        });
    });
}
```

### Comment Section

```rust
fn show_comment(ui: &mut Ui, comment: &Comment) {
    ui.horizontal(|ui| {
        ui.add(Avatar::new(&comment.author).size(AvatarSize::Small));
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong(&comment.author);
                ui.weak(&comment.timestamp);
            });
            ui.label(&comment.text);
        });
    });
}
```

### Avatar Group

```rust
fn show_avatar_group(ui: &mut Ui, names: &[&str]) {
    ui.horizontal(|ui| {
        for (i, name) in names.iter().take(3).enumerate() {
            // Overlap avatars slightly
            if i > 0 {
                ui.add_space(-8.0);
            }
            ui.add(Avatar::new(*name).size(AvatarSize::Small));
        }

        if names.len() > 3 {
            ui.add_space(-8.0);
            ui.add(Avatar::new(format!("+{}", names.len() - 3))
                .size(AvatarSize::Small));
        }
    });
}
```
