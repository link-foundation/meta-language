// Source and literal construction only. Oracles and represented controls never
// execute here; their complete authored modules are parsed without evaluation.
import {readFileSync,writeFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {readProjectData,writeProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const javascript=['js/src/program-representation.js','js/src/program-binding-extents.js','js/src/program-parameter-bindings.js','js/src/translation/source-parameter-binding-rules.js','js/tests/source-parameter-bindings.test.js','experiments/source-parameter-bindings/oracle.mjs'];
for(const path of [...javascript,'js/scripts/draft-source-parameter-bindings.mjs','js/scripts/draft-source-parameter-binding-rules.mjs','js/scripts/draft-source-parameter-binding-fixtures.mjs'])parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);
await import('./draft-source-parameter-binding-rules.mjs');globalThis.gc?.();
await import('./draft-source-parameter-binding-fixtures.mjs');globalThis.gc?.();
const file=new URL('parity/language-features.lino',root),inventory=readProjectData(file),id='source-parameter-bindings',fixture='parity/fixtures/source-parameter-bindings.lino';
const row={id,required:true,javascript:{status:'implemented',evidence:[...javascript,fixture]},rust:{status:'implemented',evidence:['rust/src/translation/source_parameter_binding_rules.rs','rust/src/program_representation/analysis.rs','rust/src/program_representation/analysis/javascript_hoist.rs','rust/src/program_representation/analysis/parameters.rs','rust/src/program_representation/analysis/declarations.rs','rust/src/program_representation/analysis/extents.rs','rust/src/program_representation/analysis/resolution.rs','rust/tests/unit/source_parameter_bindings.rs','experiments/source-parameter-bindings/oracle.mjs',fixture]}};
const index=inventory.features.findIndex(prior=>prior.id===id);if(index<0)inventory.features.push(row);else inventory.features[index]=row;writeProjectData(file,inventory);
const modulesFile=new URL('rust/tests/unit/mod.rs',root),modules=readFileSync(modulesFile,'utf8'),declaration='mod source_parameter_bindings;';
if(!modules.includes(declaration))writeFileSync(modulesFile,modules+'\n'+declaration+'\n');
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
