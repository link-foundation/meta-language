// Construct source and literal assets only. Never import deferred controls.
import {readFileSync,writeFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {readProjectData,writeProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url);
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const javascript=['js/src/project-translation-archive-rules.js','js/src/project-translation-archive.js','js/src/project-translation-archive-cli.js','js/src/cli.js','js/tests/project-translation-archive.test.js','js/scripts/draft-project-translation.mjs','js/tests/source-module-requests.test.js'];
for(const path of [...javascript,'js/scripts/draft-project-translation-archive.mjs','js/scripts/draft-project-translation-archive-rules.mjs','js/scripts/draft-project-translation-archive-fixtures.mjs']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
await import('./draft-project-translation-archive-rules.mjs');globalThis.gc?.();await import('./draft-project-translation-archive-fixtures.mjs');globalThis.gc?.();
const file=new URL('parity/language-features.lino',root),inventory=readProjectData(file),id='project-translation-complete-source-archives',fixture='parity/fixtures/project-translation-archive.lino';
const row={id,required:true,javascript:{status:'implemented',evidence:[...javascript,fixture]},rust:{status:'implemented',evidence:['rust/src/project_translation_archive.rs','rust/src/project_translation_archive_rules.rs','rust/src/main.rs','rust/tests/unit/project_translation_archive.rs',fixture]}};
const index=inventory.features.findIndex(row=>row.id===id);if(index<0)inventory.features.push(row);else inventory.features[index]=row;writeProjectData(file,inventory);
const path=new URL('rust/tests/unit/mod.rs',root),modules=readFileSync(path,'utf8');if(!modules.includes('mod project_translation_archive;'))writeFileSync(path,modules+'\nmod project_translation_archive;\n');
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
