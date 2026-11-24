// Exact original programs and independent interval/source-order controls.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],premises=[],refusals=[];
function add(name,body,abort=null,helpers=''){cases.push({name,module:'checked_mapper_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:abort===null?'true':null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;']});}
for(const signed of [false,true])for(const [bits,pointer]of [[8,false],[16,false],[32,false],[64,false],[128,false],[64,true]]){
 const owner=pointer?(signed?'isize':'usize'):(signed?'i':'u')+bits,limit=1n<<BigInt(signed?bits-1:bits),minimum=signed?-limit:0n,maximum=limit-1n;
 add(owner+' successful checked mapper retains actual scalar input','7'+owner+'.checked_add(1'+owner+').map_or(9,|value|value)==8'+owner);
 add(owner+' failed checked mapper selects eager default','let value='+maximum+owner+';value.checked_add(1'+owner+').map_or(9'+owner+',|value|value+1'+owner+')==9'+owner);
 add(owner+' successful checked mapper retains exact minimum','let value='+minimum+owner+';value.checked_add(0'+owner+').map_or(9'+owner+',|value|value)=='+minimum+owner);
 add(owner+' checked mapper retains immutable Copy capture','let increment=2'+owner+';7'+owner+'.checked_add(1'+owner+').map_or(9'+owner+',|value|value+increment)==10'+owner);
 add(owner+' checked mapper retains actual named function signature','7'+owner+'.checked_add(1'+owner+').map_or(9'+owner+',identity)==8'+owner,null,'pub fn identity(value:'+owner+')->'+owner+'{value}');
 add(owner+' checked mapper has a contextual unsuffixed result literal','7'+owner+'.checked_add(1'+owner+').map_or(9'+owner+',|_|1)==1'+owner);
 add(owner+' checked shift failure cannot invoke a diverging mapper','1'+owner+'.checked_shl('+bits+'u32).map_or(9'+owner+',|_|panic!("unreachable mapper"))==9'+owner);
 add(owner+' Euclidean arithmetic keeps mapper invocation guarded','7'+owner+'.checked_div_euclid(2'+owner+').map_or(9'+owner+',|value|value+1'+owner+')==4'+owner);
}
add('checked mapper changes integer result width','7u8.checked_add(1u8).map_or(9u64,|value|u64::from(value)+1u64)==9u64');
add('checked mapper changes integer result to Boolean','7u8.checked_add(1u8).map_or(false,|value|value>7u8)');
add('checked mapper changes integer result to binary64','7u8.checked_add(1u8).map_or(-1.5f64,|value|f64::from(value)+0.5f64)==8.5f64');
add('typed Boolean default gives literal Boolean mapper context','7u8.checked_add(1u8).map_or(false,|_|true)');
add('successful checked mapper invokes diverging body','7u8.checked_add(1u8).map_or(9u8,|_|panic!("mapper invoked"))==0u8','mapper invoked');
add('successful checked arithmetic still evaluates eager default','7u8.checked_add(1u8).map_or(fallback(),|value|value)==8u8','default eager','pub fn fallback()->u8{panic!("default eager")}');
add('failed checked arithmetic still constructs mapper eagerly','255u8.checked_add(1u8).map_or(9u8,factory())==9u8','factory eager','pub fn factory()->fn(u8)->u8{panic!("factory eager")}');
add('checked default precedes mapper factory','255u8.checked_add(1u8).map_or(fallback(),factory())==9u8','default first','pub fn fallback()->u8{panic!("default first")}pub fn factory()->fn(u8)->u8{panic!("factory second")}');
add('checked receiver precedes argument default and mapper factory','input().checked_add(argument()).map_or(fallback(),factory())==0u8','receiver first','pub fn input()->u8{panic!("receiver first")}pub fn argument()->u8{panic!("argument second")}pub fn fallback()->u8{panic!("default third")}pub fn factory()->fn(u8)->u8{panic!("factory fourth")}');
add('checked mapper factory retains actual returned function signature','7u8.checked_add(1u8).map_or(9u8,factory())==8u8',null,'pub fn identity(value:u8)->u8{value}pub fn factory()->fn(u8)->u8{identity}');
premises.push({operation:'readRustCheckedIntegerConsumerArity',args:['map_or'],expected:2},{operation:'readRustCheckedIntegerResultOwner',args:['map_or','u8','u64'],expected:'u64'},{operation:'planRustCheckedIntegerConsumer',args:['map_or'],expected:['condition','success','fallback','if']});
refusals.push({name:'checked mapper must retain actual callable source kind',source:'pub fn result()->u8{7u8.checked_add(1u8).map_or(9u8,true)}fn main(){}',kind:'unsupported',reason:'Rust checked integer mapper: a Result mapper needs a direct one-argument callable with its actual integer parameter and scalar result'});
writeProjectData(new URL('../../parity/fixtures/rust-checked-integer-mappers.lino',import.meta.url),{schemaVersion:1,model:'Rust checked integer scalar mapper consumers 1',retainSourceTypeOwners:true,cases,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/rust-checked-integer-program','meta-language/source-program/rust-primitive-try-from-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or'],boundary:'Unexecuted nonescaping immutable Copy scalar mapper originals. General mapped Option values, mutable/non-Copy callbacks, generic source control and full grammars remain requirements.'});
