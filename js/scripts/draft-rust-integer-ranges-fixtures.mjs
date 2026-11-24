// Independent complete original programs and exact integer facts; no execution.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],premises=[],refusals=[];
function add(name,body,helpers='',abort=null){cases.push({name,module:'integer_range_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:abort===null?'true':null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;']});}
for(const [owner,bits,signed] of [['u8',8,false],['i8',8,true],['u16',16,false],['i16',16,true],['u32',32,false],['i32',32,true],['u64',64,false],['i64',64,true],['u128',128,false],['i128',128,true],['usize',64,false],['isize',64,true]]){
 const minimum=signed?String(-(1n<<BigInt(bits-1))):'0',maximum=String((1n<<BigInt(signed?bits-1:bits))-1n);
 add(owner+' inclusive exclusive and open bounds preserve source rows','choose(0'+owner+')==1u8&&choose(3'+owner+')==1u8&&choose(4'+owner+')==2u8&&choose(6'+owner+')==2u8&&choose(7'+owner+')==3u8','pub fn choose(value:'+owner+')->u8{match value{0..=3=>1u8,4..7=>2u8,7..=>3u8,_=>0u8}}');
 add(owner+' complete extrema compile without enumerating the domain','all('+minimum+owner+')&&all('+maximum+owner+')','pub fn all(value:'+owner+')->bool{match value{'+minimum+owner+'..='+maximum+owner+'=>true}}');
 add(owner+' overlapping ranges preserve first matching arm','choose(7'+owner+')==1u8&&choose(12'+owner+')==2u8&&choose(20'+owner+')==3u8','pub fn choose(value:'+owner+')->u8{match value{0..=10=>1u8,5..=15=>2u8,_=>3u8}}');
 add(owner+' singleton inclusive ranges retain their exact value','matches!(7'+owner+',7..=7)&&!matches!(8'+owner+',7..=7)');
 add(owner+' adjacent ranges prove complete coverage','all('+minimum+owner+')&&all('+maximum+owner+')','pub fn all(value:'+owner+')->bool{match value{..=3=>true,4..=>true}}');
 add(owner+' range aliases and guards retain source binding','match 7'+owner+'{selected@0..=9 if selected==8'+owner+'=>false,selected@1..=9=>selected==7'+owner+',_=>false}');
 add(owner+' ranges compose with other tuple columns','choose(2'+owner+',true)==1u8&&choose(2'+owner+',false)==2u8&&choose(20'+owner+',true)==3u8','pub fn choose(value:'+owner+',flag:bool)->u8{match (value,flag){(0..=3,true)=>1u8,(0..=3,false)=>2u8,_=>3u8}}');
 add(owner+' a closure retains contextual range scrutinee typing','let choose:fn('+owner+')->bool=|value|matches!(value,0..=9);choose(7'+owner+')&&!choose(10'+owner+')');
 if(signed)add(owner+' negative ranges and the zero boundary stay exact','choose(-7'+owner+')==1u8&&choose(0'+owner+')==2u8&&choose(7'+owner+')==3u8','pub fn choose(value:'+owner+')->u8{match value{..=-1=>1u8,0..1=>2u8,1..=>3u8}}');
}
add('open range fields remain distinct from tuple rest syntax','match (2u8,7u8){(..3,last)=>last==7u8,_=>false}');
add('open range payloads remain distinct from constructor rest syntax','match Event::Data(2u8,7u8){Event::Data(..3,last)=>last==7u8,_=>false}','pub enum Event{Data(u8,u8)}');
add('open ranges compose with conditional matching','if let ..3=2u8{true}else{false}');
add('byte endpoints retain the actual u8 source owner',"matches!(b'c',b'a'..=b'z')&&!matches!(b'0',b'a'..b'z')");
add('reference ranges compose with actual explicit reference patterns','let value=7u8;match &value{&(selected@1..=9)=>selected==7u8,_=>false}');
add('range endpoints infer an unsuffixed literal source integer owner','matches!(7,1u16..=9u16)');
add('nested nominal fields support integer range coverage','match Event::Data(7u128){Event::Data(value@1..=9)=>value==7u128,_=>false}','pub enum Event{Empty,Data(u128)}');
add('range receiver effects execute once before selecting any arm','match receiver(){0u8..=9u8=>true,_=>false}','pub fn receiver()->u8{panic!("range receiver")}','range receiver');
add('a false range guard preserves the next source alternative','let value=7u8;match value{selected@1..=9 if selected==8u8=>false,0..=10=>true,_=>false}');
function premise(operation,args,expected){premises.push({operation,args,expected});}
for(const [input,expected] of [['0','1'],['9','10'],['99','100'],['-1','0'],['-10','-9'],['-100','-99'],['340282366920938463463374607431768211455','340282366920938463463374607431768211456'],['-170141183460469231731687303715884105728','-170141183460469231731687303715884105727'],['01',''],['-0',''],['','']])premise('readRustIntegerRangeSuccessor',[input],expected);
for(const [a,b,order] of [['-10','-9',-1],['-1','0',-1],['-7','-7',0],['0','0',0],['10','9',1],['340282366920938463463374607431768211456','340282366920938463463374607431768211455',1],['x','0',2]])premise('compareRustIntegerRangeDecimals',[a,b],order);
for(const [value,expected] of [['0',true],['-7',true],['7',true],['00',false],['-0',false],['+1',false],['1.0',false],['',false]])premise('testRustIntegerRangeCanonicalDecimal',[value],expected);
premise('readRustIntegerRangeCuts',['0','256',['0','4',''],['4','7','']],['0','4','7','256']);
premise('readRustIntegerRangeCuts',['-128','128',['-128','0'],['0','128']],['-128','0','128']);
premise('readRustIntegerRangeCuts',['0','256',['8'],['8']],[]);
premise('planRustIntegerRangeRows',[['interval','interval','wild'],['0','4',''],['4','7',''],['0','4','7','256']],[0,3,2,0,2,2,1,2,1,2]);
premise('planRustIntegerRangeRows',[['interval'],['0'],['4'],['0','4','256']],[-2,1]);
premise('planRustIntegerRangeRows',[['interval','interval'],['-128','0'],['0','128'],['-128','0','128']],[0,2,1,0,1,1]);
premise('planRustIntegerRangeRows',[['interval'],['1'],['4'],['0','4','256']],[-1]);
premise('planRustIntegerRangeRows',[['lit'],['7'],['9'],['7','9']],[-1]);
premise('planRustIntegerRangeRows',[['wild'],['0'],['1'],['0','1']],[-1]);
premise('readRustIntegerRangeSourceError',[true,'..',true,false],'');
premise('readRustIntegerRangeSourceError',[true,'..=',false,true],'');
premise('readRustIntegerRangeSourceError',[true,'..',false,false],'a range pattern needs at least one endpoint');
premise('readRustIntegerRangeSourceError',[true,'..=',true,false],'an inclusive range pattern needs its upper endpoint');
premise('readRustIntegerRangeSourceError',[false,'..',true,true],'ranges are outside the portable pattern language');
for(const [next,expected] of [[',',true],[')',true],['7',false],['-',false],['=',false]])premise('testRustIntegerRangeRestFollower',[next],expected);
for(const [name,source,kind,reason] of [
 ['an exclusive empty range is invalid','pub fn result()->bool{matches!(3u8,3..3)}','type','a range pattern must contain at least one integer'],
 ['a reversed inclusive range is invalid','pub fn result()->bool{matches!(3u8,4..=3)}','type','a range pattern must contain at least one integer'],
 ['an open upper bound cannot exclude the entire domain','pub fn result()->bool{matches!(0u8,..0)}','type','a range pattern must contain at least one integer'],
 ['a missing domain cell remains non-exhaustive','pub fn result()->bool{match 7u8{0..4=>true,5..=>true}}','type','non-exhaustive match'],
 ['an inclusive upper endpoint cannot be omitted','pub fn result()->bool{matches!(7u8,1..=)}','unsupported','range pattern: an inclusive range pattern needs its upper endpoint'],
])refusals.push({name,source:source+'fn main(){}',kind,reason});
writeProjectData(new URL('../../parity/fixtures/rust-integer-ranges.lino',import.meta.url),{schemaVersion:1,model:'exact Rust integer range pattern domain partitions 1',retainSourceTypeOwners:true,cases,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/rust-integer-range-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/reference/patterns.html#range-patterns'],boundary:'Unexecuted independent source programs, exact integer premises, transport/emission/native controls. The complete integer domain is partitioned at actual checked endpoints without enumerating values or losing u128 precision. Float/char/associated-constant range endpoints, complete ownership/traits and all full-language, grammar and shared-concept requirements remain active.'});
