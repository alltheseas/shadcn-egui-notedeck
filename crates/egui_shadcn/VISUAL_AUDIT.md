# Visual Audit: egui_shadcn vs shadcn/ui

This document tracks visual differences between our implementation and shadcn/ui.

## Methodology

1. Compare screenshots of egui_shadcn showcase with shadcn/ui examples
2. Document specific visual differences (colors, spacing, borders, shadows, etc.)
3. Prioritize fixes by visual impact
4. Iterate until pixel-perfect

## Components to Compare

### ✅ Already Implemented

#### Button
**shadcn Reference**: https://ui.shadcn.com/docs/components/button

**Visual Checklist**:
- [ ] Default variant: solid purple background, white text
- [ ] Secondary variant: gray background
- [ ] Outline variant: transparent bg, 1px border
- [ ] Ghost variant: transparent, hover shows bg
- [ ] Destructive variant: red background
- [ ] Link variant: underline on hover
- [ ] Padding matches (sm: 16x4, default: 24x8, lg: 32x16)
- [ ] Border radius matches (8px for buttons)
- [ ] Focus ring: 2px offset ring on focus
- [ ] Hover state darkens by ~10%
- [ ] Pressed state darkens by ~20%
- [ ] Font size appropriate for each size
- [ ] Disabled state: reduced opacity

**Observed Differences**:
- ✅ Default button: Purple background (hsl(262, 83%, 58%)) matches well
- ✅ Secondary button: Gray variant present
- ✅ Outline button: Border-based styling present
- ✅ Ghost button: Transparent with hover state
- ✅ Destructive button: Red/pink background present
- ✅ Link button: Text-based styling present
- ✅ Button sizes: Small, Default, Large, Icon variants all working
- ⚠️ Border radius appears correct but needs verification
- ⚠️ Focus ring implementation needs checking - not visible in static screenshots
- ⚠️ Padding looks close but needs pixel-perfect comparison
- ⚠️ Font weight may need adjustment (shadcn uses font-medium)

**Priority Fixes**:
1. [MEDIUM] Verify focus ring implementation matches shadcn (2px ring with offset)
2. [LOW] Fine-tune padding if needed after pixel comparison
3. [LOW] Ensure font-medium weight is used

---

#### Checkbox
**shadcn Reference**: https://ui.shadcn.com/docs/components/checkbox

**Visual Checklist**:
- [x] Size: 16x16px ✅ Appears correct
- [x] Border: 1px solid border ✅ Present
- [x] Border radius: small (4px) ✅ Looks correct
- [ ] Checked state: purple background - NEEDS VERIFICATION
- [ ] Checkmark: white, properly sized and positioned - NEEDS VERIFICATION
- [ ] Indeterminate: horizontal line - NOT VISIBLE IN SCREENSHOTS
- [ ] Hover: focus ring appears - NOT TESTABLE IN STATIC SCREENSHOTS
- [ ] Disabled: reduced opacity - NOT SHOWN IN SCREENSHOTS

**Observed Differences**:
- ✅ Checkbox component is visible in Phase 3 section
- ⚠️ Checkbox is shown but partially cut off in screenshot (only label "Accept terms" visible)
- ⚠️ Cannot verify checked/unchecked states from current screenshots
- ⚠️ Need to scroll or take additional screenshots to see full checkbox demo

**Priority Fixes**:
1. [HIGH] Need better screenshots showing checkbox states
2. [MEDIUM] Verify purple background on checked state
3. [MEDIUM] Verify checkmark styling and positioning

---

#### Switch
**shadcn Reference**: https://ui.shadcn.com/docs/components/switch

**Visual Checklist**:
- [x] Track width: ~44px ✅ Looks correct
- [x] Track height: ~24px ✅ Looks correct
- [x] Fully rounded pill shape ✅ Correct
- [ ] Off state: gray background - PARTIALLY VISIBLE
- [x] On state: purple background ✅ Visible in screenshot
- [x] Thumb: white circle, ~20px diameter ✅ Looks good
- [x] Thumb padding: 2px from edges ✅ Appears correct
- [ ] Smooth animation (not applicable in current impl)
- [ ] Focus ring on hover - NOT TESTABLE IN STATIC SCREENSHOTS

**Observed Differences**:
- ✅ Switch component visible in Phase 3 section
- ✅ Label "Enable notifications" present
- ✅ Purple background when enabled looks correct
- ✅ White circular thumb visible
- ✅ Pill-shaped track with full rounding
- ⚠️ Cannot see OFF state clearly in screenshots
- ⚠️ Need to verify exact dimensions match 44x24px

**Priority Fixes**:
1. [LOW] Verify exact pixel dimensions (44x24)
2. [LOW] Confirm off-state gray color matches shadcn

---

#### Slider
**shadcn Reference**: https://ui.shadcn.com/docs/components/slider

**Visual Checklist**:
- [x] Track height: 4px ✅ Looks very thin, appears correct
- [x] Track background: light gray ✅ Visible
- [x] Filled track: purple ✅ Strong purple color visible
- [x] Thumb: white circle with border ✅ Clearly visible white circle
- [x] Thumb size: ~20px diameter ✅ Looks appropriately sized
- [ ] Focus ring on hover/drag - NOT TESTABLE IN STATIC SCREENSHOTS
- [ ] Smooth dragging - NOT TESTABLE IN STATIC SCREENSHOTS

**Observed Differences**:
- ✅ Slider visible in screenshot with label "Slider:"
- ✅ Value: 50 displayed
- ✅ Purple filled portion on left side of track
- ✅ White circular thumb positioned at ~50% mark
- ✅ Thin gray track background visible
- ✅ Overall appearance matches shadcn/ui well
- ⚠️ Track appears to use rounded ends (good!)
- ⚠️ Thumb appears to have subtle border/shadow (matches shadcn)

**Priority Fixes**:
- ✅ NO MAJOR ISSUES - Slider looks very close to shadcn/ui!

---

#### Progress
**shadcn Reference**: https://ui.shadcn.com/docs/components/progress

**Visual Checklist**:
- [x] Height: 8px default ✅ Looks correct
- [x] Fully rounded (pill shape) ✅ Perfect pill shape
- [x] Background: light gray ✅ Visible
- [x] Fill: purple ✅ Strong purple fill at 65%
- [x] Smooth rounded corners on fill ✅ Rounded ends visible

**Observed Differences**:
- ✅ Progress bar visible with label "Progress:"
- ✅ Shows filled bar with +/- controls (65% filled)
- ✅ Purple fill color matches primary color
- ✅ Pill-shaped with fully rounded ends
- ✅ Light gray background track
- ✅ Height appears to be ~8px (correct)
- ✅ Fill portion has rounded ends (proper clipping)

**Priority Fixes**:
- ✅ NO MAJOR ISSUES - Progress looks excellent!

---

#### Badge
**shadcn Reference**: https://ui.shadcn.com/docs/components/badge

**Visual Checklist**:
- [x] Pill shaped (fully rounded) ✅ Visible in screenshot
- [x] Compact padding (12x4) ✅ Looks compact
- [x] Small text size ✅ Appropriately small
- [x] Default: purple bg, white text ✅ Visible
- [x] Secondary: gray bg ✅ Visible
- [x] Destructive: red/pink bg ✅ Visible
- [ ] Outline: transparent bg, 1px border - NOT CLEARLY VISIBLE

**Observed Differences**:
- ✅ Badges section visible in Phase 2 Demo
- ✅ "Default" badge: Purple background, white text
- ✅ "Secondary" badge: Gray background
- ✅ "Destructive" badge: Red/pink background
- ✅ All badges have pill-shaped (fully rounded) corners
- ✅ Compact padding looks correct
- ✅ Small text size appropriate
- ⚠️ "Outline" badge appears dark - need to verify if it has transparent background with border

**Priority Fixes**:
1. [LOW] Verify outline variant has transparent background + border (not solid dark bg)

---

#### Card
**shadcn Reference**: https://ui.shadcn.com/docs/components/card

**Visual Checklist**:
- [x] Border: 1px solid ✅ Visible border
- [x] Border radius: 8px ✅ Rounded corners visible
- [x] Background: white (light) / dark (dark mode) ✅ Both modes working
- [x] Shadow: subtle card shadow ✅ Appears to have subtle elevation
- [x] Header padding ✅ Looks good
- [x] Content padding ✅ Looks good
- [ ] Footer padding - NO FOOTER IN CURRENT EXAMPLES

**Observed Differences**:
- ✅ Cards visible in screenshots showing "Simple Card" and "User Profile"
- ✅ Card has visible border (subtle gray)
- ✅ Rounded corners at ~8px
- ✅ Light background in light mode (white)
- ✅ Dark background in dark mode
- ✅ Good padding around content
- ✅ Card titles are bold/prominent
- ✅ Card descriptions are muted/secondary color
- ✅ Subtle shadow/elevation visible

**Priority Fixes**:
- ✅ NO MAJOR ISSUES - Cards look great!

---

#### Avatar
**shadcn Reference**: https://ui.shadcn.com/docs/components/avatar

**Visual Checklist**:
- [x] Perfect circle ✅ Circular avatars visible
- [x] Initials extraction correct ✅ "JD", "AS", "EM", "TS" visible
- [x] Background color ✅ Varied colors (purple, gray, etc)
- [x] Text color (white) ✅ White text visible
- [x] Sizes: sm, md, lg, xl ✅ Multiple sizes shown (S, M, L, XL labels)

**Observed Differences**:
- ✅ Avatars section clearly visible in Phase 2 Demo
- ✅ Perfect circular shape
- ✅ Multiple size variants shown: S, M, L, XL
- ✅ Initials properly extracted and displayed
- ✅ White text color on colored backgrounds
- ✅ Different background colors for variety
- ✅ Clean, modern appearance

**Priority Fixes**:
- ✅ NO MAJOR ISSUES - Avatars look perfect!

---

#### Alert
**shadcn Reference**: https://ui.shadcn.com/docs/components/alert

**Visual Checklist**:
- [x] Border: 1px solid ✅ Visible
- [x] Border radius: 8px ✅ Visible
- [x] Padding: appropriate ✅ Looks good
- [x] Default/Info variant ✅ Visible with blue-ish tint
- [x] Destructive/Error variant ✅ Visible with red/pink border
- [x] Title typography ✅ Bold/prominent
- [x] Description typography ✅ Secondary/muted

**Observed Differences**:
- ✅ Alert components visible in screenshots
- ✅ "Informational" alert with "This is a default alert with helpful information" text
- ✅ "Error" alert with red/pink border and "Something went wrong. Please try again." text
- ✅ Appropriate padding and spacing
- ✅ Border visible and properly colored
- ✅ Rounded corners
- ✅ Title in bold, description in regular weight

**Priority Fixes**:
- ✅ NO MAJOR ISSUES - Alerts look good!

---

#### Skeleton (Loading States)
**shadcn Reference**: https://ui.shadcn.com/docs/components/skeleton

**Visual Checklist**:
- [x] Rounded rectangles ✅ Visible
- [x] Gray background ✅ Appropriate color
- [x] Various sizes ✅ Circle and rectangles shown

**Observed Differences**:
- ✅ Skeleton loading states visible
- ✅ Circular skeleton (avatar placeholder)
- ✅ Rectangular skeletons of varying widths
- ✅ Gray color appropriate for loading state
- ⚠️ No animation visible in static screenshots (pulse/shimmer not testable)

**Priority Fixes**:
- ✅ Looks good! (Animation would be nice but not critical for egui)

---

#### Kbd (Keyboard Shortcuts)
**shadcn Reference**: https://ui.shadcn.com/docs/components/kbd

**Visual Checklist**:
- [x] Border: 1px solid ✅ Visible
- [x] Border radius: small ✅ Slightly rounded
- [x] Monospace font ✅ "Ctrl" and "K" shown
- [x] Compact padding ✅ Looks appropriate
- [x] Subtle background ✅ Visible

**Observed Differences**:
- ✅ Kbd component visible showing "Ctrl" + "K"
- ✅ Border around each key
- ✅ Monospace-style appearance
- ✅ Compact size
- ✅ Label "Press Ctrl + K to search"
- ✅ Plus sign separator between keys

**Priority Fixes**:
- ✅ Looks good!

---

#### Input & Textarea
**shadcn Reference**:
- https://ui.shadcn.com/docs/components/input
- https://ui.shadcn.com/docs/components/textarea

**Visual Checklist**:
- [x] Border: visible ✅ Dark border in screenshots
- [x] Border radius: small ✅ Slightly rounded
- [x] Padding: appropriate ✅ Looks good
- [x] Placeholder text ✅ "Enter your email..." and "Type your message..." visible
- [x] Background color ✅ Dark in dark mode
- [ ] Focus ring: 2-3px with offset - NOT TESTABLE IN STATIC SCREENSHOTS

**Observed Differences**:
- ✅ Input field visible with label "Email"
- ✅ Textarea visible with label "Message"
- ✅ Both have appropriate placeholders
- ✅ Border visible (dark in dark theme)
- ✅ Rounded corners
- ✅ Good padding
- ✅ Help text below: "We'll never share your email."
- ⚠️ **IMPORTANT**: Current implementation appears to have borders, which is good!
- ⚠️ Need to verify focus ring on interactive testing

**Priority Fixes**:
1. [MEDIUM] Verify focus ring implementation (2-3px ring with offset)
2. [LOW] Ensure border colors match shadcn exactly

---

#### Separator
**shadcn Reference**: https://ui.shadcn.com/docs/components/separator

**Visual Checklist**:
- [x] Horizontal line ✅ Visible
- [x] Subtle gray color ✅ Appropriate
- [x] Thin line (1px) ✅ Looks correct

**Observed Differences**:
- ✅ Separator visible in Phase 4 section
- ✅ Label: "Content above" / "Content below"
- ✅ Thin horizontal gray line
- ✅ Subtle and unobtrusive
- ✅ Matches shadcn style

**Priority Fixes**:
- ✅ Looks perfect!

---

#### Tabs
**shadcn Reference**: https://ui.shadcn.com/docs/components/tabs

**Visual Checklist**:
- [x] Tab triggers in horizontal row ✅ Visible
- [x] Active tab styling ✅ "Overview" appears active (white bg)
- [x] Inactive tab styling ✅ "Settings" and "About" appear inactive (gray)
- [x] Border radius on triggers ✅ Rounded corners visible
- [x] Content area below ✅ "Overview content goes here" visible
- [ ] Underline indicator - NOT CLEARLY VISIBLE (may use background instead)

**Observed Differences**:
- ✅ Tabs component visible with "Overview", "Settings", "About" triggers
- ✅ "Overview" tab appears active with white/light background
- ✅ Inactive tabs have gray/muted appearance
- ✅ Tab content displays: "Overview content goes here / This is the first tab"
- ✅ Rounded corners on tab triggers
- ⚠️ **DIFFERENT**: Appears to use background color instead of underline for active state
  - shadcn/ui typically uses an underline indicator
  - Current implementation uses filled background (more like "pills" style)

**Priority Fixes**:
1. [MEDIUM] Consider switching to underline-based active indicator (more aligned with shadcn default)
2. [LOW] Alternative: Keep current style but document as "pills" variant

---

## Summary of Findings

### Components Looking Great ✅
1. **Slider** - Nearly pixel-perfect
2. **Progress** - Excellent implementation
3. **Card** - Very close to shadcn
4. **Avatar** - Perfect circular avatars with proper sizing
5. **Alert** - Good styling and colors
6. **Skeleton** - Appropriate placeholder styling
7. **Kbd** - Keyboard shortcut display looks good
8. **Separator** - Simple and correct

### Components Needing Minor Improvements ⚠️
1. **Button** - Focus ring verification needed
2. **Checkbox** - Need better screenshots to verify states
3. **Switch** - Mostly good, verify dimensions
4. **Badge** - Verify outline variant transparency
5. **Input/Textarea** - Focus ring needs verification
6. **Tabs** - Consider underline indicator instead of background fill

### Priority Fix List

#### HIGH Priority
1. Get better screenshots of Checkbox in various states (checked, unchecked, indeterminate)

#### MEDIUM Priority
1. Verify and fix focus ring implementation across all interactive components (Button, Input, Textarea, Checkbox, Switch, Slider)
2. Fix Tabs to use underline indicator instead of background fill for active state
3. Verify Input/Textarea border colors match shadcn exactly

#### LOW Priority
1. Verify Badge outline variant has transparent background + border
2. Fine-tune button padding if needed
3. Confirm Switch dimensions exactly match 44x24px
4. Ensure button font-medium weight is used

---

## Next Steps

1. **USER ACTION NEEDED**: Please take screenshots of the showcase app showing:
   - All button variants
   - Checkbox (checked, unchecked, indeterminate)
   - Switch (on, off)
   - Slider at different positions
   - Progress bar
   - Badge variants
   - Card examples
   - Avatar examples

2. Save screenshots in this directory or share them

3. I'll analyze and create specific fix tasks

## Visual Comparison Template

For each component difference found:

```
Component: [Name]
Element: [Specific part, e.g., "Button Default variant"]
Issue: [What's wrong]
Expected: [From shadcn/ui]
Actual: [From our implementation]
Fix: [Specific code change needed]
Priority: [High/Medium/Low]
```
