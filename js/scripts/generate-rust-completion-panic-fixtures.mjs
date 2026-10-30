// Independent source programs and literal-message observations.
import {fileURLToPath} from 'node:url';
import {readProjectData,checkProjectData,writeProjectData} from './project-data.mjs';
import {parseRust} from '../src/translation/rust.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {createTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
const model='closed guarded function returns 1';
const examples=[['literal-braces','literal {capture} {{pair}}'],['empty-message',''],['escaped-message','line\n"quotes" \\ braces {}'],['unicode-message','π 😀 literal {value}']];
const negatives=[
 ['dynamic-message','let message=String::from("dynamic");panic!("{}",message)'],
 ['converted-message','panic!("{}",String::from("dynamic"))'],
 ['numeric-message','panic!("{}",1)'],
 ['boolean-message','panic!("{}",true)'],
 ['changed-format','panic!("message {}","literal")'],
 ['multiple-slots','panic!("{} {}","first","second")'],
 ['extra-arguments','panic!("{}","first","second")'],
 ['template-binding','let template="{}";panic!(template,"literal")'],
 ['nonpanic-macro','unreachable!("{}","literal")'],
 ['literal-capture-one-argument','panic!("{capture}")'],
];
function refusal(id,source,options){try{checkProgram(parseRust(source,options));}catch(error){return{id,source,kind:error.kind,reason:error.reason,span:error.span??null};}throw new Error('unexpected literal panic admission: '+id);}
export function rustCompletionPanicFixture(){
 const historical=readProjectData(new URL('../../parity/fixtures/rust-formatted-panic-history.lino',import.meta.url));
 const cases=examples.map(([id,message])=>{
  const literal=JSON.stringify(message),source='fn fail(flag:bool)->bool{if flag{return true;}panic!("{}",'+literal+')}fn main(){println!("{}",fail(true));}';
  const originalJavaScript='/** @param {boolean} flag */function fail(flag){if(flag)return true;throw new Error('+literal+');}console.log(fail(true));';
  const links=createTypedProgramLinks(source,'Rust',{rustCompletionModel:model});
  const emitted=emitTypedProgramLinks(links.network,links.root,'Rust');
  return{id,message,source,originalJavaScript,output:['true'],surface:JSON.parse(JSON.stringify(parseRust(source,{completionModel:model}))),javascript:emitTypedProgramLinks(links.network,links.root,'JavaScript').code,rust:emitted.code,reverseRust:emitRust(checkProgram(parseJavaScript(originalJavaScript))).text,archive:links.network.toLino(),root:links.root.asU64()};
 });
 const legacySource='fn f(n: u64) -> u64 {\n  panic!("a {}", n)\n}\nfn main() { println!("x"); }\n';
 return{schemaVersion:1,id:'rust-completion-literal-panic',completionModel:model,cases,legacy:{...historical.legacy,source:legacySource},invalid:negatives.map(([id,body])=>refusal(id,'fn f()->bool{'+body+'}fn main(){println!("x");}',{completionModel:model})),defaultCanonical:historical.defaultCanonical};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('../../parity/fixtures/rust-completion-panic.lino',import.meta.url),value=rustCompletionPanicFixture();
 if(process.argv.includes('--check'))checkProjectData(file,value);else writeProjectData(file,value);
 console.log('Rust completion panic: '+value.cases.length+' independent literal pairs; '+value.invalid.length+' exact closed-profile refusals; former ordinary refusals retained as history');
}
