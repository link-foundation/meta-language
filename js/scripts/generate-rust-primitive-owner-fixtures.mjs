import{fileURLToPath}from'node:url';
import{readProjectData,writeProjectData,checkProjectData}from'./project-data.mjs';
import{parseRust}from'../src/translation/rust.js';
import{checkProgram}from'../src/translation/check.js';
import{emitJavaScript}from'../src/translation/emit-javascript.js';
import{emitRust}from'../src/translation/emit-rust.js';
import{createTypedProgramLinks}from'../src/typed-program-links.js';
const model='standard primitive Rust binary64 owner 1',signature='Rust source function signature 1';
const previous=readProjectData(new URL('../../parity/fixtures/rust-float-operations.lino',import.meta.url));
const owners=[['qualified-standard','','std::primitive::f64'],['qualified-core','','core::primitive::f64'],['imported-standard','use std::primitive::f64 as Real;','Real'],['imported-core','use core::primitive::f64 as Float;','Float']];
const operations=['abs','floor','ceil','trunc','is_nan','is_finite','is_infinite'];
export function rustPrimitiveOwnerFixture(){
 const specimens=[];
 for(const[id,prefix,owner]of owners){
  for(const operation of operations){
   const predicate=operation.startsWith('is_'),source=`${prefix}fn apply(value:${owner})->${predicate?'bool':owner}{value.${operation}()}fn main(){}`;
   const observations=predicate?[
    {javascriptArguments:['-0'],expectedBits:String(operation!=='is_nan'&&operation!=='is_infinite')},
    {javascriptArguments:['NaN'],expectedBits:String(operation==='is_nan')},
    {javascriptArguments:['Infinity'],expectedBits:String(operation==='is_infinite')},
    {javascriptArguments:['-Infinity'],expectedBits:String(operation==='is_infinite')},
   ]:previous.cases.find(entry=>entry.id==='primitive-'+operation).observations;
   const expression=operation==='is_nan'?'Number.isNaN(value)':operation==='is_finite'?'Number.isFinite(value)':operation==='is_infinite'?'Math.abs(value)===Infinity':`Math.${operation}(value)`;
   specimens.push({id:id+'-'+operation,source,originalJavaScript:`function apply(value){return ${expression};}`,observations});
  }
  for(const[name,bits]of[['NAN','NaN'],['INFINITY','7ff0000000000000'],['NEG_INFINITY','fff0000000000000']])specimens.push({id:id+'-'+name,source:`${prefix}fn apply()->${owner}{${owner}::${name}}fn main(){}`,originalJavaScript:`function apply(){return ${name==='NAN'?'NaN':name==='INFINITY'?'Infinity':'-Infinity'};}`,observations:[{javascriptArguments:[],expectedBits:bits}]});
 }
 specimens.push({id:'immutable-explicit-dereference',source:'use core::primitive::f64 as Float;fn apply(value:&Float)->Float{(*value).ceil()}fn main(){}',originalJavaScript:'function apply(value){return Math.ceil(value);}',observations:[{javascriptArguments:['-0.5'],expectedBits:'8000000000000000'}]});
 specimens.push({id:'editable-value-body',source:'fn apply(value:std::primitive::f64)->std::primitive::f64{value+1.0}fn main(){}',originalJavaScript:'function apply(value){return value+1;}',observations:[{javascriptArguments:['3'],expectedBits:'4010000000000000'}]});
 const invalid=[
 ['custom-root','mod std{}fn apply(value:std::primitive::f64)->bool{value.is_nan()}fn main(){}'],
 ['custom-type','enum f64{V}fn apply(value:f64)->bool{value.is_nan()}fn main(){}'],
 ['custom-alias','enum Value{V}use crate::Value as Float;fn apply(value:Float)->bool{value.is_nan()}fn main(){}'],
 ['module-constant-owner','fn apply()->f64{std::f64::INFINITY}fn main(){}'],
 ['nonprimitive-qualified','fn apply(value:std::primitive::f32)->bool{value.is_nan()}fn main(){}'],
 ['mutable-reference','fn apply(value:&mut core::primitive::f64)->bool{value.is_nan()}fn main(){}'],
 ['implicit-reference','fn apply(value:&core::primitive::f64)->bool{value.is_nan()}fn main(){}'],
 ['mutable-parameter','fn apply(mut value:core::primitive::f64)->bool{value.is_nan()}fn main(){}'],
 ['constant-call','fn apply()->f64{core::primitive::f64::NAN()}fn main(){}'],
 ['constant-value-shadow','use core::primitive::f64 as Float;fn apply(Float:f64)->f64{Float::NAN}fn main(){}'],
 ['namespace-alias','use core::primitive::f64 as std;fn apply(value:std)->bool{value.is_nan()}fn main(){}'],
 ['duplicate-alias','use core::primitive::f64 as Float;use std::primitive::f64 as Float;fn apply(value:Float)->bool{value.is_nan()}fn main(){}'],
 ['keyword-alias','use core::primitive::f64 as fn;fn apply()->bool{true}fn main(){}'],
 ['unicode-alias','use core::primitive::f64 as 値;fn apply(value:値)->bool{value.is_nan()}fn main(){}'],
 ['named-lifetime-context',"fn apply<'a>(value:&'a core::primitive::f64)->bool{(*value).is_nan()}fn main(){}"],
 ['call-context','fn base(value:core::primitive::f64)->core::primitive::f64{value}fn apply(value:core::primitive::f64)->core::primitive::f64{base(value)}fn main(){}'],
 ['main-effect','fn apply(value:core::primitive::f64)->core::primitive::f64{value}fn main(){println!("{}",apply(1f64));}'],
 ['unsupported-method','fn apply(value:core::primitive::f64)->core::primitive::f64{value.sqrt()}fn main(){}'],
 ['wrong-arity','fn apply(value:core::primitive::f64)->core::primitive::f64{value.floor(1f64)}fn main(){}'],
 ['unproved-literal','fn apply()->core::primitive::f64{1.0.floor()}fn main(){}'],
 ];
 const options={primitiveOwnerModel:model,sourceSignatureModel:signature};
 return {schemaVersion:1,model,signatureModel:signature,scope:'Flat complete primitive leaf programs; exact standard primitive owner reexports, unique ASCII imports, immutable direct or explicitly dereferenced receivers; no named lifetime binders, custom types, calls or effects.',cases:specimens.map(entry=>{
  const surface=parseRust(entry.source,options),program=checkProgram(surface),rust=emitRust(program),archive=createTypedProgramLinks(entry.source,'Rust',{rustPrimitiveOwnerModel:model,rustSourceSignatureModel:signature});
  const replay=checkProgram(parseRust(rust.definitions.join('\n')+'\nfn main(){}'));
  return {...entry,surface:JSON.parse(JSON.stringify(surface)),javascript:emitJavaScript(program).text,rust:rust.text,rustDefinitions:rust.definitions.join('\n'),replayJavaScript:emitJavaScript(replay).text,links:archive.network.toLino(),root:archive.root.asU64()};
 }),invalid:invalid.map(([id,source])=>{
  try{checkProgram(parseRust(source,options));}catch(error){return{id,source,kind:error.kind,reason:error.reason,span:error.span??null};}
  throw new Error('unexpected primitive owner admission '+id);
 })};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('../../parity/fixtures/rust-primitive-owners.lino',import.meta.url),fixture=rustPrimitiveOwnerFixture();
 if(process.argv.includes('--check')){if(!checkProjectData(file,fixture))throw new Error('Rust primitive owner fixtures are stale');}else writeProjectData(file,fixture);
 console.log(`Rust primitive owners: ${fixture.cases.length} complete sources, ${fixture.invalid.length} strict refusals`);
}
