// Source-only construction; never parse or execute represented fixture bodies.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {draftTranslationProfileInventory} from './draft-translation-profile-inventory.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
for(const path of ['js/src/translation/rust-array-lookup-rules.js','js/src/translation/rust-array-lookups.js',
 'js/scripts/draft-rust-array-lookup-lazy.mjs','js/scripts/draft-rust-array-lookup-lazy-fixtures.mjs',
 'js/tests/translation-rust-array-lookup-lazy.test.js','experiments/rust-records/array-lookup-lazy.mjs'])parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);
await import('./draft-rust-array-lookup-rules.mjs');globalThis.gc?.();
await import('./draft-rust-array-lookup-lazy-fixtures.mjs');globalThis.gc?.();
draftTranslationProfileInventory({id:'rust-array-lookup-lazy',fixture:'rust-array-lookup-lazy',programAsset:'rust-array-lookup-program',sourceLanguage:'Rust',experimentPath:'experiments/rust-records/array-lookup-lazy.mjs',nativeTest:'translation_rust_array_lookup_lazy',
 javascript:['js/src/translation/rust-array-lookup-rules.js','js/src/translation/rust-array-lookups.js','js/tests/translation-rust-array-lookup-lazy.test.js'],
 rust:['rust/src/translation/rust_array_lookup_rules.rs','rust/src/translation/rust/expressions/array_lookups.rs','rust/tests/unit/translation_rust_array_lookup_lazy.rs']});
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
