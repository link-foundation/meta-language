// Source/data authoring only: never parse or execute represented fixture programs.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],premises=[],refusals=[];
function add(name,body,value='true',abort=null,helpers=''){cases.push({name,module:'result_predicate_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:abort===null?value:null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;']});}
for(const signed of [false,true])for(const bits of [8,16,32,64,128]){
 const owner=(signed?'i':'u')+bits,limit=1n<<BigInt(signed?bits-1:bits),minimum=signed?-limit:0n,maximum=limit-1n;
 for(const value of new Set([minimum,0n,maximum]))add(owner+' successful boundary '+value,owner+'::try_from('+value+owner+').is_ok_and(|value|value=='+value+owner+')');
 add(owner+' successful false predicate',owner+'::try_from(0'+owner+').is_ok_and(|value|value!=0'+owner+')','false');
 if(bits<128)add(owner+' upper failure cannot invoke predicate',owner+'::try_from('+limit+(signed?'i128':'u128')+').is_ok_and(|_|panic!("unreachable predicate"))','false');
 if(!signed)add(owner+' negative failure cannot invoke predicate',owner+'::try_from(-1i128).is_ok_and(|_|panic!("unreachable negative predicate"))','false');
}
add('contextual predicate retains an immutable Copy capture','let threshold=6u8;u8::try_from(7u16).is_ok_and(|value|value>threshold)');
add('named predicate keeps its actual argument owner','u8::try_from(7u16).is_ok_and(accept)', 'true',null,'pub fn accept(value:u8)->bool{value==7u8}');
add('function pointer factory result retains actual predicate signature','u8::try_from(7u16).is_ok_and(factory())','true',null,'pub fn accept(value:u8)->bool{value==7u8}pub fn factory()->fn(u8)->bool{accept}');
add('failed conversion evaluates the callable factory before branching','u8::try_from(256u16).is_ok_and(factory())','true','factory eager','pub fn factory()->fn(u8)->bool{panic!("factory eager")}');
add('input is evaluated before callable construction','u8::try_from(input()).is_ok_and(factory())','true','input first','pub fn input()->u16{panic!("input first")}pub fn factory()->fn(u8)->bool{panic!("factory second")}');
add('successful conversion invokes a diverging predicate','u8::try_from(7u16).is_ok_and(|_|panic!("predicate invoked"))','true','predicate invoked');
add('pointer target retains its source owner','usize::try_from(7u128).is_ok_and(|value|value==7usize)');
premises.push({operation:'readRustPrimitiveTryFromConsumerArity',args:['is_ok_and'],expected:1},{operation:'readRustPrimitiveTryFromResultOwner',args:['is_ok_and','u8',''],expected:'bool'});
for(const [guards,condition]of [[0,['true']],[1,['input','minimum','ge']],[2,['input','maximum','le']],[3,['input','minimum','ge','input','maximum','le','and']]])premises.push({operation:'planRustPrimitiveTryFromConsumer',args:['is_ok_and',guards],expected:condition.concat(['predicate','false','if'])});
const reason='a Result predicate needs a direct one-argument callable with its actual integer parameter and Boolean result';
for(const args of [['function',1,0,true,true],['function',0,0,true,true],['function',2,0,true,true],['function',1,1,true,true],['function',1,0,false,true],['function',1,0,true,false],['bool',1,0,true,true]])premises.push({operation:'readRustPrimitiveTryFromPredicateError',args,expected:args[0]==='function'&&args[1]===1&&args[2]===0&&args[3]&&args[4]?'':reason});
refusals.push({name:'non-callable predicate is refused by its actual signature',source:'pub fn result()->bool{u8::try_from(7u16).is_ok_and(true)}fn main(){}',kind:'unsupported',reason:'Rust primitive TryFrom predicate: '+reason});
writeProjectData(new URL('../../parity/fixtures/rust-primitive-try-from-predicates.lino',import.meta.url),{schemaVersion:1,model:'Rust standard integer Result predicates 1',retainSourceTypeOwners:true,cases,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/rust-primitive-try-from-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=32 or 64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/std/result/enum.Result.html#method.is_ok_and'],boundary:'Unexecuted original source and independent interval controls. General Result values, mutable or non-Copy captures, and error callbacks remain obligations.'});
