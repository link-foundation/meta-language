// Source declaration author only. No product index, oracle or fixture executes.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {renderRustTest,renderRustItemImports} from './generate-export-parity.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file),id='javascript-ordinary-record-callable-parameters';
const common=['parity/fixtures/record-callable-parameters.lino','parity/fixtures/record-callable-parameter-assertion-history.lino','parity/self-translation/record-callable-parameter-rules-native.lino'];
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/translation/record-callable-parameter-rules.js','js/src/translation/javascript-record-parameters.js','js/src/translation/javascript.js','js/src/record-callable-parameter-program.js','js/tests/translation-record-callable-parameters.test.js','js/tests/translation-source-exceptions.test.js','experiments/javascript-program/record-callable-parameters.mjs',...common]},rust:{status:'implemented',evidence:['rust/src/translation/record_callable_parameter_rules.rs','rust/src/translation/javascript/declarations.rs','rust/src/translation/javascript/record_parameters.rs','rust/src/translation/record_callable_parameter_program.rs','rust/tests/unit/translation_javascript_record_parameters.rs',...common]}};
const index=value.features.findIndex(row=>row.id===id);if(index<0)value.features.push(row);else value.features[index]=row;
value.javascriptExports.createJavaScriptRecordCallableParameterProgramLinks={rust:'create_java_script_record_callable_parameter_program_links',kind:'function'};
value.javascriptExports=Object.fromEntries(Object.entries(value.javascriptExports).sort(([a],[b])=>a<b?-1:a>b?1:0));
writeProjectData(file,value);
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity.rs',import.meta.url),renderRustTest(value.javascriptExports));
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity/items.rs',import.meta.url),renderRustItemImports(value.javascriptExports));
const matrixFile=new URL('../tests/translation-source-exceptions.test.js',import.meta.url);let matrix=readFileSync(matrixFile,'utf8');
if(!matrix.includes("['record-callable-parameters',createJavaScriptRecordCallableParameterProgramLinks]")){
 const anchor="['record-closure-parameters',createJavaScriptRecordClosureParameterProgramLinks],";
 if(matrix.split(anchor).length!==2)throw new TypeError('the future callable matrix requires its actual unique anchor');
 matrix=matrix.replace(anchor,()=>anchor+"['record-callable-parameters',createJavaScriptRecordCallableParameterProgramLinks],");writeFileSync(matrixFile,matrix);
}
