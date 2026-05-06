#![allow(clippy::doc_markdown)]
#![doc = "Rust scaffolding for NutOS's Acorn desktop environment."]

pub mod desktop;
pub mod rendering;
pub mod toolkit;
pub mod wayland;

/// Top-level Acorn profile assembled from `architecture/UI_DESIGN.md`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcornProfile {
    /// Desktop environment name.
    pub name: &'static str,
    /// Native implementation language.
    pub language: &'static str,
    /// Display protocol used by the compositor and clients.
    pub display_protocol: &'static str,
    /// GPU abstraction used by the renderer.
    pub gpu_backend: &'static str,
    /// Desktop regions enabled by the shell.
    pub layout: desktop::DesktopLayout,
    /// UI design-system tokens used by widgets and shell surfaces.
    pub design_system: toolkit::DesignSystem,
    /// Render plan for early GPU initialization.
    pub render_plan: rendering::RenderPlan,
    /// Wayland integration points for compositor and native shell clients.
    pub wayland_plan: wayland::WaylandPlan,
}

impl AcornProfile {
    /// Architecture-derived default profile for early Acorn scaffolding.
    #[must_use]
    pub const fn architecture_default() -> Self {
        Self {
            name: "Acorn",
            language: "Rust",
            display_protocol: "Wayland",
            gpu_backend: "wgpu",
            layout: desktop::DesktopLayout::architecture_default(),
            design_system: toolkit::DesignSystem::architecture_default(),
            render_plan: rendering::RenderPlan::architecture_default(),
            wayland_plan: wayland::WaylandPlan::architecture_default(),
        }
    }
}
