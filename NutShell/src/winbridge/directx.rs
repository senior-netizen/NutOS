//! DirectX-to-Vulkan translation declarations.

/// DirectX feature families tracked by WinBridge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectXVersion {
    /// DirectX 9.
    Dx9,
    /// DirectX 10.
    Dx10,
    /// DirectX 11.
    Dx11,
    /// DirectX 12.
    Dx12,
    /// DirectX Raytracing.
    Raytracing,
}

/// Architecture-level support status for a DirectX translation path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectXSupportStatus {
    /// Excellent compatibility in the architecture matrix.
    Excellent,
    /// Very good compatibility in the architecture matrix.
    VeryGood,
    /// Good compatibility in the architecture matrix.
    Good,
    /// Experimental compatibility in the architecture matrix.
    Experimental,
}

/// Vulkan translation backend selected for a DirectX family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VulkanBridge {
    /// DXVK backend.
    Dxvk,
    /// VKD3D-Proton backend.
    Vkd3dProton,
}

/// Planned DirectX translation route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectXTranslation {
    /// DirectX version family.
    pub version: DirectXVersion,
    /// Vulkan bridge used for this version.
    pub bridge: VulkanBridge,
    /// Architecture support status.
    pub status: DirectXSupportStatus,
}

impl DirectXTranslation {
    /// Return the architecture-defined translation path for a DirectX version.
    #[must_use]
    pub const fn for_version(version: DirectXVersion) -> Option<Self> {
        match version {
            DirectXVersion::Dx9 | DirectXVersion::Dx10 => Some(Self {
                version,
                bridge: VulkanBridge::Dxvk,
                status: DirectXSupportStatus::Excellent,
            }),
            DirectXVersion::Dx11 => Some(Self {
                version,
                bridge: VulkanBridge::Dxvk,
                status: DirectXSupportStatus::VeryGood,
            }),
            DirectXVersion::Dx12 => Some(Self {
                version,
                bridge: VulkanBridge::Vkd3dProton,
                status: DirectXSupportStatus::Good,
            }),
            DirectXVersion::Raytracing => Some(Self {
                version,
                bridge: VulkanBridge::Vkd3dProton,
                status: DirectXSupportStatus::Experimental,
            }),
        }
    }
}

/// DirectX support matrix from the NutShell architecture.
pub const SUPPORT_MATRIX: &[DirectXTranslation] = &[
    DirectXTranslation {
        version: DirectXVersion::Dx9,
        bridge: VulkanBridge::Dxvk,
        status: DirectXSupportStatus::Excellent,
    },
    DirectXTranslation {
        version: DirectXVersion::Dx10,
        bridge: VulkanBridge::Dxvk,
        status: DirectXSupportStatus::Excellent,
    },
    DirectXTranslation {
        version: DirectXVersion::Dx11,
        bridge: VulkanBridge::Dxvk,
        status: DirectXSupportStatus::VeryGood,
    },
    DirectXTranslation {
        version: DirectXVersion::Dx12,
        bridge: VulkanBridge::Vkd3dProton,
        status: DirectXSupportStatus::Good,
    },
    DirectXTranslation {
        version: DirectXVersion::Raytracing,
        bridge: VulkanBridge::Vkd3dProton,
        status: DirectXSupportStatus::Experimental,
    },
];
