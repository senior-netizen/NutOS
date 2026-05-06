use acorn::{AcornProfile, desktop, rendering, toolkit, wayland};

#[test]
fn default_profile_matches_ui_architecture_stack() {
    let profile = AcornProfile::architecture_default();

    assert_eq!(profile.name, "Acorn");
    assert_eq!(profile.language, "Rust");
    assert_eq!(profile.display_protocol, "Wayland");
    assert_eq!(profile.gpu_backend, "wgpu");
}

#[test]
fn layout_includes_core_shell_surfaces() {
    let layout = desktop::DesktopLayout::architecture_default();

    assert_eq!(layout.dock_position, desktop::DockPosition::Left);
    assert_eq!(layout.max_workspaces, 16);
    assert!(layout.tiling_enabled);
    assert!(layout.surfaces.contains(&desktop::ShellSurface::NutBar));
    assert!(layout.surfaces.contains(&desktop::ShellSurface::Dock));
    assert!(layout.surfaces.contains(&desktop::ShellSurface::Spotlight));
}

#[test]
fn rendering_targets_wgpu_backends_and_refresh_goals() {
    let plan = rendering::RenderPlan::architecture_default();

    assert_eq!(plan.renderer, "wgpu");
    assert!(plan.backends.contains(wgpu::Backends::VULKAN));
    assert!(plan.backends.contains(wgpu::Backends::METAL));
    assert!(plan.backends.contains(wgpu::Backends::GL));
    assert_eq!(plan.frame_pacing.minimum_fps, 60);
    assert_eq!(plan.frame_pacing.preferred_fps, 120);
    assert!(plan.frame_pacing.vsync_locked);
}

#[test]
fn wayland_plan_has_client_server_and_protocol_bindings() {
    let plan = wayland::WaylandPlan::architecture_default();

    assert_eq!(plan.client_crate, "wayland-client");
    assert_eq!(plan.server_crate, "wayland-server");
    assert_eq!(plan.protocols_crate, "wayland-protocols");
    assert!(plan.roles.contains(&wayland::WaylandRole::ServerCompositor));
    assert!(plan.roles.contains(&wayland::WaylandRole::ShellClient));
}

#[test]
fn design_system_uses_documented_tokens() {
    let system = toolkit::DesignSystem::architecture_default();

    assert_eq!(system.spacing.micro_px, 4);
    assert_eq!(system.spacing.base_px, 16);
    assert_eq!(system.spacing.xl_px, 48);
    assert_eq!(system.window_open.stiffness, 280);
    assert_eq!(system.window_open.damping, 26);
    assert_eq!(system.duration_cap_ms, 400);
    assert!(system.dual_theme);
    assert!(system.accessibility_first);
}
