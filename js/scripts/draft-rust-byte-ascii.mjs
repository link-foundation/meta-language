// Bounded source author only; never execute represented behavior or tests.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
for(const path of ['js/scripts/stage-source-library-files.mjs','js/src/translation/rust.js','js/src/translation/rust-byte-ascii-rules.js','js/src/translation/rust-byte-ascii.js','js/scripts/draft-rust-byte-ascii-rules.mjs','js/scripts/draft-rust-byte-ascii-fixtures.mjs','js/scripts/draft-rust-byte-ascii-inventory.mjs','js/scripts/draft-rust-byte-ascii.mjs','js/tests/translation-rust-byte-ascii.test.js','experiments/rust-records/byte-ascii.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
for(const path of ['draft-rust-byte-ascii-rules.mjs','draft-rust-byte-ascii-fixtures.mjs','draft-rust-byte-ascii-inventory.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
