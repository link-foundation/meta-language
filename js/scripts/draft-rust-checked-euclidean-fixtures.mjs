// Independent mathematical Euclidean identity; source/data authoring only.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],premises=[],refusals=[];
function add(name,body,abort=null,helpers=''){cases.push({name,module:'checked_euclidean_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:abort===null?'true':null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;']});}
for(const signed of [false,true])for(const [bits,pointer]of [[8,false],[16,false],[32,false],[64,false],[128,false],[64,true]]){
 const owner=pointer?(signed?'isize':'usize'):(signed?'i':'u')+bits,limit=1n<<BigInt(signed?bits-1:bits),minimum=signed?-limit:0n,maximum=limit-1n;
 const pairs=[[0n,0n],[0n,1n],[7n,2n],[maximum,1n],[maximum,maximum],...signed?[[-7n,2n],[7n,-2n],[-7n,-2n],[-6n,2n],[-1n,minimum],[minimum,minimum],[minimum,-1n],[minimum,3n],[maximum,minimum]]:[]];
 for(const [left,right]of pairs){const present=right!==0n&&!(signed&&left===minimum&&right===-1n),absolute=right<0n?-right:right,remainder=present?((left%absolute)+absolute)%absolute:0n,quotient=present?(left-remainder)/right:0n;
  for(const method of ['checked_div_euclid','checked_rem_euclid']){const expression='left.'+method+'('+right+owner+')',value=method==='checked_div_euclid'?quotient:remainder,prefix='let left='+left+owner+';';
   add(owner+' '+method+' '+left+' '+right+' retains exact Euclidean value',prefix+expression+'.unwrap_or(7'+owner+')=='+(present?value:7n)+owner);
   add(owner+' '+method+' '+left+' '+right+' retains independent presence',prefix+expression+'.is_some()=='+present);
  }
 }
 for(const consumer of ['unwrap','expect']){const argument=consumer==='expect'?'"Euclidean failure"':'';
  add(owner+' Euclidean zero division '+consumer,'let left=7'+owner+';left.checked_div_euclid(0'+owner+').'+consumer+'('+argument+')==0'+owner,consumer==='expect'?'Euclidean failure':'called `Option::unwrap()` on a `None` value');
 }
}
add('Euclidean input precedes divisor and eager default','first().checked_div_euclid(second()).unwrap_or(fallback())==0i64','first operand','pub fn first()->i64{panic!("first operand")}pub fn second()->i64{panic!("second operand")}pub fn fallback()->i64{panic!("default third")}');
add('Euclidean divisor precedes eager default','7i64.checked_rem_euclid(second()).unwrap_or(fallback())==0i64','second operand','pub fn second()->i64{panic!("second operand")}pub fn fallback()->i64{panic!("default third")}');
add('successful Euclidean computation still evaluates eager default','7i64.checked_div_euclid(2i64).unwrap_or(fallback())==3i64','default eager','pub fn fallback()->i64{panic!("default eager")}');
const condition=['right','zero','eq','false','operand','minimum','eq','right','negative unit','ne','true','if','if'];
for(const method of ['checked_div_euclid','checked_rem_euclid'])for(const signed of [false,true]){premises.push({operation:'readRustCheckedIntegerArity',args:[method,true],expected:1},{operation:'readRustCheckedIntegerArity',args:[method,false],expected:-1},{operation:'planRustCheckedIntegerCondition',args:[method,signed],expected:signed?condition:['right','zero','ne']});}
const quotient=['operand','right','div'],remainder=['operand','right','rem'];
premises.push({operation:'planRustCheckedIntegerSuccess',args:['checked_div_euclid',false],expected:quotient},{operation:'planRustCheckedIntegerSuccess',args:['checked_rem_euclid',false],expected:remainder},{operation:'planRustCheckedIntegerSuccess',args:['checked_div_euclid',true],expected:['operand','right','rem','zero','lt','right','zero','gt','operand','right','div','negative unit','add','operand','right','div','negative unit','sub','if','operand','right','div','if']},{operation:'planRustCheckedIntegerSuccess',args:['checked_rem_euclid',true],expected:['operand','right','rem','zero','lt','right','zero','gt','operand','right','rem','right','add','operand','right','rem','right','sub','if','operand','right','rem','if']});
refusals.push({name:'Euclidean divisor retains its actual integer width',source:'pub fn result()->u8{7u8.checked_div_euclid(2u16).unwrap_or(0u8)}fn main(){}',kind:'unsupported',reason:'Rust checked integer argument: the actual argument must retain the receiver type and source owner'});
writeProjectData(new URL('../../parity/fixtures/rust-checked-euclidean.lino',import.meta.url),{schemaVersion:1,model:'Rust checked Euclidean integer division and remainder 1',retainSourceTypeOwners:true,cases,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/rust-checked-integer-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/std/primitive.i64.html#method.checked_div_euclid','https://doc.rust-lang.org/std/primitive.i64.html#method.checked_rem_euclid'],boundary:'Unexecuted originals derive expectations from 0<=remainder<abs(divisor) and exact reconstruction, independently from the truncating division/correction implementation. Every wider source/Option/ownership/grammar requirement remains.'});
