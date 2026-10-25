// Source and future-control declarations only; no tests or product imports.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file),id='javascript-actual-closed-own-field-object-rest-formals';
const common=['parity/fixtures/object-rest-formals.lino','parity/self-translation/javascript-record-parameter-rules-native.lino','parity/self-translation/object-rest-rules-native.lino'];
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/translation/javascript-record-parameter-rules.js','js/src/translation/javascript-record-parameters.js','js/src/translation/object-rest.js','js/tests/translation-object-rest-formals.test.js','js/tests/translation-source-exceptions.test.js','experiments/javascript-program/object-rest-formals.mjs',...common]},rust:{status:'implemented',evidence:['rust/src/translation/javascript_record_parameter_rules.rs','rust/src/translation/javascript/record_parameters.rs','rust/src/translation/object_rest.rs','rust/tests/unit/translation_javascript_object_rest_formals.rs',...common]}};
const at=value.features.findIndex(prior=>prior.id===id);if(at<0)value.features.push(row);else value.features[at]=row;writeProjectData(file,value);
const matrixFile=new URL('../tests/translation-source-exceptions.test.js',import.meta.url);let matrix=readFileSync(matrixFile,'utf8');
const addition="['object-rest-formals',createJavaScriptRecordParameterProgramLinks],",anchor="['object-binding-declarations',createJavaScriptRecordParameterProgramLinks],";
if(!matrix.includes(addition)){if(matrix.split(anchor).length!==2)throw new TypeError('the object rest formal matrix needs its actual unique anchor');matrix=matrix.replace(anchor,()=>anchor+addition);writeFileSync(matrixFile,matrix);}
const modulesFile=new URL('../../rust/tests/unit/mod.rs',import.meta.url);let modules=readFileSync(modulesFile,'utf8');if(!modules.includes('mod translation_javascript_object_rest_formals;')){modules+='\nmod translation_javascript_object_rest_formals;\n';writeFileSync(modulesFile,modules);}
