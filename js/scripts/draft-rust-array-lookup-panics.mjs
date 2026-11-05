// Complete source authoring only; represented fixture source never executes.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {draftTranslationProfileInventory} from './draft-translation-profile-inventory.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/rust-panic-value-rules.js','js/src/translation/abrupt-result-rules.js','js/src/translation/check.js','js/src/translation/output.js','js/src/translation/emit-javascript.js','js/src/translation/emit-rust.js','js/src/translation/rust-array-lookup-rules.js','js/src/translation/rust-array-lookups.js','js/scripts/draft-rust-array-lookup-panics.mjs','js/scripts/draft-rust-array-lookup-panic-fixtures.mjs','js/tests/translation-rust-array-lookup-panics.test.js','experiments/rust-records/array-lookup-panics.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
await import('./draft-rust-panic-value-rules.mjs');globalThis.gc?.();
await import('./draft-abrupt-result-rules.mjs');globalThis.gc?.();
await import('./draft-rust-array-lookup-rules.mjs');globalThis.gc?.();
await import('./draft-rust-array-lookup-panic-fixtures.mjs');globalThis.gc?.();
draftTranslationProfileInventory({id:'ordinary-rust-copied-array-lookup-panics',fixture:'rust-array-lookup-panics',programAsset:'rust-array-lookup-program',sourceLanguage:'Rust',experimentPath:'experiments/rust-records/array-lookup-panics.mjs',nativeTest:'translation_rust_array_lookup_panics',javascript:['js/src/translation/rust-panic-value-rules.js','js/src/translation/abrupt-result-rules.js','js/src/translation/check.js','js/src/translation/output.js','js/src/translation/emit-rust.js','js/src/translation/emit-javascript.js','js/src/translation/rust-array-lookup-rules.js','js/src/translation/rust-array-lookups.js','js/tests/translation-rust-array-lookup-panics.test.js'],rust:['rust/src/translation/rust_panic_value_rules.rs','rust/src/translation/abrupt_result_rules.rs','rust/src/translation/check/expressions.rs','rust/src/translation/output.rs','rust/src/translation/output/settle.rs','rust/src/translation/emit_rust/math.rs','rust/src/translation/emit_javascript/math.rs','rust/src/translation/rust_array_lookup_rules.rs','rust/src/translation/rust/expressions/array_lookups.rs','rust/tests/unit/translation_rust_array_lookup_panics.rs']});
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
