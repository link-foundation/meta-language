> This document is subordinate to the authoritative
> [vision and architecture specification](vision.md)
> and is the dependency inventory that its Dependencies section refers to.

Audit date: 2026-09-29

This inventory lists every runtime, development, build and optional dependency
of the two published packages, their lockfile and transitive resolutions, the
vendored grammar assets, the generators, the toolchains, the GitHub Actions and
the pinned build images, each with the current stable release on the audit
date. It records the state of the repository at commit `99a7d85`; it does not
change any pin. An item that is behind its current stable release is listed
with the compatibility reason recorded in the repository, or with "no recorded
compatibility reason — must be updated" where none is recorded.

"Current stable release" is the newest non-prerelease version that the named
source reported on the audit date.

## Method

The manifests and lockfiles were read directly (`js/package.json`,
`js/package-lock.json`, `rust/Cargo.toml`, `rust/Cargo.lock`,
`rust/web/Cargo.toml`, `rust/web/Cargo.lock`, `lean-toolchain`,
`.pre-commit-config.yaml`, `.github/workflows/*.yml`,
`js/src/vendor/grammars/grammar-lock.json`,
`js/src/vendor/web-tree-sitter/runtime-lock.json`,
`js/scripts/build-vendored-grammars.mjs` and the `NOTICE.md` of each
`rust/vendor/tree-sitter-*` directory). The current releases were queried with
these read-only commands:

```sh
# npm
cd js
npm outdated --json                     # direct dependencies
npm ls --all --json                     # resolved tree
npm view <package> dist-tags.latest     # every package in the lockfile
npm pack --dry-run --json               # published package contents
npm view npm dist-tags --json

# crates.io (every registry crate in rust/Cargo.lock and rust/web/Cargo.lock)
curl -s https://crates.io/api/v1/crates/<name> \
  -H 'User-Agent: meta-language-audit'  # .crate.max_stable_version
cd rust
cargo update --dry-run --verbose        # updates reachable within the ranges
cargo metadata --format-version 1 --locked --offline
cargo package --list --allow-dirty --offline
# `cargo outdated` is not installed and was not installed for this audit.

# GitHub releases (actions, toolchains, grammar upstreams)
gh api repos/<owner>/<repo>/releases/latest --jq .tag_name
gh api repos/<owner>/<repo>/tags --jq '.[0].name'
gh api repos/<owner>/<repo>/compare/<pinned>...<head>

# Node.js, opam and Docker Hub
curl -s https://nodejs.org/dist/index.json
curl -s https://raw.githubusercontent.com/nodejs/Release/main/schedule.json
gh api repos/ocaml/opam-repository/contents/packages/rocq-core
curl -s 'https://hub.docker.com/v2/repositories/emscripten/emsdk/tags?ordering=last_updated'
```

No lockfile or manifest was modified: `cargo update` ran with `--dry-run`, and
the checksums of both Cargo lockfiles were identical before and after.

## Summary

| Category | Items | Current | Behind | Not applicable |
|---|---|---|---|---|
| npm direct dependencies (`js/package.json`) | 5 | 4 | 1 | 0 |
| npm transitive packages (`js/package-lock.json`) | 11 | 5 | 6 | 0 |
| Rust direct dependencies (`rust/Cargo.toml`, unique crates) | 76 | 55 | 21 | 0 |
| Rust transitive registry crates (`rust/Cargo.lock`) | 318 | 136 | 182 | 0 |
| Website crate dependencies (`rust/web/Cargo.lock`, all) | 21 | 4 | 17 | 0 |
| Vendored grammar assets | 66 | 57 | 9 | 0 |
| Toolchains and generator tools | 17 | 8 | 8 | 1 |
| GitHub Actions (distinct refs) | 19 | 15 | 4 | 0 |
| Pinned build images | 1 | 0 | 1 | 0 |

The Rust transitive count treats each locked version as one item (a crate
locked at two versions counts twice). The vendored grammar count is the 60
WebAssembly grammars, the patched web-tree-sitter runtime and the five
vendored generated C parsers.

## JavaScript package (`js/`)

`js/package.json` declares `dependencies` and `devDependencies` only; it
declares no `optionalDependencies`, `peerDependencies` or `bundleDependencies`.
`js/package-lock.json` is lockfile version 3 with 16 packages: 5 direct and 11
transitive. The only peer dependency in the tree is the optional
`@types/emscripten ^1.40.0` of `web-tree-sitter`, which is not installed.

### Engines

| Field | Declared | Current stable release (source) | Status | Reason |
|---|---|---|---|---|
| `engines.node` | `>=20` | Node.js 24.21.0 LTS; 26.10.0 Current (nodejs.org `dist/index.json`) | behind | The floor admits Node.js 20, which reached end of life on 2026-04-30 (nodejs/Release `schedule.json`). No recorded compatibility reason — must be updated. |

### Direct dependencies

Source of the current release: `npm view <package> dist-tags.latest` and
`npm outdated --json`.

| Package | Section | Declared | Resolved | Current stable | Status | Reason |
|---|---|---|---|---|---|---|
| `links-notation` | `dependencies` | `0.22.0` | 0.22.0 | 0.22.0 | current | |
| `peggy` | `dependencies` | `5.1.0` | 5.1.0 | 5.1.0 | current | External grammar engine; see [Production parser dependencies](#production-parser-dependencies). |
| `web-tree-sitter` | `dependencies` | `0.25.10` | 0.25.10 | 0.27.0 | behind | No recorded compatibility reason — must be updated. Kept in step with the Rust `tree-sitter =0.25.10` pin and the vendored runtime; the vision removes it as a production parser dependency. |
| `franc-min` | `devDependencies` | `6.2.0` | 6.2.0 | 6.2.0 | current | |
| `pdf-lib` | `devDependencies` | `1.17.1` | 1.17.1 | 1.17.1 | current | |

### Transitive packages

| Package | Scope | Resolved | Required by | Current stable | Status | Reason |
|---|---|---|---|---|---|---|
| `@pdf-lib/standard-fonts` | dev | 1.0.0 | `pdf-lib ^1.0.0` | 1.0.0 | current | |
| `@pdf-lib/upng` | dev | 1.0.1 | `pdf-lib ^1.0.1` | 1.0.1 | current | |
| `@peggyjs/from-mem` | production | 3.1.3 | `peggy 3.1.3` (exact) | 3.1.4 | behind | Held by the exact range of `peggy` 5.1.0, the latest `peggy`; no recorded repository decision. |
| `collapse-white-space` | dev | 2.1.0 | `trigram-utils ^2.0.0` | 2.1.0 | current | |
| `commander` | production | 14.0.3 | `peggy ^14.0.3` | 15.0.0 | behind | Held by the range of `peggy` 5.1.0 (14.0.3 is the newest 14.x); no recorded repository decision. |
| `n-gram` | dev | 2.0.2 | `trigram-utils ^2.0.0` | 2.0.2 | current | |
| `pako` | dev | 1.0.11 | `pdf-lib ^1.0.11`, `@pdf-lib/* ^1.0.x` | 3.0.2 | behind | Held by the range of `pdf-lib` 1.17.1, the latest `pdf-lib` (1.0.11 is the newest 1.x); no recorded repository decision. |
| `semver` | production | 7.7.4 | `@peggyjs/from-mem 7.7.4` (exact) | 7.8.5 | behind | Held by the exact range of `@peggyjs/from-mem` 3.1.3; no recorded repository decision. |
| `source-map-generator` | production | 2.0.6 | `peggy 2.0.6` (exact) | 2.0.7 | behind | Held by the exact range of `peggy` 5.1.0; no recorded repository decision. |
| `trigram-utils` | dev | 2.0.1 | `franc-min ^2.0.0` | 2.0.1 | current | |
| `tslib` | dev | 1.14.1 | `pdf-lib ^1.11.1` | 2.8.1 | behind | Held by the range of `pdf-lib` 1.17.1 (1.14.1 is the newest 1.x); no recorded repository decision. |

Every transitive package that is behind is already at the newest version its
dependent's range allows, so none of them can move without upgrading or
replacing the dependent (`peggy`, `pdf-lib`) or overriding the range.

### Published artifact contents

`npm pack --dry-run` for `meta-language@0.58.2` lists 206 files, 7,930,953
bytes packed and 9,485,820 bytes unpacked. 126 of the files (8,280,990 bytes)
are under `src/vendor/`: 60 gzip-compressed tree-sitter grammar WebAssembly
binaries with their licenses, `NOTICE.md` and `grammar-lock.json` in
`src/vendor/grammars/`, and the patched web-tree-sitter runtime
(`tree-sitter.wasm.gz`, `runtime-lock.json`, `utf8-input.patch`, `LICENSE`) in
`src/vendor/web-tree-sitter/`.

## Rust crate (`rust/`)

### Package settings

| Setting | Declared in | Value | Current stable release (source) | Status | Reason |
|---|---|---|---|---|---|
| `edition` | `rust/Cargo.toml` | 2021 | 2024 (Rust 1.98.1 release) | behind | No recorded compatibility reason — must be updated. `docs/four-language-contracts.md` already declares Rust 1.98.1, edition 2024, as the Rust source profile. |
| `rust-version` | `rust/Cargo.toml` | 1.77 | 1.98.1 (`rust-lang/rust` releases) | not applicable | A minimum-supported-version floor, not a toolchain pin. It is inaccurate: locked crates declare higher floors (`home` 0.5.12 needs 1.88, `wasip2` 1.0.3 needs 1.87, the `icu_*` 2.2.0 crates need 1.86; `cargo metadata`). No CI job builds with 1.77. |
| `rust-toolchain` file | none | none | | not applicable | The toolchain is chosen by the workflows (see [Toolchains](#toolchains)). |
| features | `rust/Cargo.toml` | `default = ["lindera"]`, `lindera`, `doublets` (`doublets` + `platform-mem`), `llm-assist` (no dependencies) | | not applicable | |

`rust/Cargo.lock` locks 395 packages: the `meta-language` root and 394
crates.io crates (76 direct, 318 transitive). There are no git or path
dependencies. Source of the current release: the crates.io API
`max_stable_version`.

### `[dependencies]` (non-grammar)

| Crate | Declared | Resolved | Current stable | Status | Reason |
|---|---|---|---|---|---|
| `abnf` | `0.13.0` | 0.13.0 | 0.13.0 | current | |
| `bnf` | `0.6.0` | 0.6.0 | 0.6.0 | current | |
| `clap` | `4.4` | 4.5.60 | 4.6.7 | behind | No recorded compatibility reason — must be updated; `cargo update` reaches 4.6.7 within the declared range |
| `doublets` | `0.4.0` (optional) | 0.4.0 | 0.5.0 | behind | No recorded compatibility reason — must be updated; needs a manifest change |
| `ebnf` | `0.1.4` | 0.1.4 | 0.1.4 | current | |
| `lindera` | `3.0.7` (optional) | 3.0.7 | 6.2.0 | behind | No recorded compatibility reason — must be updated; needs a manifest change |
| `lingua` | `1.8.0` | 1.8.0 | 1.8.0 | current | |
| `links-notation` | `0.22.0` | 0.22.0 | 0.22.0 | current | |
| `pest_meta` | `2.8.6` | 2.8.6 | 2.9.2 | behind | No recorded compatibility reason — must be updated; `cargo update` reaches 2.9.2 within the declared range |
| `regex` | `1` | 1.12.3 | 1.13.1 | behind | No recorded compatibility reason — must be updated; `cargo update` reaches 1.13.1 within the declared range |
| `serde` | `1` | 1.0.228 | 1.0.229 | behind | No recorded compatibility reason — must be updated; `cargo update` reaches 1.0.229 within the declared range |
| `platform-mem` | `0.3.0` (optional) | 0.3.0 | 0.3.0 | current | |
| `serde_json` | `1` | 1.0.150 | 1.0.151 | behind | No recorded compatibility reason — must be updated; `cargo update` reaches 1.0.151 within the declared range |
| `sha2` | `0.10` | 0.10.9 | 0.11.0 | behind | No recorded compatibility reason — must be updated; needs a manifest change |
| `tree-sitter` | `=0.25.10` | 0.25.10 | 0.27.0 | behind | No recorded compatibility reason — must be updated; the vision removes it as a production parser dependency |
| `tree-sitter-language` | `0.1.7` | 0.1.7 | 0.1.8 | behind | No recorded compatibility reason — must be updated; `cargo update` reaches 0.1.8 within the declared range |
| `unicode-bidi` | `0.3.18` | 0.3.18 | 0.3.18 | current | |
| `unicode-normalization` | `0.1.25` | 0.1.25 | 0.1.25 | current | |
| `unicode-segmentation` | `1.13.3` | 1.13.3 | 1.13.3 | current | |
| `whatlang` | `0.18.0` | 0.18.0 | 0.18.0 | current | |

### `[dependencies]` (tree-sitter grammar crates)

All 51 grammar crates are exact (`=`) pins. They are production parser
dependencies that the vision removes as native grammars replace them (see
[Production parser dependencies](#production-parser-dependencies)).
`cargo update --dry-run` reports `ts-parser-perl` 1.2.1 as the newest 1.x;
crates.io reports 2.0.0 as its current stable release.

| Crate | Declared | Resolved | Current stable | Status | Reason |
|---|---|---|---|---|---|
| `tree-sitter-agda` | `=1.3.3` | 1.3.3 | 1.3.3 | current | |
| `tree-sitter-bash` | `=0.25.1` | 0.25.1 | 0.25.1 | current | |
| `tree-sitter-c` | `=0.24.2` | 0.24.2 | 0.24.2 | current | |
| `tree-sitter-c-sharp` | `=0.23.5` | 0.23.5 | 0.23.5 | current | |
| `tree-sitter-cpp` | `=0.23.4` | 0.23.4 | 0.23.4 | current | |
| `tree-sitter-css` | `=0.25.0` | 0.25.0 | 0.25.0 | current | |
| `tree-sitter-dart` | `=0.2.0` | 0.2.0 | 0.2.0 | current | |
| `tree-sitter-diff` | `=0.1.0` | 0.1.0 | 0.1.0 | current | |
| `tree-sitter-elixir` | `=0.3.5` | 0.3.5 | 0.3.5 | current | |
| `tree-sitter-elm` | `=5.9.4` | 5.9.4 | 5.9.4 | current | |
| `tree-sitter-erlang` | `=0.20.0` | 0.20.0 | 0.20.0 | current | |
| `tree-sitter-go` | `=0.25.0` | 0.25.0 | 0.25.0 | current | |
| `tree-sitter-graphql` | `=0.1.0` | 0.1.0 | 0.2.1 | behind | No recorded compatibility reason — must be updated; exact pin, and the JavaScript WASM build and CST fixtures are generated from it |
| `tree-sitter-groovy` | `=0.1.2` | 0.1.2 | 0.1.2 | current | |
| `tree-sitter-haskell` | `=0.23.1` | 0.23.1 | 0.23.1 | current | |
| `tree-sitter-hcl` | `=1.1.0` | 1.1.0 | 1.1.0 | current | |
| `tree-sitter-html` | `=0.23.2` | 0.23.2 | 0.23.2 | current | |
| `tree-sitter-ini` | `=1.4.0` | 1.4.0 | 1.4.0 | current | |
| `tree-sitter-java` | `=0.23.5` | 0.23.5 | 0.23.5 | current | |
| `tree-sitter-javascript` | `=0.25.0` | 0.25.0 | 0.25.0 | current | |
| `tree-sitter-json` | `=0.24.8` | 0.24.8 | 0.24.8 | current | |
| `tree-sitter-json5-orchard` | `=0.1.0` | 0.1.0 | 0.1.0 | current | |
| `tree-sitter-kotlin-ng` | `=1.1.0` | 1.1.0 | 1.1.0 | current | |
| `tree-sitter-lua` | `=0.2.0` | 0.2.0 | 0.5.0 | behind | No recorded compatibility reason — must be updated; exact pin, and the JavaScript WASM build and CST fixtures are generated from it |
| `tree-sitter-make` | `=1.1.1` | 1.1.1 | 1.1.1 | current | |
| `tree-sitter-matlab` | `=1.3.1` | 1.3.1 | 1.3.1 | current | |
| `tree-sitter-md-025` | `=0.5.6` | 0.5.6 | 0.5.6 | current | |
| `tree-sitter-nix` | `=0.3.0` | 0.3.0 | 0.3.0 | current | |
| `tree-sitter-ocaml` | `=0.24.2` | 0.24.2 | 0.26.0 | behind | No recorded compatibility reason — must be updated; exact pin, and the JavaScript WASM build and CST fixtures are generated from it |
| `tree-sitter-odin` | `=1.3.0` | 1.3.0 | 1.3.0 | current | |
| `tree-sitter-pascal` | `=0.10.2` | 0.10.2 | 0.10.2 | current | |
| `ts-parser-perl` | `=1.1.2` | 1.1.2 | 2.0.0 | behind | No recorded compatibility reason — must be updated; exact pin, and the JavaScript WASM build and CST fixtures are generated from it |
| `tree-sitter-php` | `=0.24.2` | 0.24.2 | 0.24.2 | current | |
| `tree-sitter-powershell` | `=0.26.4` | 0.26.4 | 0.26.4 | current | |
| `tree-sitter-proto` | `=0.4.0` | 0.4.0 | 0.6.0 | behind | No recorded compatibility reason — must be updated; exact pin, and the JavaScript WASM build and CST fixtures are generated from it |
| `tree-sitter-python` | `=0.25.0` | 0.25.0 | 0.25.0 | current | |
| `tree-sitter-r` | `=1.2.0` | 1.2.0 | 1.3.0 | behind | No recorded compatibility reason — must be updated; exact pin, and the JavaScript WASM build and CST fixtures are generated from it |
| `tree-sitter-racket` | `=0.25.0` | 0.25.0 | 0.25.0 | current | |
| `tree-sitter-regex` | `=0.25.0` | 0.25.0 | 0.25.0 | current | |
| `tree-sitter-ruby` | `=0.23.1` | 0.23.1 | 0.23.1 | current | |
| `tree-sitter-scala` | `=0.25.1` | 0.25.1 | 0.26.2 | behind | No recorded compatibility reason — must be updated; exact pin, and the JavaScript WASM build and CST fixtures are generated from it |
| `tree-sitter-scheme` | `=0.24.7` | 0.24.7 | 0.24.7 | current | |
| `tree-sitter-sequel` | `=0.3.11` | 0.3.11 | 0.3.11 | current | |
| `tree-sitter-solidity` | `=1.2.13` | 1.2.13 | 1.2.13 | current | |
| `tree-sitter-swift` | `=0.7.3` | 0.7.3 | 0.7.3 | current | |
| `tree-sitter-toml-ng` | `=0.7.0` | 0.7.0 | 0.7.0 | current | |
| `tree-sitter-typescript` | `=0.23.2` | 0.23.2 | 0.23.2 | current | |
| `tree-sitter-vb-dotnet` | `=0.1.0` | 0.1.0 | 0.1.0 | current | |
| `tree-sitter-xml` | `=0.7.0` | 0.7.0 | 0.7.0 | current | |
| `tree-sitter-yaml` | `=0.7.2` | 0.7.2 | 0.7.2 | current | |
| `tree-sitter-zig` | `=1.1.2` | 1.1.2 | 1.1.2 | current | |

### `[dev-dependencies]`

| Crate | Declared | Resolved | Current stable | Status | Reason |
|---|---|---|---|---|---|
| `links-notation` | `0.22.0` | 0.22.0 | 0.22.0 | current | |
| `pest` | `2.8.6` | 2.8.6 | 2.9.2 | behind | No recorded compatibility reason — must be updated; `cargo update` reaches 2.9.2 within the declared range |
| `pest_derive` | `2.8.6` | 2.8.6 | 2.9.2 | behind | No recorded compatibility reason — must be updated; `cargo update` reaches 2.9.2 within the declared range |
| `regex` | `1` | 1.12.3 | 1.13.1 | behind | No recorded compatibility reason — must be updated; `cargo update` reaches 1.13.1 within the declared range |
| `serde_json` | `1` | 1.0.150 | 1.0.151 | behind | No recorded compatibility reason — must be updated; `cargo update` reaches 1.0.151 within the declared range |
| `walkdir` | `2` | 2.5.0 | 2.5.0 | current | |

### `[build-dependencies]`

| Crate | Declared | Resolved | Current stable | Status | Reason |
|---|---|---|---|---|---|
| `cc` | `1.2` | 1.2.63 | 1.5.1 | behind | No recorded compatibility reason — must be updated; `cargo update` reaches only 1.2.67 |
| `flate2` | `1` | 1.1.9 | 1.1.10 | behind | No recorded compatibility reason — must be updated; `cargo update` reaches 1.1.10 within the declared range |

### Transitive resolutions behind the current stable release

Of the 318 transitive registry crate versions in `rust/Cargo.lock`, 182 are
behind the crates.io current stable release and 136 are current.
`cargo update --dry-run --verbose` would move 119 of the 182 within the
declared ranges and drop 18 (their dependents move to newer crates that no
longer need them); the other 45 are held by the semver range of the locked
version of the crate named in the last column. None of the 182 has a recorded
compatibility reason; all must be updated, the held ones by updating the
crate that holds them. The resolved versions of the transitive crates that are
current are in `rust/Cargo.lock`.

| Crate | Locked | Current stable | Resolution |
|---|---|---|---|
| `abnf-core` | 0.5.0 | 0.6.0 | held by `abnf` |
| `aho-corasick` | 1.1.4 | 1.1.5 | `cargo update` moves it to 1.1.5 |
| `allocator-api2` | 0.2.21 | 0.4.0 | held by `hashbrown` |
| `anstream` | 0.6.21 | 1.0.0 | `cargo update` moves it to 1.0.0 |
| `anstyle-parse` | 0.2.7 | 1.0.0 | `cargo update` moves it to 1.0.0 |
| `anyhow` | 1.0.102 | 1.0.104 | `cargo update` moves it to 1.0.104 |
| `aws-lc-rs` | 1.13.3 | 1.18.1 | `cargo update` moves it to 1.18.1 |
| `aws-lc-sys` | 0.30.0 | 0.45.0 | `cargo update` moves it to 0.45.0 |
| `base64` | 0.22.1 | 0.23.1 | `cargo update` moves it to 0.23.1 |
| `bindgen` | 0.69.5 | 0.73.2 | dropped by `cargo update` (dependents move to newer crates) |
| `bitflags` | 1.3.2 | 2.13.2 | held by `kanaria` |
| `bitflags` | 2.13.0 | 2.13.2 | `cargo update` moves it to 2.13.2 |
| `block-buffer` | 0.10.4 | 0.12.1 | held by `digest` |
| `bytecheck` | 0.8.2 | 0.8.3 | `cargo update` moves it to 0.8.3 |
| `bytecheck_derive` | 0.8.2 | 0.8.3 | `cargo update` moves it to 0.8.3 |
| `bytes` | 1.11.1 | 1.12.1 | `cargo update` moves it to 1.12.1 |
| `cfg-if` | 1.0.4 | 1.0.5 | `cargo update` moves it to 1.0.5 |
| `cfg_aliases` | 0.2.1 | 0.2.2 | `cargo update` moves it to 0.2.2 |
| `clang-sys` | 1.8.1 | 1.9.1 | dropped by `cargo update` (dependents move to newer crates) |
| `clap_builder` | 4.5.60 | 4.6.7 | `cargo update` moves it to 4.6.7 |
| `clap_derive` | 4.5.55 | 4.6.7 | `cargo update` moves it to 4.6.7 |
| `clap_lex` | 1.1.0 | 1.1.1 | `cargo update` moves it to 1.1.1 |
| `cmake` | 0.1.53 | 0.1.58 | `cargo update` moves it to 0.1.58 |
| `combine` | 4.6.7 | 4.6.8 | `cargo update` moves it to 4.6.8 |
| `cpufeatures` | 0.2.17 | 0.3.1 | held by `sha2` |
| `crc32fast` | 1.5.0 | 1.5.2 | `cargo update` moves it to 1.5.2 |
| `crossbeam-deque` | 0.8.6 | 0.8.8 | `cargo update` moves it to 0.8.8 |
| `crossbeam-epoch` | 0.9.18 | 0.9.21 | `cargo update` moves it to 0.9.21 |
| `crossbeam-utils` | 0.8.21 | 0.8.23 | `cargo update` moves it to 0.8.23 |
| `crypto-common` | 0.1.7 | 0.2.2 | held by `digest` |
| `daachorse` | 2.1.1 | 5.0.0 | held by `lindera`, `lindera-dictionary` |
| `darling` | 0.20.11 | 0.24.1 | held by `derive_builder_core` |
| `darling_core` | 0.20.11 | 0.24.1 | held by `darling`, `darling_macro` |
| `darling_macro` | 0.20.11 | 0.24.1 | held by `darling` |
| `digest` | 0.10.7 | 0.11.3 | held by `sha2` |
| `displaydoc` | 0.2.6 | 0.2.7 | `cargo update` moves it to 0.2.7 |
| `either` | 1.16.0 | 1.18.0 | `cargo update` moves it to 1.18.0 |
| `encoding_rs` | 0.8.35 | 0.8.42 | `cargo update` moves it to 0.8.42 |
| `encoding_rs_io` | 0.1.7 | 0.1.8 | `cargo update` moves it to 0.1.8 |
| `fastrand` | 2.4.1 | 2.5.0 | `cargo update` moves it to 2.5.0 |
| `find-msvc-tools` | 0.1.9 | 0.1.14 | `cargo update` moves it to 0.1.14 |
| `foldhash` | 0.1.5 | 0.2.0 | held by `hashbrown` |
| `futures-channel` | 0.3.32 | 0.3.34 | `cargo update` moves it to 0.3.34 |
| `futures-core` | 0.3.32 | 0.3.34 | `cargo update` moves it to 0.3.34 |
| `futures-task` | 0.3.32 | 0.3.34 | `cargo update` moves it to 0.3.34 |
| `futures-util` | 0.3.32 | 0.3.34 | `cargo update` moves it to 0.3.34 |
| `generic-array` | 0.14.7 | 1.4.5 | held by `block-buffer`, `crypto-common` |
| `getrandom` | 0.2.17 | 0.4.3 | held by `ring` |
| `getrandom` | 0.3.4 | 0.4.3 | held by `bnf`, `fastrand`, `jobserver` and others |
| `glob` | 0.3.3 | 0.3.4 | `cargo update` moves it to 0.3.4 |
| `hashbrown` | 0.14.5 | 0.17.1 | held by `dashmap` |
| `hashbrown` | 0.15.5 | 0.17.1 | held by `whatlang` |
| `hashbrown` | 0.16.1 | 0.17.1 | held by `bnf` |
| `hermit-abi` | 0.5.2 | 0.5.3 | `cargo update` moves it to 0.5.3 |
| `html-escape` | 0.2.13 | 0.2.15 | `cargo update` moves it to 0.2.15 |
| `http` | 1.4.1 | 1.5.0 | `cargo update` moves it to 1.5.0 |
| `http-body` | 1.0.1 | 1.1.0 | `cargo update` moves it to 1.1.0 |
| `http-body-util` | 0.1.3 | 0.1.5 | `cargo update` moves it to 0.1.5 |
| `hyper` | 1.10.1 | 1.11.1 | `cargo update` moves it to 1.11.1 |
| `hyper-rustls` | 0.27.9 | 0.27.10 | `cargo update` moves it to 0.27.10 |
| `hyper-util` | 0.1.20 | 0.1.21 | `cargo update` moves it to 0.1.21 |
| `icu_collections` | 2.2.0 | 2.3.0 | `cargo update` moves it to 2.3.0 |
| `icu_locale_core` | 2.2.0 | 2.3.0 | `cargo update` moves it to 2.3.0 |
| `icu_normalizer` | 2.2.0 | 2.3.0 | `cargo update` moves it to 2.3.0 |
| `icu_normalizer_data` | 2.2.0 | 2.3.0 | `cargo update` moves it to 2.3.0 |
| `icu_properties` | 2.2.0 | 2.3.0 | `cargo update` moves it to 2.3.0 |
| `icu_properties_data` | 2.2.0 | 2.3.0 | `cargo update` moves it to 2.3.0 |
| `icu_provider` | 2.2.0 | 2.3.1 | `cargo update` moves it to 2.3.1 |
| `indexmap` | 2.14.0 | 2.14.2 | `cargo update` moves it to 2.14.2 |
| `ipnet` | 2.12.0 | 2.12.2 | `cargo update` moves it to 2.12.2 |
| `itertools` | 0.12.1 | 0.15.0 | dropped by `cargo update` (dependents move to newer crates) |
| `itertools` | 0.14.0 | 0.15.0 | held by `lingua` |
| `jobserver` | 0.1.34 | 0.1.35 | `cargo update` moves it to 0.1.35 |
| `js-sys` | 0.3.99 | 0.3.106 | `cargo update` moves it to 0.3.106 |
| `libc` | 0.2.186 | 0.2.189 | `cargo update` moves it to 0.2.189 |
| `libloading` | 0.8.9 | 0.9.0 | dropped by `cargo update` (dependents move to newer crates) |
| `lindera-dictionary` | 3.0.7 | 6.2.0 | held by `lindera`, `lindera-jieba` |
| `lindera-jieba` | 3.0.7 | 6.2.0 | held by `lindera` |
| `linux-raw-sys` | 0.4.15 | 0.12.1 | dropped by `cargo update` (dependents move to newer crates) |
| `litemap` | 0.8.2 | 0.8.3 | `cargo update` moves it to 0.8.3 |
| `log` | 0.4.32 | 0.4.34 | `cargo update` moves it to 0.4.34 |
| `lru-slab` | 0.1.2 | 0.1.3 | `cargo update` moves it to 0.1.3 |
| `md5` | 0.8.0 | 0.8.1 | `cargo update` moves it to 0.8.1 |
| `memchr` | 2.8.0 | 2.8.3 | `cargo update` moves it to 2.8.3 |
| `memmap2` | 0.9.10 | 0.9.11 | `cargo update` moves it to 0.9.11 |
| `miniz_oxide` | 0.8.9 | 0.9.1 | `cargo update` moves it to 0.9.1 |
| `mio` | 1.2.1 | 1.2.3 | `cargo update` moves it to 1.2.3 |
| `nom` | 7.1.3 | 8.0.0 | held by `abnf`, `abnf-core`, `cexpr` and others |
| `parse-hyperlinks` | 0.23.4 | 0.29.3 | held by `ebnf` |
| `pest_generator` | 2.8.6 | 2.9.2 | `cargo update` moves it to 2.9.2 |
| `potential_utf` | 0.1.5 | 0.1.6 | `cargo update` moves it to 0.1.6 |
| `prettyplease` | 0.2.37 | 0.3.0 | dropped by `cargo update` (dependents move to newer crates) |
| `proc-macro2` | 1.0.103 | 1.0.107 | `cargo update` moves it to 1.0.107 |
| `ptr_meta` | 0.3.1 | 0.3.2 | `cargo update` moves it to 0.3.2 |
| `ptr_meta_derive` | 0.3.1 | 0.3.2 | `cargo update` moves it to 0.3.2 |
| `quinn` | 0.11.9 | 0.11.12 | `cargo update` moves it to 0.11.12 |
| `quinn-proto` | 0.11.14 | 0.11.18 | `cargo update` moves it to 0.11.18 |
| `quinn-udp` | 0.5.14 | 0.6.2 | `cargo update` moves it to 0.5.15; newer major held by a dependent |
| `quote` | 1.0.45 | 1.0.47 | `cargo update` moves it to 1.0.47 |
| `r-efi` | 5.3.0 | 7.1.0 | held by `getrandom` |
| `rancor` | 0.1.1 | 0.1.3 | `cargo update` moves it to 0.1.3 |
| `rand` | 0.9.4 | 0.10.3 | dropped by `cargo update` (dependents move to newer crates) |
| `rand` | 0.10.1 | 0.10.3 | dropped by `cargo update` (dependents move to newer crates) |
| `rand_chacha` | 0.9.0 | 0.10.0 | held by `rand` |
| `rand_core` | 0.9.5 | 0.10.1 | held by `rand`, `rand_chacha` |
| `redox_syscall` | 0.5.18 | 0.9.4 | held by `parking_lot_core` |
| `regex-automata` | 0.4.14 | 0.4.18 | `cargo update` moves it to 0.4.18 |
| `regex-syntax` | 0.8.10 | 0.8.11 | `cargo update` moves it to 0.8.11 |
| `rend` | 0.5.3 | 0.5.4 | `cargo update` moves it to 0.5.4 |
| `reqwest` | 0.13.4 | 0.13.5 | `cargo update` moves it to 0.13.5 |
| `ring` | 0.17.9 | 0.17.14 | `cargo update` moves it to 0.17.14 |
| `rkyv` | 0.8.16 | 0.8.18 | `cargo update` moves it to 0.8.18 |
| `rkyv_derive` | 0.8.16 | 0.8.18 | `cargo update` moves it to 0.8.18 |
| `rustc-hash` | 1.1.0 | 2.1.3 | dropped by `cargo update` (dependents move to newer crates) |
| `rustc-hash` | 2.1.2 | 2.1.3 | dropped by `cargo update` (dependents move to newer crates) |
| `rustix` | 0.38.44 | 1.1.5 | dropped by `cargo update` (dependents move to newer crates) |
| `rustix` | 1.1.4 | 1.1.5 | dropped by `cargo update` (dependents move to newer crates) |
| `rustls` | 0.23.31 | 0.23.45 | `cargo update` moves it to 0.23.45 |
| `rustls-pki-types` | 1.14.1 | 1.15.1 | `cargo update` moves it to 1.15.1 |
| `rustls-platform-verifier` | 0.7.0 | 0.7.1 | `cargo update` moves it to 0.7.1 |
| `rustls-platform-verifier-android` | 0.1.1 | 0.2.0 | `cargo update` moves it to 0.2.0 |
| `rustls-webpki` | 0.103.4 | 0.103.15 | `cargo update` moves it to 0.103.15 |
| `rustversion` | 1.0.22 | 1.0.23 | `cargo update` moves it to 1.0.23 |
| `serde_core` | 1.0.228 | 1.0.229 | `cargo update` moves it to 1.0.229 |
| `serde_derive` | 1.0.228 | 1.0.229 | `cargo update` moves it to 1.0.229 |
| `shlex` | 1.3.0 | 2.0.1 | dropped by `cargo update` (dependents move to newer crates) |
| `simd-adler32` | 0.3.9 | 0.3.10 | `cargo update` moves it to 0.3.10 |
| `simd_cesu8` | 1.1.1 | 1.2.0 | `cargo update` moves it to 1.2.0 |
| `smallvec` | 1.15.1 | 1.16.2 | `cargo update` moves it to 1.16.2 |
| `socket2` | 0.6.4 | 0.6.5 | `cargo update` moves it to 0.6.5 |
| `strum` | 0.27.2 | 0.28.0 | held by `lingua` |
| `strum_macros` | 0.27.2 | 0.28.0 | held by `lingua` |
| `syn` | 2.0.111 | 3.0.6 | dropped by `cargo update` (dependents move to newer crates) |
| `synstructure` | 0.13.2 | 0.14.0 | `cargo update` moves it to 0.14.0 |
| `thiserror` | 1.0.69 | 2.0.21 | held by `parse-hyperlinks` |
| `thiserror` | 2.0.18 | 2.0.21 | `cargo update` moves it to 2.0.21 |
| `thiserror-impl` | 1.0.69 | 2.0.21 | held by `thiserror` |
| `thiserror-impl` | 2.0.18 | 2.0.21 | `cargo update` moves it to 2.0.21 |
| `tinystr` | 0.8.3 | 0.8.4 | `cargo update` moves it to 0.8.4 |
| `tinyvec` | 1.11.0 | 1.13.3 | `cargo update` moves it to 1.13.3 |
| `tokio` | 1.52.3 | 1.53.1 | `cargo update` moves it to 1.53.1 |
| `tokio-macros` | 2.7.0 | 2.7.2 | `cargo update` moves it to 2.7.2 |
| `tokio-rustls` | 0.26.4 | 0.26.6 | `cargo update` moves it to 0.26.6 |
| `tower-http` | 0.6.11 | 0.7.1 | held by `reqwest` |
| `unicode-blocks` | 0.1.9 | 0.1.10 | `cargo update` moves it to 0.1.10 |
| `unicode-ident` | 1.0.22 | 1.0.26 | `cargo update` moves it to 1.0.26 |
| `utf8-width` | 0.1.8 | 0.1.9 | dropped by `cargo update` (dependents move to newer crates) |
| `uuid` | 1.23.2 | 1.26.1 | `cargo update` moves it to 1.26.1 |
| `wasi` | 0.11.1+wasi-snapshot-preview1 | 0.14.7+wasi-0.2.4 | held by `getrandom`, `mio` |
| `wasip2` | 1.0.3+wasi-0.2.9 | 2.0.1+wasi-0.2.12 | `cargo update` moves it to 1.0.4+wasi-0.2.12; newer major held by a dependent |
| `wasm-bindgen` | 0.2.122 | 0.2.129 | `cargo update` moves it to 0.2.129 |
| `wasm-bindgen-futures` | 0.4.72 | 0.4.79 | `cargo update` moves it to 0.4.79 |
| `wasm-bindgen-macro` | 0.2.122 | 0.2.129 | `cargo update` moves it to 0.2.129 |
| `wasm-bindgen-macro-support` | 0.2.122 | 0.2.129 | `cargo update` moves it to 0.2.129 |
| `wasm-bindgen-shared` | 0.2.122 | 0.2.129 | `cargo update` moves it to 0.2.129 |
| `web-sys` | 0.3.99 | 0.3.106 | `cargo update` moves it to 0.3.106 |
| `webpki-root-certs` | 1.0.7 | 1.0.9 | `cargo update` moves it to 1.0.9 |
| `which` | 4.4.2 | 8.0.6 | dropped by `cargo update` (dependents move to newer crates) |
| `windows-link` | 0.2.1 | 0.100.0 | held by `jni`, `libloading`, `parking_lot_core` and others |
| `windows-sys` | 0.52.0 | 0.61.2 | held by `ring` |
| `windows-sys` | 0.59.0 | 0.61.2 | dropped by `cargo update` (dependents move to newer crates) |
| `windows-sys` | 0.60.2 | 0.61.2 | dropped by `cargo update` (dependents move to newer crates) |
| `windows-targets` | 0.52.6 | 0.53.5 | held by `windows-sys` |
| `windows_aarch64_gnullvm` | 0.52.6 | 0.53.1 | held by `windows-targets` |
| `windows_aarch64_msvc` | 0.52.6 | 0.53.1 | held by `windows-targets` |
| `windows_i686_gnu` | 0.52.6 | 0.53.1 | held by `windows-targets` |
| `windows_i686_gnullvm` | 0.52.6 | 0.53.1 | held by `windows-targets` |
| `windows_i686_msvc` | 0.52.6 | 0.53.1 | held by `windows-targets` |
| `windows_x86_64_gnu` | 0.52.6 | 0.53.1 | held by `windows-targets` |
| `windows_x86_64_gnullvm` | 0.52.6 | 0.53.1 | held by `windows-targets` |
| `windows_x86_64_msvc` | 0.52.6 | 0.53.1 | held by `windows-targets` |
| `wit-bindgen` | 0.57.1 | 0.62.0 | held by `wasip2` |
| `writeable` | 0.6.3 | 0.6.4 | `cargo update` moves it to 0.6.4 |
| `yoke-derive` | 0.8.2 | 0.8.3 | `cargo update` moves it to 0.8.3 |
| `zerocopy` | 0.8.50 | 0.8.59 | `cargo update` moves it to 0.8.59 |
| `zerocopy-derive` | 0.8.50 | 0.8.59 | `cargo update` moves it to 0.8.59 |
| `zerofrom-derive` | 0.1.7 | 0.1.8 | `cargo update` moves it to 0.1.8 |
| `zeroize` | 1.8.2 | 1.9.0 | `cargo update` moves it to 1.9.0 |
| `zerotrie` | 0.2.4 | 0.2.5 | `cargo update` moves it to 0.2.5 |
| `zerovec` | 0.11.6 | 0.11.8 | `cargo update` moves it to 0.11.8 |
| `zerovec-derive` | 0.11.3 | 0.11.6 | `cargo update` moves it to 0.11.6 |
| `zmij` | 1.0.21 | 1.0.23 | `cargo update` moves it to 1.0.23 |

### Published artifact contents

`cargo package --list` for `meta-language` 0.58.2 lists 266 files. 31 of them
are the vendored grammar sources under
`vendor/tree-sitter-{rocq,csv,rust,lean,cmake}/` (compressed `parser.c.gz`,
scanners, headers, patches, licenses and notices), compiled by `build.rs`.
The data files are `src/data/grammar-lock.json`, `language-catalog.json`,
`language-trigrams.json` and `semantic-lexicon.json`.

## Website crate (`rust/web/`)

`rust/web` is the unpublished (`publish = false`) WebAssembly demo crate built
by the Pages job with `wasm-pack`. Its `Cargo.lock` locks 21 crates.io crates.

| Crate | Declared | Resolved | Current stable | Status | Reason |
|---|---|---|---|---|---|
| `links-notation` | `0.13` | 0.13.0 | 0.22.0 | behind | No recorded compatibility reason — must be updated. The main crate and the npm package already use 0.22.0, so the playground runs an older Links Notation than both runtimes. Needs a manifest change. |
| `wasm-bindgen` | `0.2` | 0.2.125 | 0.2.129 | behind | No recorded compatibility reason — must be updated; `cargo update` reaches 0.2.129. |
| `serde_json` | `1` | 1.0.150 | 1.0.151 | behind | No recorded compatibility reason — must be updated; `cargo update` reaches 1.0.151. |

Transitive crates behind in `rust/web/Cargo.lock`, all moved by
`cargo update` to the listed version: `cfg-if` 1.0.4 (1.0.5), `memchr` 2.8.2
(2.8.3), `proc-macro2` 1.0.106 (1.0.107), `quote` 1.0.45 (1.0.47),
`rustversion` 1.0.22 (1.0.23), `serde`, `serde_core` and `serde_derive`
1.0.228 (1.0.229), `syn` 2.0.117 (3.0.6), `unicode-ident` 1.0.24 (1.0.26),
`wasm-bindgen-macro`, `wasm-bindgen-macro-support` and `wasm-bindgen-shared`
0.2.125 (0.2.129), and `zmij` 1.0.21 (1.0.23).

## Experiment manifests (not published)

These manifests are tracked under `experiments/` and are not part of either
published package; none has a tracked lockfile.

| Manifest | Declared dependencies | Current stable | Status | Reason |
|---|---|---|---|---|
| `experiments/issue-195-cmake-scanner/Cargo.toml` | `tree-sitter =0.25.10`, `tree-sitter-language =0.1.7`, build `cc =1.2.63` | 0.27.0, 0.1.8, 1.5.1 | behind | Mirrors the main crate's pins; no recorded compatibility reason — must be updated with them. |
| `experiments/issue-195-projects/rust/Cargo.toml` | none | | not applicable | |
| `experiments/issue-195-projects/javascript/package.json` | none | | not applicable | |
| `experiments/issue-195-projects/lean/lean-toolchain` | `leanprover/lean4:v4.33.1` | v4.34.1 | behind | Same as the root `lean-toolchain`. |
| `experiments/issue-195-projects/rocq/_CoqProject` | no version | | not applicable | |

## Vendored grammar assets

### WebAssembly grammars and runtime (`js/src/vendor/`)

`js/src/vendor/grammars/grammar-lock.json` pins 60 grammars built by
`js/scripts/build-vendored-grammars.mjs` with tree-sitter CLI 0.25.10
(`da6fe9be`): 55 from the grammar crates locked in `rust/Cargo.lock` (several
crates provide more than one grammar, for example `tree-sitter-typescript`
provides `typescript` and `tsx`) and five from the vendored sources below. Each
WebAssembly grammar has the version of its source, so its status is the status
of the crate or vendored source in the tables above: the grammars built from
`tree-sitter-graphql`, `tree-sitter-lua`, `tree-sitter-ocaml` (`ocaml` and
`ocaml_interface`), `ts-parser-perl`, `tree-sitter-proto`, `tree-sitter-r` and
`tree-sitter-scala` are behind.

| Asset | Source | Pinned | Current stable (source) | Status | Reason |
|---|---|---|---|---|---|
| `js/src/vendor/web-tree-sitter/tree-sitter.wasm.gz` | `tree-sitter/tree-sitter` at `da6fe9be` with `utf8-input.patch`, built with emscripten 4.0.4 (`runtime-lock.json`) | 0.25.10 | v0.27.0 (GitHub release) | behind | No recorded compatibility reason — must be updated. The patch (read UTF-8 input like the native runtime) is recorded; a reason to stay on 0.25.10 is not. |
| 60 `js/src/vendor/grammars/*.wasm.gz` | grammar crates and vendored sources, tree-sitter CLI 0.25.10 | per `grammar-lock.json` | per source | 52 current, 8 behind | As above; the CLI itself is behind (see [Generators](#generators)). |

### Vendored generated C parsers (`rust/vendor/`)

`rust/build.rs` compiles these five generated parsers into the crate, and the
same sources build the corresponding WebAssembly grammars. Each `NOTICE.md`
records why the parser is vendored instead of used from a crate.

| Directory | Upstream | Pinned | Current stable (source) | Status | Reason for vendoring (recorded) |
|---|---|---|---|---|---|
| `rust/vendor/tree-sitter-cmake` | `uyha/tree-sitter-cmake` | v0.7.5 (`e997bd0b`) + `scanner-state.patch` | v0.7.5 (newest tag; crate `tree-sitter-cmake` 0.7.5); later untagged commits on the default branch | current | The external scanner read uninitialised memory; the patch starts it with no bracket open. |
| `rust/vendor/tree-sitter-csv` | `tree-sitter-grammars/tree-sitter-csv` | `f6bf6e35` + `rfc4180-quotes.patch` | v1.2.0 (newest tag and crate); the pin is master 9 commits after v1.2.0 and is the upstream head | current | Needs upstream fix `4934ce2`, made after the 1.2.0 crate, which also pins `cc ~1.0.82` against the other grammar crates; the patch restores RFC 4180 quoting. |
| `rust/vendor/tree-sitter-lean` | `wvhulle/tree-sitter-lean` | `bd942cd2` (the sources of crate `tree-sitter-lean4` 0.3.0) | crate `tree-sitter-lean4` 0.3.0 (crates.io); upstream head is 5 commits later, unreleased | current | The crate's build script compiles without MSVC `/utf-8`, so node names are not UTF-8 on Windows. |
| `rust/vendor/tree-sitter-rocq` | `aruzdh/tree-sitter-rocq` | `300fe33f` + `meta-language.patch` | no release after tag v0.2.0; the pin is the upstream head, 2 commits after v0.2.0 | current | The patch adds constructs that translator output and the pinned Rocq 9.2.0 stdlib use and the upstream grammar reports as errors. |
| `rust/vendor/tree-sitter-rust` | `tree-sitter/tree-sitter-rust` | v0.24.2 (`77a37472`) + `meta-language.patch` | v0.24.2 (GitHub release; crate `tree-sitter-rust` 0.24.2) | current | The patch accepts syntax real published crates use that the release reports as errors. |

## Production parser dependencies

The vision removes the production parser dependencies — tree-sitter,
web-tree-sitter and the vendored grammar binaries — as native grammars replace
them, and ledger row `I195-DEPENDENCY-PRODUCTION-PARSERS-REMOVED` adds `peggy`
and any other external grammar engine. On the audit date they are all still
production dependencies:

- `rust/Cargo.toml` declares `tree-sitter =0.25.10`, `tree-sitter-language`
  and 51 grammar crates in `[dependencies]`, and `build.rs` compiles the five
  vendored parsers. 15 files under `rust/src` use `tree_sitter`, including
  `tree_sitter_adapter.rs`, `language_parser.rs`, `incremental.rs`,
  `embedded_region_parser.rs`, `structured_text_parser.rs`, `docx_parser.rs`,
  `parity.rs`, `lib.rs`, `main.rs` and the `grammar/import` and `grammar/emit`
  tree-sitter modules. The published crate ships `vendor/tree-sitter-*`.
- `js/package.json` declares `web-tree-sitter` 0.25.10 in `dependencies`.
  `js/src/programming-language-parser.js` imports it, loads the patched
  runtime from `src/vendor/web-tree-sitter/tree-sitter.wasm.gz` and the
  grammars from `src/vendor/grammars/`, and is imported by `js/src/index.js`,
  `js/src/network.js` and `js/src/regions.js`. The published npm package ships
  the 61 WebAssembly binaries.
- `js/package.json` declares `peggy` 5.1.0 in `dependencies`;
  `js/src/grammar.js` imports it and also emits parsers that import it.

The tree-sitter CLI remains an independent oracle for the fixture generators
(see below), which is the test and development role the vision keeps.

## Generators

| Generator | Output | External tools and pins | Status |
|---|---|---|---|
| `js/scripts/build-vendored-grammars.mjs` | `js/src/vendor/grammars/*.wasm.gz`, both `grammar-lock.json` files, `NOTICE.md` | `cargo fetch`, tree-sitter CLI 0.25.10, Docker or emcc | behind (CLI) |
| `js/scripts/build-web-tree-sitter-runtime.mjs` | `js/src/vendor/web-tree-sitter/tree-sitter.wasm.gz` | git, Docker image `emscripten/emsdk:4.0.4` | behind (image, runtime) |
| `js/scripts/generate-default-cst-expectations.mjs` | `parity/fixtures/default-cst-expected.json` | vendored grammars; `--cli` cross-checks with tree-sitter CLI | behind (CLI) |
| `js/scripts/generate-issue-195-conformance.mjs` | `parity/fixtures/issue-195-conformance/` | tree-sitter CLI, required to equal the lock's 0.25.10; upstream corpora at pinned tags (`leanprover/lean4` v4.33.1 among them) | behind (CLI, Lean tag) |
| `js/scripts/generate-issue-195-generative.mjs` | `parity/fixtures/issue-195-generative/` | tree-sitter CLI | behind (CLI) |
| `js/scripts/generate-builtin-cst-expectations.mjs` | `parity/fixtures/builtin-cst-expected.json` | `Intl.Segmenter` (Node.js ICU), `links-notation`, `pdf-lib` | current |
| `js/scripts/generate-lino-grammar-cases.mjs` | `parity/fixtures/lino-grammar-cases.json` | `links-notation` 0.22.0 | current |
| `js/scripts/generate-pdf-grammar-cases.mjs` | `parity/fixtures/pdf-grammar-cases.json` | `pdf-lib` 1.17.1 | current |
| `js/scripts/build-language-catalog.mjs` | both `language-catalog.json` copies | grammar lock, `parity/language-grammar-inventory.json` | not applicable |
| `js/scripts/build-language-identification.mjs` | `language-trigrams.json` and `language-trigrams.js` | `franc-min` 6.2.0 | current |
| `js/scripts/build-bidi-table.mjs` | `js/src/unicode-bidi-classes.js` | `unicode-bidi` 0.3.18 crate tables | current |
| `js/scripts/build-lean-root-names.mjs` | `js/src/translation/lean-root-names.js`, `rust/src/translation/lean_root_names.rs` | `lean` on `PATH` (4.33.1 by `lean-toolchain`) | behind (Lean) |
| `js/scripts/build-translation-stage-fixtures.mjs` | `parity/fixtures/translation-stages.json` | none | not applicable |
| `rust/scripts/build-site.rs` | website under `_site` | `rust-script` | current |

## Toolchains

| Toolchain | Declared in | Pin | Current stable release (source) | Status | Reason |
|---|---|---|---|---|---|
| Node.js (JavaScript test job) | `.github/workflows/js.yml` | `node-version: 20` | 24.21.0 LTS, 26.10.0 Current (nodejs.org) | behind | Node.js 20 reached end of life on 2026-04-30. Tests the `engines` floor; no recorded compatibility reason — must be updated. |
| Node.js (publish, Rust, acceptance jobs) | `js.yml`, `rust.yml`, `issue-195-acceptance.yml` | `node-version: 24` | 24.21.0 LTS | current | Resolves to the newest 24.x. 26.x is not LTS until 2026-10-28. |
| npm (publish job) | `.github/workflows/js.yml` | `npm install -g npm@11` | 12.1.0 (`npm view npm dist-tags`) | behind | Installed for trusted publishing; no recorded reason to stay on 11.x — must be updated. npm 12 supports Node.js `^24.15.0`. |
| Rust (most jobs) | `.github/workflows/rust.yml` | `dtolnay/rust-toolchain@stable` | 1.98.1 (`rust-lang/rust` release) | current | Floating. |
| Rust (acceptance) | `.github/workflows/issue-195-acceptance.yml` | `dtolnay/rust-toolchain@1.98.1` | 1.98.1 | current | |
| elan | `issue-195-acceptance.yml` (`ELAN_VERSION`, SHA-256 checked) | v4.2.4 | v4.2.4 (`leanprover/elan` release) | current | |
| Lean 4 | `lean-toolchain`, `issue-195-acceptance.yml`, `docs/four-language-contracts.md`, `js/scripts/issue-195-requirements.mjs` | v4.33.1 | v4.34.1 (`leanprover/lean4` release, 2026-09-24) | behind | 4.33.1 is the declared Lean profile of the four-language contract; no recorded compatibility reason — must be updated. |
| OCaml (for Rocq) | `issue-195-acceptance.yml` via `ocaml/setup-ocaml` | `ocaml-compiler: "5.4"` | 5.5.1 (`ocaml/ocaml` release) | behind | No recorded compatibility reason — must be updated. `rocq-runtime` 9.2.0 requires only `ocaml >= 4.14.0`. |
| Rocq | `issue-195-acceptance.yml`, `docs/four-language-contracts.md`, `js/scripts/issue-195-requirements.mjs` | `rocq-core=9.2.0`, `rocq-stdlib=9.2.0`, `rocq-prover=meta.1` | V9.3.0 (`rocq-prover/rocq` release, 2026-09-19) | behind | 9.2 is the declared Rocq profile. No reason is recorded in the repository. Externally, `ocaml/opam-repository` lists `rocq-core` only up to 9.2.0 on the audit date (the 9.3.0 package is open pull request #30776), so 9.3.0 is not yet installable through opam. |
| tree-sitter CLI (generators) | `grammar-lock.json` `treeSitterCli`, generator headers | 0.25.10 | 0.27.0 (crates.io `tree-sitter-cli`, npm `tree-sitter-cli`) | behind | Tied to the `tree-sitter` 0.25.10 pin; no recorded compatibility reason — must be updated. |
| emscripten (generators) | `runtime-lock.json`, `build-web-tree-sitter-runtime.mjs` | 4.0.4 | 6.0.10 (`emscripten-core/emsdk` tags) | behind | 4.0.4 is the version tree-sitter 0.25.10 declares in `cli/loader/emscripten-version` (0.27.0 declares 4.0.15). Follows the tree-sitter pin, which has no recorded reason — must be updated. |
| rust-script | `rust/scripts/install-rust-script.sh` (`cargo install rust-script --locked`) | unpinned | 0.36.0 (crates.io) | current | Installs the newest release. |
| wasm-pack | `rust.yml` via `taiki-e/install-action@v2` (`tool: wasm-pack`) | unpinned | v0.15.0 (`wasm-bindgen/wasm-pack` release) | current | Installs the newest release. |
| cargo-llvm-cov | `rust.yml` via `taiki-e/install-action@cargo-llvm-cov` | unpinned | v0.9.1 (`taiki-e/cargo-llvm-cov` release) | current | Installs the newest release. |
| secretlint | `js.yml`, `rust.yml` (`npx --yes -p secretlint -p @secretlint/secretlint-rule-preset-recommend`) | unpinned | 13.0.6 (npm) | current | Runs the newest release. |
| pre-commit hooks | `.pre-commit-config.yaml` | `pre-commit/pre-commit-hooks` `rev: v5.0.0` | v6.0.0 (GitHub release) | behind | No recorded compatibility reason — must be updated. |
| Python | none | none | | not applicable | No workflow or script requires Python; tracked `.py` files are benchmark corpora and one experiment. |

## GitHub Actions

Source of the current release: `gh api repos/<owner>/<repo>/releases/latest`.
A major tag such as `v6` resolves to the newest release of that major, so a
major tag is current when it equals the latest release's major.

| Action | Workflows | Pinned | Latest release | Status | Reason |
|---|---|---|---|---|---|
| `actions/checkout` | all three (19 uses) | `v6` | v7.0.1 | behind | No recorded compatibility reason — must be updated. |
| `actions/cache` | `rust.yml` (6 uses) | `v5` | v6.1.0 | behind | No recorded compatibility reason — must be updated. |
| `actions/setup-node` | all three (6 uses) | `v6` | v7.0.0 | behind | No recorded compatibility reason — must be updated. |
| `actions/download-artifact` | `issue-195-acceptance.yml` (3 uses) | `v7` | v8.0.1 | behind | No recorded compatibility reason — must be updated. |
| `actions/upload-artifact` | `issue-195-acceptance.yml` (4 uses) | `v7` | v7.0.1 | current | |
| `actions/configure-pages` | `rust.yml` | `v6` | v6.0.0 | current | |
| `actions/deploy-pages` | `rust.yml` | `v5` | v5.0.1 | current | |
| `actions/upload-pages-artifact` | `rust.yml` | `v5` | v5.0.0 | current | |
| `codecov/codecov-action` | `rust.yml` | `v7` | v7.1.1 | current | |
| `docker/build-push-action` | `rust.yml` (2 uses) | `v7` | v7.4.0 | current | |
| `docker/login-action` | `rust.yml` (2 uses) | `v4` | v4.6.0 | current | |
| `docker/metadata-action` | `rust.yml` (2 uses) | `v6` | v6.2.0 | current | |
| `docker/setup-buildx-action` | `rust.yml` (2 uses) | `v4` | v4.4.1 | current | |
| `dtolnay/rust-toolchain` | `rust.yml` (13 uses) | `stable` branch | branch installs Rust 1.98.1, the latest Rust | current | |
| `dtolnay/rust-toolchain` | `issue-195-acceptance.yml` (3 uses) | `1.98.1` branch | installs Rust 1.98.1, the latest Rust | current | |
| `ocaml/setup-ocaml` | `issue-195-acceptance.yml` | `v3` | v3.9.0 | current | |
| `peter-evans/create-pull-request` | `rust.yml` | `v8` | v8.1.1 | current | |
| `taiki-e/install-action` | `rust.yml` (wasm-pack) | `v2` | v2.87.21 | current | |
| `taiki-e/install-action` | `rust.yml` | `cargo-llvm-cov` tool tag | installs cargo-llvm-cov v0.9.1 | current | |

No action is pinned to a commit SHA.

## Build images and runners

| Item | Declared in | Pin | Current stable release (source) | Status | Reason |
|---|---|---|---|---|---|
| `emscripten/emsdk` Docker image | `js/scripts/build-web-tree-sitter-runtime.mjs` (`emscripten/emsdk:${SOURCE.emscripten}`, from `runtime-lock.json`) | 4.0.4 | 6.0.10 (Docker Hub tags) | behind | See emscripten in [Toolchains](#toolchains). |
| Docker Hub release image | `rust.yml` (`docker/build-push-action`, context `.`) | no Dockerfile | | not applicable | The repository has no `Dockerfile`; the job disables Docker Hub publishing when none is found at the root. |
| GitHub-hosted runners | all workflows | `ubuntu-latest`, `macos-latest`, `windows-latest` | | current | Floating labels. |

No workflow declares a `container:` or service `image:`.

## Behind the current stable release

Every item below is behind the current stable release on 2026-09-29. Except
where a reason is quoted, no compatibility reason is recorded in the
repository and the item must be updated.

npm (`js/`):

- `engines.node` `>=20` admits end-of-life Node.js 20 (current LTS 24.21.0).
- `web-tree-sitter` 0.25.10 → 0.27.0 (production parser dependency).
- Transitive, held by the latest release of their dependent: `@peggyjs/from-mem`
  3.1.3 → 3.1.4, `commander` 14.0.3 → 15.0.0, `semver` 7.7.4 → 7.8.5,
  `source-map-generator` 2.0.6 → 2.0.7 (all via `peggy` 5.1.0); `pako`
  1.0.11 → 3.0.2 and `tslib` 1.14.1 → 2.8.1 (via `pdf-lib` 1.17.1,
  development only).

Rust (`rust/`):

- `edition` 2021 → 2024; `rust-version` 1.77 is lower than the locked crates
  require.
- Within the declared ranges (`cargo update`): `clap` 4.5.60 → 4.6.7,
  `pest_meta`, `pest`, `pest_derive` 2.8.6 → 2.9.2, `regex` 1.12.3 → 1.13.1,
  `serde` 1.0.228 → 1.0.229, `serde_json` 1.0.150 → 1.0.151,
  `tree-sitter-language` 0.1.7 → 0.1.8, `flate2` 1.1.9 → 1.1.10, `cc` 1.2.63
  → 1.5.1 (`cargo update` reaches 1.2.67).
- Needing a manifest change: `tree-sitter` =0.25.10 → 0.27.0, `lindera` 3.0.7
  → 6.2.0 (optional), `doublets` 0.4.0 → 0.5.0 (optional), `sha2` 0.10.9 →
  0.11.0.
- Grammar crates (exact pins): `tree-sitter-graphql` 0.1.0 → 0.2.1,
  `tree-sitter-lua` 0.2.0 → 0.5.0, `tree-sitter-ocaml` 0.24.2 → 0.26.0,
  `ts-parser-perl` 1.1.2 → 2.0.0, `tree-sitter-proto` 0.4.0 → 0.6.0,
  `tree-sitter-r` 1.2.0 → 1.3.0, `tree-sitter-scala` 0.25.1 → 0.26.2, and
  the eight WebAssembly grammars built from them.
- 182 transitive crate versions (table above): 119 reachable with
  `cargo update`, 18 dropped by it, 45 held by a dependent's range.
- `rust/web`: `links-notation` 0.13.0 → 0.22.0, `wasm-bindgen` 0.2.125 →
  0.2.129, `serde_json` 1.0.150 → 1.0.151, and 14 transitive crates.
- `experiments/issue-195-cmake-scanner`: `tree-sitter` =0.25.10,
  `tree-sitter-language` =0.1.7, `cc` =1.2.63.

Vendored assets, generators, toolchains and images:

- Patched web-tree-sitter runtime 0.25.10 → 0.27.0.
- tree-sitter CLI 0.25.10 → 0.27.0.
- emscripten and the `emscripten/emsdk` image 4.0.4 → 6.0.10 (4.0.4 is what
  tree-sitter 0.25.10 declares).
- Node.js 20 in the JavaScript test job → 24.21.0 LTS (20 is end of life).
- npm 11 in the publish job → 12.1.0.
- Lean v4.33.1 (`lean-toolchain`, acceptance workflow, experiment project) →
  v4.34.1.
- OCaml 5.4 → 5.5.1 in the acceptance workflow.
- Rocq 9.2.0 → 9.3.0; externally, 9.3.0 is not yet in `ocaml/opam-repository`
  (pull request #30776 is open).
- `pre-commit/pre-commit-hooks` v5.0.0 → v6.0.0.

GitHub Actions:

- `actions/checkout` v6 → v7.0.1, `actions/cache` v5 → v6.1.0,
  `actions/setup-node` v6 → v7.0.0, `actions/download-artifact` v7 → v8.0.1.
