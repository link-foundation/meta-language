// Future original/native source products and complete predecessor contexts.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[],increment='fn increment(value:i64)->i64{value+1}',apply='fn apply(read:fn(i64)->i64,value:i64)->i64{read(value)}';
function add(name,body,value,prefix=increment){cases.push({name,module:'function_pointer_case_'+cases.length,source:prefix+'pub fn result()->i64{'+body+'}',value,abort:null});}
add('a free function value is distinct from calling the function','let read=increment;read(6)','7');
add('an actual written pointer type retains its signature','let read:fn(i64)->i64=increment;read(6)','7');
add('a zero-argument pointer remains an actual call','let read:fn()->i64=seven;read()','7','fn seven()->i64{7}');
add('a zero-argument free function value does not execute on binding','let read=seven;read()','7','fn seven()->i64{7}');
add('a typed higher-order function calls a free function pointer','apply(increment,6)','7',increment+apply);
add('a noncapturing typed closure coerces at a written pointer binding','let read:fn(i64)->i64=|value:i64|->i64{value+1};read(6)','7','');
add('a noncapturing closure coerces at a real callback argument','apply(|value:i64|->i64{value+1},6)','7',apply);
add('a noncapturing closure alias retains its coercion witness','let read=|value:i64|->i64{value+1};apply(read,6)','7',apply);
add('a move closure without captures still coerces to a pointer','let read:fn(i64)->i64=move |value:i64|->i64{value+1};read(6)','7','');
add('a function can return its actual free function pointer','let read=factory();read(6)','7',increment+'fn factory()->fn(i64)->i64{increment}');
add('a function can return a noncapturing closure as its declared pointer','let read=factory();read(6)','7','fn factory()->fn(i64)->i64{|value:i64|->i64{value+1}}');
add('a returned pointer can be called immediately','factory()(6)','7',increment+'fn factory()->fn(i64)->i64{increment}');
add('different function pointers may join with the same actual pointer type','let first:fn(i64)->i64=increment;let last:fn(i64)->i64=double;let read=if true{first}else{last};read(6)','7',increment+'fn double(value:i64)->i64{value*2}');
add('the selected pointer alternative remains observable by its call','let first:fn(i64)->i64=increment;let last:fn(i64)->i64=double;let read=if false{first}else{last};read(3)+1','7',increment+'fn double(value:i64)->i64{value*2}');
add('an immutable pointer is a proven Copy closure capture','let read:fn(i64)->i64=increment;let use_value=|value:i64|->i64{read(value)};use_value(6)','7');
add('a typed closure can accept an actual pointer parameter','let use_value=|read:fn(i64)->i64,value:i64|->i64{read(value)};use_value(increment,6)','7');
add('a pointer to a higher-order function validates its pointer arguments','let use_value:fn(fn(i64)->i64,i64)->i64=apply;use_value(increment,6)','7',increment+apply);
add('a written pointer without a result returns actual Unit','let step:fn()=nothing;step();7','7','fn nothing(){}');
add('bare pointer parameter labels retain their actual types','let read:fn(value:i64)->i64=increment;read(6)','7');
add('an empty noncapturing closure can be a zero-argument pointer','let read:fn()->i64=||->i64{7};read()','7','');
add('a qualified free function keeps its original declaration owner','let read=source::increment;read(6)','7','mod source{pub fn increment(value:i64)->i64{value+1}}');
add('a forward free function reference resolves through the complete catalog','factory()(6)','7','fn factory()->fn(i64)->i64{increment}'+increment);
add('cloning a Copy pointer retains its callable value','let read:fn(i64)->i64=increment;let alias=read.clone();alias(6)','7');
add('an unnamed labeled pointer argument and trailing comma retain their slot','let read:fn(_:i64,)->i64=increment;read(6)','7');
add('a local pointer binding shadows an imported function value','let read:fn(i64)->i64=double;let saved=read;saved(3)+1','7',increment+'fn double(value:i64)->i64{value*2}use self::increment as read;');
const coercion='Rust function pointer coercion: capturing closures cannot coerce to function pointers',observation='Rust callable observation: callable identity, address and numeric casts require their actual source representation';
const refusals=[
 {name:'a captured closure cannot coerce at a typed pointer binding',source:'fn f()->i64{let offset=3i64;let read:fn(i64)->i64=|value:i64|->i64{value+offset};read(4)}fn main(){}',kind:'unsupported',reason:coercion},
 {name:'a captured closure cannot coerce at a named pointer parameter',source:apply+'fn f()->i64{let offset=3i64;apply(|value:i64|->i64{value+offset},4)}fn main(){}',kind:'unsupported',reason:coercion},
 {name:'computed pointer invocations retain their noncapturing argument proof',source:apply+'fn f()->i64{let offset=3i64;let use_value:fn(fn(i64)->i64,i64)->i64=apply;use_value(|value:i64|->i64{value+offset},4)}fn main(){}',kind:'unsupported',reason:coercion},
 {name:'owned pointer argument domains retain their ownership obligation',source:'fn f(read:fn(String)->i64)->i64{7}fn main(){}',kind:'unsupported',reason:'Rust function pointer: function pointer parameters require direct immutable Copy source types'},
 {name:'owned pointer results retain their ownership obligation',source:'fn f(read:fn()->String)->i64{7}fn main(){}',kind:'unsupported',reason:'Rust function pointer: function pointer results require direct immutable Copy source types'},
 {name:'pointer addresses do not become invented numeric values',source:increment+'fn f()->usize{let read:fn(i64)->i64=increment;read as usize}fn main(){}',kind:'unsupported',reason:observation},
 {name:'pointer equality retains its actual representation obligation',source:increment+'fn f()->bool{let read:fn(i64)->i64=increment;read==read}fn main(){}',kind:'unsupported',reason:observation},
 {name:'a pointer cannot acquire an invented Display implementation',source:increment+'fn f()->String{let read:fn(i64)->i64=increment;format!("{}",read)}fn main(){}',kind:'unsupported',reason:observation},
 {name:'named function-item Display does not silently invoke the function',source:'fn seven()->i64{7}fn f()->String{format!("{seven}")}fn main(){}',kind:'unsupported',reason:observation}
];
const revision='07d17457',history=['js/src/translation/rust.js','js/src/translation/rust-source-origins.js','js/src/translation/rust-copy-closures.js','rust/src/translation/rust/expressions/paths.rs','rust/src/translation/rust/items/types.rs','rust/src/translation/rust/copy_closures.rs','rust/src/translation/rust/source_origins.rs','rust/src/translation/rust/formatting.rs','js/tests/translation-rust-records.test.js'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/rust-function-pointers.lino',import.meta.url),{schemaVersion:1,model:'ordinary safe Rust function pointers over immutable Copy source domains 1',cases,refusals,history,source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},references:['https://doc.rust-lang.org/reference/types/function-pointer.html'],assertions:['actualBareFunctionPointerSignaturesAndUnitDefaults','freeFunctionValuesDifferFromZeroArgumentCalls','actualForwardQualifiedAndLocalDeclarationOwners','onlyProvenNoncapturingClosuresCoerceToPointers','namedAndComputedHigherOrderArgumentProofs','pointerResultsAliasesAndCopyCaptures','noInventedIdentityAddressOrDisplayBehavior','completePredecessorsBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted future controls. Source signatures use safe Rust ABI and direct immutable Copy parameter/result domains; current callable encodings preserve invocation rather than physical pointer representation. Identity/address observation, unsafe/extern/variadic conventions, lifetime/owned domains, contextual closure inference, generated Rc wrappers and full callable source environments remain obligations.'});
