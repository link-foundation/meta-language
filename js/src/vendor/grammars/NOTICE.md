# Vendored tree-sitter grammars

These WebAssembly grammars are compiled from exactly the generated parsers the
Rust crate links (`rust/Cargo.lock` crates or `rust/vendor`), so both
runtimes parse with the same grammar revision. They are rebuilt by
`node js/scripts/build-vendored-grammars.mjs` with tree-sitter CLI
0.25.10 and verified against `grammar-lock.json` by
`--check`. Each file is a zero-mtime gzip of the `.wasm` whose SHA-256 is
listed; the parser digest is of the generated `src/parser.c` (after the
listed patch, if any).

| File | Source | parser.c SHA-256 | wasm SHA-256 | License |
| --- | --- | --- | --- | --- |
| `c.wasm.gz` | crate `tree-sitter-c` 0.24.2 | `f2883ff9b21f4a5bd5553c1b10366c418947d17a7bc6bf256124a7542f079dd2` | `fa5c7f64735914f32849b44ef5ff0e459a43f5e5d28ccdc3f11785a2043948b4` | [`c.LICENSE`](c.LICENSE) |
| `cpp.wasm.gz` | crate `tree-sitter-cpp` 0.23.4 | `2a35a43b4af6c9f7b69624ac00c2c50808912591450dc79c05dea03ac1bae814` | `84cd0ee5427a58e49791e157c50a9e7020766faad9922dfbc98c1efc8e1040d6` | [`cpp.LICENSE`](cpp.LICENSE) |
| `csharp.wasm.gz` | crate `tree-sitter-c-sharp` 0.23.5 | `0a2651e49de7c7237c535c41a132a7ec0424da79d12f197f1df3edd7d6ea4427` | `f265130d20fe62d3dc0cec756ee5a4159bd9825894a48fc0f7e72c8bd30ef099` | [`csharp.LICENSE`](csharp.LICENSE) |
| `css.wasm.gz` | crate `tree-sitter-css` 0.25.0 | `2e5150071220012ee635ac9e2119f4f3a93e0b51a7a3b69ab0cd87c18cf5e51e` | `c2b8c4f2f69454a60a7562d6b21150b4879fc26e9a69d4674b9d7c11df838b62` | [`css.LICENSE`](css.LICENSE) |
| `csv.wasm.gz` | `tree-sitter-grammars/tree-sitter-csv` f6bf6e35eb0b95fbadea4bb39cb9709507fcb181 with [`rfc4180-quotes.patch`](../../../../rust/vendor/tree-sitter-csv/rfc4180-quotes.patch) (vendored in `rust/vendor/tree-sitter-csv`) | `eacf58622b8d50ab75acf91172d35e314874327d98d1bd03acfd1ab789ddb1d1` | `19e23890eef87611c6a8c71575ec7d1b4150ba90c946f17ea818ecf329a5322b` | [`csv.LICENSE`](csv.LICENSE) |
| `dtd.wasm.gz` | crate `tree-sitter-xml` 0.7.0 (`dtd`) | `79aa52e71ba9685115e2fc2742a3a30f91987f96fcf5ffbb7e34827155a6b932` | `e74dac556b2ef4ad7b10a721d7c171731f176b649c26123b01d5099658fa0cb4` | [`dtd.LICENSE`](dtd.LICENSE) |
| `go.wasm.gz` | crate `tree-sitter-go` 0.25.0 | `3dbf6ed1238b5dfcf2be4d2f2d4cb27a14d34f34d7784eccccbfd532fd4a6d85` | `62de6d1fe0c811f194a97672fde719c1a9ca84a7f69685e88ee0883b7fcad238` | [`go.LICENSE`](go.LICENSE) |
| `graphql.wasm.gz` | crate `tree-sitter-graphql` 0.1.0 | `d27f89090319f885811fefb5c5453f9cc95543c04df02b7c43aa1fa6f7d353d9` | `fd4081c84308f5db14a4a6e1bea91c08f9c38e5ab47e5a0730c4a39448844d00` | [`graphql.LICENSE`](graphql.LICENSE) |
| `html.wasm.gz` | crate `tree-sitter-html` 0.23.2 | `65768172733b3bbe461cbdc14ea928f00fbfcc51d8ac68f0a5c72071c6a0bbf1` | `6b997d3b7754edd84eea05dc6e53f06a94112a5d97b90d90af820afb065ac05d` | [`html.LICENSE`](html.LICENSE) |
| `ini.wasm.gz` | crate `tree-sitter-ini` 1.4.0 | `3537bf540af5b3def849c4f15d4ba8a02b37f5414849140f1c139d2abb1e4123` | `4884a74e0e60d02fc897b1f780c044e578ff839c9eeb6f4742d2feaef9839ece` | [`ini.LICENSE`](ini.LICENSE) |
| `java.wasm.gz` | crate `tree-sitter-java` 0.23.5 | `4add5150cf4531eb5dd97f3343dcf65cd11704c84711348b328582b83424a0e4` | `a2f2689856a2aceeb50c23bb4bda86daf872abdcb75ecbf33e71e6f3fb2c014f` | [`java.LICENSE`](java.LICENSE) |
| `javascript.wasm.gz` | crate `tree-sitter-javascript` 0.25.0 | `67209ca7ef6e1a4f74e29e48b5928455f892fe1821a3960fbcd62f4e972f7384` | `35070cda936e479202e22c72cf129d6894eee7f4de8b41912a516cfdbe109204` | [`javascript.LICENSE`](javascript.LICENSE) |
| `json.wasm.gz` | crate `tree-sitter-json` 0.24.8 | `e8e1ff5df0d73e3b82574129724e68ef4fa0faf1b8c43dd3f5c1a84839f830ab` | `6862a0aac3af8bed434d4031afbe330895a277cc80e80733ea67b6dee01d4500` | [`json.LICENSE`](json.LICENSE) |
| `json5.wasm.gz` | crate `tree-sitter-json5-orchard` 0.1.0 | `0024a5353393c7186a9a4f531f451279d301c7c3925ba292348176a83d3765b4` | `f1ccbd6f0eb0b07b233094e3779daa8fe7c111658f23a491b6cb957b997c6e11` | [`json5.LICENSE`](json5.LICENSE) |
| `kotlin.wasm.gz` | crate `tree-sitter-kotlin-ng` 1.1.0 | `9ff65161845b9e9c9d62c12e9a4e4b8d8628bdc31c681ec7e6b4bd3bd6444cb3` | `35470989975963d68d2f60e6ae7180a12807bf9136a12e1cdcf23da9d0882f38` | [`kotlin.LICENSE`](kotlin.LICENSE) |
| `lean.wasm.gz` | crate `tree-sitter-lean4` 0.3.0 | `d04d21e8ad0132f9f3410296817a4317fc11d4f8cbd35c62dc3fe0b72f4f9a6b` | `899cf73abfb11fcc72cf82f6b39c6edbd30e26a8b80185401f88ce36da0b8ff2` | [`lean.LICENSE`](lean.LICENSE) |
| `lua.wasm.gz` | crate `tree-sitter-lua` 0.2.0 | `fa3a2efe5803017c536d4727fe408b9eaf3e33b86662a5b1a1d12227d0022231` | `79f8f2dc2e924aafb21183c04a3edd9bf529bd4ee8da207d50960a0562be1f47` | [`lua.LICENSE`](lua.LICENSE) |
| `markdown.wasm.gz` | crate `tree-sitter-md-025` 0.5.6 (`tree-sitter-markdown`) | `0f51c93b5d79d08bc74088d91a9aafb7e40ad7e2f169164865c01d392277bb07` | `6d129b4d3d4f4fd7fa88f2243db845d53d65dcd2521614ce94209e6c45a05b6a` | [`markdown.LICENSE`](markdown.LICENSE) |
| `pascal.wasm.gz` | crate `tree-sitter-pascal` 0.10.2 | `d7a5f5c04880fac79e0eb1841cbbc00d7e305863efe56d83c9754fba70385329` | `9f04719399fff078431f3fd6fa77fbb8a5b3c9681edfd2d97c4dfe5ad86a5d35` | [`pascal.LICENSE`](pascal.LICENSE) |
| `perl.wasm.gz` | crate `ts-parser-perl` 1.1.2 | `38ebca6dd96edeb01da421780001ea26b572133e83525c993bbeb78e8e9a24bc` | `ae40bd28ba389d92fde2f8b4eb024581e038395f639df1e7ec6be4ddbe379e46` | [`perl.LICENSE`](perl.LICENSE) |
| `php.wasm.gz` | crate `tree-sitter-php` 0.24.2 (`php`) | `59ad8e5e4fde3fe60687a488ab8420612840cc966b83739af1b3a4317ed27ec6` | `507649c167779621b46bfcfab2bd88744883a92ed52991bd1bdc35e2af1f935e` | [`php.LICENSE`](php.LICENSE) |
| `proto.wasm.gz` | crate `tree-sitter-proto` 0.4.0 | `ecdea8962be9f1879e067ee23127abd79ec8d3fad83799eb9052ffa48ec78448` | `0b226f4132ac4493642b8df1fbeb7541c92e36e3fb152064ef662d072135b795` | [`proto.LICENSE`](proto.LICENSE) |
| `python.wasm.gz` | crate `tree-sitter-python` 0.25.0 | `a895f10b3cf7b2608f3283b43cd5cfed70971c7ee4a0136abbaaccbc4a7a25e0` | `a43153418b186cd2c170ba6e848d37e7f8abfcaab275f632a64504020f491cbc` | [`python.LICENSE`](python.LICENSE) |
| `r.wasm.gz` | crate `tree-sitter-r` 1.2.0 | `622165734714c6e81f70d99d522d78cf98915fdf2b1032f8bb33e775a0b971c0` | `b492695b6767dfd1216ceed4efa3e62977b1ac3261899eb405a88aa4a1352e8e` | [`r.LICENSE`](r.LICENSE) |
| `rocq.wasm.gz` | `aruzdh/tree-sitter-rocq` 300fe33fc299c30f736fd56d8ef8a28b08acd4e6 (vendored in `rust/vendor/tree-sitter-rocq`) | `fc714e6d0dbecff5264dbea99768d1b44474d1de69726f6ce4cc2121125d8c9c` | `37566727cced22a9cab2f65bf09e174de262822639519f7177afe8bc627034b8` | [`rocq.LICENSE`](rocq.LICENSE) |
| `ruby.wasm.gz` | crate `tree-sitter-ruby` 0.23.1 | `4ce468358b6f4e25a35c8cf6bc0eaf60665bc22d602f8c939323c2347255cd15` | `e77f88559b47c0fe2771821c6c160cdf6bbfe17afcd642ab3fdda9aebb70b2f3` | [`ruby.LICENSE`](ruby.LICENSE) |
| `rust.wasm.gz` | crate `tree-sitter-rust` 0.24.2 | `9602518f9e57919910bf0e777e52f6bfc9325d4c182e998bdb4efd5682b76e4a` | `bad860425fafa0321d6898b6cbaeee1f571f75fa7584bf726c3ec04f3b19c474` | [`rust.LICENSE`](rust.LICENSE) |
| `scala.wasm.gz` | crate `tree-sitter-scala` 0.25.1 | `049ef02b206b3ff06b5066040d01ad04161bc1afc2bd26073c9a865082af9d1e` | `4f099f94793e54287875a3e8bc041a548733bb592f426ce3837c3d2a76afb1ca` | [`scala.LICENSE`](scala.LICENSE) |
| `sql.wasm.gz` | crate `tree-sitter-sequel` 0.3.11 | `852e088fb8470952cdb2a1b78c1c58626c7d91562b26baa4672d51f9754bf580` | `f9d5638965f6f7ecc6b6979b8717f44e756687a1cbe4fef411b84506bdb81451` | [`sql.LICENSE`](sql.LICENSE) |
| `swift.wasm.gz` | crate `tree-sitter-swift` 0.7.3 | `d3edff6effe31b9a507f496577407987343b101b23eb7bee7a9b050e8ab5d27a` | `92a6aa39712255e7332d1b06db52bc6d49e9f563215846978c0ca63b6cc38359` | [`swift.LICENSE`](swift.LICENSE) |
| `toml.wasm.gz` | crate `tree-sitter-toml-ng` 0.7.0 | `1991a2608e6f0214e563fe5f762e8df72954dc69a3dd9a1e22ea8a870e4052c3` | `1e1a98653af3577c723f93bb4063dcee96ef11f7be861094b60d8bec50dd5b78` | [`toml.LICENSE`](toml.LICENSE) |
| `tsx.wasm.gz` | crate `tree-sitter-typescript` 0.23.2 (`tsx`) | `1902cb53fa7ff5179df89b2eea863165e84c8cc866226419dc26921d8c055885` | `772bd591654c4d7c7c4427652138af9f118212738cadbbfef96adf2275736d35` | [`tsx.LICENSE`](tsx.LICENSE) |
| `typescript.wasm.gz` | crate `tree-sitter-typescript` 0.23.2 (`typescript`) | `74fe453edd70f4eae9af0a1050cbd7943d8971d59165b6aaebbaa0a0b716d1aa` | `427ea5f2c24c98989a53ee3520df03afa1f9ade5bc21362de17b9982ac374a88` | [`typescript.LICENSE`](typescript.LICENSE) |
| `vb.wasm.gz` | crate `tree-sitter-vb-dotnet` 0.1.0 | `7e9e6275f9b83c1f6c79112fd2021effc8529e9d6b794bbd41e39d99db1214d8` | `f8bcf03f07a8d957f26ae728d8a0cb5e90240f095c44e5174ecea15c38059e85` | [`vb.LICENSE`](vb.LICENSE) |
| `xml.wasm.gz` | crate `tree-sitter-xml` 0.7.0 (`xml`) | `e41811d97cfd672a902924a3d99bdf696d6fcec08188a35404e4a3d2698ab844` | `fafa7327c7a09ba74612e23543ff4e93ee08b785f0a8c2f8292c03f0b65c4917` | [`xml.LICENSE`](xml.LICENSE) |
| `yaml.wasm.gz` | crate `tree-sitter-yaml` 0.7.2 | `8a3baaab33fb63cf9a89f97ec61dbb3ab0d4ef69be9f0f229092c79d129617c9` | `e93163e5d306e4b328ab0b4c29ce82b086057e1d2dad47bcdcac358abfdec518` | [`yaml.LICENSE`](yaml.LICENSE) |

Every grammar is distributed under its upstream license (MIT unless the
linked license file states otherwise).
