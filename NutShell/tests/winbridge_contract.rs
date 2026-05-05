#![allow(missing_docs)]

use nutshell::winbridge::database::{CompatibilityEntry, CompatibilityRating, EntrySource};
use nutshell::winbridge::directx::{
    DirectXSupportStatus, DirectXTranslation, DirectXVersion, SUPPORT_MATRIX, VulkanBridge,
};
use nutshell::winbridge::paths::{NutFsTarget, PathTranslation, WindowsPathKind};
use nutshell::winbridge::registry::{ConfigNamespace, RegistryHive, RegistryTranslation};
use nutshell::winbridge::services::{NutServiceEquivalent, ServiceOperation, ServiceTranslation};
use nutshell::winbridge::translator::TranslationPlan;
use nutshell::winbridge::win32::{PosixEquivalent, Win32Call, Win32Translation};
use nutshell::winbridge::winrt::{WinRtNamespace, WinRtShimSurface, WinRtTranslation};
use nutshell::winbridge::{
    TRANSLATION_DOMAINS, TranslationBackend, TranslationDomain, WinBridgeProfile,
    WinBridgeTranslator,
};
use nutshell::{CompatibilitySubsystem, NutShellProfile};

#[test]
fn nutshell_profile_tracks_architecture_subsystems() {
    let profile = NutShellProfile::architecture_default();

    assert_eq!(profile.name, "NutShell");
    assert_eq!(profile.subsystems.len(), 4);
    assert_eq!(profile.subsystems[0], CompatibilitySubsystem::LinuxRun);
    assert_eq!(profile.subsystems[1], CompatibilitySubsystem::WinBridge);
    assert_eq!(profile.subsystems[2], CompatibilitySubsystem::DroidShell);
    assert_eq!(profile.subsystems[3], CompatibilitySubsystem::WebApp);
    assert_eq!(profile.kernel_interface, "NutOS Kernel Compat Syscalls");
}

#[test]
fn winbridge_profile_tracks_architecture_backends() {
    let profile = WinBridgeProfile::architecture_default();

    assert_eq!(profile.name, "WinBridge");
    assert_eq!(profile.compatibility_goal_percent, 90);
    assert_eq!(profile.backends.len(), 5);
    assert_eq!(profile.backends[0], TranslationBackend::Wine);
    assert_eq!(profile.backends[1], TranslationBackend::Proton);
    assert_eq!(profile.backends[2], TranslationBackend::Dxvk);
    assert_eq!(profile.backends[3], TranslationBackend::Vkd3dProton);
    assert_eq!(profile.backends[4], TranslationBackend::NutOsWinRtShim);
}

#[test]
fn registers_winbridge_translation_domains() {
    assert_eq!(TRANSLATION_DOMAINS.len(), 6);
    assert_eq!(TRANSLATION_DOMAINS[0], TranslationDomain::Win32);
    assert_eq!(TRANSLATION_DOMAINS[1], TranslationDomain::DirectX);
    assert_eq!(TRANSLATION_DOMAINS[2], TranslationDomain::Registry);
    assert_eq!(TRANSLATION_DOMAINS[3], TranslationDomain::Paths);
    assert_eq!(TRANSLATION_DOMAINS[4], TranslationDomain::Services);
    assert_eq!(TRANSLATION_DOMAINS[5], TranslationDomain::WinRt);
}

#[test]
fn directx_support_matrix_matches_architecture() {
    assert_eq!(SUPPORT_MATRIX.len(), 5);
    assert_eq!(
        DirectXTranslation::for_version(DirectXVersion::Dx9),
        Some(DirectXTranslation {
            version: DirectXVersion::Dx9,
            bridge: VulkanBridge::Dxvk,
            status: DirectXSupportStatus::Excellent,
        })
    );
    assert_eq!(
        DirectXTranslation::for_version(DirectXVersion::Dx12),
        Some(DirectXTranslation {
            version: DirectXVersion::Dx12,
            bridge: VulkanBridge::Vkd3dProton,
            status: DirectXSupportStatus::Good,
        })
    );
    assert_eq!(
        DirectXTranslation::for_version(DirectXVersion::Raytracing),
        Some(DirectXTranslation {
            version: DirectXVersion::Raytracing,
            bridge: VulkanBridge::Vkd3dProton,
            status: DirectXSupportStatus::Experimental,
        })
    );
}

#[test]
fn translates_core_winbridge_domains() {
    let translator = WinBridgeTranslator::architecture_default();

    assert_eq!(
        translator.translate_win32(Win32Call::FileSystem),
        Ok(TranslationPlan::Win32(Win32Translation {
            call: Win32Call::FileSystem,
            target: PosixEquivalent::FileDescriptors,
        }))
    );
    assert_eq!(
        translator.translate_registry(RegistryHive::LocalMachine),
        Ok(TranslationPlan::Registry(RegistryTranslation {
            hive: RegistryHive::LocalMachine,
            namespace: ConfigNamespace::System,
        }))
    );
    assert_eq!(
        translator.translate_path(WindowsPathKind::DriveLetter),
        Ok(TranslationPlan::Path(PathTranslation {
            source: WindowsPathKind::DriveLetter,
            target: NutFsTarget::PrefixDrive,
        }))
    );
    assert_eq!(
        translator.translate_service(ServiceOperation::Start),
        Ok(TranslationPlan::Service(ServiceTranslation {
            operation: ServiceOperation::Start,
            target: NutServiceEquivalent::ActivateUnit,
        }))
    );
    assert_eq!(
        translator.translate_winrt(WinRtNamespace::Notifications),
        Ok(TranslationPlan::WinRt(WinRtTranslation {
            namespace: WinRtNamespace::Notifications,
            surface: WinRtShimSurface::NotificationCenter,
        }))
    );
}

#[test]
fn creates_compatibility_database_placeholders() {
    let entry = CompatibilityEntry::community_unvalidated("example.windows.app");

    assert_eq!(entry.app_id, "example.windows.app");
    assert_eq!(entry.rating, CompatibilityRating::Unsupported);
    assert!(!entry.has_workarounds);
    assert_eq!(entry.source, EntrySource::Community);
}
