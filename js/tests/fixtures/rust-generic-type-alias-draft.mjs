// Literal future controls only. No imports, execution, observations or evidence.
export const parameterPlans=[
 {kinds:['punct','identifier','punct'],values:['<','T','>'],raws:['<','T','>'],position:0,expected:['parameters','2','','T']},
 {kinds:['punct','identifier','punct','identifier','punct'],values:['<','T',',','U','>='],raws:['<','T',',','U','>='],position:0,expected:['parameters','4','=','T','U']},
 {kinds:['punct','identifier','punct','punct'],values:['<','T',',','>'],raws:['<','T',',','>'],position:0,expected:['parameters','3','','T']},
 {kinds:['punct','identifier','punct'],values:['<','gen','>'],raws:['<','r#gen','>'],position:0,expected:['parameters','2','','gen']},
 {kinds:['punct','identifier','punct'],values:['<','gen','>'],raws:['<','gen','>'],position:0,expected:['error','generic alias needs explicit unconstrained type parameters']},
 {kinds:['punct','punct'],values:['<','>'],raws:['<','>'],position:0,expected:['error','generic alias needs explicit unconstrained type parameters']},
 {kinds:['punct','identifier','punct','identifier','punct'],values:['<','T',',','T','>'],raws:['<','T',',','T','>'],position:0,expected:['error','duplicate generic alias type parameter']},
 {kinds:['punct','identifier','punct','identifier','punct'],values:['<','T',':','Copy','>'],raws:['<','T',':','Copy','>'],position:0,expected:['error','generic alias parameter bounds and defaults need their complete interpretation']},
 {kinds:['punct'],values:['<'],raws:[],position:0,expected:['error','unaligned generic alias token columns']}
];
export const substitutionPlans=[
 {parameters:['alias-1/parameter-0','alias-1/parameter-1'],argumentsProven:[true,true],occurrences:['alias-1/parameter-1','module-2/type-T','alias-1/parameter-0','alias-1/parameter-1'],expected:['substitution','1','-1','0','1']},
 {parameters:['alias-1/parameter-0'],argumentsProven:[],occurrences:[],expected:['error','generic alias argument arity differs from its declaration']},
 {parameters:['alias-1/parameter-0'],argumentsProven:[false],occurrences:[],expected:['error','generic alias arguments need complete checked source origins']},
 {parameters:['same','same'],argumentsProven:[true,true],occurrences:[],expected:['error','generic alias parameter binding identities must be actual and distinct']},
 {parameters:['actual-parameter'],argumentsProven:[true],occurrences:[''],expected:['error','generic alias occurrence needs an actual resolved source binding']}
];
export const expansionPlans=[
 {declaration:'crate.Alias',declarations:[],expansions:0,expected:''},
 {declaration:'crate.Alias',declarations:['crate.Alias'],expansions:1,expected:'recursive source type alias'},
 {declaration:'crate.Last',declarations:['crate.First','crate.Last'],expansions:2,expected:'recursive source type alias'},
 {declaration:'crate.Alias',declarations:[],expansions:4096,expected:'source type alias expansion limit exceeded'},
 {declaration:'',declarations:[],expansions:0,expected:'generic alias expansion needs actual declaration owners'}
];
// These are full-frontend source drafts with independent expected results;
// neither the represented programs nor their frontend controls have executed.
export const sourcePrograms=[
 {name:'identity keeps independent concrete instantiations',source:'type Identity<T>=T;pub fn result()->i64{let first:Identity<i64>=7;let second:Identity<bool>=true;if second{first}else{0}}',expected:'7'},
 {name:'permuted parameters use binding identities',source:'type Swap<T,U>=(U,T);pub fn result()->i64{let value:Swap<bool,i64>=(7,true);if value.1{value.0}else{0}}',expected:'7'},
 {name:'caller primitive names do not replace argument origins',source:'mod values{pub type Identity<T>=T;pub fn read(value:Identity<i64>)->i64{value}}struct i64{value:bool}pub fn result()->values::Identity<core::primitive::i64>{values::read(7)}',expected:'7'},
 {name:'nested applications retain closing token coordinates',source:'type Identity<T>=T;pub fn result()->i64{let value:Identity<Identity<i64>>=7;value}',expected:'7'},
 {name:'composed templates retain reordered parameter bindings',source:'type Swap<T,U>=(U,T);type Pair<T>=Swap<T,i64>;pub fn result()->i64{let value:Pair<bool>=(7,true);if value.1{value.0}else{0}}',expected:'7'},
 {name:'unused well-formed identity is checked without exporting a value',source:'type Identity<T>=T;pub fn result()->i64{7}',expected:'7'},
 {name:'the preceding generic refusal input has a complete checked declaration',source:'type Identity<T>=T;fn main(){}',expectedExports:[],predecessorAssertion:'generic, constrained or malformed type aliases require their complete parameter interpretation'},
 {name:'concrete leaves and Unit retain definition context',source:'type Pair<T>=(T,(i64,()));pub fn result()->i64{let value:Pair<bool>=(true,(7,()));if value.0{value.1.0}else{0}}',expected:'7'},
 {name:'generic Option aliases instantiate the actual standard sum',source:'type Maybe<T>=Option<T>;pub fn result()->i64{let value:Maybe<i64>=Some(7);match value{Some(value)=>value,None=>0}}',expected:'7'},
 {name:'nested generic Option aliases preserve both named sum layers',source:'type Maybe<T>=Option<T>;type Nested<T>=Maybe<Maybe<T>>;pub fn result()->i64{let value:Nested<i64>=Some(Some(7));match value{Some(Some(value))=>value,Some(None)=>0,None=>1}}',expected:'7'},
 {name:'a local Result alias retains its fully qualified library definition',source:'type Error=i64;type Result<T>=std::result::Result<T,Error>;fn read()->Result<i64>{Ok(7)}pub fn result()->i64{match read(){Ok(value)=>value,Err(error)=>error}}',expected:'7'},
 {name:'symbolic nominal payload wellformedness does not invent Copy evidence',source:'enum Error{Bad}type Result<T>=std::result::Result<T,Error>;pub fn result()->i64{7}',expected:'7'},
 {name:'a generic vector alias retains an ordinary scalar parameter',source:'type Values<T>=std::vec::Vec<T>;fn read(values:Values<i64>)->i64{values[0]}pub fn result()->i64{read(vec![7])}',expected:'7'},
 {name:'a generic vector alias retains a fresh scalar result',source:'type Values<T>=std::vec::Vec<T>;fn make()->Values<i64>{vec![7]}pub fn result()->i64{make()[0]}',expected:'7'},
 {name:'generic vector aliases retain actual fresh call forwarding',source:'type Values<T>=Vec<T>;fn forward()->Values<i64>{make()}fn make()->Values<i64>{vec![7i64]}pub fn result()->i64{forward()[0]}',expected:'7'},
 {name:'generic vector aliases retain complete pointer-sized element owners',source:'type Values<T>=Vec<T>;fn read(value:Values<usize>)->usize{value[0]}pub fn result()->usize{read(vec![7usize])}',expected:'7'},
 {name:'generic vector aliases retain all unsigned 128-bit element bits',source:'type Values<T>=Vec<T>;fn make()->Values<u128>{vec![340282366920938463463374607431768211455u128]}pub fn result()->u128{make()[0]}',expected:'340282366920938463463374607431768211455'},
 {name:'generic vector aliases compose through actual identity arguments',source:'type Identity<T>=T;type Values<T>=Vec<Identity<T>>;fn make()->Values<i64>{Vec::<i64>::new()}pub fn result()->usize{make().len()}',expected:'0'},
 {name:'concrete aliases of generic vectors retain existing empty construction',source:'type Values<T>=Vec<T>;type Integers=Values<i64>;pub fn result()->usize{Integers::new().len()}',expected:'0'},
 {name:'owned String vector elements retain their distinct source owner',source:'type Values<T>=Vec<T>;type Words=Values<String>;pub fn result()->usize{Words::new().len()}',expected:'0'},
 {name:'generic vector signatures preserve owned String elements without Copy claims',source:'type Values<T>=Vec<T>;fn count(values:Values<String>)->usize{values.len()}pub fn result()->i64{7}',expected:'7',vectorSignature:{function:'count',elementKind:'string',elementOwner:'std::string::String'}},
 {name:'alternating generic vector specializations never share stale element origins',source:'type Values<T>=Vec<T>;type Integers=Values<i64>;type Words=Values<String>;fn count(values:Words)->usize{values.len()}pub fn result()->usize{Integers::new().len()+Words::new().len()}',expected:'0',vectorSignature:{function:'count',elementKind:'string',elementOwner:'std::string::String'}},
 {name:'qualified generic vector aliases survive caller Vec shadows',source:'mod values{pub type Values<T>=std::vec::Vec<T>;pub fn make()->Values<i64>{vec![7]}}struct Vec{value:bool}pub fn result()->i64{values::make()[0]}',expected:'7'},
 {name:'a local Vec alias retains its fully qualified library template',source:'type Vec<T>=std::vec::Vec<T>;fn read(value:Vec<i64>)->i64{value[0]}pub fn result()->i64{read(vec![7])}',expected:'7'},
 {name:'unused vector and tuple composition checks symbolic bindings only',source:'type Pair<T>=(T,T);type Values<T>=Vec<T>;type Rows<T>=Values<Pair<T>>;pub fn result()->i64{7}',expected:'7'},
 {name:'unused vector and sum composition never invents a Copy proof',source:'type Values<T>=Vec<T>;type MaybeValues<T>=Option<Values<T>>;pub fn result()->i64{7}',expected:'7'},
 {name:'the preceding symbolic vector refusal now retains a checked declaration',source:'type Vector<T>=Vec<T>;fn main(){}',expectedExports:[],previousExpectation:{kind:'unsupported',reason:'Rust array element: only direct immutable scalar and owned string elements are admitted'}},
 {name:'unused missing owner still refuses',source:'type Missing<T>=(T,Absent);pub fn result()->i64{7}',refusal:'unresolved definition owner before publication'},
 {name:'changing arguments do not evade recursion',source:'type Recursive<T>=Recursive<Box<T>>;pub fn result()->i64{7}',refusal:'recursive source type alias'},
 {name:'bounds remain unresolved',source:'type Bounded<T:Copy>=T;pub fn result()->i64{7}',refusal:'complete parameter-bound interpretation required'},
 {name:'a standard sum template checks all symbolic argument positions',source:'type Bad<T>=Option<T,T>;pub fn result()->i64{7}',refusal:'complete standard sum arity required'},
 {name:'a library constructor cannot become a type template',source:'type Bad<T>=Some<T>;pub fn result()->i64{7}',refusal:'standard constructor is not a type owner'},
 {name:'a local standard-spelled datatype remains local',source:'struct Option{value:i64}type Bad<T>=Option<T>;pub fn result()->i64{7}',refusal:'actual local generic datatype interpretation required'},
 {name:'a local Result alias is recursive without its actual library path',source:'type Error=i64;type Result<T>=Result<T,Error>;pub fn result()->i64{7}',refusal:'recursive source type alias'},
 {name:'a vector template retains complete explicit arity',source:'type Values<T>=Vec<T,T>;pub fn result()->i64{7}',refusal:'one vector element required'},
 {name:'a local standard-spelled vector datatype remains local',source:'struct Vec{value:i64}type Values<T>=Vec<T>;pub fn result()->i64{7}',refusal:'local generic datatype interpretation required'},
 {name:'a source std module cannot impersonate the vector library',source:'mod std{}type Values<T>=std::vec::Vec<T>;pub fn result()->i64{7}',refusal:'actual unshadowed vector definition required'},
 {name:'a generic vector alias cannot make borrowed elements owned',source:'type Values<T>=Vec<T>;fn read(value:Values<&i64>)->usize{value.len()}pub fn result()->i64{7}',refusal:'borrowed vector element interpretation required'},
 {name:'a generic vector alias retains the existing tuple element boundary',source:'type Values<T>=Vec<T>;fn read(value:Values<(i64,bool)>)->usize{value.len()}pub fn result()->i64{7}',refusal:'tuple vector element interpretation required'},
 {name:'generic vectors never conflate usize with an equal-width u64 owner',source:'type Values<T>=Vec<T>;fn read(value:Values<usize>)->usize{value[0]}pub fn result()->usize{read(vec![7u64])}',refusal:'complete pointer-sized source owner must agree'},
 {name:'vector results retain their actual fresh-tail obligation',source:'type Values<T>=Vec<T>;fn reuse(value:Values<i64>)->Values<i64>{value}pub fn result()->i64{7}',refusal:'existing local is not a fresh vector constructor'},
 {name:'owned String vectors do not gain scalar result Copy evidence',source:'type Values<T>=Vec<T>;fn make()->Values<String>{vec![String::from("x")]}pub fn result()->i64{7}',refusal:'owned String vector result interpretation required'},
 {name:'generic String vector aliases never invent scalar lookup Copy evidence',source:'type Values<T>=Vec<T>;type Words=Values<String>;pub fn result()->String{Words::new().first().copied().unwrap_or_default()}',refusal:'owned String vector elements are not scalar Copy lookups'},
 {name:'a sum of generic vectors keeps its actual nonCopy payload',source:'type Values<T>=Vec<T>;type MaybeValues<T>=Option<Values<T>>;fn read()->MaybeValues<i64>{None}pub fn result()->i64{7}',refusal:'vector is not a direct Copy sum payload'},
 {name:'missing arguments never infer from return context',source:'type Identity<T>=T;pub fn result()->Identity{7}',refusal:'explicit argument arity required'}
];
export const declarationPlans=[
 {kinds:['keyword','identifier','punct','identifier','punct','identifier','punct'],values:['type','Identity','<','T','>=','T',';'],raws:['type','Identity','<','T','>=','T',';'],expected:['aliases','Identity','0','5','6','7','1','T']},
 {kinds:['keyword','identifier','punct','identifier','punct'],values:['type','Number','=','i64',';'],raws:['type','Number','=','i64',';'],expected:['aliases','Number','0','3','4','5','0']},
 {kinds:['identifier','identifier','punct','punct','punct','keyword','identifier','punct','identifier','punct','identifier','punct','punct'],values:['fn','read','(',')','{','type','Local','<','T','>=','T',';','}'],raws:['fn','read','(',')','{','type','Local','<','T','>=','T',';','}'],expected:['aliases']}
];
export const vectorScalarDomains=[
 {name:'i8',kind:'fixed',bits:'8',signed:true},{name:'u8',kind:'fixed',bits:'8',signed:false},
 {name:'i16',kind:'fixed',bits:'16',signed:true},{name:'u16',kind:'fixed',bits:'16',signed:false},
 {name:'i32',kind:'fixed',bits:'32',signed:true},{name:'u32',kind:'fixed',bits:'32',signed:false},
 {name:'i64',kind:'fixed',bits:'64',signed:true},{name:'u64',kind:'fixed',bits:'64',signed:false},
 {name:'i128',kind:'fixed',bits:'128',signed:true},{name:'u128',kind:'fixed',bits:'128',signed:false},
 {name:'isize',kind:'fixed',bits:'64',signed:true},{name:'usize',kind:'fixed',bits:'64',signed:false},
 {name:'bool',kind:'bool',bits:'',signed:false},{name:'f64',kind:'float',bits:'',signed:false},
 {name:'()',kind:'unit',bits:'',signed:false}
];
export const boundary='Both frontend adapters are source drafts for explicit identity, scalar/Unit and actual nominal leaves, grouping/tuples, composed aliases and canonical standard Option/Result/Vec templates. Rust >=1.90.0 <1.100.0, edition 2024, target_pointer_width=64 is the proposed existing source fixture profile. Policies require actual lexer, declaration-scoped binding and complete concrete-origin facts. Tuple instantiations retain the existing product admission. Other parameter-dependent library/wrapper/callable templates, bounds, defaults, lifetime/const parameters and inference still refuse. Native source/Links generation, complete independent controls, source/ledger registration and all execution are deferred; this supplies no runtime parity or semantic-equivalence evidence.';
