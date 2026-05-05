# 🥜 NutShell — Universal Compatibility Layer
*Rodent, Inc. | SquirrelLabs Technologies*

---

## Overview

**NutShell** is NutOS's most ambitious technical component — a universal application compatibility layer that allows users to run software built for Windows, Linux, and Android on NutOS without modification.

It is the single biggest reason a user can switch to NutOS without giving up any of their existing tools.

---

## Compatibility Targets

| Platform | Compatibility Goal | Mechanism |
|---|---|---|
| Linux (x86-64) | 99% native | Direct execution (same kernel) |
| Windows (x86-64) | 90%+ | Wine-based translation layer + Proton |
| Android APKs | 85%+ | Waydroid-inspired container |
| macOS (x86-64) | 60%+ | Darling-inspired layer (Phase 3) |
| Web Apps | 100% | Bundled Chromium engine |

---

## Architecture

NutShell is composed of four sub-systems:

```
┌────────────────────────────────────────────────┐
│                  NutShell                      │
│                                                │
│  ┌──────────────┐   ┌──────────────────────┐  │
│  │  LinuxRun    │   │     WinBridge        │  │
│  │ (Native ELF) │   │  (Windows .exe/.dll) │  │
│  └──────────────┘   └──────────────────────┘  │
│                                                │
│  ┌──────────────┐   ┌──────────────────────┐  │
│  │  DroidShell  │   │     WebApp           │  │
│  │  (APK/AOSP)  │   │  (PWA/Electron)      │  │
│  └──────────────┘   └──────────────────────┘  │
└────────────────────────────────────────────────┘
                        │
           NutOS Kernel Compat Syscalls
```

---

## 1. LinuxRun — Native Linux Compatibility

Since NutOS uses a Linux kernel, native Linux ELF binaries run directly. LinuxRun provides:

- A **compatibility FHS** (Filesystem Hierarchy Standard) shim so Linux apps find their expected paths
- **Glibc and musl** support — both C library variants are supported
- An **AppImage runner** built in — double-click any AppImage and it just works
- **Flatpak** integration for sandboxed Linux app distribution

This means **any Linux app that runs on Ubuntu will run on NutOS** — without any changes.

---

## 2. WinBridge — Windows Application Compatibility

WinBridge is NutShell's most complex component. It is built on:

- **Wine** — the open-source Windows API implementation
- **Proton** (Valve's gaming-focused Wine fork) — for DirectX and game compatibility
- **DXVK** — DirectX 9/10/11 → Vulkan translation
- **VKD3D-Proton** — DirectX 12 → Vulkan translation
- **NutOS WinRT shim** — for modern Windows Store-style apps

### How It Works

```
Windows .exe
     │
     ▼
WinBridge API Translator
     │
     ├── Win32 API calls → NutOS POSIX equivalents
     ├── DirectX calls → Vulkan (via DXVK/VKD3D)
     ├── Registry → NutOS config store
     ├── NTFS paths → NutFS paths
     └── Windows services → NutOS service equivalents
     │
     ▼
NutKernel (runs the translated code)
```

### WinBridge Compatibility Database

WinBridge includes a **community-maintained compatibility database** (like ProtonDB) that:
- Rates each Windows application for NutOS compatibility
- Stores per-app workarounds and fixes
- Updates automatically via Harvest (the update manager)
- Is crowdsourced and community-maintained

### DirectX Support Matrix

| DirectX Version | Translation | Status |
|---|---|---|
| DirectX 9 | DXVK | ✅ Excellent |
| DirectX 10 | DXVK | ✅ Excellent |
| DirectX 11 | DXVK | ✅ Very Good |
| DirectX 12 | VKD3D-Proton | ✅ Good |
| DirectX Raytracing | VKD3D-Proton | 🔧 Experimental |

---

## 3. DroidShell — Android Application Compatibility

DroidShell allows Android APKs to run as first-class applications on NutOS:

- Built on **Waydroid** (Android in a container, using the host Linux kernel)
- Android apps appear in the **Acorn desktop** alongside native apps
- Supports **Google Play Store** via OpenGApps (optional, user-controlled)
- Supports **F-Droid** as a privacy-friendly alternative
- Android notifications integrate with NutOS notification center
- File sharing between Android apps and the NutOS filesystem is seamless

### Use Case

This closes the mobile app gap that Linux has always suffered from. Tools like Microsoft Teams mobile, banking apps, and niche Android utilities all work under DroidShell.

---

## 4. WebApp — Progressive Web App Engine

- A bundled **Chromium-based engine** runs PWAs as standalone desktop apps
- Any website can be "installed" as a desktop app via The Cache (app store)
- Electron apps (VS Code, Slack, Discord, etc.) run natively under LinuxRun and use this engine
- WebApp sandboxing is stricter than a browser tab — network access is opt-in per app

---

## Performance Considerations

| App Type | Performance vs Native | Notes |
|---|---|---|
| Native NutOS app | 100% | No overhead |
| Linux app (LinuxRun) | 99% | Negligible shim overhead |
| Windows app (WinBridge) | 70–90% | API translation overhead |
| DirectX game (WinBridge) | 85–95% | Vulkan is near-native |
| Android app (DroidShell) | 80–90% | Container overhead |
| Web app | 90–95% | Chromium is well-optimized |

---

## The Cache — App Discovery

**The Cache** is NutOS's app store. It unifies:
- Native NutOS apps
- Verified Linux Flatpaks
- WinBridge-compatible Windows apps (with compatibility ratings)
- DroidShell Android APKs
- Web Apps / PWAs

Users never need to know which compatibility layer is running an app. They just install and use it.

---

*Document maintained by: Rodent, Inc. NutShell Team*
*Classification: Internal — Technical*
