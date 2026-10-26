// Source/export inventory construction only; these are not observations.
import {writeFileSync,readFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {renderRustTest,renderRustItemImports} from './generate-export-parity.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),file=new URL('parity/language-features.lino',root),inventory=readProjectData(file);
for(const [id,fixture,shared,javascript,rust]of [
 ['source-qualified-grammar-composition-batches','grammar-composition-batch','grammar-composition-batch-rules',['js/src/grammar-composition-batch-rules.js','js/src/grammar-composition-batch.js','js/tests/grammar-composition-batch.test.js','experiments/grammar-composition/batch.mjs'],['rust/src/grammar_composition_batch_rules.rs','rust/src/grammar_composition_batch.rs','rust/tests/unit/grammar_composition_batch.rs']],
 ['dense-primitive-string-copying-sort','string-array-copy-sort','string-array-sort-rules',['js/src/translation/string-array-sort-rules.js','js/src/translation/check.js','js/src/translation/emit-rust.js','js/src/translation/emit-javascript.js','js/src/translation/emit-lean.js','js/src/translation/emit-rocq.js','js/src/typed-program-links-validation.js','js/src/typed-program-expression-projection.js','js/tests/translation-string-array-copy-sort.test.js','experiments/javascript-program/string-array-copy-sort.mjs'],['rust/src/translation/string_array_sort_rules.rs','rust/src/translation/check/primitive_methods.rs','rust/src/translation/emit_rust/primitive_methods.rs','rust/src/translation/emit_javascript/expressions.rs','rust/src/translation/emit_lean.rs','rust/src/translation/emit_rocq/expressions.rs','rust/src/typed_program_links/checked.rs','rust/tests/unit/translation_string_array_copy_sort.rs']],
]){
 const common=['parity/fixtures/'+fixture+'.lino','parity/self-translation/'+shared+'-native.lino'],row={id,required:true,javascript:{status:'implemented',evidence:[...javascript,...common]},rust:{status:'implemented',evidence:[...rust,...common]}};
 const at=inventory.features.findIndex(previous=>previous.id===id);if(at<0)inventory.features.push(row);else inventory.features[at]=row;
}
for(const [name,rust]of [['createGrammarCompositionBatchLinks','create_grammar_composition_batch_links'],['readGrammarCompositionBatchLinks','read_grammar_composition_batch_links']])inventory.javascriptExports[name]={rust,kind:'function'};
inventory.javascriptExports=Object.fromEntries(Object.entries(inventory.javascriptExports).sort(([first],[second])=>first<second?-1:first>second?1:0));
writeProjectData(file,inventory);
writeFileSync(new URL('rust/tests/unit/javascript_export_parity.rs',root),renderRustTest(inventory.javascriptExports));
writeFileSync(new URL('rust/tests/unit/javascript_export_parity/items.rs',root),renderRustItemImports(inventory.javascriptExports));
const modulesFile=new URL('rust/tests/unit/mod.rs',root);let modules=readFileSync(modulesFile,'utf8');for(const name of ['grammar_composition_batch','translation_string_array_copy_sort'])if(!modules.includes('mod '+name+';'))modules+='\nmod '+name+';\n';writeFileSync(modulesFile,modules);
