// Literal complete modules/contracts only. Never parse fixture or assertion text.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,owner,expected,helpers='',following=false){
 const module='canonical_discovery_case_'+cases.length,source=(following?'':helpers)+'pub fn select()->'+owner+'{'+body+'}pub fn result()->'+owner+'{select()}'+(following?helpers:'')+'fn main(){}';
 const contract={kind:'primitive',path:[owner],arguments:[]};
 cases.push({name,module,source,expected:String(expected),value:String(expected),abort:null,resultType:owner,
  checks:['const _: fn()->'+owner+'=CASE::select;','const _: fn()->'+owner+'=CASE::result;'],
  canonicalChecks:['const _: fn()->'+owner+'=CASE::select;','const _: fn()->'+owner+'=CASE::result;'],
  sourceContracts:[{name:'select',parameters:[],result:contract},{name:'result',parameters:[],result:contract}]});
}
add('ASCII byte predicate uses its actual canonical comparison','65u8.is_ascii_uppercase()','bool',true);
add('checked arithmetic retains its guarded consumer','250u8.checked_add(10u8).unwrap_or(3u8)','u8',3);
add('integer bits retain their unsigned count result','15u8.count_ones()','u32',4);
add('integer magnitude retains its unsigned result owner','(-5i8).unsigned_abs()','u8',5);
add('associated primitive selection retains its actual owner','u8::max(3u8,7u8)','u8',7);
add('standard TryFrom retains its selected default','u8::try_from(256u16).unwrap_or(3u8)','u8',3);
add('primitive receiver selection has no free-function dependency','3u8.max(7u8)','u8',7);
add('signed primitive property keeps its Boolean result','(-7i8).is_negative()','bool',true);
add('saturating addition retains exact upper saturation','250u8.saturating_add(10u8)','u8',255);
add('saturating multiplication retains exact upper saturation','20u8.saturating_mul(20u8)','u8',255);
add('wrapping addition retains modular arithmetic','250u8.wrapping_add(10u8)','u8',4);
add('standard scalar Clone retains its borrowed argument','let value:u8=7u8;std::clone::Clone::clone(&value)','u8',7);
add('standard String ToOwned retains its complete original value','let value:String=String::from("kept");let copied=std::borrow::ToOwned::to_owned(&value);copied.len()==4usize','bool',true);
add('standard vector Clone retains its actual element owner','let values:Vec<u8>=vec![7u8];let copied=std::clone::Clone::clone(&values);copied[0]','u8',7);
add('wrapping shifts use their actual unsigned count','129u8.wrapping_shl(1u32)','u8',2);
add('standard From retains its source and destination domains','u16::from(7u8)','u16',7);
add('declared method-name collision cannot change integer semantics','15u8.count_ones()','u32',4,'fn count_ones(value:u8)->u32{99u32}');
add('declared method-name collision cannot change wrapping semantics','250u8.wrapping_add(10u8)','u8',4,'fn wrapping_add(value:u8,other:u8)->u8{99u8}');
add('forward sibling source result retains primitive provenance','source().count_ones()','u32',4,'fn source()->u8{15u8}',true);
add('checked standard String view reaches its nonescaping consumer','let value:String=String::from("kept");width(std::string::String::as_str(&value))==4usize','bool',true,'fn width(value:&str)->usize{value.len()}');
const printedSources=[{name:'canonical checked arithmetic retains receiver argument fallback order',source:'pub fn left()->u8{println!("left");250u8}pub fn right()->u8{println!("right");10u8}pub fn fallback()->u8{println!("fallback");3u8}fn main(){println!("{}",left().checked_add(right()).unwrap_or(fallback()));}',lines:['left','right','fallback','3']}];
writeProjectData(new URL('../../parity/fixtures/rust-canonical-discovery.lino',import.meta.url),{
 schemaVersion:1,retainSourceTypeOwners:true,model:null,cases,printedSources,
 refusals:[{name:'a synthetic free callee cannot supply a standard integer receiver',source:'fn count_ones(value:String)->u32{99u32}pub fn select(value:String)->u32{value.count_ones()}fn main(){}',kind:'unsupported',reason:'Rust integer bit operation: a proven standard fixed integer receiver with its actual source owner is required',originalRustAccepted:false}],
 malformed:[{name:'canonical source discovery retains primitive method arity',source:'pub fn select()->u32{15u8.count_ones(1u8)}fn main(){}'}],
 source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64'},
 sourceFamilies:['byte-ascii','checked-integer','integer-bits','integer-magnitudes','primitive-associated-calls','primitive-try-from','primitive-selections','standard-owned-calls','standard-string-views','wrapping-shifts'],
 boundary:'Unexecuted source draft. Discovery and final parsing use actual shared canonical projections and retain ordinary semantic checks, source-owner validation, operation/arity guards and independent native products. Removal of synthetic callees does not establish complete Rust grammar, general trait dispatch, ownership or project translation.'});
