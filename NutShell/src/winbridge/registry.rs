//! Windows Registry to NutOS config-store declarations.

/// Registry hives recognized by the skeleton translator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistryHive {
    /// `HKEY_CURRENT_USER`.
    CurrentUser,
    /// `HKEY_LOCAL_MACHINE`.
    LocalMachine,
    /// `HKEY_CLASSES_ROOT`.
    ClassesRoot,
}

/// NutOS config-store namespace used for a registry hive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigNamespace {
    /// Per-user application configuration.
    User,
    /// System-wide application configuration.
    System,
    /// File and COM-style association metadata.
    Associations,
}

/// Planned registry translation route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RegistryTranslation {
    /// Source registry hive.
    pub hive: RegistryHive,
    /// Target NutOS config namespace.
    pub namespace: ConfigNamespace,
}

impl RegistryTranslation {
    /// Return the NutOS config namespace for a registry hive.
    #[must_use]
    pub const fn for_hive(hive: RegistryHive) -> Self {
        let namespace = match hive {
            RegistryHive::CurrentUser => ConfigNamespace::User,
            RegistryHive::LocalMachine => ConfigNamespace::System,
            RegistryHive::ClassesRoot => ConfigNamespace::Associations,
        };

        Self { hive, namespace }
    }
}
