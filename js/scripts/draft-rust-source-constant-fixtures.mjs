// Future actual source constant values and full predecessors; execute none.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,prefix,body='VALUE',value='7',ret='i64',suffix=''){cases.push({name,module:'source_constant_case_'+cases.length,source:prefix+'pub fn result()->'+ret+'{'+body+'}'+suffix,value,abort:null});}
add('a scalar source constant remains a value','const VALUE:i64=7;');
add('a scalar constant participates in arithmetic','const VALUE:i64=4;','VALUE+3');
add('a Boolean source constant retains its actual branch','const FLAG:bool=true;','if FLAG{7}else{8}');
add('a binary64 source constant keeps its actual domain','const VALUE:f64=7.0;','VALUE','7','f64');
add('a negative source constant retains its sign','const VALUE:i64=-7;','VALUE','-7');
add('an exact source usize constant retains its width','const VALUE:usize=7;','VALUE','7','usize');
add('a full unsigned source constant never becomes a Number','const VALUE:u64=18446744073709551615;','VALUE','18446744073709551615','u64');
add('a narrower scalar constant retains its actual cast','const VALUE:i32=7;','VALUE as i64');
add('a qualified constant retains its actual module owner','mod values{pub const VALUE:i64=7;}','values::VALUE');
add('an imported constant retains its actual module owner','use self::values::VALUE;mod values{pub const VALUE:i64=7;}');
add('an imported constant alias retains its original value','use self::values::VALUE as chosen;mod values{pub const VALUE:i64=7;}','chosen');
add('a raw keyword constant is never a capture keyword','const r#move:i64=7;','r#move');
add('a source constant alias retains its actual scalar value','const FIRST:i64=7;const VALUE:i64=FIRST;');
add('a forward source constant alias retains its actual value','const VALUE:i64=LATER;','VALUE','7','i64','const LATER:i64=7;');
add('a scalar constant reference retains repeated reads','const VALUE:i64=3;','VALUE+VALUE+1');
add('a source condition may initialize its Boolean constant','const FLAG:bool=7>3;','if FLAG{7}else{8}');
add('a source constant can supply a concrete Copy capture','const VALUE:i64=7;','let chosen=VALUE;let read=||chosen;read()');
add('a literal source constant remains distinct from a zero input function','const VALUE:i64=3;fn known()->i64{4}','let read=known;VALUE+read()');
add('a source tuple constant retains each exact field','const VALUE:(i64,i64)=(3,4);','VALUE.0+VALUE.1');
add('a static scalar string constant keeps its actual UTF8 bytes','const VALUE:&str="😀";','VALUE.len()','4','usize');
const fn='fn known(value:i64)->i64{value+1}';
add('a pointer constant reads the actual function value',fn+'const READ:fn(i64)->i64=known;','let read=READ;read(6)');
add('a pointer constant can be called directly',fn+'const READ:fn(i64)->i64=known;','READ(6)');
add('a parenthesized pointer constant remains an actual value',fn+'const READ:fn(i64)->i64=known;','(READ)(6)');
add('a pointer constant reaches its independent callback parameter',fn+'const READ:fn(i64)->i64=known;fn apply(read:fn(i64)->i64,value:i64)->i64{read(value)}','apply(READ,6)');
add('a forward function item initializes a pointer constant','const READ:fn(i64)->i64=known;','READ(6)','7','i64',fn);
add('an unannotated source closure uses the real constant pointer domain','const READ:fn(i64)->i64=|value|value+1;','READ(6)');
add('a written source closure retains the constant pointer domain','const READ:fn(i64)->i64=|value:i64|->i64{value+1};','READ(6)');
add('a source pointer constant aliases an actual pointer value',fn+'const READ:fn(i64)->i64=known;const ALIAS:fn(i64)->i64=READ;','ALIAS(6)');
add('a callback signature belongs to the pointer constant value','const APPLY:fn(fn(i64)->i64,i64)->i64=apply;fn apply(read:fn(i64)->i64,value:i64)->i64{read(value)}','APPLY(|value|value+1,6)');
add('a constant closure may refer to another global source constant','const OFFSET:i64=1;const READ:fn(i64)->i64=|value|value+OFFSET;','READ(6)');
add('a zero input pointer constant is still actually called','const READ:fn()->i64=||7;','READ()');
add('a Unit pointer constant retains its actual discard call','const STEP:fn()=||{};','STEP();7');
add('a source local value keeps precedence over an imported function name','use self::other::chosen;mod other{pub fn chosen()->i64{99}}','let chosen=7i64;chosen');
add('a pointer constant can initialize an explicitly typed pointer binding',fn+'const READ:fn(i64)->i64=known;','let read:fn(i64)->i64=READ;read(6)');
add('a child module binding does not inherit an unimported parent constant','const VALUE:i64=99;mod child{pub fn read(VALUE:i64)->i64{VALUE}}','child::read(7)');
add('a child module can explicitly read its parent constant','const VALUE:i64=7;mod child{pub fn read()->i64{super::VALUE}}','child::read()');
add('a child module can explicitly import its parent constant','const VALUE:i64=7;mod child{use super::VALUE;pub fn read()->i64{VALUE}}','child::read()');
add('a nested local may shadow another valid local without importing a parent constant','const VALUE:i64=99;mod child{pub fn read(VALUE:i64)->i64{let VALUE=VALUE+1;VALUE}}','child::read(6)');
add('a zero input factory call supplies its result to a typed pointer binding','fn make()->fn(i64)->i64{|value|value+1}','let read:fn(i64)->i64=make();read(6)');
add('a zero input factory call remains a call in a pointer result position','fn make()->fn(i64)->i64{|value|value+1}fn relay()->fn(i64)->i64{make()}','relay()(6)');
add('a returned zero input factory call remains an actual application','fn make()->fn(i64)->i64{|value|value+1}fn relay()->fn(i64)->i64{return make();}','relay()(6)');
const refusals=[
 {name:'a parameter cannot turn a visible source constant into a binding',source:'const VALUE:i64=7;fn f(VALUE:i64)->i64{VALUE}fn main(){}',kind:'unsupported',reason:'Rust constant pattern: source constant patterns require their represented value and type'},
 {name:'a let cannot turn a visible source constant into a binding',source:'const VALUE:i64=7;fn f()->i64{let VALUE=7i64;VALUE}fn main(){}',kind:'unsupported',reason:'Rust constant pattern: source constant patterns require their represented value and type'},
 {name:'a child module cannot implicitly import a parent scalar constant',source:'const VALUE:i64=7;mod child{pub fn f()->i64{VALUE}}fn main(){}',kind:'type',reason:'unknown name crate.child.VALUE'},
 {name:'a scalar constant is not its internal zero argument function',source:'const VALUE:i64=7;fn f()->i64{VALUE()}fn main(){}',kind:'type',reason:'the source constant is not callable'},
 {name:'a local scalar cannot become a shadowed imported callable',source:'use self::other::chosen;mod other{pub fn chosen(value:i64)->i64{value}}fn f()->i64{let chosen=7i64;chosen(1)}fn main(){}',kind:'type',reason:'the source local value is not callable'},
 {name:'a constant pattern never silently turns into a binding',source:'const VALUE:i64=7;fn f(value:i64)->i64{match value{VALUE=>1,_=>2}}fn main(){}',kind:'unsupported',reason:'Rust constant pattern: source constant patterns require their represented value and type'}
];
const revision='9e328b2c',history=['js/src/translation/rust.js','js/src/translation/rust-function-pointers.js','js/src/translation/rust-source-origins.js','js/src/translation/rust-copy-closures.js','rust/src/translation/rust/copy_closures.rs','rust/src/translation/rust/items/functions.rs','rust/src/translation/rust/items/product_bindings.rs','rust/src/translation/rust/source_origins.rs','rust/src/translation/rust/source_origins/type_owners.rs','rust/src/translation/rust/type_namespaces.rs','rust/src/translation/rust/items/constants.rs','rust/src/translation/rust/function_pointers.rs','rust/src/translation/rust/expressions/paths.rs','rust/src/translation/rust/expressions/control.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/rust-source-constants.lino',import.meta.url),{schemaVersion:1,model:'ordinary Rust source constant values distinct from free function items 1',cases,refusals,history,source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; source target_pointer_width=64'},references:['https://doc.rust-lang.org/reference/const_eval.html','https://doc.rust-lang.org/reference/names/scopes.html','https://doc.rust-lang.org/reference/patterns.html#constant-patterns'],assertions:['actualSourceConstantDeclarationIsDistinctFromAFunctionItem','constantReadsKeepValuesIndependentlyOfExpectedCallableTypes','checkedValueCapturedOnceBeforeItsUseOrActualApplication','pointerConstantArgumentsUseTheirOwnActualSignature','sourceQualifiedImportsRawNamesAndLocalValueShadowsPreserved','unqualifiedModuleValuesNeverImplicitlyImportParents','zeroArgumentCallsRemainCallsInPointerContexts','scalarConstantCallsAndUnrepresentedValuePatternsNeverFabricateSemantics','completePredecessorsBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted future controls. Retains the existing constant-as-zero-input-declaration lowering while restoring actual source constant values independently of free function items. This is not complete compile-time constant evaluation: cycle/overflow diagnostics, full constant contexts, value/type-correct constant patterns, promotions, statics, associated/generic constants and all earlier requirements remain obligations.'});
