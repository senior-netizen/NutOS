# 🧱 NutOS Hardware Support Strategy
*Rodent, Inc. | SquirrelLabs Technologies*

---

## Overview

NutOS is designed to run on widely available hardware without requiring premium vendor lock-in. Hardware strategy balances broad compatibility with predictable quality.

---

## Architecture Targets

| Architecture | Priority | Notes |
|---|---|---|
| x86-64 | Primary | Desktop/laptop baseline |
| ARM64 | Primary | Modern ultrabooks, SBCs, and mobile-class chips |
| RISC-V | Planned | Incremental enablement in later phases |
| 32-bit ARM/x86 | Not planned | Legacy-only, outside v1 support scope |

---

## Compatibility Principles

1. Prefer upstream Linux driver alignment whenever possible.
2. Maintain stable kernel ABI expectations across release trains.
3. Ship clear fallback behavior for partially supported components.
4. Publish compatibility status transparently to users.

---

## Certification Program (Draft)

### Tier A — NutOS Certified
- Full graphics acceleration
- Reliable suspend/resume
- Stable Wi-Fi/Bluetooth/audio/camera
- Secure Boot + TPM validated

### Tier B — NutOS Compatible
- Core functionality works
- Minor caveats documented
- No critical data integrity or boot issues

### Tier C — Community Supported
- Best-effort support maintained by contributors
- No official SLA from Rodent, Inc.

---

## Driver & Firmware Policy

- Prefer open drivers when they meet quality/performance requirements.
- Permit proprietary firmware blobs where unavoidable for user functionality.
- Document firmware dependencies and update channels clearly.

---

## Hardware QA Matrix

Validation runs should include:
- Cold boot / warm reboot reliability
- Sleep/hibernate cycles
- Multi-display + high-DPI behavior
- Battery drain and thermal behavior
- GPU stress and video decode/encode checks
- Filesystem integrity under power-loss simulation

---

## Release Gates

No stable release should ship without:
1. Passing Tier A reference-device test suite
2. Regression scan on previous stable hardware matrix
3. Documented known issues and mitigations

---

*Document maintained by: Rodent, Inc. Hardware Team*
*Review cycle: Quarterly*
*Classification: Internal — Technical*
