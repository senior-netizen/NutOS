//! NutOS custom syscall ABI registry.

/// Stable names for NutOS-specific syscalls.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NutSyscall {
    /// Creates a sandboxed execution environment.
    SandboxCreate,
    /// Marks memory regions as privacy-protected.
    PrivacyFence,
    /// Launches a binary through the NutShell compatibility layer.
    CompatExec,
    /// Hints to the kernel for GPU-accelerated UI rendering.
    UiAccelerate,
}

impl NutSyscall {
    /// C ABI symbol name used by the Linux syscall table patch.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::SandboxCreate => "nut_sandbox_create",
            Self::PrivacyFence => "nut_privacy_fence",
            Self::CompatExec => "nut_compat_exec",
            Self::UiAccelerate => "nut_ui_accelerate",
        }
    }
}

/// Initial custom syscall registry.
pub const REGISTRY: &[NutSyscall] = &[
    NutSyscall::SandboxCreate,
    NutSyscall::PrivacyFence,
    NutSyscall::CompatExec,
    NutSyscall::UiAccelerate,
];
