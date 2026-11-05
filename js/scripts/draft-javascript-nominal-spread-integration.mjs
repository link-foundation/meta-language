// Author source/transport/literal inventories only. Never import controls.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/javascript-nominal-spread-rules.js','js/src/translation/javascript-nominal-spreads.js','js/src/translation/javascript-infer.js','js/src/translation/check.js',
 'js/scripts/portable-source-library-style.mjs','js/scripts/draft-javascript-nominal-spread-rules.mjs','js/scripts/draft-javascript-nominal-spread-fixtures.mjs','js/scripts/draft-javascript-nominal-spread-integration.mjs',
 'experiments/javascript-properties/nominal-spread-oracle.mjs','js/tests/translation-javascript-nominal-spreads.test.js']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
for(const path of ['draft-javascript-nominal-spread-rules.mjs','draft-javascript-nominal-spread-fixtures.mjs']){await import(new URL(path,import.meta.url));globalThis.gc?.();}
const file=new URL('parity/language-features.lino',root),inventory=readProjectData(file),common=['parity/fixtures/javascript-nominal-spreads.lino','experiments/javascript-properties/nominal-spread-oracle.mjs'];
const row={id:'javascript-nominal-spreads',required:true,javascript:{status:'implemented',evidence:['js/src/translation/javascript-nominal-spread-rules.js','js/src/translation/javascript-nominal-spreads.js','js/tests/translation-javascript-nominal-spreads.test.js',...common]},
 rust:{status:'implemented',evidence:['rust/src/translation/javascript_nominal_spread_rules.rs','rust/src/translation/check/javascript_nominal_spreads.rs','rust/tests/unit/translation_javascript_nominal_spreads.rs',...common]}};
const at=inventory.features.findIndex(entry=>entry.id===row.id);if(at<0)inventory.features.push(row);else inventory.features[at]=row;writeProjectData(file,inventory);
const fileNative=new URL('rust/tests/unit/mod.rs',root),declaration='mod translation_javascript_nominal_spreads;',source=readFileSync(fileNative,'utf8');if(!source.includes(declaration))writeFileSync(fileNative,source+'\n'+declaration+'\n');
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
