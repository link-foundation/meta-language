// All work here authors source/assertion data. It never invokes a source reader,
// shared primitive rule, represented program, test, product or native compiler.
import {execFileSync} from 'node:child_process';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {draftCallableTypeAssertion} from './draft-callable-type-assertions.mjs';
import {draftPrimitiveFunctionValueCorrections} from './draft-primitive-function-value-corrections.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),cases=[],revision='59341c5f';
const primitives=['bool','i8','u8','i16','u16','i32','u32','i64','u64','i128','u128','isize','usize','f64'];
function add(name,source,expected,resultType,checks,body='select()'){
 const parsed=checks.map(draftCallableTypeAssertion);
 cases.push({name,module:'primitive_function_value_case_'+cases.length,source,expected:String(expected),value:String(expected),abort:null,resultType,checks,canonicalChecks:parsed.map(item=>item.canonical),sourceContracts:parsed.map(item=>item.sourceContract),body});
}
const retained=[];
for(const fixture of ['rust-primitive-associated-call','rust-primitive-default']){
 const path='parity/fixtures/'+fixture+'.lino',bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:16*1024*1024}),historyPath='parity/fixtures/history/'+fixture+'-'+revision+'.lino';
 writeFileSync(new URL(historyPath,root),bytes);retained.push({fixture,path:historyPath,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')});
 const prior=readProjectData(new URL(historyPath,root)),data=readProjectData(new URL(path,root));
 if(fixture==='rust-primitive-associated-call')for(const row of prior.cases){
  // Retain independent preceding arithmetic oracles; only parenthesize actual
  // standard associated paths. Reference-interface controls stay in their
  // preceding profile, whose original input and assertions are never removed.
  if(row.checks.some(check=>check.includes('&')||check.includes("'")))continue;
  let changed=0;const source=row.source.replace(/((?:::)?(?:(?:std|core)::primitive::)?(?:bool|i8|u8|i16|u16|i32|u32|i64|u64|i128|u128|isize|usize|Word|Wide)::[a-z_]+)\(/gu,(_,path)=>{changed+=1;return '('+path+')(';});
  if(changed)add('first-class '+row.name,source,row.expected,row.resultType,row.checks);
 }
 const old=prior.refusals.find(row=>row.name===(fixture==='rust-primitive-associated-call'?'associated function values retain a separate callable obligation':'a primitive Default function value retains its callable obligation'));
 if(!old)throw new TypeError('the whole original callable refusal must remain available');
 add(old.name,old.source+'\npub fn result()->u64{select()}',fixture==='rust-primitive-associated-call'?'3':'0','u64',['const _: fn()->u64=CASE::select;']);
 cases.at(-1).retainedRefusal={fixture,source:old.source,kind:old.kind,reason:old.reason};
 // Preserve all preceding non-corrected fields, cases, scopes and assertions.
 writeProjectData(new URL(path,root),draftPrimitiveFunctionValueCorrections(data,fixture));
}
for(const owner of primitives)for(const style of ['inferred','pointer','callback','returned','alias']){
 const result=owner==='bool'?'bool':owner==='f64'?'bool':owner,expression=owner==='f64'?'operation().is_finite()':'operation()',expected=owner==='bool'?'false':owner==='f64'?'true':'0';
 let prefix='',body='',checks=['const _: fn()->'+result+'=CASE::select;'];
 if(style==='inferred')body='let operation='+owner+'::default;'+expression;
 if(style==='pointer')body='let operation:fn()->'+owner+'='+owner+'::default;'+expression;
 if(style==='callback'){prefix='pub fn apply(operation:fn()->'+owner+')->'+result+'{'+expression+'}';body='apply('+owner+'::default)';checks.push('const _: fn(fn()->'+owner+')->'+result+'=CASE::apply;');}
 if(style==='returned'){prefix='pub fn factory()->fn()->'+owner+'{return '+owner+'::default;}';body='let operation=factory();'+expression;checks.push('const _: fn()->fn()->'+owner+'=CASE::factory;');}
 if(style==='alias'){prefix='use std::primitive::'+owner+' as Primitive;';body='let operation=Primitive::default;'+expression;}
 add(owner+' default function value '+style,prefix+'pub fn select()->'+result+'{'+body+'}pub fn result()->'+result+'{select()}fn main(){}',expected,result,checks);
}
// Independent signatures and mathematical boundary expectations for operation
// families added after the retained associated-call corpus.
const methods=['count_ones','count_zeros','leading_ones','leading_zeros','trailing_ones','trailing_zeros','reverse_bits','wrapping_shl','wrapping_shr','rotate_left','rotate_right','unsigned_abs','abs_diff'];
for(const signed of [false,true])for(const [bits,pointer]of [[8,false],[16,false],[32,false],[64,false],[128,false],[64,true]]){
 const owner=pointer?(signed?'isize':'usize'):(signed?'i':'u')+bits,unsigned=pointer?'usize':'u'+bits,modulus=1n<<BigInt(bits),minimum=signed?-(modulus/2n):0n,maximum=signed?modulus/2n-1n:modulus-1n,wrap=n=>{const value=(n%modulus+modulus)%modulus;return signed&&value>=modulus/2n?value-modulus:value;};
 for(const method of methods){if(method==='unsigned_abs'&&!signed)continue;
  for(const value of [minimum,maximum]){
   const binary=['wrapping_shl','wrapping_shr','rotate_left','rotate_right','abs_diff'].includes(method),shift=binary&&method!=='abs_diff',other=method==='abs_diff'?maximum:BigInt(bits+1),pattern=(value%modulus+modulus)%modulus,bitString=pattern.toString(2).padStart(bits,'0'),ones=[...bitString].filter(bit=>bit==='1').length;
   const run=(text,bit)=>{let n=0;for(const actual of text){if(actual!==bit)break;n+=1;}return BigInt(n);};let expected;
   if(method==='count_ones')expected=BigInt(ones);else if(method==='count_zeros')expected=BigInt(bits-ones);
   else if(method.startsWith('leading_'))expected=run(bitString,method.endsWith('ones')?'1':'0');else if(method.startsWith('trailing_'))expected=run([...bitString].reverse(),method.endsWith('ones')?'1':'0');
   else if(method==='reverse_bits')expected=wrap(BigInt('0b'+[...bitString].reverse().join('')));
   else if(method==='unsigned_abs')expected=value<0n?-value:value;else if(method==='abs_diff')expected=value>other?value-other:other-value;
   else{const count=other%BigInt(bits),inverse=(BigInt(bits)-count)%BigInt(bits);expected=method==='wrapping_shl'?wrap(value<<count):method==='wrapping_shr'?wrap(value>>count):method==='rotate_left'?wrap((pattern<<count)|(pattern>>inverse)):wrap((pattern>>count)|(pattern<<inverse));}
   const countResult=method.includes('ones')||method.includes('zeros'),result=countResult?'u32':method==='unsigned_abs'||method==='abs_diff'?unsigned:owner,argumentOwner=shift?'u32':owner,signature='fn('+owner+(binary?','+argumentOwner:'')+')->'+result,args=value+owner+(binary?','+other+argumentOwner:'');
   const source='pub fn apply(operation:'+signature+',value:'+owner+(binary?',other:'+argumentOwner:'')+')->'+result+'{operation(value'+(binary?',other':'')+')}pub fn select()->'+result+'{let operation:'+signature+'='+owner+'::'+method+';apply(operation,'+args+')}pub fn result()->'+result+'{select()}fn main(){}';
   add(owner+' '+method+' first-class independent boundary '+value,source,String(expected),result,['const _: fn('+signature+','+owner+(binary?','+argumentOwner:'')+')->'+result+'=CASE::apply;','const _: fn()->'+result+'=CASE::select;']);
  }
 }
}
const printedSources=[{name:'first-class associated operands evaluate once in original order',source:'pub fn first()->usize{println!("first");18446744073709551615usize}pub fn second()->usize{println!("second");1usize}fn main(){let operation:fn(usize,usize)->usize=usize::wrapping_add;println!("{}",operation(first(),second()));}',lines:['first','second','0']},{name:'a standard function factory keeps its actual call effects',source:'pub fn factory()->fn(usize)->u32{println!("factory");return usize::count_ones;}pub fn input()->usize{println!("input");7usize}fn main(){let operation=factory();println!("{}",operation(input()));}',lines:['factory','input','3']}];
const signatureControls=[];
for(const owner of [...primitives,'char','f32','unknown'])for(const method of ['min','max','signum','is_negative','is_positive','saturating_add','saturating_sub','saturating_mul','saturating_neg','saturating_abs','wrapping_add','wrapping_sub','wrapping_neg','wrapping_abs',...methods,'default','checked_add','from','unknown']){
 const integer=!['bool','f64','char','f32','unknown'].includes(owner),signed=integer&&owner.startsWith('i'),base=['min','max'].includes(method)&&(integer||owner==='bool')?2:integer&&['saturating_add','saturating_sub','saturating_mul','wrapping_add','wrapping_sub','wrapping_shl','wrapping_shr','rotate_left','rotate_right','abs_diff'].includes(method)?2:integer&&(['wrapping_neg','count_ones','count_zeros','leading_ones','leading_zeros','trailing_ones','trailing_zeros','reverse_bits'].includes(method)||signed&&['signum','is_negative','is_positive','saturating_neg','saturating_abs','wrapping_abs','unsigned_abs'].includes(method))?1:method==='default'&&primitives.includes(owner)?0:-1,result=base<0?'':['is_negative','is_positive'].includes(method)?'bool':method.includes('ones')||method.includes('zeros')?'u32':['unsigned_abs','abs_diff'].includes(method)?owner==='isize'?'usize':owner.replace(/^i/u,'u'):owner;
 signatureControls.push({owner,method,arity:base,result,parameters:Array.from({length:Math.max(0,base)},(_,index)=>index===1&&['wrapping_shl','wrapping_shr','rotate_left','rotate_right'].includes(method)?'u32':owner)});
}
const mismatch='distinct pointer-sized and fixed-width integer source types need an explicit cast';
const refusals=[{name:'standard function parameter source identities remain exact',source:'pub fn select()->u64{let operation:fn(u64,u64)->u64=usize::wrapping_add;operation(1u64,2u64)}fn main(){}',kind:'unsupported',reason:'Rust callable source owner: '+mismatch,originalRustErrorCodes:['E0308']},{name:'bit-query function results retain their real u32 domain',source:'pub fn select()->u64{let operation:fn(u64)->u64=u64::count_ones;operation(7u64)}fn main(){}',kind:'type'},{name:'shift function counts require their real u32 input',source:'pub fn select()->u64{let operation:fn(u64,u64)->u64=u64::rotate_left;operation(7u64,1u64)}fn main(){}',kind:'type'}];
const malformed=[{name:'a first-class standard function retains complete invocation syntax',source:'pub fn select()->u64{let operation=u64::wrapping_add;operation(,)}fn main(){}'}];
const history=['js/src/translation/rust.js','js/src/translation/rust-primitive-associated-calls.js','rust/src/translation/mod.rs','rust/src/translation/rust/expressions.rs','rust/src/translation/rust/expressions/primitive_associated_calls.rs','rust/src/translation/rust/expressions/primitive_default.rs','experiments/rust-records/callable-owner.mjs','rust/tests/unit/translation_rust_callable_owner.rs','js/tests/translation-rust-callable-owner.test.js'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return{path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('parity/fixtures/rust-primitive-function-value.lino',root),{schemaVersion:1,retainSourceTypeOwners:true,model:null,cases,printedSources,signatureControls,refusals,malformed,retained,history,source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64'},references:['https://doc.rust-lang.org/reference/types/function-item.html','https://doc.rust-lang.org/reference/types/function-pointer.html'],assertions:['standardPrimitiveFunctionValuesReuseCompleteExistingMethodSemantics','everyRegisteredScalarSignaturePreservesOriginalParameterAndResultOwners','allSupportedWidthsSignsPointerDomainsAndBooleanOrdersRetainIndependentBoundaries','actualFunctionPointersCallbacksReturnedFunctionsAndAliasesPreserveTheirMeaning','eachActualCallArgumentAndFactoryEffectOccursOnceInOriginalOrder','bothCorrectedRefusalInputsRetainTheirExactSourcesAndPositiveNativeProducts','allUncorrectedPredecessorCasesAssertionsRefusalsAndScopesRemainIntact','bothRuntimesBothCodecsFourTargetsAndOriginalGeneratedNativeProductsPreserveCompleteContracts'],boundary:'Unexecuted source/data draft. Shared scalar signatures project complete existing method bodies; no duplicate arithmetic semantics. Functions returning Option/Result, generic From function values, unsafe/extern ABI, physical function identity, exact native source callable ABI, generated wrapper reimport and full project translation remain obligations. No project rule, parser fixture, represented source/product, code test, validation gate or native compiler executed.'});
process.stdout.write('Authored '+cases.length+' complete standard primitive function-value source programs.\n');
