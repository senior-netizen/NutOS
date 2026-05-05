//! RISC-V NutKernel target metadata.

use super::{SupportStatus, TargetArchitecture};

/// Planned RISC-V target for a later NutOS phase.
pub const TARGET: TargetArchitecture = TargetArchitecture {
    name: "riscv64",
    status: SupportStatus::Planned,
    notes: "Phase 3 target tracked in the kernel roadmap.",
};
