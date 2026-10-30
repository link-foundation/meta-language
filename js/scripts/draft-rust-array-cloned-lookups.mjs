// Writing-only source batch; no behavior, fixtures, products or tests execute.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
for(const path of ['js/scripts/draft-callable-type-assertions.mjs','js/src/translation/rust-array-lookup-rules.js','js/src/translation/rust-array-lookups.js','js/src/translation/rust-array-lookup-source-owners.js','js/scripts/draft-rust-array-cloned-lookup-fixtures.mjs','js/scripts/draft-rust-array-cloned-lookup-inventory.mjs','js/scripts/draft-rust-array-cloned-lookups.mjs','js/tests/translation-rust-array-cloned-lookup.test.js','experiments/rust-records/array-cloned-lookup.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
for(const path of ['draft-rust-array-lookup-rules.mjs','draft-rust-array-cloned-lookup-fixtures.mjs','draft-rust-array-cloned-lookup-inventory.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
