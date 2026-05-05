//! Win32-to-NutOS POSIX translation declarations.

/// Coarse Win32 API families handled by the skeleton translator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Win32Call {
    /// File and directory APIs.
    FileSystem,
    /// Process and thread management APIs.
    Process,
    /// Windowing and message-loop APIs.
    UserInterface,
    /// Socket and network APIs.
    Networking,
}

/// POSIX-facing NutOS operation family selected for a Win32 call.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PosixEquivalent {
    /// POSIX file descriptor and path operations.
    FileDescriptors,
    /// POSIX process, thread, and signal operations.
    ProcessesAndThreads,
    /// Acorn desktop window/session bridge.
    DesktopSession,
    /// POSIX socket operations.
    Sockets,
}

/// Planned Win32 translation route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Win32Translation {
    /// Source Win32 call family.
    pub call: Win32Call,
    /// NutOS POSIX-facing target.
    pub target: PosixEquivalent,
}

impl Win32Translation {
    /// Return the initial POSIX equivalent for a Win32 API family.
    #[must_use]
    pub const fn for_call(call: Win32Call) -> Self {
        let target = match call {
            Win32Call::FileSystem => PosixEquivalent::FileDescriptors,
            Win32Call::Process => PosixEquivalent::ProcessesAndThreads,
            Win32Call::UserInterface => PosixEquivalent::DesktopSession,
            Win32Call::Networking => PosixEquivalent::Sockets,
        };

        Self { call, target }
    }
}
