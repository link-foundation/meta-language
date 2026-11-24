// Independent interval decisions and original Rust predicates; no execution.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],premises=[],refusals=[];
function add(name,body,abort=null,helpers=''){cases.push({name,module:'checked_predicate_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:abort===null?'true':null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;']});}
for(const signed of [false,true])for(const [bits,pointer]of [[8,false],[16,false],[32,false],[64,false],[128,false],[64,true]]){
 const owner=pointer?(signed?'isize':'usize'):(signed?'i':'u')+bits,limit=1n<<BigInt(signed?bits-1:bits),minimum=signed?-limit:0n,maximum=limit-1n;
 for(const [method,left,right,value]of [['checked_add',maximum,1n,null],['checked_add',7n,1n,8n],['checked_sub',0n,1n,signed?-1n:null],['checked_div',1n,0n,null],['checked_rem',7n,2n,1n],['checked_neg',minimum,null,null],...signed?[['checked_div_euclid',-7n,2n,-4n],['checked_abs',minimum,null,null]]:[]]){
  // Unsigned zero negation is the sole successful unsigned checked_neg.
  const actual=method==='checked_neg'&&!signed?0n:value;
  for(const consumer of ['is_some_and','is_none_or']){const expected=actual===null?consumer==='is_none_or':actual>0n;
   add(owner+' '+method+' '+left+' '+right+' '+consumer,'let value='+left+owner+';value.'+method+'('+(right===null?'':right+owner)+').'+consumer+'(|selected|selected>0'+owner+')=='+expected);
  }
 }
 for(const consumer of ['is_some_and','is_none_or'])add(owner+' failed guard cannot invoke '+consumer,'let value='+maximum+owner+';value.checked_add(1'+owner+').'+consumer+'(|_|panic!("unreachable predicate"))=='+(consumer==='is_none_or'));
 add(owner+' predicate retains immutable Copy capture','let threshold=7'+owner+';8'+owner+'.checked_add(0'+owner+').is_some_and(|value|value>threshold)');
 add(owner+' predicate function retains actual input owner','8'+owner+'.checked_add(0'+owner+').is_some_and(accept)',null,'pub fn accept(value:'+owner+')->bool{value>7'+owner+'}');
 add(owner+' shift failure retains '+owner+' predicate context','1'+owner+'.checked_shl('+bits+'u32).is_none_or(|value|value>0'+owner+')');
}
add('successful checked arithmetic invokes a diverging predicate','7u8.checked_add(1u8).is_some_and(|_|panic!("predicate invoked"))','predicate invoked');
add('failed checked arithmetic still constructs the predicate eagerly','255u8.checked_add(1u8).is_some_and(factory())','factory eager','pub fn factory()->fn(u8)->bool{panic!("factory eager")}');
add('receiver precedes argument and predicate construction','input().checked_add(argument()).is_some_and(factory())','receiver first','pub fn input()->u8{panic!("receiver first")}pub fn argument()->u8{panic!("argument second")}pub fn factory()->fn(u8)->bool{panic!("factory third")}');
add('argument precedes predicate construction','7u8.checked_add(argument()).is_none_or(factory())','argument first','pub fn argument()->u8{panic!("argument first")}pub fn factory()->fn(u8)->bool{panic!("factory second")}');
add('predicate factory retains actual returned function pointer signature','8u8.checked_add(0u8).is_some_and(factory())',null,'pub fn accept(value:u8)->bool{value>7u8}pub fn factory()->fn(u8)->bool{accept}');
for(const consumer of ['is_some_and','is_none_or']){premises.push({operation:'testRustCheckedIntegerPredicateConsumer',args:[consumer],expected:true},{operation:'readRustCheckedIntegerConsumerArity',args:[consumer],expected:1},{operation:'readRustCheckedIntegerResultOwner',args:[consumer,'usize',''],expected:'bool'},{operation:'planRustCheckedIntegerConsumer',args:[consumer],expected:['condition','fallback','success','call one',consumer==='is_none_or'?'true':'false','if']});}
premises.push({operation:'testRustCheckedIntegerPredicateConsumer',args:['is_some'],expected:false});
const reason='a checked integer predicate needs a direct one-argument callable with its actual integer parameter and Boolean result';
for(const args of [['function',1,0,true,true],['function',0,0,true,true],['function',2,0,true,true],['function',1,1,true,true],['function',1,0,false,true],['function',1,0,true,false],['bool',1,0,true,true]])premises.push({operation:'readRustCheckedIntegerPredicateError',args,expected:args[0]==='function'&&args[1]===1&&args[2]===0&&args[3]&&args[4]?'':reason});
refusals.push({name:'checked predicate retains actual callable kind',source:'pub fn result()->bool{7u8.checked_add(1u8).is_some_and(true)}fn main(){}',kind:'unsupported',reason:'Rust checked integer predicate: '+reason});
writeProjectData(new URL('../../parity/fixtures/rust-checked-integer-predicates.lino',import.meta.url),{schemaVersion:1,model:'Rust checked integer Option predicate consumers 1',retainSourceTypeOwners:true,cases,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/rust-checked-integer-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/std/option/enum.Option.html#method.is_some_and','https://doc.rust-lang.org/std/option/enum.Option.html#method.is_none_or'],boundary:'Unexecuted source-owned predicate controls across checked operators and source owners. Mutable/non-Copy callbacks, escaping Option values and complete source/grammar translation remain obligations.'});
