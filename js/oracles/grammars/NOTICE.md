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
| `agda.wasm.gz` | crate `tree-sitter-agda` 1.3.3 | `ab45429d4acee054bcc0d93dabc7b818051b8c98aca3695b3a08b5ba47315980` | `9587dfbf28a91100dc4cb8717288ae9d99693e1d8aade1396443a5142fe28f7a` | [`agda.LICENSE`](agda.LICENSE) |
| `c.wasm.gz` | crate `tree-sitter-c` 0.24.2 | `f2883ff9b21f4a5bd5553c1b10366c418947d17a7bc6bf256124a7542f079dd2` | `2030a766fa6c6fe18039524008e52fd4a4995bdd9ed07fc4a73ec892450d2f4a` | [`c.LICENSE`](c.LICENSE) |
| `cmake.wasm.gz` | `uyha/tree-sitter-cmake` v0.7.5 with [`scanner-state.patch`](../../../rust/vendor/tree-sitter-cmake/scanner-state.patch) (vendored in `rust/vendor/tree-sitter-cmake`) | `ece92c2e3fbf15634fb90fab91d0af749348c069d27208eeb10dcdd4e77fe5a2` | `5925d5f08a726ebb9024ccae1e12165a56c35630c53d9dd7fc3eeb87434849c2` | [`cmake.LICENSE`](cmake.LICENSE) |
| `cpp.wasm.gz` | crate `tree-sitter-cpp` 0.23.4 | `2a35a43b4af6c9f7b69624ac00c2c50808912591450dc79c05dea03ac1bae814` | `f5ff3ba7af054af846f37dfd2525bdfede95530d5747a8fe30df87d070ece607` | [`cpp.LICENSE`](cpp.LICENSE) |
| `css.wasm.gz` | crate `tree-sitter-css` 0.25.0 | `2e5150071220012ee635ac9e2119f4f3a93e0b51a7a3b69ab0cd87c18cf5e51e` | `f0fa4c3171f3322560e3a56445989f2da9d77c38bae61e9f07ae0b3e01d88b36` | [`css.LICENSE`](css.LICENSE) |
| `csv.wasm.gz` | `tree-sitter-grammars/tree-sitter-csv` f6bf6e35eb0b95fbadea4bb39cb9709507fcb181 with [`rfc4180-quotes.patch`](../../../rust/vendor/tree-sitter-csv/rfc4180-quotes.patch) (vendored in `rust/vendor/tree-sitter-csv`) | `6a7ede804cb8ced3f9466cecf38872d175e6179e8ea951956a195b2d55add8bc` | `926e300e68fca1881734251e2b0b3a51cc56625c1afd3471f3102ce5274a1cd9` | [`csv.LICENSE`](csv.LICENSE) |
| `dart.wasm.gz` | crate `tree-sitter-dart` 0.2.0 | `1894ef57f8e0ceb483ccf6d548429517dc42b0d983293ebdb5ff9ff9c612c4a9` | `e700b38561a3f1e641340fac8232dfca493dbc024d142d338a5075feca3e9efc` | [`dart.LICENSE`](dart.LICENSE) |
| `diff.wasm.gz` | crate `tree-sitter-diff` 0.1.0 | `4c9e3e545073a7d93088f7d90b0ee89e56fb43ca751f25fa440c4a1906ecb3e6` | `8c32b60b7900e18ad10e2223e6311f4f1aff29b4d12226862a2a889a99596f29` | [`diff.LICENSE`](diff.LICENSE) |
| `dtd.wasm.gz` | crate `tree-sitter-xml` 0.7.0 (`dtd`) | `79aa52e71ba9685115e2fc2742a3a30f91987f96fcf5ffbb7e34827155a6b932` | `2534944fea9484174b791535509c09a25fcd1c4099da9392056b009311dcfe81` | [`dtd.LICENSE`](dtd.LICENSE) |
| `erlang.wasm.gz` | crate `tree-sitter-erlang` 0.20.0 | `c8c83443d536875c2f28f12ae4f0242f477cac82115073f4d9506c2bb188dbcc` | `345a2693219c24236afdbacf3af1082aaa59eef6c21c06371845690505ee3af3` | [`erlang.LICENSE`](erlang.LICENSE) |
| `go.wasm.gz` | crate `tree-sitter-go` 0.25.0 | `3dbf6ed1238b5dfcf2be4d2f2d4cb27a14d34f34d7784eccccbfd532fd4a6d85` | `058a4f6c7cb7156c599c2ca07af9645f03b3f655b84bced9c72816e755c7030b` | [`go.LICENSE`](go.LICENSE) |
| `graphql.wasm.gz` | crate `tree-sitter-graphql` 0.3.0 | `8b7ebff40dc5b802df9ec03b1570fd53d234300ac4705899643ae61dda77dc57` | `a9239652fcc467ec3bf9dbe7d5271ff3511c40c7dc9bba440667599dcfb608bc` | [`graphql.LICENSE`](graphql.LICENSE) |
| `groovy.wasm.gz` | crate `tree-sitter-groovy` 0.1.2 | `786637c026add3b2112aa717fa17b34e3093ea291b3b46d60438ba84382fdb06` | `9c7236d8ab8c127a59644321213e479ab8ac0f53ac5720c1461ef406e46db4a6` | [`groovy.LICENSE`](groovy.LICENSE) |
| `hcl.wasm.gz` | crate `tree-sitter-hcl` 1.1.0 | `0c821e2979f54f8f460201eafe9464d5d3dae0076c5bdd51567a798fb7add543` | `c7fa4ff3cecbd8c9eba1b9643b511a5b226b47d29eb5ec38656a5bc7a216bfc5` | [`hcl.LICENSE`](hcl.LICENSE) |
| `html.wasm.gz` | crate `tree-sitter-html` 0.23.2 | `65768172733b3bbe461cbdc14ea928f00fbfcc51d8ac68f0a5c72071c6a0bbf1` | `11960fcc4fe3a01930de6bbae9b2c5c4d0a6c16012a5d20cc2069fa5b78d3bbd` | [`html.LICENSE`](html.LICENSE) |
| `ini.wasm.gz` | crate `tree-sitter-ini` 1.4.0 | `3537bf540af5b3def849c4f15d4ba8a02b37f5414849140f1c139d2abb1e4123` | `49e4442e41b44226268a89d810a4a3d2b3dde810b539579c2acd4a55223d8548` | [`ini.LICENSE`](ini.LICENSE) |
| `java.wasm.gz` | crate `tree-sitter-java` 0.23.5 | `4add5150cf4531eb5dd97f3343dcf65cd11704c84711348b328582b83424a0e4` | `6476728734128931bd50de2d1e5e9c75c506b51e278f54eaf709836a0de35759` | [`java.LICENSE`](java.LICENSE) |
| `javascript.wasm.gz` | crate `tree-sitter-javascript` 0.25.0 | `67209ca7ef6e1a4f74e29e48b5928455f892fe1821a3960fbcd62f4e972f7384` | `7ccbce5ba189363aee524c4be7f3a0bdb25b961d4c6d2472a8f3aeb526eecbfe` | [`javascript.LICENSE`](javascript.LICENSE) |
| `json.wasm.gz` | crate `tree-sitter-json` 0.24.8 | `e8e1ff5df0d73e3b82574129724e68ef4fa0faf1b8c43dd3f5c1a84839f830ab` | `564e489724cbcf9b4563cd758a7fb7f355f896b11127819ae9e8ef71e7c15e60` | [`json.LICENSE`](json.LICENSE) |
| `json5.wasm.gz` | crate `tree-sitter-json5-orchard` 0.1.0 | `0024a5353393c7186a9a4f531f451279d301c7c3925ba292348176a83d3765b4` | `b726ecea6c05ff5b62b9a32563762d297e019c5eaaa7371d20b2245ca17421f4` | [`json5.LICENSE`](json5.LICENSE) |
| `lean.wasm.gz` | `wvhulle/tree-sitter-lean` bd942cd2795016239be02b3b3d5ef635645ddd38 (vendored in `rust/vendor/tree-sitter-lean`) | `d04d21e8ad0132f9f3410296817a4317fc11d4f8cbd35c62dc3fe0b72f4f9a6b` | `7b58425afbe7f59e90fa9f01ebafe9241cdedbf1fe7e2daca4bec2f15122cc59` | [`lean.LICENSE`](lean.LICENSE) |
| `lua.wasm.gz` | crate `tree-sitter-lua` 0.5.0 | `933206d96a78f7785c13b2600182f1527dcd755c200b1271bb5bc4d8da4b17b3` | `43db0f3e64ffd4ed1ada7a77e4489e54afa5afd7b5aa013679d8284d4ea0f074` | [`lua.LICENSE`](lua.LICENSE) |
| `make.wasm.gz` | crate `tree-sitter-make` 1.1.1 | `659bb2a3712bc6ecf77a96c82de2bc3062141f405c96d52582b2e01bd8722152` | `3cd0a9967c477e0cf3330d96ae79e2bbc8580119daf7e02266649fa6cd136e76` | [`make.LICENSE`](make.LICENSE) |
| `nix.wasm.gz` | crate `tree-sitter-nix` 0.3.0 | `2b1d7945441b83e324cd225663481d35e51cd9ae84189cc7e69f58a11ecb85ad` | `98faa2e223fc3fe88c20c02c51fd739da8ebe1f1591b131d0964ede6cf320b08` | [`nix.LICENSE`](nix.LICENSE) |
| `odin.wasm.gz` | crate `tree-sitter-odin` 1.3.0 | `8f672d8d022d70eeae822796924f2e45e692d6b2acd2ddc79bfeb5f696fa24f0` | `8ce37ea03c3d565d1c4e922d357a82fd9aa5184c36f712509803a63fec8d3194` | [`odin.LICENSE`](odin.LICENSE) |
| `pascal.wasm.gz` | crate `tree-sitter-pascal` 0.10.2 | `d7a5f5c04880fac79e0eb1841cbbc00d7e305863efe56d83c9754fba70385329` | `e1ae02b62ce27ba86d202e77fa42f5e59d8e945b00ff64803cc275741ecca628` | [`pascal.LICENSE`](pascal.LICENSE) |
| `powershell.wasm.gz` | crate `tree-sitter-powershell` 0.26.4 | `f39f67abaae4c488ccea5d9bbd003ef0b00c303779e0491950886a12a9305720` | `84555e38f85a391095b349a906b9790d319e3946ce12d9bc21dbec5cded5dad9` | [`powershell.LICENSE`](powershell.LICENSE) |
| `proto.wasm.gz` | crate `tree-sitter-proto` 0.6.0 | `ac481450c32e8fa52976075581701ecc63790fbab4c9c4f10d0901c9a39b61eb` | `2179ef77376c7c1a1c08c145915c8902436d6f08724cb055ce597879f107bf61` | [`proto.LICENSE`](proto.LICENSE) |
| `python.wasm.gz` | crate `tree-sitter-python` 0.25.0 | `a895f10b3cf7b2608f3283b43cd5cfed70971c7ee4a0136abbaaccbc4a7a25e0` | `3523e4dc3d12894bbb2f7ea4b86b50eec69aed321c3d9b49271d82ed9f5f79b0` | [`python.LICENSE`](python.LICENSE) |
| `r.wasm.gz` | crate `tree-sitter-r` 1.3.0 | `43ec2413de8aec823c76e6994991fe07d9877e019ab2e1892534a76ce81a0771` | `8443d229942c1719c40fac5729e8d9252e48950aa2e0dbcf86c52bd38e875209` | [`r.LICENSE`](r.LICENSE) |
| `racket.wasm.gz` | crate `tree-sitter-racket` 0.25.0 | `3cbab79ab9bd99684a7de4cb93a0aafd6773bb3ecf6720c59f1b7cdb0ceddd9c` | `5affec93f88d8b9e66aee6ad9a385d04f6af07dfc55bc6944993b02af1fa21b2` | [`racket.LICENSE`](racket.LICENSE) |
| `regex.wasm.gz` | crate `tree-sitter-regex` 0.25.0 | `ddf28eb5ad0dd0898b9ed0f2593852edac639a339a410d9e93350e472859700f` | `0b64ac3dc55cfb73a95da3ea3eb1ac89e0f8fafd6050aa32139988b8413c14f1` | [`regex.LICENSE`](regex.LICENSE) |
| `rocq.wasm.gz` | `aruzdh/tree-sitter-rocq` 300fe33fc299c30f736fd56d8ef8a28b08acd4e6 with [`meta-language.patch`](../../../rust/vendor/tree-sitter-rocq/meta-language.patch) (vendored in `rust/vendor/tree-sitter-rocq`) | `4f0807b15d47e4335c28ccb7be5eb8d01851baaab4843dfa24b2d8fb1966ecd2` | `9d0d3172500d14ee0d06d61b0d3a3a33e757003a94eb7f1c7221b11c740e0530` | [`rocq.LICENSE`](rocq.LICENSE) |
| `rust.wasm.gz` | `tree-sitter/tree-sitter-rust` v0.24.2 with [`meta-language.patch`](../../../rust/vendor/tree-sitter-rust/meta-language.patch) (vendored in `rust/vendor/tree-sitter-rust`) | `555eee81e8d1ab541d5c76ab5a17bc37abb3c37a43ae02a11d60aaef0cb765a3` | `9e42546db0d259b49f09ce1170528b57d97453f19824ca27197016c330dcae03` | [`rust.LICENSE`](rust.LICENSE) |
| `scheme.wasm.gz` | crate `tree-sitter-scheme` 0.24.7 | `5d4c1786eb70c3be05f395e5a26edbb7cca12e7547f7a1d8cdb94e092ee74d0d` | `f138334dfc4d3df3c85c3e0d675e154f2bd6b1a8d4325f04c53512eef099241e` | [`scheme.LICENSE`](scheme.LICENSE) |
| `solidity.wasm.gz` | crate `tree-sitter-solidity` 1.2.13 | `e71971f6ddc0704e81f4af3f43ecc79793742bb5f1d256a5fdd718aa06d74acb` | `7ae2d8aff2ec119ff8706667f45f5ff0d4edea901acef86e227664de83c6df51` | [`solidity.LICENSE`](solidity.LICENSE) |
| `sql.wasm.gz` | crate `tree-sitter-sequel` 0.3.11 | `852e088fb8470952cdb2a1b78c1c58626c7d91562b26baa4672d51f9754bf580` | `e03fb5079e88cfe0807056c284fd2b0589cee35df9bbb3d07e72f2f4178de8de` | [`sql.LICENSE`](sql.LICENSE) |
| `toml.wasm.gz` | crate `tree-sitter-toml-ng` 0.7.0 | `1991a2608e6f0214e563fe5f762e8df72954dc69a3dd9a1e22ea8a870e4052c3` | `057f48e81072cb0eb5969632a7cf032fc6d33cd4ba3198c3f58227346c012d97` | [`toml.LICENSE`](toml.LICENSE) |
| `tsx.wasm.gz` | crate `tree-sitter-typescript` 0.23.2 (`tsx`) | `1902cb53fa7ff5179df89b2eea863165e84c8cc866226419dc26921d8c055885` | `3c6cb1c86cecc6d8c99b3a7dfc0aba8b838e2adf036bb23f384d6d8be395ae30` | [`tsx.LICENSE`](tsx.LICENSE) |
| `typescript.wasm.gz` | crate `tree-sitter-typescript` 0.23.2 (`typescript`) | `74fe453edd70f4eae9af0a1050cbd7943d8971d59165b6aaebbaa0a0b716d1aa` | `d24392183c75c103feafb827c9c086de00c30198e27f362cc971fbca6e91a87f` | [`typescript.LICENSE`](typescript.LICENSE) |
| `vb.wasm.gz` | crate `tree-sitter-vb-dotnet` 0.1.0 | `7e9e6275f9b83c1f6c79112fd2021effc8529e9d6b794bbd41e39d99db1214d8` | `4bc68c0fb1cbc42a44eb3199affe54173b6519f52ffdd173e31c9dc87b9099f8` | [`vb.LICENSE`](vb.LICENSE) |
| `xml.wasm.gz` | crate `tree-sitter-xml` 0.7.0 (`xml`) | `e41811d97cfd672a902924a3d99bdf696d6fcec08188a35404e4a3d2698ab844` | `2edd39d0ef194dc70878fe99b277180547fabf252f85b58e537db5a11e3027fa` | [`xml.LICENSE`](xml.LICENSE) |
| `zig.wasm.gz` | crate `tree-sitter-zig` 1.1.2 | `5449f98eb876939fcb12be76891ecb0c99b78be3bfe843e140d64c680b66ae63` | `cc95fa4d69178617b19fe50b070dc871e315aea55107fcd1bcd238ae77811edd` | [`zig.LICENSE`](zig.LICENSE) |

Every grammar is distributed under its upstream license (MIT unless the
linked license file states otherwise).
