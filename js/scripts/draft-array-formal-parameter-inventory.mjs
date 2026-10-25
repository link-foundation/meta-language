// Source/API/matrix declaration writer only. Never imports the product index.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {renderRustTest,renderRustItemImports} from './generate-export-parity.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file),id='javascript-ordinary-array-formal-parameters';
const common=['parity/fixtures/array-formal-parameters.lino','parity/self-translation/array-formal-parameter-rules-native.lino'];
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/translation/array-formal-parameter-rules.js','js/src/translation/array-formal-parameters.js','js/src/translation/array-formal-iterables.js','js/src/translation/javascript.js','js/src/translation/javascript-infer.js','js/src/array-formal-parameter-program.js','js/tests/translation-array-formal-parameters.test.js','js/tests/translation-source-exceptions.test.js','experiments/javascript-program/array-formal-parameters.mjs',...common]},rust:{status:'implemented',evidence:['rust/src/translation/array_formal_parameter_rules.rs','rust/src/translation/javascript/array_formal_parameters.rs','rust/src/translation/javascript/infer/products.rs','rust/src/translation/check/arrays.rs','rust/src/translation/array_formal_parameter_program.rs','rust/tests/unit/translation_javascript_array_formals.rs',...common]}};
const index=value.features.findIndex(row=>row.id===id);if(index<0)value.features.push(row);else value.features[index]=row;
value.javascriptExports.createJavaScriptArrayFormalParameterProgramLinks={rust:'create_java_script_array_formal_parameter_program_links',kind:'function'};
value.javascriptExports=Object.fromEntries(Object.entries(value.javascriptExports).sort(([a],[b])=>a<b?-1:a>b?1:0));writeProjectData(file,value);
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity.rs',import.meta.url),renderRustTest(value.javascriptExports));
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity/items.rs',import.meta.url),renderRustItemImports(value.javascriptExports));
const matrixFile=new URL('../tests/translation-source-exceptions.test.js',import.meta.url);let matrix=readFileSync(matrixFile,'utf8');
if(!matrix.includes("['array-formal-parameters',createJavaScriptArrayFormalParameterProgramLinks]")){
 const anchor="['record-callable-parameters',createJavaScriptRecordCallableParameterProgramLinks],";
 if(matrix.split(anchor).length!==2)throw new TypeError('the array formal matrix needs its actual unique anchor');
 matrix=matrix.replace(anchor,()=>anchor+"['array-formal-parameters',createJavaScriptArrayFormalParameterProgramLinks],");writeFileSync(matrixFile,matrix);
}
