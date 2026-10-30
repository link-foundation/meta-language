// Source-only authors: never execute represented checked arithmetic or controls.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {draftTranslationProfileInventory} from './draft-translation-profile-inventory.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
const javascript=['js/src/translation/rust-nominal-copy-rules.js','js/src/translation/rust-nominal-copies.js','js/src/translation/rust-source-origins.js','js/src/translation/rust-records.js','js/src/translation/rust.js','js/tests/translation-rust-nominal-copies.test.js'];
for(const path of [...javascript,'js/scripts/draft-rust-nominal-copy-rules.mjs','js/scripts/draft-rust-nominal-copies.mjs','js/scripts/draft-rust-nominal-copies-fixtures.mjs','experiments/rust-records/nominal-copies.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
for(const file of ['draft-rust-nominal-copy-rules.mjs','draft-rust-nominal-copies-fixtures.mjs']){await import(new URL(file,import.meta.url));globalThis.gc?.();}
draftTranslationProfileInventory({id:'ordinary-rust-nominal-copy-declarations',fixture:'rust-nominal-copies',programAsset:'rust-nominal-copy-program',sourceLanguage:'Rust',experimentPath:'experiments/rust-records/nominal-copies.mjs',nativeTest:'translation_rust_nominal_copies',javascript,rust:['rust/src/translation/rust_nominal_copy_rules.rs','rust/src/translation/rust/source_origins.rs','rust/src/translation/rust/source_origins/nominal_copies.rs','rust/src/translation/rust/items.rs','rust/src/translation/rust/records.rs','rust/tests/unit/translation_rust_nominal_copies.rs']});
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
