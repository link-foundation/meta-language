// Actual source/type/format generation plus literal fixture/inventory writing.
// No future control, modeled descriptor, accessor or evidence observation runs.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
const mode=process.argv.slice(2).join(' '),reuse=mode==='--draft --reuse-native';
if(mode!=='--draft'&&!reuse)throw new TypeError('use --draft or --draft --reuse-native');
process.argv.splice(2,process.argv.length-2,'--draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/javascript-descriptor-conversion.js','js/scripts/draft-native-model-codecs.mjs','js/scripts/draft-javascript-descriptor-field-rules.mjs',
 'js/scripts/draft-javascript-descriptor-conversion.mjs','js/scripts/draft-javascript-descriptor-conversion-codecs.mjs','js/scripts/draft-javascript-descriptor-conversion-fixtures.mjs','js/scripts/draft-javascript-descriptor-conversion-integration.mjs',
 'experiments/javascript-properties/heap-controls.mjs','experiments/javascript-properties/descriptor-conversion-controls.mjs','experiments/javascript-properties/descriptor-conversion-oracle.mjs','js/tests/translation-javascript-descriptor-conversion.test.js']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
for(const path of [reuse?'draft-javascript-descriptor-conversion-codecs.mjs':'draft-javascript-descriptor-conversion.mjs','draft-javascript-descriptor-conversion-fixtures.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
const file=new URL('parity/language-features.lino',root),inventory=readProjectData(file);
const common=['parity/fixtures/javascript-descriptor-conversion.lino','experiments/javascript-properties/descriptor-conversion-controls.mjs','experiments/javascript-properties/descriptor-conversion-oracle.mjs','parity/translation/javascript-descriptor-fields.lino','parity/translation/native-model-wire-rules.lino'];
const row={id:'javascript-descriptor-conversion',required:true,javascript:{status:'implemented',evidence:['js/src/translation/javascript-descriptor-conversion.js','js/src/translation/javascript-descriptor-field-rules.js','js/tests/translation-javascript-descriptor-conversion.test.js',...common]},
 rust:{status:'implemented',evidence:['rust/src/translation/javascript_descriptor_conversion.rs','rust/tests/unit/translation_javascript_descriptor_conversion.rs','rust/tests/unit/javascript_descriptor_conversion_codecs.rs',...common]}};
const at=inventory.features.findIndex(entry=>entry.id===row.id);if(at<0)inventory.features.push(row);else inventory.features[at]=row;writeProjectData(file,inventory);
for(const [path,declaration]of [['rust/src/translation/mod.rs','pub(crate) mod javascript_descriptor_conversion;'],['rust/tests/unit/mod.rs','mod translation_javascript_descriptor_conversion;']]){
 const file=new URL(path,root),source=readFileSync(file,'utf8');if(!source.includes(declaration))writeFileSync(file,source+'\n'+declaration+'\n');
}
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
