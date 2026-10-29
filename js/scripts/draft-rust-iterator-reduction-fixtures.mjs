// Literal source/control construction only; represented programs stay unparsed.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,value,ret='bool',declarations='',abort=null){cases.push({name,module:'iterator_reduction_case_'+cases.length,source:declarations+'pub fn result()->'+ret+'{'+body+'}',value,abort});}
add('all accepts an empty byte iterator','b"".iter().copied().all(|value|value==0)','true');
add('any rejects an empty byte iterator','b"".iter().copied().any(|value|value==0)','false');
add('all visits every matching byte','b"AAA".iter().copied().all(|value|value==65)','true');
add('all stops before an aborting later element','b"AB".iter().copied().all(|value|if value==65{false}else{panic!("late all")})','false');
add('any stops before an aborting later element','b"AB".iter().copied().any(|value|if value==65{true}else{panic!("late any")})','true');
add('any reaches its final matching element','b"ABC".iter().cloned().any(|value|value==67)','true');
add('all keeps its final negative result','b"ABC".iter().cloned().all(|value|value!=67)','false');
add('fold keeps its explicit initializer on empty input','b"".iter().copied().fold(7u64,|acc,value|acc+u64::from(value))','7','u64');
add('fold consumes bytes strictly left to right','b"123".iter().copied().fold(0u64,|acc,value|acc*10+u64::from(value-48))','123','u64');
add('direct byte vectors retain exact element ownership','vec![65u8,66u8].iter().copied().any(|value|value==66)','true');
add('boolean vectors retain their actual primitive owner','vec![true,false].iter().cloned().all(|value|value)','false');
add('float vectors retain their actual primitive owner','vec![1.0f64,2.0f64].iter().copied().fold(3.0f64,|acc,value|acc+value)','6','f64');
add('slice arguments retain their source element contract','check(b"ABC".as_slice())','true','bool','fn check(values:&[u8])->bool{values.iter().copied().any(|value|value==67)}');
add('immutable captured scalar predicates keep their environment','let threshold=66u8;b"ABC".iter().copied().any(|value|value>threshold)','true');
add('named scalar predicate retains its full source signature','b"ABC".iter().copied().all(check)','true','bool','fn check(value:u8)->bool{value>=65}');
add('empty fold still evaluates its initializer','b"".iter().copied().fold(initial(),|acc,value|acc+u64::from(value))',null,'u64','fn initial()->u64{panic!("fold initial")}', 'fold initial');
add('borrowed byte predicates preserve explicit dereferencing','b"A".iter().all(|value|*value==65)','true');
add('typed borrowed predicates retain their complete source owner','b"AB".iter().any(|value:&u8|*value==66)','true');
add('borrowed left fold keeps its direct accumulator','b"123".iter().fold(0u64,|acc,value|acc*10+u64::from(*value-48))','123','u64');
add('borrowed named predicate keeps its reference parameter','b"AB".iter().all(check)','true','bool','fn check(value:&u8)->bool{*value>=65}');
add('borrowed boolean vector predicates retain their source owner','vec![true,false].iter().all(|value|*value)','false');
add('borrowed empty all never invokes its aborting predicate','b"".iter().all(|value|if *value==65{true}else{panic!("empty borrowed")})','true');
add('borrowed short circuit never reaches the later abort','b"AB".iter().any(|value|if *value==65{true}else{panic!("late borrowed")})','true');
add('right fold visits the final source element first','b"123".iter().copied().rfold(0u64,|acc,value|acc*10+u64::from(value-48))','321','u64');
add('borrowed right fold retains reference parameter semantics','b"123".iter().rfold(0u64,|acc,value|acc*10+u64::from(*value-48))','321','u64');
add('reversal before copying retains reverse iteration','b"123".iter().rev().copied().fold(0u64,|acc,value|acc*10+u64::from(value-48))','321','u64');
add('reversal after cloning retains reverse iteration','b"123".iter().cloned().rev().fold(0u64,|acc,value|acc*10+u64::from(value-48))','321','u64');
add('two reversals cancel without changing borrowed items','b"123".iter().rev().rev().fold(0u64,|acc,value|acc*10+u64::from(*value-48))','123','u64');
add('reversed right fold restores original visit order','b"123".iter().rev().copied().rfold(0u64,|acc,value|acc*10+u64::from(value-48))','123','u64');
add('reverse predicates short circuit at the last source element','b"ABC".iter().rev().any(|value|if *value==67{true}else{panic!("reverse tail")})','true');
add('reversal never changes the original vector','let values=vec![65u8,66u8];let ignored=values.iter().rev().all(|value|*value>=65);values[0]','65','u8');
add('empty reversed right fold retains its initializer','b"".iter().rev().rfold(7u64,|acc,value|acc+u64::from(*value))','7','u64');
const parameterPremises=[];
for(const known of [false,true])for(const baseCopy of [false,true])for(const layers of [0,1,2])for(const reference of [false,true])for(const primitiveOwner of [false,true])parameterPremises.push({known,baseCopy,layers,reference,primitiveOwner,expected:known&&baseCopy&&(layers===0||layers===1&&reference&&primitiveOwner)});
const receiverError='a reduction needs an actual standard immutable scalar array and its complete primitive element owner',accumulatorError='fold needs the actual direct immutable Copy accumulator type',callbackError='a reduction needs its complete direct scalar callable source signature';
const premises=[];function premise(operation,args,expected){premises.push({operation,args,expected});}
for(const method of ['iter','into_iter'])for(const count of [0,1])for(const ordinary of [false,true])premise('acceptRustIteratorReductionStart',[method,count,ordinary],method==='iter'&&count===0&&ordinary);
for(const method of ['copied','cloned','map',''])premise('acceptRustIteratorCopyAdapter',[method],method==='copied'||method==='cloned');
for(const [method,operation,arity,roles]of [['all','every',1,['receiver','callback']],['any','some',1,['receiver','callback']],['fold','reduce',2,['receiver','initial','callback']],['rfold','reduce',2,['receiver','initial','callback']],['map','',-1,[]]]){premise('readRustIteratorReductionOperation',[method],operation);premise('readRustIteratorReductionCallbackArity',[method],arity);premise('readRustIteratorReductionCaptureRoles',[method],roles);}
for(const receiver of [false,true])for(const owner of [false,true])premise('readRustIteratorReductionReceiverError',[receiver,owner],receiver&&owner?'':receiverError);
for(const known of [false,true])for(const copy of [false,true])for(const layers of [0,1])premise('readRustIteratorReductionAccumulatorError',[known,copy,layers],known&&copy&&layers===0?'':accumulatorError);
for(const method of ['all','any','fold','rfold']){const n=method==='fold'||method==='rfold'?2:1,valid=[method,'function',n,0,Array(n).fill(true),true];premise('readRustIteratorReductionCallbackError',valid,'');for(const [index,value]of [[1,'fixed'],[2,n+1],[3,1],[4,[]],[4,Array(n).fill(false)],[5,false]]){const args=structuredClone(valid);args[index]=value;premise('readRustIteratorReductionCallbackError',args,callbackError);}}
premise('readRustIteratorReductionCallbackError',['map','function',1,0,[true],true],callbackError);
for(const method of ['copied','cloned','rev','map']){
 premise('acceptRustIteratorReductionAdapter',[method],method!=='map');
 for(const state of ['borrowed','copied','unknown'])for(const direction of ['forward','reverse','unknown']){
  const expected=state==='unknown'||direction==='unknown'?[]:method==='rev'?[state,direction==='forward'?'reverse':'forward']:(method==='copied'||method==='cloned')&&state==='borrowed'?['copied',direction]:[];
  premise('readRustIteratorReductionAdapterState',[method,state,direction],expected);
 }
}
for(const method of ['all','any','fold','rfold'])for(const direction of ['forward','reverse'])premise('planRustIteratorReductionReceiver',[method,direction],(method==='rfold')!==(direction==='reverse')?['map reversed indices']:[]);
premise('planRustIteratorReductionReverseIndex',[],['receiver','receiver','length','index','sub','one','sub','natural','lookup']);
const ownerSelections=[{identities:[],admitted:[],expected:-1},{identities:['primitive:u8'],admitted:[true],expected:0},{identities:['primitive:u8','primitive:u8'],admitted:[true,true],expected:0},{identities:['primitive:usize','primitive:u64'],admitted:[true,true],expected:-1},{identities:['primitive:u8','primitive:u8'],admitted:[true,false],expected:-1},{identities:['primitive:u8'],admitted:[],expected:-1}];
const refusals=[{name:'escaping iterator states are not guessed',source:'pub fn result()->bool{let values=b"A".iter();true}fn main(){}',kind:'unsupported',reason:'Rust iterator reduction: an iterator requires its immediate copied or cloned reduction'},{name:'unregistered consumers retain a precise boundary',source:'pub fn result()->u8{b"A".iter().copied().sum()}fn main(){}',kind:'unsupported',reason:'Rust iterator reduction: the complete standard iterator reduction needs a registered operation'}];
writeProjectData(new URL('../../parity/fixtures/rust-iterator-reductions.lino',import.meta.url),{schemaVersion:1,model:'ordinary Rust immediate copied scalar iterator reductions 1',retainSourceTypeOwners:true,cases,premises,parameterPremises,ownerSelections,refusals,history:[{revision:'ed989412',path:'parity/fixtures/rust-iterator-reductions.lino',source:'pub fn result()->bool{b"A".iter().all(|value|*value==65)}fn main(){}',formerReason:'Rust iterator reduction: an iterator requires its immediate copied or cloned reduction',change:'The exact former refusal input is retained as a positive original after implementing its immutable reference parameter semantics.'}],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=32 or 64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.copied','https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all','https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any','https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold'],boundary:'Unexecuted complete originals and independently declared decisions. Immediate copied/cloned standard scalar all/any/fold retain receiver, initializer and callback construction order, short circuit and left fold. Additional reference and borrowing semantics, escaping iterator state, other combinators, general source inference and complete Rust/JavaScript translation remain full obligations.'});
