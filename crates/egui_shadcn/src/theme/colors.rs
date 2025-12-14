//! Color system ported from shadcn/ui
//!
//! shadcn/ui uses OKLCH color format with semantic tokens. This module converts
//! those to egui's Color32 (RGBA) format while preserving the design system's intent.
//!
//! Reference: <https://ui.shadcn.com/docs/theming>
//!
//! The color system uses a background/foreground convention:
//! - `background`: The background color of the component
//! - `foreground`: The foreground (text) color of the component
//!
//! Semantic color roles:
//! - **primary**: Primary brand color for main actions
//! - **secondary**: Secondary color for less prominent actions
//! - **destructive**: Error/danger states and destructive actions
//! - **muted**: Subtle/muted UI elements
//! - **accent**: Accent color for highlights and focus states
//! - **card**: Card/container backgrounds
//! - **popover**: Popover/dropdown backgrounds
//! - **border**: Border colors
//! - **input**: Input field borders
//! - **ring**: Focus ring colors

use egui::Color32;

/// Complete shadcn color palette with light and dark mode variants
///
/// Each color has a background and foreground variant. The background is the
/// surface color, and the foreground is the text/content color that goes on top.
///
/// Converted from shadcn's OKLCH format to RGB for egui compatibility.
/// OKLCH values from: <https://ui.shadcn.com/docs/theming>
#[derive(Debug, Clone, PartialEq)]
pub struct ShadcnColors {
    /// Primary background color (main app surface)
    pub background: Color32,
    /// Primary foreground/text color
    pub foreground: Color32,

    /// Card container background
    pub card: Color32,
    /// Card foreground/text color
    pub card_foreground: Color32,

    /// Popover/dropdown background
    pub popover: Color32,
    /// Popover foreground/text color
    pub popover_foreground: Color32,

    /// Primary brand color (buttons, links, etc.)
    pub primary: Color32,
    /// Primary foreground (text on primary background)
    pub primary_foreground: Color32,

    /// Secondary/alternative color
    pub secondary: Color32,
    /// Secondary foreground
    pub secondary_foreground: Color32,

    /// Muted/subtle elements
    pub muted: Color32,
    /// Muted foreground
    pub muted_foreground: Color32,

    /// Accent color for highlights
    pub accent: Color32,
    /// Accent foreground
    pub accent_foreground: Color32,

    /// Destructive/error/danger color
    pub destructive: Color32,
    /// Destructive foreground (text on destructive background)
    pub destructive_foreground: Color32,

    /// Border colors for UI elements
    pub border: Color32,

    /// Input field borders
    pub input: Color32,

    /// Focus ring color
    pub ring: Color32,

    /// Sidebar background
    pub sidebar: Color32,
    /// Sidebar foreground/text
    pub sidebar_foreground: Color32,
    /// Sidebar border
    pub sidebar_border: Color32,
    /// Sidebar accent (active/hover items)
    pub sidebar_accent: Color32,
    /// Sidebar accent foreground
    pub sidebar_accent_foreground: Color32,
}

impl ShadcnColors {
    /// Create light mode color palette
    ///
    /// Based on notedeck's vibrant purple/pink color scheme
    pub fn light() -> Self {
        Self {
            background: Color32::WHITE,
            foreground: Color32::BLACK,

            card: Color32::WHITE,
            card_foreground: Color32::BLACK,

            popover: Color32::WHITE,
            popover_foreground: Color32::BLACK,

            // Notedeck purple as primary (darkened for 4.5:1 WCAG AA contrast)
            primary: Color32::from_rgb(0xB7, 0x3C, 0xB1),
            primary_foreground: Color32::WHITE,

            // Light gray for secondary
            secondary: Color32::from_rgb(0xf8, 0xf8, 0xf8),
            secondary_foreground: Color32::BLACK,

            muted: Color32::from_rgb(0xf8, 0xf8, 0xf8),
            muted_foreground: Color32::from_rgb(0x6B, 0x6B, 0x6B), // Darker for better contrast on white

            // Purple alt for accent
            accent: Color32::from_rgb(0x82, 0x56, 0xDD),
            accent_foreground: Color32::WHITE,

            // Notedeck red for destructive
            destructive: Color32::from_rgb(0xC7, 0x37, 0x5A),
            destructive_foreground: Color32::WHITE,

            // Darker border for better visibility on white
            border: Color32::from_rgb(200, 200, 205),
            input: Color32::from_rgb(200, 200, 205),

            // Even darker for focus rings and scrollbars
            ring: Color32::from_rgb(140, 140, 150),

            // Sidebar colors - slightly off-white background
            sidebar: Color32::from_rgb(0xFA, 0xFA, 0xFA),
            sidebar_foreground: Color32::BLACK,
            sidebar_border: Color32::from_rgb(0xE4, 0xE4, 0xE7),
            sidebar_accent: Color32::from_rgb(0xF4, 0xF4, 0xF5),
            sidebar_accent_foreground: Color32::BLACK,
        }
    }

    /// Create dark mode color palette
    ///
    /// Based on notedeck's dark theme with purple accents
    pub fn dark() -> Self {
        Self {
            // Notedeck dark backgrounds
            background: Color32::from_rgb(0x1F, 0x1F, 0x1F),
            foreground: Color32::WHITE,

            card: Color32::from_rgb(0x25, 0x25, 0x25),
            card_foreground: Color32::WHITE,

            popover: Color32::from_rgb(0x25, 0x25, 0x25),
            popover_foreground: Color32::WHITE,

            // Notedeck purple as primary (original - balanced 4.06:1 contrast both ways)
            primary: Color32::from_rgb(0xCC, 0x43, 0xC5),
            primary_foreground: Color32::WHITE,

            secondary: Color32::from_rgb(0x44, 0x44, 0x44),
            secondary_foreground: Color32::WHITE,

            muted: Color32::from_rgb(0x44, 0x44, 0x44),
            muted_foreground: Color32::from_rgb(0xE8, 0xE8, 0xE8), // Nearly white for excellent contrast on dark backgrounds

            // Purple alt for accent
            accent: Color32::from_rgb(0x82, 0x56, 0xDD),
            accent_foreground: Color32::WHITE,

            // Notedeck red for destructive
            destructive: Color32::from_rgb(0xC7, 0x37, 0x5A),
            destructive_foreground: Color32::WHITE,

            // oklch(1 0 0 / 10%) = white with low opacity
            border: Color32::from_rgba_unmultiplied(255, 255, 255, 25),
            // oklch(1 0 0 / 15%) = white with slightly higher opacity
            input: Color32::from_rgba_unmultiplied(255, 255, 255, 38),

            // oklch(0.556 0 0) = mid gray
            ring: Color32::from_rgb(113, 113, 122),

            // Sidebar colors - dark background
            sidebar: Color32::from_rgb(0x18, 0x18, 0x1B),
            sidebar_foreground: Color32::from_rgb(0xFA, 0xFA, 0xFA),
            sidebar_border: Color32::from_rgb(0x27, 0x27, 0x2A),
            sidebar_accent: Color32::from_rgb(0x27, 0x27, 0x2A),
            sidebar_accent_foreground: Color32::from_rgb(0xFA, 0xFA, 0xFA),
        }
    }
}

impl Default for ShadcnColors {
    /// Default to light mode colors
    fn default() -> Self {
        Self::light()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_colors() {
        let colors = ShadcnColors::light();
        // Background should be white in light mode
        assert_eq!(colors.background, Color32::from_rgb(255, 255, 255));
        // Foreground should be dark in light mode
        assert_eq!(colors.foreground.r(), 10);
    }

    #[test]
    fn test_dark_colors() {
        let colors = ShadcnColors::dark();
        // Background should be very dark in dark mode
        assert_eq!(colors.background, Color32::from_rgb(10, 10, 10));
        // Foreground should be light in dark mode
        assert_eq!(colors.foreground.r(), 250);
    }

    #[test]
    fn test_color_contrast() {
        let light = ShadcnColors::light();
        let dark = ShadcnColors::dark();

        // Light and dark should be inverses
        assert_eq!(light.background, dark.foreground);
        assert_eq!(light.foreground, dark.background);
    }
}
