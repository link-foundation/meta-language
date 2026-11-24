// Deferred originals and independent outcomes; no represented code executes.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],premises=[],refusals=[];
function add(name,body,abort=null,helpers=''){cases.push({name,module:'result_mapper_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:abort===null?'true':null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;']});}
for(const owner of ['u8','i8','u16','i16','u32','i32','u64','i64','u128','i128','usize','isize']){
 add(owner+' identity mapper retains exact input and inferred fallback',owner+'::try_from(7u128).map_or(9,|value|value)==7'+owner);
 add(owner+' eager default is selected on negative conversion failure',owner+'::try_from(-1i128).map_or(9'+owner+',|value|value)=='+(owner.startsWith('u')?'9':'-1')+owner);
 add(owner+' Copy capture remains observable in mapped arithmetic','let increment=2'+owner+';'+owner+'::try_from(7u128).map_or(9'+owner+',|value|value+increment)==9'+owner);
}
for(const [owner,value]of [['u8','255u128'],['i8','127i128'],['u64','18446744073709551615u128'],['i64','-9223372036854775808i128'],['u128','340282366920938463463374607431768211455u128'],['i128','-170141183460469231731687303715884105728i128']])add(owner+' mapper preserves its complete wide endpoint',owner+'::try_from('+value+').map_or(0'+owner+',|value|value)=='+value.slice(0,-4)+owner);
add('mapper changes the scalar result width','u8::try_from(7u16).map_or(99u64,|value|u64::from(value)+1u64)==8u64');
add('mapper changes integer input to Boolean output','u8::try_from(7u16).map_or(false,|value|value>6u8)');
add('mapper changes integer input to binary64 output','u8::try_from(7u16).map_or(-1.5f64,|value|f64::from(value)+0.5f64)==7.5f64');
add('named mapper retains actual signature','u8::try_from(7u16).map_or(9u64,widen)==7u64',null,'pub fn widen(value:u8)->u64{u64::from(value)}');
add('pointer factory returns actual mapper signature','u8::try_from(7u16).map_or(9u64,factory())==7u64',null,'pub fn widen(value:u8)->u64{u64::from(value)}pub fn factory()->fn(u8)->u64{widen}');
add('failed conversion cannot invoke a diverging mapper','u8::try_from(256u16).map_or(9u8,|value|->u8{panic!("unreachable mapper")})==9u8');
add('eager fallback is evaluated even on successful conversion','u8::try_from(7u16).map_or(fallback(),|value|value)==7u8','fallback eager','pub fn fallback()->u8{panic!("fallback eager")}');
add('callable factory is eager even on conversion failure','u8::try_from(256u16).map_or(9u8,factory())==9u8','mapper factory','pub fn factory()->fn(u8)->u8{panic!("mapper factory")}');
add('input precedes fallback and mapper factory','u8::try_from(input()).map_or(fallback(),factory())==0u8','input first','pub fn input()->u16{panic!("input first")}pub fn fallback()->u8{panic!("fallback second")}pub fn factory()->fn(u8)->u8{panic!("factory third")}');
add('fallback precedes mapper factory','u8::try_from(256u16).map_or(fallback(),factory())==0u8','fallback first','pub fn fallback()->u8{panic!("fallback first")}pub fn factory()->fn(u8)->u8{panic!("factory second")}');
for(const owner of ['u8','i8','u16','i16','u32','i32','u64','i64','u128','i128','usize','isize']){
 add(owner+' typed fallback provides an unsuffixed literal result context',owner+'::try_from(7u128).map_or(9'+owner+',|_|1)==1'+owner);
 add(owner+' typed fallback provides a zero literal result context',owner+'::try_from(7u128).map_or(9'+owner+',|_|0)==0'+owner);
 add(owner+' typed fallback provides a diverging success result context',owner+'::try_from(7u128).map_or(9'+owner+',|_|panic!("contextual mapper"))==0'+owner,'contextual mapper');
}
add('Boolean fallback provides the result context for a literal Boolean mapper','u8::try_from(7u16).map_or(false,|_|true)');
add('Boolean fallback provides the result context for a diverging mapper','u8::try_from(7u16).map_or(false,|_|panic!("Boolean mapper"))','Boolean mapper');
add('binary64 fallback provides a literal floating result context','u8::try_from(7u16).map_or(-1.5f64,|_|0.5)==0.5f64');
add('typed source default does not coerce a named mapper parameter owner','u8::try_from(7u16).map_or(1u64,mapper)==7u64',null,'pub fn mapper(value:u8)->u64{u64::from(value)}');
const reason='a Result mapper needs a direct one-argument callable with its actual integer parameter and scalar result';
for(const args of [['function',1,0,true,true],['function',0,0,true,true],['function',2,0,true,true],['function',1,1,true,true],['function',1,0,false,true],['function',1,0,true,false],['fixed',1,0,true,true]])premises.push({operation:'readRustPrimitiveTryFromMapperError',args,expected:args[0]==='function'&&args[1]===1&&args[2]===0&&args[3]&&args[4]?'':reason});
for(const [args,expected]of [[['fixed','8',false,'primitive','u8',0,true,0],'u8'],[['bool','',false,'primitive','bool',0,true,0],'bool'],[['float','',false,'primitive','f64',0,true,0],'f64'],[['fixed','8',false,'primitive','u16',0,true,0],''],[['fixed','8',false,'primitive','u8',0,true,1],''],[['fixed','8',false,'primitive','u8',0,false,0],''],[['string','',false,'library','std::string::String',0,true,0],'']])premises.push({operation:'readRustPrimitiveTryFromMapperOwner',args,expected});
refusals.push({name:'non-callable mapper retains its actual source type',source:'pub fn result()->u8{u8::try_from(7u16).map_or(9u8,true)}fn main(){}',kind:'unsupported',reason:'Rust primitive TryFrom mapper: '+reason});
writeProjectData(new URL('../../parity/fixtures/rust-primitive-try-from-mappers.lino',import.meta.url),{schemaVersion:1,model:'Rust standard integer Result scalar mappers 1',retainSourceTypeOwners:true,cases,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/rust-primitive-try-from-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=32 or 64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/std/result/enum.Result.html#method.map_or'],boundary:'Unexecuted immutable Copy scalar callback controls. General Result/error values, generic or mutable/non-Copy mapping and unrestricted source translation remain requirements.'});
