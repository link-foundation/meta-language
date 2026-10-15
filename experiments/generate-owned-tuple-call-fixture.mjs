// Complete closed programs exercise the new initializer; the formatter stays unresolved.
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {readProjectData,writeProjectData,checkProjectData} from '../js/scripts/project-data.mjs';
import {parseRust} from '../js/src/translation/rust.js';
import {checkProgram} from '../js/src/translation/check.js';
import {tokenize} from '../js/src/translation/lexer.js';
import {ownedTupleCallPolicy as policy} from './generate-rust-owned-tuple-call-rules.mjs';
import {withRustOwnedLocalAuthenticSource,RUST_OWNED_LOCAL_AUTHENTIC_SOURCE_COMMIT} from './rust-owned-local-authentic-source.mjs';
const helper='fn seed(text:String,count:i32)->(String,i32){(text,count)}';
const main='fn main(){}';
const cases=[
 {name:'conditional',source:helper+'fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=seed(text,count);if n==0i32{digits=String::from("zero");}digits}'+main,
 observations:[['Ω😀','0',false,'zero'],['Ω😀','-2147483648',true,'Ω😀'],['seed','2147483647',false,'seed']]},
 {name:'forward declaration',source:'fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=seed(text,count);digits=String::from("after");digits}'+helper+main,
 observations:[['before','1',false,'after'],['😀','-1',true,'after']]},
 {name:'ordered moves and joins',source:'fn seed(text:String,count:i32)->(String,i32){(String::from("seed"),count)}fn choose(text:String,count:i32,flag:bool)->String{let saved=String::from("saved");let(mut digits,n)=seed(text,count);if flag{digits=saved;}digits}'+main,
 observations:[['before','1',false,'seed'],['before','1',true,'saved']]},
 {name:'source-free target',source:'fn seed(text:String,count:i32)->(String,i32){(String::from("first"),count)}fn alternate(text:String,count:i32)->(String,i32){(String::from("second"),count)}fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=seed(text,count);if flag{digits=String::from("selected");}digits}'+main,
 observations:[['before','1',false,'first'],['before','1',true,'selected']]},
 {name:'ordered constructor arguments',source:helper+'fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=seed(String::from("argument"),count);if n==0i32{digits=String::from("zero");}digits}'+main,
 observations:[['before','1',false,'argument'],['before','0',true,'zero']]},
];
const invalid=[
 ['unknown target','fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=foreign(text,count);digits=String::from("new");digits}'],
 ['moved input','fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=seed(text,count);digits=String::from("new");text}'],
 ['second String move','fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=seed(text,count);let duplicate=text;digits=String::from("new");duplicate}'],
 ['immutable field replacement','fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=seed(text,count);n=1i32;digits=String::from("new");digits}'],
 ['non-call initializer','fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);digits=String::from("new");digits}'],
 ['borrowed argument','fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=seed(&text,count);digits=String::from("new");digits}'],
 ['wrong argument order','fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=seed(count,text);digits=String::from("new");digits}'],
 ['captured value callee','fn choose(seed:String,count:i32,flag:bool)->String{let(mut digits,n)=seed(seed,count);digits=String::from("new");digits}'],
 ['non-initializer call','fn choose(text:String,count:i32,flag:bool)->String{let saved=seed(text,count);let(mut digits,n)=seed(String::from("x"),count);digits=String::from("new");digits}'],
 ['clone interpretation','fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=seed(text.clone(),count);digits=String::from("new");digits}'],
 ['completion composition','fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=seed(text,count);digits=String::from("new");return digits;}'],
 ['loop composition','fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=seed(text,count);loop{digits=String::from("new");}digits}'],
].map(([name,caller])=>({name,source:helper+caller+main}));
invalid.push({name:'recursive tuple helper',source:'fn seed(text:String,count:i32)->(String,i32){seed(text,count)}fn choose(text:String,count:i32,flag:bool)->String{let(mut digits,n)=seed(text,count);digits=String::from("new");digits}'+main});
export function ownedTupleCallFixture(){
 const refusal=entry=>{try{checkProgram(parseRust(entry.source,{localValueStateModel:policy.model}));}catch(error){return{...entry,kind:error.kind,reason:error.reason,span:error.span??null};}throw new Error('unexpected tuple-call admission '+entry.name);};
 for(const entry of cases)checkProgram(parseRust(entry.source,{localValueStateModel:policy.model}));
 const files=[0,1,2,3].map(index=>`rust/src/link_network_binary_rules/part_${index}.rs`);
 const authentic=withRustOwnedLocalAuthenticSource(files,paths=>{
  const entries=paths.map((path,index)=>{const bytes=readFileSync(path);return{file:files[index],bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')};});
  const complete=entries.map(entry=>entry.source).join('\n'),source=entries[0].source;
  const site=text=>{const start=source.indexOf(text);if(start<0)throw new Error('the actual formatter site changed');return{file:files[0],start,end:start+text.length,originalText:source.slice(start,start+text.length)};};
  const initializer=site('let (mut digits, n) = decimal(&format!("{:e}", x.abs()));');
  const definition=site('fn decimal(text: &str) -> (String, i32)');
  const replacement=site("digits = other.trim_end_matches('0').to_string();");
  const bodyEnd=source.indexOf('/// What console.log prints:');
  const occurrences=tokenize(source,'Rust').tokens.filter(token=>['digits','n'].includes(token.value)&&token.start>=initializer.start&&token.end<bodyEnd).map(token=>({name:token.value,start:token.start,end:token.end}));
  return{gitCommit:RUST_OWNED_LOCAL_AUTHENTIC_SOURCE_COMMIT,files:entries.map(({source,...receipt})=>receipt),combined:{bytes:Buffer.byteLength(complete),sha256:createHash('sha256').update(complete).digest('hex')},initializer,definition,replacement,occurrences,
   selectedRefusal:refusal({name:'all four original formatter files',source:complete}),
   unresolved:['The actual decimal helper takes borrowed str, computes its product through split_once/chars/filter/collect/parse, and is outside the direct owned String/i32 helper interpretation.','The actual initializer formats and borrows a temporary before the call.','The enclosing formatter has an f64 interface and guarded completions; profile composition remains unproved.','Borrowed slices, byte iterators/all, Option branches, parse and remaining standard-library calls require independent source interpretations.']};
 });
 // The complete input is always authenticated and parsed, but not copied into fixture data.
 delete authentic.selectedRefusal.source;
 return{version:1,model:policy.model,cases,refusals:invalid.map(refusal),authentic};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('./owned-tuple-call-fixture.lino',import.meta.url),data=ownedTupleCallFixture();
 if(process.argv.includes('--check'))checkProjectData(file,data);else writeProjectData(file,data);
 console.log(`Owned tuple initializer calls: ${data.cases.length} complete programs, ${data.refusals.length} exact refusals; complete formatter unresolved`);
}
