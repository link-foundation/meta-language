// Future original-source controls and authenticated expectations; no execution.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,source,body,value='7',ret='i64'){cases.push({name,module:'fresh_result_case_'+cases.length,source:source+'pub fn result()->'+ret+'{'+body+'}',value,abort:null});}
add('a default source function returns a fresh scalar vector','fn values()->Vec<i64>{vec![3,4]}','let values=values();values[0]+values[1]');
for(const type of ['i8','i16','i32','i64','u8','u16','u32','u64'])add('fresh results retain '+type+' elements','fn values()->Vec<'+type+'>{vec![7]}','let values=values();values[0]','7',type);
add('fresh Boolean results preserve actual reads','fn values()->Vec<bool>{vec![true]}','let values=values();if values[0]{7}else{8}');
add('fresh binary64 results preserve their element model','fn values()->Vec<f64>{vec![7.0]}','let values=values();values[0]','7','f64');
add('fresh Unit results preserve their element model','fn values()->Vec<()>{vec![()]}','let values=values();let value=values[0];7');
add('empty results retain the declared element context','fn values()->Vec<i64>{vec![]}','let values=values();if values.len()==0{7}else{8}');
add('transparent result aliases keep their actual vector type','type Values=Vec<i64>;fn values()->Values{vec![3,4]}','let values=values();values[0]+values[1]');
add('later aliases retain an earlier result declaration','fn values()->Values{vec![7]}type Values=Vec<i64>;','let values=values();values[0]');
add('a direct terminal return keeps constructor evidence','fn values()->Vec<i64>{return vec![7];}','let values=values();values[0]');
add('both conditional result alternatives are fresh','fn values(flag:bool)->Vec<i64>{if flag{vec![7]}else{vec![8]}}','let values=values(true);values[0]');
add('terminal returns in both branches retain actual result ownership','fn values(flag:bool)->Vec<i64>{if flag{return vec![7];}else{return vec![8];}}','let values=values(true);values[0]');
add('every match result constructs a fresh allocation','fn values(flag:bool)->Vec<i64>{match flag{true=>vec![7],false=>vec![8]}}','let values=values(true);values[0]');
add('a diverging branch supplies no old vector','fn values(flag:bool)->Vec<i64>{if flag{vec![7]}else{panic!("unselected")}}','let values=values(true);values[0]');
add('a scoped scalar binding precedes the fresh result','fn values()->Vec<i64>{let value=7i64;vec![value]}','let values=values();values[0]');
add('borrowed input scalars are copied into an owned result','fn values(input:&[i64])->Vec<i64>{vec![input[0]]}','let input=vec![7i64];let values=values(&input);values[0]');
add('a module owns its complete fresh result declaration','mod values{pub fn read()->Vec<i64>{vec![7]}}','let values=values::read();values[0]');
add('an inherent function retains its fresh result owner','struct Value{value:i64}impl Value{fn values()->Vec<i64>{vec![7]}}','let values=Value::values();values[0]');
add('a checked fresh result is consumed by a real scalar-vector parameter','fn values()->Vec<i64>{vec![7]}fn read(values:Vec<i64>)->i64{values[0]}','read(values())');
add('a later declaration still supplies its real fresh call result','fn read(values:Vec<i64>)->i64{values[0]}pub fn first()->i64{read(values())}fn values()->Vec<i64>{vec![7]}','first()');
add('fresh calls remain fresh within complete conditional arguments','fn values()->Vec<i64>{vec![7]}fn read(values:Vec<i64>)->i64{values[0]}','read(if true{values()}else{vec![8]})');
add('fresh calls remain fresh within complete match arguments','fn values()->Vec<i64>{vec![7]}fn read(values:Vec<i64>)->i64{values[0]}','read(match true{true=>values(),false=>vec![8]})');
const rawPath=new URL('../../parity/fixtures/rust-array-source.lino',import.meta.url),pinned=execFileSync('git',['show','afb637a2:parity/fixtures/rust-array-source.lino'],{maxBuffer:8*1024*1024});
if(!pinned.equals(readFileSync(rawPath)))throw new TypeError('the correction requires its exact unchanged historical array fixture');
const raw=readProjectData(rawPath);
const original=raw.invalid.find(row=>row.id==='array-return');if(!original)throw new TypeError('retain the exact earlier array-return input');
const legacy={...original,resultType:{kind:'array',element:{kind:'float'}},value:[1],currentInterpretation:'ordinary fresh Rust scalar vector results 1',previousExpectation:{kind:original.kind,reason:original.reason,span:original.span}};
const corrections=[{...legacy,functionName:'values',calls:[{arguments:[],value:['1']}]}];
for(const [id,body,value,ret,functionName,resultType,calls]of [
 ['unconverted-source-length','let input=vec![1f64,2f64];count(&input)','2','usize','count',{kind:'fixed',bits:64,signed:false},[{arguments:[[]],value:'0'},{arguments:[[1]],value:'1'},{arguments:[[1,2]],value:'2'}]],
 ['other-length-conversion','let input=vec![1f64,2f64];count(&input)','2','u64','count',{kind:'fixed',bits:64,signed:false},[{arguments:[[]],value:'0'},{arguments:[[1]],value:'1'},{arguments:[[1,2]],value:'2'}]],
 ['source-indexing','let input=vec![7f64];first(&input)','7','f64','first',{kind:'float'},[{arguments:[[7]],value:'7'},{arguments:[[9]],value:'9'}]],
]){const previous=raw.invalid.find(row=>row.id===id);if(!previous)throw new TypeError('retain '+id);corrections.push({...previous,functionName,resultType,calls,currentInterpretation:'ordinary Rust read-only array source interpretation 2',previousExpectation:{kind:previous.kind,reason:previous.reason,span:previous.span}});add('the unchanged earlier '+id+' input has its actual source value',previous.source,body,value,ret);}
add('the unchanged earlier default refusal now has its original runtime value',original.source,'let values=values();values[0]','1','f64');
for(const correction of corrections)for(const [call,control]of correction.calls.entries()){
 const argumentsSource=control.arguments.map(value=>Array.isArray(value)?'&vec!['+value.map(number=>String(number)+'f64').join(',')+']':String(value)).join(',');
 const application=correction.functionName+'('+argumentsSource+')';
 const array=correction.resultType.kind==='array',body=array?'let values='+application+';values.len()==1&&values[0]==1f64':application;
 add('complete original corrected call '+correction.id+' '+call,correction.source,body,array?'true':control.value,array?'bool':correction.resultType.kind==='float'?'f64':'u64');
 cases[cases.length-1].legacyCorrection={id:correction.id,call};
}
const refusals=[],correctedAssertions=[];
const tailReason='fresh Rust array result: every returning tail must construct a new vector of direct scalar Copy values; vector names, calls, clones and borrowed results need separate consumption proofs';
for(const [name,source,call]of [['named vector result','fn bad(values:Vec<i64>)->Vec<i64>{values}','bad(vec![7i64])'],['locally bound result','fn bad()->Vec<i64>{let values=vec![7i64];values}','bad()'],['one old result alternative','fn bad(flag:bool,values:Vec<i64>)->Vec<i64>{if flag{vec![7]}else{values}}','bad(false,vec![7i64])']]){correctedAssertions.push({name,source:source+'fn main(){}',revision:'fabd42ee',formerReason:tailReason,change:'The tracked owner transfers into the result; its exact previous complete input is retained as an admission control.'});add(name,source,'let values='+call+';values[0]');}
refusals.push({name:'unchecked interprocedural result tail',source:'fn values()->Vec<i64>{vec![7]}fn bad()->Vec<i64>{values()}fn main(){}',kind:'unsupported',reason:tailReason});
for(const [name,source]of [['owned String elements need consumption evidence','fn bad()->Vec<String>{vec![String::from("value")]}'],['borrowed results need outlives evidence','fn bad(values:&[i64])->&[i64]{values}']])refusals.push({name,source:source+'fn main(){}',kind:'unsupported',reason:'Rust array result: returning vectors or borrowed slices needs a separate consumption and outlives proof'});
const decisions=[];for(const kind of ['app','name','array','if','let'])for(const model of ['owned vector','immutable slice'])for(const wrappers of [0,1])for(const checked of [false,true])decisions.push({kind,model,wrappers,checked,expected:checked&&['app','name'].includes(kind)&&model==='owned vector'&&wrappers===0});
const revision='afb637a2',history=['js/src/translation/rust.js','js/src/translation/rust-source-origins.js','rust/src/translation/rust/items/functions.rs','rust/src/translation/rust/source_origins.rs','js/scripts/generate-rust-array-source-fixtures.mjs','js/tests/translation-rust-arrays.test.js','js/tests/translation-rust-array-results.test.js','rust/tests/unit/translation_rust_arrays.rs','rust/tests/unit/translation_rust_array_results.rs','parity/fixtures/rust-array-source.lino','parity/fixtures/rust-array-results.lino'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:8*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/rust-fresh-vector-results.lino',import.meta.url),{schemaVersion:1,model:'ordinary fresh Rust scalar vector results 1',cases,refusals,decisions,legacy,corrections,history,correctedAssertions,source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64'},references:['https://doc.rust-lang.org/std/macro.vec.html','https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope','https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len','https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions'],assertions:['defaultOwnedScalarVectorResultsRequireCompleteFreshConstructorTails','checkedSourceCallResultsMaySupplyFreshArgumentsButNotNamedMoves','originalLegacyInputAndEveryPredecessorAssertionRemainAuthenticated','explicitOlderResultProfileRetainsIndependentPremises','bothSourceRuntimesOriginalRustBothCodecsAndFourEmissions','allKnownCallDecisionsRetainActualSourceOccurrenceEvidence'],boundary:'Unexecuted future controls. Ordinary direct scalar owned vector constructor tails, scoped bindings, complete conditional/match tails and diverging alternatives only. Known actually called checked result declarations can supply argument freshness; local names clear that evidence. Tracked returned names and mixed branch owners now have independent consumption proofs. Partial-return completion carriers, string/product elements, mutation, borrowed outlives, arbitrary function pointers, generic vectors and complete ownership remain obligations. The old fixture remains a preserved historical document; obsolete current-default throw expectations use explicit return/length/index corrections, with all original source/diagnostic fields retained.'});
