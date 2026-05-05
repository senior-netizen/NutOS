//! x86-64 NutKernel target metadata.

use super::{SupportStatus, TargetArchitecture};

/// Primary x86-64 build target with mandatory SMEP/SMAP hardening.
pub const TARGET: TargetArchitecture = TargetArchitecture {
    name: "x86_64",
    status: SupportStatus::Primary,
    notes: "Primary day-one target with KASLR, SMEP, and SMAP enforcement.",
};
