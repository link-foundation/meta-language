// Only source/API and future control declarations. No learner/product imports.
import {writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {renderRustTest,renderRustItemImports} from './generate-export-parity.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file),id='regular-grammar-counterexample-refinement-checkpoints';
const common=['parity/fixtures/regular-grammar-refinement.lino','parity/self-translation/grammar-inference-refinement-rules-native.lino'];
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/grammar-inference-refinement.js','js/src/grammar-inference-refinement-rules.js','js/tests/grammar-inference-refinement.test.js','experiments/grammar-inference/refinement-oracle.mjs',...common]},rust:{status:'implemented',evidence:['rust/src/grammar_inference/refinement.rs','rust/src/grammar_inference_refinement_rules.rs','rust/tests/unit/regular_grammar_inference_refinement.rs',...common]}};
const index=value.features.findIndex(row=>row.id===id);if(index<0)value.features.push(row);else value.features[index]=row;
for(const [javascript,rust]of Object.entries({refineRegularGrammarInference:'refine_regular_grammar_inference',readRefinedRegularGrammarInferenceLinks:'read_refined_regular_grammar_inference_links'}))value.javascriptExports[javascript]={rust,kind:'function'};
value.javascriptExports=Object.fromEntries(Object.entries(value.javascriptExports).sort(([a],[b])=>a<b?-1:a>b?1:0));
writeProjectData(file,value);
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity.rs',import.meta.url),renderRustTest(value.javascriptExports));
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity/items.rs',import.meta.url),renderRustItemImports(value.javascriptExports));
