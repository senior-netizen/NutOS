//! NutOS hardware abstraction layer boundary types.

/// Kernel-facing hardware families in the NutOS layer stack.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HardwareFamily {
    /// Commodity and workstation x86-64 systems.
    X86_64,
    /// ARM64 systems, including Apple Silicon virtualized targets.
    Arm64,
    /// Future RISC-V systems.
    RiscV,
}

/// Describes a HAL backend exposed above NutKernel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HalBackend {
    /// Hardware family served by the backend.
    pub family: HardwareFamily,
    /// Whether this backend is expected in v1 day-one builds.
    pub day_one: bool,
}

/// HAL backends tracked by the initial architecture.
pub const BACKENDS: &[HalBackend] = &[
    HalBackend {
        family: HardwareFamily::X86_64,
        day_one: true,
    },
    HalBackend {
        family: HardwareFamily::Arm64,
        day_one: true,
    },
    HalBackend {
        family: HardwareFamily::RiscV,
        day_one: false,
    },
];
