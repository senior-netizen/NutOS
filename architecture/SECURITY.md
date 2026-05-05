# 🛡️ NutOS Security & Privacy Architecture
*Rodent, Inc. | SquirrelLabs Technologies*

---

## Overview

Security and privacy are not features of NutOS. They are the foundation. Every architectural decision is evaluated through a security lens first.

NutOS's security model is built on three principles:
1. **Zero trust** — nothing is trusted by default, including the OS itself
2. **Least privilege** — every process runs with the minimum permissions it needs
3. **Transparency** — all security mechanisms are auditable by the user

---

## Threat Model

NutOS is designed to protect against:

| Threat | Severity | Mitigation |
|---|---|---|
| Malicious software | Critical | App sandboxing, signature verification |
| Data exfiltration | Critical | Network firewall, permission model |
| Remote exploitation | Critical | Attack surface minimization, fast patching |
| Physical access attacks | High | Full-disk encryption, secure boot |
| Supply chain attacks | High | Reproducible builds, signed packages |
| Telemetry / surveillance | High | Zero telemetry, network transparency |
| Browser-based attacks | Medium | Browser isolation, DNS filtering |
| Social engineering | Medium | Clear permission prompts, user education |

---

## App Sandbox (NutCage)

Every application on NutOS runs inside **NutCage** — a mandatory sandboxing system.

### Sandbox Tiers

| Tier | Access | Used For |
|---|---|---|
| **Isolated** | No filesystem, no network | Simple utilities |
| **Standard** | Home folder only, opt-in network | Most user apps |
| **Extended** | User-granted paths + network | Creative/professional apps |
| **System** | Full access (requires user password + justification) | System tools only |

### Permission Model

Apps must declare required permissions in their manifest. Users see a clear prompt:

```
┌─────────────────────────────────────────────────┐
│  📷 "Camera App" wants access to:               │
│                                                 │
│  ✅ Camera                                      │
│  ✅ ~/Pictures folder                           │
│  ⚠️  Your location (tap to see why)             │
│  ❌ Microphone (you can deny this)              │
│                                                 │
│  [Deny Location & Microphone]  [Allow All]      │
└─────────────────────────────────────────────────┘
```

Permissions can be:
- Granted permanently
- Granted for the current session only
- Denied permanently
- Denied with the app still functional (if permission is non-essential)

### NutCage Technical Implementation

- Built on Linux **namespaces**, **seccomp**, and **cgroups**
- Network access controlled by per-app **nftables** rules
- Filesystem access via **OverlayFS** — apps see a virtual view of the filesystem
- IPC (Inter-Process Communication) is mediated — apps cannot freely communicate
- App data is stored encrypted in isolated per-app containers

---

## Full Disk Encryption

NutOS enables full disk encryption by default during installation using **LUKS2**:

- **Algorithm:** AES-256-XTS
- **Key derivation:** Argon2id (memory-hard, resistant to GPU cracking)
- **TPM integration:** Unlock automatically on trusted hardware without a password
- **Recovery key:** Generated at setup, displayed once, stored nowhere by the OS
- **Per-user home encryption:** Each user's home directory has an additional encryption layer

---

## Secure Boot

NutOS supports and encourages Secure Boot:

- NutOS bootloader and kernel are signed with SquirrelLabs' keys
- Users can enroll their own keys (full user control)
- The system will warn (not block) if Secure Boot is disabled
- **Measured Boot** logs the boot state into the TPM for remote attestation

---

## Network Privacy

### Firewall (NutWall)
- **Deny-all outbound by default** for apps — each app must be explicitly allowed
- **Deny-all inbound by default** — ports must be explicitly opened
- User-friendly GUI for managing rules (no iptables knowledge required)
- Per-app network rules visible in a dashboard

### DNS
- DNS-over-HTTPS (DoH) enabled by default using **Cloudflare 1.1.1.1** and **Quad9**
- Users can configure any DoH provider or self-host
- DNS queries are never sent to ISP DNS by default

### Connectivity Checks
- NutOS does NOT perform connectivity checks that phone home to SquirrelLabs
- If a connectivity check is needed, it pings a user-selected neutral endpoint

### Anti-Fingerprinting
- MAC address randomization per network connection (kernel-level)
- Hostname randomization on public networks
- Timezone is not exposed to websites without user consent

---

## Update Security (Harvest)

The **Harvest** update system ensures updates cannot be tampered with:

- All updates are signed with SquirrelLabs' GPG key
- Updates are delivered over HTTPS with certificate pinning
- **Reproducible builds** — anyone can verify that the binary matches the source
- Updates are applied to a **snapshot** — if an update fails, rollback is automatic
- Critical security patches are delivered out-of-band (not waiting for a scheduled update)

### Update Transparency Log

Every update delivered to any NutOS device is logged in a public **Certificate Transparency**-style log. This means:
- You can verify your device received the same update as everyone else
- No targeted malicious updates can be silently delivered to specific users
- The log is append-only and publicly auditable

---

## Telemetry Policy

**NutOS collects zero data by default.**

The only data-related options:
1. **Crash reports** — opt-in, anonymized, sent only when you choose "Send Report"
2. **App analytics** — entirely up to individual app developers, clearly disclosed
3. **Hardware telemetry** — never collected by NutOS

SquirrelLabs Technologies commits to:
- Never selling any user data
- Never collecting behavioral data
- Publishing an annual transparency report
- Accepting independent security audits

---

## Security Response Process

When a vulnerability is discovered:

1. **Report received** at security@squirrellabs.tech (PGP key published)
2. **Triage within 24 hours** by the Rodent, Inc. security team
3. **Patch developed** within 7 days for critical issues
4. **Silent patch deployed** to all devices via Harvest
5. **CVE published** after 90% of devices have received the patch
6. **Post-mortem published** publicly within 30 days

---

## Compliance

NutOS is designed to be compliant with:

| Standard | Status |
|---|---|
| GDPR (EU) | ✅ Compliant by design |
| CCPA (California) | ✅ Compliant |
| ISO 27001 | 🔧 Targeting certification by v1.0 |
| SOC 2 Type II | 🔧 Targeting Year 2 |
| FedRAMP | 🔧 Long-term goal (NutOS Enterprise) |

---

*Document maintained by: Rodent, Inc. Security Team*
*Classification: Internal — Confidential*
*Review cycle: Quarterly*
