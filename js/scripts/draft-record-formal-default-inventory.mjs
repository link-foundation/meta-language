// Source and deferred matrix declarations only; no product or fixture execution.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file),id='javascript-represented-record-formal-defaults';
const common=['parity/fixtures/record-formal-defaults.lino','parity/self-translation/javascript-record-parameter-rules-native.lino','parity/self-translation/array-formal-parameter-rules-native.lino'];
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/translation/javascript-record-parameter-rules.js','js/src/translation/javascript-record-parameters.js','js/src/translation/formal-initializers.js','js/src/translation/record-formal-defaults.js','js/src/translation/javascript-binding-defaults.js','js/src/translation/check.js','js/src/translation/javascript-infer.js','js/tests/translation-record-formal-defaults.test.js','experiments/javascript-program/record-formal-defaults.mjs',...common]},rust:{status:'implemented',evidence:['rust/src/translation/javascript_record_parameter_rules.rs','rust/src/translation/javascript/record_parameters.rs','rust/src/translation/javascript/formal_initializers.rs','rust/src/translation/record_formal_defaults.rs','rust/src/translation/check/binding_defaults.rs','rust/src/translation/check/record_formal_defaults.rs','rust/src/translation/javascript/infer/record_formal_defaults.rs','rust/src/translation/javascript/infer/array_bindings.rs','rust/tests/unit/translation_javascript_record_formal_defaults.rs',...common]}};
const position=value.features.findIndex(row=>row.id===id);if(position<0)value.features.push(row);else value.features[position]=row;writeProjectData(file,value);
const matrixFile=new URL('../tests/translation-source-exceptions.test.js',import.meta.url);let matrix=readFileSync(matrixFile,'utf8');
if(!matrix.includes("['record-formal-defaults',createJavaScriptRecordParameterProgramLinks]")){
 const anchor="['record-closure-parameters',createJavaScriptRecordClosureParameterProgramLinks],";
 if(matrix.split(anchor).length!==2)throw new TypeError('the record formal matrix needs its actual unique anchor');
 matrix=matrix.replace(anchor,()=>anchor+"['record-formal-defaults',createJavaScriptRecordParameterProgramLinks],");writeFileSync(matrixFile,matrix);
}
const modulesFile=new URL('../../rust/tests/unit/mod.rs',import.meta.url);let modules=readFileSync(modulesFile,'utf8');if(!modules.includes('mod translation_javascript_record_formal_defaults;')){modules+='\nmod translation_javascript_record_formal_defaults;\n';writeFileSync(modulesFile,modules);}
