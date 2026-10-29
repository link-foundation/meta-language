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
const printedSources=[
 {name:'present lookup captures receiver and index but does not invoke fallback',source:'pub fn receiver()->Vec<u32>{println!("receiver");vec![7u32]}pub fn index()->usize{println!("index");0usize}pub fn fallback()->u32{println!("fallback");37u32}fn main(){println!("{}",receiver().get(index()).copied().unwrap_or_else(||->u32{fallback()}));}',lines:['receiver','index','7']},
 {name:'absent lookup invokes fallback after receiver and index once',source:'pub fn receiver()->Vec<u32>{println!("receiver");vec![]}pub fn index()->usize{println!("index");0usize}pub fn fallback()->u32{println!("fallback");37u32}fn main(){println!("{}",receiver().get(index()).copied().unwrap_or_else(||->u32{fallback()}));}',lines:['receiver','index','fallback','37']}
];
const refusals=[
 {name:'lazy fallback preserves pointer-sized result ownership',source:'pub fn select()->usize{let values:Vec<usize>=vec![];values.first().copied().unwrap_or_else(||->u64{1u64})}fn main(){}',kind:'unsupported',reason:'Rust integer source owner: distinct pointer-sized and fixed-width integer source types need an explicit cast',originalRustAccepted:false},
 {name:'lazy fallback requires zero arguments',source:'pub fn select()->u32{let values:Vec<u32>=vec![];values.first().copied().unwrap_or_else(|value:u32|->u32{value})}fn main(){}',kind:'unsupported',reason:'Rust array lookup lazy fallback: a lazy lookup fallback needs a known direct zero-argument callable result',originalRustAccepted:false},
 {name:'borrowed optional results retain their separate consumer obligation',source:'pub fn select(){let values:Vec<u32>=vec![];values.first().unwrap_or_else(||->u32{1u32});}fn main(){}',kind:'unsupported',reason:'Rust array lookup Option: an immediate standard presence or copied-value consumer is required',originalRustAccepted:false}
];
writeProjectData(new URL('../../parity/fixtures/rust-array-lookup-lazy.lino',import.meta.url),{
 schemaVersion:1,retainSourceTypeOwners:true,model:null,cases,printedSources,refusals,
 malformed:[{name:'lazy fallback retains its one argument',source:'pub fn select()->u32{let values:Vec<u32>=vec![];values.first().copied().unwrap_or_else()}fn main(){}'}],
 source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64'},
 lazySignatures:[{kind:'function',parameters:0,wrappers:0,knownResult:true,expected:''},
  ...[{kind:'string'},{parameters:1},{wrappers:1},{knownResult:false}].map(edit=>({kind:'function',parameters:0,wrappers:0,knownResult:true,...edit,expected:'a lazy lookup fallback needs a known direct zero-argument callable result'}))],
 references:['https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else'],
 boundary:'Unexecuted source draft for actual immutable scalar lookup, copied/cloned values and known direct zero-argument callables. Callable construction remains eager; invocation occurs only on absence. Complete FnOnce/FnMut, non-Copy closure captures/results, mutable lookups, borrow/drop/allocation observations and escaping Option values retain separate obligations.'});
