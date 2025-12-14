# Visual Fixes Priority List

Generated from visual comparison between egui_shadcn screenshots and shadcn/ui website.

## Excellent Components (No fixes needed) ✅

These components match shadcn/ui very closely:

1. **Slider** - Nearly pixel-perfect, track and thumb sizing correct
2. **Progress** - Excellent pill shape, correct height and colors
3. **Card** - Border, padding, shadow all correct
4. **Avatar** - Perfect circles, sizing variants working well
5. **Alert** - Good border and color variants
6. **Skeleton** - Appropriate loading state styling
7. **Kbd** - Keyboard shortcut display correct
8. **Separator** - Simple horizontal line matches

## HIGH Priority Fixes

### 1. Checkbox - Missing Visual States
**File**: `src/components/checkbox.rs`

**Issue**: Cannot verify checkbox appearance from current screenshots (cut off)

**Action Needed**:
- Get better screenshot showing checkbox in all states
- Verify checked state has purple background
- Verify checkmark is white and properly sized
- Verify unchecked has border only
- Test indeterminate state (horizontal line)

**Priority**: HIGH - Core form component

---

## MEDIUM Priority Fixes

### 2. Focus Ring Implementation
**Files**: Multiple components (Button, Input, Textarea, Checkbox, Switch, Slider)

**Issue**: Focus rings not visible in static screenshots, implementation needs verification

**Expected from shadcn/ui**:
- 2-3px ring width
- Ring offset of 2px from element
- Uses ring color from theme (semi-transparent primary)
- Visible only on keyboard focus (focus-visible)

**Action Items**:
- Add focus ring to Button component
- Add focus ring to Input component
- Add focus ring to Textarea component
- Add focus ring to Checkbox component
- Add focus ring to Switch component
- Verify Slider thumb focus ring

**Files to modify**:
- `src/components/button.rs`
- `src/components/input.rs`
- `src/components/textarea.rs`
- `src/components/checkbox.rs`
- `src/components/switch.rs`

**Priority**: MEDIUM - Accessibility and UX critical

---

### 3. Tabs Active Indicator Style
**File**: `src/components/tabs.rs`

**Issue**: Current implementation uses filled background for active tab. shadcn/ui default uses underline indicator.

**Current Style**: Active tab has white/light background (pills style)
**shadcn Style**: Active tab has underline below text

**Options**:
1. Switch to underline-based indicator (more aligned with shadcn)
2. Keep current style and document as "pills" variant

**Recommendation**: Switch to underline for default, offer pills as variant

**Priority**: MEDIUM - Visual consistency with shadcn

---

### 4. Input/Textarea Border Colors
**Files**: `src/components/input.rs`, `src/components/textarea.rs`

**Issue**: Border colors appear correct but need exact verification

**Action**:
- Compare border colors in light mode
- Compare border colors in dark mode
- Ensure hover state border color matches
- Verify disabled state border opacity

**Priority**: MEDIUM - Visual polish

---

## LOW Priority Fixes

### 5. Badge Outline Variant Transparency
**File**: `src/components/badge.rs`

**Issue**: Outline variant appears to have dark background instead of transparent

**Expected**: Transparent background with 1px border
**Current**: Appears to have solid dark background

**Action**: Verify in code and fix if needed

**Priority**: LOW - Minor variant styling

---

### 6. Button Padding Fine-tuning
**File**: `src/components/button.rs`

**Issue**: Padding looks close but may need pixel-perfect adjustment

**shadcn sizes**:
- sm: padding-x: 12px, padding-y: 4px
- default: padding-x: 16px, padding-y: 8px
- lg: padding-x: 24px, padding-y: 12px

**Action**: Measure and compare exact padding values

**Priority**: LOW - Already very close

---

### 7. Button Font Weight
**File**: `src/components/button.rs`

**Issue**: shadcn uses font-medium (500 weight)

**Action**: Verify button text uses medium weight

**Priority**: LOW - Typography detail

---

### 8. Switch Dimensions Verification
**File**: `src/components/switch.rs`

**Issue**: Switch looks good but dimensions should be exactly verified

**Expected**: Track 44x24px, thumb 20px with 2px padding

**Action**: Measure implementation against spec

**Priority**: LOW - Already looks very close

---

## Implementation Plan

### Phase 1: High Priority (Do First)
1. Get better Checkbox screenshots
2. Verify and document Checkbox visual states

### Phase 2: Medium Priority (Focus Ring Sprint)
1. Add focus ring helper to theme system
2. Implement focus rings on Button
3. Implement focus rings on Input/Textarea
4. Implement focus rings on Checkbox/Switch
5. Verify Slider thumb focus ring
6. Fix Tabs to use underline indicator
7. Verify Input/Textarea border colors

### Phase 3: Low Priority (Polish)
1. Fix Badge outline variant if needed
2. Fine-tune button padding
3. Verify button font weight
4. Verify switch dimensions

---

## Testing Checklist

For each fix:
- [ ] Visual comparison with shadcn/ui website
- [ ] Test in light mode
- [ ] Test in dark mode
- [ ] Interactive testing (hover, focus, click)
- [ ] Update VISUAL_AUDIT.md
- [ ] Screenshot for documentation

---

## Notes

- Most components are already very close to shadcn/ui
- Focus rings are the biggest missing piece for accessibility
- Tabs underline vs background is a stylistic choice
- Overall implementation quality is high
