#![allow(missing_docs)]

use nutkernel::arch::{SupportStatus, TARGETS};
use nutkernel::memory::MemoryPolicy;
use nutkernel::performance::PerformanceTuning;
use nutkernel::privacy::PrivacyControls;
use nutkernel::security::{
    AppManifest, NutCagePolicy, NutCagePrompt, PermissionDecision, PermissionDeclaration,
    PermissionGrant, PermissionKind, PromptDecision, PromptTreatment, SandboxTier,
    SecurityHardening,
};
use nutkernel::syscalls::{NutSyscall, REGISTRY};

#[test]
fn tracks_kernel_architecture_targets() {
    assert_eq!(TARGETS.len(), 3);
    assert_eq!(TARGETS[0].name, "x86_64");
    assert_eq!(TARGETS[0].status, SupportStatus::Primary);
    assert_eq!(TARGETS[1].name, "aarch64");
    assert_eq!(TARGETS[1].status, SupportStatus::Primary);
    assert_eq!(TARGETS[2].name, "riscv64");
    assert_eq!(TARGETS[2].status, SupportStatus::Planned);
}

#[test]
fn baseline_profiles_enable_architecture_requirements() {
    let security = SecurityHardening::baseline();
    assert!(security.kaslr);
    assert!(security.smep);
    assert!(security.smap);
    assert!(security.signed_modules);
    assert!(security.restricted_proc_sys);

    let privacy = PrivacyControls::baseline();
    assert!(privacy.passive_fingerprinting_resistance);
    assert!(privacy.randomized_mac_addresses);
    assert!(privacy.redact_hardware_identifiers);
    assert!(privacy.covert_channel_mitigations);

    let performance = PerformanceTuning::desktop_default();
    assert!(performance.bore_scheduler);
    assert!(performance.zstd_compression);
    assert!(performance.transparent_huge_pages);
    assert!(performance.laptop_power_management);

    let memory = MemoryPolicy::baseline();
    assert!(memory.zram);
    assert!(memory.priority_aware_oom);
    assert!(memory.sandbox_memory_limits);
}

#[test]
fn registers_initial_nutos_syscalls() {
    assert_eq!(REGISTRY.len(), 4);
    assert_eq!(NutSyscall::SandboxCreate.symbol(), "nut_sandbox_create");
    assert_eq!(NutSyscall::PrivacyFence.symbol(), "nut_privacy_fence");
    assert_eq!(NutSyscall::CompatExec.symbol(), "nut_compat_exec");
    assert_eq!(NutSyscall::UiAccelerate.symbol(), "nut_ui_accelerate");
}

#[test]
fn nutcage_prompt_reflects_security_architecture() {
    static PERMISSIONS: &[PermissionDeclaration] = &[
        PermissionDeclaration::essential(PermissionKind::Camera, None),
        PermissionDeclaration::essential(
            PermissionKind::PicturesFolder,
            Some("Save captured images to the user's photo library"),
        ),
        PermissionDeclaration::essential(
            PermissionKind::Location,
            Some("Tag images with where they were captured"),
        ),
        PermissionDeclaration::optional(
            PermissionKind::Microphone,
            Some("Record ambient audio with video clips"),
        ),
    ];
    let manifest = AppManifest::new("Camera App", SandboxTier::Standard, PERMISSIONS);

    let prompt = NutCagePrompt::from_manifest(&manifest);
    let rows: Vec<_> = prompt.rows().collect();

    assert_eq!(prompt.app_name, "Camera App");
    assert_eq!(prompt.tier, SandboxTier::Standard);
    assert!(!prompt.requires_password);
    assert_eq!(prompt.len(), 4);
    assert_eq!(rows[0].label, "Camera");
    assert_eq!(rows[0].treatment, PromptTreatment::Required);
    assert_eq!(rows[0].treatment.marker(), "✅");
    assert_eq!(rows[1].label, "~/Pictures folder");
    assert_eq!(rows[1].treatment, PromptTreatment::Required);
    assert_eq!(rows[2].label, "Your location");
    assert_eq!(rows[2].treatment, PromptTreatment::Sensitive);
    assert_eq!(rows[2].treatment.marker(), "⚠️");
    assert_eq!(rows[3].label, "Microphone");
    assert_eq!(rows[3].treatment, PromptTreatment::Deniable);
    assert_eq!(rows[3].treatment.marker(), "❌");
}

#[test]
fn nutcage_policy_allows_optional_denials_but_blocks_essential_denials() {
    static PERMISSIONS: &[PermissionDeclaration] = &[
        PermissionDeclaration::essential(PermissionKind::Camera, None),
        PermissionDeclaration::optional(PermissionKind::Microphone, None),
    ];
    let manifest = AppManifest::new("Camera App", SandboxTier::Standard, PERMISSIONS);

    let optional_denied = [
        PermissionDecision::new(PermissionKind::Camera, PermissionGrant::GrantedForSession),
        PermissionDecision::new(PermissionKind::Microphone, PermissionGrant::DeniedOptional),
    ];
    assert_eq!(
        NutCagePolicy::evaluate(&manifest, &optional_denied, false),
        PromptDecision::AllowLaunch
    );

    let essential_denied = [PermissionDecision::new(
        PermissionKind::Camera,
        PermissionGrant::DeniedPermanently,
    )];
    assert_eq!(
        NutCagePolicy::evaluate(&manifest, &essential_denied, false),
        PromptDecision::BlockedEssentialDenied(PermissionKind::Camera)
    );
}

#[test]
fn nutcage_enforces_tier_and_system_prompt_requirements() {
    static PATH_PERMISSIONS: &[PermissionDeclaration] = &[PermissionDeclaration::essential(
        PermissionKind::UserPath("~/Projects"),
        Some("Open user-selected project files"),
    )];
    static SYSTEM_PERMISSIONS: &[PermissionDeclaration] = &[PermissionDeclaration::essential(
        PermissionKind::SystemAccess,
        Some("Repair signed system packages"),
    )];

    let standard_manifest = AppManifest::new("Editor", SandboxTier::Standard, PATH_PERMISSIONS);
    assert_eq!(
        NutCagePolicy::evaluate(
            &standard_manifest,
            &[PermissionDecision::new(
                PermissionKind::UserPath("~/Projects"),
                PermissionGrant::GrantedPermanently,
            )],
            false,
        ),
        PromptDecision::BlockedTierViolation(PermissionKind::UserPath("~/Projects"))
    );

    let system_manifest = AppManifest::system(
        "Harvest Repair",
        SYSTEM_PERMISSIONS,
        "Needs full access to restore signed packages",
    );
    assert!(NutCagePrompt::from_manifest(&system_manifest).requires_password);
    assert_eq!(
        NutCagePolicy::evaluate(
            &system_manifest,
            &[PermissionDecision::new(
                PermissionKind::SystemAccess,
                PermissionGrant::GrantedForSession,
            )],
            false,
        ),
        PromptDecision::BlockedSystemConfirmationRequired
    );
    assert_eq!(
        NutCagePolicy::evaluate(
            &system_manifest,
            &[PermissionDecision::new(
                PermissionKind::SystemAccess,
                PermissionGrant::GrantedForSession,
            )],
            true,
        ),
        PromptDecision::AllowLaunch
    );
}
