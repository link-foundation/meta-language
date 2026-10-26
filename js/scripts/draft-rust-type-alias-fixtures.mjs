// Complete future alias inputs and predecessor definitions, never executed.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,source,body,value='7',ret='i64'){cases.push({name,module:'type_alias_case_'+cases.length,source:source+'pub fn result()->'+ret+'{'+body+'}',value,abort:null});}
add('a source scalar alias keeps its existing type','type Number=i64;','let value:Number=7;value');
add('an alias remains transparent in both function positions','type Number=i64;fn read(value:Number)->Number{value+1}','read(6)');
add('a later alias owns an earlier function signature','fn read(value:Number)->Number{value+1}type Number=i64;','read(6)');
add('reference token splitting preserves later alias source coordinates','fn read(value:&&i64)->i64{**value}type Number=i64;','let value:Number=7;let first=&value;let second=&first;read(second)');
add('a forward alias chain retains every definition','type First=Last;type Last=i64;','let value:First=7;value');
add('a public alias is still a source type','pub type Number=i64;','let value:Number=7;value');
add('aliases retain their original module owner','mod values{pub type Number=i64;pub fn read(value:Number)->Number{value+1}}','values::read(6)');
add('an imported alias keeps the provider definition','mod values{pub type Number=i64;}use self::values::Number;fn read(value:Number)->Number{value+1}','read(6)');
add('a later import resolves the complete alias body context','type Number=Imported;mod values{pub type Base=i64;}use self::values::Base as Imported;','let value:Number=7;value');
add('a child alias can reference its actual parent type','type Number=i64;mod values{pub type Alias=super::Number;pub fn read(value:Alias)->Alias{value+1}}','values::read(6)');
add('an alias uses its own module when the consumer shadows a datatype','mod values{pub struct Value{pub value:i64}pub type Alias=Value;pub fn read(value:Alias)->i64{value.value}}struct Value{value:bool}fn read(value:values::Alias)->i64{values::read(value)}','read(values::Value{value:7})');
add('a later datatype remains the alias definition owner','type Alias=Value;struct Value{value:i64}fn read(value:Alias)->i64{value.value}','read(Value{value:7})');
add('an alias may name an actual enum','enum Value{Stored(i64)}type Alias=Value;fn read(value:Alias)->i64{match value{Value::Stored(value)=>value}}','read(Value::Stored(7))');
add('distinct module aliases retain independent scalar widths','mod first{pub type Number=i32;pub fn read()->Number{3}}mod last{pub type Number=i64;pub fn read()->Number{4}}','first::read() as i64+last::read()');
add('a recursively Copy tuple alias retains every field','type Pair=(i64,bool);fn read(value:Pair)->i64{if value.1{value.0}else{8}}','read((7,true))');
add('an alias chain can contain a complete tuple definition','type Number=i64;type Pair=(Number,Number);','let pair:Pair=(3,4);pair.0+pair.1');
add('a callable alias retains its actual source signature','type Function=fn(i64)->i64;fn read(value:i64)->i64{value+1}fn apply(function:Function)->i64{function(6)}','apply(read)');
add('a callable alias can own a pointer constant','type Function=fn(i64)->i64;const READ:Function=|value|value+1;','READ(6)');
add('a callable alias can be a complete source result','type Function=fn(i64)->i64;fn read(value:i64)->i64{value+1}fn choose()->Function{read}','choose()(6)');
add('a Box alias retains the existing owned wrapper','type Wrapped=Box<i64>;fn read(value:Wrapped)->i64{*value}','read(Box::new(7))');
add('an explicit reference alias retains its source lifetime','type Reference=&\'static i64;fn read(value:Reference)->i64{*value}','read(&7)');
add('an owned text alias keeps UTF8 byte observations','type Text=String;fn read(value:Text)->usize{value.len()}','read(String::from("😀"))','4','usize');
add('an alias to a scalar vector retains ordered element reads','type Values=Vec<i64>;fn read(value:Values)->i64{value[0]+value[1]}','read(vec![3,4])');
add('an unsigned alias retains all original bits','type Number=u64;','let value:Number=18446744073709551615;value','18446744073709551615','u64');
add('a binary64 alias retains its actual arithmetic','type Number=f64;fn read(value:Number)->Number{value+1.0}','read(6.0)','7','f64');
add('a Boolean alias remains distinct from numeric aliases','type Flag=bool;fn read(value:Flag)->Flag{!value}','if read(false){7}else{8}');
add('a raw alias name retains its actual source spelling','type r#move=i64;','let value:r#move=7;value');
add('a Unicode alias keeps its canonical source name','type e\u0301=i64;','let value:e\u0301=7;value');
add('a raw function name never invents an alias declaration','fn r#type()->i64{7}','r#type()');
add('an unused alias remains checked without a runtime declaration','type Unused=i64;','7');
add('an alias and a function can inhabit their distinct namespaces','type Read=i64;fn Read(value:Read)->Read{value+1}','Read(6)');
add('a direct alias constructs its actual named record','struct Value{value:i64}type Alias=Value;','Alias{value:7}.value');
add('a direct alias retains its actual record pattern','struct Value{value:i64}type Alias=Value;','match (Value{value:7}){Alias{value}=>value}');
add('record updates through aliases retain their original owner','struct Value{first:i64,last:i64}type Alias=Value;','let base=Value{first:1,last:4};let value=Alias{first:3,..base};value.first+value.last');
add('a nominal alias chain retains the actual record constructor','struct Value{value:i64}type First=Value;type Alias=First;','Alias{value:7}.value');
add('a forward nominal alias retains its later constructor owner','type Alias=Value;struct Value{value:i64}','Alias{value:7}.value');
add('an alias retains an enum tuple constructor','enum Value{Stored(i64)}type Alias=Value;','match Alias::Stored(7){Value::Stored(value)=>value}');
add('an alias retains an enum tuple pattern','enum Value{Stored(i64)}type Alias=Value;','match Value::Stored(7){Alias::Stored(value)=>value}');
add('an alias retains complete constructor rest patterns','enum Value{Stored(i64,bool)}type Alias=Value;','match Alias::Stored(7,true){Alias::Stored(value,..)=>value}');
add('an alias retains an enum unit value','enum Value{First,Last}type Alias=Value;','match Alias::First{Value::First=>7,Value::Last=>8}');
add('an alias retains both enum unit alternatives','enum Value{First,Last}type Alias=Value;','match Value::First{Alias::First=>7,Alias::Last=>8}');
add('an alias retains a named enum construction and pattern','enum Value{Stored{value:i64}}type Alias=Value;','match (Alias::Stored{value:7}){Alias::Stored{value}=>value}');
add('an imported nominal alias retains its original variant owner','mod values{pub enum Value{Stored(i64)}pub type Alias=Value;}use self::values::Alias;','match Alias::Stored(7){values::Value::Stored(value)=>value}');
add('an alias reaches the actual associated function','struct Value{value:i64}type Alias=Value;impl Value{fn read()->i64{7}}','Alias::read()');
add('an alias reaches the actual associated scalar value','struct Value{value:i64}type Alias=Value;impl Value{const VALUE:i64=7;}','Alias::VALUE');
add('an alias reaches a callable associated constant','struct Value{value:i64}type Alias=Value;impl Value{const READ:fn(i64)->i64=|value|value+1;}','Alias::READ(6)');
add('an alias preserves the receiver of an associated method call','struct Value{value:i64}type Alias=Value;impl Value{fn read(&self)->i64{self.value}}','let value=Value{value:7};Alias::read(&value)');
add('an inherent implementation through an alias belongs to its datatype','struct Value{value:i64}type Alias=Value;impl Alias{fn new(value:i64)->Self{Self{value}}fn read(&self)->i64{self.value}}','let value=Alias::new(7);value.read()');
add('an earlier alias implementation retains its later datatype','type Alias=Value;impl Alias{fn read(&self)->i64{self.value}}struct Value{value:i64}','let value=Value{value:7};Alias::read(&value)');
add('an inner datatype shadows an outer alias in the type namespace','type Alias=i64;mod values{pub struct Alias{pub value:i64}pub fn read(value:Alias)->i64{value.value}}','values::read(values::Alias{value:7})');
add('an inner module shadows an outer alias for associated paths','type Alias=i64;mod values{pub mod Alias{pub fn read()->i64{7}}pub fn read()->i64{Alias::read()}}','values::read()');
add('a local value does not replace a braced alias type','struct Value{value:i64}type Alias=Value;','let Alias=7;Alias{value:Alias}.value');
add('a local value does not replace an associated alias type','struct Value{value:i64}type Alias=Value;impl Value{fn read()->i64{7}}','let Alias=99;Alias::read()');
add('a pattern binding can share a source type alias name','type Alias=i64;','let value=7i64;match value{Alias=>Alias}');
const publications=[
 {name:'private aliases give public functions real expanded signatures',source:'type Number=i64;pub fn read(value:Number)->Number{value+1}pub fn observe()->Number{read(6)}',exports:['read','observe'],checks:[{name:'read',call:['6n'],value:'7'}],value:'7'},
 {name:'public aliases add no fabricated JavaScript value exports',source:'pub type Number=i64;pub const VALUE:Number=7;pub fn observe()->Number{VALUE}',exports:['VALUE','observe'],checks:[{name:'VALUE',call:null,value:'7'}],value:'7'},
 {name:'public callable constants preserve expanded alias signatures',source:'type Function=fn(i64)->i64;pub const READ:Function=|value|value+1;pub fn observe()->i64{READ(6)}',exports:['READ','observe'],checks:[{name:'READ',call:['6n'],value:'7'}],value:'7'}
];
const refusals=[];
function refuse(name,source,message){refusals.push({name,source:source+'fn main(){}',kind:'unsupported',reason:'Rust type alias: '+message});}
refuse('an unused alias cannot hide a missing source datatype','type Missing=Absent;','the complete alias definition needs an actual source datatype owner');
refuse('a direct recursive alias has no finite type expansion','type Recursive=Recursive;','recursive source type alias');
refuse('mutually recursive aliases keep both original definitions','type First=Last;type Last=First;','recursive source type alias');
refuse('wrappers cannot hide a recursive alias','type Recursive=Box<Recursive>;','recursive source type alias');
refuse('generic aliases retain their complete parameter obligation','type Identity<T>=T;','generic, constrained or malformed type aliases require their complete parameter interpretation');
refuse('where predicates retain their actual source obligation','type Identity<T> where T:Copy=T;','generic, constrained or malformed type aliases require their complete parameter interpretation');
refuse('duplicate aliases retain negative namespace coverage','type Number=i64;type Number=i32;','duplicate source type alias');
refuse('an alias cannot replace an actual datatype','struct Value{value:i64}type Value=i64;','a type alias cannot replace another source type or module');
refuse('an alias cannot replace an actual module','mod values{}type values=i64;','a type alias cannot replace another source type or module');
refuse('a complete alias RHS cannot carry trailing type syntax','type Number=i64 i32;','the complete alias definition needs one admitted source type');
refuse('a borrowed alias cannot invent an elided item lifetime','type Reference=&i64;','reference aliases require explicit source lifetimes');
refuse('a nested alias needs an explicit parent item path','type Number=i64;mod values{type Missing=Number;}','the complete alias definition needs an actual source datatype owner');
refuse('deep chains use their declared expansion depth bound',Array.from({length:130},(_,index)=>'type Alias'+index+'='+(index===129?'i64':'Alias'+(index+1))+';').join(''),'source type alias depth limit exceeded');
refuse('a small doubling definition cannot allocate an exponential type tree','type Alias0=i64;'+Array.from({length:22},(_,index)=>'type Alias'+(index+1)+'=(Alias'+index+',Alias'+index+');').join(''),'source type alias materialization limit exceeded');
for(const [name,source] of [['a tuple alias is not a bare tuple value constructor','struct Value(i64);type Alias=Value;fn read()->Value{Alias(7)}'],['a unit alias is not a bare unit value constructor','struct Value;type Alias=Value;fn read()->Value{Alias}']])refusals.push({name,source:source+'fn main(){}',kind:'unsupported',reason:'Rust type alias value constructor: a source type alias has no independent value constructor'});
const plans=[{name:'macro token trees do not declare aliases',source:'macro_rules! names{()=>{type Phantom=Missing;}}fn main(){}',keys:[]},{name:'function-local aliases keep a separate source obligation',source:'fn read(){type Local=i64;}fn main(){}',keys:[]},{name:'actual inline modules own complete alias rows',source:'type First=i64;mod nested{type Last=super::First;}fn main(){}',keys:['First','nested.Last']}];
const revision='900d58ff',history=['js/src/translation/rust.js','js/src/translation/rust-module-value-rules.js','rust/src/translation/rust.rs','rust/src/translation/rust/items.rs','rust/src/translation/rust/items/types.rs','rust/src/translation/mod.rs','js/tests/translation-rust-records.test.js','experiments/rust-records/const-publications.mjs','rust/tests/unit/translation_rust_function_qualifiers.rs','js/scripts/draft-rust-source-project-fixtures.mjs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
for(const path of ['js/src/translation/rust-type-alias-rules.js','js/src/translation/rust-type-aliases.js','js/src/translation/rust.js','js/src/translation/rust-records.js','js/src/translation/rust-inherent.js','js/src/translation/rust-source-constants.js','rust/src/translation/rust/type_aliases.rs','rust/src/translation/rust/records.rs','rust/src/translation/rust/inherent.rs','rust/src/translation/rust/source_constants.rs','rust/src/translation/rust/expressions/control.rs','rust/src/translation/rust/expressions/paths.rs','rust/src/translation/rust/items.rs','parity/fixtures/rust-type-aliases.lino','js/tests/translation-rust-type-aliases.test.js','rust/tests/unit/translation_rust_type_aliases.rs','js/scripts/draft-rust-type-alias-inventory.mjs']){const revision='8a272f0c',bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:8*1024*1024});history.push({path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')});}
writeProjectData(new URL('../../parity/fixtures/rust-type-aliases.lino',import.meta.url),{schemaVersion:1,model:'ordinary transparent Rust source type alias definitions 1',cases,publications,refusals,plans,history,source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64'},references:['https://doc.rust-lang.org/reference/items/type-aliases.html','https://doc.rust-lang.org/error_codes/E0106.html','https://doc.rust-lang.org/reference/names/scopes.html#item-scopes'],assertions:['actualModuleScopesAndIndependentTypeAndValueNamespaces','directNominalAliasConstructionsPatternsAndAssociatedItems','bareTupleAndUnitAliasesNeverInventValueConstructors','independentNativeLexerAndActualShippedPlanningRules','actualModuleDeclarationsAndCompleteDefinitionOrder','transparentExpandedTypesWithOriginalLexicalOwners','importedForwardRecursiveTupleCallableAndWrapperDefinitions','everyUnusedAliasCheckedBeforePublishingAnyInterface','cyclesUnknownOwnersNamespaceConflictsAndAllBoundsExplicit','originalAliasesRemainInCompleteConcreteSourceSnapshots','publicRuntimeValuesNeverInventATypeAliasExport','originalRustProductsBothCodecsFourEmissionsAndCompletePredecessors'],boundary:'Future unexecuted controls. The ordinary source type positions unfold nongeneric module aliases through the existing admitted type/ownership interpretation. This does not implement generic/constrained/local/associated type aliases, wrapper/primitive/generic and remaining constructor or inherent paths through aliases, arbitrary source lifetimes, complete namespace/privacy analysis or Rust generic monomorphization. Complete original CSTs retain every alias spelling and definition; transparent typing supplies no cross-language concept deduplication certificate. All preceding requirements remain mandatory.'});
