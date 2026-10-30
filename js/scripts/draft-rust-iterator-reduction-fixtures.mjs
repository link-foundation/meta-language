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
add('borrowed empty iterator count has its actual usize owner','b"".iter().count()','0','usize');
add('copied iterator count retains exact source length','b"ABC".iter().copied().count()','3','usize');
add('reverse borrowed count retains exact source length','b"ABC".iter().rev().count()','3','usize');
add('standard vector count retains its primitive receiver owner','vec![true,false,true].iter().cloned().rev().count()','3','usize');
add('count still evaluates a failing receiver','bytes().iter().count()',null,'usize','fn bytes()->Vec<u8>{panic!("count receiver")}', 'count receiver');
add('collection keeps actual byte elements','b"AB".iter().copied().collect::<Vec<u8>>()[1]','66','u8');
add('collection infers its placeholder from actual byte items','b"AB".iter().copied().collect::<Vec<_>>()[0]','65','u8');
add('fully qualified collection keeps standard owner identity','b"AB".iter().cloned().collect::<std::vec::Vec<_>>()[1]','66','u8');
add('empty collected vectors retain their actual element type','b"".iter().copied().collect::<Vec<_>>().len()','0','usize');
add('reversed collection retains actual visit order','b"ABC".iter().rev().copied().collect::<Vec<_>>()[0]','67','u8');
add('collected booleans keep their source primitive owner','vec![true,false].iter().copied().collect::<Vec<_>>()[1]','false','bool');
add('collected floats keep their source primitive owner','vec![1.0f64,2.0f64].iter().copied().collect::<Vec<_>>()[1]','2','f64');
add('collection can discharge a fresh vector function result','bytes()[1]','66','u8','fn bytes()->Vec<u8>{b"AB".iter().copied().collect::<Vec<_>>()}');
add('collection leaves the source vector unchanged','let values=vec![65u8,66u8];let copy=values.iter().rev().copied().collect::<Vec<_>>();values[0]+copy[0]','131','u8');
add('collection retains an explicit pointer-sized source owner','vec![7usize].iter().copied().collect::<Vec<usize>>()[0]','7','usize');
add('adjacent type closing and assignment retain the original tokens','let values:Vec<u8>=vec![65u8];values[0]','65','u8');
add('compound nested type closing preserves its assignment','let value:Box<Box<u8>>=Box::new(Box::new(65u8));**value','65','u8');
add('expression shift tokens are independent of type closings','8u8>>1u32','4','u8');
add('borrowed owned-string vector counts do not require Copy elements','vec![String::from("a"),String::from("b")].iter().count()','2','usize');
add('borrowed owned-string slice counts do not clone their elements','size(&vec![String::from("a")])','1','usize','fn size(values:&[String])->usize{values.iter().rev().count()}');
add('unit vector counts do not need invented primitive owners','vec![(),(),()].iter().count()','3','usize');
add('fresh owned iterator predicates receive scalar values','vec![65u8,66u8].into_iter().all(|value|value>=65)','true');
add('fresh owned iterator folds retain element ownership','vec![1u8,2u8,3u8].into_iter().fold(0u64,|acc,value|acc*10+u64::from(value))','123','u64');
add('fresh owned iterators collect their actual item type','vec![65u8,66u8].into_iter().rev().collect::<Vec<_>>()[0]','66','u8');
add('fresh owned boolean iterator predicates preserve short circuit','vec![true,false].into_iter().any(|value|value)','true');
add('owned string counts do not require primitive Copy items','vec![String::from("a")].into_iter().count()','1','usize');
add('fresh vector function results may be consumed immediately','bytes().into_iter().count()','2','usize','fn bytes()->Vec<u8>{b"AB".to_vec()}');
add('a collected temporary may be consumed as a fresh vector','b"AB".iter().copied().collect::<Vec<_>>().into_iter().all(|value|value>=65)','true');
add('the complete former sum refusal now retains its native value','b"A".iter().copied().sum()','65','u8');
const typeTokens=[{token:'>>',expected:['>','>']},{token:'>=',expected:['>','=']},{token:'>>=',expected:['>','>','=']},{token:'>',expected:[]},{token:'<<',expected:[]},{token:'&&',expected:[]},{token:'😀',expected:[]}];
const parameterPremises=[];
for(const known of [false,true])for(const baseCopy of [false,true])for(const layers of [0,1,2])for(const reference of [false,true])for(const primitiveOwner of [false,true])parameterPremises.push({known,baseCopy,layers,reference,primitiveOwner,expected:known&&baseCopy&&(layers===0||layers>=1&&layers<=256&&reference&&primitiveOwner)});
const receiverError='a reduction needs an actual standard immutable scalar array and its complete primitive element owner',accumulatorError='fold needs the actual direct immutable Copy accumulator type',callbackError='a reduction needs its complete direct scalar callable source signature';
const premises=[];function premise(operation,args,expected){premises.push({operation,args,expected});}
for(const method of ['iter','into_iter','bytes'])for(const count of [0,1])for(const ordinary of [false,true])premise('acceptRustIteratorReductionStart',[method,count,ordinary],(method==='iter'||method==='into_iter'||method==='bytes')&&count===0&&ordinary);
for(const method of ['copied','cloned','map',''])premise('acceptRustIteratorCopyAdapter',[method],method==='copied'||method==='cloned');
for(const [method,operation,arity,roles]of [['all','every',1,['receiver','callback']],['any','some',1,['receiver','callback']],['fold','reduce',2,['receiver','initial','callback']],['rfold','reduce',2,['receiver','initial','callback']],['map','',-1,[]]]){premise('readRustIteratorReductionOperation',[method],operation);premise('readRustIteratorReductionCallbackArity',[method],arity);premise('readRustIteratorReductionCaptureRoles',[method],roles);}
for(const receiver of [false,true])for(const owner of [false,true])premise('readRustIteratorReductionReceiverError',[receiver,owner],receiver&&owner?'':receiverError);
for(const known of [false,true])for(const copy of [false,true])for(const layers of [0,1])premise('readRustIteratorReductionAccumulatorError',[known,copy,layers],known&&copy&&layers===0?'':accumulatorError);
for(const method of ['all','any','fold','rfold']){const n=method==='fold'||method==='rfold'?2:1,valid=[method,'function',n,0,Array(n).fill(true),true];premise('readRustIteratorReductionCallbackError',valid,'');for(const [index,value]of [[1,'fixed'],[2,n+1],[3,1],[4,[]],[4,Array(n).fill(false)],[5,false]]){const args=structuredClone(valid);args[index]=value;premise('readRustIteratorReductionCallbackError',args,callbackError);}}
premise('readRustIteratorReductionCallbackError',['map','function',1,0,[true],true],callbackError);
for(const method of ['copied','cloned','rev','map']){
 premise('acceptRustIteratorReductionAdapter',[method],method!=='map');
 for(const state of ['borrowed','copied','owned','unknown'])for(const direction of ['forward','reverse','unknown']){
  const expected=state==='unknown'||direction==='unknown'?[]:method==='rev'?[state,direction==='forward'?'reverse':'forward']:(method==='copied'||method==='cloned')&&state==='borrowed'?['copied',direction]:[];
  premise('readRustIteratorReductionAdapterState',[method,state,direction],expected);
 }
}
for(const method of ['all','any','fold','rfold'])for(const direction of ['forward','reverse'])premise('planRustIteratorReductionReceiver',[method,direction],(method==='rfold')!==(direction==='reverse')?['map reversed indices']:[]);
premise('planRustIteratorReductionReverseIndex',[],['receiver','receiver','length','index','sub','one','sub','natural','lookup']);
premise('readRustIteratorReductionOperation',['count'],'count');premise('readRustIteratorReductionOperation',['collect'],'collect');
premise('readRustIteratorReductionCountResultOwner',[],'usize');premise('planRustIteratorReductionCount',[],['receiver','integer length','result cast']);
for(const token of ['_','u8',''])for(const known of [false,true])premise('acceptRustIteratorCollectionInferredType',[token,known],token==='_'&&known);
for(const copied of [false,true])for(const standard of [false,true])for(const same of [false,true])for(const layers of [0,1])premise('readRustIteratorCollectionError',[copied,standard,'array','owned vector','library','std::vec::Vec',1,same,layers],copied&&standard&&same&&layers===0?'':'collection needs copied scalar items and an actual standard Vec of their complete source type');
for(const [index,value]of [[2,'named'],[3,'immutable slice'],[4,'declaration'],[5,'crate::Vec'],[6,0],[6,2]]){const args=[true,true,'array','owned vector','library','std::vec::Vec',1,true,0];args[index]=value;premise('readRustIteratorCollectionError',args,'collection needs copied scalar items and an actual standard Vec of their complete source type');}
for(const [kind,model,known,layers,adapters,expected]of [['array','owned vector',true,true,true,''],['array','immutable slice',true,true,true,''],['named','owned vector',true,true,true,'count needs an actual standard array receiver and valid item adapters'],['array','unknown',true,true,true,'count needs an actual standard array receiver and valid item adapters'],['array','owned vector',false,true,true,'count needs an actual standard array receiver and valid item adapters'],['array','owned vector',true,false,true,'count needs an actual standard array receiver and valid item adapters'],['array','owned vector',true,true,false,'count needs an actual standard array receiver and valid item adapters']])premise('readRustIteratorCountReceiverError',[kind,model,known,layers,adapters],expected);
for(const [method,expected]of [['iter','borrowed'],['into_iter','owned'],['bytes','copied'],['other','']])premise('readRustIteratorReductionInitialState',[method],expected);
const owned=["array","owned vector",true,0,true];premise('readRustIteratorOwnedReceiverError',owned,'');
for(const [index,value]of [[0,'named'],[1,'immutable slice'],[2,false],[3,1],[4,false]]){const args=[...owned];args[index]=value;premise('readRustIteratorOwnedReceiverError',args,'owned iteration needs an actual consumable standard vector and its consumption proof');}
const ownerSelections=[{identities:[],admitted:[],expected:-1},{identities:['primitive:u8'],admitted:[true],expected:0},{identities:['primitive:u8','primitive:u8'],admitted:[true,true],expected:0},{identities:['primitive:usize','primitive:u64'],admitted:[true,true],expected:-1},{identities:['primitive:u8','primitive:u8'],admitted:[true,false],expected:-1},{identities:['primitive:u8'],admitted:[],expected:-1}];
const refusals=[{name:'escaping iterator states are not guessed',source:'pub fn result()->bool{let values=b"A".iter();true}fn main(){}',kind:'unsupported',reason:'Rust iterator reduction: an iterator requires its immediate copied or cloned reduction'},{name:'unregistered consumers retain a precise boundary',source:'pub fn result()->u8{b"A".iter().copied().min().unwrap_or(0)}fn main(){}',kind:'unsupported',reason:'Rust iterator reduction: the complete standard iterator reduction needs a registered operation'}];
for(const [name,source]of [['borrowed item collection retains its independent ownership boundary','pub fn result()->u8{*b"A".iter().collect::<Vec<&u8>>()[0]}fn main(){}'],['equal representation never merges usize and u64 collection owners','pub fn result()->u64{vec![7usize].iter().copied().collect::<Vec<u64>>()[0]}fn main(){}'],['collection never coerces element width','pub fn result()->u16{b"A".iter().copied().collect::<Vec<u16>>()[0]}fn main(){}']])refusals.push({name,source,kind:'unsupported',reason:'Rust iterator reduction: collection needs copied scalar items and an actual standard Vec of their complete source type'});
refusals.push({name:'count never legitimizes an invalid Copy adapter on owned strings',source:'pub fn result()->usize{vec![String::from("a")].iter().copied().count()}fn main(){}',kind:'unsupported',reason:'Rust iterator reduction: count needs an actual standard array receiver and valid item adapters'});
add('a tracked named vector is consumed once','let values=vec![65u8];values.into_iter().count()','1','usize');
writeProjectData(new URL('../../parity/fixtures/rust-iterator-reductions.lino',import.meta.url),{schemaVersion:1,model:'ordinary Rust immediate copied scalar iterator reductions 1',retainSourceTypeOwners:true,cases,premises,parameterPremises,typeTokens,ownerSelections,refusals,history:[{revision:'ef4ae3ab',source:'pub fn result()->usize{let values=vec![65u8];values.into_iter().count()}fn main(){}',formerReason:'Rust iterator reduction: owned iteration needs an actual fresh standard vector and its consumption proof',change:'The exact formerly refused valid source is retained as a positive original; tracked source identity now consumes its owner and independent invalid-reuse originals remain covered.'},{revision:'9f63bf87d89bad98a3bec0ef28bb8c68be748a9f',source:'pub fn result()->u8{b"A".iter().copied().sum()}fn main(){}',formerReason:'Rust iterator reduction: the complete standard iterator reduction needs a registered operation',change:'The exact former sum input is retained as a positive original after drafting its standard numeric fold; unregistered min consumers remain refused.'},{revision:'e49eb119550f9ab92ac3686ce743586b45390d79',operation:'acceptRustIteratorReductionStart',args:['into_iter',0,true],formerExpected:false,expected:true,reference:'https://doc.rust-lang.org/std/vec/struct.Vec.html',change:'Fresh owned standard-vector consumption is now drafted; the original admission input remains covered, with separate actual freshness and invalidation controls.'},{revision:'ed989412',path:'parity/fixtures/rust-iterator-reductions.lino',source:'pub fn result()->bool{b"A".iter().all(|value|*value==65)}fn main(){}',formerReason:'Rust iterator reduction: an iterator requires its immediate copied or cloned reduction',change:'The exact former refusal input is retained as a positive original after implementing its immutable reference parameter semantics.'}],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=32 or 64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/reference/types/inferred.html','https://doc.rust-lang.org/reference/tokens.html','https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.copied','https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all','https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any','https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold'],boundary:'Unexecuted complete originals and independently declared decisions. Immediate copied/cloned standard scalar all/any/fold retain receiver, initializer and callback construction order, short circuit and left fold. Additional reference and borrowing semantics, escaping iterator state, other combinators, general source inference and complete Rust/JavaScript translation remain full obligations.'});
