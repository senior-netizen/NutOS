//! Native Acorn UI toolkit design tokens.

/// Spacing tokens in Acorn's 8pt grid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpacingScale {
    /// Micro spacing for borders and tight padding.
    pub micro_px: u8,
    /// Small spacing for button padding and icon gaps.
    pub small_px: u8,
    /// Base spacing for cards and common sections.
    pub base_px: u8,
    /// Medium spacing for panels.
    pub medium_px: u8,
    /// Large spacing for major section dividers.
    pub large_px: u8,
    /// Extra-large spacing for hero layouts and large empty states.
    pub xl_px: u8,
}

/// Motion spring parameters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpringPreset {
    /// Spring stiffness coefficient.
    pub stiffness: u16,
    /// Spring damping coefficient.
    pub damping: u16,
}

/// Native widget categories included in the Acorn UI toolkit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentKind {
    /// Primary, secondary, ghost, and destructive buttons.
    Buttons,
    /// Text, number, password, and search inputs.
    Inputs,
    /// Single and multi-select dropdowns.
    Select,
    /// Modal dialogs and sheets.
    Modals,
    /// Toasts and persistent banners.
    Notifications,
    /// Determinate and indeterminate progress indicators.
    Progress,
    /// Toggle, checkbox, and radio controls.
    SelectionControls,
    /// Sortable, filterable, and virtualized tables.
    Tables,
    /// Collapsible and multi-level navigation.
    Navigation,
}

/// Architecture-derived design-system model.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DesignSystem {
    /// Spacing scale in physical pixels before user scaling is applied.
    pub spacing: SpacingScale,
    /// Available native widget categories.
    pub components: &'static [ComponentKind],
    /// Window-open spring preset.
    pub window_open: SpringPreset,
    /// Button-press spring preset.
    pub button_press: SpringPreset,
    /// Panel-slide spring preset.
    pub panel_slide: SpringPreset,
    /// Animation duration cap in milliseconds.
    pub duration_cap_ms: u16,
    /// Whether dark and light themes are both first-class default targets.
    pub dual_theme: bool,
    /// Whether accessibility is a baseline requirement for toolkit components.
    pub accessibility_first: bool,
}

impl DesignSystem {
    /// Architecture-derived UI design-system defaults.
    #[must_use]
    pub const fn architecture_default() -> Self {
        Self {
            spacing: SpacingScale {
                micro_px: 4,
                small_px: 8,
                base_px: 16,
                medium_px: 24,
                large_px: 32,
                xl_px: 48,
            },
            components: &[
                ComponentKind::Buttons,
                ComponentKind::Inputs,
                ComponentKind::Select,
                ComponentKind::Modals,
                ComponentKind::Notifications,
                ComponentKind::Progress,
                ComponentKind::SelectionControls,
                ComponentKind::Tables,
                ComponentKind::Navigation,
            ],
            window_open: SpringPreset {
                stiffness: 280,
                damping: 26,
            },
            button_press: SpringPreset {
                stiffness: 400,
                damping: 30,
            },
            panel_slide: SpringPreset {
                stiffness: 220,
                damping: 28,
            },
            duration_cap_ms: 400,
            dual_theme: true,
            accessibility_first: true,
        }
    }
}
