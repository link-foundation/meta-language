// Source authoring only: syntax, static translation and literal fixture writing.
// Do not import future controls or invoke any represented class/source program.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
for(const path of ['js/src/translation/javascript-class-source-rules.js','js/src/translation/javascript-classes.js','js/src/translation/javascript.js','js/src/self-translation.js',
 'js/scripts/draft-javascript-class-source-rules.mjs','js/scripts/draft-javascript-classes-fixtures.mjs','js/scripts/draft-javascript-classes-inventory.mjs','js/scripts/draft-javascript-classes.mjs',
 'js/tests/translation-javascript-classes.test.js','experiments/javascript-classes/oracle.mjs']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
for(const path of ['draft-javascript-class-source-rules.mjs','draft-javascript-classes-fixtures.mjs','draft-javascript-classes-inventory.mjs']){
 await import(new URL(path,import.meta.url));globalThis.gc?.();
}
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
