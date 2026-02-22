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

### `bazel run //src:main`

When we run this command, bazel compiles in a deeply nested folder on your machine (usually in `~/.cache/bazel/...`). However, finding a hidden cache directory is annoying, so Bazel generates four symlinks in your root folder so you can easily access the results.

