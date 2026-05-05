#![no_std]
#![allow(clippy::doc_markdown, clippy::struct_excessive_bools)]
#![doc = "Rust scaffolding for NutOS's NutKernel integration layer."]

pub mod arch;
pub mod hal;
pub mod memory;
pub mod performance;
pub mod privacy;
pub mod security;
pub mod syscalls;

/// High-level NutKernel profile assembled from the architecture document.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelProfile {
    /// Kernel layer name used across NutOS architecture documents.
    pub name: &'static str,
    /// Whether the profile assumes a modified Linux base.
    pub linux_based: bool,
    /// Kernel modules must be cryptographically signed before loading.
    pub signed_modules_required: bool,
    /// Default filesystem expected by the NutOS boot profile.
    pub default_filesystem: &'static str,
}

impl KernelProfile {
    /// Architecture-derived default profile for early NutKernel scaffolding.
    #[must_use]
    pub const fn architecture_default() -> Self {
        Self {
            name: "NutKernel",
            linux_based: true,
            signed_modules_required: true,
            default_filesystem: "NutFS",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KernelProfile;

    #[test]
    fn default_profile_matches_architecture() {
        let profile = KernelProfile::architecture_default();

        assert_eq!(profile.name, "NutKernel");
        assert!(profile.linux_based);
        assert!(profile.signed_modules_required);
        assert_eq!(profile.default_filesystem, "NutFS");
    }
}
