# NutKernel

NutKernel is the NutOS kernel integration workspace. It starts as Rust scaffolding around the modified Linux kernel plan described in [`../architecture/KERNEL.md`](../architecture/KERNEL.md), with module boundaries for hardening, privacy, performance tuning, custom syscalls, memory management, and the HAL.

## Layout

```text
NutKernel/
├── Cargo.toml              # Rust crate metadata and lint policy
├── README.md               # This overview
├── configs/                # Per-architecture kernel configuration fragments
├── patches/linux/          # Linux patch queue grouped by NutOS subsystem
├── scripts/                # Build, patch, and validation helpers
├── src/
│   ├── arch/               # x86-64, ARM64, and RISC-V target metadata
│   ├── hal/                # NutOS hardware abstraction boundaries
│   ├── memory/             # zram, OOM policy, sandbox memory limits
│   ├── performance/        # BORE scheduler, zstd, huge pages, power policy
│   ├── privacy/            # Fingerprinting, MAC randomization, log redaction
│   ├── security/           # KASLR, SMEP/SMAP, module signing, proc/sys policy
│   ├── syscalls/           # NutOS syscall registry and ABI metadata
│   └── lib.rs              # Public crate surface
└── tests/                  # Integration tests for crate-level invariants
```

This crate does not vendor Linux yet. The `patches/linux/` and `configs/` directories are placeholders for the eventual patch queue and architecture-specific kernel config fragments.
