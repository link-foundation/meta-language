// Construct actual source programs and literal controls; never execute controls.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/rust-copy-closure-parameter-rules.js','js/src/translation/rust-copy-closure-parameter-origins.js','js/src/translation/rust-copy-closures.js','js/scripts/draft-rust-copy-closure-parameter-rules.mjs','js/src/translation/rust-iterator-reduction-rules.js','js/src/translation/rust-array-element-owner-rules.js','js/src/translation/rust-iterator-reductions.js','js/src/translation/rust.js','js/tests/translation-rust-iterator-reductions.test.js','experiments/rust-records/iterator-reductions.mjs','js/scripts/draft-rust-iterator-reduction-rules.mjs','js/scripts/draft-rust-array-element-owner-rules.mjs','js/scripts/draft-rust-iterator-reduction-fixtures.mjs','js/scripts/draft-rust-iterator-reduction-inventory.mjs','js/scripts/draft-rust-iterator-reductions.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
for(const file of ['draft-rust-copy-closure-parameter-rules.mjs','draft-rust-array-element-owner-rules.mjs','draft-rust-iterator-reduction-rules.mjs','draft-rust-iterator-reduction-fixtures.mjs','draft-rust-iterator-reduction-inventory.mjs']){await import(new URL(file,import.meta.url));globalThis.gc?.();}
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
