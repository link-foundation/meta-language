// Source construction only. Never import controls or parse their originals.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
for(const path of ['js/src/translation/rust-byte-string-expression-rules.js','js/src/translation/rust-byte-string-expressions.js','js/src/translation/rust.js','js/src/translation/rust-source-origins.js','js/src/translation/rust-array-index.js','js/src/translation/rust-array-lookups.js','js/src/translation/rust-scalar-vector-copy.js','js/tests/translation-rust-byte-string-literals.test.js','js/tests/translation-rust-byte-string-expressions.test.js','experiments/rust-records/byte-string-expressions.mjs','js/scripts/draft-rust-byte-string-expression-rules.mjs','js/scripts/draft-rust-byte-string-expression-fixtures.mjs','js/scripts/draft-rust-byte-string-expression-inventory.mjs','js/scripts/draft-rust-byte-string-expressions.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
for(const path of ['draft-rust-byte-string-expression-rules.mjs','draft-rust-byte-string-expression-fixtures.mjs','draft-rust-byte-string-expression-inventory.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
