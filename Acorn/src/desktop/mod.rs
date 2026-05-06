//! Shell layout models for Acorn's desktop surfaces.

/// Primary screen regions described by the Acorn UI architecture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShellSurface {
    /// Top status and menu bar.
    NutBar,
    /// Configurable launcher and running-app dock.
    Dock,
    /// GPU-composited desktop canvas.
    DesktopCanvas,
    /// Universal search overlay.
    Spotlight,
    /// Overview for windows and virtual desktops.
    MissionControl,
}

/// Dock placement preference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DockPosition {
    /// Dock is attached to the left edge.
    Left,
    /// Dock is attached to the right edge.
    Right,
    /// Dock is attached to the bottom edge.
    Bottom,
    /// Dock is hidden until invoked by shortcut or gesture.
    Hidden,
}

/// Desktop layout defaults for the first Acorn shell milestone.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DesktopLayout {
    /// Enabled shell surfaces.
    pub surfaces: &'static [ShellSurface],
    /// Default dock edge.
    pub dock_position: DockPosition,
    /// Maximum number of virtual workspaces.
    pub max_workspaces: u8,
    /// Whether tiling window-management primitives are included.
    pub tiling_enabled: bool,
}

impl DesktopLayout {
    /// Architecture-derived layout defaults.
    #[must_use]
    pub const fn architecture_default() -> Self {
        Self {
            surfaces: &[
                ShellSurface::NutBar,
                ShellSurface::Dock,
                ShellSurface::DesktopCanvas,
                ShellSurface::Spotlight,
                ShellSurface::MissionControl,
            ],
            dock_position: DockPosition::Left,
            max_workspaces: 16,
            tiling_enabled: true,
        }
    }
}
