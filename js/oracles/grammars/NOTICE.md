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
| `go.wasm.gz` | crate `tree-sitter-go` 0.25.0 | `3dbf6ed1238b5dfcf2be4d2f2d4cb27a14d34f34d7784eccccbfd532fd4a6d85` | `058a4f6c7cb7156c599c2ca07af9645f03b3f655b84bced9c72816e755c7030b` | [`go.LICENSE`](go.LICENSE) |
| `graphql.wasm.gz` | crate `tree-sitter-graphql` 0.3.0 | `8b7ebff40dc5b802df9ec03b1570fd53d234300ac4705899643ae61dda77dc57` | `a9239652fcc467ec3bf9dbe7d5271ff3511c40c7dc9bba440667599dcfb608bc` | [`graphql.LICENSE`](graphql.LICENSE) |
| `ini.wasm.gz` | crate `tree-sitter-ini` 1.4.0 | `3537bf540af5b3def849c4f15d4ba8a02b37f5414849140f1c139d2abb1e4123` | `49e4442e41b44226268a89d810a4a3d2b3dde810b539579c2acd4a55223d8548` | [`ini.LICENSE`](ini.LICENSE) |
| `java.wasm.gz` | crate `tree-sitter-java` 0.23.5 | `4add5150cf4531eb5dd97f3343dcf65cd11704c84711348b328582b83424a0e4` | `6476728734128931bd50de2d1e5e9c75c506b51e278f54eaf709836a0de35759` | [`java.LICENSE`](java.LICENSE) |
| `javascript.wasm.gz` | crate `tree-sitter-javascript` 0.25.0 | `67209ca7ef6e1a4f74e29e48b5928455f892fe1821a3960fbcd62f4e972f7384` | `7ccbce5ba189363aee524c4be7f3a0bdb25b961d4c6d2472a8f3aeb526eecbfe` | [`javascript.LICENSE`](javascript.LICENSE) |
| `json.wasm.gz` | crate `tree-sitter-json` 0.24.8 | `e8e1ff5df0d73e3b82574129724e68ef4fa0faf1b8c43dd3f5c1a84839f830ab` | `564e489724cbcf9b4563cd758a7fb7f355f896b11127819ae9e8ef71e7c15e60` | [`json.LICENSE`](json.LICENSE) |
| `json5.wasm.gz` | crate `tree-sitter-json5-orchard` 0.1.0 | `0024a5353393c7186a9a4f531f451279d301c7c3925ba292348176a83d3765b4` | `b726ecea6c05ff5b62b9a32563762d297e019c5eaaa7371d20b2245ca17421f4` | [`json5.LICENSE`](json5.LICENSE) |
| `make.wasm.gz` | crate `tree-sitter-make` 1.1.1 | `659bb2a3712bc6ecf77a96c82de2bc3062141f405c96d52582b2e01bd8722152` | `3cd0a9967c477e0cf3330d96ae79e2bbc8580119daf7e02266649fa6cd136e76` | [`make.LICENSE`](make.LICENSE) |
| `proto.wasm.gz` | crate `tree-sitter-proto` 0.6.0 | `ac481450c32e8fa52976075581701ecc63790fbab4c9c4f10d0901c9a39b61eb` | `2179ef77376c7c1a1c08c145915c8902436d6f08724cb055ce597879f107bf61` | [`proto.LICENSE`](proto.LICENSE) |
| `racket.wasm.gz` | crate `tree-sitter-racket` 0.25.0 | `3cbab79ab9bd99684a7de4cb93a0aafd6773bb3ecf6720c59f1b7cdb0ceddd9c` | `5affec93f88d8b9e66aee6ad9a385d04f6af07dfc55bc6944993b02af1fa21b2` | [`racket.LICENSE`](racket.LICENSE) |
| `regex.wasm.gz` | crate `tree-sitter-regex` 0.25.0 | `ddf28eb5ad0dd0898b9ed0f2593852edac639a339a410d9e93350e472859700f` | `0b64ac3dc55cfb73a95da3ea3eb1ac89e0f8fafd6050aa32139988b8413c14f1` | [`regex.LICENSE`](regex.LICENSE) |
| `rust.wasm.gz` | `tree-sitter/tree-sitter-rust` v0.24.2 with [`meta-language.patch`](../../../rust/vendor/tree-sitter-rust/meta-language.patch) (vendored in `rust/vendor/tree-sitter-rust`) | `555eee81e8d1ab541d5c76ab5a17bc37abb3c37a43ae02a11d60aaef0cb765a3` | `9e42546db0d259b49f09ce1170528b57d97453f19824ca27197016c330dcae03` | [`rust.LICENSE`](rust.LICENSE) |
| `scheme.wasm.gz` | crate `tree-sitter-scheme` 0.24.7 | `5d4c1786eb70c3be05f395e5a26edbb7cca12e7547f7a1d8cdb94e092ee74d0d` | `f138334dfc4d3df3c85c3e0d675e154f2bd6b1a8d4325f04c53512eef099241e` | [`scheme.LICENSE`](scheme.LICENSE) |
| `solidity.wasm.gz` | crate `tree-sitter-solidity` 1.2.13 | `e71971f6ddc0704e81f4af3f43ecc79793742bb5f1d256a5fdd718aa06d74acb` | `7ae2d8aff2ec119ff8706667f45f5ff0d4edea901acef86e227664de83c6df51` | [`solidity.LICENSE`](solidity.LICENSE) |
| `tsx.wasm.gz` | crate `tree-sitter-typescript` 0.23.2 (`tsx`) | `1902cb53fa7ff5179df89b2eea863165e84c8cc866226419dc26921d8c055885` | `3c6cb1c86cecc6d8c99b3a7dfc0aba8b838e2adf036bb23f384d6d8be395ae30` | [`tsx.LICENSE`](tsx.LICENSE) |
| `typescript.wasm.gz` | crate `tree-sitter-typescript` 0.23.2 (`typescript`) | `74fe453edd70f4eae9af0a1050cbd7943d8971d59165b6aaebbaa0a0b716d1aa` | `d24392183c75c103feafb827c9c086de00c30198e27f362cc971fbca6e91a87f` | [`typescript.LICENSE`](typescript.LICENSE) |

Every grammar is distributed under its upstream license (MIT unless the
linked license file states otherwise).
