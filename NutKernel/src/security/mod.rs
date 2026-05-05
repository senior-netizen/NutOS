//! Security hardening switches for the NutKernel Linux patch set.

/// Required security controls from the NutKernel architecture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecurityHardening {
    /// Kernel Address Space Layout Randomization.
    pub kaslr: bool,
    /// Supervisor Mode Execution Prevention on x86-64.
    pub smep: bool,
    /// Supervisor Mode Access Prevention on x86-64.
    pub smap: bool,
    /// Require signatures for all loadable kernel modules.
    pub signed_modules: bool,
    /// Restrict unprivileged access to `/proc` and `/sys`.
    pub restricted_proc_sys: bool,
}

impl SecurityHardening {
    /// Baseline hardening profile mandated for NutKernel builds.
    #[must_use]
    pub const fn baseline() -> Self {
        Self {
            kaslr: true,
            smep: true,
            smap: true,
            signed_modules: true,
            restricted_proc_sys: true,
        }
    }
}
