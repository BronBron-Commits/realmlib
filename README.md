# realmlib

**realmlib** is a Rust-based Godot 4 GDExtension that provides AW/DW-style world primitives for building large, network-driven virtual worlds.

It is designed to solve the engine-missing pieces of classic Active Worlds / DeltaWorlds–style clients while staying modular, modern, and realistic in scope.

## Goals

- Provide a native, high-performance client-side world layer
- Support large, mostly static, network-driven worlds
- Integrate cleanly with Godot 4 as a GDExtension
- Be engine-agnostic in design, but Godot-first in implementation
- Avoid rewriting entire game engines or monolithic clients

## What this is

- A Rust crate compiled as a Godot GDExtension
- A world state owner, **not** a renderer or a client
- A reusable foundation for:
  - AW/DW-style clients
  - World viewers
  - Editors
  - Experiments with large virtual spaces

## What this is NOT

❌ A full client  
❌ A networking stack  
❌ A server  
❌ A gameplay framework  

Networking, authentication, and protocol handling are intentionally out of scope and expected to be handled by external SDKs (e.g. LibreWorlds or similar).

## Architecture Overview

```
[ Network / SDK (C#, etc.) ]
              ↓
      [ World Adapter ]
              ↓
      [ realmlib (Rust) ]
              ↓
      [ Godot SceneGraph ]
```

- **Rust (realmlib)** owns world state and performance-critical logic  
- **Godot** provides rendering, editor tooling, and platform glue  
- **C# / SDK layers** orchestrate networking and high-level behavior

## Current Status

- ✅ Rust crate builds cleanly
- ✅ Compiles as a cdylib
- ✅ Loads successfully as a Godot 4 GDExtension
- ✅ Correct GDExtension entry point (`gdext_rust_init`)
- ✅ Basic project structure and module layout in place

**APIs are unstable and evolving.**  
This project is in **early development**.

## Project Structure

```
realmlib/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── lib.rs          # GDExtension entry point
│   ├── core/           # Engine-agnostic world model
│   │   ├── world.rs
│   │   ├── object.rs
│   │   └── transform.rs
│   └── godot_layer/    # Godot-facing bindings
│       ├── mod.rs
│       └── root.rs
```

## Design Principles

- **Thin engine boundary**  
  Godot should render and host, not own the world model.
- **Static-first world model**  
  Optimize for many static objects, not per-frame ticking entities.
- **Explicit scope control**  
  No networking, no gameplay, no UI logic inside realmlib.
- **Human-scale development**  
  Small team, realistic milestones, no “hero project” assumptions.

## Future Direction (Planned)

- Expose RealmWorld as a Godot node
- Add object insertion/removal APIs
- Batch static geometry for rendering
- RWX parsing and mesh generation
- Terrain streaming primitives
- Optional Godot-specific helpers
