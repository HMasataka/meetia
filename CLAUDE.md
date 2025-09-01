# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Meetia is a 3D application built with Rust and Godot 4.3 that demonstrates online GLTF model loading and 3D player movement. The architecture consists of:

- **Rust GDExtension**: Core logic implemented in Rust using `godot-rust/gdext`
- **Godot Engine**: 3D scene management and rendering using Godot 4.3
- **GLTF Loading**: Dynamic loading of 3D models from online sources (specifically GLTF/GLB files)

## Common Development Commands

Use the Task runner (requires `task` command):

```bash
# Development workflow - builds Rust and runs Godot
task dev

# Individual tasks
task build              # Build Rust library
task build-release      # Build in release mode
task check              # Check Rust code without building
task fmt                # Format Rust code
task clippy             # Run Clippy linter
task test               # Run Rust tests
task clean              # Clean build artifacts
task godot-run          # Run Godot project directly

# Full pipeline (format, lint, build, test)
task all

# Copy project to Windows host (requires manual setup of .local/copy.sh)
task copy
```

Direct cargo commands (from `/rust` directory):

```bash
cargo build             # Build the Rust GDExtension
cargo check             # Fast syntax/type checking
cargo clippy            # Linting
cargo fmt               # Code formatting
```

## Code Architecture

### Rust Components (`rust/src/`)

**Main Library (`lib.rs`)**

- Entry point that registers the `Meetia` GDExtension
- Modules: `gltf_loader`, `player`

**Online GLTF Loader (`gltf_loader.rs`)**

- `OnlineGltfLoader`: Downloads and imports GLTF models from URLs
- Creates reference world objects (cubes, spheres, ground plane)
- Applies automatic model orientation correction (90° Y-axis rotation, scale adjustment)
- Integrates loaded models with the Player3D system
- Default model: Khronos Group Duck sample if no URL specified

**Player Controller (`player.rs`)**

- `Player3D`: Third-person player controller with camera
- WASD movement relative to player rotation
- Arrow key rotation controls
- Automatic camera setup (follows behind player, angled downward)
- Configurable speed and rotation speed

### Godot Scene Structure (`godot/`)

**Main Scene (`gltf.tscn`)**

- Root Node3D with Camera3D and DirectionalLight3D
- OnlineGltfLoader node that initiates the model loading process

**Input Mapping**

- W/A/S/D: Player movement (forward/left/backward/right)
- Left/Right Arrow: Player rotation

### Key Integration Points

1. **GDExtension Registration**: Rust types are registered with Godot through `#[derive(GodotClass)]`
2. **Model Loading Flow**: HttpRequest → GLTF parsing → Model correction → Player integration
3. **Scene Hierarchy**: OnlineGltfLoader creates Player3D → Player3D creates Camera3D → GLTF model becomes child of Player3D

### Development Environment Notes

- Built for WSL2/Linux development with optional Windows copying via `task copy`
- Godot 4.3 with Forward+ rendering
- Uses latest `godot-rust/gdext` from master branch
- Compiled as cdylib for Godot integration

### Model Loading Behavior

- Automatic orientation correction for common GLTF coordinate system differences
- Scale adjustment for models that are too small (<0.1) or too large (>50.0)
- Reference objects placed in world to help with spatial orientation
- Player controller automatically attached to loaded models

