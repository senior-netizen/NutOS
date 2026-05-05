//! WinBridge API translation layer skeleton.
//!
//! WinBridge models the Windows `.exe`/`.dll` path described in the NutShell
//! architecture: Windows API calls are classified, translated to NutOS-facing
//! facilities, and dispatched through the NutKernel compatibility syscall
//! boundary.

pub mod database;
pub mod directx;
pub mod paths;
pub mod registry;
pub mod services;
pub mod translator;
pub mod win32;
pub mod winrt;

pub use translator::{TranslationError, TranslationPlan, WinBridgeTranslator};

/// Compatibility goal for Windows applications under WinBridge.
pub const WINDOWS_COMPATIBILITY_GOAL_PERCENT: u8 = 90;

/// External translation components WinBridge is designed to orchestrate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TranslationBackend {
    /// Wine supplies the Win32 and NT user-mode API implementation.
    Wine,
    /// Proton supplies gaming-focused Wine patches and runtime behavior.
    Proton,
    /// DXVK maps DirectX 9, 10, and 11 calls to Vulkan.
    Dxvk,
    /// VKD3D-Proton maps DirectX 12 and experimental raytracing to Vulkan.
    Vkd3dProton,
    /// NutOS WinRT shim supports modern Windows Store-style APIs.
    NutOsWinRtShim,
}

/// Static profile for the WinBridge subsystem.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WinBridgeProfile {
    /// Human-readable subsystem name.
    pub name: &'static str,
    /// Target Windows application compatibility percentage.
    pub compatibility_goal_percent: u8,
    /// Translation engines used by WinBridge.
    pub backends: &'static [TranslationBackend],
}

impl WinBridgeProfile {
    /// Default profile derived from the NutShell architecture document.
    #[must_use]
    pub const fn architecture_default() -> Self {
        Self {
            name: "WinBridge",
            compatibility_goal_percent: WINDOWS_COMPATIBILITY_GOAL_PERCENT,
            backends: &[
                TranslationBackend::Wine,
                TranslationBackend::Proton,
                TranslationBackend::Dxvk,
                TranslationBackend::Vkd3dProton,
                TranslationBackend::NutOsWinRtShim,
            ],
        }
    }
}

/// WinBridge translation domains from the architecture data flow.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TranslationDomain {
    /// Win32 API calls mapped to NutOS POSIX equivalents.
    Win32,
    /// DirectX calls mapped to Vulkan.
    DirectX,
    /// Registry operations mapped to the NutOS config store.
    Registry,
    /// NTFS/Windows paths mapped to NutFS paths.
    Paths,
    /// Windows service operations mapped to NutOS service equivalents.
    Services,
    /// Modern WinRT calls handled by the NutOS WinRT shim.
    WinRt,
}

/// Ordered set of API domains handled by the initial WinBridge translator.
pub const TRANSLATION_DOMAINS: &[TranslationDomain] = &[
    TranslationDomain::Win32,
    TranslationDomain::DirectX,
    TranslationDomain::Registry,
    TranslationDomain::Paths,
    TranslationDomain::Services,
    TranslationDomain::WinRt,
];
