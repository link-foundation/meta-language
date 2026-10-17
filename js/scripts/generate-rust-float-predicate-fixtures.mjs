// Original source pairs and independent boolean oracles; native source and
// emitted Rust execute only in CI. This does not claim full generated reimport.
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { createTypedProgramLinks } from '../src/typed-program-links.js';

const values=['0.0','-0.0','42.5','-42.5','5e-324','1.7976931348623157e308','0.0/0.0','1.0/0.0','-1.0/0.0'];
const classification=['false true false','false true false','false true false','false true false','false true false','false true false','true false false','false false true','false false true'];
const definitions='fn describe(x:f64)->String{format!("{} {} {}",x.is_nan(),x.is_finite(),x.is_infinite())}';
const javascript='/** @param {number} x */ function describe(x){return String(Number.isNaN(x))+" "+String(Number.isFinite(x))+" "+String(Math.abs(x)===Infinity);}';
const positives=[
 ['all-binary64-classes',definitions+' fn main(){'+values.map(value=>'println!("{}",describe('+value+'));').join('')+'}',javascript+values.map(value=>'console.log(describe('+value+'));').join(''),classification],
 ['explicit-f64-literal','fn test()->bool{1.0f64.is_finite()} fn main(){println!("{}",test());}','function test(){return Number.isFinite(1);}console.log(test());',['true']],
 ['typed-negative-zero','fn test()->bool{let value:f64=-0.0;value.is_finite()} fn main(){println!("{}",test());}','function test(){const value=-0;return Number.isFinite(value);}console.log(test());',['true']],
 ['receiver-call-once','fn provider()->f64{1.0/0.0} fn test()->bool{provider().is_infinite()} fn main(){println!("{}",test());}','function provider(){return 1/0;}function test(){return Math.abs(provider())===Infinity;}console.log(test());',['true']],
 ['qualified-function-origin','mod values{pub fn test(value:f64)->bool{value.is_finite()}} fn main(){println!("{}",values::test(1.0));}','/** @param {number} value */function test(value){return Number.isFinite(value);}console.log(test(1));',['true']],
 ['explicit-borrow-dereference','fn test(value:&f64)->bool{(*value).is_finite()} fn main(){let value:f64=-0.0;println!("{}",test(&value));}','/** @param {number} value */function test(value){return Number.isFinite(value);}console.log(test(-0));',['true']],
 ['free-function-name-is-not-method-owner','fn is_nan(value:f64)->bool{true} fn test(value:f64)->bool{value.is_nan()} fn main(){println!("{}",test(0.0));}','/** @param {number} value */function is_nan(value){return true;}/** @param {number} value */function test(value){return Number.isNaN(value);}console.log(test(0));',['false']],
];
const main=' fn main(){println!("x");}';
const negatives=[
 ['f32-receiver','fn test(value:f32)->bool{value.is_nan()}'],
 ['f32-suffix','fn test()->bool{1.0f32.is_finite()}'],
 ['integer-receiver','fn test(value:u64)->bool{value.is_finite()}'],
 ['string-receiver','fn test(value:String)->bool{value.is_nan()}'],
 ['custom-receiver','enum Value{V} fn test(value:Value)->bool{value.is_infinite()}'],
 ['shadowed-primitive','enum f64{V} fn test(value:f64)->bool{value.is_nan()}'],
 ['aliased-custom-primitive','enum Value{V} use crate::Value as f64; fn test(value:f64)->bool{value.is_finite()}'],
 ['module-shadowed-primitive','mod f64{} fn test(value:f64)->bool{value.is_infinite()}'],
 ['unproven-float-literal','fn test()->bool{1.0.is_nan()}'],
 ['unproven-float-binding','fn test()->bool{let value=1.0;value.is_finite()}'],
 ['borrowed-receiver-without-dereference','fn test(value:&f64)->bool{value.is_nan()}'],
 ['nan-wrong-arity','fn test(value:f64)->bool{value.is_nan(1.0)}'],
 ['finite-wrong-arity','fn test(value:f64)->bool{value.is_finite(1.0)}'],
 ['infinite-wrong-arity','fn test(value:f64)->bool{value.is_infinite(1.0)}'],
];
export function rustFloatPredicateFixture(){
 const cases=positives.map(([id,source,originalJavaScript,output])=>{
  const surface=parseRust(source),program=checkProgram(surface),original=checkProgram(parseJavaScript(originalJavaScript));
  const links=createTypedProgramLinks(source,'Rust');
  return{id,source,originalJavaScript,output,surface:JSON.parse(JSON.stringify(surface)),javascript:emitJavaScript(program).text,rust:emitRust(program).text,
   reverseRust:emitRust(original).text,archive:links.network.toLino(),root:links.root.asU64()};
 });
 const invalid=negatives.map(([id,body])=>{const source=body+main;try{checkProgram(parseRust(source));}catch(error){return{id,source,kind:error.kind,reason:error.reason,span:error.span??null};}throw new Error('unexpected predicate admission: '+id);});
 return{schemaVersion:1,id:'rust-float-predicates',premises:['explicit primitive binary64 source owner evidence','each method receives zero arguments','the receiver occurs exactly once in the shared linked expression template'],cases,invalid};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const value=rustFloatPredicateFixture();const path=new URL('../../parity/fixtures/rust-float-predicates.lino',import.meta.url);
 if(process.argv.includes('--check'))checkProjectData(path,value);else writeProjectData(path,value);
 console.log('Rust float predicates: '+value.cases.length+' independent source pairs; '+value.invalid.length+' exact refusals');
}
