# Rust & C++ Sandbox

## Initialization

### `MODULE.bazel` - Supply Chain

Tells Bazel what tools to download from the internet.

Here, we added the following tools:

- `module(name="sandbox")`: names your workspace
- `bazel_dep(name="rules_rust")`: download the logic for compiling Rust. otherwise, Bazel doesn't know what `rust_binary` means
- `bazel_dep(name="rules_cc")`: download the logic for compiling c++
- `use_extension`: prepares us to download the specific Rust crates like `cxx` or `serde` later

### `BUILD.bazel` - The Map

This file makes the directory as a **Package**

- Bazel allows you to have nested projects. By putting a `BUILD.bazel` file in the root, we are saying that this root directory is a valid place to run commands
- `package(default_visability...)`: "allow" other parts of my project to see my code

---

## Project Context (Migration Sandbox)

### Purpose

This repository is a **standalone learning environment** designed to mimic the build and interoperability challenges of the `pw_bluetooth_sapphire` migration. It allows for safe experimentation with C++/Rust bridges without the complexity of the full Pigweed/Fuchsia build system.

### Project Scope

1.  **Phase 1: The Basics (Current)**:
    - Set up a Bazel workspace that builds both C++ and Rust.
    - **Goal**: Ensure `bazel run //src:main` prints "Hello". (✅ Done)
2.  **Phase 2: The Bridge (`cxx`)**:
    - Configure the `cxx` crate.
    - Create a bidirectional bridge where Rust can instantiate and call methods on a C++ class (`src/counter.h`).
    - **Goal**: `main.rs` creates a `Counter`, increments it, and prints the result.

3.  **Phase 3: The Hard Stuff (Simulation)**:
    - Simulate the specific patterns found in `pw_bluetooth_sapphire`:
      - **`Identifier`**: migrating a "NewType" wrapper (like `PeerId`).
      - **`ByteBuffer`**: migrating memory ownership and views (Span/Vector).
      - **`WeakSelf`**: simulating the async callback pattern.

### Requirements

- **Build System**: Must use **Bazel**.
- **Version Pinning**: Must use **Bazel 7.4.1** (defined in `.bazelversion`) to ensure compatibility with `rules_rust`.
- **Self-Contained**: All dependencies must be declared in `MODULE.bazel`. No system-wide library installations.
- **Reproducibility**: `bazel run //src:main` must always work on a fresh clone.

### Current Status

- Bazel environment configured.
- C++ and Rust targets defined but isolated.
- Next step: Add `cxx` to `MODULE.bazel` and bridge the languages.
