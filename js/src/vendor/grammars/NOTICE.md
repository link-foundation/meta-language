# Vendored tree-sitter grammars

These WebAssembly grammars are compiled from exactly the generated parsers the
Rust crate links (`rust/Cargo.lock` crates or `rust/vendor`), so both
runtimes parse with the same grammar revision. They are rebuilt by
`node js/scripts/build-vendored-grammars.mjs` with tree-sitter CLI
0.27.0 and verified against `grammar-lock.json` by
`--check`. Each file is a zero-mtime gzip of the `.wasm` whose SHA-256 is
listed; the parser digest is of the generated `src/parser.c` (after the
listed patch, if any).

The tree-sitter oracles of the languages a native grammar parses are
development files this package does not ship (`js/oracles/grammars` in the
repository).

| File | Source | parser.c SHA-256 | wasm SHA-256 | License |
| --- | --- | --- | --- | --- |
| `agda.wasm.gz` | crate `tree-sitter-agda` 1.3.3 | `ab45429d4acee054bcc0d93dabc7b818051b8c98aca3695b3a08b5ba47315980` | `9587dfbf28a91100dc4cb8717288ae9d99693e1d8aade1396443a5142fe28f7a` | [`agda.LICENSE`](agda.LICENSE) |
| `bash.wasm.gz` | crate `tree-sitter-bash` 0.25.1 | `5ad30bb1a260c76df5397490b8bd97e272e62ed9c99be36fa53da959d7667e7f` | `f0270796c32f9acfc180e6068d6bef6614b983107b679a247fb63b200c6346e7` | [`bash.LICENSE`](bash.LICENSE) |
| `cmake.wasm.gz` | `uyha/tree-sitter-cmake` v0.7.5 with [`scanner-state.patch`](../../../../rust/vendor/tree-sitter-cmake/scanner-state.patch) (vendored in `rust/vendor/tree-sitter-cmake`) | `ece92c2e3fbf15634fb90fab91d0af749348c069d27208eeb10dcdd4e77fe5a2` | `5925d5f08a726ebb9024ccae1e12165a56c35630c53d9dd7fc3eeb87434849c2` | [`cmake.LICENSE`](cmake.LICENSE) |
| `cpp.wasm.gz` | crate `tree-sitter-cpp` 0.23.4 | `2a35a43b4af6c9f7b69624ac00c2c50808912591450dc79c05dea03ac1bae814` | `f5ff3ba7af054af846f37dfd2525bdfede95530d5747a8fe30df87d070ece607` | [`cpp.LICENSE`](cpp.LICENSE) |
| `csharp.wasm.gz` | crate `tree-sitter-c-sharp` 0.23.5 | `0a2651e49de7c7237c535c41a132a7ec0424da79d12f197f1df3edd7d6ea4427` | `a4b053f6fdedfc9c94876b415441ea953c0046ef83d45e0cbc4d9b8938ae6cbc` | [`csharp.LICENSE`](csharp.LICENSE) |
| `css.wasm.gz` | crate `tree-sitter-css` 0.25.0 | `2e5150071220012ee635ac9e2119f4f3a93e0b51a7a3b69ab0cd87c18cf5e51e` | `f0fa4c3171f3322560e3a56445989f2da9d77c38bae61e9f07ae0b3e01d88b36` | [`css.LICENSE`](css.LICENSE) |
| `dart.wasm.gz` | crate `tree-sitter-dart` 0.2.0 | `1894ef57f8e0ceb483ccf6d548429517dc42b0d983293ebdb5ff9ff9c612c4a9` | `e700b38561a3f1e641340fac8232dfca493dbc024d142d338a5075feca3e9efc` | [`dart.LICENSE`](dart.LICENSE) |
| `dtd.wasm.gz` | crate `tree-sitter-xml` 0.7.0 (`dtd`) | `79aa52e71ba9685115e2fc2742a3a30f91987f96fcf5ffbb7e34827155a6b932` | `2534944fea9484174b791535509c09a25fcd1c4099da9392056b009311dcfe81` | [`dtd.LICENSE`](dtd.LICENSE) |
| `elixir.wasm.gz` | crate `tree-sitter-elixir` 0.3.5 | `d1c2000b477e873e44e485f55809e3af7e2bfce83fa4d58cfa3a9f83660e7d79` | `33f71c86131d02eab80841e4142005ccaa54bcc06bcc3a7e8dc6a9a8a0ca5e94` | [`elixir.LICENSE`](elixir.LICENSE) |
| `elm.wasm.gz` | crate `tree-sitter-elm` 5.9.4 | `7c993bdabb63c6c6424fc0b433073c9000446a0e3de189d8a2ab93b33636369d` | `30e2708c30704223fa0351428d4e81aef32acd9121a7ea35606ddc9fa45bc319` | [`elm.LICENSE`](elm.LICENSE) |
| `erlang.wasm.gz` | crate `tree-sitter-erlang` 0.20.0 | `c8c83443d536875c2f28f12ae4f0242f477cac82115073f4d9506c2bb188dbcc` | `345a2693219c24236afdbacf3af1082aaa59eef6c21c06371845690505ee3af3` | [`erlang.LICENSE`](erlang.LICENSE) |
| `go.wasm.gz` | crate `tree-sitter-go` 0.25.0 | `3dbf6ed1238b5dfcf2be4d2f2d4cb27a14d34f34d7784eccccbfd532fd4a6d85` | `058a4f6c7cb7156c599c2ca07af9645f03b3f655b84bced9c72816e755c7030b` | [`go.LICENSE`](go.LICENSE) |
| `graphql.wasm.gz` | crate `tree-sitter-graphql` 0.3.0 | `8b7ebff40dc5b802df9ec03b1570fd53d234300ac4705899643ae61dda77dc57` | `a9239652fcc467ec3bf9dbe7d5271ff3511c40c7dc9bba440667599dcfb608bc` | [`graphql.LICENSE`](graphql.LICENSE) |
| `groovy.wasm.gz` | crate `tree-sitter-groovy` 0.1.2 | `786637c026add3b2112aa717fa17b34e3093ea291b3b46d60438ba84382fdb06` | `9c7236d8ab8c127a59644321213e479ab8ac0f53ac5720c1461ef406e46db4a6` | [`groovy.LICENSE`](groovy.LICENSE) |
| `haskell.wasm.gz` | crate `tree-sitter-haskell` 0.24.1 | `a136b31118c6767f3a9fbbb245adc559370eabd914b72b90438395542327a089` | `ea89cb1199b1ca2102e273b27239e795baff17248128a9bcd3c3825ac17d8cc0` | [`haskell.LICENSE`](haskell.LICENSE) |
| `hcl.wasm.gz` | crate `tree-sitter-hcl` 1.1.0 | `0c821e2979f54f8f460201eafe9464d5d3dae0076c5bdd51567a798fb7add543` | `c7fa4ff3cecbd8c9eba1b9643b511a5b226b47d29eb5ec38656a5bc7a216bfc5` | [`hcl.LICENSE`](hcl.LICENSE) |
| `html.wasm.gz` | crate `tree-sitter-html` 0.23.2 | `65768172733b3bbe461cbdc14ea928f00fbfcc51d8ac68f0a5c72071c6a0bbf1` | `11960fcc4fe3a01930de6bbae9b2c5c4d0a6c16012a5d20cc2069fa5b78d3bbd` | [`html.LICENSE`](html.LICENSE) |
| `java.wasm.gz` | crate `tree-sitter-java` 0.23.5 | `4add5150cf4531eb5dd97f3343dcf65cd11704c84711348b328582b83424a0e4` | `6476728734128931bd50de2d1e5e9c75c506b51e278f54eaf709836a0de35759` | [`java.LICENSE`](java.LICENSE) |
| `kotlin.wasm.gz` | crate `tree-sitter-kotlin-ng` 1.1.0 | `9ff65161845b9e9c9d62c12e9a4e4b8d8628bdc31c681ec7e6b4bd3bd6444cb3` | `5d44eef5c02b4546f01ddc376ed237294b18604f727b192a832a5ce08a040e8f` | [`kotlin.LICENSE`](kotlin.LICENSE) |
| `lean.wasm.gz` | `wvhulle/tree-sitter-lean` bd942cd2795016239be02b3b3d5ef635645ddd38 (vendored in `rust/vendor/tree-sitter-lean`) | `d04d21e8ad0132f9f3410296817a4317fc11d4f8cbd35c62dc3fe0b72f4f9a6b` | `7b58425afbe7f59e90fa9f01ebafe9241cdedbf1fe7e2daca4bec2f15122cc59` | [`lean.LICENSE`](lean.LICENSE) |
| `lua.wasm.gz` | crate `tree-sitter-lua` 0.5.0 | `933206d96a78f7785c13b2600182f1527dcd755c200b1271bb5bc4d8da4b17b3` | `43db0f3e64ffd4ed1ada7a77e4489e54afa5afd7b5aa013679d8284d4ea0f074` | [`lua.LICENSE`](lua.LICENSE) |
| `make.wasm.gz` | crate `tree-sitter-make` 1.1.1 | `659bb2a3712bc6ecf77a96c82de2bc3062141f405c96d52582b2e01bd8722152` | `3cd0a9967c477e0cf3330d96ae79e2bbc8580119daf7e02266649fa6cd136e76` | [`make.LICENSE`](make.LICENSE) |
| `markdown.wasm.gz` | crate `tree-sitter-md-025` 0.5.6 (`tree-sitter-markdown`) | `0f51c93b5d79d08bc74088d91a9aafb7e40ad7e2f169164865c01d392277bb07` | `479b27880bce5dc9501d8cfa7f427eb2f5ffcb143ef5863fa02296da02c830e9` | [`markdown.LICENSE`](markdown.LICENSE) |
| `markdown_inline.wasm.gz` | crate `tree-sitter-md-025` 0.5.6 (`tree-sitter-markdown-inline`) | `0c6aaec9097b10118b34601047c3b4d3d565defe48e95d133f714b592795fd2c` | `3e1db657850e83e19e2769157c24eac7b590d8bc869c2753201417c40298b388` | [`markdown_inline.LICENSE`](markdown_inline.LICENSE) |
| `matlab.wasm.gz` | crate `tree-sitter-matlab` 1.3.1 | `d13d9cfa1cead80c000e21ed37833df31d910fe531a5cb11165a850c384a4960` | `2fd8f9f251c59a0242a8899d6725a8edc47ff477371497ba2025f9ee8cc8a57c` | [`matlab.LICENSE`](matlab.LICENSE) |
| `nix.wasm.gz` | crate `tree-sitter-nix` 0.3.0 | `2b1d7945441b83e324cd225663481d35e51cd9ae84189cc7e69f58a11ecb85ad` | `98faa2e223fc3fe88c20c02c51fd739da8ebe1f1591b131d0964ede6cf320b08` | [`nix.LICENSE`](nix.LICENSE) |
| `ocaml.wasm.gz` | crate `tree-sitter-ocaml` 0.26.0 (`grammars/ocaml`) | `f76cc46f96f9ed152354743539cc642dcb6a67343795dd9d548fc4d0b0ebce74` | `bfa3eef92b9d3cc1117aa486d46ecb06c98a3a28c1747660aaa5851df5e4530c` | [`ocaml.LICENSE`](ocaml.LICENSE) |
| `ocaml_interface.wasm.gz` | crate `tree-sitter-ocaml` 0.26.0 (`grammars/interface`) | `62634bd351247bd2152eb15438f5bea3f0a1ccce4291f318aa86a79d29bf111e` | `3ce03850c36618e3530292372aae9156a45b5a923210a87c285539d7f13b0354` | [`ocaml_interface.LICENSE`](ocaml_interface.LICENSE) |
| `odin.wasm.gz` | crate `tree-sitter-odin` 1.3.0 | `8f672d8d022d70eeae822796924f2e45e692d6b2acd2ddc79bfeb5f696fa24f0` | `8ce37ea03c3d565d1c4e922d357a82fd9aa5184c36f712509803a63fec8d3194` | [`odin.LICENSE`](odin.LICENSE) |
| `pascal.wasm.gz` | crate `tree-sitter-pascal` 0.10.2 | `d7a5f5c04880fac79e0eb1841cbbc00d7e305863efe56d83c9754fba70385329` | `e1ae02b62ce27ba86d202e77fa42f5e59d8e945b00ff64803cc275741ecca628` | [`pascal.LICENSE`](pascal.LICENSE) |
| `perl.wasm.gz` | crate `ts-parser-perl` 2.0.0 | `1317bdaae2f50f5f82b6dbc7176425e9cee64a0dbfbb82aa17959aaefc54f199` | `af4e99e2379e9ae0228357a15f0d3b6037846b8a328deafb70dfa7a13b58b140` | [`perl.LICENSE`](perl.LICENSE) |
| `php.wasm.gz` | crate `tree-sitter-php` 0.24.2 (`php`) | `59ad8e5e4fde3fe60687a488ab8420612840cc966b83739af1b3a4317ed27ec6` | `a100e693f0133e5f559a5d90daf4ba9c12607162108c80368f2dfcbac0ec8bff` | [`php.LICENSE`](php.LICENSE) |
| `powershell.wasm.gz` | crate `tree-sitter-powershell` 0.26.4 | `f39f67abaae4c488ccea5d9bbd003ef0b00c303779e0491950886a12a9305720` | `84555e38f85a391095b349a906b9790d319e3946ce12d9bc21dbec5cded5dad9` | [`powershell.LICENSE`](powershell.LICENSE) |
| `proto.wasm.gz` | crate `tree-sitter-proto` 0.6.0 | `ac481450c32e8fa52976075581701ecc63790fbab4c9c4f10d0901c9a39b61eb` | `2179ef77376c7c1a1c08c145915c8902436d6f08724cb055ce597879f107bf61` | [`proto.LICENSE`](proto.LICENSE) |
| `python.wasm.gz` | crate `tree-sitter-python` 0.25.0 | `a895f10b3cf7b2608f3283b43cd5cfed70971c7ee4a0136abbaaccbc4a7a25e0` | `3523e4dc3d12894bbb2f7ea4b86b50eec69aed321c3d9b49271d82ed9f5f79b0` | [`python.LICENSE`](python.LICENSE) |
| `r.wasm.gz` | crate `tree-sitter-r` 1.3.0 | `43ec2413de8aec823c76e6994991fe07d9877e019ab2e1892534a76ce81a0771` | `8443d229942c1719c40fac5729e8d9252e48950aa2e0dbcf86c52bd38e875209` | [`r.LICENSE`](r.LICENSE) |
| `regex.wasm.gz` | crate `tree-sitter-regex` 0.25.0 | `ddf28eb5ad0dd0898b9ed0f2593852edac639a339a410d9e93350e472859700f` | `0b64ac3dc55cfb73a95da3ea3eb1ac89e0f8fafd6050aa32139988b8413c14f1` | [`regex.LICENSE`](regex.LICENSE) |
| `rocq.wasm.gz` | `aruzdh/tree-sitter-rocq` 300fe33fc299c30f736fd56d8ef8a28b08acd4e6 with [`meta-language.patch`](../../../../rust/vendor/tree-sitter-rocq/meta-language.patch) (vendored in `rust/vendor/tree-sitter-rocq`) | `4f0807b15d47e4335c28ccb7be5eb8d01851baaab4843dfa24b2d8fb1966ecd2` | `9d0d3172500d14ee0d06d61b0d3a3a33e757003a94eb7f1c7221b11c740e0530` | [`rocq.LICENSE`](rocq.LICENSE) |
| `ruby.wasm.gz` | crate `tree-sitter-ruby` 0.23.1 | `4ce468358b6f4e25a35c8cf6bc0eaf60665bc22d602f8c939323c2347255cd15` | `2a06cd2fa165a2913f6598b92c9d3d3132ecfbf3fa7ffa8ca93b31efb2bfb58d` | [`ruby.LICENSE`](ruby.LICENSE) |
| `scala.wasm.gz` | crate `tree-sitter-scala` 0.26.2 | `9f6d03fa6c63d2d855b6f9e0368046a58568587eefc38b5e38bdb001e8ec68db` | `c2d92c43913a24efa76528c6aff75c97c0964798c4d360c7dc1951b04058fe52` | [`scala.LICENSE`](scala.LICENSE) |
| `solidity.wasm.gz` | crate `tree-sitter-solidity` 1.2.13 | `e71971f6ddc0704e81f4af3f43ecc79793742bb5f1d256a5fdd718aa06d74acb` | `7ae2d8aff2ec119ff8706667f45f5ff0d4edea901acef86e227664de83c6df51` | [`solidity.LICENSE`](solidity.LICENSE) |
| `sql.wasm.gz` | crate `tree-sitter-sequel` 0.3.11 | `852e088fb8470952cdb2a1b78c1c58626c7d91562b26baa4672d51f9754bf580` | `e03fb5079e88cfe0807056c284fd2b0589cee35df9bbb3d07e72f2f4178de8de` | [`sql.LICENSE`](sql.LICENSE) |
| `swift.wasm.gz` | crate `tree-sitter-swift` 0.7.4 with [`tree-sitter-swift-wasm.patch`](../../../../js/scripts/grammar-patches/tree-sitter-swift-wasm.patch) | `1d332e60ec28e6b398db36e06bf05414061dfbbd3e981b33ff1edfd5159e3e08` | `5b54286c25842ea5c60a659f4f3276af7419d877f8946ef4b8d26b86849f1708` | [`swift.LICENSE`](swift.LICENSE) |
| `toml.wasm.gz` | crate `tree-sitter-toml-ng` 0.7.0 | `1991a2608e6f0214e563fe5f762e8df72954dc69a3dd9a1e22ea8a870e4052c3` | `057f48e81072cb0eb5969632a7cf032fc6d33cd4ba3198c3f58227346c012d97` | [`toml.LICENSE`](toml.LICENSE) |
| `vb.wasm.gz` | crate `tree-sitter-vb-dotnet` 0.1.0 | `7e9e6275f9b83c1f6c79112fd2021effc8529e9d6b794bbd41e39d99db1214d8` | `4bc68c0fb1cbc42a44eb3199affe54173b6519f52ffdd173e31c9dc87b9099f8` | [`vb.LICENSE`](vb.LICENSE) |
| `xml.wasm.gz` | crate `tree-sitter-xml` 0.7.0 (`xml`) | `e41811d97cfd672a902924a3d99bdf696d6fcec08188a35404e4a3d2698ab844` | `2edd39d0ef194dc70878fe99b277180547fabf252f85b58e537db5a11e3027fa` | [`xml.LICENSE`](xml.LICENSE) |
| `yaml.wasm.gz` | crate `tree-sitter-yaml` 0.7.2 | `8a3baaab33fb63cf9a89f97ec61dbb3ab0d4ef69be9f0f229092c79d129617c9` | `f89cf2a8ccd4f29292e502f1c37efb4f1dead281246605e0a890c697d4db88c7` | [`yaml.LICENSE`](yaml.LICENSE) |
| `zig.wasm.gz` | crate `tree-sitter-zig` 1.1.2 | `5449f98eb876939fcb12be76891ecb0c99b78be3bfe843e140d64c680b66ae63` | `cc95fa4d69178617b19fe50b070dc871e315aea55107fcd1bcd238ae77811edd` | [`zig.LICENSE`](zig.LICENSE) |

Every grammar is distributed under its upstream license (MIT unless the
linked license file states otherwise).
