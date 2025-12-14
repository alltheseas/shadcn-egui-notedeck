# Design Consistency Checklist for Notedeck Apps

Use this checklist when building or reviewing notedeck apps to ensure design consistency across the ecosystem.

## Theme Integration

- [ ] **Apply theme in every frame**: Call `NotedeckTheme::apply(ctx, dark_mode)` at the start of `update()`
- [ ] **Support both light and dark modes**: Implement dark mode toggle
- [ ] **Use theme getter for tokens**: Access `let theme = NotedeckTheme::get(dark_mode)` when needed

## Spacing & Layout

- [ ] **Use theme spacing constants**: Replace magic numbers with `theme.spacing.*`
  - `xs` (4px) for tight spacing
  - `sm` (8px) for compact layouts
  - `md` (16px) for standard spacing
  - `lg` (24px) for section breaks
  - `xl` (32px) for major divisions

- [ ] **Consistent margins**: Use same spacing scale for margins and padding
- [ ] **Add space between sections**: Call `ui.add_space(theme.spacing.md)` between major UI sections

## Typography

- [ ] **Use semantic font sizes**: Apply `theme.typography.*` for text
  - `h1()` through `h4()` for headings
  - `body()` for main content
  - `small()` for secondary text
  - `lead()` for introductory paragraphs

- [ ] **Consistent text colors**: Use `theme.colors.foreground` for main text, `theme.colors.muted_foreground` for secondary

## Colors

- [ ] **Use semantic color tokens**: Reference `theme.colors.*` instead of hard-coded colors
  - `primary` for brand/main actions
  - `secondary` for secondary actions
  - `destructive` for errors/warnings
  - `muted` for subtle backgrounds
  - `accent` for highlights

- [ ] **Proper foreground/background pairs**: Always pair background colors with their corresponding foreground
  - `primary` with `primary_foreground`
  - `destructive` with `destructive_foreground`
  - etc.

## Components

- [ ] **Prefer shadcn components over raw egui**: Use `Card`, `Badge`, `Alert` etc. instead of raw `Frame` and `Label`
- [ ] **Use component variants**: Apply appropriate variants (Default, Secondary, Destructive, Outline)
- [ ] **Consistent form fields**: Use `patterns::form_field()` for standard form inputs
- [ ] **Standard headers**: Use `patterns::header()` for page/section headers

## Visual Elements

- [ ] **Rounded corners**: Use `theme.radii.*` for corner radius
  - `sm` (4px) for small elements
  - `md` (8px) for standard elements
  - `lg` (12px) for cards/containers
  - `xl` (16px) for large containers

- [ ] **Shadows for elevation**: Use `theme.shadows.*` for depth
  - `sm` for subtle elevation
  - `md` for standard cards
  - `lg` for modals/popovers
  - `xl` for floating elements

## Patterns & Conventions

- [ ] **User cards**: Use `patterns::user_card()` for displaying user profiles
- [ ] **Settings rows**: Use `patterns::setting_row()` for settings UI
- [ ] **Messages**: Use `patterns::error_message()` and `patterns::success_message()` for notifications
- [ ] **Forms**: Use `Card` with header/content/footer for form layout

## Accessibility

- [ ] **Sufficient contrast**: Ensure text meets WCAG contrast requirements (automatically handled by theme foreground/background pairs)
- [ ] **Keyboard navigation**: Ensure all interactive elements are keyboard accessible
- [ ] **Clear labels**: Provide descriptive labels for all form inputs
- [ ] **Helper text**: Add helper text for complex form fields

## Responsive Design

- [ ] **Flexible layouts**: Use `ui.horizontal_wrapped()` for responsive horizontal layouts
- [ ] **Scroll areas**: Wrap long content in `egui::ScrollArea`
- [ ] **Minimum sizes**: Set sensible `desired_width` and `desired_height` for components

## Code Quality

- [ ] **No magic numbers**: All spacing, sizes, and colors come from theme tokens
- [ ] **Reuse patterns**: Don't reinvent common UI patterns - use the pattern helpers
- [ ] **Consistent naming**: Follow egui/Rust naming conventions
- [ ] **Documentation**: Add doc comments to custom components

## Testing

- [ ] **Test in both modes**: Verify UI works in both light and dark modes
- [ ] **Test different content lengths**: Ensure layout handles short and long text
- [ ] **Test window resizing**: Verify responsive behavior
- [ ] **Visual regression**: Compare against other notedeck apps for consistency

## Before Merging

- [ ] **Run showcase**: Check that components look consistent with `cargo run -p egui_shadcn --example showcase`
- [ ] **Run quickstart**: Verify integration example still works
- [ ] **Review with designer**: Get feedback on visual consistency (if applicable)
- [ ] **Update docs**: Document any new patterns or components

## Common Mistakes to Avoid

- ❌ Hard-coding colors like `Color32::RED`
- ❌ Using pixel values like `ui.add_space(16.0)` instead of `theme.spacing.md`
- ❌ Forgetting to apply theme in `update()`
- ❌ Mixing shadcn components with unstyled egui widgets
- ❌ Using different spacing scales across features
- ❌ Not testing dark mode
- ❌ Creating custom Frame/Card instead of using shadcn Card component

## Quick Reference

### Most Common Patterns

```rust
// Apply theme
NotedeckTheme::apply(ctx, dark_mode);
let theme = NotedeckTheme::get(dark_mode);

// Spacing
ui.add_space(theme.spacing.md);

// Typography
ui.label(egui::RichText::new("Heading").size(theme.typography.h2().size));

// Card
Card::new(ui)
    .header(|ui| { card_title(ui, "Title"); })
    .content(|ui| { /* content */ })
    .show();

// Form
patterns::form_field(ui, "Label", &mut text, "placeholder", None);

// Badges
ui.add(Badge::new("Status").variant(BadgeVariant::Secondary));
```

## Resources

- See `INTEGRATION.md` for integration guide
- See `examples/showcase.rs` for all components
- See `examples/quickstart.rs` for minimal example
- See `src/notedeck.rs` for pattern helpers
