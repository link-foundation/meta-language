// Register authored sources and future controls; never imports product APIs.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file),id='javascript-homogeneous-own-entry-value-reads';
const common=['parity/fixtures/javascript-object-entries.lino','parity/self-translation/object-entry-pair-rules-native.lino'];
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/translation/javascript-object-enumeration.js','js/src/translation/record-value-order-template.js','js/src/translation/record-value-order-specialization.js','js/src/translation/ordered-record-preparation.js','js/src/typed-program-links-validation.js','js/tests/translation-source-exceptions.test.js','js/tests/translation-object-entry-boundaries.test.js','experiments/javascript-program/javascript-object-entries.mjs',...common]},rust:{status:'implemented',evidence:['rust/src/translation/object_entry_pair_rules.rs','rust/src/translation/check/javascript_object_enumeration.rs','rust/src/translation/record_value_order_specialization.rs','rust/src/translation/ordered_record_preparation.rs','rust/tests/unit/translation_javascript_object_entries.rs',...common]}};
const index=value.features.findIndex(row=>row.id===id);if(index<0)value.features.push(row);else value.features[index]=row;writeProjectData(file,value);
const matrixFile=new URL('../tests/translation-source-exceptions.test.js',import.meta.url);let matrix=readFileSync(matrixFile,'utf8');
if(!matrix.includes("['javascript-object-entries',()=>createSourceDefinitionProgramLinks(")){
 const anchor="['javascript-object-values',createJavaScriptRecordOrderProgramLinks],";if(matrix.split(anchor).length!==2)throw new TypeError('actual unique own entry matrix anchor is required');
 matrix=matrix.replace(anchor,()=>anchor+"['javascript-object-entries',()=>createSourceDefinitionProgramLinks('meta-language/source-program/object-entry-pair-program')],");writeFileSync(matrixFile,matrix);
}
const modulesFile=new URL('../../rust/tests/unit/mod.rs',import.meta.url);let modules=readFileSync(modulesFile,'utf8');if(!modules.includes('mod translation_javascript_object_entries;')){modules+='\nmod translation_javascript_object_entries;\n';writeFileSync(modulesFile,modules);}
