//! Wayland protocol integration for Acorn compositor and shell clients.

/// Wayland roles Acorn needs during the desktop-environment bring-up.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WaylandRole {
    /// Compositor-side display server for applications.
    ServerCompositor,
    /// Shell-owned clients such as NutBar, Dock, and Spotlight.
    ShellClient,
    /// Protocol-extension support for shell surfaces and workspace UX.
    ProtocolExtensions,
}

/// Wayland binding plan for Acorn.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WaylandPlan {
    /// Rust crate used for client-side protocol bindings.
    pub client_crate: &'static str,
    /// Rust crate used for server-side compositor bindings.
    pub server_crate: &'static str,
    /// Rust crate used for generated protocol extension bindings.
    pub protocols_crate: &'static str,
    /// Enabled roles.
    pub roles: &'static [WaylandRole],
}

impl WaylandPlan {
    /// Architecture-derived Wayland binding defaults.
    #[must_use]
    pub const fn architecture_default() -> Self {
        Self {
            client_crate: "wayland-client",
            server_crate: "wayland-server",
            protocols_crate: "wayland-protocols",
            roles: &[
                WaylandRole::ServerCompositor,
                WaylandRole::ShellClient,
                WaylandRole::ProtocolExtensions,
            ],
        }
    }
}

/// Acorn's Wayland bootstrap status.
#[derive(Debug)]
pub enum BootstrapStatus {
    /// The shell connected to an existing Wayland compositor for client-mode testing.
    ConnectedClient(wayland_client::Connection),
    /// No Wayland server was available in the current environment.
    MissingDisplay(wayland_client::ConnectError),
}

/// Attempt to connect a shell client to the ambient Wayland display.
///
/// This is useful for early NutBar, Dock, and Spotlight development before the
/// full Acorn compositor is ready to host applications itself.
pub fn connect_shell_client() -> BootstrapStatus {
    match wayland_client::Connection::connect_to_env() {
        Ok(connection) => BootstrapStatus::ConnectedClient(connection),
        Err(error) => BootstrapStatus::MissingDisplay(error),
    }
}

/// Create a Wayland display for compositor-side integration tests and future app hosting.
#[must_use]
pub fn create_server_display<State>() -> wayland_server::Display<State> {
    wayland_server::Display::new().expect("Wayland display allocation should succeed")
}
