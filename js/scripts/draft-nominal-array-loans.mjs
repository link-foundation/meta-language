// Actual source generation, independent literal controls and source inventories
// only. Never import a future control or execute a represented projection.
import {readFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
for(const path of ['js/src/translation/nominal-array-loan-rules.js','js/src/translation/nominal-array-loans.js','js/src/translation/emit-rust.js',
 'js/scripts/draft-nominal-array-loan-rules.mjs','js/scripts/draft-nominal-array-loan-fixtures.mjs','js/scripts/draft-nominal-array-loans.mjs',
 'js/scripts/draft-javascript-data-fields-fixtures.mjs','js/tests/translation-nominal-array-loans.test.js','experiments/javascript-properties/data-field-oracle.mjs']){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);globalThis.gc?.();
}
for(const path of ['draft-nominal-array-loan-rules.mjs','draft-javascript-ordinary-heap.mjs','draft-nominal-array-loan-fixtures.mjs','draft-javascript-data-fields-fixtures.mjs']){
 await import(new URL(path,import.meta.url));globalThis.gc?.();
}
const file=new URL('parity/language-features.lino',root),inventory=readProjectData(file);
const common=['parity/fixtures/nominal-array-loans.lino','parity/fixtures/javascript-data-fields.lino','experiments/javascript-properties/data-field-oracle.mjs'];
const row={id:'nominal-array-loans',required:true,javascript:{status:'implemented',evidence:['js/src/translation/nominal-array-loan-rules.js','js/src/translation/nominal-array-loans.js','js/tests/translation-nominal-array-loans.test.js',...common]},
 rust:{status:'implemented',evidence:['rust/src/translation/nominal_array_loan_rules.rs','rust/src/translation/emit_rust/nominal_array_loans.rs','rust/tests/unit/translation_javascript_data_fields.rs',...common]}};
const at=inventory.features.findIndex(entry=>entry.id===row.id);if(at<0)inventory.features.push(row);else inventory.features[at]=row;writeProjectData(file,inventory);
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');draftRustPackageSourceAssets();globalThis.gc?.();
process.argv.splice(2,process.argv.length-2,'--refresh-manifest');await import('./check-issue-195-acceptance.mjs');
