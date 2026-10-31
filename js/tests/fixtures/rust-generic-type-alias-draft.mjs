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
// These are future full-frontend obligations, not newly admitted programs.
export const sourcePrograms=[
 {name:'identity keeps independent concrete instantiations',source:'type Identity<T>=T;pub fn result()->i64{let first:Identity<i64>=7;let second:Identity<bool>=true;if second{first}else{0}}',expected:'7'},
 {name:'permuted parameters use binding identities',source:'type Swap<T,U>=(U,T);pub fn result()->i64{let value:Swap<bool,i64>=(7,true);if value.1{value.0}else{0}}',expected:'7'},
 {name:'caller primitive names do not replace argument origins',source:'mod values{pub type Identity<T>=T;pub fn read(value:Identity<i64>)->i64{value}}struct i64{value:bool}pub fn result()->values::Identity<core::primitive::i64>{values::read(7)}',expected:'7'},
 {name:'nested applications retain closing token coordinates',source:'type Identity<T>=T;pub fn result()->i64{let value:Identity<Identity<i64>>=7;value}',expected:'7'},
 {name:'unused missing owner still refuses',source:'type Missing<T>=(T,Absent);pub fn result()->i64{7}',refusal:'unresolved definition owner before publication'},
 {name:'changing arguments do not evade recursion',source:'type Recursive<T>=Recursive<Box<T>>;pub fn result()->i64{7}',refusal:'recursive source type alias'},
 {name:'bounds remain unresolved',source:'type Bounded<T:Copy>=T;pub fn result()->i64{7}',refusal:'complete parameter-bound interpretation required'},
 {name:'missing arguments never infer from return context',source:'type Identity<T>=T;pub fn result()->Identity{7}',refusal:'explicit argument arity required'}
];
export const boundary='Unintegrated source-policy draft. Rust >=1.90.0 <1.100.0, edition 2024, target_pointer_width=64 is the proposed existing source fixture profile. No frontend admits these generic aliases yet. Policies require actual lexer, resolved binding and checked-origin facts; they establish no symbolic type checking or semantic equivalence. Native source/Links generation, complete independent controls, source/ledger registration and all execution are deferred.';
