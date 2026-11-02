// Literal public declaration consumers only; no type checker or source body runs
// when this inventory is authored. The normal CI consumer gate owns execution.
export function publicRustSourceStateConsumers(moduleSpecifier){
 const module=JSON.stringify(moduleSpecifier);
 return [
  {name:'valid-complete-Rust-source-state-library-and-project-options',expectedDiagnostics:[],assertionPolicy:'typescript-rust-state-valid-consumer-diagnostic',source:`import {createTypedProgramLinks,createRustSourceProjectProgramLinks,captureRustSuppliedModuleProject,readRustSuppliedModuleOwnership,type RustStandardCollectionSourceLibrary,type RustSourceProjectCompilation,type RustSuppliedModuleOwnership} from ${module};
export function translateSelected(library:RustStandardCollectionSourceLibrary) {
 return createTypedProgramLinks('fn main() {}','Rust',{rustSourceVersion:'1.90.0',rustExclusiveSourceModel:'Rust exclusive source loans 1',rustStandardCollectionSourceModel:'Rust closed standard collections 1',rustStandardCollectionSourceLibrary:library,rustStandardCollectionPointerWidth:64});
}
export function composeSelected(library:RustStandardCollectionSourceLibrary) {
 const result:RustSourceProjectCompilation=createRustSourceProjectProgramLinks([{path:'main.rs',source:'fn main() {}'}],'main.rs',{retainCompilation:true,sourceVersion:'1.90.0',exclusiveSourceModel:'Rust exclusive source loans 1',standardCollectionSourceModel:'Rust closed standard collections 1',standardCollectionSourceLibrary:library,standardCollectionPointerWidth:64,moduleLimit:32});
 return result.composition.source;
}
export function selectConcreteFnMut() {
 return createTypedProgramLinks('fn main() {}','Rust',{rustSourceVersion:'1.90.0',rustExclusiveSourceModel:'Rust exclusive source loans 1',rustFnMutSourceModel:'Rust closed source FnMut specializations 1'});
}
export function inspectOwnership() {
 const packet=captureRustSuppliedModuleProject([{path:'main.rs',source:'fn main() {}'}],{crateRoot:'main.rs',moduleLimit:32});
 const ownership:RustSuppliedModuleOwnership=readRustSuppliedModuleOwnership(packet.network,packet.root);
 const selected:boolean=ownership.files[0].selected;return {selected,archive:packet.archive};
}
export const contractEvidence={model:'declared Rust source state and concrete ownership interfaces'};
`},
  {name:'reject-uncovered-Rust-collection-frontend-pointer-width',expectedDiagnostics:['TS2322'],assertionPolicy:'typescript-rust-state-pointer-width-rejection-diagnostic',source:`import {createTypedProgramLinks} from ${module};
createTypedProgramLinks('fn main() {}','Rust',{rustStandardCollectionSourceModel:'Rust closed standard collections 1',rustStandardCollectionPointerWidth:32});
`},
  {name:'reject-invented-Rust-exclusive-source-profile',expectedDiagnostics:['TS2322'],assertionPolicy:'typescript-rust-state-exclusive-profile-rejection-diagnostic',source:`import {createTypedProgramLinks} from ${module};
createTypedProgramLinks('fn main() {}','Rust',{rustExclusiveSourceModel:'Rust exclusive source loans 2'});
`},
  {name:'reject-Rust-library-without-resource-continuation',expectedDiagnostics:['TS2741'],assertionPolicy:'typescript-rust-state-resource-handler-rejection-diagnostic',source:`import type {RustStandardCollectionSourceHandlers} from ${module};
const handlers:RustStandardCollectionSourceHandlers={invalid:reason=>({k:'abort',message:'invalid source packet'})};
export const contractEvidence=handlers;
`},
  {name:'reject-CST-source-link-identity-as-display-text',expectedDiagnostics:['TS2322'],assertionPolicy:'typescript-rust-state-concrete-identity-rejection-diagnostic',source:`import type {RustSuppliedModuleOwnership} from ${module};
export function invalidIdentity(graph:RustSuppliedModuleOwnership) {const identity:string=graph.modules[0].sourceLink;return identity;}
`},
 ];
}
