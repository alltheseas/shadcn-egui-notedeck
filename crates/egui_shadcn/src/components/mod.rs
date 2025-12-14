//! shadcn/ui components ported to egui
//!
//! This module contains all the UI components from shadcn/ui,
//! adapted for use with egui's immediate mode paradigm.
//!
//! ## Component Categories
//!
//! ### Phase 2: Core Components ✓
//! - Badge, Avatar, Card, Alert, Skeleton, Kbd
//!
//! ### Phase 3: Form Components ✓
//! - Input helpers with shadcn styling
//!
//! ### Phase 4: Navigation & Layout ✓
//! - Separator, Tabs
//!
//! ### Future Phases
//! - Advanced interactions (dialogs, drawers, etc.)
//! - Data display (tables, carousels, charts)

// Phase 2: Core Components
pub mod badge;
pub mod avatar;
pub mod card;
pub mod alert;
pub mod skeleton;
pub mod kbd;
pub mod button;
pub mod spinner;

// Phase 3: Form Components
pub mod input;
pub mod checkbox;
pub mod switch;
pub mod slider;
pub mod progress;
pub mod label;
pub mod textarea;
pub mod toggle;
pub mod toggle_group;
pub mod radio;
pub mod select;
pub mod dropdown_menu;
pub mod combobox;

// Phase 4: Navigation & Layout
pub mod separator;
pub mod tabs;
pub mod collapsible;
pub mod accordion;
pub mod breadcrumb;

// Phase 5: Overlays & Feedback
pub mod dialog;
pub mod tooltip;
pub mod toast;
pub mod popover;
pub mod hover_card;
pub mod sheet;
pub mod drawer;
pub mod alert_dialog;
pub mod context_menu;

// Phase 6: Data Display & Advanced
pub mod pagination;
pub mod aspect_ratio;
pub mod table;
pub mod command;
pub mod calendar;
pub mod date_picker;
pub mod carousel;
pub mod chart;
pub mod resizable;

// Phase 7: Navigation & Forms
pub mod menubar;
pub mod sidebar;
pub mod navigation_menu;
pub mod field;
pub mod form;

pub use badge::{Badge, BadgeVariant};
pub use avatar::{Avatar, AvatarSize};
pub use card::{Card, card_title, card_description};
pub use alert::{Alert, AlertVariant};
pub use skeleton::Skeleton;
pub use kbd::Kbd;
pub use button::{Button, ButtonVariant, ButtonSize};
pub use spinner::{Spinner, SpinnerSize};

pub use input::{shadcn_input, shadcn_input_with_error, shadcn_textarea, form_label, form_helper};
pub use checkbox::Checkbox;
pub use switch::Switch;
pub use slider::Slider;
pub use progress::Progress;
pub use label::Label;
pub use textarea::Textarea;
pub use toggle::{Toggle, ToggleVariant, ToggleSize};
pub use toggle_group::{ToggleGroup, ToggleGroupType, ToggleGroupVariant};
pub use radio::{RadioGroup, RadioButton};
pub use select::Select;
pub use dropdown_menu::{DropdownMenu, DropdownMenuResponse};
pub use combobox::{Combobox, ComboboxOption};

pub use separator::{Separator, SeparatorOrientation};
pub use tabs::Tabs;
pub use collapsible::{Collapsible, collapsible_trigger};
pub use accordion::{Accordion, AccordionType};
pub use breadcrumb::{Breadcrumb, BreadcrumbSeparator};

pub use dialog::{Dialog, confirm_dialog, ConfirmResult};
pub use tooltip::{Tooltip, TooltipExt, shadcn_tooltip_for};
pub use toast::{Toast, ToastVariant, Toaster};
pub use popover::{Popover, PopoverExt, PopoverTrigger};
pub use hover_card::{HoverCard, HoverCardExt};
pub use sheet::{Sheet, SheetSide};
pub use drawer::{Drawer, DrawerSide};
pub use alert_dialog::{AlertDialog, AlertDialogResult};
pub use context_menu::{ContextMenu, ContextMenuResponse, ContextMenuExt};

pub use pagination::Pagination;
pub use aspect_ratio::{AspectRatio, AspectRatioPreset, AspectRatioResponse};
pub use table::{Table, TableBody, TableResponse, simple_table};
pub use command::{Command, CommandGroupBuilder};
pub use calendar::{Calendar, CalendarMode, CalendarSelection};
pub use date_picker::DatePicker;
pub use carousel::{Carousel, CarouselOrientation};
pub use chart::{Chart, ChartType, DataPoint};
pub use resizable::{ResizablePanelGroup, ResizableDirection};

pub use menubar::{Menubar, MenuBuilder, MenubarResponse};
pub use sidebar::{Sidebar, SidebarBuilder, SidebarResponse};
pub use navigation_menu::{NavigationMenu, NavDropdownBuilder, NavigationMenuResponse};
pub use field::{Field, FieldResponse, labeled_input, required_input};
pub use form::{FormState, validators};
