//! Typography system ported from shadcn/ui
//!
//! shadcn/ui uses Tailwind's typography scale for font sizes.
//! This module maps those sizes to egui's TextStyle system.
//!
//! Reference: <https://ui.shadcn.com/docs/components/typography>
//!
//! ## Font Sizes (Tailwind Scale)
//! - xs: 12px - Extra small text
//! - sm: 14px - Small text
//! - base: 16px - Body text (default)
//! - lg: 18px - Large text
//! - xl: 20px - Extra large
//! - 2xl: 24px - Heading 4
//! - 3xl: 30px - Heading 3
//! - 4xl: 36px - Heading 2
//! - 5xl: 48px - Heading 1
//!
//! ## Semantic Mapping
//! - h1: 4xl (36px) with extrabold weight
//! - h2: 3xl (30px) with semibold weight
//! - h3: 2xl (24px) with semibold weight
//! - h4: xl (20px) with semibold weight
//! - body/p: base (16px)
//! - small: sm (14px)
//! - muted: sm (14px) with muted color

use egui::FontId;

/// Typography scale based on shadcn/Tailwind font sizes
///
/// Provides semantic font sizes that match shadcn's design system.
/// All sizes are in pixels and follow Tailwind's typography scale.
#[derive(Debug, Clone, PartialEq)]
pub struct ShadcnTypography {
    /// Font family to use (defaults to egui's proportional font)
    pub font_family: egui::FontFamily,

    /// Monospace font family for code
    pub mono_family: egui::FontFamily,
}

impl Default for ShadcnTypography {
    fn default() -> Self {
        Self::new()
    }
}

impl ShadcnTypography {
    /// Create typography system with default font families
    pub fn new() -> Self {
        Self {
            font_family: egui::FontFamily::Proportional,
            mono_family: egui::FontFamily::Monospace,
        }
    }

    // Font size constants (in pixels, following Tailwind scale)

    /// Extra small: 12px
    pub const XS: f32 = 12.0;

    /// Small: 14px
    pub const SM: f32 = 14.0;

    /// Base/body: 16px (default for most text)
    pub const BASE: f32 = 16.0;

    /// Large: 18px
    pub const LG: f32 = 18.0;

    /// Extra large: 20px
    pub const XL: f32 = 20.0;

    /// 2XL: 24px (h4)
    pub const XL2: f32 = 24.0;

    /// 3XL: 30px (h3)
    pub const XL3: f32 = 30.0;

    /// 4XL: 36px (h2)
    pub const XL4: f32 = 36.0;

    /// 5XL: 48px (h1)
    pub const XL5: f32 = 48.0;

    // Semantic font getters

    /// Heading 1: 48px (5xl) - largest heading
    pub fn h1(&self) -> FontId {
        FontId::new(Self::XL5, self.font_family.clone())
    }

    /// Heading 2: 36px (4xl)
    pub fn h2(&self) -> FontId {
        FontId::new(Self::XL4, self.font_family.clone())
    }

    /// Heading 3: 30px (3xl)
    pub fn h3(&self) -> FontId {
        FontId::new(Self::XL3, self.font_family.clone())
    }

    /// Heading 4: 24px (2xl)
    pub fn h4(&self) -> FontId {
        FontId::new(Self::XL2, self.font_family.clone())
    }

    /// Body text: 16px (base) - default text size
    pub fn body(&self) -> FontId {
        FontId::new(Self::BASE, self.font_family.clone())
    }

    /// Small text: 14px (sm)
    pub fn small(&self) -> FontId {
        FontId::new(Self::SM, self.font_family.clone())
    }

    /// Muted text: 14px (sm) - use with muted color
    pub fn muted(&self) -> FontId {
        FontId::new(Self::SM, self.font_family.clone())
    }

    /// Lead paragraph: 20px (xl) - introductory text
    pub fn lead(&self) -> FontId {
        FontId::new(Self::XL, self.font_family.clone())
    }

    /// Large text: 18px (lg)
    pub fn large(&self) -> FontId {
        FontId::new(Self::LG, self.font_family.clone())
    }

    /// Code/monospace: 14px (sm)
    pub fn code(&self) -> FontId {
        FontId::new(Self::SM, self.mono_family.clone())
    }

    /// Code block/monospace body: 16px (base)
    pub fn code_block(&self) -> FontId {
        FontId::new(Self::BASE, self.mono_family.clone())
    }
}

/// Helper to map shadcn typography to egui's built-in TextStyle
///
/// This provides a mapping from egui's standard text styles to shadcn sizes.
/// Use this to configure egui's Style.text_styles with shadcn typography.
pub fn configure_text_styles(typography: &ShadcnTypography) -> std::collections::BTreeMap<egui::TextStyle, FontId> {
    use egui::TextStyle;

    [
        (TextStyle::Small, typography.small()),
        (TextStyle::Body, typography.body()),
        (TextStyle::Monospace, typography.code()),
        (TextStyle::Button, typography.body()),
        (TextStyle::Heading, typography.h2()),
    ]
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_sizes() {
        let typo = ShadcnTypography::new();

        // Verify heading hierarchy (largest to smallest)
        assert!(typo.h1().size > typo.h2().size);
        assert!(typo.h2().size > typo.h3().size);
        assert!(typo.h3().size > typo.h4().size);
        assert!(typo.h4().size > typo.body().size);
    }

    #[test]
    fn test_semantic_sizes() {
        let typo = ShadcnTypography::new();

        // Verify specific sizes match Tailwind scale
        assert_eq!(typo.h1().size, ShadcnTypography::XL5); // 48px
        assert_eq!(typo.h2().size, ShadcnTypography::XL4); // 36px
        assert_eq!(typo.body().size, ShadcnTypography::BASE); // 16px
        assert_eq!(typo.small().size, ShadcnTypography::SM); // 14px
    }

    #[test]
    fn test_monospace() {
        let typo = ShadcnTypography::new();

        // Code should use monospace family
        assert_eq!(typo.code().family, egui::FontFamily::Monospace);
        assert_eq!(typo.code_block().family, egui::FontFamily::Monospace);
    }
}
