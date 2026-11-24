// Source-only batch: no represented definition, fixture or product executes.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
for(const path of ['js/src/translation/rust.js','js/src/translation/rust-array-lookup-rules.js','js/src/translation/rust-array-lookups.js','js/scripts/draft-rust-array-lookup-rules.mjs','js/scripts/draft-rust-array-lookup-fixtures.mjs','js/scripts/draft-rust-array-lookup-inventory.mjs','js/scripts/draft-rust-array-lookups.mjs','js/tests/translation-rust-array-lookup.test.js','experiments/rust-records/array-lookup.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
for(const path of ['draft-rust-array-lookup-rules.mjs','draft-rust-array-lookup-fixtures.mjs','draft-rust-array-lookup-inventory.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
