#![allow(missing_docs)]

use nutkernel::arch::{SupportStatus, TARGETS};
use nutkernel::memory::MemoryPolicy;
use nutkernel::performance::PerformanceTuning;
use nutkernel::privacy::PrivacyControls;
use nutkernel::security::SecurityHardening;
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
