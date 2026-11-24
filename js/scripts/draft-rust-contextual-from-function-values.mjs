// Bounded source author only: never execute fixtures, behavior or tests.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
for(const path of ['js/src/translation/rust-primitive-function-value-rules.js','js/src/translation/rust-primitive-function-values.js','js/src/translation/rust-copy-closures.js','js/scripts/draft-primitive-function-value-corrections.mjs','js/scripts/draft-rust-primitive-from-fixtures.mjs','js/scripts/draft-rust-contextual-from-function-value-fixtures.mjs','js/scripts/draft-rust-contextual-from-function-value-inventory.mjs','js/scripts/draft-rust-contextual-from-function-values.mjs','js/tests/translation-rust-contextual-from-function-value.test.js','experiments/rust-records/contextual-from-function-value.mjs']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
for(const path of ['draft-rust-primitive-function-value-rules.mjs','draft-rust-contextual-from-function-value-fixtures.mjs','draft-rust-contextual-from-function-value-inventory.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
