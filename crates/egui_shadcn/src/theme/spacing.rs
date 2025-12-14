//! Spacing system ported from shadcn/ui
//!
//! shadcn/ui follows Tailwind CSS's spacing scale, which uses a base unit
//! of 0.25rem (4px). This module adapts that system for egui.
//!
//! Reference: <https://tailwindcss.com/docs/customizing-spacing>
//!
//! The spacing scale provides consistent values for:
//! - Margins between elements
//! - Padding inside containers
//! - Gaps in layouts (flex, grid)
//! - Component dimensions
//!
//! ## Tailwind Scale (in rem/px)
//! - 0.5 = 0.125rem = 2px
//! - 1 = 0.25rem = 4px
//! - 2 = 0.5rem = 8px
//! - 3 = 0.75rem = 12px
//! - 4 = 1rem = 16px
//! - 5 = 1.25rem = 20px
//! - 6 = 1.5rem = 24px
//! - 8 = 2rem = 32px
//! - 10 = 2.5rem = 40px
//! - 12 = 3rem = 48px
//! - 16 = 4rem = 64px

use egui::Vec2;

/// Spacing constants from shadcn/Tailwind design system
///
/// These values are in pixels and follow the Tailwind spacing scale.
/// Use these for consistent spacing across all components.
///
/// The scale progresses in a way that feels natural and provides
/// enough granularity for most UI needs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadcnSpacing {
    /// Extra extra small: 2px - for very tight spacing
    pub xxs: f32,
    /// Extra small: 4px - minimal spacing
    pub xs: f32,
    /// Small: 8px - tight spacing between related elements
    pub sm: f32,
    /// Medium-small: 12px - comfortable spacing
    pub md_sm: f32,
    /// Medium: 16px - standard spacing (Tailwind's base '4')
    pub md: f32,
    /// Medium-large: 20px - comfortable separation
    pub md_lg: f32,
    /// Large: 24px - clear visual separation
    pub lg: f32,
    /// Extra large: 32px - significant spacing
    pub xl: f32,
    /// 2XL: 40px - major section spacing
    pub xl2: f32,
    /// 3XL: 48px - large section gaps
    pub xl3: f32,
    /// 4XL: 64px - very large spacing
    pub xl4: f32,
}

impl Default for ShadcnSpacing {
    fn default() -> Self {
        Self::new()
    }
}

impl ShadcnSpacing {
    /// Create the standard shadcn spacing scale
    pub const fn new() -> Self {
        Self {
            xxs: 2.0,   // 0.5
            xs: 4.0,    // 1
            sm: 8.0,    // 2
            md_sm: 12.0, // 3
            md: 16.0,   // 4 (base)
            md_lg: 20.0, // 5
            lg: 24.0,   // 6
            xl: 32.0,   // 8
            xl2: 40.0,  // 10
            xl3: 48.0,  // 12
            xl4: 64.0,  // 16
        }
    }

    /// Get spacing value by Tailwind scale number
    ///
    /// Maps Tailwind spacing numbers to pixel values:
    /// - `spacing(1)` → 4px
    /// - `spacing(2)` → 8px
    /// - `spacing(4)` → 16px (base)
    /// - etc.
    pub fn spacing(&self, scale: u8) -> f32 {
        match scale {
            0 => 0.0,
            1 => self.xs,       // 4px
            2 => self.sm,       // 8px
            3 => self.md_sm,    // 12px
            4 => self.md,       // 16px
            5 => self.md_lg,    // 20px
            6 => self.lg,       // 24px
            8 => self.xl,       // 32px
            10 => self.xl2,     // 40px
            12 => self.xl3,     // 48px
            16 => self.xl4,     // 64px
            // For other values, calculate proportionally from base
            n => (n as f32) * 4.0,
        }
    }

    /// Create a Vec2 with uniform spacing
    pub fn vec2(&self, scale: u8) -> Vec2 {
        let s = self.spacing(scale);
        Vec2::new(s, s)
    }

    /// Create a Vec2 with different x and y spacing
    pub fn vec2_xy(&self, x_scale: u8, y_scale: u8) -> Vec2 {
        Vec2::new(self.spacing(x_scale), self.spacing(y_scale))
    }
}

/// Commonly used spacing presets for egui layouts
///
/// These provide semantic names for common spacing scenarios,
/// making code more readable and maintainable.
impl ShadcnSpacing {
    /// Spacing for items within a tight group (8px)
    pub const fn item_spacing(&self) -> f32 {
        self.sm
    }

    /// Spacing between form elements (12px)
    pub const fn form_spacing(&self) -> f32 {
        self.md_sm
    }

    /// Standard button padding (8px horizontal, 16px vertical)
    pub fn button_padding(&self) -> Vec2 {
        Vec2::new(self.md, self.sm)
    }

    /// Standard window padding (16px)
    pub const fn window_padding(&self) -> f32 {
        self.md
    }

    /// Spacing between sections (24px)
    pub const fn section_spacing(&self) -> f32 {
        self.lg
    }

    /// Large spacing for page-level separation (48px)
    pub const fn page_spacing(&self) -> f32 {
        self.xl3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_scale() {
        let spacing = ShadcnSpacing::new();

        // Verify base Tailwind scale
        assert_eq!(spacing.spacing(1), 4.0);
        assert_eq!(spacing.spacing(2), 8.0);
        assert_eq!(spacing.spacing(4), 16.0);
        assert_eq!(spacing.spacing(8), 32.0);
    }

    #[test]
    fn test_semantic_spacing() {
        let spacing = ShadcnSpacing::new();

        // Item spacing should be small/tight
        assert_eq!(spacing.item_spacing(), 8.0);

        // Form spacing should be comfortable
        assert_eq!(spacing.form_spacing(), 12.0);

        // Section spacing should be clear
        assert_eq!(spacing.section_spacing(), 24.0);
    }

    #[test]
    fn test_vec2_creation() {
        let spacing = ShadcnSpacing::new();

        let uniform = spacing.vec2(4);
        assert_eq!(uniform, Vec2::new(16.0, 16.0));

        let custom = spacing.vec2_xy(2, 4);
        assert_eq!(custom, Vec2::new(8.0, 16.0));
    }
}
