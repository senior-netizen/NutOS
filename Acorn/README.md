# Acorn Desktop Environment

Acorn is the NutOS desktop environment described in `architecture/UI_DESIGN.md`.
This crate starts the Rust project structure for the shell, compositor-facing
Wayland integration, GPU rendering through `wgpu`, and the native Acorn UI
toolkit.

## Initial modules

| Module | Purpose |
|---|---|
| `desktop` | NutBar, Dock, Spotlight, Mission Control, tiling, and workspace defaults. |
| `rendering` | `wgpu` backend policy and compositor instance bootstrap. |
| `wayland` | Wayland client/server/protocol binding plan and bootstrap helpers. |
| `toolkit` | Design-system spacing, motion, component, theme, and accessibility tokens. |

## Graphics and display dependencies

- `wgpu` is the GPU abstraction layer for Vulkan, Metal, and OpenGL/GLES.
- `wayland-client` supports shell-owned clients during early UI development.
- `wayland-server` provides compositor-side display bindings for app hosting.
- `wayland-protocols` provides generated protocol extension bindings.
