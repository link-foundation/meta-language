// Source construction only. Never import oracle/tests or parse case sources.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
for(const path of ['js/src/translation/rust.js','js/src/translation/rust-source-origins.js','js/src/translation/check.js','js/src/translation/javascript-infer.js','js/src/translation/rust-numeric-pattern-rules.js','js/scripts/draft-rust-numeric-pattern-rules.mjs','js/scripts/draft-rust-numeric-pattern-fixtures.mjs','js/scripts/draft-rust-numeric-pattern-inventory.mjs','js/scripts/draft-rust-numeric-patterns.mjs','js/tests/translation-rust-numeric-patterns.test.js','experiments/rust-records/numeric-patterns.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
for(const path of ['draft-rust-numeric-pattern-rules.mjs','draft-rust-numeric-pattern-fixtures.mjs','draft-rust-numeric-pattern-inventory.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
