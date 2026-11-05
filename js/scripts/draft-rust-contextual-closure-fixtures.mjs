// Authored source inputs only. No source product or fixture is executed.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,value='7',ret='i64',prefix=''){cases.push({name,module:'contextual_closure_case_'+cases.length,source:prefix+'pub fn result()->'+ret+'{'+body+'}',value,abort:null});}
add('a direct pointer context supplies the original missing parameter and result','let read:fn(i64)->i64=|value|value+1;read(6)');
add('a direct pointer context supplies a missing result after a typed parameter','let read:fn(i64)->i64=|value:i64|value+1;read(6)');
add('a direct pointer context supplies a missing parameter before a written result','let read:fn(i64)->i64=|value|->i64{value+1};read(6)');
add('a direct pointer context retains a block body','let read:fn(i64)->i64=|value|{value+1};read(6)');
add('a direct pointer context retains an explicit terminal return','let read:fn(i64)->i64=|value|{return value+1;};read(6)');
add('a direct pointer context retains guarded return completion','let read:fn(i64)->i64=|value|{if value>0{return value;}7};read(3)','3');
add('a direct pointer context retains its ordinary alternative','let read:fn(i64)->i64=|value|if value>0{value}else{7};read(0)');
add('a zero argument pointer supplies its exact result domain','let read:fn()->i64=||7;read()');
add('a zero argument move pointer keeps its actual invocation','let read:fn()->i64=move ||7;read()');
add('several contextual parameters retain their actual positions','let read:fn(i64,i64)->i64=|first,last|first+last;read(3,4)');
add('mixed written and contextual domains retain their actual positions','let read:fn(i64,i64)->i64=|first:i64,last|first+last;read(3,4)');
add('contextual parameters retain the trailing comma','let read:fn(i64)->i64=|value,|value+1;read(6)');
add('a Boolean pointer retains its actual argument and result','let read:fn(bool)->bool=|flag|!flag;if read(false){7}else{8}');
add('a fixed-width contextual parameter retains its width','let read:fn(i32)->i32=|value|value+1;read(6) as i64');
add('an unsigned contextual parameter retains its sign','let read:fn(u64)->u64=|value|value+1;read(6)','7','u64');
add('a binary64 contextual parameter retains its arithmetic','let read:fn(f64)->f64=|value|value+1.0;read(6.0)','7','f64');
add('an implicit Unit result remains an actual callable result','let step:fn()=||{};step();7');
add('a contextual source parameter shadows an outer binding','let value=99i64;let read:fn(i64)->i64=|value|value;read(7)');
add('a contextual pointer may be copied into an actual local alias','let read:fn(i64)->i64=|value|value+1;let alias=read;alias(6)');
add('a contextual pointer reaches a named callback parameter','let read:fn(i64)->i64=|value|value+1;apply(read,6)','7','i64','fn apply(read:fn(i64)->i64,value:i64)->i64{read(value)}');
add('a contextual pointer can be returned from its declared source function','let read=make();read(6)','7','i64','fn make()->fn(i64)->i64{let read:fn(i64)->i64=|value|value+1;read}');
add('a typed scalar literal supplies its own concrete inferred result','let read=||7i64;read()');
add('a typed scalar literal in a block supplies its concrete result','let read=||{7i64};read()');
add('a terminal typed scalar return supplies its concrete result','let read=||{return 7i64;};read()');
add('a written parameter supplies the concrete identity result','let read=|value:i64|value;read(7)');
add('a block parameter identity supplies its concrete result','let read=|value:i64|{value};read(7)');
add('a typed body local supplies its concrete result','let read=|value:i64|{let last=value;last};read(7)');
add('a Boolean literal supplies its concrete inferred result','let read=||true;if read(){7}else{8}');
add('a Boolean expression supplies its concrete inferred result','let read=|flag:bool|!flag;if read(false){7}else{8}');
add('an immutable scalar capture supplies its concrete inferred result','let value=7i64;let read=||value;read()');
add('a move scalar capture retains its copied source and concrete result','let value=7i64;let read=move ||value;read()');
add('a cast supplies its actual concrete result after arithmetic','let read=|value:i64|(value+1) as i64;read(6)');
add('a known source function supplies its concrete result','let read=||known();read()','7','i64','fn known()->i64{7}');
add('a String byte count supplies its actual unsigned result','let read=||"😀".len();read()','4','usize');
add('the exact vector count supplies its actual unsigned result','let read=||vec![3i64,4i64].len();read()','2','usize');
add('an inferred Unit body keeps its actual discard call','let step=||{};step();7');
const refusals=[
 {name:'an unbound result domain retains the original explicit-type boundary',source:'fn f()->i64{let read=||{7};read()}fn main(){}',kind:'unsupported',reason:'Rust Copy closure: the closure result requires its actual explicit type'},
 {name:'a context never licenses a captured closure to become a function pointer',source:'fn f()->i64{let offset=3i64;let read:fn(i64)->i64=|value|value+offset;read(4)}fn main(){}',kind:'unsupported',reason:'Rust function pointer coercion: capturing closures cannot coerce to function pointers'}
];
const revision='5772a3b9',history=['js/src/translation/rust-copy-closures.js','js/src/translation/rust.js','js/src/translation/rust-copy-closure-rules.js','rust/src/translation/rust/copy_closures.rs','rust/src/translation/rust/items/product_bindings.rs','js/tests/translation-rust-records.test.js'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/rust-contextual-closures.lino',import.meta.url),{schemaVersion:1,model:'ordinary Rust contextual immutable Copy closures 1',cases,refusals,history,source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; source target_pointer_width=64'},references:['https://doc.rust-lang.org/reference/expressions/closure-expr.html'],assertions:['actualDirectFunctionPointerContextOnly','actualWrittenTypesRemainIndependentCheckedConstraints','implicitExpressionAndBlockBodiesUseRealSourceTerms','concreteSourceResultsMaySupplyMissingResultAnnotations','sourceCaptureSelectionAndPointerCoercionRemainChecked','anonymousClosureIdentityAndOriginalUntypedRefusalPreserved','completePredecessorsBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted future controls. Missing annotations come only from an actual direct fn-pointer initializer context or a concrete proven Copy body result. General inference from later calls, contextual named callback arguments and source returns, parenthesized contextual expressions, borrowed/owned/mutable captures and trait/async composition remain obligations.'});
