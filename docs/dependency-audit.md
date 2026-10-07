> This document is subordinate to the authoritative
> [vision and architecture specification](vision.md)
> and is the dependency inventory that its Dependencies section refers to.

<!-- Generated from parity/dependency-inventory.json by `npm run dependencies:refresh` (js/scripts/check-dependencies.mjs). Do not edit by hand. -->

# Dependency audit

Audit date: 2026-10-06

This inventory lists every runtime, development, build and optional dependency
of the two published packages and the website crate, every lockfile
resolution, the experiment manifests, the vendored grammar assets, the
generators, the toolchains, the GitHub Actions, the build images, the runners
and the published artifact contents, each with the current stable release its
source reported on the audit date.

## Method

`node js/scripts/check-dependencies.mjs --refresh` (`npm run dependencies:refresh`)
reads every pin from the repository, queries the npm registry, the crates.io
index, GitHub releases, nodejs.org and Docker Hub for the current stable
release, derives the reason of a locked package that a dependent's requirement
holds from `js/package-lock.json` and `cargo metadata`, and rewrites
`parity/dependency-inventory.json` and this document. Every other reason is
recorded by hand in the inventory.

`npm run check:dependencies`, which CI runs, is offline. It fails when the
repository declares an item the inventory does not list, or a pin the
inventory does not record; when the audit date is missing or invalid; and when
a retained item is behind its current stable release without a recorded
compatibility reason.

`npm run check:dependencies:delivery` is the delivery gate. A recorded reason
does not count as an upgrade: an item behind its current stable release is
delivered only at its newest compatible release, the newest stable release
that every requirement holding it admits, which the refresh records with the
holders whose requirements exclude the current release. Each holder must be an
inventoried item that is itself delivered, or an external package at its
newest release; anything else, and anything built from it, is stale. Pull
request checks compare offline against this committed audit. With `--live`,
which runs only on main, the gate refreshes the inventory from the registries
in memory and fails when a current or compatible release differs from this
audit; the scheduled dependency refresh on main then opens a pull request that
moves the item to it.

Comparisons: `version` means the pin must be at least the current release;
`major` and `minor` mean a moving tag (`v7`, `5.4`) that must name the current
release line; `floor` means a supported minimum (the `engines` floor must be a
maintained Node.js line, `rust-version` must be at least what the resolved
crates declare); `revision` means a vendored commit that must contain the
upstream release; `floating` means the newest release is installed on every
run; `derived` means an artifact built from the items it names, behind when
they are.

## Summary

| Category | Items | Current | Behind | Not applicable |
|---|---|---|---|---|
| JavaScript engines | 1 | 1 | 0 | 0 |
| npm packages | 17 | 17 | 0 | 0 |
| Rust package settings | 6 | 6 | 0 | 0 |
| Rust crates | 306 | 282 | 24 | 0 |
| Experiment manifests | 6 | 6 | 0 | 0 |
| Vendored generated parsers | 5 | 5 | 0 | 0 |
| Vendored runtime | 1 | 0 | 1 | 0 |
| Vendored WebAssembly grammars | 60 | 60 | 0 | 0 |
| Generators | 23 | 22 | 1 | 0 |
| Toolchains and tools | 16 | 14 | 1 | 1 |
| GitHub Actions | 20 | 20 | 0 | 0 |
| Build images | 1 | 0 | 1 | 0 |
| Runners | 3 | 3 | 0 | 0 |
| Published artifact contents | 18 | 0 | 0 | 18 |

## JavaScript engines

| Item | Declared in | Pinned | Current stable release | Comparison | Status | Reason |
|---|---|---|---|---|---|---|
| `node` | `js/package.json` | `>=22` | 22 (nodejs/Release schedule, oldest maintained LTS line (maintained: 22, 24)) | floor | current |  |

## npm packages

| Item | Scope | Declared in | Pinned | Role | Requirement | Current stable release | Comparison | Status | Reason |
|---|---|---|---|---|---|---|---|---|---|
| `@pdf-lib/standard-fonts` | `js/package-lock.json` | `js/package-lock.json` | `1.0.0` | transitive, development |  | 1.0.0 (npm registry, @pdf-lib/standard-fonts latest) | version | current |  |
| `@pdf-lib/upng` | `js/package-lock.json` | `js/package-lock.json` | `1.0.1` | transitive, development |  | 1.0.1 (npm registry, @pdf-lib/upng latest) | version | current |  |
| `@peggyjs/from-mem` | `js/package-lock.json` | `js/package-lock.json` | `3.1.4` | transitive, development |  | 3.1.4 (npm registry, @peggyjs/from-mem latest) | version | current |  |
| `collapse-white-space` | `js/package-lock.json` | `js/package-lock.json` | `2.1.0` | transitive, development |  | 2.1.0 (npm registry, collapse-white-space latest) | version | current |  |
| `commander` | `js/package-lock.json` | `js/package-lock.json` | `15.0.0` | transitive, development |  | 15.0.0 (npm registry, commander latest) | version | current |  |
| `franc-min` | `js/package-lock.json` | `js/package-lock.json`, `js/package.json` | `6.2.0` | direct, development | `6.2.0` | 6.2.0 (npm registry, franc-min latest) | version | current |  |
| `links-notation` | `js/package-lock.json` | `js/package-lock.json`, `js/package.json` | `0.22.0` | direct, runtime | `0.22.0` | 0.22.0 (npm registry, links-notation latest) | version | current |  |
| `n-gram` | `js/package-lock.json` | `js/package-lock.json` | `2.0.2` | transitive, development |  | 2.0.2 (npm registry, n-gram latest) | version | current |  |
| `pako` | `js/package-lock.json` | `js/package-lock.json` | `3.0.2` | transitive, development |  | 3.0.2 (npm registry, pako latest) | version | current |  |
| `pdf-lib` | `js/package-lock.json` | `js/package-lock.json`, `js/package.json` | `1.17.1` | direct, development | `1.17.1` | 1.17.1 (npm registry, pdf-lib latest) | version | current |  |
| `peggy` | `js/package-lock.json` | `js/package-lock.json`, `js/package.json` | `5.1.0` | direct, development | `5.1.0` | 5.1.0 (npm registry, peggy latest) | version | current |  |
| `semver` | `js/package-lock.json` | `js/package-lock.json` | `7.8.5` | transitive, development |  | 7.8.5 (npm registry, semver latest) | version | current |  |
| `source-map-generator` | `js/package-lock.json` | `js/package-lock.json` | `2.0.7` | transitive, development |  | 2.0.7 (npm registry, source-map-generator latest) | version | current |  |
| `trigram-utils` | `js/package-lock.json` | `js/package-lock.json` | `2.0.1` | transitive, development |  | 2.0.1 (npm registry, trigram-utils latest) | version | current |  |
| `tslib` | `js/package-lock.json` | `js/package-lock.json` | `2.8.1` | transitive, development |  | 2.8.1 (npm registry, tslib latest) | version | current |  |
| `web-tree-sitter` | `js/package-lock.json` | `js/package-lock.json`, `js/package.json` | `0.27.0` | direct, runtime | `0.27.0` | 0.27.0 (npm registry, web-tree-sitter latest) | version | current |  |
| `wordnet-db` | `js/package-lock.json` | `js/package-lock.json`, `js/package.json` | `3.1.14` | direct, development | `3.1.14` | 3.1.14 (npm registry, wordnet-db latest) | version | current |  |

## Rust package settings

| Item | Scope | Declared in | Pinned | Current stable release | Comparison | Status | Reason |
|---|---|---|---|---|---|---|---|
| `edition` | `experiments/grammar-merge-rust-stub/Cargo.toml` | `experiments/grammar-merge-rust-stub/Cargo.toml` | `2024` | 2024 (Rust editions, the newest stable edition) | version | current |  |
| `edition` | `experiments/issue-195-cmake-scanner/Cargo.toml` | `experiments/issue-195-cmake-scanner/Cargo.toml` | `2024` | 2024 (Rust editions, the newest stable edition) | version | current |  |
| `edition` | `experiments/issue-195-projects/rust/Cargo.toml` | `experiments/issue-195-projects/rust/Cargo.toml` | `2024` | 2024 (Rust editions, the newest stable edition) | version | current |  |
| `edition` | `rust/Cargo.toml` | `rust/Cargo.toml` | `2024` | 2024 (Rust editions, the newest stable edition) | version | current |  |
| `rust-version` | `rust/Cargo.toml` | `rust/Cargo.toml` | `1.90` | 1.90 (cargo metadata, the highest rust-version of the resolved crates (meta-language 0.58.2, tree-sitter 0.27.0, tree-sitter-graphql 0.3.0)) | floor | current |  |
| `edition` | `rust/web/Cargo.toml` | `rust/web/Cargo.toml` | `2024` | 2024 (Rust editions, the newest stable edition) | version | current |  |

## Rust crates

| Item | Scope | Declared in | Pinned | Role | Requirement | Current stable release | Comparison | Status | Reason |
|---|---|---|---|---|---|---|---|---|---|
| `adler2` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.0.1` | transitive |  | 2.0.1 (crates.io, adler2) | version | current |  |
| `aho-corasick` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.1.5` | transitive |  | 1.1.5 (crates.io, aho-corasick) | version | current |  |
| `allocator-api2` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.21` | transitive |  | 0.4.0 (crates.io, allocator-api2) | version | behind | Held by the requirement of `hashbrown` 0.15.5 (`^0.2.9`), which does not admit 0.4.0; it moves when that dependent does. |
| `allocator-api2` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.4.0` | transitive |  | 0.4.0 (crates.io, allocator-api2) | version | current |  |
| `anstream` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.0` | transitive |  | 1.0.0 (crates.io, anstream) | version | current |  |
| `anstyle-parse` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.0` | transitive |  | 1.0.0 (crates.io, anstyle-parse) | version | current |  |
| `anstyle-query` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.1.5` | transitive |  | 1.1.5 (crates.io, anstyle-query) | version | current |  |
| `anstyle-wincon` | `rust/Cargo.lock` | `rust/Cargo.lock` | `3.0.11` | transitive |  | 3.0.11 (crates.io, anstyle-wincon) | version | current |  |
| `anstyle` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.14` | transitive |  | 1.0.14 (crates.io, anstyle) | version | current |  |
| `anyhow` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.104` | transitive |  | 1.0.104 (crates.io, anyhow) | version | current |  |
| `autocfg` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.5.1` | transitive |  | 1.5.1 (crates.io, autocfg) | version | current |  |
| `base64` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.23.1` | transitive |  | 0.23.1 (crates.io, base64) | version | current |  |
| `beef` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.5.2` | transitive |  | 0.5.2 (crates.io, beef) | version | current |  |
| `bitflags` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.13.2` | transitive |  | 2.13.2 (crates.io, bitflags) | version | current |  |
| `block-buffer` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.12.1` | transitive |  | 0.12.1 (crates.io, block-buffer) | version | current |  |
| `bumpalo` | `rust/Cargo.lock` | `rust/Cargo.lock` | `3.20.3` | transitive |  | 3.20.3 (crates.io, bumpalo) | version | current |  |
| `bytecheck_derive` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.3` | transitive |  | 0.8.3 (crates.io, bytecheck_derive) | version | current |  |
| `bytecheck` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.3` | transitive |  | 0.8.3 (crates.io, bytecheck) | version | current |  |
| `byteorder` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.5.0` | transitive |  | 1.5.0 (crates.io, byteorder) | version | current |  |
| `bytes` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.12.1` | transitive |  | 1.12.1 (crates.io, bytes) | version | current |  |
| `cc` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.2.67` | direct, build | `1.2` | 1.6.0 (crates.io, cc) | version | behind | Held by the requirement of `tree-sitter-sequel` 0.3.11 (`~1.2.1`), which does not admit 1.6.0; it moves when that dependent does. |
| `cfg-if` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.5` | transitive |  | 1.0.5 (crates.io, cfg-if) | version | current |  |
| `clap_builder` | `rust/Cargo.lock` | `rust/Cargo.lock` | `4.6.7` | transitive |  | 4.6.7 (crates.io, clap_builder) | version | current |  |
| `clap_derive` | `rust/Cargo.lock` | `rust/Cargo.lock` | `4.6.7` | transitive |  | 4.6.7 (crates.io, clap_derive) | version | current |  |
| `clap_lex` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.1.1` | transitive |  | 1.1.1 (crates.io, clap_lex) | version | current |  |
| `clap` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `4.6.7` | direct, runtime | `4.4` | 4.6.7 (crates.io, clap) | version | current |  |
| `colorchoice` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.5` | transitive |  | 1.0.5 (crates.io, colorchoice) | version | current |  |
| `const-oid` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.10.2` | transitive |  | 0.10.2 (crates.io, const-oid) | version | current |  |
| `core_detect` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.0` | transitive |  | 1.0.0 (crates.io, core_detect) | version | current |  |
| `counter` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.7.1` | transitive |  | 0.7.1 (crates.io, counter) | version | current |  |
| `cpufeatures` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.3.1` | transitive |  | 0.3.1 (crates.io, cpufeatures) | version | current |  |
| `crawdad` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.4.1` | transitive |  | 0.4.1 (crates.io, crawdad) | version | current |  |
| `crc32fast` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.5.2` | transitive |  | 1.5.2 (crates.io, crc32fast) | version | current |  |
| `crossbeam-deque` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.8` | transitive |  | 0.8.8 (crates.io, crossbeam-deque) | version | current |  |
| `crossbeam-epoch` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.9.21` | transitive |  | 0.9.21 (crates.io, crossbeam-epoch) | version | current |  |
| `crossbeam-utils` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.23` | transitive |  | 0.8.23 (crates.io, crossbeam-utils) | version | current |  |
| `crypto-common` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.2` | transitive |  | 0.2.2 (crates.io, crypto-common) | version | current |  |
| `csv-core` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.1.13` | transitive |  | 0.1.13 (crates.io, csv-core) | version | current |  |
| `csv` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.4.0` | transitive |  | 1.4.0 (crates.io, csv) | version | current |  |
| `daachorse` | `rust/Cargo.lock` | `rust/Cargo.lock` | `5.0.0` | transitive |  | 5.0.0 (crates.io, daachorse) | version | current |  |
| `dashmap` | `rust/Cargo.lock` | `rust/Cargo.lock` | `6.2.1` | transitive |  | 6.2.1 (crates.io, dashmap) | version | current |  |
| `digest` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.11.3` | transitive |  | 0.11.3 (crates.io, digest) | version | current |  |
| `displaydoc` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.7` | transitive |  | 0.2.7 (crates.io, displaydoc) | version | current |  |
| `doublets` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.5.0` | direct, runtime (optional) | `0.5.0` | 0.5.0 (crates.io, doublets) | version | current |  |
| `either` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.18.0` | transitive |  | 1.18.0 (crates.io, either) | version | current |  |
| `encoding_rs_io` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.1.8` | transitive |  | 0.1.8 (crates.io, encoding_rs_io) | version | current |  |
| `encoding_rs` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.42` | transitive |  | 0.8.42 (crates.io, encoding_rs) | version | current |  |
| `equivalent` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.2` | transitive |  | 1.0.2 (crates.io, equivalent) | version | current |  |
| `errno` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.3.14` | transitive |  | 0.3.14 (crates.io, errno) | version | current |  |
| `fastrand` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.5.0` | transitive |  | 2.5.0 (crates.io, fastrand) | version | current |  |
| `filetime` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.29` | transitive |  | 0.2.29 (crates.io, filetime) | version | current |  |
| `find-msvc-tools` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.1.14` | transitive |  | 0.1.14 (crates.io, find-msvc-tools) | version | current |  |
| `flate2` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.1.10` | direct, build | `1` | 1.1.10 (crates.io, flate2) | version | current |  |
| `foldhash` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.1.5` | transitive |  | 0.2.0 (crates.io, foldhash) | version | behind | Held by the requirement of `hashbrown` 0.15.5 (`^0.1.2`), which does not admit 0.2.0; it moves when that dependent does. |
| `form_urlencoded` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.2.2` | transitive |  | 1.2.2 (crates.io, form_urlencoded) | version | current |  |
| `fst` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.4.7` | transitive |  | 0.4.7 (crates.io, fst) | version | current |  |
| `futures-core` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.3.34` | transitive |  | 0.3.34 (crates.io, futures-core) | version | current |  |
| `futures-task` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.3.34` | transitive |  | 0.3.34 (crates.io, futures-task) | version | current |  |
| `futures-util` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.3.34` | transitive |  | 0.3.34 (crates.io, futures-util) | version | current |  |
| `getrandom` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.17` | transitive |  | 0.4.3 (crates.io, getrandom) | version | behind | Held by the requirement of `ring` 0.17.14 (`^0.2.10`), which does not admit 0.4.3; it moves when that dependent does. |
| `getrandom` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.4.3` | transitive |  | 0.4.3 (crates.io, getrandom) | version | current |  |
| `glob` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.3.4` | transitive |  | 0.3.4 (crates.io, glob) | version | current |  |
| `hashbrown` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.14.5` | transitive |  | 0.17.1 (crates.io, hashbrown) | version | behind | Held by the requirement of `dashmap` 6.2.1 (`^0.14.5`), which does not admit 0.17.1; it moves when that dependent does. |
| `hashbrown` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.15.5` | transitive |  | 0.17.1 (crates.io, hashbrown) | version | behind | Held by the requirement of `whatlang` 0.18.0 (`^0.15`), which does not admit 0.17.1; it moves when that dependent does. |
| `hashbrown` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.17.1` | transitive |  | 0.17.1 (crates.io, hashbrown) | version | current |  |
| `heck` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.5.0` | transitive |  | 0.5.0 (crates.io, heck) | version | current |  |
| `http` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.5.0` | transitive |  | 1.5.0 (crates.io, http) | version | current |  |
| `httparse` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.10.1` | transitive |  | 1.10.1 (crates.io, httparse) | version | current |  |
| `hybrid-array` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.4.15` | transitive |  | 0.4.15 (crates.io, hybrid-array) | version | current |  |
| `icu_collections` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.3.0` | transitive |  | 2.3.0 (crates.io, icu_collections) | version | current |  |
| `icu_locale_core` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.3.0` | transitive |  | 2.3.0 (crates.io, icu_locale_core) | version | current |  |
| `icu_normalizer_data` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.3.0` | transitive |  | 2.3.0 (crates.io, icu_normalizer_data) | version | current |  |
| `icu_normalizer` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.3.0` | transitive |  | 2.3.0 (crates.io, icu_normalizer) | version | current |  |
| `icu_properties_data` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.3.0` | transitive |  | 2.3.0 (crates.io, icu_properties_data) | version | current |  |
| `icu_properties` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.3.0` | transitive |  | 2.3.0 (crates.io, icu_properties) | version | current |  |
| `icu_provider` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.3.1` | transitive |  | 2.3.1 (crates.io, icu_provider) | version | current |  |
| `idna_adapter` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.2.2` | transitive |  | 1.2.2 (crates.io, idna_adapter) | version | current |  |
| `idna` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.1.0` | transitive |  | 1.1.0 (crates.io, idna) | version | current |  |
| `include_dir_macros` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.7.4` | transitive |  | 0.7.4 (crates.io, include_dir_macros) | version | current |  |
| `include_dir` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.7.4` | transitive |  | 0.7.4 (crates.io, include_dir) | version | current |  |
| `indexmap` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.14.2` | transitive |  | 2.14.2 (crates.io, indexmap) | version | current |  |
| `is_terminal_polyfill` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.70.2` | transitive |  | 1.70.2 (crates.io, is_terminal_polyfill) | version | current |  |
| `itertools` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.14.0` | transitive |  | 0.15.0 (crates.io, itertools) | version | behind | Held by the requirement of `lingua` 1.8.0 (`^0.14.0`), which does not admit 0.15.0; it moves when that dependent does. |
| `itoa` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.18` | transitive |  | 1.0.18 (crates.io, itoa) | version | current |  |
| `js-sys` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.3.106` | transitive |  | 0.3.106 (crates.io, js-sys) | version | current |  |
| `leak_slice` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.0` | transitive |  | 0.2.0 (crates.io, leak_slice) | version | current |  |
| `libc` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.190` | transitive |  | 0.2.190 (crates.io, libc) | version | current |  |
| `lindera-dictionary` | `rust/Cargo.lock` | `rust/Cargo.lock` | `6.2.0` | transitive |  | 6.2.0 (crates.io, lindera-dictionary) | version | current |  |
| `lindera-jieba` | `rust/Cargo.lock` | `rust/Cargo.lock` | `6.2.0` | transitive |  | 6.2.0 (crates.io, lindera-jieba) | version | current |  |
| `lindera` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `6.2.0` | direct, runtime (optional) | `6.2.0` | 6.2.0 (crates.io, lindera) | version | current |  |
| `lingua-arabic-language-model` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.3.0` | transitive |  | 1.3.0 (crates.io, lingua-arabic-language-model) | version | current |  |
| `lingua-bengali-language-model` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.3.0` | transitive |  | 1.3.0 (crates.io, lingua-bengali-language-model) | version | current |  |
| `lingua-chinese-language-model` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.3.0` | transitive |  | 1.3.0 (crates.io, lingua-chinese-language-model) | version | current |  |
| `lingua-english-language-model` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.3.0` | transitive |  | 1.3.0 (crates.io, lingua-english-language-model) | version | current |  |
| `lingua-french-language-model` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.3.0` | transitive |  | 1.3.0 (crates.io, lingua-french-language-model) | version | current |  |
| `lingua-hindi-language-model` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.3.0` | transitive |  | 1.3.0 (crates.io, lingua-hindi-language-model) | version | current |  |
| `lingua-portuguese-language-model` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.3.0` | transitive |  | 1.3.0 (crates.io, lingua-portuguese-language-model) | version | current |  |
| `lingua-russian-language-model` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.3.0` | transitive |  | 1.3.0 (crates.io, lingua-russian-language-model) | version | current |  |
| `lingua-spanish-language-model` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.3.0` | transitive |  | 1.3.0 (crates.io, lingua-spanish-language-model) | version | current |  |
| `lingua-urdu-language-model` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.3.0` | transitive |  | 1.3.0 (crates.io, lingua-urdu-language-model) | version | current |  |
| `lingua` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.8.0` | direct, runtime | `1.8.0` | 1.8.0 (crates.io, lingua) | version | current |  |
| `links-notation` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.22.0` | direct, development, runtime | `0.22.0` | 0.22.0 (crates.io, links-notation) | version | current |  |
| `linux-raw-sys` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.12.1` | transitive |  | 0.12.1 (crates.io, linux-raw-sys) | version | current |  |
| `litemap` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.3` | transitive |  | 0.8.3 (crates.io, litemap) | version | current |  |
| `lock_api` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.4.14` | transitive |  | 0.4.14 (crates.io, lock_api) | version | current |  |
| `log` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.4.34` | transitive |  | 0.4.34 (crates.io, log) | version | current |  |
| `maplit` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.2` | transitive |  | 1.0.2 (crates.io, maplit) | version | current |  |
| `md5` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.1` | transitive |  | 0.8.1 (crates.io, md5) | version | current |  |
| `memchr` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.8.3` | transitive |  | 2.8.3 (crates.io, memchr) | version | current |  |
| `memmap2` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.9.11` | transitive |  | 0.9.11 (crates.io, memmap2) | version | current |  |
| `miniz_oxide` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.9.1` | transitive |  | 0.9.1 (crates.io, miniz_oxide) | version | current |  |
| `multiversion_no_op` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.0` | transitive |  | 1.0.0 (crates.io, multiversion_no_op) | version | current |  |
| `munge_macro` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.4.7` | transitive |  | 0.4.7 (crates.io, munge_macro) | version | current |  |
| `munge` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.4.7` | transitive |  | 0.4.7 (crates.io, munge) | version | current |  |
| `nom` | `rust/Cargo.lock` | `rust/Cargo.lock` | `8.0.0` | transitive |  | 8.0.0 (crates.io, nom) | version | current |  |
| `num-traits` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.19` | transitive |  | 0.2.19 (crates.io, num-traits) | version | current |  |
| `once_cell_polyfill` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.70.2` | transitive |  | 1.70.2 (crates.io, once_cell_polyfill) | version | current |  |
| `once_cell` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.21.4` | transitive |  | 1.21.4 (crates.io, once_cell) | version | current |  |
| `parking_lot_core` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.9.12` | transitive |  | 0.9.12 (crates.io, parking_lot_core) | version | current |  |
| `percent-encoding` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.3.2` | transitive |  | 2.3.2 (crates.io, percent-encoding) | version | current |  |
| `pest_derive` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `2.9.2` | direct, development | `2.8.6` | 2.9.2 (crates.io, pest_derive) | version | current |  |
| `pest_generator` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.9.2` | transitive |  | 2.9.2 (crates.io, pest_generator) | version | current |  |
| `pest_meta` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `2.9.2` | direct, development | `2.8.6` | 2.9.2 (crates.io, pest_meta) | version | current |  |
| `pest` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `2.9.2` | direct, development | `2.8.6` | 2.9.2 (crates.io, pest) | version | current |  |
| `pin-project-lite` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.17` | transitive |  | 0.2.17 (crates.io, pin-project-lite) | version | current |  |
| `platform-data` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.0.0` | transitive |  | 2.0.0 (crates.io, platform-data) | version | current |  |
| `platform-mem` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.3.0` | direct, runtime (optional) | `0.3.0` | 0.3.0 (crates.io, platform-mem) | version | current |  |
| `platform-num` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.0` | transitive |  | 0.8.0 (crates.io, platform-num) | version | current |  |
| `platform-trees` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.3.4` | transitive |  | 0.3.4 (crates.io, platform-trees) | version | current |  |
| `potential_utf` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.1.6` | transitive |  | 0.1.6 (crates.io, potential_utf) | version | current |  |
| `proc-macro2` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.107` | transitive |  | 1.0.107 (crates.io, proc-macro2) | version | current |  |
| `ptr_meta_derive` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.3.2` | transitive |  | 0.3.2 (crates.io, ptr_meta_derive) | version | current |  |
| `ptr_meta` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.3.2` | transitive |  | 0.3.2 (crates.io, ptr_meta) | version | current |  |
| `quote` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.47` | transitive |  | 1.0.47 (crates.io, quote) | version | current |  |
| `r-efi` | `rust/Cargo.lock` | `rust/Cargo.lock` | `6.0.0` | transitive |  | 7.1.0 (crates.io, r-efi) | version | behind | Held by the requirement of `getrandom` 0.4.3 (`^6`), which does not admit 7.1.0; it moves when that dependent does. |
| `rancor` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.1.3` | transitive |  | 0.1.3 (crates.io, rancor) | version | current |  |
| `rand_core` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.10.1` | transitive |  | 0.10.1 (crates.io, rand_core) | version | current |  |
| `rand` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.10.3` | transitive |  | 0.10.3 (crates.io, rand) | version | current |  |
| `rayon-core` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.13.0` | transitive |  | 1.13.0 (crates.io, rayon-core) | version | current |  |
| `rayon` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.12.0` | transitive |  | 1.12.0 (crates.io, rayon) | version | current |  |
| `redox_syscall` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.5.18` | transitive |  | 0.9.4 (crates.io, redox_syscall) | version | behind | Held by the requirement of `parking_lot_core` 0.9.12 (`^0.5`), which does not admit 0.9.4; it moves when that dependent does. |
| `regex-automata` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.4.18` | transitive |  | 0.4.18 (crates.io, regex-automata) | version | current |  |
| `regex-syntax` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.11` | transitive |  | 0.8.11 (crates.io, regex-syntax) | version | current |  |
| `regex` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.13.1` | direct, development, runtime | `1` | 1.13.1 (crates.io, regex) | version | current |  |
| `rend` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.5.4` | transitive |  | 0.5.4 (crates.io, rend) | version | current |  |
| `ring` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.17.14` | transitive |  | 0.17.14 (crates.io, ring) | version | current |  |
| `rkyv_derive` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.18` | transitive |  | 0.8.18 (crates.io, rkyv_derive) | version | current |  |
| `rkyv` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.18` | transitive |  | 0.8.18 (crates.io, rkyv) | version | current |  |
| `rustix` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.1.5` | transitive |  | 1.1.5 (crates.io, rustix) | version | current |  |
| `rustls-pki-types` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.15.1` | transitive |  | 1.15.1 (crates.io, rustls-pki-types) | version | current |  |
| `rustls-webpki` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.103.15` | transitive |  | 0.103.15 (crates.io, rustls-webpki) | version | current |  |
| `rustls` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.23.45` | transitive |  | 0.23.45 (crates.io, rustls) | version | current |  |
| `rustversion` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.23` | transitive |  | 1.0.23 (crates.io, rustversion) | version | current |  |
| `ryu` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.23` | transitive |  | 1.0.23 (crates.io, ryu) | version | current |  |
| `same-file` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.6` | transitive |  | 1.0.6 (crates.io, same-file) | version | current |  |
| `scopeguard` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.2.0` | transitive |  | 1.2.0 (crates.io, scopeguard) | version | current |  |
| `serde_core` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.229` | transitive |  | 1.0.229 (crates.io, serde_core) | version | current |  |
| `serde_derive` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.229` | transitive |  | 1.0.229 (crates.io, serde_derive) | version | current |  |
| `serde_json` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.0.151` | direct, development, runtime | `1` | 1.0.151 (crates.io, serde_json) | version | current |  |
| `serde-wasm-bindgen` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.6.5` | transitive |  | 0.6.5 (crates.io, serde-wasm-bindgen) | version | current |  |
| `serde` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.0.229` | direct, runtime | `1` | 1.0.229 (crates.io, serde) | version | current |  |
| `sha2` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.11.0` | direct, runtime | `0.11` | 0.11.0 (crates.io, sha2) | version | current |  |
| `shlex` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.0.1` | transitive |  | 2.0.1 (crates.io, shlex) | version | current |  |
| `simd-adler32` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.3.10` | transitive |  | 0.3.10 (crates.io, simd-adler32) | version | current |  |
| `simdutf8` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.1.5` | transitive |  | 0.1.5 (crates.io, simdutf8) | version | current |  |
| `slab` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.4.12` | transitive |  | 0.4.12 (crates.io, slab) | version | current |  |
| `smallvec` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.16.2` | transitive |  | 1.16.2 (crates.io, smallvec) | version | current |  |
| `stable_deref_trait` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.2.1` | transitive |  | 1.2.1 (crates.io, stable_deref_trait) | version | current |  |
| `streaming-iterator` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.1.9` | transitive |  | 0.1.9 (crates.io, streaming-iterator) | version | current |  |
| `strsim` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.11.1` | transitive |  | 0.11.1 (crates.io, strsim) | version | current |  |
| `strum_macros` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.27.2` | transitive |  | 0.28.0 (crates.io, strum_macros) | version | behind | Held by the requirement of `lingua` 1.8.0 (`^0.27.2`), which does not admit 0.28.0; it moves when that dependent does. |
| `strum_macros` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.28.0` | transitive |  | 0.28.0 (crates.io, strum_macros) | version | current |  |
| `strum` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.27.2` | transitive |  | 0.28.0 (crates.io, strum) | version | behind | Held by the requirement of `lingua` 1.8.0 (`^0.27.2`), which does not admit 0.28.0; it moves when that dependent does. |
| `strum` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.28.0` | transitive |  | 0.28.0 (crates.io, strum) | version | current |  |
| `subtle` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.6.1` | transitive |  | 2.6.1 (crates.io, subtle) | version | current |  |
| `syn` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.0.119` | transitive |  | 3.0.6 (crates.io, syn) | version | behind | Held by the requirement of `munge_macro` 0.4.7 (`^2`), `pest_generator` 2.9.2 (`^2.0`), `strum_macros` 0.27.2 (`^2.0`), `strum_macros` 0.28.0 (`^2.0`), which does not admit 3.0.6; it moves when that dependent does. |
| `syn` | `rust/Cargo.lock` | `rust/Cargo.lock` | `3.0.6` | transitive |  | 3.0.6 (crates.io, syn) | version | current |  |
| `synstructure` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.14.0` | transitive |  | 0.14.0 (crates.io, synstructure) | version | current |  |
| `tap` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.1` | transitive |  | 1.0.1 (crates.io, tap) | version | current |  |
| `tar` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.4.46` | transitive |  | 0.4.46 (crates.io, tar) | version | current |  |
| `tempfile` | `rust/Cargo.lock` | `rust/Cargo.lock` | `3.27.0` | transitive |  | 3.27.0 (crates.io, tempfile) | version | current |  |
| `thiserror-impl` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.0.21` | transitive |  | 2.0.21 (crates.io, thiserror-impl) | version | current |  |
| `thiserror` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.0.21` | transitive |  | 2.0.21 (crates.io, thiserror) | version | current |  |
| `tinystr` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.4` | transitive |  | 0.8.4 (crates.io, tinystr) | version | current |  |
| `tinyvec` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.13.3` | transitive |  | 1.13.3 (crates.io, tinyvec) | version | current |  |
| `tree-sitter-agda` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.3.3` | direct, runtime | `=1.3.3` | 1.3.3 (crates.io, tree-sitter-agda) | version | current |  |
| `tree-sitter-bash` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.25.1` | direct, runtime | `=0.25.1` | 0.25.1 (crates.io, tree-sitter-bash) | version | current |  |
| `tree-sitter-c-sharp` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.23.5` | direct, runtime | `=0.23.5` | 0.23.5 (crates.io, tree-sitter-c-sharp) | version | current |  |
| `tree-sitter-c` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.24.2` | direct, development | `=0.24.2` | 0.24.2 (crates.io, tree-sitter-c) | version | current |  |
| `tree-sitter-cpp` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.23.4` | direct, development | `=0.23.4` | 0.23.4 (crates.io, tree-sitter-cpp) | version | current |  |
| `tree-sitter-css` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.25.0` | direct, development | `=0.25.0` | 0.25.0 (crates.io, tree-sitter-css) | version | current |  |
| `tree-sitter-dart` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.2.0` | direct, runtime | `=0.2.0` | 0.2.0 (crates.io, tree-sitter-dart) | version | current |  |
| `tree-sitter-diff` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.1.0` | direct, development | `=0.1.0` | 0.1.0 (crates.io, tree-sitter-diff) | version | current |  |
| `tree-sitter-elixir` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.3.5` | direct, runtime | `=0.3.5` | 0.3.5 (crates.io, tree-sitter-elixir) | version | current |  |
| `tree-sitter-elm` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `5.9.4` | direct, runtime | `=5.9.4` | 5.9.4 (crates.io, tree-sitter-elm) | version | current |  |
| `tree-sitter-erlang` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.20.0` | direct, development | `=0.20.0` | 0.20.0 (crates.io, tree-sitter-erlang) | version | current |  |
| `tree-sitter-go` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.25.0` | direct, development | `=0.25.0` | 0.25.0 (crates.io, tree-sitter-go) | version | current |  |
| `tree-sitter-graphql` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.3.0` | direct, development | `=0.3.0` | 0.3.0 (crates.io, tree-sitter-graphql) | version | current |  |
| `tree-sitter-groovy` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.1.2` | direct, development | `=0.1.2` | 0.1.2 (crates.io, tree-sitter-groovy) | version | current |  |
| `tree-sitter-haskell` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.24.1` | direct, runtime | `=0.24.1` | 0.24.1 (crates.io, tree-sitter-haskell) | version | current |  |
| `tree-sitter-hcl` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.1.0` | direct, runtime | `=1.1.0` | 1.1.0 (crates.io, tree-sitter-hcl) | version | current |  |
| `tree-sitter-html` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.23.2` | direct, runtime | `=0.23.2` | 0.23.2 (crates.io, tree-sitter-html) | version | current |  |
| `tree-sitter-ini` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.4.0` | direct, development | `=1.4.0` | 1.4.0 (crates.io, tree-sitter-ini) | version | current |  |
| `tree-sitter-java` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.23.5` | direct, development | `=0.23.5` | 0.23.5 (crates.io, tree-sitter-java) | version | current |  |
| `tree-sitter-javascript` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.25.0` | direct, development | `=0.25.0` | 0.25.0 (crates.io, tree-sitter-javascript) | version | current |  |
| `tree-sitter-json` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.24.8` | direct, development | `=0.24.8` | 0.24.8 (crates.io, tree-sitter-json) | version | current |  |
| `tree-sitter-json5-orchard` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.1.0` | direct, development | `=0.1.0` | 0.1.0 (crates.io, tree-sitter-json5-orchard) | version | current |  |
| `tree-sitter-kotlin-ng` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.1.0` | direct, runtime | `=1.1.0` | 1.1.0 (crates.io, tree-sitter-kotlin-ng) | version | current |  |
| `tree-sitter-language` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.1.8` | direct, runtime | `0.1.8` | 0.1.8 (crates.io, tree-sitter-language) | version | current |  |
| `tree-sitter-lua` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.5.0` | direct, development | `=0.5.0` | 0.5.0 (crates.io, tree-sitter-lua) | version | current |  |
| `tree-sitter-make` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.1.1` | direct, development | `=1.1.1` | 1.1.1 (crates.io, tree-sitter-make) | version | current |  |
| `tree-sitter-matlab` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.3.1` | direct, runtime | `=1.3.1` | 1.3.1 (crates.io, tree-sitter-matlab) | version | current |  |
| `tree-sitter-md-025` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.5.6` | direct, runtime | `=0.5.6` | 0.5.6 (crates.io, tree-sitter-md-025) | version | current |  |
| `tree-sitter-nix` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.3.0` | direct, runtime | `=0.3.0` | 0.3.0 (crates.io, tree-sitter-nix) | version | current |  |
| `tree-sitter-ocaml` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.26.0` | direct, runtime | `=0.26.0` | 0.26.0 (crates.io, tree-sitter-ocaml) | version | current |  |
| `tree-sitter-odin` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.3.0` | direct, runtime | `=1.3.0` | 1.3.0 (crates.io, tree-sitter-odin) | version | current |  |
| `tree-sitter-pascal` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.10.2` | direct, development | `=0.10.2` | 0.10.2 (crates.io, tree-sitter-pascal) | version | current |  |
| `tree-sitter-php` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.24.2` | direct, runtime | `=0.24.2` | 0.24.2 (crates.io, tree-sitter-php) | version | current |  |
| `tree-sitter-powershell` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.26.4` | direct, development | `=0.26.4` | 0.26.4 (crates.io, tree-sitter-powershell) | version | current |  |
| `tree-sitter-proto` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.6.0` | direct, development | `=0.6.0` | 0.6.0 (crates.io, tree-sitter-proto) | version | current |  |
| `tree-sitter-python` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.25.0` | direct, runtime | `=0.25.0` | 0.25.0 (crates.io, tree-sitter-python) | version | current |  |
| `tree-sitter-r` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.3.0` | direct, runtime | `=1.3.0` | 1.3.0 (crates.io, tree-sitter-r) | version | current |  |
| `tree-sitter-racket` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.25.0` | direct, development | `=0.25.0` | 0.25.0 (crates.io, tree-sitter-racket) | version | current |  |
| `tree-sitter-regex` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.25.0` | direct, development | `=0.25.0` | 0.25.0 (crates.io, tree-sitter-regex) | version | current |  |
| `tree-sitter-ruby` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.23.1` | direct, runtime | `=0.23.1` | 0.23.1 (crates.io, tree-sitter-ruby) | version | current |  |
| `tree-sitter-scala` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.26.2` | direct, runtime | `=0.26.2` | 0.26.2 (crates.io, tree-sitter-scala) | version | current |  |
| `tree-sitter-scheme` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.24.7` | direct, development | `=0.24.7` | 0.24.7 (crates.io, tree-sitter-scheme) | version | current |  |
| `tree-sitter-sequel` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.3.11` | direct, development | `=0.3.11` | 0.3.11 (crates.io, tree-sitter-sequel) | version | current |  |
| `tree-sitter-solidity` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.2.13` | direct, development | `=1.2.13` | 1.2.13 (crates.io, tree-sitter-solidity) | version | current |  |
| `tree-sitter-swift` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.7.4` | direct, runtime | `=0.7.4` | 0.7.4 (crates.io, tree-sitter-swift) | version | current |  |
| `tree-sitter-toml-ng` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.7.0` | direct, development | `=0.7.0` | 0.7.0 (crates.io, tree-sitter-toml-ng) | version | current |  |
| `tree-sitter-typescript` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.23.2` | direct, development | `=0.23.2` | 0.23.2 (crates.io, tree-sitter-typescript) | version | current |  |
| `tree-sitter-vb-dotnet` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.1.0` | direct, development | `=0.1.0` | 0.1.0 (crates.io, tree-sitter-vb-dotnet) | version | current |  |
| `tree-sitter-xml` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.7.0` | direct, runtime | `=0.7.0` | 0.7.0 (crates.io, tree-sitter-xml) | version | current |  |
| `tree-sitter-yaml` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.7.2` | direct, runtime | `=0.7.2` | 0.7.2 (crates.io, tree-sitter-yaml) | version | current |  |
| `tree-sitter-zig` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.1.2` | direct, development | `=1.1.2` | 1.1.2 (crates.io, tree-sitter-zig) | version | current |  |
| `tree-sitter` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.27.0` | direct, runtime | `=0.27.0` | 0.27.0 (crates.io, tree-sitter) | version | current |  |
| `ts-parser-perl` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `2.0.0` | direct, runtime | `=2.0.0` | 2.0.0 (crates.io, ts-parser-perl) | version | current |  |
| `typenum` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.20.1` | transitive |  | 1.20.1 (crates.io, typenum) | version | current |  |
| `ucd-trie` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.1.7` | transitive |  | 0.1.7 (crates.io, ucd-trie) | version | current |  |
| `unicode-bidi` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.3.18` | direct, runtime | `0.3.18` | 0.3.18 (crates.io, unicode-bidi) | version | current |  |
| `unicode-ident` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.26` | transitive |  | 1.0.26 (crates.io, unicode-ident) | version | current |  |
| `unicode-normalization` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.1.25` | direct, runtime | `0.1.25` | 0.1.25 (crates.io, unicode-normalization) | version | current |  |
| `unicode-segmentation` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `1.13.3` | direct, runtime | `1.13.3` | 1.13.3 (crates.io, unicode-segmentation) | version | current |  |
| `untrusted` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.9.0` | transitive |  | 0.9.0 (crates.io, untrusted) | version | current |  |
| `ureq-proto` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.6.4` | transitive |  | 0.6.4 (crates.io, ureq-proto) | version | current |  |
| `ureq` | `rust/Cargo.lock` | `rust/Cargo.lock` | `3.4.2` | transitive |  | 3.4.2 (crates.io, ureq) | version | current |  |
| `url` | `rust/Cargo.lock` | `rust/Cargo.lock` | `2.5.8` | transitive |  | 2.5.8 (crates.io, url) | version | current |  |
| `utf8_iter` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.4` | transitive |  | 1.0.4 (crates.io, utf8_iter) | version | current |  |
| `utf8-zero` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.1` | transitive |  | 0.8.1 (crates.io, utf8-zero) | version | current |  |
| `utf8parse` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.2` | transitive |  | 0.2.2 (crates.io, utf8parse) | version | current |  |
| `uuid` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.27.0` | transitive |  | 1.27.0 (crates.io, uuid) | version | current |  |
| `walkdir` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `2.5.0` | direct, development | `2` | 2.5.0 (crates.io, walkdir) | version | current |  |
| `wasi` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.11.1+wasi-snapshot-preview1` | transitive |  | 0.14.7+wasi-0.2.4 (crates.io, wasi) | version | behind | Held by the requirement of `getrandom` 0.2.17 (`^0.11`), which does not admit 0.14.7+wasi-0.2.4; it moves when that dependent does. |
| `wasm-bindgen-macro-support` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.129` | transitive |  | 0.2.129 (crates.io, wasm-bindgen-macro-support) | version | current |  |
| `wasm-bindgen-macro` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.129` | transitive |  | 0.2.129 (crates.io, wasm-bindgen-macro) | version | current |  |
| `wasm-bindgen-shared` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.129` | transitive |  | 0.2.129 (crates.io, wasm-bindgen-shared) | version | current |  |
| `wasm-bindgen` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.129` | transitive |  | 0.2.129 (crates.io, wasm-bindgen) | version | current |  |
| `webpki-roots` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.9` | transitive |  | 1.0.9 (crates.io, webpki-roots) | version | current |  |
| `whatlang` | `rust/Cargo.lock` | `rust/Cargo.lock`, `rust/Cargo.toml` | `0.18.0` | direct, runtime | `0.18.0` | 0.18.0 (crates.io, whatlang) | version | current |  |
| `winapi-util` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.1.11` | transitive |  | 0.1.11 (crates.io, winapi-util) | version | current |  |
| `windows_aarch64_gnullvm` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.52.6` | transitive |  | 0.53.1 (crates.io, windows_aarch64_gnullvm) | version | behind | Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. |
| `windows_aarch64_msvc` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.52.6` | transitive |  | 0.53.1 (crates.io, windows_aarch64_msvc) | version | behind | Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. |
| `windows_i686_gnu` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.52.6` | transitive |  | 0.53.1 (crates.io, windows_i686_gnu) | version | behind | Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. |
| `windows_i686_gnullvm` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.52.6` | transitive |  | 0.53.1 (crates.io, windows_i686_gnullvm) | version | behind | Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. |
| `windows_i686_msvc` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.52.6` | transitive |  | 0.53.1 (crates.io, windows_i686_msvc) | version | behind | Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. |
| `windows_x86_64_gnu` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.52.6` | transitive |  | 0.53.1 (crates.io, windows_x86_64_gnu) | version | behind | Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. |
| `windows_x86_64_gnullvm` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.52.6` | transitive |  | 0.53.1 (crates.io, windows_x86_64_gnullvm) | version | behind | Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. |
| `windows_x86_64_msvc` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.52.6` | transitive |  | 0.53.1 (crates.io, windows_x86_64_msvc) | version | behind | Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. |
| `windows-link` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.1` | transitive |  | 0.100.0 (crates.io, windows-link) | version | behind | Held by the requirement of `parking_lot_core` 0.9.12 (`^0.2.0`), `windows-sys` 0.61.2 (`^0.2.1`), which does not admit 0.100.0; it moves when that dependent does. |
| `windows-sys` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.52.0` | transitive |  | 0.61.2 (crates.io, windows-sys) | version | behind | Held by the requirement of `ring` 0.17.14 (`^0.52`), which does not admit 0.61.2; it moves when that dependent does. |
| `windows-sys` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.61.2` | transitive |  | 0.61.2 (crates.io, windows-sys) | version | current |  |
| `windows-targets` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.52.6` | transitive |  | 0.53.5 (crates.io, windows-targets) | version | behind | Held by the requirement of `windows-sys` 0.52.0 (`^0.52.0`), which does not admit 0.53.5; it moves when that dependent does. |
| `writeable` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.6.4` | transitive |  | 0.6.4 (crates.io, writeable) | version | current |  |
| `xattr` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.6.1` | transitive |  | 1.6.1 (crates.io, xattr) | version | current |  |
| `yoke-derive` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.4` | transitive |  | 0.8.4 (crates.io, yoke-derive) | version | current |  |
| `yoke` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.8.3` | transitive |  | 0.8.3 (crates.io, yoke) | version | current |  |
| `zerofrom-derive` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.1.8` | transitive |  | 0.1.8 (crates.io, zerofrom-derive) | version | current |  |
| `zerofrom` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.1.8` | transitive |  | 0.1.8 (crates.io, zerofrom) | version | current |  |
| `zeroize` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.9.0` | transitive |  | 1.9.0 (crates.io, zeroize) | version | current |  |
| `zerotrie` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.2.5` | transitive |  | 0.2.5 (crates.io, zerotrie) | version | current |  |
| `zerovec-derive` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.11.6` | transitive |  | 0.11.6 (crates.io, zerovec-derive) | version | current |  |
| `zerovec` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.11.8` | transitive |  | 0.11.8 (crates.io, zerovec) | version | current |  |
| `zlib-rs` | `rust/Cargo.lock` | `rust/Cargo.lock` | `0.6.8` | transitive |  | 0.6.8 (crates.io, zlib-rs) | version | current |  |
| `zmij` | `rust/Cargo.lock` | `rust/Cargo.lock` | `1.0.23` | transitive |  | 1.0.23 (crates.io, zmij) | version | current |  |
| `bumpalo` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `3.20.3` | transitive |  | 3.20.3 (crates.io, bumpalo) | version | current |  |
| `cfg-if` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `1.0.5` | transitive |  | 1.0.5 (crates.io, cfg-if) | version | current |  |
| `itoa` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `1.0.18` | transitive |  | 1.0.18 (crates.io, itoa) | version | current |  |
| `links-notation` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock`, `rust/web/Cargo.toml` | `0.22.0` | direct, runtime | `0.22.0` | 0.22.0 (crates.io, links-notation) | version | current |  |
| `memchr` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `2.8.3` | transitive |  | 2.8.3 (crates.io, memchr) | version | current |  |
| `nom` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `8.0.0` | transitive |  | 8.0.0 (crates.io, nom) | version | current |  |
| `once_cell` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `1.21.4` | transitive |  | 1.21.4 (crates.io, once_cell) | version | current |  |
| `proc-macro2` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `1.0.107` | transitive |  | 1.0.107 (crates.io, proc-macro2) | version | current |  |
| `quote` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `1.0.47` | transitive |  | 1.0.47 (crates.io, quote) | version | current |  |
| `rustversion` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `1.0.23` | transitive |  | 1.0.23 (crates.io, rustversion) | version | current |  |
| `serde_core` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `1.0.229` | transitive |  | 1.0.229 (crates.io, serde_core) | version | current |  |
| `serde_derive` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `1.0.229` | transitive |  | 1.0.229 (crates.io, serde_derive) | version | current |  |
| `serde_json` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock`, `rust/web/Cargo.toml` | `1.0.151` | direct, runtime | `1` | 1.0.151 (crates.io, serde_json) | version | current |  |
| `serde` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `1.0.229` | transitive |  | 1.0.229 (crates.io, serde) | version | current |  |
| `syn` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `3.0.6` | transitive |  | 3.0.6 (crates.io, syn) | version | current |  |
| `unicode-ident` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `1.0.26` | transitive |  | 1.0.26 (crates.io, unicode-ident) | version | current |  |
| `wasm-bindgen-macro-support` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `0.2.129` | transitive |  | 0.2.129 (crates.io, wasm-bindgen-macro-support) | version | current |  |
| `wasm-bindgen-macro` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `0.2.129` | transitive |  | 0.2.129 (crates.io, wasm-bindgen-macro) | version | current |  |
| `wasm-bindgen-shared` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `0.2.129` | transitive |  | 0.2.129 (crates.io, wasm-bindgen-shared) | version | current |  |
| `wasm-bindgen` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock`, `rust/web/Cargo.toml` | `0.2.129` | direct, runtime | `0.2` | 0.2.129 (crates.io, wasm-bindgen) | version | current |  |
| `zmij` | `rust/web/Cargo.lock` | `rust/web/Cargo.lock` | `1.0.23` | transitive |  | 1.0.23 (crates.io, zmij) | version | current |  |

## Experiment manifests

| Item | Scope | Declared in | Pinned | Role | Requirement | Current stable release | Comparison | Status | Reason |
|---|---|---|---|---|---|---|---|---|---|
| `serde_json` | `experiments/grammar-merge-rust-stub/Cargo.toml` | `experiments/grammar-merge-rust-stub/Cargo.toml` | `=1.0.151` | runtime |  | 1.0.151 (crates.io, serde_json) | version | current |  |
| `serde` | `experiments/grammar-merge-rust-stub/Cargo.toml` | `experiments/grammar-merge-rust-stub/Cargo.toml` | `=1.0.229` | runtime |  | 1.0.229 (crates.io, serde) | version | current |  |
| `sha2` | `experiments/grammar-merge-rust-stub/Cargo.toml` | `experiments/grammar-merge-rust-stub/Cargo.toml` | `=0.11.0` | runtime |  | 0.11.0 (crates.io, sha2) | version | current |  |
| `cc` | `experiments/issue-195-cmake-scanner/Cargo.toml` | `experiments/issue-195-cmake-scanner/Cargo.toml` | `=1.6.0` | build |  | 1.6.0 (crates.io, cc) | version | current |  |
| `tree-sitter-language` | `experiments/issue-195-cmake-scanner/Cargo.toml` | `experiments/issue-195-cmake-scanner/Cargo.toml` | `=0.1.8` | runtime |  | 0.1.8 (crates.io, tree-sitter-language) | version | current |  |
| `tree-sitter` | `experiments/issue-195-cmake-scanner/Cargo.toml` | `experiments/issue-195-cmake-scanner/Cargo.toml` | `=0.27.0` | runtime |  | 0.27.0 (crates.io, tree-sitter) | version | current |  |

## Vendored generated parsers

| Item | Declared in | Pinned | Current stable release | Comparison | Status | Reason |
|---|---|---|---|---|---|---|
| `rust/vendor/tree-sitter-cmake` | `js/src/vendor/grammars/grammar-lock.json`, `rust/vendor/tree-sitter-cmake/NOTICE.md` | `v0.7.5` | v0.7.5 (GitHub release, uyha/tree-sitter-cmake (the pin is identical of it)) | revision | current |  |
| `rust/vendor/tree-sitter-csv` | `js/src/vendor/grammars/grammar-lock.json`, `rust/vendor/tree-sitter-csv/NOTICE.md` | `f6bf6e35eb0b95fbadea4bb39cb9709507fcb181` | v1.2.0 (GitHub release, tree-sitter-grammars/tree-sitter-csv (the pin is ahead of it)) | revision | current |  |
| `rust/vendor/tree-sitter-lean` | `js/src/vendor/grammars/grammar-lock.json`, `rust/vendor/tree-sitter-lean/NOTICE.md` | `bd942cd2795016239be02b3b3d5ef635645ddd38` | v0.2.2 (GitHub release, wvhulle/tree-sitter-lean (the pin is ahead of it)) | revision | current |  |
| `rust/vendor/tree-sitter-rocq` | `js/src/vendor/grammars/grammar-lock.json`, `rust/vendor/tree-sitter-rocq/NOTICE.md` | `300fe33fc299c30f736fd56d8ef8a28b08acd4e6` | v0.2.0 (GitHub release, aruzdh/tree-sitter-rocq (the pin is ahead of it)) | revision | current |  |
| `rust/vendor/tree-sitter-rust` | `js/src/vendor/grammars/grammar-lock.json`, `rust/vendor/tree-sitter-rust/NOTICE.md` | `v0.24.2` | v0.24.2 (GitHub release, tree-sitter/tree-sitter-rust (the pin is identical of it)) | revision | current |  |

## Vendored runtime

| Item | Declared in | Pinned | Current stable release | Comparison | Status | Reason |
|---|---|---|---|---|---|---|
| `js/src/vendor/web-tree-sitter/web-tree-sitter.wasm.gz` | `js/src/vendor/web-tree-sitter/runtime-lock.json` | `0.27.0` | v0.27.0 (GitHub release, tree-sitter/tree-sitter) | version | behind | Built from `image emscripten/emsdk@4.0.15`, which is behind (see its reason). |

## Vendored WebAssembly grammars

| Item | Declared in | Pinned | Current stable release | Comparison | Status | Reason |
|---|---|---|---|---|---|---|
| `js/oracles/grammars/c.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.24.2` |  | derived | current |  |
| `js/oracles/grammars/cpp.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.23.4` |  | derived | current |  |
| `js/oracles/grammars/css.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.25.0` |  | derived | current |  |
| `js/oracles/grammars/csv.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `f6bf6e35eb0b95fbadea4bb39cb9709507fcb181` |  | derived | current |  |
| `js/oracles/grammars/diff.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.1.0` |  | derived | current |  |
| `js/oracles/grammars/erlang.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.20.0` |  | derived | current |  |
| `js/oracles/grammars/go.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.25.0` |  | derived | current |  |
| `js/oracles/grammars/graphql.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.3.0` |  | derived | current |  |
| `js/oracles/grammars/groovy.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.1.2` |  | derived | current |  |
| `js/oracles/grammars/ini.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `1.4.0` |  | derived | current |  |
| `js/oracles/grammars/java.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.23.5` |  | derived | current |  |
| `js/oracles/grammars/javascript.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.25.0` |  | derived | current |  |
| `js/oracles/grammars/json.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.24.8` |  | derived | current |  |
| `js/oracles/grammars/json5.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.1.0` |  | derived | current |  |
| `js/oracles/grammars/lua.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.5.0` |  | derived | current |  |
| `js/oracles/grammars/make.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `1.1.1` |  | derived | current |  |
| `js/oracles/grammars/pascal.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.10.2` |  | derived | current |  |
| `js/oracles/grammars/powershell.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.26.4` |  | derived | current |  |
| `js/oracles/grammars/proto.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.6.0` |  | derived | current |  |
| `js/oracles/grammars/racket.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.25.0` |  | derived | current |  |
| `js/oracles/grammars/regex.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.25.0` |  | derived | current |  |
| `js/oracles/grammars/rust.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `v0.24.2` |  | derived | current |  |
| `js/oracles/grammars/scheme.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.24.7` |  | derived | current |  |
| `js/oracles/grammars/solidity.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `1.2.13` |  | derived | current |  |
| `js/oracles/grammars/sql.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.3.11` |  | derived | current |  |
| `js/oracles/grammars/toml.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.7.0` |  | derived | current |  |
| `js/oracles/grammars/tsx.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.23.2` |  | derived | current |  |
| `js/oracles/grammars/typescript.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.23.2` |  | derived | current |  |
| `js/oracles/grammars/vb.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.1.0` |  | derived | current |  |
| `js/oracles/grammars/zig.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `1.1.2` |  | derived | current |  |
| `js/src/vendor/grammars/agda.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `1.3.3` |  | derived | current |  |
| `js/src/vendor/grammars/bash.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.25.1` |  | derived | current |  |
| `js/src/vendor/grammars/cmake.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `v0.7.5` |  | derived | current |  |
| `js/src/vendor/grammars/csharp.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.23.5` |  | derived | current |  |
| `js/src/vendor/grammars/dart.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.2.0` |  | derived | current |  |
| `js/src/vendor/grammars/dtd.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.7.0` |  | derived | current |  |
| `js/src/vendor/grammars/elixir.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.3.5` |  | derived | current |  |
| `js/src/vendor/grammars/elm.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `5.9.4` |  | derived | current |  |
| `js/src/vendor/grammars/haskell.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.24.1` |  | derived | current |  |
| `js/src/vendor/grammars/hcl.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `1.1.0` |  | derived | current |  |
| `js/src/vendor/grammars/html.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.23.2` |  | derived | current |  |
| `js/src/vendor/grammars/kotlin.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `1.1.0` |  | derived | current |  |
| `js/src/vendor/grammars/lean.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `bd942cd2795016239be02b3b3d5ef635645ddd38` |  | derived | current |  |
| `js/src/vendor/grammars/markdown_inline.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.5.6` |  | derived | current |  |
| `js/src/vendor/grammars/markdown.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.5.6` |  | derived | current |  |
| `js/src/vendor/grammars/matlab.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `1.3.1` |  | derived | current |  |
| `js/src/vendor/grammars/nix.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.3.0` |  | derived | current |  |
| `js/src/vendor/grammars/ocaml_interface.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.26.0` |  | derived | current |  |
| `js/src/vendor/grammars/ocaml.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.26.0` |  | derived | current |  |
| `js/src/vendor/grammars/odin.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `1.3.0` |  | derived | current |  |
| `js/src/vendor/grammars/perl.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `2.0.0` |  | derived | current |  |
| `js/src/vendor/grammars/php.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.24.2` |  | derived | current |  |
| `js/src/vendor/grammars/python.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.25.0` |  | derived | current |  |
| `js/src/vendor/grammars/r.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `1.3.0` |  | derived | current |  |
| `js/src/vendor/grammars/rocq.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `300fe33fc299c30f736fd56d8ef8a28b08acd4e6` |  | derived | current |  |
| `js/src/vendor/grammars/ruby.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.23.1` |  | derived | current |  |
| `js/src/vendor/grammars/scala.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.26.2` |  | derived | current |  |
| `js/src/vendor/grammars/swift.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.7.4` |  | derived | current |  |
| `js/src/vendor/grammars/xml.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.7.0` |  | derived | current |  |
| `js/src/vendor/grammars/yaml.wasm.gz` | `js/src/vendor/grammars/grammar-lock.json` | `0.7.2` |  | derived | current |  |

## Generators

| Item | Declared in | Pinned | Current stable release | Comparison | Status | Reason |
|---|---|---|---|---|---|---|
| `js/scripts/build-bidi-table.mjs` | `js/scripts/build-bidi-table.mjs` | `script` |  | derived | current |  |
| `js/scripts/build-concept-records.mjs` | `js/scripts/build-concept-records.mjs` | `script` |  | derived | current |  |
| `js/scripts/build-foundation-models.mjs` | `js/scripts/build-foundation-models.mjs` | `script` |  | derived | current |  |
| `js/scripts/build-language-catalog.mjs` | `js/scripts/build-language-catalog.mjs` | `script` |  | derived | current |  |
| `js/scripts/build-language-identification.mjs` | `js/scripts/build-language-identification.mjs` | `script` |  | derived | current |  |
| `js/scripts/build-lean-root-names.mjs` | `js/scripts/build-lean-root-names.mjs` | `script` |  | derived | current |  |
| `js/scripts/build-merge-quality-evidence.mjs` | `js/scripts/build-merge-quality-evidence.mjs` | `script` |  | derived | current |  |
| `js/scripts/build-native-grammar-concept-reuse.mjs` | `js/scripts/build-native-grammar-concept-reuse.mjs` | `script` |  | derived | current |  |
| `js/scripts/build-translation-stage-fixtures.mjs` | `js/scripts/build-translation-stage-fixtures.mjs` | `script` |  | derived | current |  |
| `js/scripts/build-vendored-grammars.mjs` | `js/scripts/build-vendored-grammars.mjs` | `script` |  | derived | current |  |
| `js/scripts/build-web-tree-sitter-runtime.mjs` | `js/scripts/build-web-tree-sitter-runtime.mjs` | `script` |  | derived | behind | Built from `image emscripten/emsdk@4.0.15`, which is behind (see its reason). |
| `js/scripts/generate-builtin-cst-expectations.mjs` | `js/scripts/generate-builtin-cst-expectations.mjs` | `script` |  | derived | current |  |
| `js/scripts/generate-default-cst-expectations.mjs` | `js/scripts/generate-default-cst-expectations.mjs` | `script` |  | derived | current |  |
| `js/scripts/generate-export-parity.mjs` | `js/scripts/generate-export-parity.mjs` | `script` |  | derived | current |  |
| `js/scripts/generate-issue-195-conformance.mjs` | `js/scripts/generate-issue-195-conformance.mjs` | `script` |  | derived | current |  |
| `js/scripts/generate-issue-195-generative.mjs` | `js/scripts/generate-issue-195-generative.mjs` | `script` |  | derived | current |  |
| `js/scripts/generate-lino-grammar-cases.mjs` | `js/scripts/generate-lino-grammar-cases.mjs` | `script` |  | derived | current |  |
| `js/scripts/generate-native-grammar-fixtures.mjs` | `js/scripts/generate-native-grammar-fixtures.mjs` | `script` |  | derived | current |  |
| `js/scripts/generate-native-recovery.mjs` | `js/scripts/generate-native-recovery.mjs` | `script` |  | derived | current |  |
| `js/scripts/generate-pdf-grammar-cases.mjs` | `js/scripts/generate-pdf-grammar-cases.mjs` | `script` |  | derived | current |  |
| `js/scripts/generate-self-translation-cases.mjs` | `js/scripts/generate-self-translation-cases.mjs` | `script` |  | derived | current |  |
| `js/scripts/generate-self-translation-report.mjs` | `js/scripts/generate-self-translation-report.mjs` | `script` |  | derived | current |  |
| `rust/scripts/build-site.rs` | `rust/scripts/build-site.rs` | `script` |  | derived | current |  |

## Toolchains and tools

| Item | Declared in | Pinned | Current stable release | Comparison | Status | Reason |
|---|---|---|---|---|---|---|
| `@secretlint/secretlint-rule-preset-recommend` | `.github/workflows/js.yml`, `.github/workflows/rust.yml` | `latest` |  | floating | current |  |
| `cargo-llvm-cov` | `.github/workflows/rust.yml` | `latest` |  | floating | current |  |
| `elan` | `.github/workflows/ci.yml` | `v4.2.4` | v4.2.4 (GitHub release, leanprover/elan) | version | current |  |
| `lean` | `.github/workflows/ci.yml`, `experiments/issue-195-projects/lean/lean-toolchain`, `lean-toolchain` | `v4.34.1` | v4.34.1 (GitHub release, leanprover/lean4) | version | current |  |
| `node` | `.github/workflows/ci.yml`, `.github/workflows/js.yml` | `22` | 22 (nodejs/Release schedule, oldest maintained LTS line (maintained: 22, 24)) | floor | current |  |
| `node` | `.github/workflows/ci.yml`, `.github/workflows/dependency-refresh.yml`, `.github/workflows/js.yml`, `.github/workflows/rust.yml` | `24` | 24.21.0 (nodejs.org, newest LTS release (Krypton)) | major | current |  |
| `npm` | `.github/workflows/js.yml` | `12` | 12.2.0 (npm registry, npm latest) | major | current |  |
| `ocaml` | `.github/workflows/ci.yml` | `5.4` | 5.5.1 (GitHub release, ocaml/ocaml) | minor | behind | Held by ocamlfind 1.9.8, the newest ocamlfind release in opam-repository, which requires OCaml <5.5.0; rocq-runtime 9.3.0 needs ocamlfind (>=1.9.1), so the Rocq acceptance job builds with the newest OCaml it admits and moves when ocamlfind admits a newer line. |
| `pre-commit/pre-commit-hooks` | `.pre-commit-config.yaml` | `v6.0.0` | v6.0.0 (GitHub release, pre-commit/pre-commit-hooks) | version | current |  |
| `rocq-core` | `.github/workflows/ci.yml` | `9.3.0` | V9.3.0 (GitHub release, rocq-prover/rocq) | version | current |  |
| `rocq-prover` | `.github/workflows/ci.yml` | `meta.1` |  | unversioned | not applicable |  |
| `rocq-stdlib` | `.github/workflows/ci.yml` | `9.2.0` | V9.2.0 (GitHub release, rocq-prover/stdlib) | version | current |  |
| `rust-script` | `rust/scripts/install-rust-script.sh` | `latest` |  | floating | current |  |
| `secretlint` | `.github/workflows/js.yml`, `.github/workflows/rust.yml` | `latest` |  | floating | current |  |
| `tree-sitter-cli` | `js/src/vendor/grammars/grammar-lock.json`, `rust/src/data/grammar-lock.json` | `0.27.0` | 0.27.0 (npm registry, tree-sitter-cli latest) | version | current |  |
| `wasm-pack` | `.github/workflows/rust.yml` | `latest` |  | floating | current |  |

## GitHub Actions

| Item | Declared in | Pinned | Current stable release | Comparison | Status | Reason |
|---|---|---|---|---|---|---|
| `actions/cache` | `.github/workflows/ci.yml`, `.github/workflows/rust.yml` | `v6` | v6.1.0 (GitHub release, actions/cache) | major | current |  |
| `actions/checkout` | `.github/workflows/ci.yml`, `.github/workflows/dependency-refresh.yml`, `.github/workflows/js.yml`, `.github/workflows/rust.yml` | `v7` | v7.0.1 (GitHub release, actions/checkout) | major | current |  |
| `actions/configure-pages` | `.github/workflows/rust.yml` | `v6` | v6.0.0 (GitHub release, actions/configure-pages) | major | current |  |
| `actions/deploy-pages` | `.github/workflows/rust.yml` | `v5` | v5.0.1 (GitHub release, actions/deploy-pages) | major | current |  |
| `actions/download-artifact` | `.github/workflows/ci.yml`, `.github/workflows/rust.yml` | `v8` | v8.0.1 (GitHub release, actions/download-artifact) | major | current |  |
| `actions/setup-node` | `.github/workflows/ci.yml`, `.github/workflows/dependency-refresh.yml`, `.github/workflows/js.yml`, `.github/workflows/rust.yml` | `v7` | v7.0.0 (GitHub release, actions/setup-node) | major | current |  |
| `actions/upload-artifact` | `.github/workflows/ci.yml`, `.github/workflows/rust.yml` | `v7` | v7.0.1 (GitHub release, actions/upload-artifact) | major | current |  |
| `actions/upload-pages-artifact` | `.github/workflows/rust.yml` | `v5` | v5.0.0 (GitHub release, actions/upload-pages-artifact) | major | current |  |
| `codecov/codecov-action` | `.github/workflows/rust.yml` | `v7` | v7.1.1 (GitHub release, codecov/codecov-action) | major | current |  |
| `docker/build-push-action` | `.github/workflows/rust.yml` | `v7` | v7.4.0 (GitHub release, docker/build-push-action) | major | current |  |
| `docker/login-action` | `.github/workflows/rust.yml` | `v4` | v4.6.0 (GitHub release, docker/login-action) | major | current |  |
| `docker/metadata-action` | `.github/workflows/rust.yml` | `v6` | v6.2.0 (GitHub release, docker/metadata-action) | major | current |  |
| `docker/setup-buildx-action` | `.github/workflows/rust.yml` | `v4` | v4.4.1 (GitHub release, docker/setup-buildx-action) | major | current |  |
| `dtolnay/rust-toolchain` | `.github/workflows/ci.yml`, `.github/workflows/dependency-refresh.yml` | `1.99.0` | 1.99.0 (GitHub release, rust-lang/rust) | version | current |  |
| `dtolnay/rust-toolchain` | `.github/workflows/rust.yml` | `master` |  | floating | current |  |
| `dtolnay/rust-toolchain` | `.github/workflows/rust.yml` | `stable` |  | floating | current |  |
| `ocaml/setup-ocaml` | `.github/workflows/ci.yml` | `v3` | v3.9.0 (GitHub release, ocaml/setup-ocaml) | major | current |  |
| `peter-evans/create-pull-request` | `.github/workflows/rust.yml` | `v8` | v8.1.1 (GitHub release, peter-evans/create-pull-request) | major | current |  |
| `taiki-e/install-action` | `.github/workflows/rust.yml` | `cargo-llvm-cov` |  | floating | current |  |
| `taiki-e/install-action` | `.github/workflows/rust.yml` | `v2` | v2.87.25 (GitHub release, taiki-e/install-action) | major | current |  |

## Build images

| Item | Declared in | Pinned | Current stable release | Comparison | Status | Reason |
|---|---|---|---|---|---|---|
| `emscripten/emsdk` | `js/scripts/build-web-tree-sitter-runtime.mjs`, `js/src/vendor/web-tree-sitter/runtime-lock.json` | `4.0.15` | 6.0.11 (Docker Hub, emscripten/emsdk tags) | version | behind | Held by tree-sitter v0.27.0, whose `crates/loader/emscripten-version` pins emscripten 4.0.15 for the web-tree-sitter 0.27.0 runtime; another emscripten produces a different runtime, so the image moves with tree-sitter. |

## Runners

| Item | Declared in | Pinned | Current stable release | Comparison | Status | Reason |
|---|---|---|---|---|---|---|
| `macos-latest` | `.github/workflows/ci.yml`, `.github/workflows/rust.yml` | `latest` |  | floating | current |  |
| `ubuntu-latest` | `.github/workflows/ci.yml`, `.github/workflows/dependency-refresh.yml`, `.github/workflows/js.yml`, `.github/workflows/rust.yml` | `latest` |  | floating | current |  |
| `windows-latest` | `.github/workflows/ci.yml`, `.github/workflows/rust.yml` | `latest` |  | floating | current |  |

## Published artifact contents

| Item | Scope | Declared in | Pinned | Current stable release | Comparison | Status | Reason |
|---|---|---|---|---|---|---|---|
| `/LICENSE` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `build.rs` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `CHANGELOG.md` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `examples/**/*.rs` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `README.md` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `src/**/*.rs` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `src/data/concept-records.json` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `src/data/foundation-models.json` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `src/data/grammar-lock.json` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `src/data/language-catalog.json` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `src/data/language-trigrams.json` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `src/data/native-grammars/*.lino` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `src/data/semantic-lexicon.json` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `vendor/tree-sitter-cmake/**` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `vendor/tree-sitter-lean/**` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `vendor/tree-sitter-rocq/**` | `crate` | `rust/Cargo.toml` | `include` |  | unversioned | not applicable |  |
| `README.md` | `npm` | `js/package.json` | `files` |  | unversioned | not applicable |  |
| `src` | `npm` | `js/package.json` | `files` |  | unversioned | not applicable |  |

## Behind the current stable release

26 retained items are behind their current stable release on 2026-10-06, each for the recorded reason.
26 of them are at their newest compatible release, verified against the requirements that hold them;
0 are stale and fail the delivery check.

- `allocator-api2` `0.2.21` → `0.4.0` (`rust/Cargo.lock`): Held by the requirement of `hashbrown` 0.15.5 (`^0.2.9`), which does not admit 0.4.0; it moves when that dependent does. Delivered at its newest compatible release `0.2.21`, held by `crate rust/Cargo.lock hashbrown@0.15.5` (`^0.2.9`).
- `cc` `1.2.67` → `1.6.0` (`rust/Cargo.lock`, `rust/Cargo.toml`): Held by the requirement of `tree-sitter-sequel` 0.3.11 (`~1.2.1`), which does not admit 1.6.0; it moves when that dependent does. Delivered at its newest compatible release `1.2.67`, held by `crate rust/Cargo.lock tree-sitter-sequel@0.3.11` (`~1.2.1`).
- `foldhash` `0.1.5` → `0.2.0` (`rust/Cargo.lock`): Held by the requirement of `hashbrown` 0.15.5 (`^0.1.2`), which does not admit 0.2.0; it moves when that dependent does. Delivered at its newest compatible release `0.1.5`, held by `crate rust/Cargo.lock hashbrown@0.15.5` (`^0.1.2`).
- `getrandom` `0.2.17` → `0.4.3` (`rust/Cargo.lock`): Held by the requirement of `ring` 0.17.14 (`^0.2.10`), which does not admit 0.4.3; it moves when that dependent does. Delivered at its newest compatible release `0.2.17`, held by `crate rust/Cargo.lock ring@0.17.14` (`^0.2.10`).
- `hashbrown` `0.14.5` → `0.17.1` (`rust/Cargo.lock`): Held by the requirement of `dashmap` 6.2.1 (`^0.14.5`), which does not admit 0.17.1; it moves when that dependent does. Delivered at its newest compatible release `0.14.5`, held by `crate rust/Cargo.lock dashmap@6.2.1` (`^0.14.5`).
- `hashbrown` `0.15.5` → `0.17.1` (`rust/Cargo.lock`): Held by the requirement of `whatlang` 0.18.0 (`^0.15`), which does not admit 0.17.1; it moves when that dependent does. Delivered at its newest compatible release `0.15.5`, held by `crate rust/Cargo.lock whatlang@0.18.0` (`^0.15`).
- `itertools` `0.14.0` → `0.15.0` (`rust/Cargo.lock`): Held by the requirement of `lingua` 1.8.0 (`^0.14.0`), which does not admit 0.15.0; it moves when that dependent does. Delivered at its newest compatible release `0.14.0`, held by `crate rust/Cargo.lock lingua@1.8.0` (`^0.14.0`).
- `r-efi` `6.0.0` → `7.1.0` (`rust/Cargo.lock`): Held by the requirement of `getrandom` 0.4.3 (`^6`), which does not admit 7.1.0; it moves when that dependent does. Delivered at its newest compatible release `6.0.0`, held by `crate rust/Cargo.lock getrandom@0.4.3` (`^6`).
- `redox_syscall` `0.5.18` → `0.9.4` (`rust/Cargo.lock`): Held by the requirement of `parking_lot_core` 0.9.12 (`^0.5`), which does not admit 0.9.4; it moves when that dependent does. Delivered at its newest compatible release `0.5.18`, held by `crate rust/Cargo.lock parking_lot_core@0.9.12` (`^0.5`).
- `strum_macros` `0.27.2` → `0.28.0` (`rust/Cargo.lock`): Held by the requirement of `lingua` 1.8.0 (`^0.27.2`), which does not admit 0.28.0; it moves when that dependent does. Delivered at its newest compatible release `0.27.2`, held by `crate rust/Cargo.lock lingua@1.8.0` (`^0.27.2`).
- `strum` `0.27.2` → `0.28.0` (`rust/Cargo.lock`): Held by the requirement of `lingua` 1.8.0 (`^0.27.2`), which does not admit 0.28.0; it moves when that dependent does. Delivered at its newest compatible release `0.27.2`, held by `crate rust/Cargo.lock lingua@1.8.0` (`^0.27.2`).
- `syn` `2.0.119` → `3.0.6` (`rust/Cargo.lock`): Held by the requirement of `munge_macro` 0.4.7 (`^2`), `pest_generator` 2.9.2 (`^2.0`), `strum_macros` 0.27.2 (`^2.0`), `strum_macros` 0.28.0 (`^2.0`), which does not admit 3.0.6; it moves when that dependent does. Delivered at its newest compatible release `2.0.119`, held by `crate rust/Cargo.lock munge_macro@0.4.7` (`^2`), `crate rust/Cargo.lock pest_generator@2.9.2` (`^2.0`), `crate rust/Cargo.lock strum_macros@0.27.2` (`^2.0`), `crate rust/Cargo.lock strum_macros@0.28.0` (`^2.0`).
- `wasi` `0.11.1+wasi-snapshot-preview1` → `0.14.7+wasi-0.2.4` (`rust/Cargo.lock`): Held by the requirement of `getrandom` 0.2.17 (`^0.11`), which does not admit 0.14.7+wasi-0.2.4; it moves when that dependent does. Delivered at its newest compatible release `0.11.1+wasi-snapshot-preview1`, held by `crate rust/Cargo.lock getrandom@0.2.17` (`^0.11`).
- `windows_aarch64_gnullvm` `0.52.6` → `0.53.1` (`rust/Cargo.lock`): Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. Delivered at its newest compatible release `0.52.6`, held by `crate rust/Cargo.lock windows-targets@0.52.6` (`^0.52.6`).
- `windows_aarch64_msvc` `0.52.6` → `0.53.1` (`rust/Cargo.lock`): Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. Delivered at its newest compatible release `0.52.6`, held by `crate rust/Cargo.lock windows-targets@0.52.6` (`^0.52.6`).
- `windows_i686_gnu` `0.52.6` → `0.53.1` (`rust/Cargo.lock`): Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. Delivered at its newest compatible release `0.52.6`, held by `crate rust/Cargo.lock windows-targets@0.52.6` (`^0.52.6`).
- `windows_i686_gnullvm` `0.52.6` → `0.53.1` (`rust/Cargo.lock`): Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. Delivered at its newest compatible release `0.52.6`, held by `crate rust/Cargo.lock windows-targets@0.52.6` (`^0.52.6`).
- `windows_i686_msvc` `0.52.6` → `0.53.1` (`rust/Cargo.lock`): Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. Delivered at its newest compatible release `0.52.6`, held by `crate rust/Cargo.lock windows-targets@0.52.6` (`^0.52.6`).
- `windows_x86_64_gnu` `0.52.6` → `0.53.1` (`rust/Cargo.lock`): Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. Delivered at its newest compatible release `0.52.6`, held by `crate rust/Cargo.lock windows-targets@0.52.6` (`^0.52.6`).
- `windows_x86_64_gnullvm` `0.52.6` → `0.53.1` (`rust/Cargo.lock`): Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. Delivered at its newest compatible release `0.52.6`, held by `crate rust/Cargo.lock windows-targets@0.52.6` (`^0.52.6`).
- `windows_x86_64_msvc` `0.52.6` → `0.53.1` (`rust/Cargo.lock`): Held by the requirement of `windows-targets` 0.52.6 (`^0.52.6`), which does not admit 0.53.1; it moves when that dependent does. Delivered at its newest compatible release `0.52.6`, held by `crate rust/Cargo.lock windows-targets@0.52.6` (`^0.52.6`).
- `windows-link` `0.2.1` → `0.100.0` (`rust/Cargo.lock`): Held by the requirement of `parking_lot_core` 0.9.12 (`^0.2.0`), `windows-sys` 0.61.2 (`^0.2.1`), which does not admit 0.100.0; it moves when that dependent does. Delivered at its newest compatible release `0.2.1`, held by `crate rust/Cargo.lock parking_lot_core@0.9.12` (`^0.2.0`), `crate rust/Cargo.lock windows-sys@0.61.2` (`^0.2.1`).
- `windows-sys` `0.52.0` → `0.61.2` (`rust/Cargo.lock`): Held by the requirement of `ring` 0.17.14 (`^0.52`), which does not admit 0.61.2; it moves when that dependent does. Delivered at its newest compatible release `0.52.0`, held by `crate rust/Cargo.lock ring@0.17.14` (`^0.52`).
- `windows-targets` `0.52.6` → `0.53.5` (`rust/Cargo.lock`): Held by the requirement of `windows-sys` 0.52.0 (`^0.52.0`), which does not admit 0.53.5; it moves when that dependent does. Delivered at its newest compatible release `0.52.6`, held by `crate rust/Cargo.lock windows-sys@0.52.0` (`^0.52.0`).
- `ocaml` `5.4` → `5.5.1` (`.github/workflows/ci.yml`): Held by ocamlfind 1.9.8, the newest ocamlfind release in opam-repository, which requires OCaml <5.5.0; rocq-runtime 9.3.0 needs ocamlfind (>=1.9.1), so the Rocq acceptance job builds with the newest OCaml it admits and moves when ocamlfind admits a newer line. Delivered at its newest compatible release `5.4.1`, held by `opam ocamlfind 1.9.8` (`<5.5.0`).
- `emscripten/emsdk` `4.0.15` → `6.0.11` (`js/scripts/build-web-tree-sitter-runtime.mjs`, `js/src/vendor/web-tree-sitter/runtime-lock.json`): Held by tree-sitter v0.27.0, whose `crates/loader/emscripten-version` pins emscripten 4.0.15 for the web-tree-sitter 0.27.0 runtime; another emscripten produces a different runtime, so the image moves with tree-sitter. Delivered at its newest compatible release `4.0.15`, held by `npm js/package-lock.json web-tree-sitter@0.27.0` (`=4.0.15`).
