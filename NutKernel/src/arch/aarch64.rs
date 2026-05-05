//! ARM64 NutKernel target metadata.

use super::{SupportStatus, TargetArchitecture};

/// Primary ARM64 build target, including initial Apple Silicon VM support.
pub const TARGET: TargetArchitecture = TargetArchitecture {
    name: "aarch64",
    status: SupportStatus::Primary,
    notes: "Primary day-one target, including Apple Silicon via virtual machines initially.",
};
