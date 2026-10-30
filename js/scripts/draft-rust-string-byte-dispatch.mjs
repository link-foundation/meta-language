// Actual production-module syntax and package source metadata authoring only.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/rust.js','js/src/translation/rust-string-byte-views.js','js/scripts/draft-rust-string-byte-dispatch.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
