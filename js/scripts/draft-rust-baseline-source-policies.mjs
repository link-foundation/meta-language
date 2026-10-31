// Future source authoring only. Do not run under the source-edit-only phase.
// This emits policy libraries/literal data; it does not integrate generic
// aliases or standard sums into either frontend, or supply behavioral evidence.
import {readFileSync} from 'node:fs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {draftSourceLibrary} from './draft-source-library.mjs';
import {writeProjectData} from './project-data.mjs';

if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url);
const libraries=[
 {sourcePath:'js/src/translation/rust-generic-type-alias-rules.js',moduleName:'rust_generic_type_alias_rules',model:'Rust generic type alias source parameter policy 1',programAsset:'rust-generic-type-alias-program'},
 {sourcePath:'js/src/translation/rust-standard-sum-source-rules.js',moduleName:'rust_standard_sum_source_rules',model:'Rust standard owned sum source policy 1',programAsset:'rust-standard-sum-source-program'},
];
const fixtures=[
 'js/tests/fixtures/rust-generic-type-alias-draft.mjs',
 'js/tests/fixtures/rust-standard-sum-source-rules.mjs',
];
for(const path of [...libraries.map(row=>row.sourcePath),...fixtures]){
 parseAuthoredJavaScriptModule(readFileSync(new URL(path,root),'utf8'),path);
 globalThis.gc?.();
}
for(const library of libraries){draftSourceLibrary(library);globalThis.gc?.();}
// Fixture modules contain independently authored literals only. No policy is
// called to obtain an expected value, and no represented Rust program runs.
const aliases=await import('../tests/fixtures/rust-generic-type-alias-draft.mjs');
const sums=await import('../tests/fixtures/rust-standard-sum-source-rules.mjs');
const boundary='Unexecuted source-policy controls. Frontend integration, native parity, source registration and complete semantic closure remain required; generated code alone is not language support.';
writeProjectData(new URL('parity/fixtures/rust-generic-type-alias-source-draft.lino',root),{
 schemaVersion:1,model:libraries[0].model,boundary,
 sourceProfile:aliases.boundary,
 parameterPlans:aliases.parameterPlans,substitutionPlans:aliases.substitutionPlans,
 expansionPlans:aliases.expansionPlans,sourcePrograms:aliases.sourcePrograms,
});
writeProjectData(new URL('parity/fixtures/rust-standard-sum-source-draft.lino',root),{
 schemaVersion:1,model:libraries[1].model,boundary,
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 decisions:sums.rustStandardSumSourceDecisions,
 originals:sums.rustStandardSumSourceOriginals,
 refusals:sums.rustStandardSumSourceRefusals,
});
