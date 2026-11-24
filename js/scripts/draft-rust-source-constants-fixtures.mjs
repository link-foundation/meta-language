// Complete independent originals and fact packets; no represented code executes.
import {writeProjectData} from './project-data.mjs';
import {createSurfaceReferenceFacts} from '../src/translation/surface-reference-facts.js';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],packets=[],premises=[],refusals=[];
function add(name,body,helpers=''){cases.push({name,module:'source_constant_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:'true',abort:null,resultType:'bool',checks:['const _: fn()->bool=CASE::result;']});}
for(const owner of ['u8','i8','u16','i16','u32','i32','u64','i64','u128','i128','usize','isize']){
 add(owner+' source constants retain their actual owner as literal patterns','matches!(7'+owner+',SELECTED)','pub const SELECTED:'+owner+'=7;');
 add(owner+' source constants supply exact range endpoints','matches!(7'+owner+',LOW..=HIGH)&&!matches!(10'+owner+',LOW..HIGH)','pub const LOW:'+owner+'=1;pub const HIGH:'+owner+'=9;');
 add(owner+' forward constant dependencies remain declaration identities','matches!(7'+owner+',FIRST)','pub const FIRST:'+owner+'=SECOND;pub const SECOND:'+owner+'=3'+owner+'+4'+owner+';');
 add(owner+' aliases bind actual values selected by constant patterns','match 7'+owner+'{value@SELECTED=>value==7'+owner+',_=>false}','pub const SELECTED:'+owner+'=7;');
 add(owner+' primitive bounds in initializers cover the complete domain','all('+owner+'::MIN)&&all('+owner+'::MAX)','pub const LOW:'+owner+'='+owner+'::MIN;pub const HIGH:'+owner+'='+owner+'::MAX;pub fn all(value:'+owner+')->bool{match value{LOW..=HIGH=>true}}');
}
add('module imports resolve the actual constant declaration','matches!(7u8,Imported)','use crate::data::VALUE as Imported;pub mod data{pub const VALUE:u8=7;}');
add('qualified source constants remain distinct from equal local names','let VALUE=99u8;matches!(7u8,data::VALUE)&&VALUE==99u8','pub mod data{pub const VALUE:u8=7;}');
add('Boolean constant patterns retain complete Boolean coverage','all(true)&&all(false)','pub const YES:bool=true;pub const NO:bool=false;pub fn all(value:bool)->bool{match value{YES=>true,NO=>true}}');
add('borrowed string constant patterns retain exact text','matches!("content",TEXT)','pub const TEXT:&str="content";');
add('unused overflowing branches do not become false constant failures','matches!(7u8,VALUE)','pub const VALUE:u8=if true{7}else{255u8+1u8};');
add('lazy Boolean and does not evaluate its unused right operand','matches!(false,VALUE)','pub const VALUE:bool=false&&(1u8/0u8==0);');
add('lazy Boolean or does not evaluate its unused right operand','matches!(true,VALUE)','pub const VALUE:bool=true||(1u8/0u8==0);');
add('typed locals preserve default inference before a narrowing cast','matches!(44u8,VALUE)','pub const VALUE:u8={let number=300;number as u8};');
add('explicit local types retain the integer owner','matches!(7u8,VALUE)','pub const VALUE:u8={let number:u16=263;number as u8};');
add('constant casts preserve signed to unsigned wrapping','matches!(255u8,VALUE)','pub const VALUE:u8=-1i32 as u8;');
add('constant unary minimum remains the exact signed lower bound','matches!(i128::MIN,VALUE)','pub const VALUE:i128=-170141183460469231731687303715884105728i128;');
add('Boolean integer casts retain their actual target owner','matches!(1u8,VALUE)','pub const VALUE:u8=true as u8;');
add('constant source hex octal and binary spellings stay exact','matches!(7u8,VALUE)','pub const VALUE:u8=0x3u8+0o3u8+0b1u8;');
add('constant integer bit operations retain fixed-width behavior','matches!(254u8,VALUE)','pub const VALUE:u8=255u8<<1u32;');
add('constant Boolean bit operations evaluate both primitive values','matches!(true,VALUE)','pub const VALUE:bool=true^false;');
function packet(name,constants,owner,expected){const value={constants},facts=createSurfaceReferenceFacts(value);packets.push({name,value,facts,owner,expected});}
const lit=(value,bits=8,signed=false)=>({k:'lit',type:{kind:'fixed',bits,signed},value}),bool=value=>({k:'lit',type:{kind:'bool'},value}),call=owner=>({k:'call',fn:owner,args:[],type:{kind:'fixed',bits:8,signed:false}});
packet('actual typed integer literal',[{owner:'VALUE',domain:'u8',body:lit('7')}],'VALUE',['integer','u8','7']);
packet('actual dependency identities',[{owner:'FIRST',domain:'u8',body:call('SECOND')},{owner:'SECOND',domain:'u8',body:lit('7')}],'FIRST',['integer','u8','7']);
packet('chosen branches alone determine constant values',[{owner:'VALUE',domain:'u8',body:{k:'if',cond:bool(true),then:lit('7'),else:{k:'binary',op:'div',left:lit('1'),right:lit('0'),type:{kind:'fixed',bits:8,signed:false}},type:{kind:'fixed',bits:8,signed:false}}}],'VALUE',['integer','u8','7']);
packet('typed local inference survives target casts',[{owner:'VALUE',domain:'u8',body:{k:'let',name:'local',value:lit('300',32,true),body:{k:'cast',arg:{k:'var',name:'local',type:{kind:'fixed',bits:32,signed:true}},to:{kind:'fixed',bits:8,signed:false},type:{kind:'fixed',bits:8,signed:false}},type:{kind:'fixed',bits:8,signed:false}}}],'VALUE',['integer','u8','44']);
packet('used zero division is an actual constant error',[{owner:'VALUE',domain:'u8',body:{k:'binary',op:'div',left:lit('1'),right:lit('0'),type:{kind:'fixed',bits:8,signed:false}}}],'VALUE',['error','division by zero in a constant integer expression']);
packet('source declaration cycles refuse',[{owner:'VALUE',domain:'u8',body:call('VALUE')}],'VALUE',['error','a constant pattern dependency cannot evaluate itself recursively']);
packet('actual declared owner cannot be replaced',[{owner:'VALUE',domain:'u8',body:lit('7',16)}],'VALUE',['error','a constant initializer must retain its actual declared primitive owner']);
packet('ordinary function calls supply no constant value',[{owner:'VALUE',domain:'u8',body:call('ordinary')}],'VALUE',['error','a constant expression name needs its actual source constant or local binding']);
packet('constant function arguments require their source interpretation',[{owner:'VALUE',domain:'u8',body:{k:'call',fn:'VALUE',args:[lit('1')],type:{kind:'fixed',bits:8,signed:false}}}],'VALUE',['error','constant function calls need their actual const source interpretation']);
packet('checked source shifts keep their optional success carrier',[{owner:'VALUE',domain:'u8',body:{k:'coalesce',left:{k:'math',op:'readCheckedRustLeftShift',args:[lit('255'),lit('1',32,false)],type:{kind:'option',element:{kind:'fixed',bits:8,signed:false},absence:'undefined'}},right:{k:'abort',message:'shift overflow',type:{kind:'fixed',bits:8,signed:false}},type:{kind:'fixed',bits:8,signed:false}}}],'VALUE',['integer','u8','254']);
packet('an absent checked shift selects its actual fallback',[{owner:'VALUE',domain:'u8',body:{k:'coalesce',left:{k:'math',op:'readCheckedRustLeftShift',args:[lit('1'),lit('8',32,false)],type:{kind:'option',element:{kind:'fixed',bits:8,signed:false},absence:'undefined'}},right:lit('7'),type:{kind:'fixed',bits:8,signed:false}}}],'VALUE',['integer','u8','7']);
for(const [args,expected]of [[['0xff',false],['integer','255']],[['0X7F',false],['integer','127']],[['0o17',false],['integer','15']],[['0b111',false],['integer','7']],[['128',true],['integer','-128']],[['-128',true],['integer','128']],[['0x',false],['error','a constant integer literal needs its actual integer spelling']],[['1.5',false],['error','a constant integer literal needs its actual integer spelling']]])premises.push({operation:'readRustSourceConstantIntegerLiteral',args,expected});
premises.push({operation:'readRustSourceConstantInitializerBoundary',args:[['identifier','punct','punct','number','punct','punct','number','punct'],['start','{',';','7','}',';','8',';'],0],expected:5});
writeProjectData(new URL('../../parity/fixtures/rust-source-constant-evaluation.lino',import.meta.url),{schemaVersion:1,model:'bounded source constant scalar evaluation 1',retainSourceTypeOwners:true,cases,packets,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/rust-source-constant-evaluation-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/reference/const_eval.html','https://doc.rust-lang.org/reference/patterns.html#constant-patterns'],boundary:'Unexecuted originals and typed primitive fact packets. Actual source declarations are reparsed in isolated scope, checked using the existing type checker, and evaluated through shared source rules. General const functions, aggregate structural equality, mutable locals/loops, floating/character constants and every full-language/grammar/concept obligation remain active.'});
