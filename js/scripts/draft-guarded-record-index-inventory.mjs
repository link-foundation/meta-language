// Source and deferred matrix declarations only.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file),id='javascript-guarded-homogeneous-record-indices';
const common=['parity/fixtures/javascript-guarded-record-indices.lino','parity/self-translation/javascript-own-property-rules-native.lino','parity/self-translation/javascript-record-order-rules-native.lino'];
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/translation/javascript-own-property-rules.js','js/src/translation/javascript-guarded-record-indices.js','js/src/translation/javascript-record-order-rules.js','js/src/translation/check.js','js/src/translation/javascript-infer.js','js/tests/translation-guarded-record-indices.test.js','js/tests/translation-source-exceptions.test.js','experiments/javascript-program/javascript-guarded-record-indices.mjs',...common]},rust:{status:'implemented',evidence:['rust/src/translation/javascript_own_property_rules.rs','rust/src/translation/check/guarded_record_indices.rs','rust/src/translation/javascript_record_order_rules.rs','rust/src/translation/check/expressions/branches.rs','rust/src/translation/javascript/infer/products.rs','rust/tests/unit/translation_javascript_guarded_record_indices.rs',...common]}};
const index=value.features.findIndex(row=>row.id===id);if(index<0)value.features.push(row);else value.features[index]=row;writeProjectData(file,value);
const matrixFile=new URL('../tests/translation-source-exceptions.test.js',import.meta.url);let matrix=readFileSync(matrixFile,'utf8');
if(!matrix.includes("['javascript-guarded-record-indices',createJavaScriptRecordOrderProgramLinks]")){
 const anchor="['javascript-object-keys',createJavaScriptRecordOrderProgramLinks],";if(matrix.split(anchor).length!==2)throw new TypeError('actual unique guarded record index matrix anchor required');
 matrix=matrix.replace(anchor,()=>anchor+"['javascript-guarded-record-indices',createJavaScriptRecordOrderProgramLinks],");writeFileSync(matrixFile,matrix);
}
const modulesFile=new URL('../../rust/tests/unit/mod.rs',import.meta.url);let modules=readFileSync(modulesFile,'utf8');if(!modules.includes('mod translation_javascript_guarded_record_indices;')){modules+='\nmod translation_javascript_guarded_record_indices;\n';writeFileSync(modulesFile,modules);}
