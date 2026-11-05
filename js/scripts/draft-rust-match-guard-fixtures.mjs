// Source-only declarations. Complete predecessor assertions are data, not evidence.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const definition='enum Value{Some(i64),None} ',cases=[];
function add(name,body,value,prefix=definition,abort=null){cases.push({name,module:'match_guard_case_'+cases.length,source:prefix+'pub fn result()->i64{'+body+'}',value,abort});}
add('selected primitive guard','match 3i64{x if x>2=>x,_=>0}','3','');
add('false guard reaches the next actual arm','match 3i64{x if x>4=>x,y=>y+4}','7','');
add('false guard does not capture the outer fallback name','let x=7;match 3i64{x if x>4=>x,_=>x}','7','');
add('later guarded arm gets its own actual binding','match 3i64{x if x>4=>x,y if y==3=>y+4,_=>0}','7','');
add('unmatched constructor never evaluates its guard','match Value::None{Value::Some(x) if fail()=>x,_=>7}','7',definition+'fn fail()->bool{panic!("unmatched guard")}');
add('earlier unguarded arm keeps later guards lazy','match 3i64{first=>first,second if fail()=>second}','3','fn fail()->bool{panic!("later guard")}');
add('selected guard observes its actual failure','match 3i64{x if fail()=>x,_=>0}',null,'fn fail()->bool{panic!("selected guard")}','selected guard');
add('a false guard keeps its body failure lazy','match 3i64{x if x>4=>panic!("unselected body"),_=>7}','7','');
add('a true guard observes its body failure','match 3i64{x if x>2=>panic!("selected body"),_=>7}',null,'','selected body');
add('or alternatives retry the guard with each alternative binding','match (2i64,5i64){(x,_)|(_,x) if x>3=>x,_=>0}','5','');
add('or alternatives preserve their first successful guard','match (5i64,7i64){(x,_)|(_,x) if x>3=>x,_=>0}','5','');
add('constructor guards retain original pattern fields','match Value::Some(3){Value::Some(x) if x==3=>x,Value::Some(x)=>x+1,Value::None=>0}','3');
add('guard aliases are initialized before the condition','match Value::Some(3){whole@Value::Some(x) if x>0=>match whole{Value::Some(y)=>y,Value::None=>0},_=>0}','3');
add('success packets preserve Unit without selecting fallback','let value=match 3i64{x if x>0=>(),_=>panic!("Unit guard success")};7','7','');
add('block-valued guard arms retain optional source commas','match 3i64{x if x>0=>if let Value::Some(y)=Value::Some(x)&&y>0{y}else{0} _=>0}','3');
add('the source scrutinee is evaluated before every arm','match fail(){Value::Some(x) if x>0=>x,_=>0}',null,definition+'fn fail()->Value{panic!("scrutinee before guard")}','scrutinee before guard');
const original='fn f(n: u64) -> u64 { match n { k if k > 2 => 1, _ => 0 } } fn main() { println!("x"); }';
cases.push({name:'exact former guarded match input retains both outcomes',module:'match_guard_original',source:original+'pub fn result()->u64{f(3)+f(1)}',value:'1',abort:null});
const revision='1afe9c54',history=['rust/tests/unit/translation_rust_frontend.rs','js/tests/translation-rust-records.test.js','js/src/translation/rust.js','rust/src/translation/rust/expressions/control.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
const refusals=[{name:'guarded wildcard alone does not discharge exhaustiveness',source:'fn result(value:i64)->i64{match value{x if true=>x}}fn main(){}',kind:'type',reason:'non-exhaustive match'},{name:'guarded constructor does not cover a missing unguarded constructor',source:'enum Value{Some(i64),None}fn result(value:Value)->i64{match value{Value::Some(x) if true=>x,Value::None=>0}}fn main(){}',kind:'type',reason:'non-exhaustive match'}];
writeProjectData(new URL('../../parity/fixtures/rust-match-guards.lino',import.meta.url),{schemaVersion:1,model:'ordered Rust match guard continuation packets 1',cases,refusals,history,source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; Boolean guards'},references:['https://doc.rust-lang.org/reference/expressions/match-expr.html#match-guards'],assertions:['scrutineeCapturedOnceBeforeOrderedArms','guardOnlyRunsForItsActualMatchingAlternative','falseGuardsRetryLaterOrAlternativesAndArms','outerFallbackScopeAndArmLocalShadowing','aliasesAndLazyActualAbortOrder','unguardedCoverageAloneDischargesExhaustiveness','sourceBlockBoundaryAndUnitPayloadPreservation','exactOriginalGuardInputAndFullPredecessorAssertions','completeSharedDefinitionsBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted future controls. The current value representation and source ownership checks remain in force. Full guard borrow/move/drop/temporary protocols, guard let chains, outer arm attributes, mutable/interior state and full Rust semantics remain requirements. At most256 expanded actual alternatives; exhaustion refuses.'});
