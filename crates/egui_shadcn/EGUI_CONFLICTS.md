# egui-shadcn Conflicts and Solutions

This document catalogs known conflicts between egui's built-in visual system and shadcn-style theming, along with their solutions. Use this as a reference when building new components or debugging visual issues.

## How egui Visuals Work

egui uses a hierarchical visual system where global `Visuals` settings affect all widgets. Understanding this is crucial for avoiding conflicts.

### Widget Visual States

egui widgets use different visual states:
- `noninteractive` - Labels, disabled elements, decorative items
- `inactive` - Default state for interactive widgets (buttons, inputs)
- `hovered` - When mouse is over the widget
- `active` - When widget is being clicked/dragged
- `open` - For dropdown menus, popups in open state

Each state has:
- `bg_fill` - Main background color
- `weak_bg_fill` - Secondary background (scrollbar handles, some widget parts)
- `bg_stroke` - Border stroke
- `fg_stroke` - Foreground stroke (text, icons)
- `corner_radius` - Border radius
- `expansion` - Visual expansion on interaction

### Key Global Settings

- `extreme_bg_color` - Used for scrollbar track backgrounds, some popups
- `override_text_color` - Forces all text to use this color
- `dark_mode` - Affects some internal egui rendering decisions

---

## Known Conflicts

### 1. Scrollbar Invisibility in Light Mode

**Symptoms:** Vertical/horizontal scrollbars not visible in light mode, but work in dark mode.

**Root Cause:**
- Floating scrollbars (egui default) use `fg_stroke.color` for the handle when `scroll.foreground_color = true`
- We set `inactive.fg_stroke.color = primary_foreground` (WHITE for button text)
- We set `extreme_bg_color = card` (WHITE in light mode)
- Result: White handle on white background = invisible

**Solution:**
```rust
// In theme apply():

// Set extreme_bg_color to visible gray for scrollbar track
visuals.extreme_bg_color = if is_light_mode {
    egui::Color32::from_rgb(240, 240, 240) // Light gray
} else {
    egui::Color32::from_rgb(40, 40, 40) // Dark gray
};

// Use bg_fill (primary color) instead of fg_stroke for handles
style.spacing.scroll.foreground_color = false;
```

**Files:** `src/theme/mod.rs:180-190`

---

### 2. Button Text Color Conflicts

**Symptoms:** Button text appears in wrong color, or all text becomes same color as buttons.

**Root Cause:**
- `override_text_color` applies to ALL text globally
- `fg_stroke.color` is used for both button text AND other widget foregrounds
- Setting one affects the other

**Solution:**
- Use `override_text_color` for base text color
- For buttons needing different text colors, use `RichText::color()` explicitly
- Custom components should check theme and apply colors directly

**Pattern:**
```rust
// For custom text colors in themed components:
let text_color = if is_primary {
    theme.colors.primary_foreground
} else {
    theme.colors.foreground
};
ui.label(RichText::new(text).color(text_color));
```

---

### 3. Focus Ring Rendering Outside Bounds

**Symptoms:** Focus rings get clipped at window/panel edges, appear partially visible.

**Root Cause:**
- Focus rings drawn outside widget bounds get clipped by parent containers
- egui's clip rect system clips rendering outside allocated space

**Solution:**
- Draw focus rings INSIDE the widget bounds using `rect.shrink()`
- Use `StrokeKind::Inside` for strokes

```rust
pub fn draw_focus_ring(&self, painter: &egui::Painter, rect: egui::Rect, ...) {
    let ring_rect = rect.shrink(1.0); // Draw inside
    painter.rect_stroke(
        ring_rect,
        corner_radius,
        egui::Stroke::new(ring_width, self.colors.ring),
        egui::StrokeKind::Inside, // Inside stroke
    );
}
```

**Files:** `src/theme/mod.rs:105-129`

---

### 4. Modal/Overlay Click-Through Issues

**Symptoms:**
- Clicking backdrop doesn't close modal
- Modal closes immediately on open
- Clicks pass through to widgets underneath

**Root Cause:**
- `Sense::click()` on backdrop captures ALL clicks including the open button
- Using `Area` with `Sense::click()` can interfere with other areas
- Same-frame detection: click that opens modal also triggers backdrop close

**Solution:**
- Use `layer_painter` for visual-only backdrops (no interaction capture)
- Track open time and add delay before allowing close
- Check `pointer.primary_released()` not `any_click()`

```rust
// Visual-only backdrop
let backdrop_layer = egui::LayerId::new(egui::Order::Middle, id.with("backdrop"));
ui.ctx().layer_painter(backdrop_layer).rect_filled(
    screen_rect,
    0.0,
    Color32::from_black_alpha(128),
);

// Time-based close detection
let opened_time: f64 = ui.ctx().data(|d| d.get_temp(opened_time_id).unwrap_or(current_time));
let time_open = current_time - opened_time;
if time_open > 0.1 { // 100ms delay
    // Check for outside clicks
}
```

**Files:** `src/components/sheet.rs`, `src/components/drawer.rs`, `src/components/dialog.rs`

---

### 5. Z-Order/Layer Conflicts

**Symptoms:**
- Popups appear behind other elements
- Multiple overlays compete for top position
- Tooltips appear behind modals

**Root Cause:**
- egui uses `Order` enum for z-ordering
- Multiple components using same `Order::Foreground` can conflict
- Areas created later appear on top of earlier ones

**Solution:**
- Use appropriate order levels:
  - `Order::Background` - Behind everything
  - `Order::Middle` - Backdrops, overlays
  - `Order::Foreground` - Modals, sheets, drawers
  - `Order::Tooltip` - Tooltips (highest)
- Use unique IDs for each layer

```rust
// Backdrop at middle level
egui::LayerId::new(egui::Order::Middle, id.with("backdrop"))

// Content at foreground level
egui::Area::new(id.with("panel"))
    .order(egui::Order::Foreground)
```

---

### 6. Input Field Border Visibility

**Symptoms:** Input fields appear borderless or with wrong border color.

**Root Cause:**
- egui's TextEdit uses `noninteractive.bg_stroke` for unfocused state
- Our theme might set this to transparent or same as background
- Focus state uses different stroke settings

**Solution:**
- Explicitly set `noninteractive.bg_stroke` to visible border color
- For custom inputs, draw border manually

```rust
visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, self.colors.border);
```

---

### 7. Checkbox/Switch Visual State Conflicts

**Symptoms:** Checkboxes show wrong colors for checked/unchecked states.

**Root Cause:**
- egui checkboxes use `inactive.bg_fill` for background
- We set this to `primary` (purple) for buttons
- Unchecked checkbox appears filled with primary color

**Solution:**
- Custom checkbox/switch components that manage their own colors
- Draw checked/unchecked states explicitly using theme colors

```rust
let bg_color = if *checked {
    theme.colors.primary
} else {
    theme.colors.background // or muted for slight fill
};
```

**Files:** `src/components/checkbox.rs`, `src/components/switch.rs`

---

### 8. Spacing Inheritance Issues

**Symptoms:**
- Widgets too close together or too far apart
- Inconsistent padding inside containers
- Layout breaks at certain window sizes

**Root Cause:**
- `style.spacing.item_spacing` affects ALL layouts
- `style.spacing.button_padding` affects all buttons
- Nested containers compound spacing

**Solution:**
- Use explicit spacing in custom components: `ui.add_space()`
- Reset spacing locally when needed: `ui.spacing_mut().item_spacing = ...`
- Use `Frame::inner_margin()` for container padding

---

### 9. Animation/Transition Flickering

**Symptoms:** Hover states flicker, animations stutter, boolean transitions jump.

**Root Cause:**
- `animate_bool_responsive()` needs consistent ID
- Rapid state changes can cause animation conflicts
- Multiple animations on same ID interfere

**Solution:**
- Use unique, stable IDs for animations
- Avoid changing animation targets mid-animation
- Use `ctx.request_repaint()` for smooth animations

```rust
let hover_t = ui.ctx().animate_bool_responsive(
    id.with("hover"), // Unique, stable ID
    response.hovered()
);
```

---

### 10. Shadow/Elevation Conflicts

**Symptoms:** Shadows don't appear, appear in wrong color, or get clipped.

**Root Cause:**
- `visuals.window_shadow` and `popup_shadow` are global
- Shadows rendered outside widget bounds get clipped
- Dark mode shadows need different offset/color than light mode

**Solution:**
- Set appropriate shadow for each mode in theme
- Use `Shadow` with mode-appropriate settings
- Consider painting shadows manually for custom components

```rust
visuals.window_shadow = self.shadows.card();
visuals.popup_shadow = self.shadows.popover();
```

---

## Testing Checklist for New Components

When adding a new component, test for these conflicts:

- [ ] **Light mode visibility** - All elements visible on white background
- [ ] **Dark mode visibility** - All elements visible on dark background
- [ ] **Scrollbar interaction** - Scrollbars visible and functional if component scrolls
- [ ] **Focus states** - Focus rings visible, not clipped
- [ ] **Hover states** - Hover colors appropriate, no flickering
- [ ] **Text colors** - All text readable in both modes
- [ ] **Borders** - Borders visible when expected
- [ ] **Shadows** - Shadows render correctly, not clipped
- [ ] **Z-order** - Component appears at correct layer
- [ ] **Nested in other components** - Works inside Card, Dialog, etc.
- [ ] **Near screen edges** - Nothing clipped or cut off
- [ ] **With long content** - Handles overflow gracefully

## Debugging Tips

1. **Toggle dark mode** - Many conflicts only appear in one mode
2. **Check widget state colors** - Print `ui.visuals().widgets.inactive` etc.
3. **Temporarily use bright colors** - Replace subtle colors with RED to see what's rendering
4. **Check clip_rect** - Call `ui.clip_rect()` to see rendering bounds
5. **Inspect layer order** - Use different `Order` values to diagnose z-order issues

---

## Component Audit Results

### Well-Designed Components (Self-Managed Colors)

These components manage their own colors explicitly and don't rely on egui's widget state system:

| Component | Approach | Status |
|-----------|----------|--------|
| Button | Explicit colors for all variants via `colors()` method | Safe |
| Checkbox | Explicit fill/stroke, draws checkmark manually | Safe |
| Switch | Explicit track/thumb colors, draws manually | Safe |
| Input/Textarea | Uses TextEdit with `frame(false)`, draws own border | Safe |
| Select | Explicit colors for trigger and popup items | Safe |
| Slider | Explicit track/thumb colors, draws manually | Safe |
| RadioGroup | Explicit circle/dot colors, draws manually | Safe |
| Progress | Explicit track/fill colors | Safe |
| DropdownMenu | Explicit colors throughout | Safe |
| Accordion | Uses Label with explicit RichText color | Safe |
| Toast | Explicit colors, uses Button with TRANSPARENT fill | Safe |
| Sheet | Explicit colors, manually drawn X button | Safe |
| Drawer | Explicit colors, layer_painter for backdrop | Safe |
| Popover | Explicit Frame styling | Safe |
| Tooltip | Applies text color override locally in scope | Safe |
| Tabs | Explicit text colors, TRANSPARENT button fill | Safe |
| Dialog | Uses egui Modal with explicit Frame styling | Safe |

### Components Using egui Widgets (Potential Conflicts)

These components use egui's built-in widgets and may inherit theme visuals:

| Component | Widget Used | Mitigation |
|-----------|-------------|------------|
| Dialog (close btn) | `egui::Button` | Uses TRANSPARENT fill, explicit text color via RichText |
| DropdownMenu (trigger) | `egui::Button` | Only when trigger_text set; inherits theme button style |
| confirm_dialog | `egui::Button` | Uses fill() override; text inherits fg_stroke.color |

### Recommended Pattern

For maximum safety, always use one of these approaches:

1. **Draw everything manually** using `ui.painter()`:
   ```rust
   let (response, painter) = ui.allocate_painter(size, sense);
   painter.rect_filled(rect, radius, explicit_color);
   painter.text(pos, align, text, font, explicit_color);
   ```

2. **Use egui widgets with explicit overrides**:
   ```rust
   egui::Button::new(
       egui::RichText::new("Text").color(explicit_color)
   )
   .fill(explicit_bg_color)
   .stroke(egui::Stroke::NONE)
   ```

3. **Use Frame with explicit styling**:
   ```rust
   egui::Frame::NONE
       .fill(theme.colors.background)
       .stroke(egui::Stroke::new(1.0, theme.colors.border))
       .show(ui, |ui| { ... });
   ```

---

## Contributing

When you discover a new conflict:

1. Document the symptoms clearly
2. Identify the root cause in egui's visual system
3. Provide a tested solution
4. Add to this document with file references
5. Update the testing checklist if needed
