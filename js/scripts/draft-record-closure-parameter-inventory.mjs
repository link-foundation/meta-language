// Write source declarations and future matrix registrations only. Never import
// a product index or execute the registered controls during source authoring.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {renderRustTest,renderRustItemImports} from './generate-export-parity.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file),id='javascript-flat-record-closure-parameters';
const common=['parity/fixtures/record-closure-parameters.lino','parity/self-translation/record-closure-parameter-rules-native.lino'];
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/translation/record-closure-parameter-rules.js','js/src/translation/javascript-bindings.js','js/src/translation/javascript-record-parameters.js','js/src/record-closure-parameter-program.js','js/tests/translation-record-closure-parameters.test.js','js/tests/translation-source-exceptions.test.js','experiments/javascript-program/record-closure-parameters.mjs',...common]},
 rust:{status:'implemented',evidence:['rust/src/translation/record_closure_parameter_rules.rs','rust/src/translation/javascript/callables.rs','rust/src/translation/javascript/record_parameters.rs','rust/src/translation/record_closure_parameter_program.rs','rust/tests/unit/translation_javascript_record_parameters.rs',...common]}};
const position=value.features.findIndex(row=>row.id===id);if(position<0)value.features.push(row);else value.features[position]=row;
value.javascriptExports.createJavaScriptRecordClosureParameterProgramLinks={rust:'create_java_script_record_closure_parameter_program_links',kind:'function'};
value.javascriptExports=Object.fromEntries(Object.entries(value.javascriptExports).sort(([first],[second])=>first<second?-1:first>second?1:0));
writeProjectData(file,value);
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity.rs',import.meta.url),renderRustTest(value.javascriptExports));
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity/items.rs',import.meta.url),renderRustItemImports(value.javascriptExports));
const matrixFile=new URL('../tests/translation-source-exceptions.test.js',import.meta.url);
let matrix=readFileSync(matrixFile,'utf8');
if(!matrix.includes("['record-closure-parameters',createJavaScriptRecordClosureParameterProgramLinks]")){
 const anchor="['record-parameter-writes',createJavaScriptRecordParameterProgramLinks],";
 if(matrix.split(anchor).length!==2)throw new TypeError('the future common matrix needs its actual unique registration anchor');
 matrix=matrix.replace(anchor,()=>anchor+"['record-closure-parameters',createJavaScriptRecordClosureParameterProgramLinks],");
 writeFileSync(matrixFile,matrix);
}
