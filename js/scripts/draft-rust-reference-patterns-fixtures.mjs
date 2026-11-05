// Source authoring only; independent reference-pattern originals remain deferred.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],refusals=[],premises=[];
function add(name,body,helpers='',abort=null,checks=[]){cases.push({name,module:'reference_pattern_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:abort===null?'true':null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;',...checks]});}
for(const owner of ['u8','i8','u16','i16','u32','i32','u64','i64','u128','i128','usize','isize'])for(const [pattern,depth] of [['&value',1],['&&value',2],['& &value',2],['&&&value',3],['&&&&value',4]]){
 let bindings='let value=7'+owner+';';let previous='value';for(let index=0;index<depth;index+=1){const name='reference'+index;bindings+='let '+name+'=&'+previous+';';previous=name;}
 add(owner+' reference match '+pattern,bindings+'match '+previous+'{'+pattern+'=>value==7'+owner+'}');
}
for(const [name,pattern] of [['joint literal','&&7u8'],['spaced literal','& &7u8'],['joint wildcard','&&_'],['triple literal','&&&7u8']]){
 const depth=name==='triple literal'?3:2,source='let value=7u8;let first=&value;let second=&first;'+(depth===3?'let third=&second;':''),input=depth===3?'third':'second';
 add(name+' matches its actual source reference layers',source+'match '+input+'{'+pattern+'=>true,_=>false}');
}
for(const present of [false,true]){
 const source='let value='+(present?'7':'8')+'u8;let first=&value;let second=&first;';
 add('conditional reference literal retains branch '+present,source+'(if let &&7u8=second{true}else{false})=='+String(present));
}
add('guard reads the scalar bound after two actual reference layers','let value=7u8;let first=&value;let second=&first;match second{&&selected if selected==7u8=>true,_=>false}');
add('reference or patterns retain both original alternatives','let value=8u8;let first=&value;let second=&first;match second{&&7u8|&&8u8=>true,_=>false}');
add('one consumed layer retains an actual scalar reference','let value=7u8;let first=&value;let second=&first;match second{&selected=>read(selected)==7u8}','pub fn read(value:&u8)->u8{*value}',null,['const _: fn(&u8)->u8=CASE::read;']);
add('Boolean references retain exact pattern and fallback','let value=true;let first=&value;let second=&first;match second{&&true=>true,&&false=>false}');
add('joint reference patterns preserve an outer name shadow','let selected=99u8;let value=7u8;let first=&value;let second=&first;(match second{&&selected=>selected==7u8})&&selected==99u8');
add('conditional chains retain reference patterns and Boolean operators','let value=7u8;let first=&value;let second=&first;if let &&selected=second&&selected==7u8{true}else{false}');
add('Boolean conjunction retains its two operands','left()&&right()','pub fn left()->bool{panic!("left Boolean operand")}pub fn right()->bool{panic!("right Boolean operand")}','left Boolean operand');
add('ordinary Boolean short circuit never enters its second operand','(false&&right())==false','pub fn right()->bool{panic!("unreachable Boolean operand")}');
add('joint type spelling remains a source function signature','let value=7u8;let first=&value;read(&first)==7u8','pub fn read(value:&&u8)->u8{match value{&&selected=>selected}}',null,['const _: fn(&&u8)->u8=CASE::read;']);
for(const [token,expected] of [['&&',['&','&']],['&',[]],['&&&',[]],['||',[]],['>>',[]],['',[]]])premises.push({operation:'readRustReferenceTokenParts',args:[token],expected});
for(const [token,expected] of [['>>',['>','>']],['>=',['>','=']],['>>=',['>','>','=']],['&&',[]]])premises.push({operation:'readRustTypeClosingTokenParts',args:[token],expected});
refusals.push({name:'reference pattern never supplies missing actual layers',source:'pub fn result()->u8{let value=7u8;let reference=&value;match reference{&&selected=>selected}}fn main(){}',kind:'unsupported',reason:'source pattern: a reference pattern requires a proven reference source type'});
writeProjectData(new URL('../../parity/fixtures/rust-reference-patterns.lino',import.meta.url),{schemaVersion:1,model:'Rust contextual compound reference patterns 1',retainSourceTypeOwners:true,cases,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/rust-type-token-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/reference/patterns.html#reference-patterns','https://doc.rust-lang.org/reference/tokens.html#punctuation'],boundary:'Unexecuted independent original matching, CST, codec, four-emission and native controls. Splitting is contextual; Boolean conjunction remains an expression token. Mutable/non-Copy patterns, escaping borrow regions and full Rust semantics remain requirements.'});
