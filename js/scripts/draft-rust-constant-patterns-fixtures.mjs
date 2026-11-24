// Independent complete originals and operation facts; never execute their code.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],premises=[],refusals=[];
function add(name,body,helpers='',abort=null){cases.push({name,module:'constant_pattern_'+cases.length,source:helpers+'pub fn result()->bool{'+body+'}',value:abort===null?'true':null,abort,resultType:'bool',checks:['const _: fn()->bool=CASE::result;']});}
for(const owner of ['u8','i8','u16','i16','u32','i32','u64','i64','u128','i128','usize','isize']){
 add(owner+' actual minimum and maximum remain literal patterns','matches!('+owner+'::MIN,'+owner+'::MIN)&&matches!('+owner+'::MAX,'+owner+'::MAX)');
 add(owner+' primitive paths provide complete range bounds','all('+owner+'::MIN)&&all('+owner+'::MAX)','pub fn all(value:'+owner+')->bool{match value{'+owner+'::MIN..='+owner+'::MAX=>true}}');
 add(owner+' aliased primitive paths preserve the original source owner','matches!(Number::MAX,Number::MIN..=Number::MAX)','use core::primitive::'+owner+' as Number;');
 add(owner+' transparent type aliases preserve bound owners','matches!(Number::MAX,Number::MIN..=Number::MAX)','type Number='+owner+';');
 add(owner+' absolute standard paths retain actual endpoint ownership','matches!(::core::primitive::'+owner+'::MAX,::core::primitive::'+owner+'::MIN..=::core::primitive::'+owner+'::MAX)');
 add(owner+' width constants have their actual u32 result type','matches!('+owner+'::BITS,'+owner+'::BITS)');
 add(owner+' bound patterns compose with actual matched-value aliases','match '+owner+'::MAX{selected@'+owner+'::MAX=>selected=='+owner+'::MAX,_=>false}');
 add(owner+' open primitive bounds keep complete domain coverage','all('+owner+'::MIN)&&all('+owner+'::MAX)','pub fn all(value:'+owner+')->bool{match value{..='+owner+'::MIN=>true,_=>true}}');
}
add('bounds inside tuple fields keep existing product syntax','match (7u8,3u16){(u8::MIN..=u8::MAX,last)=>last==3u16}');
add('associated constants compose with nested nominal payloads','match Event::Data(7u8){Event::Data(u8::MIN..=u8::MAX)=>true}','pub enum Event{Data(u8)}');
add('associated ranges retain the actual reference layer','let value=7u8;match &value{&(u8::MIN..=u8::MAX)=>true}');
add('alias chains resolve the same actual primitive owner','matches!(Last::MAX,First::MIN..=Last::MAX)','type First=u8;type Last=First;');
add('module-qualified aliases retain actual primitive ownership','matches!(model::Number::MAX,model::Number::MIN..=model::Number::MAX)','pub mod model{pub type Number=u8;}');
add('ordinary constant values retain exact source arithmetic','VALUE==44u8','pub const VALUE:u8=300u16 as u8;');
add('ordinary signed constant division truncates toward zero','VALUE==-2i8','pub const VALUE:i8=-7i8/3i8;');
add('ordinary unsigned source shifts truncate shifted high bits','VALUE==254u8','pub const VALUE:u8=255u8<<1u32;');
add('a receiver failure remains before any constant range selection','match receiver(){u8::MIN..=u8::MAX=>true}','pub fn receiver()->u8{panic!("constant range receiver")}','constant range receiver');
function premise(operation,args,expected){premises.push({operation,args,expected});}
const signed=['-128','127',8],unsigned=['0','255',8];
for(const [op,a,b,domain,expected] of [
 ['add','7','3',signed,['integer','10']],['sub','7','3',signed,['integer','4']],['mul','7','3',signed,['integer','21']],
 ['div','-7','3',signed,['integer','-2']],['rem','-7','3',signed,['integer','-1']],
 ['div','1','0',signed,['error','division by zero in a constant integer expression']],
 ['rem','-128','-1',signed,['error','overflow in a constant integer expression']],
 ['neg','-128','0',signed,['error','overflow in a constant integer expression']],
 ['neg','0','0',unsigned,['error','an unsigned constant integer cannot use unary negation']],
 ['neg','0','0',signed,['integer','0']],['bitnot','0','0',unsigned,['integer','255']],['bitnot','0','0',signed,['integer','-1']],
 ['bitand','7','3',unsigned,['integer','3']],['bitor','4','3',unsigned,['integer','7']],['bitxor','7','3',unsigned,['integer','4']],
 ['shl','255','1',unsigned,['integer','254']],['shl','127','1',signed,['integer','-2']],['shr','-1','1',signed,['integer','-1']],['shr','128','7',unsigned,['integer','1']],
 ['shl','1','8',unsigned,['error','overflow in a constant integer shift']],['shr','1','-1',unsigned,['error','overflow in a constant integer shift']],
 ['cast','300','0',unsigned,['integer','44']],['cast','-1','0',unsigned,['integer','255']],['cast','255','0',signed,['integer','-1']],['cast','-129','0',signed,['integer','127']],
 ['add','255','1',unsigned,['error','overflow in a constant integer expression']],
 ['add','340282366920938463463374607431768211455','1',['0','340282366920938463463374607431768211455',128],['error','overflow in a constant integer expression']],
 ['div','340282366920938463463374607431768211455','3',['0','340282366920938463463374607431768211455',128],['integer','113427455640312821154458202477256070485']],
 ['eq','7','7',unsigned,['boolean','true']],['ne','7','7',unsigned,['boolean','false']],['lt','-7','3',signed,['boolean','true']],['le','7','7',unsigned,['boolean','true']],['gt','8','7',unsigned,['boolean','true']],['ge','7','8',unsigned,['boolean','false']],
 ['add','0','0',['0','100',8],['error','constant integer evaluation needs an actual integer domain']],
 ['add','00','1',unsigned,['error','constant integer evaluation needs exact operands and an actual integer domain']],
 ['add','256','1',unsigned,['error','a constant integer operand is outside its actual source domain']],
 ['add','1','256',unsigned,['error','a constant integer operand is outside its actual source domain']],
 ['unknown','1','1',unsigned,['error','the constant integer operation needs its source interpretation']],
])premise('evaluateRustConstantIntegerOperation',[op,a,b,...domain],expected);
for(const [op,a,b,expected]of [['not',true,false,['boolean','false']],['and',true,false,['boolean','false']],['or',false,true,['boolean','true']],['eq',true,true,['boolean','true']],['ne',true,false,['boolean','true']],['unknown',true,false,['error','the constant Boolean operation needs its source interpretation']]])premise('evaluateRustConstantBooleanOperation',[op,a,b],expected);
for(const [op,a,b,expected] of [['bitnot',false,false,'true'],['bitand',true,false,'false'],['bitor',false,true,'true'],['bitxor',true,true,'false']])premise('evaluateRustConstantBooleanOperation',[op,a,b],['boolean',expected]);
for(const [op,left,expected]of [['and',false,false],['and',true,true],['or',false,true],['or',true,false],['eq',false,true]])premise('requireRustConstantBooleanRight',[op,left],expected);
for(const [args,expected]of [[[256,65536,false],''],[[257,0,false],'constant pattern evaluation exceeds its source resource budget'],[[0,65537,false],'constant pattern evaluation exceeds its source resource budget'],[[0,0,true],'a constant pattern dependency cannot evaluate itself recursively']])premise('readRustConstantPatternEvaluationError',args,expected);
for(const [args,expected]of [
 [['fixed','primitive','u8',0,0,'8',false],'u8'],[['fixed','primitive','usize',0,0,'64',false],'usize'],
 [['fixed','primitive','u8',0,1,'8',false],''],[['fixed','declaration','u8',0,0,'8',false],''],
 [['fixed','primitive','u8',1,0,'8',false],''],[['fixed','primitive','u8',0,0,'16',false],''],
 [['float','primitive','f64',0,0,'',false],''],
])premise('readRustConstantPrimitiveAliasOwner',args,expected);
refusals.push({name:'an associated constant pattern is a value rather than a call',source:'pub fn result()->bool{matches!(255u8,u8::MAX())}fn main(){}',kind:'unsupported',reason:'Rust primitive integer constant: an associated integer constant requires value syntax rather than a call'});
writeProjectData(new URL('../../parity/fixtures/rust-constant-patterns.lino',import.meta.url),{schemaVersion:1,model:'Rust primitive constant patterns and exact constant arithmetic 1',retainSourceTypeOwners:true,cases,premises,refusals,sourceDefinitionPrograms:['meta-language/source-program/rust-constant-integer-program'],source:cases.map(row=>'pub mod '+row.module+'{'+row.source+'}').join('\n')+'\nfn main(){}\n',versions:{Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0'},references:['https://doc.rust-lang.org/reference/patterns.html#path-patterns','https://doc.rust-lang.org/reference/const_eval.html'],boundary:'Unexecuted complete originals and shared arithmetic premises. Both parsers admit actual primitive integer constant values and range endpoints via existing path/type ownership rules. Exact arithmetic definitions prepare general source-constant evaluation; arbitrary source constant initializers, constant functions, aggregate structural equality, full trait/ownership and all whole-language/grammar/concept obligations remain active.'});
