//! Corner radius system ported from shadcn/ui
//!
//! shadcn/ui uses Tailwind's border-radius scale with a customizable base
//! radius value (--radius CSS variable, default 0.625rem = 10px).
//!
//! Reference: <https://tailwindcss.com/docs/border-radius>
//!
//! ## Tailwind Scale (in px)
//! - xs: 2px - Extra small corners
//! - sm: 4px - Small corners
//! - md: 6px - Medium corners
//! - lg: 8px - Large corners (common for cards)
//! - xl: 12px - Extra large
//! - 2xl: 16px - Very round
//! - 3xl: 24px - Heavily rounded
//! - 4xl: 32px - Maximum roundness
//! - none: 0px - Sharp corners
//! - full: f32::INFINITY - Perfect circles/pills
//!
//! ## Usage in egui
//! In egui, corner radius is specified via `egui::epaint::CornerRadius` which can be:
//! - Uniform: same radius on all corners
//! - Directional: different radius per corner (NW, NE, SW, SE)
//!
//! Note: egui uses u8 for corner radii (0-255 pixels), which is sufficient
//! for UI purposes and saves memory.

use egui::epaint::CornerRadius;

/// Corner radius values from shadcn/Tailwind design system
///
/// These values follow Tailwind's border-radius scale and provide
/// consistent corner styling across all components.
///
/// egui's CornerRadius type supports both uniform and per-corner radii.
/// Values are stored as u8 (0-255 pixels).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadcnRadii {
    /// Extra small: 2px - minimal rounding
    pub xs: u8,
    /// Small: 4px - subtle corners
    pub sm: u8,
    /// Medium: 6px - moderate rounding
    pub md: u8,
    /// Large: 8px - noticeable corners (common default)
    pub lg: u8,
    /// Extra large: 12px - prominent rounding
    pub xl: u8,
    /// 2XL: 16px - very round
    pub xl2: u8,
    /// 3XL: 24px - heavily rounded
    pub xl3: u8,
    /// 4XL: 32px - maximum roundness
    pub xl4: u8,
    /// None: 0px - sharp corners
    pub none: u8,
}

impl Default for ShadcnRadii {
    fn default() -> Self {
        Self::new()
    }
}

impl ShadcnRadii {
    /// Create the standard shadcn corner radius scale
    pub const fn new() -> Self {
        Self {
            xs: 2,
            sm: 4,
            md: 6,
            lg: 8,
            xl: 12,
            xl2: 16,
            xl3: 24,
            xl4: 32,
            none: 0,
        }
    }

    /// Get the base radius value (lg = 8px)
    ///
    /// This matches shadcn's --radius CSS variable default behavior.
    /// Most components use this as their standard corner radius.
    pub const fn base(&self) -> u8 {
        self.lg
    }

    /// Create uniform rounding for all corners
    ///
    /// Example: `radii.uniform_sm()` creates 4px rounding on all corners
    pub const fn uniform(&self, radius: u8) -> CornerRadius {
        CornerRadius {
            nw: radius,
            ne: radius,
            sw: radius,
            se: radius,
        }
    }

    /// Uniform rounding: none (0px)
    pub const fn uniform_none(&self) -> CornerRadius {
        self.uniform(self.none)
    }

    /// Uniform rounding: xs (2px)
    pub const fn uniform_xs(&self) -> CornerRadius {
        self.uniform(self.xs)
    }

    /// Uniform rounding: sm (4px)
    pub const fn uniform_sm(&self) -> CornerRadius {
        self.uniform(self.sm)
    }

    /// Uniform rounding: md (6px)
    pub const fn uniform_md(&self) -> CornerRadius {
        self.uniform(self.md)
    }

    /// Uniform rounding: lg (8px) - most common
    pub const fn uniform_lg(&self) -> CornerRadius {
        self.uniform(self.lg)
    }

    /// Uniform rounding: xl (12px)
    pub const fn uniform_xl(&self) -> CornerRadius {
        self.uniform(self.xl)
    }

    /// Uniform rounding: 2xl (16px)
    pub const fn uniform_2xl(&self) -> CornerRadius {
        self.uniform(self.xl2)
    }

    /// Uniform rounding: 3xl (24px)
    pub const fn uniform_3xl(&self) -> CornerRadius {
        self.uniform(self.xl3)
    }

    /// Uniform rounding: 4xl (32px)
    pub const fn uniform_4xl(&self) -> CornerRadius {
        self.uniform(self.xl4)
    }

    /// Perfect circle/pill shape
    ///
    /// Use this for avatar circles, pill-shaped buttons, etc.
    /// Set to a very large value (255 = max u8) that effectively creates circles.
    pub const fn full(&self) -> CornerRadius {
        self.uniform(255)
    }

    /// Create rounding for only top corners
    ///
    /// Useful for card headers, dialog titles, etc.
    pub const fn top(&self, radius: u8) -> CornerRadius {
        CornerRadius {
            nw: radius,
            ne: radius,
            sw: 0,
            se: 0,
        }
    }

    /// Create rounding for only bottom corners
    ///
    /// Useful for card footers, dialog actions, etc.
    pub const fn bottom(&self, radius: u8) -> CornerRadius {
        CornerRadius {
            nw: 0,
            ne: 0,
            sw: radius,
            se: radius,
        }
    }

    /// Create rounding for only left corners
    pub const fn left(&self, radius: u8) -> CornerRadius {
        CornerRadius {
            nw: radius,
            ne: 0,
            sw: radius,
            se: 0,
        }
    }

    /// Create rounding for only right corners
    pub const fn right(&self, radius: u8) -> CornerRadius {
        CornerRadius {
            nw: 0,
            ne: radius,
            sw: 0,
            se: radius,
        }
    }
}

/// Common semantic radius presets
///
/// These provide named shortcuts for frequently used corner styles,
/// making component code more readable.
impl ShadcnRadii {
    /// Standard button radius (lg = 8px)
    pub const fn button(&self) -> CornerRadius {
        self.uniform_lg()
    }

    /// Standard card radius (lg = 8px)
    pub const fn card(&self) -> CornerRadius {
        self.uniform_lg()
    }

    /// Input field radius (md = 6px)
    pub const fn input(&self) -> CornerRadius {
        self.uniform_md()
    }

    /// Badge/chip radius (full = pill shape)
    pub const fn badge(&self) -> CornerRadius {
        self.full()
    }

    /// Avatar radius (full = circle)
    pub const fn avatar(&self) -> CornerRadius {
        self.full()
    }

    /// Dialog/modal radius (lg = 8px)
    pub const fn dialog(&self) -> CornerRadius {
        self.uniform_lg()
    }

    /// Popover/dropdown radius (md = 6px)
    pub const fn popover(&self) -> CornerRadius {
        self.uniform_md()
    }

    /// Alert/notification radius (lg = 8px)
    pub const fn alert(&self) -> CornerRadius {
        self.uniform_lg()
    }

    /// Progress bar radius (full = pill shape)
    pub const fn progress(&self) -> CornerRadius {
        self.full()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radius_scale() {
        let radii = ShadcnRadii::new();

        // Verify progression of sizes
        assert!(radii.xs < radii.sm);
        assert!(radii.sm < radii.md);
        assert!(radii.md < radii.lg);
        assert!(radii.lg < radii.xl);
        assert!(radii.xl < radii.xl2);
        assert!(radii.xl2 < radii.xl3);
        assert!(radii.xl3 < radii.xl4);
    }

    #[test]
    fn test_base_radius() {
        let radii = ShadcnRadii::new();

        // Base should be lg (8px)
        assert_eq!(radii.base(), 8);
        assert_eq!(radii.base(), radii.lg);
    }

    #[test]
    fn test_uniform_rounding() {
        let radii = ShadcnRadii::new();
        let rounding = radii.uniform_lg();

        // All corners should be equal
        assert_eq!(rounding.nw, 8);
        assert_eq!(rounding.ne, 8);
        assert_eq!(rounding.sw, 8);
        assert_eq!(rounding.se, 8);
    }

    #[test]
    fn test_directional_rounding() {
        let radii = ShadcnRadii::new();

        // Top rounding
        let top = radii.top(8);
        assert_eq!(top.nw, 8);
        assert_eq!(top.ne, 8);
        assert_eq!(top.sw, 0);
        assert_eq!(top.se, 0);

        // Bottom rounding
        let bottom = radii.bottom(8);
        assert_eq!(bottom.nw, 0);
        assert_eq!(bottom.ne, 0);
        assert_eq!(bottom.sw, 8);
        assert_eq!(bottom.se, 8);
    }

    #[test]
    fn test_semantic_helpers() {
        let radii = ShadcnRadii::new();

        // Card and button should both use lg
        assert_eq!(radii.card(), radii.uniform_lg());
        assert_eq!(radii.button(), radii.uniform_lg());

        // Input should use md
        assert_eq!(radii.input(), radii.uniform_md());
    }

    #[test]
    fn test_full_rounding() {
        let radii = ShadcnRadii::new();
        let full = radii.full();

        // Full should create very large radius for circles (255 = max u8)
        assert!(full.nw > 100);
        assert_eq!(full.nw, full.ne);
        assert_eq!(full.ne, full.sw);
        assert_eq!(full.sw, full.se);
    }
}
