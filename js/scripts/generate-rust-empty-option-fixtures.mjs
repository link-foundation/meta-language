import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {readProjectData,checkProjectData,writeProjectData} from './project-data.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
export const RUST_EMPTY_OPTION_SOURCE_MODEL='closed empty primitive Rust option 1';
const type={kind:'option',element:{kind:'float'},absence:'undefined'};
function expected(source,name,resultPath,constructionPath,names=[name]){
 const resultStart=source.indexOf(resultPath+'<f64>'),constructionStart=source.indexOf(constructionPath+'::<f64>');
 const argument=start=>({type:{kind:'float'},sourcePath:['f64'],resolvedPath:['f64'],canonicalPath:['f64'],arguments:[],wrappers:[],lifetimes:[],span:{start,end:start+3}});
 const owner=(path,canonicalPath,at)=>({sourcePath:path.split('::'),resolvedPath:path.split('::'),canonicalPath,arguments:[argument(at)]});
 return {model:RUST_EMPTY_OPTION_SOURCE_MODEL,language:'Rust',sourceName:name,
 namespace:{declaredTypeOwners:[],externalOptionRoots:['std','core'],imports:[],declarationNames:names},parameters:[],
 result:{type,owner:owner(resultPath,['core','option','Option'],resultStart+resultPath.length+1),wrappers:[],lifetimes:[],span:{start:resultStart,end:resultStart+resultPath.length+5}},
 construction:{owner:owner(constructionPath,['core','option','Option','None'],constructionStart+constructionPath.length+3),span:{start:constructionStart,end:constructionStart+constructionPath.length+7},path:[]}};
}
const specifications=[
 ['prelude','fn empty()->Option<f64>{None::<f64>}fn main(){}','empty','Option','None'],
 ['standard-library','pub fn empty()->std::option::Option<f64>{std::option::Option::None::<f64>}fn main(){}','empty','std::option::Option','std::option::Option::None'],
 ['core-library','fn empty()->core::option::Option<f64>{core::option::Option::None::<f64>}fn main(){}','empty','core::option::Option','core::option::Option::None'],
 ['enum-variant','fn empty()->Option<f64>{Option::None::<f64>}fn main(){}','empty','Option','Option::None'],
 ['cross-reexport','fn empty()->std::option::Option<f64>{core::option::Option::None::<f64>}fn main(){}','empty','std::option::Option','core::option::Option::None'],
 ['main-first','fn main(){}fn empty()->Option<f64>{None::<f64>}','empty','Option','None'],
 ['type-and-value-namespace','fn f64()->Option<f64>{None::<f64>}fn main(){}','f64','Option','None'],
 ['qualified-value-shadow','fn None()->Option<f64>{Option::None::<f64>}fn main(){}','None','Option','Option::None'],
];
export const emptyOptionRefusedSources=[
 ['some','fn empty()->Option<f64>{Some::<f64>(1f64)}fn main(){}'],
 ['implicit-element','fn empty()->Option<f64>{None}fn main(){}'],
 ['wrong-element','fn empty()->Option<bool>{None::<bool>}fn main(){}'],
 ['different-constructor-element','fn empty()->Option<f64>{None::<bool>}fn main(){}'],
 ['qualified-primitive-element','fn empty()->Option<std::primitive::f64>{None::<f64>}fn main(){}'],
 ['multiple-elements','fn empty()->Option<f64,bool>{None::<f64>}fn main(){}'],
 ['bare-nonoption-result','fn empty()->f64{None::<f64>}fn main(){}'],
 ['foreign-result','fn empty()->custom::Option<f64>{None::<f64>}fn main(){}'],
 ['foreign-construction','fn empty()->Option<f64>{custom::None::<f64>}fn main(){}'],
 ['forged-standard-root','fn empty()->crate::std::option::Option<f64>{None::<f64>}fn main(){}'],
 ['import','use std::option::Option;fn empty()->Option<f64>{None::<f64>}fn main(){}'],
 ['imported-none','use std::option::Option::None;fn empty()->Option<f64>{None::<f64>}fn main(){}'],
 ['alias','use std::option::Option as Maybe;fn empty()->Maybe<f64>{None::<f64>}fn main(){}'],
 ['shadowed-option','enum Option{Empty}fn empty()->Option<f64>{None::<f64>}fn main(){}'],
 ['shadowed-element','enum f64{Empty}fn empty()->Option<f64>{None::<f64>}fn main(){}'],
 ['shadowed-standard','mod std{}fn empty()->std::option::Option<f64>{None::<f64>}fn main(){}'],
 ['shadowed-core','mod core{}fn empty()->core::option::Option<f64>{None::<f64>}fn main(){}'],
 ['future-none-shadow','fn empty()->Option<f64>{None::<f64>}fn None()->Option<f64>{Option::None::<f64>}fn main(){}'],
 ['prior-none-shadow','fn None()->Option<f64>{Option::None::<f64>}fn empty()->Option<f64>{None::<f64>}fn main(){}'],
 ['parameter','fn empty(value:f64)->Option<f64>{None::<f64>}fn main(){}'],
 ['body-binding','fn empty()->Option<f64>{let value=1f64;None::<f64>}fn main(){}'],
 ['body-reference','fn empty()->Option<f64>{&None::<f64>}fn main(){}'],
 ['body-call','fn empty()->Option<f64>{None::<f64>()}fn main(){}'],
 ['borrowed-result','fn empty()->&Option<f64>{&None::<f64>}fn main(){}'],
 ['boxed-result','fn empty()->Box<Option<f64>>{Box::new(None::<f64>)}fn main(){}'],
 ['parenthesized-result','fn empty()->(Option<f64>){None::<f64>}fn main(){}'],
 ['named-lifetime',"fn empty<'a>()->Option<f64>{None::<f64>}fn main(){}"],
 ['effectful-main','fn empty()->Option<f64>{None::<f64>}fn main(){println!("observed");}'],
 ['no-implicit-prelude','#![no_implicit_prelude]fn empty()->Option<f64>{None::<f64>}fn main(){}'],
 ['lint-attribute','#[allow(dead_code)]fn empty()->Option<f64>{None::<f64>}fn main(){}'],
 ['duplicate-main','fn empty()->Option<f64>{None::<f64>}fn main(){}fn main(){}'],
 ['nonempty-helper','fn other()->f64{1f64}fn empty()->Option<f64>{None::<f64>}fn main(){}'],
];
export function rustEmptyOptionFixture(){
 const cases=specifications.map(([id,source,functionName,resultPath,constructionPath])=>{
  const packet=createTypedProgramLinks(source,'Rust',{rustEmptyOptionSourceModel:RUST_EMPTY_OPTION_SOURCE_MODEL,provenance:{fixture:id}});
  const proof=expected(source,functionName,resultPath,constructionPath);
  assert.deepEqual(decodeTypedProgramLinks(packet.network,packet.root).declarations.get(functionName).sourceEmptyOption,proof,id+' independent owner oracle');
  const emittedFunctionName=id==='type-and-value-namespace'?'f64_':id==='qualified-value-shadow'?'none':functionName;
  return {id,source,functionName,emittedFunctionName,expected:proof,links:packet.network.toLino(),root:packet.root.asU64()};
 });
 const invalid=emptyOptionRefusedSources.map(([id,source])=>{
  try{createTypedProgramLinks(source,'Rust',{rustEmptyOptionSourceModel:RUST_EMPTY_OPTION_SOURCE_MODEL});}catch(error){return{id,source,kind:error.kind,reason:error.reason,span:error.span??null};}
  throw new Error('source refusal was admitted: '+id);
 });
 return {schemaVersion:1,model:RUST_EMPTY_OPTION_SOURCE_MODEL,type,scope:'Zero-argument empty primitive-option source interpretation only. Empty is a singleton: coherent binding rename, not a fabricated value-changing edit. Runtime modules, Some, null and general optional storage remain outside this model.',cases,invalid};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('../../parity/fixtures/rust-empty-options.lino',import.meta.url),fixture=rustEmptyOptionFixture();
 if(process.argv.includes('--check'))checkProjectData(file,fixture);else writeProjectData(file,fixture);
 console.log('Rust empty option source: '+fixture.cases.length+' independent owner cases and '+fixture.invalid.length+' exact source refusals');
}
