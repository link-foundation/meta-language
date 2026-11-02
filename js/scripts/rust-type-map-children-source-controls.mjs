// Deferred current-source capture. This module performs no filesystem reads,
// grammar parsing, fixture construction or represented execution on import.
import assert from 'node:assert/strict';
import {readFileSync,mkdirSync,writeFileSync} from 'node:fs';
import {join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {LinkNetwork} from '../src/network.js';
import {writeProjectData} from './project-data.mjs';
import {requireControlProduct} from '../tests/support/control-diagnostics.js';
import {createRustTypeMapChildrenSourceOriginals,rustTypeMapChildrenSourceScenarios,rustTypeMapChildrenSourceVersions} from '../tests/fixtures/rust-type-map-children-source.mjs';
import {createRustTypeChildrenSourceOriginals,rustTypeChildrenSourceLengths,rustTypeChildrenSourceOrderScenarios,rustTypeChildrenSourceVersions} from '../tests/fixtures/rust-type-children-source.mjs';
import {compileActualTypeOwnCrateReference} from './rust-type-map-children-own-crate-reference.mjs';
import {assertActualTypeVariantCoverage} from './rust-type-source-variant-coverage.mjs';
import {createRustTypeRegionCompositionOriginals,rustTypeRegionCompositionScenarios,rustSourceRegionCompositionVersions} from '../tests/fixtures/rust-source-region-composition.mjs';
const repository=fileURLToPath(new URL('../../',import.meta.url));
export function createActualRustTypeMapChildrenControlGroup(diagnostics,directory,nativeContext){
 return createActualRustTypeControlGroup(diagnostics,directory,nativeContext,{name:'map_children',id:'actual-type-map-children',factory:createRustTypeMapChildrenSourceOriginals,versions:rustTypeMapChildrenSourceVersions,names:rustTypeMapChildrenSourceScenarios.map(row=>row.name),inputs:rustTypeMapChildrenSourceScenarios.map(row=>row.input),fnMut:true});
}
export function createActualRustTypeChildrenControlGroup(diagnostics,directory,nativeContext){
 return createActualRustTypeControlGroup(diagnostics,directory,nativeContext,{name:'children',id:'actual-type-children',factory:createRustTypeChildrenSourceOriginals,versions:rustTypeChildrenSourceVersions,names:[...rustTypeChildrenSourceLengths.map(row=>row.input),...rustTypeChildrenSourceOrderScenarios.map(row=>row.name)],inputs:[...rustTypeChildrenSourceLengths,...rustTypeChildrenSourceOrderScenarios].map(row=>{const values=rustTypeMapChildrenSourceScenarios.filter(scenario=>scenario.name===row.input);assert.equal(values.length,1);return values[0].input;}),fnMut:false,exclusive:false,sourceSignatureModel:rustTypeChildrenSourceVersions.sourceSignatureModel});
}
export function createActualRustTypeRegionCompositionControlGroup(diagnostics,directory,nativeContext){
 return createActualRustTypeControlGroup(diagnostics,directory,nativeContext,{name:'children and map_children composition',id:'actual-type-region-composition',factory:createRustTypeRegionCompositionOriginals,versions:rustSourceRegionCompositionVersions,names:rustTypeRegionCompositionScenarios.map(row=>row.name),inputs:rustTypeRegionCompositionScenarios.map(row=>row.input),fnMut:true,sourceSignatureModel:'Rust source function signature 3',variantScope:'declared subset'});
}
function createActualRustTypeControlGroup(diagnostics,directory,nativeContext,definition){
 if(process.env.CI!=='true'||process.env.META_LANGUAGE_NATIVE_RUST_STATE!=='1')throw new Error('actual own-code native controls require explicit CI');
 const groupDirectory=join(directory,definition.id+'-source');const captured=diagnostics.attempt('actual complete Type '+definition.name+' source and original CST capture',()=>{
  const typesSource=readFileSync(join(repository,'rust/src/translation/types.rs'),'utf8'),spanSource=readFileSync(join(repository,'rust/src/translation/mod.rs'),'utf8'),typesNetwork=LinkNetwork.parse(typesSource,'Rust'),spanNetwork=LinkNetwork.parse(spanSource,'Rust');
  const result=definition.factory({typesSource,typesNetwork,spanSource,spanNetwork});assert.equal(result.originals.length,definition.names.length);
  const variants=assertActualTypeVariantCoverage(typesNetwork,typesSource,result.types,definition.inputs,{scope:definition.variantScope??'all variants'});
  const originals=join(groupDirectory,'actual-type-origins');mkdirSync(originals,{recursive:true});
  writeFileSync(join(originals,'types.rs'),typesSource);writeFileSync(join(originals,'translation-mod.rs'),spanSource);
  writeFileSync(join(originals,'types-concrete.lino'),typesNetwork.toLino());writeFileSync(join(originals,'translation-mod-concrete.lino'),spanNetwork.toLino());
  writeProjectData(join(originals,'capture.lino'),{model:result.model,scope:result.scope,versions:result.versions,types:result.types,span:result.span,variants});return result;
 });
 const ownCrateReference=diagnostics.attempt('actual locked crate Type '+definition.name+' native reference compilation',()=>{requireControlProduct(nativeContext.profile,'covered actual compiler profile');return compileActualTypeOwnCrateReference(requireControlProduct(captured,'actual original Type source capture'),{directory:groupDirectory,...nativeContext});});
 // Keep every declared scenario even when shared capture fails. Each source
 // derivation assertion then records its unavailable actual predecessor.
 const rows=definition.names.map((name,index)=>({name,stdout:'true\n',sourceVersionRange:definition.versions.rust,sourceFactory(){const row=requireControlProduct(captured,'actual original Type source capture').originals[index];assert.equal(row.name,name);assert.equal(typeof row.source,'string');assert.equal(typeof row.nativeSource,'string');return row;}}));
 return {id:definition.id,rows,fnMut:definition.fnMut,exclusive:definition.exclusive,sourceSignatureModel:definition.sourceSignatureModel,collections:false,valueProjection:true,ownCrateReference,sourceVersionRange:definition.versions.rust};
}
