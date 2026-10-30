// Independent BigInt arithmetic authors literal expectations and source data.
// This author never imports the checked rules or invokes a product/parser.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],sourceControls=[];
const methods=['checked_add','checked_sub','checked_mul','checked_div','checked_rem','checked_neg','checked_abs'];
const consumers=['is_some','is_none','unwrap_or','map_or'];
function add(name,source,body,expected,type,checks){cases.push({name,module:'checked_integer_case_'+cases.length,source:source+'pub fn result()->'+type+'{'+body+'}fn main(){}',expected:String(expected),value:String(expected),abort:null,resultType:type,checks});}
for(const signed of [false,true])for(const [bits,pointer]of [[8,false],[16,false],[32,false],[64,false],[128,false],[64,true]]){
 const owner=pointer?(signed?'isize':'usize'):(signed?'i':'u')+bits,limit=1n<<BigInt(signed?bits-1:bits),minimum=signed?-limit:0n,maximum=limit-1n,half=maximum/2n;
 const atoms=[minimum,minimum+1n,maximum,maximum-1n,0n,1n,2n,...signed?[-1n,-2n]:[]];
 const pairs=[[minimum,0n],[minimum,1n],[maximum,1n],[maximum,2n],[maximum,maximum],[minimum,minimum],[0n,0n],[0n,maximum],[1n,maximum],[half,2n],[half+1n,2n],[1n,2n],[2n,1n],...signed?[[minimum,-1n],[maximum,-1n],[-1n,minimum],[-2n,-2n]]:[]];
 for(const method of methods){
  const arity=['checked_neg','checked_abs'].includes(method)?0:1;
  sourceControls.push({method,kind:'fixed',bits:String(bits),signed,owner:'primitive',path:owner,children:0,proven:true,layers:0,expected:method!=='checked_abs'||signed});
  if(method==='checked_abs'&&!signed)continue;
  const inputs=arity?pairs:atoms.map(value=>[value,0n]);
  for(const [role,[left,right]]of inputs.entries()){
   let value=null;
   if((method!=='checked_div'&&method!=='checked_rem')||right!==0n&&!(signed&&left===minimum&&right===-1n)){
    value=method==='checked_add'?left+right:method==='checked_sub'?left-right:method==='checked_mul'?left*right:method==='checked_div'?left/right:method==='checked_rem'?left%right:method==='checked_neg'?-left:left<0n?-left:left;
    if(value<minimum||value>maximum)value=null;
   }
   const definitions='pub fn select(left:'+owner+(arity?',right:'+owner:'')+')->',args=left+owner+(arity?','+right+owner:'');
   for(const consumer of consumers){
    if(consumer==='map_or'&&bits>32)continue;
    const resultType=consumer==='is_some'||consumer==='is_none'?'bool':consumer==='map_or'?'f64':owner;
    const suffix=consumer==='unwrap_or'?'7':consumer==='map_or'?'-1.5f64,f64::from':'';
    const expected=consumer==='is_some'?value!==null:consumer==='is_none'?value===null:value===null?consumer==='map_or'?'-1.5':'7':String(value);
    const expression='left.'+method+'('+(arity?'right':'')+').'+consumer+'('+suffix+')';
    add(owner+' '+method+' '+consumer+' boundary role '+role,definitions+resultType+'{'+expression+'}','select('+args+')',expected,resultType,['const _: fn('+owner+(arity?','+owner:'')+')->'+resultType+'=CASE::select;']);
   }
  }
  const second=arity?',right:'+owner:'',arg=arity?',2'+owner:'',sourceArgs=arity?',right':'',call=method==='checked_neg'?minimum:maximum;
  for(const qualified of [owner,'std::primitive::'+owner,'::core::primitive::'+owner])add(owner+' '+method+' explicit associated owner '+qualified,'pub fn select(left:'+owner+second+')->bool{'+qualified+'::'+method+'(left'+sourceArgs+').is_some()}','select('+call+owner+arg+')',method==='checked_abs'||method==='checked_rem'||method==='checked_div','bool',['const _: fn('+owner+(arity?','+owner:'')+')->bool=CASE::select;']);
  for(const depth of [1,2,4])add(owner+' '+method+' immutable receiver depth '+depth,'pub fn select(left:'+'&'.repeat(depth)+owner+second+')->bool{left.'+method+'('+(arity?'right':'')+').is_some()}','select('+'&'.repeat(depth)+call+owner+arg+')',method==='checked_abs'||method==='checked_rem'||method==='checked_div','bool',['const _: fn('+'&'.repeat(depth)+owner+(arity?','+owner:'')+')->bool=CASE::select;']);
 }
}
// Associated division/rem with MAX and two succeed. Unsigned checked_neg(MAX)
// and signed checked_neg(MIN) fail; MAX add/sub/mul use the actual two operand.
// Subtraction succeeds for MAX-2, while the remaining selected arithmetic does not.
for(const row of cases)if(row.name.includes(' explicit associated owner ')||row.name.includes(' immutable receiver depth ')){
 const method=methods.find(method=>row.name.includes(' '+method+' '));
 if(method==='checked_sub')row.expected=row.value='true';
}
add('actual checked operands and default are captured once in source order','pub fn first()->i64{7i64}pub fn second()->i64{2i64}pub fn fallback()->i64{9i64}pub fn select()->i64{first().checked_add(second()).unwrap_or(fallback())}','select()','9','i64',['const _: fn()->i64=CASE::first;','const _: fn()->i64=CASE::second;','const _: fn()->i64=CASE::fallback;','const _: fn()->i64=CASE::select;']);
add('a successful checked result retains its integer owner for following operations','pub fn select(value:u8)->u8{value.checked_add(1).unwrap_or(0).wrapping_add(1u8)}','select(255u8)','1','u8',['const _: fn(u8)->u8=CASE::select;']);
add('actual local primitive aliases retain checked associated calls and mappers','use core::primitive::u8 as Byte;use std::primitive::u64 as Wide;pub fn select(value:u8)->u64{Byte::checked_add(value,1).map_or(9u64,Wide::from)}','select(254u8)','255','u64',['const _: fn(u8)->u64=CASE::select;']);
const printedSources=[];
for(const [value,expected]of [['7i64','9'],['9223372036854775807i64','5']])printedSources.push({name:'checked arithmetic keeps all eager operands and default on '+value,source:'pub fn first()->i64{println!("first");'+value+'}pub fn second()->i64{println!("second");2i64}pub fn fallback()->i64{println!("default");5i64}fn main(){println!("{}",first().checked_add(second()).unwrap_or(fallback()));}',lines:['first','second','default',expected]});
printedSources.push({name:'successful checked standard mapping follows its eager floating default',source:'pub fn first()->u8{println!("first");7u8}pub fn second()->u8{println!("second");2u8}pub fn fallback()->f64{println!("default");-1.5f64}fn main(){println!("{}",first().checked_add(second()).map_or(fallback(),f64::from)==9.0f64);}',lines:['first','second','default','true']});
const refusals=[
 {name:'an actual checked Option cannot escape its admitted consumer scope',source:'pub fn select(value:i64)->i64{value.checked_add(1)}fn main(){}',kind:'unsupported',reason:'Rust checked integer Option: the actual Option needs an immediate admitted standard consumer'},
 {name:'lazy checked defaults retain their separate callable interpretation',source:'pub fn select(value:i64)->i64{value.checked_add(1).unwrap_or_else(||9i64)}fn main(){}',kind:'unsupported',reason:'Rust checked integer Option: the actual Option needs an immediate admitted standard consumer'},
 {name:'unsigned integers do not acquire a checked_abs method',source:'pub fn select(value:u64)->u64{value.checked_abs().unwrap_or(0)}fn main(){}',kind:'unsupported',reason:'Rust checked integer arithmetic: a proven standard integer receiver with the actual signed operation is required'},
 {name:'checked operands retain their actual integer widths',source:'pub fn select(value:u8,other:u16)->u8{value.checked_add(other).unwrap_or(0)}fn main(){}',kind:'unsupported',reason:'Rust checked integer argument: the actual argument must retain the receiver type and source owner'},
 {name:'checked defaults retain their actual source owner',source:'pub fn select(value:usize,other:u64)->usize{value.checked_add(1).unwrap_or(other)}fn main(){}',kind:'unsupported',reason:'Rust checked integer default: the actual default must retain the selected result type and source owner'},
 {name:'a checked mapper needs its actual standard From implementation',source:'pub fn select(value:u64)->f64{value.checked_add(1).map_or(0.0f64,f64::from)}fn main(){}',kind:'unsupported',reason:'Rust checked integer mapper: the actual standard primitive From implementation must exist for the successful value'},
];
const malformed=[{name:'a checked predicate cannot accept a comma argument',source:'pub fn select(value:u8)->bool{value.checked_add(1).is_some(,)}fn main(){}'},{name:'a checked mapper retains its complete separator',source:'pub fn select(value:u8)->f64{value.checked_add(1).map_or(0.0f64 f64::from)}fn main(){}'}];
const base={method:'checked_add',kind:'fixed',bits:'64',signed:false,owner:'primitive',path:'usize',children:0,proven:true,layers:0};for(const edit of [{method:'unknown'},{kind:'float'},{kind:'bool'},{bits:'32'},{signed:true},{owner:'declaration'},{path:'char'},{children:1},{proven:false},{layers:1}])sourceControls.push({...base,...edit,expected:false});
const arities=[];for(const method of [...methods,'wrapping_add','unknown'])for(const ordinary of [false,true])arities.push({method,ordinary,expected:ordinary&&methods.includes(method)?['checked_neg','checked_abs'].includes(method)?0:1:-1});
const consumerArities=[...consumers,'unwrap','unwrap_or_else','unknown'].map(consumer=>({consumer,expected:consumer==='is_some'||consumer==='is_none'||consumer==='unwrap'?0:consumer==='unwrap_or'?1:consumer==='map_or'?2:-1}));
const plans=[{method:'checked_add',signed:false,condition:['operand','maximum','right','sub','le'],success:['operand','right','add']},{method:'checked_rem',signed:true,condition:['right','zero','eq','false','operand','minimum','eq','right','negative unit','ne','true','if','if'],success:['operand','right','rem']},{method:'checked_neg',signed:false,condition:['operand','zero','eq'],success:['zero']},{method:'checked_abs',signed:false,condition:[],success:[]},{method:'unknown',signed:true,condition:[],success:[]}];
const consumerPlans=[{consumer:'is_some',expected:['condition']},{consumer:'is_none',expected:['condition','not']},{consumer:'unwrap_or',expected:['condition','success','fallback','if']},{consumer:'map_or',expected:['condition','success','fallback','if']},{consumer:'unknown',expected:[]}];
const results=[...consumers,'unknown'].map(consumer=>({consumer,receiver:'u32',mapper:'f64',expected:consumer==='is_some'||consumer==='is_none'?'bool':consumer==='unwrap_or'?'u32':consumer==='map_or'?'f64':''}));
const history=['js/src/translation/rust.js','js/src/translation/rust-primitive-associated-call-rules.js','js/src/translation/rust-primitive-associated-calls.js','js/src/translation/rust-primitive-try-from.js','rust/src/translation/mod.rs','rust/src/translation/rust/expressions.rs','rust/src/translation/rust/expressions/primitive_associated_calls.rs','rust/src/translation/rust/expressions/primitive_try_from.rs'].map(path=>{const revision='2b732c6b',bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return{path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/rust-checked-integer.lino',import.meta.url),{schemaVersion:1,retainSourceTypeOwners:true,model:'retained displayed Rust string function interface 1',cases,printedSources,sourceControls,arities,consumerArities,plans,consumerPlans,results,refusals,malformed,history,assertionRevisions:[{input:{consumer:'unwrap'},formerExpected:-1,expected:0,originalRevision:'a83de44c74419e8960a1db379e6827867300a169',versionRange:'Rust >=1.90.0 <1.100.0',reason:'The previous negative arity described the adapter gap, not the standard Option contract; its unchanged unwrap input is now admitted with zero arguments and independently covered by original programs.',reference:'https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap'}],source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64'},references:['https://doc.rust-lang.org/std/primitive.i64.html','https://doc.rust-lang.org/std/primitive.u64.html','https://doc.rust-lang.org/std/option/enum.Option.html'],assertions:['allSevenCheckedArithmeticMethodsRetainTheirActualIntegerAndSignedDomains','independentBigIntIntervalsRetainOverflowDivisionByZeroAndMinimumNegativeOneFailures','allTwelveIntegerOwnersKeepBoundaryValuesAndPointerIdentity','isSomeIsNoneUnwrapOrAndAdmittedStandardFromMapOrKeepTheirActualMeaning','operandArgumentsAndEagerDefaultsRunOnceInSourceOrderEvenOnSuccess','guardedThresholdsAndSuccessfulArithmeticNeverEvaluateAnOverflowingOrZeroDivisionBranch','immutableCopyReceiversAndExactAssociatedOwnersRetainActualSourceTypes','bothRuntimesBothLinksTransportsAndAllFourEmissionsRetainCompleteMeaning','originalAndBothGeneratedNativeProductsRetainIndependentExactValuesTypesAndTraces','eachAdmissionRefusalMalformedAndVersionPremiseRemainsExplicit'],boundary:'Unexecuted source/data draft for seven standard checked integer operations with immediate Option is_some/is_none/unwrap_or and map_or through existing admitted primitive From definitions. Shared lazy guard terms dominate arithmetic; original eager operand and default effects are captured once. General Option storage/control/error transport, arbitrary and lazy mappers, other checked methods and full own-codebase translation remain obligations. No checked plan, source program, product, test, oracle comparison, runtime evidence or compilation executed.'});
process.stdout.write('Authored '+cases.length+' complete checked integer source declarations.\n');
