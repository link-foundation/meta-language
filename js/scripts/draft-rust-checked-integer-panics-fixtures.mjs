// Independent interval authoring, never represented checked-arithmetic execution.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],premises=[],refusals=[];
const unwrapMessage='called `Option::unwrap()` on a `None` value';
function add(name,body,abort=null,helpers=''){cases.push({name,module:'checked_panic_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:abort===null?'true':null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;']});}
for(const signed of [false,true])for(const [bits,pointer]of [[8,false],[16,false],[32,false],[64,false],[128,false],[64,true]]){
 const owner=pointer?(signed?'isize':'usize'):(signed?'i':'u')+bits,limit=1n<<BigInt(signed?bits-1:bits),minimum=signed?-limit:0n,maximum=limit-1n,modulus=1n<<BigInt(bits);
 const operations=[['checked_add',maximum,1n],['checked_add',0n,1n],['checked_sub',minimum,1n],['checked_sub',1n,1n],['checked_mul',maximum,2n],['checked_mul',2n,2n],['checked_div',1n,0n],['checked_div',minimum,signed?-1n:1n],['checked_div',7n,2n],['checked_rem',1n,0n],['checked_rem',minimum,signed?-1n:1n],['checked_rem',7n,2n],['checked_neg',minimum,null],['checked_neg',0n,null],['checked_neg',1n,null],...signed?[['checked_abs',minimum,null],['checked_abs',-1n,null]]:[],['checked_shl',maximum,1n],['checked_shl',1n,BigInt(bits)],['checked_shr',minimum,1n],['checked_shr',1n,BigInt(bits)]];
 for(const [method,left,right]of operations){
  let value=null;
  if(method==='checked_add')value=left+right;else if(method==='checked_sub')value=left-right;else if(method==='checked_mul')value=left*right;
  else if(method==='checked_neg')value=-left;else if(method==='checked_abs')value=left<0n?-left:left;
  else if(method==='checked_div'||method==='checked_rem'){if(right!==0n&&!(signed&&left===minimum&&right===-1n))value=method==='checked_div'?left/right:left%right;}
  else if(right<BigInt(bits)){if(method==='checked_shr')value=left>>right;else{value=(((left%modulus)+modulus)%modulus<<right)%modulus;if(signed&&value>=limit)value-=modulus;}}
  const present=value!==null&&value>=minimum&&value<=maximum;
  for(const consumer of ['unwrap','expect']){const message=consumer==='unwrap'?unwrapMessage:'checked failure',count=method==='checked_shl'||method==='checked_shr',argument=right===null?'':right+(count?'u32':owner),expression='value.'+method+'('+argument+').'+consumer+'('+(consumer==='expect'?'"checked failure"':'')+')';
   add(owner+' '+method+' '+left+' '+right+' '+consumer,'let value='+left+owner+';'+expression+'=='+(present?value:0n)+owner,present?null:message);
  }
 }
}
add('checked receiver evaluates before argument and expect message','input().checked_add(argument()).expect(message())==0u8','receiver first','pub fn input()->u8{panic!("receiver first")}pub fn argument()->u8{panic!("argument second")}pub fn message()->&\'static str{panic!("message third")}');
add('checked argument evaluates before expect message','7u8.checked_add(argument()).expect(message())==0u8','argument first','pub fn argument()->u8{panic!("argument first")}pub fn message()->&\'static str{panic!("message second")}');
add('successful checked arithmetic eagerly evaluates its expect message','7u8.checked_add(1u8).expect(message())==8u8','message eager','pub fn message()->&\'static str{panic!("message eager")}');
add('failed checked arithmetic evaluates its message before panic','255u8.checked_add(1u8).expect(message())==0u8','message first','pub fn message()->&\'static str{panic!("message first")}');
add('borrowed owned message retains exact Unicode and braces','let message=String::from("é😀 {checked}");255u8.checked_add(1u8).expect(&message)==0u8','é😀 {checked}');
add('successful expect retains its borrowed String owner','let message=String::from("checked");7u8.checked_add(1u8).expect(&message)==8u8&&message.len()==7usize');
add('empty expect message remains exactly empty','255u8.checked_add(1u8).expect("")==0u8','');
for(const [consumer,arity]of [['unwrap',0],['expect',1]]){premises.push({operation:'readRustCheckedIntegerConsumerArity',args:[consumer],expected:arity},{operation:'readRustCheckedIntegerResultOwner',args:[consumer,'usize',''],expected:'usize'},{operation:'planRustCheckedIntegerConsumer',args:[consumer],expected:consumer==='unwrap'?['condition','success','unwrap message','raise','if']:['condition','success','fallback','raise','if']});}
for(const [consumer,expected]of [['unwrap',true],['expect',true],['unwrap_or',false],['unknown',false]])premises.push({operation:'testRustOptionPanicConsumer',args:[consumer],expected});
for(const [consumer,message,expected]of [['unwrap','ignored',unwrapMessage],['expect','é😀 {checked}','é😀 {checked}'],['expect','',''],['unknown','ignored','']])premises.push({operation:'readRustOptionPanicMessage',args:[consumer,message],expected});
refusals.push({name:'checked expect cannot silently borrow an owned message',source:'pub fn result()->u8{7u8.checked_add(1u8).expect(String::from("message"))}fn main(){}',kind:'unsupported',reason:'Rust checked integer message: expect needs its actual immutable standard str message source'});
writeProjectData(new URL('../../parity/fixtures/rust-checked-integer-panics.lino',import.meta.url),{schemaVersion:1,model:'Rust checked integer Option panic consumers 1',retainSourceTypeOwners:true,cases,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/rust-checked-integer-program','meta-language/source-program/rust-panic-value-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap','https://doc.rust-lang.org/std/option/enum.Option.html#method.expect','https://doc.rust-lang.org/std/primitive.i64.html#method.checked_add'],boundary:'Unexecuted independent interval originals for all supported fixed/pointer owners and checked operators. Error values, stored Option control and remaining source/grammar/ownership requirements remain binding.'});
