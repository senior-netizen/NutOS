#![allow(clippy::doc_markdown)]
#![doc = "Rust scaffolding for NutOS's NutShell compatibility layer."]

pub mod winbridge;

/// NutShell compatibility subsystems described by the architecture plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompatibilitySubsystem {
    /// Native Linux ELF execution with filesystem and packaging shims.
    LinuxRun,
    /// Windows executable and DLL compatibility through the WinBridge layer.
    WinBridge,
    /// Android APK compatibility through a host-kernel container.
    DroidShell,
    /// Progressive Web App and Electron-compatible runtime support.
    WebApp,
}

/// Top-level NutShell profile assembled from `architecture/NUTSHELL.md`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NutShellProfile {
    /// Architecture layer name.
    pub name: &'static str,
    /// Subsystems tracked by the compatibility layer.
    pub subsystems: &'static [CompatibilitySubsystem],
    /// Kernel syscall family expected below NutShell.
    pub kernel_interface: &'static str,
}

impl NutShellProfile {
    /// Architecture-derived default profile for early NutShell scaffolding.
    #[must_use]
    pub const fn architecture_default() -> Self {
        Self {
            name: "NutShell",
            subsystems: &[
                CompatibilitySubsystem::LinuxRun,
                CompatibilitySubsystem::WinBridge,
                CompatibilitySubsystem::DroidShell,
                CompatibilitySubsystem::WebApp,
            ],
            kernel_interface: "NutOS Kernel Compat Syscalls",
        }
    }
}
