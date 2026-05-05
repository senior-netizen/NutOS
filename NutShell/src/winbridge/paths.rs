//! NTFS path to NutFS path translation declarations.

/// Windows path families handled by the skeleton translator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowsPathKind {
    /// Drive-letter paths such as `C:\Program Files`.
    DriveLetter,
    /// User profile paths.
    UserProfile,
    /// UNC network-share paths.
    UncShare,
}

/// NutFS mount or namespace selected for a Windows path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NutFsTarget {
    /// Per-prefix virtual drive mount.
    PrefixDrive,
    /// NutOS user home namespace.
    Home,
    /// Network filesystem mount namespace.
    NetworkMount,
}

/// Planned path translation route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PathTranslation {
    /// Source Windows path family.
    pub source: WindowsPathKind,
    /// Target NutFS location family.
    pub target: NutFsTarget,
}

impl PathTranslation {
    /// Return the NutFS target for a Windows path family.
    #[must_use]
    pub const fn for_kind(source: WindowsPathKind) -> Self {
        let target = match source {
            WindowsPathKind::DriveLetter => NutFsTarget::PrefixDrive,
            WindowsPathKind::UserProfile => NutFsTarget::Home,
            WindowsPathKind::UncShare => NutFsTarget::NetworkMount,
        };

        Self { source, target }
    }
}
