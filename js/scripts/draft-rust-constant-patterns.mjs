// Source-only authors: never execute represented checked arithmetic or controls.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {draftTranslationProfileInventory} from './draft-translation-profile-inventory.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
const javascript=['js/src/translation/rust-constant-integer-rules.js','js/src/translation/rust-constant-patterns.js','js/src/translation/rust-primitive-integer-constants.js','js/src/translation/rust-type-aliases.js','js/src/translation/rust-integer-ranges.js','js/src/translation/rust.js','js/tests/translation-rust-constant-patterns.test.js'];
for(const path of [...javascript,'js/scripts/draft-rust-constant-integer-rules.mjs','js/scripts/draft-rust-constant-patterns.mjs','js/scripts/draft-rust-constant-patterns-fixtures.mjs','experiments/rust-records/constant-patterns.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
for(const file of ['draft-rust-constant-integer-rules.mjs','draft-rust-constant-patterns-fixtures.mjs']){await import(new URL(file,import.meta.url));globalThis.gc?.();}
draftTranslationProfileInventory({id:'ordinary-rust-primitive-constant-patterns',fixture:'rust-constant-patterns',programAsset:'rust-constant-integer-program',sourceLanguage:'Rust',experimentPath:'experiments/rust-records/constant-patterns.mjs',nativeTest:'translation_rust_constant_patterns',javascript,rust:['rust/src/translation/rust_constant_integer_rules.rs','rust/src/translation/rust/expressions/primitive_integer_constants.rs','rust/src/translation/rust/type_aliases.rs','rust/src/translation/rust/expressions/control/constant_patterns.rs','rust/src/translation/rust/expressions/control/integer_ranges.rs','rust/tests/unit/translation_rust_constant_patterns.rs']});
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
