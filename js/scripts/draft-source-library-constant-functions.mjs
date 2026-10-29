// Source construction only. Embedded fixture/assertion strings never execute.
import {readFileSync,writeFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {readProjectData,writeProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const javascript=['js/src/translation/javascript.js','js/src/program-representation.js','js/src/translation/source-library-binding-rules.js','js/src/translation/source-library-export-rules.js','js/scripts/read-source-library-closure.mjs','js/scripts/compose-source-library-aliases.mjs','js/tests/source-library-constant-functions.test.js'];
for(const path of [...javascript,'js/scripts/draft-source-library-binding-rules.mjs','js/scripts/draft-source-library-constant-fixtures.mjs','js/scripts/draft-source-library-constant-functions.mjs'])parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);
await import('./draft-source-library-binding-rules.mjs');globalThis.gc?.();
await import('./draft-source-library-constant-fixtures.mjs');globalThis.gc?.();
const file=new URL('parity/language-features.lino',root),inventory=readProjectData(file),id='source-library-constant-functions';
const common=['parity/fixtures/source-library-constant-functions.lino'];
const row={id,required:true,javascript:{status:'implemented',evidence:[...javascript,...common]},rust:{status:'implemented',evidence:['rust/src/translation/source_library_binding_rules.rs','rust/src/translation/source_library_export_rules.rs','rust/src/program_representation/analysis.rs','rust/src/program_representation/analysis/declarations.rs','rust/tests/unit/source_library_bindings.rs',...common]}};
const index=inventory.features.findIndex(prior=>prior.id===id);if(index<0)inventory.features.push(row);else inventory.features[index]=row;writeProjectData(file,inventory);
const modulesFile=new URL('rust/tests/unit/mod.rs',root),modules=readFileSync(modulesFile,'utf8'),declaration='mod source_library_bindings;';
if(!modules.includes(declaration))writeFileSync(modulesFile,modules+'\n'+declaration+'\n');
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
