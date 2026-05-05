# NutShell

NutShell is the NutOS universal compatibility layer workspace. This crate starts
with Rust scaffolding for the WinBridge API translation layer described in
[`../architecture/NUTSHELL.md`](../architecture/NUTSHELL.md).

## Layout

```text
NutShell/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   └── winbridge/
│       ├── database.rs   # Compatibility database metadata
│       ├── directx.rs    # DirectX → Vulkan support matrix
│       ├── paths.rs      # NTFS/Windows paths → NutFS routing
│       ├── registry.rs   # Registry → NutOS config-store routing
│       ├── services.rs   # Windows services → NutOS services routing
│       ├── translator.rs # WinBridge translation coordinator
│       ├── win32.rs      # Win32 API → POSIX equivalents
│       └── winrt.rs      # NutOS WinRT shim routing
└── tests/
    └── winbridge_contract.rs
```

The crate is intentionally metadata-first: it defines the architecture contract
and translation boundaries before binding to Wine, Proton, DXVK, VKD3D-Proton,
or NutKernel syscall implementations.
