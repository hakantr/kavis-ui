---
title: Installation
order: -1
---

# Installation

Before you start to build your application with `kavis-ui`, you need to install the library.

## System Requirements

We can development application on macOS, Windows or Linux.

### macOS

- macOS 15 or later
- Xcode command line tools

## Windows

- Windows 10 or later

There have a bootstrap script to help install the required toolchain and dependencies.

You can run the script in PowerShell:

```ps
.\script\install-window.ps1
```

## Linux

Run `./script/bootstrap` to install system dependencies.

## Rust and Cargo

We use Rust programming language to build the `kavis-ui` library. Make sure you have Rust and Cargo installed on your system.

- Rust 1.90 or later
- Cargo (comes with Rust)

To install the `kavis-ui` library, you can use Cargo, the Rust package manager. Add the following line to your `Cargo.toml` file under the `[dependencies]` section:

```toml
gpui = { path = "../zed/crates/gpui" }
gpui_platform = { path = "../zed/crates/gpui_platform", features = ["font-kit", "runtime_shaders", "screen-capture", "wayland", "x11"] }
kavis-ui = { git = "https://github.com/hakantr/kavis-ui" }
```
