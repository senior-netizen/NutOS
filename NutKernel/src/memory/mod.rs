//! NutKernel memory management policy declarations.

/// Memory management features expected by NutOS.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryPolicy {
    /// Enable zram as compressed swap in RAM.
    pub zram: bool,
    /// Replace generic OOM behavior with priority-aware selection.
    pub priority_aware_oom: bool,
    /// Enforce sandbox memory ceilings at the kernel level.
    pub sandbox_memory_limits: bool,
}

impl MemoryPolicy {
    /// Baseline memory policy for NutKernel builds.
    #[must_use]
    pub const fn baseline() -> Self {
        Self {
            zram: true,
            priority_aware_oom: true,
            sandbox_memory_limits: true,
        }
    }
}
