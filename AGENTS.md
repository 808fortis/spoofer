# AGENTS.md — zygisk-spoof

## What this is

A Rust `cdylib` compiled for `aarch64-linux-android` that runs as a Magisk Zygisk module. Hooks `__system_property_get` to spoof device build properties (brand, model, fingerprint, etc.) on a per-app basis. Config is embedded at compile time — no runtime file parsing.

## Build

```sh
./build.sh                    # full build → zygisk_spoof.zip
ANDROID_NDK_HOME=/path ./build.sh
```

Prerequisites: `cargo-ndk`, `ANDROID_NDK_HOME`, `rustup target add aarch64-linux-android`.

CI (`.github/workflows/build.yml`) runs the same steps with `nttld/setup-ndk@v1` (NDK r27c) but **does not zip** — only uploads `module/`. The zip step is only in `build.sh`.

## Key structure

| Path | Role |
|---|---|
| `src/lib.rs` | Entrypoint + **all config data** (static `DEV_TABLE`, `PKG_TABLE`) + binary search lookup |
| `src/hook.rs` | Hooks `__system_property_get`; linear scan over 8 prop pairs per device |
| `src/trampoline.rs` | AArch64 inline hook (16-byte trampoline, `mprotect`, `mmap`) |
| `module/module.prop` | Magisk module metadata |
| `module/customize.sh` | Installer (unzips, removes placeholder) |

## Build output

- `module/zygisk/arm64-v8a.so` — native library
- `zygisk_spoof.zip` — installable module

Both gitignored.

## Conventions

- No tests, no lint, no typecheck — Android system hook, not a standard Rust project.
- Release profile: LTO=fat, codegen-units=1, opt-level=3, strip, panic=abort, overflow-checks=off.
- AArch64-only: trampoline uses hardcoded A64 instructions (`0x58000050`, `0xD61F0200`).
- `hook_state!` macro (in `hook.rs`) generates per-hook static state (atomic ORIG/INSTALLED, reentry guard).
- Config is embedded as sorted `static` arrays in `lib.rs`. To add/modify a device profile:
  1. Add a `SpoofCfg` entry to `DEV_TABLE` (8 `PropPair`s: brand, device, manufacturer, model, fingerprint × 2, product × 2)
  2. Add `PkgEnt` entries to `PKG_TABLE` **sorted lexicographically** (binary search depends on this).
- Only dependency: `libc`. No serde, no JSON parsing.
- Config is compiled in; editing runtime files has no effect.