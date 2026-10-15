// Actual complete source owner refusals before the selected AST erases indirection.
import {fileURLToPath} from 'node:url';
import {readProjectData,writeProjectData,checkProjectData} from '../js/scripts/project-data.mjs';
import {parseRust} from '../js/src/translation/rust.js';
import {checkProgram} from '../js/src/translation/check.js';
const policy=readProjectData(new URL('../js/src/translation/data/rust-owned-local-state.lino',import.meta.url));
const sources=[
 ['boxed mutable result',policy.models.moves,'let(mut digits,n)=(text,count);let boxed=Box::new(digits);digits=String::from("new");boxed'],
 ['double borrowed slot',policy.models.moves,'let(mut digits,n)=(text,count);let borrowed=&&digits;digits=String::from("new");digits'],
 ['boxed immutable result',policy.models.copies,'let boxed=Box::new(text);boxed'],
 ['dereferenced allocation',policy.models.copies,'*Box::new(text)'],
 ['borrowed immutable text',policy.models.copies,'let borrowed=&text;String::from("value")'],
 ['double borrowed immutable text',policy.models.copies,'let borrowed=&&text;String::from("value")'],
];
const methodSources=[
 ['mutable slot string view',policy.models.moves,'let(mut digits,n)=(text,count);digits=String::from("new");digits.as_str()'],
 ['canonical string view',policy.models.copies,'text.as_str()'],
 ['mutable slot to owned',policy.models.moves,'let(mut digits,n)=(text,count);digits=digits.to_owned();digits'],
 ['canonical to owned',policy.models.copies,'text.to_owned()'],
 ['borrowed literal clone',policy.models.copies,'"raw".clone()'],
 ['borrowed literal to owned',policy.models.copies,'"raw".to_owned()'],
 ['named borrowed literal clone',policy.models.copies,'let borrowed="raw";borrowed.clone()'],
];
function readSourceRefusal([name,model,body]){
  const source=`fn bad(text:String,count:i32,flag:bool)->String{${body}}fn main(){}`;
  try{checkProgram(parseRust(source,{localValueStateModel:model}));}catch(error){return{name,model,source,kind:error.kind,reason:error.reason,span:error.span??null};}
  throw new Error('unexpected selected source-operation admission '+name);
}
export function rustOwnedLocalSourceOperationFixture(){
 return {schemaVersion:1,cases:sources.map(readSourceRefusal),borrowedMethodRefusals:methodSources.map(readSourceRefusal)};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('./rust-owned-local-source-operation-fixture.lino',import.meta.url),data=rustOwnedLocalSourceOperationFixture();
 if(process.argv.includes('--check')){if(!checkProjectData(file,data))throw new Error('owned local source-operation fixture is stale');}else writeProjectData(file,data);
}
