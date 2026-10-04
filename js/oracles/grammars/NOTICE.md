# Oracle tree-sitter grammars

The languages these grammars describe are parsed by the native grammars of
`parity/grammars/native`. The tree-sitter grammars stay as their oracles: the
fixture generators and the tests compare the native trees with theirs, so they
are development files the npm package does not ship (Rust links them as
development dependencies). Their `grammar-lock.json` entries have
`"oracle": true`. They are compiled from exactly the generated parsers of
`rust/Cargo.lock` crates or `rust/vendor`, and rebuilt by
`node js/scripts/build-vendored-grammars.mjs` with tree-sitter CLI
0.27.0 and verified against `grammar-lock.json` by
`--check`. Each file is a zero-mtime gzip of the `.wasm` whose SHA-256 is
listed; the parser digest is of the generated `src/parser.c` (after the
listed patch, if any).

| File | Source | parser.c SHA-256 | wasm SHA-256 | License |
| --- | --- | --- | --- | --- |
| `c.wasm.gz` | crate `tree-sitter-c` 0.24.2 | `f2883ff9b21f4a5bd5553c1b10366c418947d17a7bc6bf256124a7542f079dd2` | `2030a766fa6c6fe18039524008e52fd4a4995bdd9ed07fc4a73ec892450d2f4a` | [`c.LICENSE`](c.LICENSE) |
| `csv.wasm.gz` | `tree-sitter-grammars/tree-sitter-csv` f6bf6e35eb0b95fbadea4bb39cb9709507fcb181 with [`rfc4180-quotes.patch`](../../../rust/vendor/tree-sitter-csv/rfc4180-quotes.patch) (vendored in `rust/vendor/tree-sitter-csv`) | `6a7ede804cb8ced3f9466cecf38872d175e6179e8ea951956a195b2d55add8bc` | `926e300e68fca1881734251e2b0b3a51cc56625c1afd3471f3102ce5274a1cd9` | [`csv.LICENSE`](csv.LICENSE) |
| `diff.wasm.gz` | crate `tree-sitter-diff` 0.1.0 | `4c9e3e545073a7d93088f7d90b0ee89e56fb43ca751f25fa440c4a1906ecb3e6` | `8c32b60b7900e18ad10e2223e6311f4f1aff29b4d12226862a2a889a99596f29` | [`diff.LICENSE`](diff.LICENSE) |
| `ini.wasm.gz` | crate `tree-sitter-ini` 1.4.0 | `3537bf540af5b3def849c4f15d4ba8a02b37f5414849140f1c139d2abb1e4123` | `49e4442e41b44226268a89d810a4a3d2b3dde810b539579c2acd4a55223d8548` | [`ini.LICENSE`](ini.LICENSE) |
| `javascript.wasm.gz` | crate `tree-sitter-javascript` 0.25.0 | `67209ca7ef6e1a4f74e29e48b5928455f892fe1821a3960fbcd62f4e972f7384` | `7ccbce5ba189363aee524c4be7f3a0bdb25b961d4c6d2472a8f3aeb526eecbfe` | [`javascript.LICENSE`](javascript.LICENSE) |
| `json.wasm.gz` | crate `tree-sitter-json` 0.24.8 | `e8e1ff5df0d73e3b82574129724e68ef4fa0faf1b8c43dd3f5c1a84839f830ab` | `564e489724cbcf9b4563cd758a7fb7f355f896b11127819ae9e8ef71e7c15e60` | [`json.LICENSE`](json.LICENSE) |
| `json5.wasm.gz` | crate `tree-sitter-json5-orchard` 0.1.0 | `0024a5353393c7186a9a4f531f451279d301c7c3925ba292348176a83d3765b4` | `b726ecea6c05ff5b62b9a32563762d297e019c5eaaa7371d20b2245ca17421f4` | [`json5.LICENSE`](json5.LICENSE) |
| `racket.wasm.gz` | crate `tree-sitter-racket` 0.25.0 | `3cbab79ab9bd99684a7de4cb93a0aafd6773bb3ecf6720c59f1b7cdb0ceddd9c` | `5affec93f88d8b9e66aee6ad9a385d04f6af07dfc55bc6944993b02af1fa21b2` | [`racket.LICENSE`](racket.LICENSE) |
| `rust.wasm.gz` | `tree-sitter/tree-sitter-rust` v0.24.2 with [`meta-language.patch`](../../../rust/vendor/tree-sitter-rust/meta-language.patch) (vendored in `rust/vendor/tree-sitter-rust`) | `555eee81e8d1ab541d5c76ab5a17bc37abb3c37a43ae02a11d60aaef0cb765a3` | `9e42546db0d259b49f09ce1170528b57d97453f19824ca27197016c330dcae03` | [`rust.LICENSE`](rust.LICENSE) |
| `scheme.wasm.gz` | crate `tree-sitter-scheme` 0.24.7 | `5d4c1786eb70c3be05f395e5a26edbb7cca12e7547f7a1d8cdb94e092ee74d0d` | `f138334dfc4d3df3c85c3e0d675e154f2bd6b1a8d4325f04c53512eef099241e` | [`scheme.LICENSE`](scheme.LICENSE) |

Every grammar is distributed under its upstream license (MIT unless the
linked license file states otherwise).
