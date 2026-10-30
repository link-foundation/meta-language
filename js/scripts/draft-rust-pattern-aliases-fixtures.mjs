// Independent complete original sources, authored without parsing or execution.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],premises=[],refusals=[];
function add(name,body,helpers='',abort=null,checks=[]){cases.push({name,module:'pattern_alias_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:abort===null?'true':null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;',...checks]});}
for(const owner of ['u8','i8','u16','i16','u32','i32','u64','i64','u128','i128','usize','isize']){
 add(owner+' alias binds the actual scalar behind a wildcard','let value=7'+owner+';match value{selected@_=>selected==7'+owner+'}');
 add(owner+' nested aliases retain one matched value','let value=7'+owner+';match value{outer@inner@_=>outer==7'+owner+'&&inner==7'+owner+'}');
 add(owner+' typed literal and its alias retain independent owner constraints','selected(7'+owner+')==7'+owner,'pub fn selected(value:'+owner+')->'+owner+'{match value{actual@7'+owner+'=>actual,_=>0'+owner+'}}',null,['const _: fn('+owner+')->'+owner+'=CASE::selected;']);
 add(owner+' explicit reference pattern preserves the scalar alias','let value=7'+owner+';match &value{&(selected@_)=>selected==7'+owner+'}');
}
add('wildcard alias preserves complete actual Boolean domain','match true{selected@_=>selected}');
add('floating wildcard aliases preserve signed values without reconstruction','match -1.5f64{selected@_=>selected==-1.5f64}');
add('unit alias retains the actual unit column','match (){selected@()=>match selected{()=>true}}');
add('tuple alias keeps ignored fields available through the actual tuple','match (3u8,4u16){whole@(first,_)=>whole.0==first&&whole.1==4u16}');
add('tuple rest aliases preserve unmentioned middle positions','match (1u8,2u16,3u32,4u64){whole@(first,..,last)=>whole.0==first&&whole.1==2u16&&whole.2==3u32&&whole.3==last}');
add('nested aliases bind their selected tuple field','match ((3u8,4u16),7u32){(whole@(first,_),last)=>whole.0==first&&whole.1==4u16&&last==7u32}');
add('record aliases retain ignored fields in declared order','match (Pair{last:4u16,first:3u8}){whole@Pair{first,..}=>whole.first==first&&whole.last==4u16}','pub struct Pair{pub first:u8,pub last:u16}');
add('variant alias retains actual variant and its ignored payload','match Event::Data(7u8){whole@Event::Data(_)=>match whole{Event::Data(value)=>value==7u8,_=>false},_=>false}','pub enum Event{Empty,Data(u8)}');
add('borrowed scalar alias remains an actual reference','let value=7u8;match &value{selected@_=>*selected==7u8}');
add('borrowed nested alias retains both original reference layers','let value=7u8;let reference=&value;match &reference{selected@_=>**selected==7u8}');
add('conditional aliases share their selected value','let value=7u8;if let selected@7u8=value{selected==7u8}else{false}');
add('guarded aliases retain false-guard fallback order','let value=7u8;match value{selected@_ if selected==8u8=>false,selected@_=>selected==7u8}');
add('alias bindings remain distinct across ordered alternatives','let value=8u8;match value{selected@7u8|selected@8u8=>selected==8u8,_=>false}');
add('macro aliases work with ignored source fields','matches!((3u8,4u16),whole@(_,4u16) if whole.0==3u8)');
add('aliases shadow an outer variable only within the matching body','let value=99u8;(match 7u8{value@_=>value==7u8})&&value==99u8');
add('closure pattern locals are not captures of equal outer spellings','let value=99u8;let read:fn(u8)->u8=|input|match input{value@_=>value};read(7u8)==7u8&&value==99u8');
add('scrutinee reads precede the source pattern binding','let value=7u8;let read=||match value{value@_=>value};read()==7u8');
add('nested closures capture the selected alias rather than a reconstruction','match 7u8{selected@_=>{let read=||selected;read()==7u8}}');
add('irrefutable let aliases retain whole tuple and selected field','let whole@(first,_):(u8,u16)=(3u8,4u16);whole.0==first&&whole.1==4u16');
add('closure product aliases retain a single source parameter','let read:fn((u8,u16))->u16=|whole@(first,_)|u16::from(first)+whole.1;read((3u8,4u16))==7u16');
add('receiver effects remain before wildcard alias selection','match receiver(){selected@_=>selected==7u8}','pub fn receiver()->u8{panic!("alias receiver")}','alias receiver');
add('a selected alias guard retains its source failure','match 7u8{selected@_ if guard(selected)=>false,_=>true}','pub fn guard(value:u8)->bool{panic!("alias guard")}','alias guard');
for(const [kind,expected] of [['alias',true],['bind',false],['ctor',false],['',false]])premises.push({operation:'testPatternAliasKind',args:[kind],expected});
for(const [args,expected] of [
 [[['alias','wild'],['whole',''],[1,-1],[0],[]],[0,1,1,0,0]],
 [[['alias','alias','wild'],['outer','inner',''],[1,2,-1],[0],[]],[0,1,2,0,0,0,1]],
 [[['alias','wild','alias','wild'],['left','','right',''],[1,-1,3,-1],[0,2],[]],[0,2,1,3,0,0,1,2]],
 [[['wild'],[''],[-1],[0],[]],[0,1,0]],
 [[['alias'],['whole'],[0],[0],[]],[-1]],
 [[['alias','wild'],['whole',''],[1,-1],[0],['whole']],[-1]],
 [[['alias','wild'],['_',''],[1,-1],[0],[]],[-1]],
 [[['alias'],['whole'],[-1],[0],[]],[-1]],
 [[['wild'],[],[-1],[0],[]],[-1]],
 [[['wild'],[''],[-1],[1],[]],[-1]],
])premises.push({operation:'planPatternAliasBindings',args,expected});
refusals.push({name:'an alias and its nested binding cannot reuse a source name',source:'pub fn result()->u8{match 7u8{value@value=>value}}fn main(){}',kind:'unsupported',reason:'duplicate source pattern binding: each pattern binds a source name at most once'});
writeProjectData(new URL('../../parity/fixtures/rust-pattern-aliases.lino',import.meta.url),{schemaVersion:1,model:'Rust actual matched value aliases 1',retainSourceTypeOwners:true,cases,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/pattern-alias-program','meta-language/source-program/callable-source-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/reference/patterns.html#identifier-patterns'],boundary:'Unexecuted original-source, transport, emission and native controls. Alias rows bind actual matched columns through a shared source plan; no partial pattern reconstructs a value. Closed source profiles retain their preceding interpretation. Ranges, mutable/non-Copy ownership, complete match ergonomics and all whole-language/grammar/concept requirements remain active.'});
