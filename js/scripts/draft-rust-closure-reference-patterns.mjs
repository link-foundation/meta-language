// Source-only authors: never execute represented checked arithmetic or controls.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {draftTranslationProfileInventory} from './draft-translation-profile-inventory.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
const javascript=['js/src/translation/rust-copy-closures.js','js/src/translation/rust-copy-closure-rules.js','js/src/translation/rust-copy-closure-parameter-rules.js','js/src/translation/rust-copy-closure-parameter-origins.js','js/src/translation/rust-iterator-pipelines.js','js/src/translation/rust-iterator-pipeline-rules.js','js/tests/translation-rust-closure-reference-patterns.test.js'];
for(const path of [...javascript,'js/scripts/draft-rust-closure-reference-patterns.mjs','js/scripts/draft-rust-closure-reference-patterns-fixtures.mjs','experiments/rust-records/closure-reference-patterns.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
for(const file of ['draft-rust-copy-closure-rules.mjs','draft-rust-contextual-closure-rules.mjs','draft-rust-closure-reference-patterns-fixtures.mjs']){await import(new URL(file,import.meta.url));globalThis.gc?.();}
draftTranslationProfileInventory({id:'ordinary-rust-closure-reference-patterns',fixture:'rust-closure-reference-patterns',programAsset:'rust-copy-closure-program',sourceLanguage:'Rust',experimentPath:'experiments/rust-records/closure-reference-patterns.mjs',nativeTest:'translation_rust_closure_reference_patterns',javascript,rust:['rust/src/translation/rust/copy_closures.rs','rust/src/translation/rust_copy_closure_parameter_rules.rs','rust/src/translation/rust_copy_closure_rules.rs','rust/src/translation/rust/expressions/iterator_pipelines.rs','rust/tests/unit/translation_rust_closure_reference_patterns.rs']});
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
