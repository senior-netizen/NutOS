# 🎨 Acorn Desktop Environment — UI Design
*Rodent, Inc. | SquirrelLabs Technologies*

---

## Overview

**Acorn** is the NutOS desktop environment — purpose-built from the ground up. It is not GNOME, KDE, or any existing Linux DE with a new skin. Acorn is a native, GPU-accelerated desktop experience designed to match macOS in polish while surpassing Windows in customizability.

---

## Design Philosophy

1. **Invisible when it's working** — The OS should get out of your way
2. **Consistent everywhere** — Every dialog, button, and icon follows the same language
3. **Responsive by default** — Acorn works on any screen size, from 1080p to 8K
4. **Dark and Light mode are both first-class** — Not an afterthought
5. **Accessible by design** — Not a feature to add later

---

## Rendering Stack

Acorn is built on a modern, GPU-accelerated rendering stack:

```
┌───────────────────────────────────────┐
│          Acorn Shell (Rust)           │  ← Window manager, compositor
├───────────────────────────────────────┤
│       Acorn UI Toolkit (Rust)         │  ← Native widget library
├───────────────────────────────────────┤
│          Wayland Protocol             │  ← Display server protocol
├───────────────────────────────────────┤
│    wgpu (GPU abstraction layer)       │  ← Vulkan / Metal / OpenGL
├───────────────────────────────────────┤
│         GPU Hardware                  │
└───────────────────────────────────────┘
```

- **Language:** Acorn Shell and UI Toolkit are written in **Rust** for memory safety and performance
- **Display protocol:** Wayland (no X11 legacy baggage)
- **GPU backend:** wgpu (supports Vulkan, Metal, and OpenGL — works everywhere)
- **Animations:** 60fps minimum, 120fps on capable hardware, locked to display refresh rate

---

## Desktop Layout

```
┌─────────────────────────────────────────────────────┐
│  NutBar (top)                              [●][●][●] │  ← Status, clock, system tray
├────┬────────────────────────────────────────────────┤
│    │                                                 │
│ D  │                                                 │
│ o  │                 Desktop Canvas                  │
│ c  │                                                 │
│ k  │                                                 │
│    │                                                 │
│ (  │                                                 │
│ l  │                                                 │
│ e  ├─────────────────────────────────────────────────┤
│ f  │  Quick Launch / Spotlight (cmd+space)           │
│ t  │                                                 │
│ )  │                                                 │
└────┴─────────────────────────────────────────────────┘
```

### NutBar (Top Bar)
- **Left:** Active app name + menu bar (macOS-style app menus)
- **Center:** Clock and date
- **Right:** System tray — network, audio, battery, notifications, quick settings

### The Dock (Left Side, Default)
- App icons for pinned and running apps
- Hover shows app name and window thumbnails
- Position is configurable: left, right, bottom, or hidden
- Auto-hide support with configurable sensitivity

### Spotlight (Universal Search)
- `Cmd+Space` (or user-defined shortcut) opens **Spotlight**
- Search: apps, files, settings, web, contacts, calculator
- AI-powered natural language: "open my presentation from last Tuesday"
- Instant results — renders as you type

---

## Window Management

### Features
- **Tiling mode** — Snap windows to halves, quarters, thirds
- **Virtual desktops** — Up to 16 workspaces, swipe or keyboard to switch
- **Mission Control** — Overview of all windows and spaces
- **Picture-in-Picture** — Any app can be floated as a mini window
- **Window grouping** — Group related windows together
- **Focus mode** — Hide all other apps with one shortcut

### Keyboard Shortcuts (Default)
| Action | Shortcut |
|---|---|
| Spotlight Search | `Cmd + Space` |
| Switch apps | `Cmd + Tab` |
| Switch windows (same app) | `Cmd + ~` |
| New virtual desktop | `Ctrl + Cmd + D` |
| Switch desktop | `Ctrl + Arrow` |
| Mission Control | `Cmd + F3` |
| Tile left/right | `Cmd + Ctrl + Left/Right` |
| Fullscreen | `Cmd + Ctrl + F` |
| Screenshot | `Cmd + Shift + 3/4/5` |

---

## Acorn UI Design System

### Spacing Scale (8pt grid)
```
4px  — micro (borders, tight padding)
8px  — small (button padding, icon gaps)
16px — base (card padding, section spacing)
24px — medium (panel padding)
32px — large (section dividers)
48px — xl (hero spacing)
```

### Component Library
Acorn ships with a full set of native UI components:

| Component | Notes |
|---|---|
| Buttons | Primary, Secondary, Ghost, Destructive |
| Inputs | Text, Number, Password, Search |
| Select / Dropdown | Single and multi-select |
| Modals / Sheets | Animated, keyboard accessible |
| Notifications | Toast and persistent banner styles |
| Progress / Loaders | Determinate and indeterminate |
| Toggle / Checkbox / Radio | Animated state transitions |
| Tables | Sortable, filterable, virtualized for large data |
| Sidebar / Navigation | Collapsible, multi-level |

### Motion Design
All animations use **spring physics** (not linear/ease curves) for a natural feel:
- Window open: `spring(stiffness: 280, damping: 26)`
- Button press: `spring(stiffness: 400, damping: 30)`
- Panel slide: `spring(stiffness: 220, damping: 28)`
- Duration cap: 400ms maximum — nothing feels slow

---

## Customization

Acorn is the most customizable premium desktop experience available:

| Customization | Scope |
|---|---|
| Themes | Full color scheme control, import/export |
| Icon packs | Swap any icon set system-wide |
| Fonts | Change UI font system-wide |
| Dock position | Left, right, bottom, hidden |
| NutBar layout | Rearrange or hide any element |
| Animations | Reduce, disable, or modify |
| Keyboard shortcuts | Remap anything |
| Window decorations | On, off, minimal |
| Transparency / blur | Configurable blur radius and opacity |

---

## Accessibility

Acorn is built WCAG 2.1 AA compliant from day one:

- Full **VoiceOver-equivalent screen reader** (NutSpeak)
- **High contrast mode** (automatic system detection)
- **Dynamic text size** — UI scales with user's preferred font size
- **Sticky Keys, Slow Keys, Bounce Keys** support
- **Color blindness modes** (Deuteranopia, Protanopia, Tritanopia)
- **Focus indicators** always visible — never hidden for aesthetics
- **Reduce motion** respects OS accessibility settings

---

*Document maintained by: Rodent, Inc. Acorn UI Team*
*Classification: Internal — Design & Technical*
