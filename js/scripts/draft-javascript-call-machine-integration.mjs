// Actual source/model-transport and literal data construction only.
// V8 parses future controls without linking or executing any module.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/javascript-call-machine.js','js/src/typed-program-links.js','js/scripts/draft-source-library.mjs','js/scripts/source-library-entry-points.mjs',
 'js/scripts/draft-native-model-codecs.mjs','js/scripts/draft-javascript-model-codecs.mjs','js/scripts/draft-javascript-call-machine.mjs','js/scripts/draft-javascript-call-machine-fixtures.mjs','js/scripts/draft-javascript-call-machine-integration.mjs',
 'js/scripts/draft-typed-program-sharing-fixtures.mjs','experiments/javascript-properties/call-machine-controls.mjs','experiments/javascript-properties/call-machine-oracle.mjs',
 'experiments/javascript-properties/typed-sharing-controls.mjs','experiments/javascript-properties/typed-sharing-oracle.mjs','js/tests/typed-program-value-sharing.test.js','js/tests/translation-javascript-call-machine.test.js']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
for(const path of ['draft-javascript-call-machine.mjs','draft-javascript-call-machine-fixtures.mjs','draft-typed-program-sharing-fixtures.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
parseAuthoredJavaScriptModule(readFileSync(new URL('experiments/javascript-properties/call-machine-codecs.mjs',root),'utf8'),'experiments/javascript-properties/call-machine-codecs.mjs');
const file=new URL('parity/language-features.lino',root),inventory=readProjectData(file);
const shared=['parity/fixtures/javascript-call-machine.lino','experiments/javascript-properties/call-machine-oracle.mjs','experiments/javascript-properties/call-machine-controls.mjs','experiments/javascript-properties/call-machine-codecs.mjs'];
const rows=[{id:'javascript-call-machine',required:true,javascript:{status:'implemented',evidence:['js/src/translation/javascript-call-machine.js','js/tests/translation-javascript-call-machine.test.js',...shared]},
 rust:{status:'implemented',evidence:['rust/src/translation/javascript_call_machine.rs','rust/tests/unit/translation_javascript_call_machine.rs','rust/tests/unit/javascript_call_machine_codecs.rs',...shared]}},
 {id:'typed-program-value-sharing',required:true,javascript:{status:'implemented',evidence:['js/src/typed-program-links.js','js/src/data/typed-program-value-sharing.lino','js/tests/typed-program-value-sharing.test.js','parity/fixtures/typed-program-value-sharing.lino','experiments/javascript-properties/typed-sharing-oracle.mjs']},
 rust:{status:'implemented',evidence:['rust/src/typed_program_links/codec_encoding.rs','rust/src/data/typed-program-value-sharing.lino','rust/tests/unit/typed_program_value_sharing.rs','parity/fixtures/typed-program-value-sharing.lino','experiments/javascript-properties/typed-sharing-oracle.mjs']}}];
for(const row of rows){const at=inventory.features.findIndex(entry=>entry.id===row.id);if(at<0)inventory.features.push(row);else inventory.features[at]=row;}writeProjectData(file,inventory);
for(const [path,declarations]of [['rust/src/translation/mod.rs',['pub(crate) mod javascript_call_machine;']],['rust/tests/unit/mod.rs',['mod translation_javascript_call_machine;','mod typed_program_value_sharing;']]]){
 const file=new URL(path,root);let source=readFileSync(file,'utf8');for(const declaration of declarations)if(!source.includes(declaration))source+='\n'+declaration+'\n';writeFileSync(file,source);
}
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
