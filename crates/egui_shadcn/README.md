# egui_shadcn

A complete port of [shadcn/ui](https://ui.shadcn.com) design system and components to [egui](https://github.com/emilk/egui), built for [notedeck](https://github.com/damus-io/notedeck) app development.

## Motivation

[notedeck](https://github.com/damus-io/notedeck) is a cross-platform nostr client built with egui that targets desktop, Android, and iOS. This library was created to bring modern, accessible UI components to notedeck while maintaining the performance benefits of egui's immediate mode rendering. By porting shadcn/ui's battle-tested design patterns to Rust, notedeck apps get a professional look and feel with proper mobile touch targets and accessibility support.

## Overview

egui_shadcn brings [shadcn/ui](https://ui.shadcn.com)'s beautiful, accessible component library to the Rust/[egui](https://github.com/emilk/egui) ecosystem with notedeck's purple branding. It provides:

- **Design System**: Colors, spacing, typography, corner radii, and shadows from shadcn/ui
- **48+ Components**: Pre-built UI components adapted for egui's immediate mode paradigm
- **Apple HIG Compliance**: 44px minimum touch targets for mobile compatibility
- **WCAG AA Accessibility**: Proper contrast ratios and keyboard navigation
- **Light/Dark Mode**: Full theming support with automatic mode detection

## Quick Start

### Running the Showcase

```bash
# Full component showcase with all 48+ components
cargo run -p egui_shadcn --example showcase

# Minimal quickstart example
cargo run -p egui_shadcn --example quickstart
```

### Basic Integration

```rust
use egui_shadcn::NotedeckTheme;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme (enables all shadcn styling)
        NotedeckTheme::apply(ctx, self.dark_mode);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Your UI here - all widgets automatically styled!
        });
    }
}
```

## Component Library

### Core Components
| Component | Description |
|-----------|-------------|
| **Button** | 6 variants (Default, Secondary, Outline, Ghost, Destructive, Link), 4 sizes |
| **Badge** | Status labels with 4 variants |
| **Avatar** | Profile pictures with auto-initials, 4 sizes |
| **Card** | Content containers with header/content/footer sections |
| **Alert** | Informational messages (Default, Destructive) |
| **Skeleton** | Animated loading placeholders |
| **Kbd** | Keyboard shortcut display |
| **Separator** | Horizontal/vertical dividers |
| **AspectRatio** | Constrained aspect ratio containers |

### Form Components
| Component | Description |
|-----------|-------------|
| **Input** | Single-line text input with validation states |
| **Textarea** | Multi-line text input |
| **Checkbox** | Square checkboxes with indeterminate state |
| **Switch** | Toggle switches |
| **Slider** | Range input sliders |
| **Progress** | Progress bars |
| **RadioGroup** | Radio button groups |
| **Select** | Dropdown selection |
| **Combobox** | Searchable dropdown with filtering |
| **Toggle** | Toggle buttons |
| **ToggleGroup** | Grouped toggle buttons |
| **Label** | Form labels |
| **Field** | Form field wrapper with label/description/error |
| **Form** | Form validation patterns |

### Navigation & Layout
| Component | Description |
|-----------|-------------|
| **Tabs** | Tabbed interfaces |
| **Sidebar** | Collapsible navigation sidebar with sections and menus |
| **Menubar** | Application menu bars |
| **Breadcrumb** | Navigation breadcrumbs |
| **NavigationMenu** | Navigation menu patterns |
| **Pagination** | Page navigation controls |
| **ResizablePanels** | Draggable split panels |

### Overlays & Dialogs
| Component | Description |
|-----------|-------------|
| **Dialog** | Modal dialogs |
| **AlertDialog** | Confirmation dialogs |
| **Drawer** | Side panel overlays |
| **Sheet** | Bottom/side sheets |
| **Popover** | Floating content panels |
| **Tooltip** | Hover tooltips |
| **HoverCard** | Rich hover previews |
| **ContextMenu** | Right-click context menus |
| **DropdownMenu** | Dropdown menus |
| **Command** | Command palette (Cmd+K style) |
| **Toast** | Notification toasts |

### Data Display
| Component | Description |
|-----------|-------------|
| **Table** | Data tables with sorting and striping |
| **Calendar** | Date picker calendar with month/year navigation |
| **DatePicker** | Date input with calendar popup |
| **Carousel** | Image/content carousels |
| **Chart** | Bar and area charts |
| **Accordion** | Collapsible content sections |
| **Collapsible** | Simple collapsible containers |
| **Spinner** | Loading spinners |

## Design System

### Colors
- **Primary**: Notedeck purple (`#CC43C5`)
- **Secondary**: Muted backgrounds
- **Destructive**: Error/danger states
- **Accent**: Hover/focus states
- Full semantic color palette for both light and dark modes

### Typography
- Consistent font scale from `xs` (12px) to `4xl` (36px)
- Heading styles (h1-h6)
- Body, small, and muted text variants

### Spacing
- 4px base unit
- Scale: `xs` (4px), `sm` (8px), `md` (16px), `lg` (24px), `xl` (32px), `2xl` (48px)

### Corner Radii
- `sm` (4px), `md` (6px), `lg` (8px), `xl` (12px), `2xl` (16px), `full` (9999px)

### Shadows
- `xs`, `sm`, `md`, `lg`, `xl`, `2xl` elevation levels

## Testing

### Run the Showcase
```bash
cargo run -p egui_shadcn --example showcase
```

The showcase demonstrates all components organized by category:
- Scroll through to see each component
- Toggle dark mode with the theme switch
- Interactive components respond to clicks/hovers
- Form components show validation states

### Run Tests
```bash
cargo test -p egui_shadcn
```

### Visual Verification Checklist
When testing, verify:
- [ ] Components render correctly in both light and dark modes
- [ ] Hover states are visible and provide feedback
- [ ] Focus states show clear indicators
- [ ] Disabled states are visually muted
- [ ] Touch targets are at least 44x44 pixels
- [ ] Text has sufficient contrast (4.5:1 minimum)

## Documentation

| Document | Description |
|----------|-------------|
| [INTEGRATION.md](INTEGRATION.md) | Complete integration guide |
| [DESIGN_CHECKLIST.md](DESIGN_CHECKLIST.md) | Design review guidelines |
| [EGUI_CONFLICTS.md](EGUI_CONFLICTS.md) | Known egui conflicts and solutions |
| [COMPONENT_AUDIT.md](COMPONENT_AUDIT.md) | Component implementation status |

## Development

### Building
```bash
cargo build -p egui_shadcn
```

### Adding New Components
1. Create component file in `src/components/`
2. Export from `src/components/mod.rs`
3. Add demo section to `examples/showcase.rs`
4. Document in this README

### Design Principles

**From shadcn/ui:**
- Composability: Components work together seamlessly
- Accessibility: Keyboard navigation and focus management
- Customization: Easy to style and extend

**For egui:**
- Immediate mode compatible
- Minimal allocations, efficient rendering
- Type-safe builder patterns

**For Mobile (Apple HIG):**
- 44px minimum touch targets
- Clear visual feedback
- Sufficient contrast ratios

## License

MIT OR Apache-2.0

## References

- [notedeck](https://github.com/damus-io/notedeck) - Cross-platform nostr client (motivation for this library)
- [shadcn/ui](https://ui.shadcn.com) - Original design system
- [egui](https://github.com/emilk/egui) - Target GUI framework
- [Apple HIG](https://developer.apple.com/design/human-interface-guidelines) - Touch target guidelines
- [WCAG](https://www.w3.org/WAI/WCAG21/quickref/) - Accessibility guidelines
