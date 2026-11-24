// Source construction, literal fixtures and inventory writing only. Never
// import a future control, semantic observation or represented source program.
import {readFileSync,writeFileSync,existsSync,unlinkSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
const paths=['js/src/translation/javascript-data-field-rules.js','js/src/translation/javascript-data-fields.js','js/src/translation/javascript-semantic-values.js','js/src/translation/javascript-property-descriptors.js',
 'js/scripts/draft-javascript-data-field-rules.mjs','js/scripts/draft-javascript-data-fields-fixtures.mjs','js/scripts/draft-javascript-property-descriptors.mjs','js/scripts/draft-javascript-property-fixtures.mjs','js/scripts/draft-javascript-properties.mjs',
 'js/tests/translation-javascript-data-fields.test.js','js/tests/translation-javascript-property-descriptors.test.js','experiments/javascript-properties/controls.mjs','experiments/javascript-properties/oracle.mjs','experiments/javascript-properties/data-field-oracle.mjs'];
for(const path of paths){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
for(const path of ['draft-javascript-data-field-rules.mjs','draft-javascript-property-descriptors.mjs','draft-javascript-data-fields-fixtures.mjs','draft-javascript-property-fixtures.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
// This untracked child was produced by this same author before the final
// two-part source layout. It contains no independent source or user changes.
const stale=new URL('rust/src/translation/javascript_property_descriptors/part_3.rs',root);
if(existsSync(stale)){
 const text=readFileSync(stale,'utf8');
 if(!text.startsWith('// Generated from complete js/src/translation/javascript-property-descriptors.js. Do not edit by hand.\n'))throw new TypeError('obsolete child does not carry this task source identity');
 const {execFileSync}=await import('node:child_process');
 const tracked=execFileSync('git',['ls-files','--','rust/src/translation/javascript_property_descriptors/part_3.rs'],{cwd:root,encoding:'utf8'});
 if(tracked)throw new TypeError('obsolete source child is tracked; preserve its history explicitly');
 unlinkSync(stale);
}
const inventoryFile=new URL('parity/language-features.lino',root),inventory=readProjectData(inventoryFile);
for(const [id,asset,files]of [
 ['javascript-data-fields','javascript-data-field-program',['js/src/translation/javascript-data-field-rules.js','js/src/translation/javascript-data-fields.js','js/tests/translation-javascript-data-fields.test.js','rust/src/translation/check/javascript_data_fields.rs','rust/tests/unit/translation_javascript_data_fields.rs']],
 ['javascript-property-descriptors','javascript-property-descriptor-program',['js/src/translation/javascript-semantic-values.js','js/src/translation/javascript-property-descriptors.js','js/tests/translation-javascript-property-descriptors.test.js','rust/src/translation/javascript_property_descriptors.rs','rust/tests/unit/translation_javascript_property_descriptors.rs']]
]){
 const common=['parity/fixtures/'+id+'.lino',...(id==='javascript-data-fields'?['experiments/javascript-properties/data-field-oracle.mjs']:['experiments/javascript-properties/controls.mjs','experiments/javascript-properties/oracle.mjs']),'js/src/data/'+asset+'.lino','rust/src/data/'+asset+'.lino'];
 const row={id,required:true,javascript:{status:'implemented',evidence:[...files.filter(path=>path.startsWith('js/')),...common]},rust:{status:'implemented',evidence:[...files.filter(path=>path.startsWith('rust/')),...common]}};
 const at=inventory.features.findIndex(prior=>prior.id===id);if(at<0)inventory.features.push(row);else inventory.features[at]=row;
}
writeProjectData(inventoryFile,inventory);
const modulesFile=new URL('rust/tests/unit/mod.rs',root);let modules=readFileSync(modulesFile,'utf8');
for(const name of ['translation_javascript_data_fields','translation_javascript_property_descriptors'])if(!modules.includes('mod '+name+';'))modules+='\nmod '+name+';\n';writeFileSync(modulesFile,modules);
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
