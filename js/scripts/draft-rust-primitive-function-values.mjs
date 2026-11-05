// One bounded, source-only batch. No represented behavior or code tests run.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
for(const path of ['js/src/translation/rust.js','js/src/translation/rust-primitive-function-value-rules.js','js/src/translation/rust-primitive-function-values.js','js/src/translation/rust-primitive-associated-calls.js','js/scripts/draft-primitive-function-value-corrections.mjs','js/scripts/draft-rust-primitive-associated-call-fixtures.mjs','js/scripts/draft-rust-primitive-default-fixtures.mjs','js/scripts/draft-rust-primitive-function-value-rules.mjs','js/scripts/draft-rust-primitive-function-value-fixtures.mjs','js/scripts/draft-rust-primitive-function-value-inventory.mjs','js/scripts/draft-rust-primitive-function-values.mjs','js/tests/helpers/rust-callable-profile.js','js/tests/translation-rust-callable-owner.test.js','js/tests/translation-rust-primitive-function-value.test.js','experiments/rust-records/callable-profile.mjs','experiments/rust-records/callable-owner.mjs','experiments/rust-records/primitive-function-value.mjs']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
for(const path of ['draft-rust-primitive-function-value-rules.mjs','draft-rust-primitive-function-value-fixtures.mjs','draft-rust-primitive-function-value-inventory.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
