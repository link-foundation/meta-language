// Source/control/metadata authorship only. Inspected programs never execute.
import {readFileSync,writeFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/source-construct-signal-rules.js','js/scripts/draft-source-construct-signal-rules.mjs','js/scripts/draft-source-construct-signal-fixtures.mjs','js/scripts/draft-own-source-construct-inventory.mjs','js/scripts/draft-own-source-constructs.mjs','js/tests/source-construct-signals.test.js','js/tests/own-source-construct-inventory.test.js']){parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();}
for(const file of ['draft-source-construct-signal-rules.mjs','draft-source-construct-signal-fixtures.mjs']){await import(new URL(file,import.meta.url));globalThis.gc?.();}
const featureFile=new URL('parity/language-features.lino',root),inventory=readProjectData(featureFile),id='literal-source-construct-candidate-statistics';
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/translation/source-construct-signal-rules.js','js/scripts/draft-own-source-construct-inventory.mjs','js/tests/source-construct-signals.test.js','parity/fixtures/source-construct-signals.lino']},rust:{status:'implemented',evidence:['rust/src/translation/source_construct_signal_rules.rs','rust/tests/unit/source_construct_signals.rs','parity/fixtures/source-construct-signals.lino']}};
inventory.features=inventory.features.filter(row=>row.id!=='static-own-production-source-candidate-inventory');
const index=inventory.features.findIndex(row=>row.id===id);if(index<0)inventory.features.push(row);else inventory.features[index]=row;writeProjectData(featureFile,inventory);
const modulesFile=new URL('rust/tests/unit/mod.rs',root);let modules=readFileSync(modulesFile,'utf8');if(!modules.includes('mod source_construct_signals;'))writeFileSync(modulesFile,modules+'\nmod source_construct_signals;\n');
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
