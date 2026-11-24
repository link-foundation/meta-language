// Complete original programs and independent premises; never parse or execute.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],premises=[],refusals=[];
function add(name,body,helpers='',abort=null){cases.push({name,module:'nominal_copy_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:abort===null?'true':null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;']});}
for(const owner of ['u8','i8','u16','i16','u32','i32','u64','i64','u128','i128','usize','isize']){
 const record='#[derive(Clone,Copy)] pub struct Value{pub number:'+owner+'}';
 add(owner+' actual nominal Copy permits typed closure parameters','let read:fn(Value)->'+owner+'=|Value{number}|number;read(Value{number:7'+owner+'})==7'+owner,record);
 add(owner+' actual nominal Copy permits repeated closure captures','let value=Value{number:7'+owner+'};let read=||value.number;read()==7'+owner+'&&read()==7'+owner,record);
}
add('a nominal Copy alias retains its complete record','let read:fn(Pair)->u16=|whole@Pair{first,..}|u16::from(first)+whole.last;read(Pair{first:3u8,last:4u16})==7u16','#[derive(Copy,Clone)]pub struct Pair{pub first:u8,pub last:u16}');
add('tuple structs carry nominal Copy declaration facts','let read:fn(Pair)->u16=|Pair(first,last)|u16::from(first)+last;read(Pair(3u8,4u16))==7u16','#[derive(Clone,Copy)]pub struct Pair(pub u8,pub u16);');
add('a unit struct is Copy only through its source trait','let read:fn(Marker)->bool=|Marker|true;read(Marker)','#[derive(Clone,Copy)]pub struct Marker;');
add('all enum variants participate in nominal Copy','let read:fn(Event)->u8=|event|match event{Event::Empty=>0u8,Event::Data(value)=>value};read(Event::Data(7u8))==7u8&&read(Event::Empty)==0u8','#[derive(Copy,Clone)]pub enum Event{Empty,Data(u8)}');
add('named enum fields participate in nominal Copy','let read:fn(Event)->u16=|event|match event{Event::Data{first,last}=>u16::from(first)+last};read(Event::Data{last:4u16,first:3u8})==7u16','#[derive(Clone,Copy)]pub enum Event{Data{first:u8,last:u16}}');
add('forward nominal dependencies reach their field proof','let read:fn(Outer)->u8=|Outer{leaf}|leaf.number;read(Outer{leaf:Leaf{number:7u8}})==7u8','#[derive(Copy,Clone)]pub struct Outer{pub leaf:Leaf}#[derive(Copy,Clone)]pub struct Leaf{pub number:u8}');
add('nested tuple field proof retains nominal dependencies','let read:fn(Outer)->u16=|Outer{pair:(leaf,last)}|u16::from(leaf.number)+last;read(Outer{pair:(Leaf{number:3u8},4u16)})==7u16','#[derive(Clone,Copy)]pub struct Outer{pub pair:(Leaf,u16)}#[derive(Clone,Copy)]pub struct Leaf{pub number:u8}');
add('module aliases retain actual Copy owner identities','let read:fn(Outer)->u8=|Outer{leaf}|leaf.number;read(Outer{leaf:LeafAlias{number:7u8}})==7u8','pub mod model{#[derive(Clone,Copy)]pub struct Leaf{pub number:u8}}use crate::model::Leaf as LeafAlias;#[derive(Clone,Copy)]pub struct Outer{pub leaf:LeafAlias}');
add('shared reference fields do not require their str target to be Copy','let read:fn(View)->bool=|View{text}|text=="content";read(View{text:"content"})','#[derive(Clone,Copy)]pub struct View{pub text:&\'static str}');
add('nested tuple imports resolve from the complete header catalog','let read:fn(Outer)->u16=|Outer{pair:(leaf,last)}|u16::from(leaf.number)+last;read(Outer{pair:(LeafAlias{number:3u8},4u16)})==7u16','#[derive(Clone,Copy)]pub struct Outer{pub pair:(LeafAlias,u16)}use crate::model::Leaf as LeafAlias;pub mod model{#[derive(Clone,Copy)]pub struct Leaf{pub number:u8}}');
add('split source derive attributes retain both trait facts','let read:fn(Value)->u8=|Value{number}|number;read(Value{number:7u8})==7u8','#[derive(Clone)]#[derive(Copy)]pub struct Value{pub number:u8}');
add('nominal Copy tuples support actual source product projection','let pair=(Value{number:3u8},4u16);u16::from(pair.0.number)+pair.1==7u16','#[derive(Clone,Copy)]pub struct Value{pub number:u8}');
add('nominal Copy receiver failure remains observable','let read:fn(Value)->u8=|Value{number}|number;read(receiver())==7u8','#[derive(Clone,Copy)]pub struct Value{pub number:u8}pub fn receiver()->Value{panic!("nominal Copy receiver")}','nominal Copy receiver');
const good='every field of every derived Copy variant needs an actual Copy proof';
for(const [name,source,reason] of [
 ['Copy without an actual Clone derivation','#[derive(Copy)]pub struct Value{number:u8}','a derived Copy owner requires its actual Clone derivation'],
 ['a non-Copy String field cannot derive Copy','#[derive(Copy,Clone)]pub struct Value{number:String}',good],
 ['a non-Copy nested nominal field cannot derive Copy','pub struct Leaf{number:u8}#[derive(Clone,Copy)]pub struct Value{leaf:Leaf}',good],
 ['every enum variant must have Copy fields','#[derive(Clone,Copy)]pub enum Event{Empty,Data(String)}',good],
 ['a Box field is not a shared-reference Copy field','#[derive(Clone,Copy)]pub struct Value{number:Box<u8>}',good],
])refusals.push({name,source:source+'pub fn result()->bool{true}fn main(){}',kind:'unsupported',reason:'Rust nominal Copy derivation: '+reason,span:{start:source.indexOf('#'),end:source.indexOf(']')+1}});
function plan(args,expected){premises.push({operation:'planRustNominalCopyOwners',args,expected});}
plan([['Leaf','Outer'],[true,true],[true,true],[0,1],[1,1],[-1,0],[true,false]],[0,1,1]);
plan([['Outer','Leaf'],[true,true],[true,true],[0,1],[1,1],[1,-1],[false,true]],[0,1,1]);
plan([['Plain'],[false],[false],[0],[1],[-1],[true]],[0,0]);
plan([['NoClone'],[true],[false],[0],[0],[],[]],[-2,0]);
plan([['Bad'],[true],[true],[0],[1],[-1],[false]],[-3,0]);
plan([['A','B'],[true,true],[true,true],[0,1],[1,1],[1,0],[false,false]],[-3,0]);
plan([['A'],[true],[true],[0],[1],[0],[true]],[0,1]);
plan([['Empty'],[true],[true],[0],[0],[],[]],[0,1]);
plan([['A','A'],[false,false],[false,false],[0,0],[0,0],[],[]],[-1,1]);
plan([['A'],[false],[false],[0],[1],[1],[false]],[-1]);
plan([['A'],[false],[false],[0],[1],[],[]],[-1,0]);
plan([['A'],[],[false],[0],[0],[],[]],[-1]);
plan([[],[],[],[],[],[],[]],[0]);
plan([['A','B'],[false,false],[false,false],[0,0],[1,1],[-1,-1],[true,true]],[-1,1]);
plan([['A'],[false],[false],[0],[0],[-1],[true]],[-1]);
for(const [args,expected] of [[['fixed',[],true],true],[['named',[],true],false],[['tuple',[],true],false],[['string',[],false],false],[['array',[],true],false],[['unknown',[],true],false],[['function',[],true],true],[['string',['reference'],false],true],[['named',['reference','box'],false],true],[['fixed',['box'],true],false],[['fixed',['mutable reference'],true],false]])premises.push({operation:'acceptRustNominalCopyFieldLeaf',args,expected});
writeProjectData(new URL('../../parity/fixtures/rust-nominal-copies.lino',import.meta.url),{schemaVersion:1,model:'Rust actual nominal Copy declaration graph 1',retainSourceTypeOwners:true,cases,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/rust-nominal-copy-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/reference/special-types-and-traits.html#copy','https://doc.rust-lang.org/std/marker/trait.Copy.html'],boundary:'Unexecuted originals and shared graph premises. Actual source derives and all constructor fields determine nominal Copy; field shape alone never supplies the trait. Manual trait implementations, generic bounds, full non-Copy ownership and all complete grammar/translation requirements remain active.'});
