import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks} from '../src/typed-program-links.js';
export const RUST_PURE_UNIT_CALL_MODEL='closed pure Rust unit calls 1';
const specifications=[
 ['implicit-empty','fn empty(){}fn main(){}',['empty'],[]],
 ['implicit-unit','fn empty(){()}fn main(){empty();}',['empty'],['empty']],
 ['explicit-empty','fn empty()->(){}fn main(){empty();}',['empty'],['empty']],
 ['explicit-unit','fn empty()->(){()}fn main(){crate::empty();}',['empty'],['empty']],
 ['ordered-repeated','fn alpha(){}fn beta()->(){()}fn main(){alpha();crate::beta();alpha();}',['alpha','beta'],['alpha','beta','alpha']],
 ['main-first','fn main(){last();}fn last(){}',['last'],['last']],
 ['uncalled-owner','fn first(){}fn unused(){}fn main(){first();}',['first','unused'],['first']],
 ['temporary-capture','fn unit_call_result(){}fn main(){unit_call_result();unit_call_result();}',['unit_call_result'],['unit_call_result','unit_call_result']],
 ['empty-program','fn main(){}',[],[]],
 ['layout','fn empty ( ) -> ( ) { ( ) }\nfn main ( ) { crate :: empty ( ) ; }',['empty'],['empty']],
 ['comments','// unit helper\nfn alpha(/* no parameters */){/* no effects */()}\nfn main(){/* owned call */alpha();}',['alpha'],['alpha']],
 ['ordinary-underscore','fn _empty(){}fn main(){_empty();}',['_empty'],['_empty']],
];
export const pureUnitRefusedSources=[
 ['body-call','fn empty(){other();}fn other(){}fn main(){}'],
 ['body-let','fn empty(){let value=();}fn main(){}'],
 ['unit-statement','fn empty(){();}fn main(){}'],
 ['unit-return','fn empty(){return ();}fn main(){}'],
 ['body-output','fn empty(){println!("effect");}fn main(){}'],
 ['body-panic','fn empty(){panic!("effect");}fn main(){}'],
 ['body-loop','fn empty(){loop{}}fn main(){}'],
 ['body-reference','fn empty(){&()}fn main(){}'],
 ['body-product','fn empty(){((),)}fn main(){}'],
 ['parameter','fn empty(value:())->(){}fn main(){}'],
 ['lifetime',"fn empty<'a>(){}fn main(){}"],
 ['borrowed-result','fn empty()->&(){}fn main(){}'],
 ['nonunit-result','fn empty()->bool{true}fn main(){}'],
 ['main-output','fn empty(){}fn main(){println!("effect");}'],
 ['main-binding','fn empty(){}fn main(){let value=();}'],
 ['main-argument','fn empty(){}fn main(){empty(());}'],
 ['main-return','fn empty(){}fn main(){return;}'],
 ['main-result','fn empty(){}fn main()->(){}'],
 ['main-tail-call','fn empty(){}fn main(){empty()}'],
 ['main-recursion','fn empty(){}fn main(){main();}'],
 ['unknown-target','fn empty(){}fn main(){foreign();}'],
 ['qualified-foreign','fn empty(){}fn main(){other::empty();}'],
 ['standard-library','fn empty(){}fn main(){std::env::args();}'],
 ['alias','use crate::empty as other;fn empty(){}fn main(){other();}'],
 ['module','mod inner{fn empty(){}}fn main(){}'],
 ['constant','const EMPTY:()=();fn main(){}'],
 ['unsafe','unsafe fn empty(){}fn main(){}'],
 ['visibility','pub fn empty(){}fn main(){}'],
 ['attribute','#[allow(dead_code)]fn empty(){}fn main(){}'],
 ['inner-attribute','#![no_std]fn empty(){}fn main(){}'],
 ['duplicate-main','fn empty(){}fn main(){}fn main(){}'],
 ['duplicate-helper','fn empty(){}fn empty(){}fn main(){}'],
 ['missing-main','fn empty(){}'],
 ['reserved-translator','fn ml_main(){}fn main(){ml_main();}'],
 ['reserved-2024','fn gen(){}fn main(){gen();}'],
 ['unicode-profile','fn κενό(){}fn main(){κενό();}'],
 ['thread-wrapper','fn empty(){}fn main(){std::thread::Builder::new().spawn(move||empty());}'],
 ['exit-wrapper','fn empty(){}fn main(){std::process::exit(101);}'],
];
const containerRoles=[
 ['items',['items']],['effects',['main','effects']],['parameters',['declarations','empty','params']],
 ['module-path',['declarations','empty','modulePath']],['arguments',['main','effects',0,'value','args']],
 ['imports',['imports']],['externals',['externals']],['declaration-map',['declarations']],
];
export const pureUnitContainerMutations=containerRoles.flatMap(([role,path])=>[
 ...['named','hidden','symbol'].map(mode=>({id:role+'-'+mode,role,path,mode,field:role==='declaration-map'?'opaqueBinding':'borrowEscape'})),
 ...(role==='declaration-map'?[]:['sparse','accessor','hidden-index','prototype'].map(mode=>({id:role+'-'+mode,role,path,mode,field:'0'}))),
]);
export function applyPureUnitContainerMutation(program,mutation){
 for(const name of ['imports','externals'])if(!Object.hasOwn(program,name))program[name]=[];
 let container=program;
 for(const part of mutation.path)container=container instanceof Map?Map.prototype.get.call(container,part):container[part];
 const value=mutation.role==='declaration-map'?Map.prototype.get.call(program.declarations,'empty'):true;
 if(mutation.mode==='named')container[mutation.field]=value;
 else if(mutation.mode==='hidden')Object.defineProperty(container,mutation.field,{value,enumerable:false});
 else if(mutation.mode==='symbol')container[Symbol.for(mutation.field)]=value;
 else if(mutation.mode==='sparse'){if(!container.length)container.length=1;else delete container[0];}
 else if(mutation.mode==='accessor')Object.defineProperty(container,'0',{enumerable:true,get(){throw Error('an unrepresented accessor was executed');}});
 else if(mutation.mode==='hidden-index')Object.defineProperty(container,'0',{value:container[0]??true,enumerable:false});
 else if(mutation.mode==='prototype')Object.setPrototypeOf(container,Object.create(Array.prototype));
 else throw Error('unknown pure Unit container mutation');
 return program;
}
export function rustPureUnitCallFixture(){
 const cases=specifications.map(([id,source,names,calls])=>{
  const packet=createTypedProgramLinks(source,'Rust',{rustPureUnitCallModel:RUST_PURE_UNIT_CALL_MODEL,provenance:{fixture:id}});
  const program=decodeTypedProgramLinks(packet.network,packet.root);
  assert.deepEqual([...program.declarations.keys()],names,id+' actual declarations');
  assert.deepEqual(program.main.effects.map(effect=>effect.value.fn),calls,id+' ordered call oracle');
  for(const name of names){const declaration=program.declarations.get(name);assert.deepEqual(declaration.ret,{kind:'unit'});assert.deepEqual(declaration.body,{k:'unit',type:{kind:'unit'}});}
  return {id,source,names,calls,output:'',links:packet.network.toLino(),root:packet.root.asU64()};
 });
 const invalid=pureUnitRefusedSources.map(([id,source])=>{
  try{createTypedProgramLinks(source,'Rust',{rustPureUnitCallModel:RUST_PURE_UNIT_CALL_MODEL});}catch(error){return{id,source,kind:error.kind,reason:error.reason,span:error.span??null};}
  throw Error('source refusal was admitted: '+id);
 });
 const mutationSource='fn empty(){}fn main(){empty();}';
 const containerMutations=pureUnitContainerMutations.map(mutation=>{
  const packet=createTypedProgramLinks(mutationSource,'Rust',{rustPureUnitCallModel:RUST_PURE_UNIT_CALL_MODEL}),program=decodeTypedProgramLinks(packet.network,packet.root);
  applyPureUnitContainerMutation(program,mutation);
  try{encodeTypedProgramLinks(program);}catch(error){return {...mutation,kind:error.kind,reason:error.reason};}
  throw Error('container metadata was erased: '+mutation.id);
 });
 return {schemaVersion:1,model:RUST_PURE_UNIT_CALL_MODEL,scope:'Flat owned zero-argument Unit helpers with empty or Unit bodies and ordered owned call statements in main. No public interfaces, imports, runtime attributes, effects, threads, process exit or full generated wrapper replay.',cases,invalid,mutationSource,containerMutations};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('../../parity/fixtures/rust-pure-unit-calls.lino',import.meta.url),fixture=rustPureUnitCallFixture();
 if(process.argv.includes('--check'))checkProjectData(file,fixture);else writeProjectData(file,fixture);
 console.log('Rust pure Unit calls: '+fixture.cases.length+' actual sources and '+fixture.invalid.length+' exact source refusals');
}
