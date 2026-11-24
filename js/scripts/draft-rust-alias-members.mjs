// Source-only production and fixture metadata authorship, never execution.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/rust.js','js/src/translation/rust-read-only-queries.js','js/src/translation/rust-type-alias-rules.js','js/src/translation/rust-type-aliases.js','js/src/translation/rust-primitive-try-from.js','js/src/translation/rust-primitive-associated-calls.js','js/scripts/draft-rust-type-alias-fixtures.mjs','js/scripts/draft-rust-type-alias-inventory.mjs','js/scripts/draft-rust-alias-members.mjs','js/tests/translation-rust-type-aliases.test.js']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
for(const file of ['draft-rust-type-alias-rules.mjs','draft-rust-type-alias-fixtures.mjs','draft-rust-type-alias-inventory.mjs']){await import(new URL(file,import.meta.url));globalThis.gc?.();}
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
