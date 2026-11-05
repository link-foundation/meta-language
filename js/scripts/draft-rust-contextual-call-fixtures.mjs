// Future real source inputs; do not parse, translate or execute these fixtures.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,value='7',prefix='',suffix='',ret='i64'){cases.push({name,module:'contextual_call_case_'+cases.length,source:prefix+'pub fn result()->'+ret+'{'+body+'}'+suffix,value,abort:null});}
const apply='fn apply(read:fn(i64)->i64,value:i64)->i64{read(value)}';
add('a preceding real header supplies a callback signature','apply(|value|value+1,6)','7',apply);
add('a following real header supplies a callback signature','apply(|value|value+1,6)','7','',apply);
add('a following real header supplies block callback syntax','apply(|value|{value+1},6)','7','',apply);
add('a following real header supplies a written-result callback','apply(|value|->i64{value+1},6)','7','',apply);
add('a following real header preserves a callback terminal return','apply(|value|{return value+1;},6)','7','',apply);
add('a following real header preserves guarded callback completion','apply(|value|{if value>0{return value;}7},3)','3','',apply);
add('a following real header preserves a move callback without captures','apply(move |value|value+1,6)','7','',apply);
add('a following real header supplies zero argument callback results','apply(||7)','7','','fn apply(read:fn()->i64)->i64{read()}');
add('several actual callback parameters retain their own source domains','apply(|a|a+1,|b|b+2)','7','','fn apply(first:fn(i64)->i64,last:fn(i64)->i64)->i64{first(1)+last(3)}');
add('a callback may have several contextual parameters','apply(|first,last|first+last,3,4)','7','','fn apply(read:fn(i64,i64)->i64,first:i64,last:i64)->i64{read(first,last)}');
add('a Boolean callback retains its own source domain','if apply(|flag|!flag,false){7}else{8}','7','','fn apply(read:fn(bool)->bool,value:bool)->bool{read(value)}');
add('a binary64 callback retains its own source domain','apply(|value|value+1.0,6.0)','7','','fn apply(read:fn(f64)->f64,value:f64)->f64{read(value)}','f64');
add('a Unit callback retains its actual discard invocation','apply(||{});7','7','','fn apply(step:fn()){step();}');
add('an aliased free function transports its actual callback signature','let invoke=apply;invoke(|value|value+1,6)','7','',apply);
add('a parenthesized free function transports its actual callback signature','(apply)(|value|value+1,6)','7','',apply);
add('a returned function pointer transports its actual callback signature','factory()(|value|value+1,6)','7','','fn factory()->fn(fn(i64)->i64,i64)->i64{apply}'+apply);
add('a contextual closure can itself invoke a callback parameter','let invoke=|read:fn(i64)->i64,value:i64|->i64{read(value)};invoke(|value|value+1,6)');
add('a local callable shadows an imported function before selecting argument context','let apply=|read:fn(bool)->bool|->i64{if read(false){7}else{8}};apply(|flag|!flag)','7','use self::other::apply;mod other{pub fn apply(read:fn(i64)->i64,value:i64)->i64{read(value)}}');
add('a qualified module path supplies its complete callback header','helpers::apply(|value|value+1,6)','7','','mod helpers{pub fn apply(read:fn(i64)->i64,value:i64)->i64{read(value)}}');
add('an actual import alias supplies its qualified callback header','invoke(|value|value+1,6)','7','use self::helpers::apply as invoke;','mod helpers{pub fn apply(read:fn(i64)->i64,value:i64)->i64{read(value)}}');
add('an inherent associated function supplies its callback header','Apply::run(|value|value+1,6)','7','struct Apply;','impl Apply{fn run(read:fn(i64)->i64,value:i64)->i64{read(value)}}');
add('nested calls retain independent callback signatures','apply(|value|apply(|last|last+1,value),6)','7','',apply);
add('callback syntax in a later function does not constrain an earlier header inventory','make()','7','','fn make()->i64{apply(|value|value+1,6)}'+apply);
add('a later declared scalar function supplies a concrete body result','let read=||known();read()','7','','fn known()->i64{7}');
add('ordinary strings never close lexical header discovery groups','let marker="}]);{";apply(|value|value+1,6)','7','',apply);
add('raw strings never close lexical header discovery groups','let marker=r#" } ] ) { "#;apply(|value|value+1,6)','7','',apply);
add('comments never close lexical header discovery groups','/* } ] ) { */apply(|value|value+1,6)','7','',apply);
add('large callback bodies cross several increasing header windows','apply(|value|{'+Array.from({length:140},(_,index)=>'let step'+index+'='+index+'i64;').join('')+'value+1},6)','7','',apply);
add('a parenthesized callback keeps its actual parameter context','apply((|value|value+1),6)','7','',apply);
add('several parenthesized callbacks retain their real delimiters','apply((((|value|value+1))),6)','7','',apply);
add('a parenthesized pointer initializer keeps its real signature','let read:fn(i64)->i64=((|value|value+1));read(6)');
add('a parenthesized move callback retains its actual keyword role','apply((move |value|value+1),6)','7','',apply);
add('a raw move identifier retains its source function value role','let read:fn(i64)->i64=r#move;read(6)','7','','fn r#move(value:i64)->i64{value+1}');
const refusals=[{name:'a real callback parameter still cannot accept captures',source:'fn f()->i64{let offset=3i64;apply(|value|value+offset,4)}'+apply+'fn main(){}',kind:'unsupported',reason:'Rust function pointer coercion: capturing closures cannot coerce to function pointers'}];
const headerCases=[
 {name:'one complete group',kinds:['punct','atom','punct'],values:['{','value','}'],end:3},
 {name:'nested delimiters stay ordered',kinds:Array(8).fill('punct'),values:['{','(','[','{','}',']',')','}'],end:8},
 {name:'string delimiters are atomic',kinds:['punct','string','punct'],values:['{','}]){','}'],end:3},
 {name:'an incomplete window requests more tokens',kinds:['punct','punct'],values:['{','('],end:-2},
 {name:'end of source refuses incomplete groups',kinds:['punct','eof'],values:['{',''],end:-1},
 {name:'mismatched groups refuse',kinds:['punct','punct','punct'],values:['{','[','}'],end:-1},
 {name:'empty input refuses',kinds:[],values:[],end:-1},
 {name:'different fact column lengths refuse',kinds:['punct'],values:['{','}'],end:-1},
 {name:'another root delimiter cannot supply a function body',kinds:['punct','punct'],values:['(',')'],end:-1}
];
const parenthesisCases=[{syntax:['(','|'],depth:1},{syntax:['(','(','||'],depth:2},{syntax:['(','move'],depth:1},{syntax:['|'],depth:-1},{syntax:['move'],depth:-1},{syntax:['('],depth:-2},{syntax:['(',''],depth:-1},{syntax:[],depth:-2}];
for(const runtime of ['js','rust'])writeProjectData(new URL('../../'+runtime+'/src/data/rust-declaration-header-controls.lino',import.meta.url),{schemaVersion:1,headerCases,parenthesisCases,boundary:'Unexecuted token-group source controls; no behavior observations.'});
const revision='bd9fee37',history=['js/src/translation/rust.js','js/src/translation/rust-copy-closures.js','js/src/translation/rust-contextual-closure-rules.js','rust/src/translation/rust_contextual_closure_rules.rs','rust/src/translation/rust_contextual_closure_rules/part_1.rs','rust/src/translation/rust.rs','rust/src/translation/rust/items.rs','rust/src/translation/rust/items/functions.rs','rust/src/translation/rust/copy_closures.rs','rust/src/translation/rust/expressions.rs','rust/src/translation/rust/expressions/paths.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/rust-contextual-calls.lino',import.meta.url),{schemaVersion:1,model:'ordinary Rust complete declaration headers and actual callback contexts 1',cases,refusals,headerCases,history,source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},references:['https://doc.rust-lang.org/reference/expressions/closure-expr.html','https://doc.rust-lang.org/reference/names/scopes.html'],assertions:['completeOrdinarySourceHeadersBeforeBodyInference','privateHeaderInventoryNeverEmittedAsFunctionSemantics','everyOriginalBodyParsedAndCheckedBeforeReturning','actualNamedComputedAndLocalCallableSignatureContexts','qualifiedImportsAndSourceLocalShadowsRemainDistinct','atomicStringRawStringCommentAndCompleteDelimiterBoundaries','sourceCapturePointerCoercionAndWrittenTypesRemainChecked','completePredecessorsBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted future controls. Header discovery runs only in the ordinary existing source interpretation. Every final body still uses complete existing syntax/type/ownership passes; a private header inventory is never a semantic translation result. Generic trait callbacks, outer-context return inference, arbitrary contextual expressions, out-of-line modules and complete ownership remain obligations.'});
