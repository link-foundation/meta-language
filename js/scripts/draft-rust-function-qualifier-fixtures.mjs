// Independent future inputs and original source context, never executed here.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,source,expression,value='7',ret='i64'){cases.push({name,module:'const_function_case_'+cases.length,source:source+'pub fn result()->'+ret+'{'+expression+'}',value,abort:null});}
add('an ordinary const function retains its runtime call','const fn read(value:i64)->i64{value+1}','read(6)');
add('a public const function keeps its source visibility','pub const fn read(value:i64)->i64{value+1}','read(6)');
add('a zero input const function remains a call','const fn read()->i64{7}','read()');
add('a const function may call another actual const function','const fn first(value:i64)->i64{value+1}const fn read(value:i64)->i64{first(value)}','read(6)');
add('a const initializer retains its actual source call','const fn read(value:i64)->i64{value+1}const VALUE:i64=read(6);','VALUE');
add('a function declared after its constant remains discoverable','const VALUE:i64=read(6);const fn read(value:i64)->i64{value+1}','VALUE');
add('const recursion retains the ordinary checked recursion','const fn read(value:i64)->i64{if value==0{0}else{1+read(value-1)}}','read(7)');
add('a const function can be an actual function pointer','const fn read(value:i64)->i64{value+1}','let function:fn(i64)->i64=read;function(6)');
add('a pointer constant can refer to a const function','const fn read(value:i64)->i64{value+1}const READ:fn(i64)->i64=read;','READ(6)');
add('const function paths retain their nested module','mod values{pub const fn read(value:i64)->i64{value+1}}','values::read(6)');
add('an imported const function retains its original owner','mod values{pub const fn read(value:i64)->i64{value+1}}use self::values::read;','read(6)');
add('an early returning const function keeps completion order','const fn read(value:i64)->i64{if value>0{return value;}7}','read(7)');
add('a const tuple result retains each field','const fn read()->(i64,bool){(7,true)}','let value=read();if value.1{value.0}else{8}');
add('a const unsigned result keeps every source bit','const fn read()->u64{18446744073709551615}','read()','18446744073709551615','u64');
add('a const Boolean body retains its actual domain','const fn read(flag:bool)->bool{!flag}','if read(false){7}else{8}');
add('a const binary64 body retains its arithmetic','const fn read(value:f64)->f64{value+1.0}','read(6.0)','7','f64');
add('a const raw source name remains distinct from its qualifier','const fn r#move(value:i64)->i64{value+1}','r#move(6)');
add('a raw const name never invents a qualifier','fn r#const()->i64{7}','r#const()');
add('an inherent const constructor retains its actual datatype','struct Value{value:i64}impl Value{const fn new(value:i64)->Self{Self{value}}}','Value::new(7).value');
add('an immutable const method retains its receiver','struct Value{value:i64}impl Value{const fn read(&self)->i64{self.value}}','let value=Value{value:7};value.read()');
add('a const by-value method retains the original move','struct Value{value:i64}impl Value{const fn read(self)->i64{self.value}}','Value{value:7}.read()');
add('a const method may call its actual const associated function','struct Value{value:i64}impl Value{const fn offset()->i64{1}const fn read(&self)->i64{self.value+Self::offset()}}','let value=Value{value:6};value.read()');
add('a scalar associated constant retains its actual owner','struct Value{value:i64}impl Value{const VALUE:i64=7;}','Value::VALUE');
add('an associated constant remains visible through Self','struct Value{value:i64}impl Value{const VALUE:i64=7;fn read()->i64{Self::VALUE}}','Value::read()');
add('an associated constant can use an actual const function','struct Value{value:i64}impl Value{const fn compute()->i64{7}const VALUE:i64=Self::compute();}','Value::VALUE');
add('an associated value declared after its method remains discoverable','struct Value{value:i64}impl Value{fn read()->i64{Self::VALUE}const VALUE:i64=7;}','Value::read()');
add('an associated pointer constant remains a callable value','struct Value{value:i64}impl Value{const READ:fn(i64)->i64=|value|value+1;}','Value::READ(6)');
add('an associated constant may be a complete value of Self','struct Value{value:i64}impl Value{const VALUE:Self=Self{value:7};}','Value::VALUE.value');
add('associated values in distinct datatypes never share owners','struct First{value:i64}struct Last{value:i64}impl First{const VALUE:i64=3;}impl Last{const VALUE:i64=4;}','First::VALUE+Last::VALUE');
add('an enum associated constant retains constructor interpretation','enum Value{Stored(i64)}impl Value{const VALUE:Self=Self::Stored(7);}','match Value::VALUE{Value::Stored(value)=>value}');
add('an associated value homograph preserves its struct constructor','struct Value{value:i64}impl Value{const Value:i64=7;}','let value=Value{value:3};value.value+Value::Value-3');
add('a method named main is not an executable entry','struct Value{value:i64}impl Value{const fn main()->i64{7}}','Value::main()');
const publications=[
 {name:'public ordinary const function has its actual callable signature',source:'pub const fn read(value:i64)->i64{value+1}pub fn observe()->i64{read(6)}',exports:['read','observe'],checks:[{name:'read',call:['6n'],value:'7'}],value:'7'},
 {name:'a private const function stays private behind a public constant',source:'const fn compute()->i64{7}pub const VALUE:i64=compute();pub fn observe()->i64{VALUE}',exports:['VALUE','observe'],checks:[{name:'VALUE',call:null,value:'7'}],value:'7'},
 {name:'a const function pointer export remains a source constant',source:'const fn compute(value:i64)->i64{value+1}pub const READ:fn(i64)->i64=compute;pub fn observe()->i64{READ(6)}',exports:['READ','observe'],checks:[{name:'READ',call:['6n'],value:'7'}],value:'7'}
];
const refusals=[{name:'a qualified executable entry stays an explicit obligation',source:'const fn main(){}',kind:'unsupported',reason:'Rust const function entry: a qualified executable main requires its own entry interpretation'}];
const revision='59ab34c2',history=['js/src/translation/rust.js','js/src/translation/rust-inherent.js','rust/src/translation/rust/items.rs','rust/src/translation/rust/items/functions.rs','rust/src/translation/rust/inherent.rs','rust/src/translation/mod.rs','js/tests/translation-rust-records.test.js'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/rust-function-qualifiers.lino',import.meta.url),{schemaVersion:1,model:'ordinary Rust const function runtime interpretation 1',cases,publications,refusals,history,source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64'},references:['https://doc.rust-lang.org/reference/const_eval.html#const-functions'],assertions:['actualQualifierTokensAndOriginalModuleOwners','ordinaryConstFreeFunctionsMethodsAndCallableValues','associatedConstantsUseActualDatatypeNamespaceAndConstantReads','closedPredicateProofsNeverAssignedByRuntimeAdmission','completeConcreteSourceRetainsOriginalQualifiers','originalRustCompilationAndRuntimeProductsInCI','publicValueInterfacesAndRealCallableSignatures','completePredecessorsBothCodecsFourEmissionsNativeProducts'],boundary:'Unexecuted future controls. This is the runtime interpretation of ordinary admitted Rust function/value constructs; it supplies no complete Rust const-context validity checker, source const-evaluation environment, const ABI, generic/trait/async/unsafe implementation, full privacy or target compile-time interface guarantee. Exact original qualifiers remain in complete source snapshots. The independently checked closed predicate profile is unchanged. All broader source translation and project requirements remain mandatory.'});
