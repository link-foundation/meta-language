// Source/metadata authorship only: never parse represented project inputs.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/rust-source-project.js','js/src/rust-source-project-cli.js','js/scripts/draft-rust-source-project-context.mjs','js/scripts/draft-rust-source-project-fixtures.mjs','js/tests/rust-source-project.test.js','js/tests/translation-rust-source-versions.test.js']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
await import('./draft-rust-source-project-fixtures.mjs');globalThis.gc?.();
await import('./draft-rust-source-project-inventory.mjs');globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
