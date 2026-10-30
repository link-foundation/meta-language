// Source-only authors: never execute represented checked arithmetic or controls.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {draftTranslationProfileInventory} from './draft-translation-profile-inventory.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
const javascript=['js/src/translation/rust-pattern-match-rules.js','js/src/translation/rust-pattern-matches.js','js/src/translation/rust-literal-matches.js','js/src/translation/rust.js','js/tests/translation-rust-pattern-matches.test.js','js/tests/translation-rust-literal-matches.test.js'];
for(const path of [...javascript,'js/scripts/draft-rust-pattern-matches.mjs','js/scripts/draft-rust-pattern-matches-fixtures.mjs','experiments/rust-records/pattern-matches.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
for(const file of ['draft-rust-pattern-match-rules.mjs','draft-rust-pattern-matches-fixtures.mjs']){await import(new URL(file,import.meta.url));globalThis.gc?.();}
draftTranslationProfileInventory({id:'ordinary-rust-pattern-matches',fixture:'rust-pattern-matches',programAsset:'rust-pattern-match-program',sourceLanguage:'Rust',experimentPath:'experiments/rust-records/pattern-matches.mjs',nativeTest:'translation_rust_pattern_matches',javascript,rust:['rust/src/translation/rust_pattern_match_rules.rs','rust/src/translation/rust/expressions/control.rs','rust/src/translation/rust/expressions/control/literal_matches.rs','rust/src/translation/rust/expressions/control/pattern_matches.rs','rust/tests/unit/translation_rust_pattern_matches.rs']});
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
