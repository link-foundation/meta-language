// Construct literal originals/contracts only. No source or assertion is parsed.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,expected,owner='u32',helpers=''){
 const module='lazy_lookup_case_'+cases.length,source=helpers+'pub fn select()->'+owner+'{'+body+'}pub fn result()->'+owner+'{select()}fn main(){}';
 const canonical=owner==='usize'?'u64':owner,contract={kind:'primitive',path:[owner],arguments:[]};
 cases.push({name,module,source,expected:String(expected),value:String(expected),abort:null,resultType:owner,
  checks:['const _: fn()->'+owner+'=CASE::select;','const _: fn()->'+owner+'=CASE::result;'],
  canonicalChecks:['const _: fn()->'+canonical+'=CASE::select;','const _: fn()->'+canonical+'=CASE::result;'],
  sourceContracts:[{name:'select',parameters:[],result:contract},{name:'result',parameters:[],result:contract}]});
}
for(const owner of ['u8','i32','u32','usize','i128','u128'])for(const method of ['get','first','last'])for(const present of [false,true]){
 const query='values.'+method+'('+(method==='get'?'0usize':'')+').copied().unwrap_or_else(||->'+owner+'{37'+owner+'})';
 add(owner+' lazy '+method+' '+(present?'present':'absent'),'let values:Vec<'+owner+'>=vec!['+(present?'7'+owner:'')+'];'+query,present?'7':'37',owner);
}
for(const method of ['get','first','last'])for(const present of [false,true]){
 add('captured Copy fallback '+method+' '+(present?'present':'absent'),'let extra:u32=37u32;let values:Vec<u32>=vec!['+(present?'7u32':'')+'];values.'+method+'('+(method==='get'?'0usize':'')+').copied().unwrap_or_else(||->u32{extra})',present?'7':'37');
}
add('empty last lazy fallback never subtracts from zero','let values:Vec<u32>=vec![];values.last().copied().unwrap_or_else(||->u32{37u32})',37);
add('maximum usize misses before lazy fallback','let values:Vec<u32>=vec![7u32];values.get(18446744073709551615usize).copied().unwrap_or_else(||->u32{37u32})',37);
add('present lookup does not invoke a failing fallback','let values:Vec<u32>=vec![7u32];values.first().copied().unwrap_or_else(||->u32{panic!("unused fallback")})',7);
add('a borrowed slice retains lazy scalar fallback','let owned:Vec<u32>=vec![];let values:&[u32]=&owned;values.first().copied().unwrap_or_else(||->u32{37u32})',37);
add('cloned scalar lookup keeps lazy fallback','let values:Vec<u32>=vec![];values.first().cloned().unwrap_or_else(||->u32{37u32})',37);
add('boolean lazy fallback retains false as a present value','let values:Vec<bool>=vec![false];values.first().copied().unwrap_or_else(||->bool{true})','false','bool');
add('Unit lazy lookup retains a complete Unit result','let values:Vec<()>=vec![];values.first().copied().unwrap_or_else(||->(){()});true','true','bool');
for(const present of [false,true])add('binary64 lazy selection retains zero sign '+present,
 'let actual=chosen();actual==0.0f64&&1.0f64/actual=='+(present?'-':'')+'1.0f64/0.0f64','true','bool',
 'fn chosen()->f64{let values:Vec<f64>=vec!['+(present?'-0.0f64':'')+'];values.first().copied().unwrap_or_else(||->f64{0.0f64})}');
for(const consumer of ['is_some_and','is_none_or'])for(const method of ['get','first','last'])for(const value of [null,1,7]){
 const present=value!==null,expected=present?value>2:consumer==='is_none_or';
 add(consumer+' '+method+' '+(present?value:'absent'),'let values:Vec<u32>=vec!['+(present?value+'u32':'')+'];values.'+method+'('+(method==='get'?'0usize':'')+').copied().'+consumer+'(|value:u32|->bool{value>2u32})',expected,'bool');
}
for(const consumer of ['is_some_and','is_none_or']){
 add(consumer+' retains immutable predicate capture','let minimum:u32=2u32;let values:Vec<u32>=vec![7u32];values.first().copied().'+consumer+'(|value:u32|->bool{value>minimum})',true,'bool');
 add(consumer+' absent branch never calls a failing predicate','let values:Vec<u32>=vec![];values.last().copied().'+consumer+'(|value:u32|->bool{panic!("unused predicate")})',consumer==='is_none_or','bool');
 add(consumer+' cloned scalar predicate','let values:Vec<u32>=vec![7u32];values.first().cloned().'+consumer+'(|value:u32|->bool{value>2u32})',true,'bool');
 add(consumer+' retains a shadowing predicate parameter','let values:Vec<u32>=vec![7u32];values.first().copied().'+consumer+'(|values:u32|->bool{values>2u32})',true,'bool');
}
for(const method of ['get','first','last'])for(const present of [false,true])for(const expression of ['||37','||{37}','(||37)'])
 add('contextual lazy '+method+' '+present+' '+expression,'let values:Vec<u32>=vec!['+(present?'7u32':'')+'];values.'+method+'('+(method==='get'?'0usize':'')+').copied().unwrap_or_else('+expression+')',present?7:37);
for(const consumer of ['is_some_and','is_none_or'])for(const method of ['get','first','last'])for(const value of [null,1,7])for(const expression of ['|value|value>2','|value|{value>2}'])
 add('contextual '+consumer+' '+method+' '+value+' '+expression,'let values:Vec<u32>=vec!['+(value===null?'':value+'u32')+'];values.'+method+'('+(method==='get'?'0usize':'')+').copied().'+consumer+'('+expression+')',value===null?consumer==='is_none_or':value>2,'bool');
for(const consumer of ['is_some_and','is_none_or']){
 add('contextual moving Copy capture '+consumer,'let minimum:u32=2;let values:Vec<u32>=vec![7u32];values.first().copied().'+consumer+'(move|value|value>minimum)',true,'bool');
 add('contextual standard predicate item '+consumer,'let values:Vec<i8>=vec![-7i8];values.first().copied().'+consumer+'(i8::is_negative)',true,'bool');
}
add('contextual predicate with only parameter type written','let values:Vec<u32>=vec![7u32];values.first().copied().is_some_and(|value:u32|value>2)',true,'bool');
add('contextual predicate with only result type written','let values:Vec<u32>=vec![7u32];values.first().copied().is_some_and(|value|->bool{value>2})',true,'bool');
add('contextual standard lazy default item','let values:Vec<u32>=vec![];values.first().copied().unwrap_or_else(u32::default)',0);
add('contextual pointer-sized fallback retains its actual source owner','let values:Vec<usize>=vec![];values.first().copied().unwrap_or_else(||37)',37,'usize');
for(const consumer of ['is_some_and','is_none_or'])for(const present of [false,true])for(const expression of ['|_|true','|_:u32|true','|_|->bool{true}'])
 add('wildcard '+consumer+' '+present+' '+expression,'let values:Vec<u32>=vec!['+(present?'7u32':'')+'];values.first().copied().'+consumer+'('+expression+')',present||consumer==='is_none_or','bool');
add('a wildcard predicate does not shadow an actual captured value','let value:u32=9;let values:Vec<u32>=vec![7u32];values.first().copied().is_some_and(|_|value==9)',true,'bool');
add('a wildcard predicate retains its Unit argument','let values:Vec<()>=vec![()];values.first().copied().is_some_and(|_|true)',true,'bool');
add('repeated wildcard parameters keep arity without duplicate bindings','invoke(|_,_|true)',true,'bool','fn invoke(callback:fn(u32,u32)->bool)->bool{callback(1u32,2u32)}');
const printedSources=[
 {name:'present lookup captures receiver and index but does not invoke fallback',source:'pub fn receiver()->Vec<u32>{println!("receiver");vec![7u32]}pub fn index()->usize{println!("index");0usize}pub fn fallback()->u32{println!("fallback");37u32}fn main(){println!("{}",receiver().get(index()).copied().unwrap_or_else(||->u32{fallback()}));}',lines:['receiver','index','7']},
 {name:'absent lookup invokes fallback after receiver and index once',source:'pub fn receiver()->Vec<u32>{println!("receiver");vec![]}pub fn index()->usize{println!("index");0usize}pub fn fallback()->u32{println!("fallback");37u32}fn main(){println!("{}",receiver().get(index()).copied().unwrap_or_else(||->u32{fallback()}));}',lines:['receiver','index','fallback','37']}
];
for(const consumer of ['is_some_and','is_none_or'])for(const present of [false,true])printedSources.push({
 name:consumer+' captures receiver and index and invokes a present predicate once '+present,
 source:'pub fn receiver()->Vec<u32>{println!("receiver");vec!['+(present?'7u32':'')+']}pub fn index()->usize{println!("index");0usize}pub fn predicate(value:u32)->bool{println!("predicate");value>2u32}fn main(){println!("{}",receiver().get(index()).copied().'+consumer+'(|value:u32|->bool{predicate(value)}));}',
 lines:['receiver','index',...(present?['predicate']:[]),String(present||consumer==='is_none_or')]});
for(const present of [false,true])printedSources.push({name:'contextual predicate keeps independent call order '+present,
 source:'pub fn receiver()->Vec<u32>{println!("receiver");vec!['+(present?'7u32':'')+']}pub fn index()->usize{println!("index");0usize}pub fn predicate(value:u32)->bool{println!("predicate");value>2u32}fn main(){println!("{}",receiver().get(index()).copied().is_none_or(|value|predicate(value)));}',
 lines:['receiver','index',...(present?['predicate']:[]),'true']});
printedSources.push({name:'wildcard callback arguments retain both eager input effects',source:'pub fn first()->u32{println!("first");1u32}pub fn second()->u32{println!("second");2u32}pub fn invoke(callback:fn(u32,u32)->bool)->bool{callback(first(),second())}fn main(){println!("{}",invoke(|_,_|true));}',lines:['first','second','true']});
const refusals=[
 {name:'lazy fallback preserves pointer-sized result ownership',source:'pub fn select()->usize{let values:Vec<usize>=vec![];values.first().copied().unwrap_or_else(||->u64{1u64})}fn main(){}',kind:'unsupported',reason:'Rust integer source owner: distinct pointer-sized and fixed-width integer source types need an explicit cast',originalRustAccepted:false},
 {name:'lazy fallback requires zero arguments',source:'pub fn select()->u32{let values:Vec<u32>=vec![];values.first().copied().unwrap_or_else(|value:u32|->u32{value})}fn main(){}',kind:'unsupported',reason:'Rust array lookup lazy fallback: a lazy lookup fallback needs a known direct zero-argument callable result',originalRustAccepted:false},
 {name:'borrowed optional results retain their separate consumer obligation',source:'pub fn select(){let values:Vec<u32>=vec![];values.first().unwrap_or_else(||->u32{1u32});}fn main(){}',kind:'unsupported',reason:'Rust array lookup Option: an immediate standard presence or copied-value consumer is required',originalRustAccepted:false}
];
refusals.push({name:'lookup predicates retain their Boolean result',source:'pub fn select()->bool{let values:Vec<u32>=vec![7u32];values.first().copied().is_some_and(|value:u32|->u32{value})}fn main(){}',kind:'unsupported',reason:'Rust array lookup predicate: a lookup predicate needs a known direct one-argument callable with a Boolean result',originalRustAccepted:false},
 {name:'lookup predicate parameters preserve actual pointer owners',source:'pub fn select()->bool{let values:Vec<usize>=vec![7usize];values.first().copied().is_none_or(|value:u64|->bool{value>2u64})}fn main(){}',kind:'unsupported',reason:'Rust integer source owner: distinct pointer-sized and fixed-width integer source types need an explicit cast',originalRustAccepted:false});
writeProjectData(new URL('../../parity/fixtures/rust-array-lookup-lazy.lino',import.meta.url),{
 schemaVersion:1,retainSourceTypeOwners:true,model:null,cases,printedSources,refusals,
 malformed:[{name:'lazy fallback retains its one argument',source:'pub fn select()->u32{let values:Vec<u32>=vec![];values.first().copied().unwrap_or_else()}fn main(){}'},
 {name:'contextual lazy fallback does not erase literal overflow',source:'pub fn select()->u8{let values:Vec<u8>=vec![];values.first().copied().unwrap_or_else(||300)}fn main(){}'},
 {name:'a wildcard is not an expression binding',source:'pub fn select()->bool{let values:Vec<u32>=vec![7u32];values.first().copied().is_some_and(|_|_==0)}fn main(){}'},
 {name:'wildcards do not erase predicate arity',source:'pub fn select()->bool{let values:Vec<u32>=vec![7u32];values.first().copied().is_some_and(|_:u32,_:u32|true)}fn main(){}'},
 {name:'contextual predicate typing does not turn an integer into Boolean',source:'pub fn select()->bool{let values:Vec<u32>=vec![7u32];values.first().copied().is_some_and(|value|value)}fn main(){}'}],
 source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64'},
 lazySignatures:[{kind:'function',parameters:0,wrappers:0,knownResult:true,expected:''},
  ...[{kind:'string'},{parameters:1},{wrappers:1},{knownResult:false}].map(edit=>({kind:'function',parameters:0,wrappers:0,knownResult:true,...edit,expected:'a lazy lookup fallback needs a known direct zero-argument callable result'}))],
 predicateSignatures:[{kind:'function',parameters:1,wrappers:0,knownParameter:true,result:'bool',knownResult:true,expected:''},
  ...[{kind:'string'},{parameters:0},{parameters:2},{wrappers:1},{knownParameter:false},{result:'fixed'},{knownResult:false}].map(edit=>({kind:'function',parameters:1,wrappers:0,knownParameter:true,result:'bool',knownResult:true,...edit,expected:'a lookup predicate needs a known direct one-argument callable with a Boolean result'}))],
 callableContexts:[{consumer:'unwrap_or_else',receiver:true,element:true,expected:0},
 {consumer:'is_some_and',receiver:true,element:true,expected:1},{consumer:'is_none_or',receiver:true,element:true,expected:1},
 {consumer:'unwrap_or',receiver:true,element:true,expected:-1},{consumer:'unknown',receiver:true,element:true,expected:-1},
 {consumer:'unwrap_or_else',receiver:false,element:true,expected:-1},{consumer:'is_some_and',receiver:true,element:false,expected:-1}],
 closureParameterForms:[{role:'binding',expected:'binding'},{role:'_',expected:'wildcard'},
 {role:'mut',expected:'unsupported'},{role:'ref',expected:'unsupported'},{role:'&',expected:'unsupported'},{role:'unknown',expected:'unsupported'}],
 references:['https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else'],
 boundary:'Unexecuted source draft for actual immutable scalar lookup, copied/cloned values and known direct zero-argument fallback or one-argument Boolean predicate callables. Callable construction remains eager; invocation occurs only on its selected presence/absence branch. Complete FnOnce/FnMut, non-Copy closure captures/results, mutable lookups, borrow/drop/allocation observations and escaping Option values retain separate obligations.'});
