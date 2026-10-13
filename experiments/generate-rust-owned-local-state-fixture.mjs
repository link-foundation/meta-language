// Bounded JavaScript-first draft; native verification and full formatter support remain open.
import {fileURLToPath} from 'node:url';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData,writeProjectData,checkProjectData} from '../js/scripts/project-data.mjs';
import {parseRust} from '../js/src/translation/rust.js';
import {checkProgram} from '../js/src/translation/check.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../js/src/translation/emit-javascript.js';
import {encodeTypedProgramLinks} from '../js/src/typed-program-links.js';
const policy=readProjectData(new URL('../js/src/translation/data/rust-owned-local-state.lino',import.meta.url));
const suffix='fn main(){}';
const sources=[
 ['select','fn select(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);if flag{digits=String::from("9");}else{digits=String::from("7");}digits}',
 'function select(text,count,flag){let digits=text;const n=count;if(flag){digits="9";}else{digits="7";}return digits;}',[['seed','0',true,'9'],['Ω😀','-2147483648',false,'7'],['seed','2147483647',true,'9']]],
 ['conditional','fn conditional(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);if n==0i32{digits=String::from("zero");}digits}',
 'function conditional(text,count,flag){let digits=text;const n=count;if(n===0n){digits="zero";}return digits;}',[['Ω😀','0',false,'zero'],['Ω😀','-1',true,'Ω😀']]],
 ['ordered','fn ordered(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);digits=String::from("first");digits=String::from("second");digits}',
 'function ordered(text,count,flag){let digits=text;const n=count;digits="first";digits="second";return digits;}',[['seed','1',false,'second']]],
 ['transition','fn transition(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);let previous=digits;digits=String::from("new");previous}',
 'function transition(text,count,flag){let digits=text;const n=count;const previous=digits;digits="new";return previous;}',[['Ω😀','1',false,'Ω😀']]],
 ['hygiene','fn hygiene(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);let ml_owned_local_value0=String::from("constant");if flag{digits=ml_owned_local_value0;}else{digits=String::from("other");}digits}',
 'function hygiene(text,count,flag){let digits=text;const n=count;const ml_owned_local_value0="constant";if(flag){digits=ml_owned_local_value0;}else{digits="other";}return digits;}',[['seed','1',true,'constant'],['seed','1',false,'other']]],
];
export function rustOwnedLocalStateFixture(){
 const cases=sources.map(([name,body,originalJavaScript,observations])=>{
  const source=body+suffix,surface=parseRust(source,{localValueStateModel:policy.models.moves}),program=checkProgram(surface),archive=encodeTypedProgramLinks(program),rustDefinitions=emitRust(program).definitions.join('\n\n'),replaySource=rustDefinitions+'\n'+suffix,replay=checkProgram(parseRust(replaySource,{localValueStateModel:policy.models.copies})),replayArchive=encodeTypedProgramLinks(replay);
  return{name,source,originalJavaScript,observations,surface:JSON.parse(JSON.stringify(surface)),javascript:emitJavaScript(program).text,rustDefinitions,replaySource,replayJavaScript:emitJavaScript(replay).text,links:archive.network.toLino(),root:archive.root.asU64(),replayLinks:replayArchive.network.toLino(),replayRoot:replayArchive.root.asU64()};
 });
 const invalid=[
  ['moved input','fn bad(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);digits=String::from("new");text}'],
  ['second move','fn bad(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);let previous=digits;let again=digits;digits=String::from("new");again}'],
  ['immutable count write','fn bad(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);n=1i32;digits=String::from("new");digits}'],
  ['shadowed binding','fn bad(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);let digits=String::from("shadow");digits}'],
  ['standard owner shadow','fn bad(String:String,count:i32,flag:bool)->String{let(mut digits,n)=(String,count);digits=String::from("new");digits}'],
  ['custom declared owner','enum String{V}fn bad(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);digits=String::from("new");digits}'],
  ['wrong width','fn bad(text:String,count:i64,flag:bool)->String{let(mut digits,n)=(text,count);digits=String::from("new");digits}'],
  ['literal width overflow','fn bad(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,2147483648i32);digits=String::from("new");digits}'],
  ['arithmetic','fn bad(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);if n+1i32==2i32{digits=String::from("new");}digits}'],
  ['source clone in move model','fn bad(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text.clone(),count);digits=String::from("new");digits}'],
  ['reference capture','fn bad(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);let reference=&digits;digits=String::from("new");digits}'],
  ['unknown call','fn bad(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);digits=foreign(digits);digits}'],
  ['formatting','fn bad(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);digits=format!("{}",n);digits}'],
  ['iteration','fn bad(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);for item in digits.chars(){digits=String::from("new");}digits}'],
  ['optional state','fn bad(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);let optional=Some(digits);digits=String::from("new");digits}'],
  ['imported namespace','use std::string::String as Text;fn bad(text:Text,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);digits=String::from("new");digits}'],
  ['main effect','fn bad(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);digits=String::from("new");digits}fn main(){println!("x");}'],
  ['mutable parameter','fn bad(mut text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);digits=String::from("new");digits}'],
 ].map(([name,body])=>{
  const source=body.includes('fn main')?body:body+suffix;
  try{checkProgram(parseRust(source,{localValueStateModel:policy.models.moves}));}catch(error){return{name,source,kind:error.kind,reason:error.reason,span:error.span??null};}
  throw new Error('unexpected owned local state admission '+name);
 });
 const borrowedStringRefusals=[
  ['borrowed replacement',policy.models.moves,'let(mut digits,n)=(text,count);digits="raw";digits'],
  ['borrowed immutable replacement',policy.models.moves,'let(mut digits,n)=(text,count);let copy="raw";digits=copy;digits'],
  ['borrowed result',policy.models.moves,'let(mut digits,n)=(text,count);digits=String::from("x");"raw"'],
  ['unproved literal method',policy.models.moves,'let(mut digits,n)=(text,count);digits="raw".to_string();digits'],
  ['canonical borrowed result',policy.models.copies,'"raw"'],
  ['canonical borrowed immutable',policy.models.copies,'let copy="raw";copy'],
  ['canonical unproved literal method',policy.models.copies,'"raw".to_string()'],
 ].map(([name,model,body])=>{
  const source='fn bad(text:String,count:i32,flag:bool)->String{'+body+'}'+suffix;
  try{checkProgram(parseRust(source,{localValueStateModel:model}));}catch(error){return{name,model,source,kind:error.kind,reason:error.reason,span:error.span??null};}
  throw new Error('unexpected borrowed literal admission '+name);
 });
 const inputValues={validText:['','ASCII\u0000','Ω','😀','\udbff\udfff','\ud7ff\ue000','😀Ω😀'],invalidText:['\ud800','\udfff','x\ud800','\ud800x','\ud800\ud800\udc00','\udc00\ud800','\udbff\ue000']};
 const files=[0,1,2,3].map(index=>{
  const file=`rust/src/link_network_binary_rules/part_${index}.rs`,bytes=readFileSync(new URL('../'+file,import.meta.url));return{file,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')};
 }),source=files.map(file=>file.source).join('\n'),diagnostics=[{}, {completionModel:'closed guarded function returns 1'}].map(options=>{
  try{checkProgram(parseRust(source,options));}catch(error){return{options,kind:error.kind,reason:error.reason,span:error.span??null};}throw new Error('full formatter gap was unexpectedly erased');
 });
 return{schemaVersion:1,models:policy.models,scope:policy.interpretation,cases,invalid,borrowedStringRefusals,inputValues,authentic:{files:files.map(({source,...provenance})=>provenance),diagnostics}};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('./rust-owned-local-state-fixture.lino',import.meta.url),fixture=rustOwnedLocalStateFixture();
 if(process.argv.includes('--check')){if(!checkProjectData(file,fixture))throw new Error('owned local state experimental fixture is stale');}else writeProjectData(file,fixture);
 console.log(`Owned local state: ${fixture.cases.length} complete programs; ${fixture.invalid.length} source refusals; authentic formatter gap retained.`);
}
