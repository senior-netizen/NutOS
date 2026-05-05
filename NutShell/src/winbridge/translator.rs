//! Translation coordinator for WinBridge API calls.

use super::directx::{DirectXTranslation, DirectXVersion};
use super::paths::PathTranslation;
use super::registry::RegistryTranslation;
use super::services::ServiceTranslation;
use super::win32::Win32Translation;
use super::winrt::WinRtTranslation;
use super::{TRANSLATION_DOMAINS, TranslationDomain};

/// A Windows API request after coarse classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TranslationPlan {
    /// Win32 call translated to a POSIX-facing NutOS operation.
    Win32(Win32Translation),
    /// DirectX call translated to a Vulkan-facing operation.
    DirectX(DirectXTranslation),
    /// Registry call translated to the NutOS config store.
    Registry(RegistryTranslation),
    /// Windows path translated to a NutFS path.
    Path(PathTranslation),
    /// Windows service translated to a NutOS service operation.
    Service(ServiceTranslation),
    /// WinRT call translated through the NutOS WinRT shim.
    WinRt(WinRtTranslation),
}

/// Translation errors reported by the skeleton coordinator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TranslationError {
    /// The requested domain is not enabled by this translator instance.
    UnsupportedDomain(TranslationDomain),
    /// The requested DirectX version has no configured Vulkan translation path.
    UnsupportedDirectXVersion(DirectXVersion),
}

/// Stateless coordinator for WinBridge translation planning.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WinBridgeTranslator {
    /// Translation domains enabled for this coordinator.
    pub enabled_domains: &'static [TranslationDomain],
}

impl WinBridgeTranslator {
    /// Construct a translator with all architecture-defined domains enabled.
    #[must_use]
    pub const fn architecture_default() -> Self {
        Self {
            enabled_domains: TRANSLATION_DOMAINS,
        }
    }

    /// Returns whether this translator currently handles a domain.
    #[must_use]
    pub fn supports(self, domain: TranslationDomain) -> bool {
        self.enabled_domains.contains(&domain)
    }

    /// Build a Win32-to-POSIX translation plan.
    ///
    /// # Errors
    ///
    /// Returns [`TranslationError::UnsupportedDomain`] if Win32 translation is disabled.
    pub fn translate_win32(
        self,
        call: super::win32::Win32Call,
    ) -> Result<TranslationPlan, TranslationError> {
        self.ensure_domain(TranslationDomain::Win32)?;
        Ok(TranslationPlan::Win32(Win32Translation::for_call(call)))
    }

    /// Build a DirectX-to-Vulkan translation plan.
    ///
    /// # Errors
    ///
    /// Returns [`TranslationError::UnsupportedDomain`] if DirectX translation is disabled,
    /// or [`TranslationError::UnsupportedDirectXVersion`] if no Vulkan bridge is configured.
    pub fn translate_directx(
        self,
        version: DirectXVersion,
    ) -> Result<TranslationPlan, TranslationError> {
        self.ensure_domain(TranslationDomain::DirectX)?;
        DirectXTranslation::for_version(version)
            .map(TranslationPlan::DirectX)
            .ok_or(TranslationError::UnsupportedDirectXVersion(version))
    }

    /// Build a registry-to-config-store translation plan.
    ///
    /// # Errors
    ///
    /// Returns [`TranslationError::UnsupportedDomain`] if registry translation is disabled.
    pub fn translate_registry(
        self,
        hive: super::registry::RegistryHive,
    ) -> Result<TranslationPlan, TranslationError> {
        self.ensure_domain(TranslationDomain::Registry)?;
        Ok(TranslationPlan::Registry(RegistryTranslation::for_hive(
            hive,
        )))
    }

    /// Build an NTFS-to-NutFS path translation plan.
    ///
    /// # Errors
    ///
    /// Returns [`TranslationError::UnsupportedDomain`] if path translation is disabled.
    pub fn translate_path(
        self,
        path_kind: super::paths::WindowsPathKind,
    ) -> Result<TranslationPlan, TranslationError> {
        self.ensure_domain(TranslationDomain::Paths)?;
        Ok(TranslationPlan::Path(PathTranslation::for_kind(path_kind)))
    }

    /// Build a Windows-service-to-NutOS-service translation plan.
    ///
    /// # Errors
    ///
    /// Returns [`TranslationError::UnsupportedDomain`] if service translation is disabled.
    pub fn translate_service(
        self,
        operation: super::services::ServiceOperation,
    ) -> Result<TranslationPlan, TranslationError> {
        self.ensure_domain(TranslationDomain::Services)?;
        Ok(TranslationPlan::Service(ServiceTranslation::for_operation(
            operation,
        )))
    }

    /// Build a WinRT shim translation plan.
    ///
    /// # Errors
    ///
    /// Returns [`TranslationError::UnsupportedDomain`] if WinRT translation is disabled.
    pub fn translate_winrt(
        self,
        namespace: super::winrt::WinRtNamespace,
    ) -> Result<TranslationPlan, TranslationError> {
        self.ensure_domain(TranslationDomain::WinRt)?;
        Ok(TranslationPlan::WinRt(WinRtTranslation::for_namespace(
            namespace,
        )))
    }

    fn ensure_domain(self, domain: TranslationDomain) -> Result<(), TranslationError> {
        if self.supports(domain) {
            Ok(())
        } else {
            Err(TranslationError::UnsupportedDomain(domain))
        }
    }
}
