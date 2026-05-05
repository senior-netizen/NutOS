//! Target architecture metadata for NutKernel builds.

pub mod aarch64;
pub mod riscv64;
pub mod x86_64;

/// Support state for a NutKernel target architecture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SupportStatus {
    /// Primary day-one target.
    Primary,
    /// Planned for a later development phase.
    Planned,
    /// Explicitly unsupported.
    Unsupported,
}

/// Static description of a NutKernel target.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TargetArchitecture {
    /// Canonical Rust-style target family label.
    pub name: &'static str,
    /// NutOS support status from the kernel architecture plan.
    pub status: SupportStatus,
    /// Human-readable support note.
    pub notes: &'static str,
}

/// Architectures tracked by the initial NutKernel plan.
pub const TARGETS: &[TargetArchitecture] = &[x86_64::TARGET, aarch64::TARGET, riscv64::TARGET];
