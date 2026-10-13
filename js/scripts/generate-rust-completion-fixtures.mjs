// Original source pairs and independent results, never regenerated oracles.
// Native original and target compiler execution belongs exclusively to CI.
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {parseRust} from '../src/translation/rust.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {createTypedProgramLinks} from '../src/typed-program-links.js';

export const completionModel='closed guarded function returns 1';
const arrayResultModel='immutable fresh scalar vector results 1';
const pairs=[
 ['simple-guard', 'fn choose(flag:bool)->String{if flag{return String::from("early");}String::from("late")}fn main(){println!("{}",choose(true));println!("{}",choose(false));}',
  '/** @param {boolean} flag */function choose(flag){if(flag)return "early";return "late";}console.log(choose(true));console.log(choose(false));',['early','late']],
 ['floating-guard-chain', 'fn describe(value:f64)->String{if value.is_nan(){return String::from("NaN");}if value.is_infinite(){return String::from("infinite");}String::from("finite")}fn main(){println!("{}",describe(0.0/0.0));println!("{}",describe(1.0/0.0));println!("{}",describe(-1.0/0.0));println!("{}",describe(-0.0));}',
  '/** @param {number} value */function describe(value){if(Number.isNaN(value))return "NaN";if(Math.abs(value)===Infinity)return "infinite";return "finite";}console.log(describe(0/0));console.log(describe(1/0));console.log(describe(-1/0));console.log(describe(-0));',['NaN','infinite','infinite','finite']],
 ['nested-closed-guards', 'fn choose(first:bool,second:bool)->String{if first{if second{return String::from("both");}return String::from("first");}String::from("neither")}fn main(){println!("{}",choose(true,true));println!("{}",choose(true,false));println!("{}",choose(false,true));}',
  '/** @param {boolean} first @param {boolean} second */function choose(first,second){if(first){if(second)return "both";return "first";}return "neither";}console.log(choose(true,true));console.log(choose(true,false));console.log(choose(false,true));',['both','first','neither']],
 ['continuation-outside-shadow-scope', '// π 😀\nfn choose(flag:bool)->String{let word=String::from("outer");if flag{let word=String::from("inner");return word;}word}fn main(){println!("{}",choose(true));println!("{}",choose(false));}',
  '/** @param {boolean} flag */function choose(flag){const word="outer";if(flag){const word="inner";return word;}return word;}console.log(choose(true));console.log(choose(false));',['inner','outer']],
 ['terminal-return', 'fn identity(value:bool)->bool{return value;}fn main(){println!("{}",identity(true));println!("{}",identity(false));}',
  '/** @param {boolean} value */function identity(value){return value;}console.log(identity(true));console.log(identity(false));',['true','false']],
 ['both-branches-return', 'fn choose(flag:bool)->bool{if flag{return true;}else{return false;}}fn main(){println!("{}",choose(true));println!("{}",choose(false));}',
  '/** @param {boolean} flag */function choose(flag){if(flag)return true;else return false;}console.log(choose(true));console.log(choose(false));',['true','false']],
 ['else-if-closed-return', 'fn choose(first:bool,second:bool)->String{if first{return String::from("one");}else if second{return String::from("two");}else{return String::from("zero");}}fn main(){println!("{}",choose(true,false));println!("{}",choose(false,true));println!("{}",choose(false,false));}',
  '/** @param {boolean} first @param {boolean} second */function choose(first,second){if(first)return "one";else if(second)return "two";else return "zero";}console.log(choose(true,false));console.log(choose(false,true));console.log(choose(false,false));',['one','two','zero']],
 ['condition-call-once', 'fn condition()->bool{true}fn choose()->String{if condition(){return String::from("taken");}String::from("fallback")}fn main(){println!("{}",choose());}',
  'function condition(){return true;}function choose(){if(condition())return "taken";return "fallback";}console.log(choose());',['taken']],
 ['abort-continuation', 'fn choose(flag:bool)->String{if flag{return String::from("safe");}panic!("selected failure");}fn main(){println!("{}",choose(true));}',
  '/** @param {boolean} flag */function choose(flag){if(flag)return "safe";throw new Error("selected failure");}console.log(choose(true));',['safe']],
 ['independent-vector-and-completion-profiles', 'fn values(flag:bool)->Vec<f64>{if flag{return vec![];}vec![1f64]}fn main(){println!("{}",values(true).is_empty());println!("{}",values(false).is_empty());}',
  '/** @param {boolean} flag */function values(flag){if(flag)return [];return [1];}console.log(values(true).length===0);console.log(values(false).length===0);',['true','false'],arrayResultModel],
];
const suffix='fn main(){println!("x");}';
const failures=[
 ['return-in-value-block','fn f(flag:bool)->bool{let value={if flag{return true;}false};value}'],
 ['return-in-value-if','fn f(flag:bool)->bool{let value=if flag{return true;}else{false};value}'],
 ['return-expression','fn f(n:u64)->u64{let x=return 1;x}'],
 ['return-in-argument','fn identity(x:bool)->bool{x}fn f()->bool{identity(return true)}'],
 ['return-in-match-arm','fn f(flag:bool)->bool{match flag{true=>{return true;},false=>false}}'],
 ['partial-nested-guard','fn f(first:bool,second:bool)->bool{if first{if second{return true;}}false}'],
 ['mixed-completion-branches','fn f(flag:bool)->bool{if flag{return true;}else{false}}'],
 ['guard-without-continuation','fn f(flag:bool)->bool{if flag{return true;}}'],
 ['guard-value-discarded','fn f(flag:bool)->bool{if flag{true}false}'],
 ['value-conditional-discarded','fn f(flag:bool)->bool{if flag{true}else{false};true}'],
 ['unreachable-after-return','fn f()->bool{return true;false}'],
 ['unreachable-after-closed-if','fn f(flag:bool)->bool{if flag{return true;}else{return false;}true}'],
 ['unreachable-after-abort','fn f()->bool{panic!("stop");true}'],
 ['return-without-value','fn f()->bool{return;}'],
 ['mutable-local','fn f(flag:bool)->bool{let mut value=false;if flag{return true;}value}'],
 ['assignment','fn f(flag:bool)->bool{let value=false;value=true;if flag{return true;}value}'],
 ['loop','fn f(flag:bool)->bool{while flag{return true;}false}'],
 ['borrowed-return','fn f(value:&u64)->&u64{return value;}'],
 ['borrowed-local-result','fn f()->&u64{let value:u64=1;return &value;}'],
 ['lifetime-result','fn f<\'a>(value:&\'a u64)->&\'a u64{return value;}'],
 ['destructor','struct Value;impl Drop for Value{fn drop(&mut self){println!("drop");}}fn f()->bool{return true;}'],
 ['escaping-closure','fn f()->bool{let callback=||return true;callback()}'],
 ['wrong-return-type','fn f(flag:bool)->bool{if flag{return String::from("wrong");}false}'],
 ['unproved-return-call','fn f()->bool{return missing();}'],
];
const legacySources=[
 ['frontend-rust/frontend-rust-negativeEarlyReturn','fn f(n: u64) -> u64 { return n; } fn main() { println!("x"); }'],
 ['frontend-rust/frontend-rust-returnExpression','fn f(n: u64) -> u64 {\n  let x = return 1;\n  x\n}\nfn main() { println!("x"); }\n'],
];
function refusal(id,source,options){try{checkProgram(parseRust(source,options));}catch(error){return{id,source,kind:error.kind,reason:error.reason,span:error.span??null};}throw new Error('unexpected admission: '+id);}
export function rustCompletionFixture(){
 const cases=pairs.map(([id,source,originalJavaScript,output,selectedArray])=>{
  const options={completionModel,...(selectedArray?{arrayResultModel:selectedArray}:{})};
  const surface=parseRust(source,options),program=checkProgram(surface);
  const original=checkProgram(parseJavaScript(originalJavaScript));
  const links=createTypedProgramLinks(source,'Rust',{rustCompletionModel:completionModel,...(selectedArray?{rustArrayResultModel:selectedArray}:{})});
  const emitted=emitRust(program),reverse=emitRust(original);
  return{id,source,originalJavaScript,output,...(selectedArray?{arrayResultModel:selectedArray}:{}),surface:JSON.parse(JSON.stringify(surface)),javascript:emitJavaScript(program).text,rust:emitted.text,reverseRust:reverse.text,rustDefinitions:emitted.definitions.join('\n\n'),reverseRustDefinitions:reverse.definitions.join('\n\n'),main:source.slice(source.lastIndexOf('fn main')),archive:links.network.toLino(),root:links.root.asU64()};
 });
 return{schemaVersion:1,id:'rust-closed-completions',completionModel,premises:['explicit selected source profile','direct function body and conditional branch completions','proved direct return values','scope restored before parsing each enclosing continuation','unreachable source statements are refused'],legacy:legacySources.map(([id,source])=>refusal(id,source)),cases,invalid:failures.map(([id,body])=>refusal(id,body+suffix,{completionModel}))};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const path=new URL('../../parity/fixtures/rust-completions.lino',import.meta.url),value=rustCompletionFixture();
 if(process.argv.includes('--check'))checkProjectData(path,value);else writeProjectData(path,value);
 console.log('Rust completions: '+value.cases.length+' independent source pairs; '+value.invalid.length+' precise refusals; '+value.legacy.length+' preserved default fixtures');
}
