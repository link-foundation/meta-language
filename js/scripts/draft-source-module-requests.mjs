// Source/literal construction only. Tests and represented cases never execute.
import {readFileSync,writeFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {readProjectData,writeProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const javascript=['js/src/source-module-requests.js','js/src/source-module-request-cli.js','js/src/cli.js','js/src/translation/source-module-request-rules.js','js/scripts/draft-project-translation.mjs','js/tests/source-module-requests.test.js'];
for(const path of [...javascript,'js/scripts/draft-source-module-requests.mjs','js/scripts/draft-source-module-request-rules.mjs','js/scripts/draft-source-module-request-fixtures.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
await import('./draft-source-module-request-rules.mjs');globalThis.gc?.();await import('./draft-source-module-request-fixtures.mjs');globalThis.gc?.();
const file=new URL('parity/language-features.lino',root),inventory=readProjectData(file),id='source-module-requests',fixture='parity/fixtures/source-module-requests.lino';
const row={id,required:true,javascript:{status:'implemented',evidence:[...javascript,fixture]},rust:{status:'implemented',evidence:['rust/src/source_module_requests.rs','rust/src/translation/source_module_request_rules.rs','rust/tests/unit/source_module_requests.rs',fixture]}};
const index=inventory.features.findIndex(row=>row.id===id);if(index<0)inventory.features.push(row);else inventory.features[index]=row;writeProjectData(file,inventory);
const path=new URL('rust/tests/unit/mod.rs',root),modules=readFileSync(path,'utf8');if(!modules.includes('mod source_module_requests;'))writeFileSync(path,modules+'\nmod source_module_requests;\n');
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
