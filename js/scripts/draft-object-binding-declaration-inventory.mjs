// Register source availability and future controls; execute no products.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file);
const nativeTest='rust/tests/unit/translation_javascript_object_binding_declarations.rs';
const rows=[
 {id:'javascript-actual-lexical-object-binding-declarations',required:true,
  javascript:{status:'implemented',evidence:['js/src/translation/javascript-record-parameter-rules.js','js/src/translation/object-binding-declarations.js','js/src/translation/formal-initializers.js','js/src/translation/javascript.js','js/tests/translation-object-binding-declarations.test.js','js/tests/translation-javascript-mutable-array-bindings.test.js','js/tests/translation-source-exceptions.test.js','experiments/javascript-program/object-binding-declarations.mjs','parity/fixtures/object-binding-declarations.lino','parity/self-translation/javascript-record-parameter-rules-native.lino']},
  rust:{status:'implemented',evidence:['rust/src/translation/javascript_record_parameter_rules.rs','rust/src/translation/javascript/object_binding_declarations.rs','rust/src/translation/javascript/formal_initializers.rs','rust/src/translation/javascript/declarations.rs','rust/src/translation/javascript/loops.rs','rust/src/translation/javascript/statements.rs',nativeTest,'rust/tests/unit/translation_javascript_array_bindings.rs','parity/fixtures/object-binding-declarations.lino','parity/self-translation/javascript-record-parameter-rules-native.lino']}},
 {id:'javascript-owned-array-reader-disjoint-primitive-results',required:true,
  javascript:{status:'implemented',evidence:['js/src/translation/owned-array-reader-domain-rules.js','js/src/translation/owned-array-append.js','js/tests/translation-owned-array-reader-domains.test.js','js/tests/translation-source-exceptions.test.js','experiments/javascript-program/owned-array-reader-domains.mjs','parity/fixtures/owned-array-reader-domains.lino','parity/self-translation/owned-array-reader-domain-rules-native.lino']},
  rust:{status:'implemented',evidence:['rust/src/translation/owned_array_reader_domain_rules.rs','rust/src/translation/javascript/owned_array_append.rs',nativeTest,'parity/fixtures/owned-array-reader-domains.lino','parity/self-translation/owned-array-reader-domain-rules-native.lino']}},
];
for(const row of rows){const position=value.features.findIndex(prior=>prior.id===row.id);if(position<0)value.features.push(row);else value.features[position]=row;}
writeProjectData(file,value);
const matrixFile=new URL('../tests/translation-source-exceptions.test.js',import.meta.url);let matrix=readFileSync(matrixFile,'utf8');
for(const [anchor,addition] of [
 ["['record-formal-defaults',createJavaScriptRecordParameterProgramLinks],","['object-binding-declarations',createJavaScriptRecordParameterProgramLinks],"],
 ["['lexical-declarations',()=>createSourceDefinitionProgramLinks('meta-language/source-program/lexical-declaration-program')],","['owned-array-reader-domains',()=>createSourceDefinitionProgramLinks('meta-language/source-program/owned-array-reader-domain-program')],"],
]){if(matrix.includes(addition))continue;if(matrix.split(anchor).length!==2)throw new TypeError('a unique complete source matrix anchor is required');matrix=matrix.replace(anchor,()=>anchor+addition);}
writeFileSync(matrixFile,matrix);
const modulesFile=new URL('../../rust/tests/unit/mod.rs',import.meta.url);let modules=readFileSync(modulesFile,'utf8');
if(!modules.includes('mod translation_javascript_object_binding_declarations;')){modules+='\nmod translation_javascript_object_binding_declarations;\n';writeFileSync(modulesFile,modules);}
