# Linux patch queue

This directory will contain NutKernel's Linux patch series, grouped by subsystem:

- `security/` for KASLR defaults, SMEP/SMAP enforcement, module signing, and `/proc`/`/sys` restrictions
- `privacy/` for network fingerprinting resistance, MAC randomization, log redaction, and covert-channel mitigations
- `performance/` for BORE scheduling, zstd defaults, huge pages, and power management
- `syscalls/` for the NutOS-specific syscall table and implementation patches
