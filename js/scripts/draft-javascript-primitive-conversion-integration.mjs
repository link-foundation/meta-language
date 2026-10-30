// Source construction and literal metadata only; no controls are imported.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
const mode=process.argv.slice(2).join(' '),reuse=mode==='--draft --reuse-native';
if(mode!=='--draft'&&!reuse)throw new TypeError('use --draft or --draft --reuse-native');
process.argv.splice(2,process.argv.length-2,'--draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/javascript-primitive-conversion.js','js/scripts/draft-javascript-primitive-conversion.mjs',
 'js/scripts/draft-javascript-primitive-conversion-codecs.mjs','js/scripts/draft-javascript-primitive-conversion-spellings.mjs',
 'js/scripts/draft-javascript-primitive-conversion-fixtures.mjs','js/scripts/draft-javascript-primitive-conversion-integration.mjs',
 'experiments/javascript-properties/heap-controls.mjs','experiments/javascript-properties/primitive-conversion-controls.mjs',
 'experiments/javascript-properties/primitive-conversion-oracle.mjs','js/tests/translation-javascript-primitive-conversion.test.js']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
for(const path of [reuse?'draft-javascript-primitive-conversion-codecs.mjs':'draft-javascript-primitive-conversion.mjs','draft-javascript-primitive-conversion-fixtures.mjs']){
 await import(new URL(path,import.meta.url));globalThis.gc?.();
}
const file=new URL('parity/language-features.lino',root),inventory=readProjectData(file);
const common=['parity/fixtures/javascript-primitive-conversion.lino','parity/translation/javascript-primitive-conversion-spellings.lino',
 'experiments/javascript-properties/primitive-conversion-controls.mjs','experiments/javascript-properties/primitive-conversion-oracle.mjs'];
const row={id:'javascript-primitive-conversion',required:true,
 javascript:{status:'implemented',evidence:['js/src/translation/javascript-primitive-conversion.js','js/src/translation/javascript-primitive-conversion-spellings.js','js/tests/translation-javascript-primitive-conversion.test.js',...common]},
 rust:{status:'implemented',evidence:['rust/src/translation/javascript_primitive_conversion.rs','rust/tests/unit/translation_javascript_primitive_conversion.rs','rust/tests/unit/javascript_primitive_conversion_codecs.rs',...common]}};
const at=inventory.features.findIndex(entry=>entry.id===row.id);if(at<0)inventory.features.push(row);else inventory.features[at]=row;writeProjectData(file,inventory);
for(const [path,declaration]of [['rust/src/translation/mod.rs','pub(crate) mod javascript_primitive_conversion;'],['rust/tests/unit/mod.rs','mod translation_javascript_primitive_conversion;']]){
 const file=new URL(path,root),source=readFileSync(file,'utf8');if(!source.includes(declaration))writeFileSync(file,source+'\n'+declaration+'\n');
}
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
