// Author production definitions and literal controls; never execute inputs.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {draftTranslationProfileInventory} from './draft-translation-profile-inventory.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/rust-panic-value-rules.js','js/src/translation/rust-panic-values.js','js/src/translation/rust.js','js/src/translation/rust-ordinary-completions.js','js/scripts/draft-rust-formatted-panic-fixtures.mjs','js/scripts/draft-rust-formatted-panics.mjs','js/scripts/generate-rust-completion-panic-fixtures.mjs','js/tests/translation-rust-completion-panic.test.js','js/tests/translation-rust-formatted-panics.test.js','experiments/rust-records/formatted-panics.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
await import('./draft-rust-panic-value-rules.mjs');globalThis.gc?.();
await import('./draft-abrupt-result-rules.mjs');globalThis.gc?.();
await import('./draft-rust-formatted-panic-fixtures.mjs');globalThis.gc?.();
draftTranslationProfileInventory({id:'ordinary-rust-formatted-panic-macros',fixture:'rust-formatted-panics',programAsset:'rust-panic-value-program',sourceLanguage:'Rust',experimentPath:'experiments/rust-records/formatted-panics.mjs',nativeTest:'translation_rust_formatted_panics',javascript:['js/src/translation/rust-panic-value-rules.js','js/src/translation/rust-panic-values.js','js/src/translation/rust.js','js/src/translation/rust-ordinary-completions.js','js/tests/translation-rust-formatted-panics.test.js','js/tests/translation-rust-completion-panic.test.js'],rust:['rust/src/translation/rust_panic_value_rules.rs','rust/src/translation/rust/expressions.rs','rust/src/translation/rust/expressions/control.rs','rust/src/translation/rust/ordinary_completions.rs','rust/src/translation/rust/items.rs','rust/src/translation/rust/formatting.rs','rust/tests/unit/translation_rust_formatted_panics.rs','rust/tests/unit/translation_rust_completions.rs']});
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
