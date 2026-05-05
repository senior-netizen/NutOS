//! NutOS WinRT shim declarations.

/// WinRT namespaces routed through the NutOS shim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WinRtNamespace {
    /// Application lifecycle APIs.
    ApplicationModel,
    /// Storage broker APIs.
    Storage,
    /// Notifications APIs.
    Notifications,
}

/// NutOS shim surface selected for a WinRT namespace.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WinRtShimSurface {
    /// App lifecycle/session integration.
    AppLifecycle,
    /// Sandboxed storage broker.
    StorageBroker,
    /// NutOS notification center bridge.
    NotificationCenter,
}

/// Planned WinRT translation route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WinRtTranslation {
    /// Source WinRT namespace family.
    pub namespace: WinRtNamespace,
    /// Target NutOS shim surface.
    pub surface: WinRtShimSurface,
}

impl WinRtTranslation {
    /// Return the NutOS shim surface for a WinRT namespace.
    #[must_use]
    pub const fn for_namespace(namespace: WinRtNamespace) -> Self {
        let surface = match namespace {
            WinRtNamespace::ApplicationModel => WinRtShimSurface::AppLifecycle,
            WinRtNamespace::Storage => WinRtShimSurface::StorageBroker,
            WinRtNamespace::Notifications => WinRtShimSurface::NotificationCenter,
        };

        Self { namespace, surface }
    }
}
