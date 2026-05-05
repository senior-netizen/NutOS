//! Performance tuning declarations for NutKernel.

/// NutKernel performance features described by the architecture plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PerformanceTuning {
    /// Use the Burst-Oriented Response Enhancer scheduler.
    pub bore_scheduler: bool,
    /// Prefer zstd compression for memory and storage operations.
    pub zstd_compression: bool,
    /// Enable transparent huge pages by default.
    pub transparent_huge_pages: bool,
    /// Apply aggressive laptop-oriented power management.
    pub laptop_power_management: bool,
}

impl PerformanceTuning {
    /// Default performance profile for desktop-responsive NutOS builds.
    #[must_use]
    pub const fn desktop_default() -> Self {
        Self {
            bore_scheduler: true,
            zstd_compression: true,
            transparent_huge_pages: true,
            laptop_power_management: true,
        }
    }
}
