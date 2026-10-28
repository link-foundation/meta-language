// Source/model/literal-data construction only; controls are parsed, never run.
import {readFileSync,writeFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/javascript-call-code.js','js/src/translation/javascript-call-primitives.js','js/src/translation/javascript-call-machine.js','js/src/translation/javascript-call-source.js','js/src/translation/javascript-call-lowering.js',
 'js/scripts/draft-javascript-call-source.mjs','js/scripts/draft-javascript-call-source-fixtures.mjs','js/scripts/draft-javascript-call-source-integration.mjs',
 'experiments/javascript-properties/call-source-controls.mjs','experiments/javascript-properties/call-source-oracle.mjs','js/tests/translation-javascript-call-source.test.js']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
for(const path of ['draft-javascript-call-source.mjs','draft-javascript-call-machine.mjs','draft-javascript-call-source-fixtures.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
for(const path of ['experiments/javascript-properties/call-source-codecs.mjs','experiments/javascript-properties/call-machine-codecs.mjs'])parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);
for(const [path,declarations]of [['rust/src/translation/mod.rs',['pub(crate) mod javascript_call_source;']],['rust/tests/unit/mod.rs',['mod translation_javascript_call_source;','mod translation_javascript_call_source_machine;']]]){
 const file=new URL(path,root);let source=readFileSync(file,'utf8');for(const declaration of declarations)if(!source.includes(declaration))source+='\n'+declaration+'\n';writeFileSync(file,source);
}
const file=new URL('parity/language-features.lino',root),inventory=readProjectData(file),common=['parity/fixtures/javascript-call-source.lino','experiments/javascript-properties/call-source-controls.mjs','experiments/javascript-properties/call-source-oracle.mjs'];
const row={id:'javascript-call-source',required:true,javascript:{status:'implemented',evidence:['js/src/translation/javascript-call-source.js','js/src/translation/javascript-call-lowering.js','js/tests/translation-javascript-call-source.test.js',...common]},
 rust:{status:'implemented',evidence:['rust/src/translation/javascript_call_source.rs','rust/tests/unit/translation_javascript_call_source.rs','rust/tests/unit/translation_javascript_call_source_machine.rs',...common]}};
const at=inventory.features.findIndex(entry=>entry.id===row.id);if(at<0)inventory.features.push(row);else inventory.features[at]=row;writeProjectData(file,inventory);
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
