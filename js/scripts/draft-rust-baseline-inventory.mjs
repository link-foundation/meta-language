// Future source-availability declarations only, never execution observations.
// Run only after automatic source generation and the required source audit.
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),inventory=readProjectData(file);
const profiles=[
 {id:'ordinary-transparent-rust-source-type-aliases',javascript:[
  'js/src/translation/rust-generic-type-alias-rules.js','js/src/translation/rust-generic-type-aliases.js',
  'js/tests/translation-rust-generic-type-aliases.test.js','parity/fixtures/rust-generic-type-alias-source-draft.lino',
 ],rust:['rust/src/translation/rust_generic_type_alias_rules.rs','rust/src/translation/rust/generic_type_aliases.rs','rust/tests/unit/translation_rust_generic_type_aliases.rs']},
 {id:'ordinary-rust-standard-sums-and-ordered-propagation',javascript:[
  'js/src/translation/rust-standard-sum-source-rules.js','js/src/translation/rust-standard-sum-source-metadata-rules.js',
  'js/src/translation/rust-standard-sum-source-metadata.js','js/src/translation/rust-standard-sum-source.js',
  'js/src/translation/rust-standard-sum-completions.js','js/src/translation/rust-expression-completion-rules.js',
  'js/src/translation/rust-expression-completions.js','js/src/translation/rust-standard-sum-method-rules.js',
  'js/src/translation/rust-standard-sum-methods.js','js/src/translation/rust-standard-sum-signature-rules.js',
  'js/src/translation/rust-source-signatures.js','js/src/translation/rust.js',
  'js/tests/translation-rust-standard-sum-integration.test.js','js/tests/translation-rust-standard-sum-signatures.test.js',
  'js/tests/translation-rust-standard-sum-source-metadata.test.js','parity/fixtures/rust-standard-sum-source-metadata-draft.lino',
  'js/tests/support/rust-standard-sum-concrete.js','parity/fixtures/rust-standard-sum-integration-source-draft.lino',
 ],rust:[
  'rust/src/translation/rust_standard_sum_source_rules.rs','rust/src/translation/rust_standard_sum_source_metadata_rules.rs',
  'rust/src/translation/rust_expression_completion_rules.rs','rust/src/translation/rust_standard_sum_method_rules.rs',
  'rust/src/translation/rust_standard_sum_signature_rules.rs','rust/src/translation/rust/standard_sum_source.rs',
  'rust/src/translation/rust/standard_sum_metadata.rs','rust/src/translation/rust/standard_sum_completions.rs',
  'rust/src/translation/rust/expression_completions.rs','rust/src/translation/rust/standard_sum_methods.rs',
  'rust/src/translation/source_signatures.rs','rust/tests/unit/translation_rust_standard_sum_integration.rs',
 ]},
 {id:'stored-copied-rust-array-lookup-options',javascript:[
  'js/src/translation/rust-array-lookup-option-rules.js','js/src/translation/rust-array-lookup-options.js',
  'js/tests/translation-rust-array-lookup-options.test.js','parity/fixtures/rust-array-lookup-options.lino',
 ],rust:['rust/src/translation/rust_array_lookup_option_rules.rs','rust/src/translation/rust/expressions/array_lookup_options.rs','rust/tests/unit/translation_rust_standard_sum_integration.rs']},
 {id:'standard-option-versioned-panic-methods',javascript:[
  'js/src/translation/rust-standard-sum-method-rules.js','js/src/translation/rust-standard-sum-methods.js',
  'js/tests/translation-rust-standard-option-panic.test.js','parity/fixtures/rust-standard-option-panic-methods.lino',
 ],rust:['rust/src/translation/rust_standard_sum_method_rules.rs','rust/src/translation/rust/standard_sum_methods.rs','rust/tests/unit/translation_rust_standard_option_panic.rs']},
];
for(const profile of profiles){
 const at=inventory.features.findIndex(row=>row.id===profile.id),previous=at<0?null:inventory.features[at];
 const row={...previous,id:profile.id,required:true};
 for(const runtime of ['javascript','rust'])row[runtime]={...previous?.[runtime],status:'implemented',evidence:[...new Set([...(previous?.[runtime]?.evidence??[]),...profile[runtime]])]};
 if(at<0)inventory.features.push(row);else inventory.features[at]=row;
}
writeProjectData(file,inventory);
