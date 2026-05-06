//! GPU rendering setup for Acorn's compositor and native UI toolkit.

/// GPU frame-rate targets from the Acorn UI architecture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FramePacing {
    /// Minimum animation target in frames per second.
    pub minimum_fps: u16,
    /// High-refresh target in frames per second on capable displays.
    pub preferred_fps: u16,
    /// Whether presentation should be locked to the display refresh rate.
    pub vsync_locked: bool,
}

/// wgpu backend policy for Acorn.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderPlan {
    /// Human-readable renderer stack name.
    pub renderer: &'static str,
    /// wgpu backend bits Acorn should request during adapter discovery.
    pub backends: wgpu::Backends,
    /// Shader source format used by built-in shell effects.
    pub shader_format: &'static str,
    /// Frame-pacing policy.
    pub frame_pacing: FramePacing,
}

impl RenderPlan {
    /// Architecture-derived GPU defaults using wgpu over Vulkan, Metal, and OpenGL/GLES.
    #[must_use]
    pub const fn architecture_default() -> Self {
        Self {
            renderer: "wgpu",
            backends: wgpu::Backends::VULKAN
                .union(wgpu::Backends::METAL)
                .union(wgpu::Backends::GL),
            shader_format: "WGSL",
            frame_pacing: FramePacing {
                minimum_fps: 60,
                preferred_fps: 120,
                vsync_locked: true,
            },
        }
    }
}

/// Construct a wgpu instance for Acorn's compositor process.
#[must_use]
pub fn create_instance() -> wgpu::Instance {
    let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
    descriptor.backends = RenderPlan::architecture_default().backends;
    wgpu::Instance::new(descriptor)
}
