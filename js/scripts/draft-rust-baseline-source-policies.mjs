// Future source authoring only. Do not run under the source-edit-only phase.
// This emits policy libraries/literal data for the source-authored frontend
// consumers. It performs no represented-program or control execution.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {draftSourceLibrary} from './draft-source-library.mjs';
import {writeProjectData} from './project-data.mjs';

if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
const libraries=[
 {sourcePath:'js/src/translation/rust-standard-sum-nested-rules.js',moduleName:'rust_standard_sum_nested_rules',model:'Rust direct Copy nested standard sum methods 1',programAsset:'rust-standard-sum-nested-program'},
 {sourcePath:'js/src/translation/rust-source-namespace-rules.js',moduleName:'rust_source_namespace_rules',model:'Rust source type and value namespaces 1',programAsset:'rust-source-namespace-program'},
 {sourcePath:'js/src/translation/rust-standard-sum-conversion-rules.js',moduleName:'rust_standard_sum_conversion_rules',model:'Rust direct Copy standard sum conversions 1',programAsset:'rust-standard-sum-conversion-program'},
 {sourcePath:'js/src/translation/rust-standard-sum-predicate-rules.js',moduleName:'rust_standard_sum_predicate_rules',model:'Rust direct Copy standard sum predicates 1',programAsset:'rust-standard-sum-predicate-program'},
 {sourcePath:'js/src/translation/rust-generic-type-alias-rules.js',moduleName:'rust_generic_type_alias_rules',model:'Rust generic type alias source parameter policy 1',programAsset:'rust-generic-type-alias-program'},
 {sourcePath:'js/src/translation/rust-standard-sum-source-metadata-rules.js',moduleName:'rust_standard_sum_source_metadata_rules',model:'Rust standard sum source metadata policy 1',programAsset:'rust-standard-sum-source-metadata-program'},
 {sourcePath:'js/src/translation/rust-standard-sum-source-rules.js',moduleName:'rust_standard_sum_source_rules',model:'Rust standard owned sum source policy 1',programAsset:'rust-standard-sum-source-program'},
 {sourcePath:'js/src/translation/rust-standard-sum-signature-rules.js',moduleName:'rust_standard_sum_signature_rules',model:'Rust standard owned sum signature policy 1',programAsset:'rust-standard-sum-signature-program'},
 {sourcePath:'js/src/translation/rust-expression-completion-rules.js',moduleName:'rust_expression_completion_rules',model:'Rust ordered expression completion policy 1',programAsset:'rust-expression-completion-program'},
 {sourcePath:'js/src/translation/rust-standard-sum-method-rules.js',moduleName:'rust_standard_sum_method_rules',model:'Rust direct Copy standard sum methods 1',programAsset:'rust-standard-sum-method-program'},
 {sourcePath:'js/src/translation/rust-function-pointer-rules.js',moduleName:'rust_function_pointer_rules',sourceDependencies:true,model:'ordinary safe Rust function pointers over immutable Copy source domains 1',programAsset:'rust-function-pointer-program'},
 {sourcePath:'js/src/translation/rust-unit-value-rules.js',moduleName:'rust_unit_value_rules',model:'Rust implicit Unit and ordered direct Unit call statements 1',programAsset:'rust-unit-value-program'},
 {sourcePath:'js/src/translation/rust-array-lookup-option-rules.js',moduleName:'rust_array_lookup_option_rules',model:'Rust owned Copy lookup option source policy 1',programAsset:'rust-array-lookup-option-program'},
];
const fixtures=[
 'js/tests/fixtures/rust-source-namespaces-draft.mjs',
 'js/tests/fixtures/rust-standard-sum-conversions.mjs',
 'js/tests/fixtures/rust-standard-sum-predicates.mjs',
 'js/tests/fixtures/rust-standard-sum-nested.mjs',
 'js/tests/fixtures/rust-generic-type-alias-defaults-draft.mjs',
 'js/tests/fixtures/rust-generic-type-alias-draft.mjs',
 'js/tests/fixtures/rust-standard-sum-source-rules.mjs',
 'js/tests/fixtures/rust-standard-sum-methods.mjs',
 'js/tests/fixtures/rust-standard-sum-method-source-originals.mjs',
 'js/tests/fixtures/rust-standard-sum-integration.mjs',
 'js/tests/fixtures/rust-standard-option-panic-methods.mjs',
 'js/tests/fixtures/rust-array-lookup-options.mjs',
 'js/tests/fixtures/rust-standard-sum-source-metadata-draft.mjs',
];
const consumers=[
 'js/src/translation/rust.js','js/src/translation/rust-type-aliases.js',
 'js/src/translation/rust-generic-type-aliases.js','js/src/translation/rust-standard-sum-source.js',
 'js/src/translation/rust-standard-sum-source-metadata.js',
 'js/src/translation/rust-standard-sum-completions.js','js/src/translation/rust-expression-completions.js',
 'js/src/translation/rust-standard-sum-methods.js','js/src/translation/rust-array-lookup-options.js',
 'js/src/translation/rust-standard-sum-conversions.js','js/src/translation/rust-standard-sum-predicates.js',
 'js/src/translation/rust-standard-sum-nested.js',
 'js/src/translation/rust-source-signatures.js','js/src/translation/rust-data-type-field-signatures.js',
 'js/src/translation/rust-nominal-copies.js','js/src/translation/rust-function-pointers.js',
 'js/src/translation/rust-copy-closures.js','js/src/translation/rust-ordinary-completions.js',
 'js/src/translation/rust-source-origins.js','js/src/translation/rust-pattern-conditions.js',
 'js/src/translation/rust-conditional-chains.js','js/src/translation/check.js',
 'js/src/typed-program-links-validation.js',
 'js/tests/translation-rust-generic-type-aliases.test.js',
 'js/tests/translation-rust-standard-sum-integration.test.js',
 'js/tests/translation-rust-standard-sum-signatures.test.js',
 'js/tests/translation-rust-standard-option-panic.test.js',
 'js/tests/translation-rust-array-lookup-options.test.js','js/tests/support/rust-standard-sum-concrete.js',
 'js/tests/translation-rust-standard-sum-source-metadata.test.js',
 'js/tests/translation-rust-source-namespaces.test.js',
 'js/tests/translation-rust-standard-sum-conversions.test.js',
 'js/tests/translation-rust-standard-sum-predicates.test.js',
 'js/tests/translation-rust-standard-sum-nested.test.js',
 'js/tests/translation-rust-generic-type-alias-defaults.test.js',
 'js/tests/rust-baseline-source-policies.test.js',
 'js/scripts/produce-rust-standard-sum-integration.mjs',
 'js/scripts/produce-rust-standard-option-panic.mjs',
 'js/scripts/draft-rust-baseline-inventory.mjs',
];
for(const path of [...libraries.map(row=>row.sourcePath),...fixtures,...consumers]){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);
 globalThis.gc?.();
}
for(const library of libraries){draftSourceLibrary({sourceDependencies:true,...library});globalThis.gc?.();}
// Fixture modules contain independently authored literals only. No policy is
// called to obtain an expected value, and no represented Rust program runs.
const aliases=await import('../tests/fixtures/rust-generic-type-alias-draft.mjs');
const sums=await import('../tests/fixtures/rust-standard-sum-source-rules.mjs');
const boundary='Unexecuted source controls for the explicitly connected frontend consumers. Automatic native products, source registration, parity and broader semantic closure remain deferred; declarations and generated code alone are not evidence of complete language support.';
writeProjectData(new URL('parity/fixtures/rust-generic-type-alias-source-draft.lino',root),{
 schemaVersion:1,model:libraries.find(row=>row.moduleName==='rust_generic_type_alias_rules').model,boundary,
 sourceProfile:aliases.boundary,
 parameterPlans:aliases.parameterPlans,substitutionPlans:aliases.substitutionPlans,
 expansionPlans:aliases.expansionPlans,sourcePrograms:aliases.sourcePrograms,
 declarationPlans:aliases.declarationPlans,vectorScalarDomains:aliases.vectorScalarDomains,
});
writeProjectData(new URL('parity/fixtures/rust-standard-sum-source-draft.lino',root),{
 schemaVersion:1,model:libraries.find(row=>row.moduleName==='rust_standard_sum_source_rules').model,boundary,
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 decisions:sums.rustStandardSumSourceDecisions,
 originals:sums.rustStandardSumSourceOriginals,
 refusals:sums.rustStandardSumSourceRefusals,
});
// Every remaining fixture namespace is an independently authored literal data
// module. Convert all of its exports automatically without dropping any rows.
for(const [source,target] of [
 ['rust-source-namespaces-draft','rust-source-namespaces-draft'],
 ['rust-standard-sum-conversions','rust-standard-sum-conversions'],
 ['rust-standard-sum-predicates','rust-standard-sum-predicates'],
 ['rust-standard-sum-nested','rust-standard-sum-nested'],
 ['rust-generic-type-alias-defaults-draft','rust-generic-type-alias-defaults-draft'],
 ['rust-standard-sum-methods','rust-standard-sum-methods-source-draft'],
 ['rust-standard-sum-method-source-originals','rust-standard-sum-method-source-originals'],
 ['rust-standard-sum-integration','rust-standard-sum-integration-source-draft'],
 ['rust-standard-option-panic-methods','rust-standard-option-panic-methods'],
 ['rust-array-lookup-options','rust-array-lookup-options'],
 ['rust-standard-sum-source-metadata-draft','rust-standard-sum-source-metadata-draft'],
]){
 const declarations=await import('../tests/fixtures/'+source+'.mjs');
 writeProjectData(new URL('parity/fixtures/'+target+'.lino',root),{
  schemaVersion:1,model:source+' source literals 1',boundary,declarations:{...declarations},
 });
 globalThis.gc?.();
}
// Preserve original alias fixture coordinates/history while appending the new
// authored controls, then register concrete source availability. Neither import
// executes represented programs or a validation gate.
await import('./draft-rust-type-alias-fixtures.mjs');globalThis.gc?.();
await import('./draft-rust-baseline-inventory.mjs');
// Assemble generated products into both actual source-definition catalogs and
// the native package manifest. These are declaration writers, not test runners.
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');
await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
const {draftRustPackageSourceAssets}=await import('./rust-package-source-assets.mjs');
draftRustPackageSourceAssets();
process.argv.splice(2,process.argv.length-2,'--draft');
