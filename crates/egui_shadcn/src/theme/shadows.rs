//! Shadow/elevation system ported from shadcn/ui
//!
//! shadcn/ui uses Tailwind's box-shadow scale to create depth and elevation.
//! This module adapts those shadows to egui's Shadow type.
//!
//! Reference: <https://tailwindcss.com/docs/box-shadow>
//!
//! ## Tailwind Shadow Scale
//! Shadows are defined with offset, blur, spread, and opacity values.
//! Tailwind provides several levels from subtle (xs) to dramatic (2xl).
//!
//! ## Conversion Notes
//! Tailwind CSS shadows often use multiple shadow layers for depth.
//! egui's Shadow type is simpler (single shadow per element), so we
//! approximate the Tailwind shadows by using the most prominent layer.
//!
//! Shadow values are:
//! - offset: [x, y] movement in pixels (i8)
//! - blur: blur radius in pixels (u8)
//! - spread: expansion in all directions (u8)
//! - color: shadow color with alpha for opacity

use egui::epaint::Shadow;
use egui::Color32;

/// Shadow/elevation values from shadcn/Tailwind design system
///
/// These values follow Tailwind's box-shadow scale and provide
/// consistent elevation styling across components.
///
/// Each shadow level represents increasing elevation/depth.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadcnShadows {
    /// No shadow (flat on surface)
    pub none: Shadow,
    /// Extra extra small: Very subtle shadow (1px blur)
    pub xs2: Shadow,
    /// Extra small: Subtle shadow (2px blur)
    pub xs: Shadow,
    /// Small: Light shadow (3px blur, used for most cards)
    pub sm: Shadow,
    /// Medium: Moderate shadow (6px blur)
    pub md: Shadow,
    /// Large: Noticeable shadow (15px blur)
    pub lg: Shadow,
    /// Extra large: Strong shadow (25px blur)
    pub xl: Shadow,
    /// 2XL: Very dramatic shadow (50px blur)
    pub xl2: Shadow,
}

impl Default for ShadcnShadows {
    fn default() -> Self {
        Self::new()
    }
}

impl ShadcnShadows {
    /// Create the standard shadcn shadow scale
    ///
    /// These values approximate Tailwind's shadows adapted for egui's
    /// simpler single-shadow-per-element model.
    pub const fn new() -> Self {
        Self::light()
    }

    /// Create light mode shadow scale (dark shadows)
    pub const fn light() -> Self {
        // Shadow color: black with varying opacity
        // Tailwind uses rgb(0 0 0 / 0.1) for most shadows
        const SHADOW_COLOR: Color32 = Color32::from_black_alpha(25); // ~10% opacity
        const SHADOW_COLOR_DARK: Color32 = Color32::from_black_alpha(64); // ~25% for 2xl

        Self {
            // No shadow
            none: Shadow {
                offset: [0, 0],
                blur: 0,
                spread: 0,
                color: Color32::TRANSPARENT,
            },

            // shadow-2xs: 0 1px rgb(0 0 0 / 0.05)
            xs2: Shadow {
                offset: [0, 1],
                blur: 0,
                spread: 0,
                color: Color32::from_black_alpha(13), // ~5% opacity
            },

            // shadow-xs: 0 1px 2px 0 rgb(0 0 0 / 0.05)
            xs: Shadow {
                offset: [0, 1],
                blur: 2,
                spread: 0,
                color: Color32::from_black_alpha(13), // ~5% opacity
            },

            // shadow-sm: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)
            // Approximation: use primary layer (first shadow)
            sm: Shadow {
                offset: [0, 1],
                blur: 3,
                spread: 0,
                color: SHADOW_COLOR,
            },

            // shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)
            // Approximation: use primary layer
            md: Shadow {
                offset: [0, 4],
                blur: 6,
                spread: 0,
                color: SHADOW_COLOR,
            },

            // shadow-lg: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)
            // Approximation: use primary layer
            lg: Shadow {
                offset: [0, 10],
                blur: 15,
                spread: 0,
                color: SHADOW_COLOR,
            },

            // shadow-xl: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)
            // Approximation: use primary layer
            xl: Shadow {
                offset: [0, 20],
                blur: 25,
                spread: 0,
                color: SHADOW_COLOR,
            },

            // shadow-2xl: 0 25px 50px -12px rgb(0 0 0 / 0.25)
            xl2: Shadow {
                offset: [0, 25],
                blur: 50,
                spread: 0,
                color: SHADOW_COLOR_DARK,
            },
        }
    }

    /// Create dark mode shadow scale (light/white shadows for glow effect)
    ///
    /// In dark mode, shadows use white/light colors with low opacity to create
    /// a subtle elevation effect that's visible against dark backgrounds.
    pub const fn dark() -> Self {
        // Dark mode shadows: white with varying opacity for glow/lift effect
        // Color32::from_rgba_premultiplied(r, g, b, a) where all channels are 255 for white
        const SHADOW_COLOR: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 40); // ~15% opacity white
        const SHADOW_COLOR_BRIGHT: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 60); // ~23% for 2xl

        Self {
            // No shadow
            none: Shadow {
                offset: [0, 0],
                blur: 0,
                spread: 0,
                color: Color32::TRANSPARENT,
            },

            // shadow-2xs: 0 1px rgb(255 255 255 / 0.15)
            xs2: Shadow {
                offset: [0, 1],
                blur: 0,
                spread: 0,
                color: Color32::from_rgba_premultiplied(255, 255, 255, 38), // ~15% opacity
            },

            // shadow-xs: 0 1px 2px 0 rgb(255 255 255 / 0.15)
            xs: Shadow {
                offset: [0, 1],
                blur: 2,
                spread: 0,
                color: Color32::from_rgba_premultiplied(255, 255, 255, 38),
            },

            // shadow-sm
            sm: Shadow {
                offset: [0, 1],
                blur: 3,
                spread: 0,
                color: SHADOW_COLOR,
            },

            // shadow-md
            md: Shadow {
                offset: [0, 4],
                blur: 6,
                spread: 0,
                color: SHADOW_COLOR,
            },

            // shadow-lg
            lg: Shadow {
                offset: [0, 10],
                blur: 15,
                spread: 0,
                color: SHADOW_COLOR,
            },

            // shadow-xl
            xl: Shadow {
                offset: [0, 20],
                blur: 25,
                spread: 0,
                color: SHADOW_COLOR,
            },

            // shadow-2xl
            xl2: Shadow {
                offset: [0, 25],
                blur: 50,
                spread: 0,
                color: SHADOW_COLOR_BRIGHT,
            },
        }
    }
}

/// Semantic shadow presets
///
/// These provide named shortcuts for common elevation scenarios,
/// making component code more intuitive.
impl ShadcnShadows {
    /// Card elevation (sm = subtle 3px blur)
    ///
    /// Standard shadow for cards, panels, and containers.
    pub const fn card(&self) -> Shadow {
        self.sm
    }

    /// Button elevation when raised (sm = subtle)
    pub const fn button(&self) -> Shadow {
        self.sm
    }

    /// Dialog/modal elevation (xl = strong shadow)
    ///
    /// Dialogs float above other content and need strong shadows.
    pub const fn dialog(&self) -> Shadow {
        self.xl
    }

    /// Popover/dropdown elevation (md = moderate shadow)
    ///
    /// Dropdowns and popovers need clear elevation to float above content.
    pub const fn popover(&self) -> Shadow {
        self.md
    }

    /// Tooltip elevation (sm = subtle shadow)
    pub const fn tooltip(&self) -> Shadow {
        self.sm
    }

    /// Floating action button (lg = noticeable shadow)
    pub const fn fab(&self) -> Shadow {
        self.lg
    }

    /// No elevation (flat on surface)
    pub const fn flat(&self) -> Shadow {
        self.none
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow_progression() {
        let shadows = ShadcnShadows::new();

        // Verify blur increases with shadow size
        assert!(shadows.none.blur == 0);
        assert!(shadows.xs2.blur < shadows.xs.blur);
        assert!(shadows.xs.blur < shadows.sm.blur);
        assert!(shadows.sm.blur < shadows.md.blur);
        assert!(shadows.md.blur < shadows.lg.blur);
        assert!(shadows.lg.blur < shadows.xl.blur);
        assert!(shadows.xl.blur < shadows.xl2.blur);
    }

    #[test]
    fn test_shadow_offsets() {
        let shadows = ShadcnShadows::new();

        // Larger shadows should have larger y-offsets (drop shadow effect)
        assert_eq!(shadows.none.offset[1], 0);
        assert!(shadows.xs2.offset[1] > 0);
        assert!(shadows.md.offset[1] > shadows.sm.offset[1]);
        assert!(shadows.lg.offset[1] > shadows.md.offset[1]);
        assert!(shadows.xl.offset[1] > shadows.lg.offset[1]);
        assert!(shadows.xl2.offset[1] > shadows.xl.offset[1]);
    }

    #[test]
    fn test_none_shadow() {
        let shadows = ShadcnShadows::new();

        // None shadow should be completely transparent
        assert_eq!(shadows.none.offset, [0, 0]);
        assert_eq!(shadows.none.blur, 0);
        assert_eq!(shadows.none.spread, 0);
        assert_eq!(shadows.none.color, Color32::TRANSPARENT);
    }

    #[test]
    fn test_semantic_shadows() {
        let shadows = ShadcnShadows::new();

        // Card and button use subtle shadows
        assert_eq!(shadows.card(), shadows.sm);
        assert_eq!(shadows.button(), shadows.sm);

        // Dialog uses strong shadow for maximum elevation
        assert_eq!(shadows.dialog(), shadows.xl);

        // Popover uses moderate shadow
        assert_eq!(shadows.popover(), shadows.md);
    }

    #[test]
    fn test_shadow_colors() {
        let shadows = ShadcnShadows::new();

        // Most shadows should use semi-transparent black
        assert!(shadows.sm.color.a() > 0);
        assert!(shadows.sm.color.a() < 255);

        // 2xl uses darker shadow
        assert!(shadows.xl2.color.a() > shadows.sm.color.a());
    }
}
