//! Privacy-preserving kernel behavior planned for NutKernel.

/// Kernel-level privacy controls from the NutOS architecture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrivacyControls {
    /// Patch network behavior to reduce passive OS fingerprinting.
    pub passive_fingerprinting_resistance: bool,
    /// Randomize MAC addresses by default.
    pub randomized_mac_addresses: bool,
    /// Redact hardware identifiers before logs reach user space.
    pub redact_hardware_identifiers: bool,
    /// Mitigate documented timing and shared-cache covert channels.
    pub covert_channel_mitigations: bool,
}

impl PrivacyControls {
    /// Default privacy profile for NutKernel.
    #[must_use]
    pub const fn baseline() -> Self {
        Self {
            passive_fingerprinting_resistance: true,
            randomized_mac_addresses: true,
            redact_hardware_identifiers: true,
            covert_channel_mitigations: true,
        }
    }
}
