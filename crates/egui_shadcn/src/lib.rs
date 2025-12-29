//! # egui_shadcn
//!
//! A port of [shadcn/ui](https://ui.shadcn.com) design system and components to [egui](https://github.com/emilk/egui).
//!
//! This crate provides beautifully designed, accessible components that follow shadcn/ui's
//! design principles, adapted for egui's immediate mode paradigm.
//!
//! ## Design System
//!
//! The shadcn design system includes:
//! - **Colors**: Semantic color tokens (primary, secondary, destructive, muted, etc.)
//! - **Spacing**: Consistent spacing scale for margins, padding, and gaps
//! - **Typography**: Font size and weight scale
//! - **Corner Radii**: Standardized border radius values
//! - **Shadows**: Elevation system for depth
//!
//! ## Components
//!
//! Components will be added progressively:
//! - Phase 1: Core components (Badge, Avatar, Card, Alert, Skeleton, Kbd)
//! - Phase 2: Form components (enhanced inputs, selects, checkboxes, etc.)
//! - Phase 3: Navigation (Breadcrumb, Tabs, Sidebar, etc.)
//! - Phase 4: Advanced interactions (Dialog, Drawer, Calendar, etc.)
//! - Phase 5: Data display (Table, Carousel, Chart, etc.)
//!
//! ## Example
//!
//! ```ignore
//! use egui_shadcn::theme::ShadcnTheme;
//!
//! // Apply shadcn theme to your egui app
//! let theme = ShadcnTheme::default();
//! theme.apply(ctx);
//!
//! // Use shadcn components (to be implemented)
//! // ui.add(Badge::new("New").variant(BadgeVariant::Destructive));
//! ```

#![warn(missing_docs)]

pub mod animation;
pub mod theme;
pub mod components;
pub mod notedeck;

// Re-export commonly used items
pub use theme::ShadcnTheme;
pub use notedeck::{NotedeckTheme, NotedeckContextExt};
pub use components::{
    // Phase 2: Core Components
    Badge, BadgeVariant,
    Avatar, AvatarSize,
    Card, card_title, card_description,
    Alert, AlertVariant,
    Skeleton,
    Kbd,
    Button, ButtonVariant, ButtonSize,
    Spinner, SpinnerSize,
    // Phase 3: Form Components
    shadcn_input, shadcn_input_with_error, shadcn_textarea, form_label, form_helper,
    Checkbox,
    Switch,
    Slider,
    Progress,
    Toggle, ToggleVariant, ToggleSize,
    ToggleGroup, ToggleGroupType, ToggleGroupVariant,
    RadioGroup, RadioButton,
    Select,
    DropdownMenu, DropdownMenuResponse,
    Combobox, ComboboxOption,
    // Phase 4: Navigation & Layout
    Separator, SeparatorOrientation,
    Tabs,
    Collapsible, collapsible_trigger,
    Accordion, AccordionType,
    Breadcrumb, BreadcrumbSeparator,
    // Phase 5: Overlays & Feedback
    Dialog, confirm_dialog, ConfirmResult,
    Tooltip, TooltipExt, shadcn_tooltip_for,
    Toast, ToastVariant, Toaster,
    Popover, PopoverExt, PopoverTrigger,
    HoverCard, HoverCardExt,
    Sheet, SheetSide,
    Drawer, DrawerSide,
    AlertDialog, AlertDialogResult,
    ContextMenu, ContextMenuResponse, ContextMenuExt,
    // Phase 6: Data Display & Advanced
    Pagination,
    AspectRatio, AspectRatioPreset, AspectRatioResponse,
    Table, TableBody, TableResponse, simple_table,
    Command, CommandGroupBuilder,
    Calendar, CalendarMode, CalendarSelection,
    DatePicker,
    Carousel, CarouselOrientation,
    Chart, ChartType, DataPoint,
    ResizablePanelGroup, ResizableDirection,
    // Phase 7: Navigation & Forms
    Menubar, MenuBuilder, MenubarResponse,
    Sidebar, SidebarBuilder, SidebarResponse,
    NavigationMenu, NavDropdownBuilder, NavigationMenuResponse,
    Field, FieldResponse, labeled_input, required_input,
    FormState, validators,
};
