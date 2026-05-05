# ⚙️ NutOS Kernel Architecture
*Rodent, Inc. | SquirrelLabs Technologies*

---

## Overview

NutOS is built on a **modified Linux kernel** — but that is where the similarity to a typical Linux distro ends. The kernel serves as the foundation only; everything above it is purpose-built by Rodent, Inc.

The decision to use Linux as the base is pragmatic:
- Mature, battle-tested codebase with 30+ years of development
- Excellent hardware driver ecosystem
- Active security patching by thousands of contributors
- Proven stability at scale (powers 90%+ of the world's servers)

However, NutOS applies significant modifications, hardening, and custom layers on top.

---

## Kernel Layer Stack

```
┌─────────────────────────────────────────────────┐
│              User Applications                  │
├─────────────────────────────────────────────────┤
│         NutShell Compatibility Layer            │  ← Custom
├─────────────────────────────────────────────────┤
│         Acorn Desktop Environment               │  ← Custom
├─────────────────────────────────────────────────┤
│       NutOS System Services & Daemons           │  ← Custom
├─────────────────────────────────────────────────┤
│        NutOS Security & Privacy Layer           │  ← Custom (hardened)
├─────────────────────────────────────────────────┤
│     NutOS Hardware Abstraction Layer (HAL)      │  ← Custom
├─────────────────────────────────────────────────┤
│     Modified Linux Kernel (NutKernel)           │  ← Linux base + modifications
├─────────────────────────────────────────────────┤
│           Hardware (x86-64 / ARM64 / RISC-V)    │
└─────────────────────────────────────────────────┘
```

---

## NutKernel — Linux Modifications

The NutOS fork of the Linux kernel, called **NutKernel**, includes the following modifications:

### Security Hardening
- **KASLR** (Kernel Address Space Layout Randomization) enabled by default
- **SMEP/SMAP** enforced on all x86-64 builds
- Integration of **grsecurity-inspired** hardening patches
- All kernel modules must be signed
- Restricted `/proc` and `/sys` access for unprivileged processes

### Privacy Modifications
- Network stack patched to prevent passive OS fingerprinting
- Randomized MAC addresses at the kernel level by default
- All kernel logs stripped of hardware identifiers before user-space access
- Disabled covert channel vectors (shared CPU caches, timing attacks mitigated)

### Performance Tuning
- **BORE scheduler** (Burst-Oriented Response Enhancer) for better desktop responsiveness
- **zstd** compression for memory and storage operations
- Transparent **huge pages** enabled by default for performance
- Aggressive **power management** for laptops (better than stock Linux)

### Custom System Calls
NutOS adds a small set of custom system calls to support NutShell and Acorn:

| Syscall | Purpose |
|---|---|
| `nut_sandbox_create()` | Creates a sandboxed execution environment |
| `nut_privacy_fence()` | Marks memory regions as privacy-protected |
| `nut_compat_exec()` | Launches a binary through the NutShell compatibility layer |
| `nut_ui_accelerate()` | Hints to the kernel for GPU-accelerated UI rendering |

---

## Init System: NutInit

NutOS replaces systemd with **NutInit**, a purpose-built init system designed for:

- **Faster boot times** — parallel service startup with dependency resolution
- **Simpler configuration** — human-readable TOML config files instead of complex unit files
- **Better error reporting** — clear, plain-language boot failure messages
- **Sandboxing from boot** — services start sandboxed by default

```toml
# Example NutInit service file: /etc/nutinit/services/networking.toml
[service]
name = "NutOS Networking"
binary = "/usr/lib/nut/netd"
sandboxed = true
restart_on_failure = true
depends_on = ["dbus", "kernel-drivers"]
priority = "high"
```

---

## Filesystem

NutOS uses **NutFS** as its default filesystem, built on top of **btrfs**:

| Feature | Detail |
|---|---|
| Default FS | NutFS (btrfs-based) |
| Snapshots | Automatic snapshots before every system update |
| Encryption | Full-disk encryption enabled by default (AES-256-XTS) |
| Compression | zstd transparent compression |
| Deduplication | Background deduplication for storage efficiency |
| Rollback | One-click system rollback via Harvest (update manager) |

### Directory Structure

```
/
├── boot/          — Bootloader and kernel images
├── etc/           — System configuration (TOML-based)
├── home/          — User home directories (encrypted per-user)
├── nut/           — NutOS system directory (replaces /usr)
│   ├── apps/      — Installed applications (sandboxed)
│   ├── compat/    — NutShell compatibility environments
│   ├── lib/       — System libraries
│   └── services/  — System services
├── tmp/           — Temporary files (RAM-based, wiped on boot)
└── var/           — Variable data (logs, caches)
```

---

## Memory Management

- **Zram** enabled by default for compressed swap in RAM
- **OOM (Out of Memory) killer** replaced with a smarter priority-aware version
- App sandbox memory limits enforced at the kernel level
- Memory-safe allocator (**mimalloc**) used for all NutOS userspace components

---

## Target Architecture Support

| Architecture | Status | Notes |
|---|---|---|
| x86-64 | ✅ Primary | Full support from day one |
| ARM64 | ✅ Primary | Including Apple Silicon (via VM initially) |
| RISC-V | 🔧 Planned | Phase 3 target |
| ARM 32-bit | ❌ Not planned | Too legacy for our target |

---

*Document maintained by: Rodent, Inc. Kernel Team*
*Classification: Internal — Technical*
