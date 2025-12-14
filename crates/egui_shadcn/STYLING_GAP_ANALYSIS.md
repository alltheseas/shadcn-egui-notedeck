# Styling Gap Analysis: egui_shadcn vs shadcn/ui

## Problem Statement

Current feedback: "looks like egui controls" - the showcase still uses default egui widget styling instead of shadcn's distinctive visual design.

## Root Cause

We've implemented:
✅ Theme color tokens (primary, secondary, etc.)
✅ Spacing/typography/radii/shadows
✅ Component structure (Badge, Card, etc.)

We're **missing**:
❌ Custom widget styling (buttons, inputs still use egui defaults)
❌ Proper focus states with rings
❌ shadcn-specific borders and hover effects
❌ Visual polish that makes shadcn distinctive

## Visual Comparison

### shadcn/ui Characteristics

**Buttons:**
- Rounded corners with specific radius
- Clear hover/focus states with ring indicators
- Variant-specific styling (default, outline, ghost, destructive)
- Consistent padding and spacing
- Smooth transitions
- Specific background/foreground color pairs

**Inputs:**
- Subtle border (usually 1px solid)
- Focus ring indicator
- Rounded corners
- Specific padding
- Background color different from page background
- Placeholder text styling

**Cards:**
- Subtle border
- Light shadow for elevation
- Rounded corners
- Consistent padding
- Background different from page

### Current egui_shadcn Issues

**Buttons (Default egui):**
- ❌ Gray background (not using theme.colors.primary)
- ❌ No rounded corners (egui default is sharp)
- ❌ No visual hierarchy between variants
- ❌ Generic hover state

**Inputs (Default egui):**
- ❌ Looks like basic text field
- ❌ No border styling
- ❌ No focus ring
- ❌ Minimal visual polish

**Cards:**
- ⚠️ Better, but still using egui Frame defaults
- ⚠️ Need more consistent padding
- ⚠️ Shadow could be more prominent

## What We Need to Do

### Phase 1: Fix Built-in Widget Styling

We need to override egui's **widget visuals** in our theme application:

1. **Button Styling**
   ```rust
   // In ShadcnTheme::apply()
   visuals.widgets.inactive.bg_fill = theme.colors.primary;
   visuals.widgets.inactive.fg_stroke = Stroke::new(0.0, theme.colors.primary_foreground);
   visuals.widgets.inactive.corner_radius = theme.radii.md;

   visuals.widgets.hovered.bg_fill = lighten(theme.colors.primary, 0.1);
   visuals.widgets.hovered.corner_radius = theme.radii.md;

   visuals.widgets.active.bg_fill = darken(theme.colors.primary, 0.1);
   visuals.widgets.active.corner_radius = theme.radii.md;
   ```

2. **Input Styling**
   ```rust
   visuals.widgets.noninteractive.bg_fill = theme.colors.background;
   visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, theme.colors.input);
   visuals.widgets.noninteractive.corner_radius = theme.radii.md;
   ```

3. **Focus Ring**
   ```rust
   visuals.selection.stroke = Stroke::new(2.0, theme.colors.ring);
   ```

### Phase 2: Create shadcn-Specific Widgets

For widgets that need more control than egui provides, create custom implementations:

1. **ShadcnButton** - Custom button widget
   ```rust
   pub struct ShadcnButton {
       text: String,
       variant: ButtonVariant, // Default, Outline, Ghost, Destructive
   }

   impl Widget for ShadcnButton {
       fn ui(self, ui: &mut Ui) -> Response {
           // Custom rendering with exact shadcn styling
       }
   }
   ```

2. **ShadcnInput** - Custom input widget (beyond the current helper)
   ```rust
   pub struct ShadcnInput {
       text: String,
       placeholder: Option<String>,
   }
   ```

### Phase 3: Polish Existing Components

Update existing components to match shadcn more closely:

1. **Badge** - ✅ Already good, but verify hover states
2. **Card** - Add more consistent padding, verify shadow
3. **Alert** - Add icon support, verify colors
4. **Tabs** - Improve visual separation, underline for active tab

## Action Items

### Immediate (High Impact)

- [ ] Override `visuals.widgets` in `ShadcnTheme::apply()` to style buttons
- [ ] Override input field styling in `visuals.widgets.noninteractive`
- [ ] Add proper focus ring to `visuals.selection`
- [ ] Ensure corner radii are applied to all widgets

### Short Term

- [ ] Create `ShadcnButton` component with variants
- [ ] Improve `shadcn_input` to have border and focus ring
- [ ] Add hover effects to interactive components
- [ ] Verify spacing consistency

### Medium Term

- [ ] Create comprehensive visual regression tests
- [ ] Side-by-side comparison screenshots
- [ ] Polish all components to match shadcn exactly

## Testing Plan

1. **Visual Comparison**
   - Take screenshots of each component
   - Compare with shadcn/ui documentation
   - Identify specific differences

2. **Interactive Testing**
   - Test hover states
   - Test focus states
   - Test all variants

3. **Cross-theme Testing**
   - Verify light mode matches shadcn light
   - Verify dark mode matches shadcn dark

## Success Criteria

When we can say:
✅ "This looks like shadcn/ui, not default egui"
✅ Buttons have clear visual hierarchy
✅ Inputs have proper borders and focus rings
✅ All components match shadcn visual design
✅ Hover/focus states are polished
✅ Consistent spacing and padding throughout

## References

- shadcn Button: https://ui.shadcn.com/docs/components/button
- shadcn Input: https://ui.shadcn.com/docs/components/input
- shadcn Card: https://ui.shadcn.com/docs/components/card
- egui Visuals: https://docs.rs/egui/latest/egui/struct.Visuals.html
