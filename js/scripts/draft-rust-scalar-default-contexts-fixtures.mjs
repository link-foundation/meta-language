// Source-only independent scalar-default originals; never execute represented code.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],refusals=[],premises=[];
function add(name,expression,expected,helpers='',abort=null){cases.push({name,module:'scalar_default_'+cases.length,source:helpers+'pub fn result()->bool{'+expression+'=='+expected+'}',value:abort===null?'true':null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;']});}
for(const owner of ['u8','i8','u16','i16','u32','i32','u64','i64','u128','i128','usize','isize']){
 for(const present of [false,true]){
  const checked=present?'7'+owner+'.checked_add(0'+owner+')':'0'+owner+'.checked_div(0'+owner+')';
  add(owner+' checked eager default receives integer mapper context '+present,checked+'.map_or(0,|value|value)',(present?'7':'0')+owner);
  add(owner+' checked unwrap default receives integer context '+present,checked+'.unwrap_or(0)',(present?'7':'0')+owner);
 }
 const signed=owner.startsWith('i');
 const input=signed?'7u8':'7i8';
 add(owner+' TryFrom eager default receives integer mapper context',owner+'::try_from('+input+').map_or(0,|value|value)','7'+owner);
 add(owner+' TryFrom unwrap default receives integer context',owner+'::try_from('+input+').unwrap_or(0)','7'+owner);
 if(!signed)add(owner+' failed TryFrom retains typed default',owner+'::try_from(-1i8).map_or(0,|value|value)','0'+owner);
}
for(const [name,query] of [['checked','1u8.checked_add(1u8)'],['TryFrom','u8::try_from(2u16)']]){
 add(name+' typed floating default remains floating',query+'.map_or(0.5f64,f64::from)','2.0f64');
 add(name+' typed Boolean default remains Boolean',query+'.map_or(false,|value|value==2u8)','true');
 add(name+' lexical signed default retains its owner','{let fallback=-1i16;'+query+'.map_or(fallback,i16::from)}','2i16');
 add(name+' eager default effect precedes mapper factory',query+'.map_or(fallback(),factory())','0u8','pub fn fallback()->u8{panic!("default first")}pub fn factory()->fn(u8)->u8{panic!("mapper second")}','default first');
 const feature=name==='checked'?'Rust checked integer default':'Rust primitive TryFrom default';
 for(const [label,expression,result,reason] of [
  ['integer cannot become floating',query+'.map_or(0,f64::from)','f64','an unsuffixed Rust integer literal needs an actual integer result context'],
  ['unsigned negative zero cannot be a default',query+'.unwrap_or(-0)','u8','an unsigned Rust result cannot use unary negation'],
  ['unsigned negation cannot be a mapper default',query+'.map_or(-0,|value|value)','u8','an unsigned Rust result cannot use unary negation']
 ])refusals.push({name:name+' '+label,source:'pub fn result()->'+result+'{'+expression+'}fn main(){}',kind:'unsupported',reason:feature+': '+reason});
}
premises.push({operation:'readRustCopyClosureLiteralResultError',args:['num',false,false,'fixed',false],expected:''},{operation:'readRustCopyClosureLiteralResultError',args:['num',false,false,'float',false],expected:'an unsuffixed Rust integer literal needs an actual integer result context'},{operation:'readRustCopyClosureLiteralResultError',args:['unary',true,false,'fixed',false],expected:'an unsigned Rust result cannot use unary negation'});
writeProjectData(new URL('../../parity/fixtures/rust-scalar-default-contexts.lino',import.meta.url),{schemaVersion:1,model:'Rust immediate integer consumer scalar default contexts 1',retainSourceTypeOwners:true,cases,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/rust-checked-integer-program','meta-language/source-program/rust-primitive-try-from-program','meta-language/source-program/rust-copy-closure-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/reference/expressions/literal-expr.html#integer-literal-expressions','https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or','https://doc.rust-lang.org/std/result/enum.Result.html#method.map_or'],boundary:'Unexecuted original scalar defaults, native products and source transport controls. No full-source inference or grammar completion claim.'});
