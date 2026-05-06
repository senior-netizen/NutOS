//! Security hardening switches for the NutKernel Linux patch set.

/// Required security controls from the NutKernel architecture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecurityHardening {
    /// Kernel Address Space Layout Randomization.
    pub kaslr: bool,
    /// Supervisor Mode Execution Prevention on x86-64.
    pub smep: bool,
    /// Supervisor Mode Access Prevention on x86-64.
    pub smap: bool,
    /// Require signatures for all loadable kernel modules.
    pub signed_modules: bool,
    /// Restrict unprivileged access to `/proc` and `/sys`.
    pub restricted_proc_sys: bool,
}

impl SecurityHardening {
    /// Baseline hardening profile mandated for NutKernel builds.
    #[must_use]
    pub const fn baseline() -> Self {
        Self {
            kaslr: true,
            smep: true,
            smap: true,
            signed_modules: true,
            restricted_proc_sys: true,
        }
    }
}

/// NutCage sandbox tiers exposed to applications.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SandboxTier {
    /// No filesystem access and no network access.
    Isolated,
    /// Home-folder access with opt-in network access.
    Standard,
    /// User-granted paths and network access for larger creative/pro apps.
    Extended,
    /// Full system access; restricted to trusted system tools.
    System,
}

impl SandboxTier {
    /// Whether the tier can request arbitrary user-granted paths.
    #[must_use]
    pub const fn supports_user_granted_paths(self) -> bool {
        matches!(self, Self::Extended | Self::System)
    }

    /// Whether the tier requires an elevated user password confirmation before launch.
    #[must_use]
    pub const fn requires_password(self) -> bool {
        matches!(self, Self::System)
    }
}

/// Permission categories that can be declared in an application manifest.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PermissionKind {
    /// Access to a camera device.
    Camera,
    /// Access to a microphone device.
    Microphone,
    /// Access to location services.
    Location,
    /// Outbound network access mediated by NutWall.
    Network,
    /// Access to the user's home folder.
    HomeFolder,
    /// Access to the user's pictures folder.
    PicturesFolder,
    /// Access to a user-selected path.
    UserPath(&'static str),
    /// Full system access for system tools.
    SystemAccess,
}

impl PermissionKind {
    /// Human-readable label for permission prompts.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Camera => "Camera",
            Self::Microphone => "Microphone",
            Self::Location => "Your location",
            Self::Network => "Network access",
            Self::HomeFolder => "~/ folder",
            Self::PicturesFolder => "~/Pictures folder",
            Self::UserPath(path) => path,
            Self::SystemAccess => "Full system access",
        }
    }

    /// Whether users should be shown an expanded rationale affordance by default.
    #[must_use]
    pub const fn is_sensitive(self) -> bool {
        matches!(self, Self::Location | Self::SystemAccess | Self::Microphone)
    }
}

/// Whether a permission is required for core app functionality.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PermissionRequirement {
    /// Denying the permission prevents launching the sandbox profile.
    Essential,
    /// Denying the permission still allows the app to run with degraded functionality.
    Optional,
}

impl PermissionRequirement {
    /// Whether denied access should be treated as launch-blocking.
    #[must_use]
    pub const fn is_essential(self) -> bool {
        matches!(self, Self::Essential)
    }
}

/// A single permission entry declared by an app manifest.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PermissionDeclaration {
    /// Permission category requested by the app.
    pub kind: PermissionKind,
    /// Whether the app considers this permission essential.
    pub requirement: PermissionRequirement,
    /// Short explanation surfaced by the prompt when the user asks why.
    pub rationale: Option<&'static str>,
}

impl PermissionDeclaration {
    /// Construct an essential permission declaration.
    #[must_use]
    pub const fn essential(kind: PermissionKind, rationale: Option<&'static str>) -> Self {
        Self {
            kind,
            requirement: PermissionRequirement::Essential,
            rationale,
        }
    }

    /// Construct an optional permission declaration.
    #[must_use]
    pub const fn optional(kind: PermissionKind, rationale: Option<&'static str>) -> Self {
        Self {
            kind,
            requirement: PermissionRequirement::Optional,
            rationale,
        }
    }

    /// Prompt treatment implied by the declaration.
    #[must_use]
    pub const fn prompt_treatment(self) -> PromptTreatment {
        if matches!(self.requirement, PermissionRequirement::Optional) {
            PromptTreatment::Deniable
        } else if self.kind.is_sensitive() {
            PromptTreatment::Sensitive
        } else {
            PromptTreatment::Required
        }
    }
}

/// App manifest fields needed by NutCage before sandbox creation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AppManifest {
    /// User-facing application name.
    pub app_name: &'static str,
    /// Requested sandbox tier.
    pub tier: SandboxTier,
    /// Declared permission list.
    pub permissions: &'static [PermissionDeclaration],
    /// Required justification for system-tier tools.
    pub system_justification: Option<&'static str>,
}

impl AppManifest {
    /// Build a manifest for a non-system app.
    #[must_use]
    pub const fn new(
        app_name: &'static str,
        tier: SandboxTier,
        permissions: &'static [PermissionDeclaration],
    ) -> Self {
        Self {
            app_name,
            tier,
            permissions,
            system_justification: None,
        }
    }

    /// Build a manifest for a system-tier app with a user-visible justification.
    #[must_use]
    pub const fn system(
        app_name: &'static str,
        permissions: &'static [PermissionDeclaration],
        justification: &'static str,
    ) -> Self {
        Self {
            app_name,
            tier: SandboxTier::System,
            permissions,
            system_justification: Some(justification),
        }
    }
}

/// Visual/semantic treatment for a permission prompt row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PromptTreatment {
    /// Required access, shown as a normal requested permission.
    Required,
    /// Sensitive access, shown with an explanation affordance.
    Sensitive,
    /// Optional access that can be denied without blocking app launch.
    Deniable,
}

impl PromptTreatment {
    /// Prompt marker matching the architecture's permission prompt language.
    #[must_use]
    pub const fn marker(self) -> &'static str {
        match self {
            Self::Required => "✅",
            Self::Sensitive => "⚠️",
            Self::Deniable => "❌",
        }
    }
}

/// A prompt row generated from a manifest permission declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PromptRow {
    /// Permission kind represented by this row.
    pub kind: PermissionKind,
    /// User-facing label.
    pub label: &'static str,
    /// Prompt treatment and marker.
    pub treatment: PromptTreatment,
    /// Explanation to show when the user expands the row.
    pub rationale: Option<&'static str>,
}

/// Iterator over prompt rows without heap allocation.
pub struct PromptRows<'a> {
    permissions: core::slice::Iter<'a, PermissionDeclaration>,
}

impl Iterator for PromptRows<'_> {
    type Item = PromptRow;

    fn next(&mut self) -> Option<Self::Item> {
        self.permissions.next().map(|permission| PromptRow {
            kind: permission.kind,
            label: permission.kind.label(),
            treatment: permission.prompt_treatment(),
            rationale: permission.rationale,
        })
    }
}

/// NutCage permission prompt generated from an application manifest.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NutCagePrompt<'a> {
    /// User-facing application name.
    pub app_name: &'a str,
    /// Requested sandbox tier.
    pub tier: SandboxTier,
    /// Whether this prompt must collect a password before allowing launch.
    pub requires_password: bool,
    /// System-tier justification, if one was provided.
    pub system_justification: Option<&'a str>,
    permissions: &'a [PermissionDeclaration],
}

impl<'a> NutCagePrompt<'a> {
    /// Generate a NutCage prompt from an app manifest.
    #[must_use]
    pub const fn from_manifest(manifest: &'a AppManifest) -> Self {
        Self {
            app_name: manifest.app_name,
            tier: manifest.tier,
            requires_password: manifest.tier.requires_password(),
            system_justification: manifest.system_justification,
            permissions: manifest.permissions,
        }
    }

    /// Iterate over prompt rows.
    #[must_use]
    pub fn rows(self) -> PromptRows<'a> {
        PromptRows {
            permissions: self.permissions.iter(),
        }
    }

    /// Number of permissions displayed by the prompt.
    #[must_use]
    pub const fn len(self) -> usize {
        self.permissions.len()
    }

    /// Whether the prompt has no requested permissions.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.permissions.is_empty()
    }
}

/// User decision for a permission request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PermissionGrant {
    /// Persist the grant across launches.
    GrantedPermanently,
    /// Grant access until the current app session ends.
    GrantedForSession,
    /// Persist the denial across launches.
    DeniedPermanently,
    /// Deny an optional permission while allowing degraded app functionality.
    DeniedOptional,
}

impl PermissionGrant {
    /// Whether the permission is available to the sandbox after prompting.
    #[must_use]
    pub const fn is_granted(self) -> bool {
        matches!(self, Self::GrantedPermanently | Self::GrantedForSession)
    }
}

/// User decision bound to a permission kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PermissionDecision {
    /// Permission category the user decided on.
    pub kind: PermissionKind,
    /// Grant or denial selected by the user.
    pub grant: PermissionGrant,
}

impl PermissionDecision {
    /// Construct a permission decision.
    #[must_use]
    pub const fn new(kind: PermissionKind, grant: PermissionGrant) -> Self {
        Self { kind, grant }
    }
}

/// Result of applying permission decisions to an app manifest.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PromptDecision {
    /// All launch-blocking permissions are granted.
    AllowLaunch,
    /// An essential permission was denied.
    BlockedEssentialDenied(PermissionKind),
    /// System-tier apps require both password confirmation and justification.
    BlockedSystemConfirmationRequired,
    /// Manifest requested access that is not allowed by the selected tier.
    BlockedTierViolation(PermissionKind),
}

/// Evaluates NutCage permission prompts and tier constraints.
pub struct NutCagePolicy;

impl NutCagePolicy {
    /// Evaluate a manifest and user decisions before creating the sandbox.
    #[must_use]
    pub fn evaluate(
        manifest: &AppManifest,
        decisions: &[PermissionDecision],
        password_confirmed: bool,
    ) -> PromptDecision {
        if manifest.tier.requires_password()
            && (!password_confirmed || manifest.system_justification.is_none())
        {
            return PromptDecision::BlockedSystemConfirmationRequired;
        }

        for permission in manifest.permissions {
            if !Self::tier_allows(manifest.tier, permission.kind) {
                return PromptDecision::BlockedTierViolation(permission.kind);
            }

            let decision = Self::decision_for(permission.kind, decisions);
            if permission.requirement.is_essential() && !decision.is_granted() {
                return PromptDecision::BlockedEssentialDenied(permission.kind);
            }
        }

        PromptDecision::AllowLaunch
    }

    const fn tier_allows(tier: SandboxTier, kind: PermissionKind) -> bool {
        match tier {
            SandboxTier::Isolated => false,
            SandboxTier::Standard => matches!(
                kind,
                PermissionKind::Network
                    | PermissionKind::HomeFolder
                    | PermissionKind::PicturesFolder
                    | PermissionKind::Camera
                    | PermissionKind::Microphone
                    | PermissionKind::Location
            ),
            SandboxTier::Extended => !matches!(kind, PermissionKind::SystemAccess),
            SandboxTier::System => true,
        }
    }

    fn decision_for(kind: PermissionKind, decisions: &[PermissionDecision]) -> PermissionGrant {
        decisions
            .iter()
            .find(|decision| decision.kind == kind)
            .map_or(PermissionGrant::DeniedPermanently, |decision| {
                decision.grant
            })
    }
}
