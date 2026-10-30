// Source integration only. Deferred test modules and represented programs never run.
import {readFileSync,writeFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {readProjectData,writeProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const javascript=['js/src/project-translation-rules.js','js/src/project-translation.js','js/scripts/draft-project-translation.mjs','js/tests/project-dependency-gaps.test.js','js/tests/source-module-requests.test.js'];
for(const path of [...javascript,'js/scripts/generate-project-translation-rules.mjs','js/scripts/draft-project-dependency-gap-fixtures.mjs','js/scripts/draft-project-dependency-gaps.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
await import('./generate-project-translation-rules.mjs');globalThis.gc?.();await import('./draft-project-dependency-gap-fixtures.mjs');globalThis.gc?.();
const file=new URL('parity/language-features.lino',root),inventory=readProjectData(file),id='project-dependency-gaps',fixture='parity/fixtures/project-dependency-gaps.lino';
const row={id,required:true,javascript:{status:'implemented',evidence:[...javascript,fixture]},rust:{status:'implemented',evidence:['rust/src/project_translation.rs','rust/src/project_translation_rules.rs','rust/tests/unit/project_dependency_gaps.rs',fixture]}};
const index=inventory.features.findIndex(row=>row.id===id);if(index<0)inventory.features.push(row);else inventory.features[index]=row;writeProjectData(file,inventory);
const path=new URL('rust/tests/unit/mod.rs',root),modules=readFileSync(path,'utf8');if(!modules.includes('mod project_dependency_gaps;'))writeFileSync(path,modules+'\nmod project_dependency_gaps;\n');
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
