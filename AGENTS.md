# AGENTS.md — zygisk-spoof

## What this is

A Rust `cdylib` compiled for `aarch64-linux-android` that runs as a Magisk Zygisk module. It hooks `__system_property_get` and `openat` to spoof device build properties and CPU info on a per-app basis.

## Build

```sh
./build.sh                    # full build → zygisk_spoof.zip
ANDROID_NDK_HOME=/path ./build.sh  # explicit NDK path
```

Prerequisites: `cargo-ndk`, `ANDROID_NDK_HOME`, `rustup target add aarch64-linux-android`.

The CI workflow in `.github/workflows/build.yml` does the same steps with setup-ndk@v1 (NDK r27c). Keep it in sync with `build.sh`.

## Key structure

| Path | Role |
|---|---|
| `src/lib.rs` | Zygisk module entrypoint (callbacks: on_module_loaded / pre/post_app_specialize) |
| `src/config.rs` | Parses `module/config.json`, builds per-package spoof bundles |
| `src/hook.rs` | Hooks `__system_property_get` |
| `src/hook_file.rs` | Hooks `openat`, returns spoofed content via `memfd_create` |
| `src/trampoline.rs` | AArch64 inline hook (16‑byte trampoline, `mprotect`, `mmap`) |
| `module/config.json` | Spoof profiles (one per device model, mapped to target package names) |
| `module/module.prop` | Magisk module metadata |
| `module/customize.sh` | Installer (unzips, removes placeholder) |

## Build output

- `module/zygisk/arm64-v8a.so` — the native library
- `zygisk_spoof.zip` — installable Magisk module

Both are gitignored.

## Conventions

- No tests, no lint, no typecheck — this is an Android system hook, not a standard Rust project.
- Release profile (`Cargo.toml`) is aggressively optimized: LTO=fat, codegen-units=1, opt-level=3, strip, panic=abort, overflow-checks=off.
- The `hook_state!` macro in `lib.rs` generates per-hook static state (atomic refs, reentry guard). Every hook follows the same pattern.
- AArch64‑only: trampoline assembles hardcoded A64 instructions (`0x58000050`, `0xD61F0200`).
- Config values are truncated at 91 chars (`PROP_MAX` in `config.rs`).
- New device profiles go in `module/config.json`; each has a `target` array of package names and optional `cpu_spoof` fields.
