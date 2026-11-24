// Independent complete originals/literal oracles; no execution on authoring.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustStandardSumPredicateDecisions=[
 {operation:'readRustStandardSumPredicateArity',arguments:['std::option::Option','is_some_and'],expected:1},
 {operation:'readRustStandardSumPredicateArity',arguments:['std::option::Option','is_none_or'],expected:1},
 {operation:'readRustStandardSumPredicateArity',arguments:['std::result::Result','is_ok_and'],expected:1},
 {operation:'readRustStandardSumPredicateArity',arguments:['std::result::Result','is_err_and'],expected:1},
 {operation:'readRustStandardSumPredicateArity',arguments:['std::option::Option','is_err_and'],expected:-1},
 {operation:'readRustStandardSumPredicateArity',arguments:['local::Option','map_or'],expected:-1},
 {operation:'readRustStandardSumPredicateArity',arguments:['std::result::Result','map_or_else'],expected:2},
 {operation:'readRustStandardSumPredicateCaptureRoles',arguments:['map_or'],expected:['receiver','default','mapper']},
 {operation:'readRustStandardSumPredicateCaptureRoles',arguments:['map_or_else'],expected:['receiver','fallback','mapper']},
 {operation:'readRustStandardSumPredicateCaptureRoles',arguments:['is_err_and'],expected:['receiver','predicate']},
 {operation:'readRustStandardSumPredicateCaptureRoles',arguments:['filter'],expected:[]},
 {operation:'readRustStandardSumPredicateCallbackArgument',arguments:['std::option::Option','map_or_else','fallback'],expected:-1},
 {operation:'readRustStandardSumPredicateCallbackArgument',arguments:['std::result::Result','map_or_else','fallback'],expected:1},
 {operation:'readRustStandardSumPredicateCallbackArgument',arguments:['std::result::Result','is_err_and','predicate'],expected:1},
 {operation:'readRustStandardSumPredicateCallbackArgument',arguments:['std::option::Option','is_none_or','predicate'],expected:0},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::option::Option','None','is_none_or'],expected:'true'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::option::Option','Some','is_none_or'],expected:'predicate'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::option::Option','None','is_some_and'],expected:'false'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::result::Result','Ok','is_err_and'],expected:'false'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::result::Result','Err','is_err_and'],expected:'predicate'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::result::Result','Err','map_or'],expected:'default'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::result::Result','Err','map_or_else'],expected:'fallback'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::result::Result','Ok','map_or_else'],expected:'mapper'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::option::Option','Err','is_none_or'],expected:''},
 {operation:'readRustStandardSumPredicateVersionError',arguments:['std::option::Option','is_none_or',90],expected:''},
 {operation:'readRustStandardSumPredicateVersionError',arguments:['std::option::Option','is_none_or',99],expected:''},
 {operation:'readRustStandardSumPredicateVersionError',arguments:['std::option::Option','is_none_or',89],expected:'standard sum predicate and mapping source semantics are uncovered outside Rust >=1.90.0 <1.100.0'},
 {operation:'readRustStandardSumPredicateVersionError',arguments:['std::result::Result','map_or',100],expected:'standard sum predicate and mapping source semantics are uncovered outside Rust >=1.90.0 <1.100.0'},
 {operation:'readRustStandardSumPredicateBorrowError',arguments:['std::option::Option','filter'],expected:'filter needs its actual borrowed predicate parameter and complete reference lifetime model'},
 {operation:'readRustStandardSumPredicateBorrowError',arguments:['std::option::Option','is_some_and'],expected:''},
 {operation:'readRustStandardSumPredicateDeferredError',arguments:['lambda',true,1,1,0,true],expected:''},
 {operation:'readRustStandardSumPredicateDeferredError',arguments:['lambda',true,1,0,0,true],expected:'an unfinished callback needs its actual parameter origins, closure source identity and independently established sibling result'},
 {operation:'readRustStandardSumPredicateResultSource',arguments:[true,true],expected:'first result'},
 {operation:'readRustStandardSumPredicateResultSource',arguments:[false,true],expected:'mapper result'},
 {operation:'readRustStandardSumPredicateResultSource',arguments:[false,false],expected:''},
];
export const rustStandardSumPredicateOriginals=[
 {name:'owned Option predicates select actual payload branches',sourceVersionRange,singleCallOwners:[],stdout:'predicate\ntrue\nfalse\npredicate\nfalse\ntrue\n',source:`
fn predicate(value:u8)->bool{println!("predicate");value>3}
fn main(){println!("{}",Some::<u8>(7).is_some_and(predicate));println!("{}",None::<u8>.is_some_and(predicate));println!("{}",Some::<u8>(1).is_none_or(predicate));println!("{}",None::<u8>.is_none_or(predicate));}
`},
 {name:'owned predicate evaluates factory on absent receiver without invoking callback',sourceVersionRange,singleCallOwners:[],stdout:'receiver\nfactory\ntrue\nreceiver\nfactory\npredicate\ntrue\n',source:`
fn receiver(flag:bool)->Option<u8>{println!("receiver");if flag{Some(7)}else{None}}
fn predicate(value:u8)->bool{println!("predicate");value>3}
fn factory()->fn(u8)->bool{println!("factory");predicate}
fn main(){println!("{}",receiver(false).is_none_or(factory()));println!("{}",receiver(true).is_none_or(factory()));}
`},
 {name:'Result predicates keep successful and error payload owners separate',sourceVersionRange,singleCallOwners:[],stdout:'ok\ntrue\nfalse\nerror\ntrue\nfalse\n',source:`
fn success(value:u8)->bool{println!("ok");value>3}
fn failure(value:u16)->bool{println!("error");value>3}
fn main(){println!("{}",Ok::<u8,u16>(7).is_ok_and(success));println!("{}",Err::<u8,u16>(5).is_ok_and(success));println!("{}",Err::<u8,u16>(5).is_err_and(failure));println!("{}",Ok::<u8,u16>(7).is_err_and(failure));}
`},
 {name:'map_or evaluates default and mapper factory in actual argument order',sourceVersionRange,singleCallOwners:[],stdout:'receiver\ndefault\nfactory\nmapper\n8\nreceiver\ndefault\nfactory\n9\n',source:`
fn receiver(flag:bool)->Option<u8>{println!("receiver");if flag{Some(7)}else{None}}
fn fallback()->u16{println!("default");9}
fn mapper(value:u8)->u16{println!("mapper");u16::from(value)+1}
fn factory()->fn(u8)->u16{println!("factory");mapper}
fn main(){println!("{}",receiver(true).map_or(fallback(),factory()));println!("{}",receiver(false).map_or(fallback(),factory()));}
`},
 {name:'Option map_or_else evaluates factories but invokes only selected callback',sourceVersionRange,singleCallOwners:[],stdout:'receiver\nfallback-factory\nmapper-factory\nmapper\n8\nreceiver\nfallback-factory\nmapper-factory\nfallback\n9\n',source:`
fn receiver(flag:bool)->Option<u8>{println!("receiver");if flag{Some(7)}else{None}}
fn fallback()->u16{println!("fallback");9}
fn mapper(value:u8)->u16{println!("mapper");u16::from(value)+1}
fn fallback_factory()->fn()->u16{println!("fallback-factory");fallback}
fn mapper_factory()->fn(u8)->u16{println!("mapper-factory");mapper}
fn main(){println!("{}",receiver(true).map_or_else(fallback_factory(),mapper_factory()));println!("{}",receiver(false).map_or_else(fallback_factory(),mapper_factory()));}
`},
 {name:'Result map_or_else passes original error only to selected fallback',sourceVersionRange,singleCallOwners:[],stdout:'receiver\nfallback-factory\nmapper-factory\nmapper\n8\nreceiver\nfallback-factory\nmapper-factory\n5\n9\n',source:`
fn receiver(flag:bool)->Result<u8,u16>{println!("receiver");if flag{Ok(7)}else{Err(5)}}
fn fallback(error:u16)->u8{println!("{}",error);9}
fn mapper(value:u8)->u8{println!("mapper");value+1}
fn fallback_factory()->fn(u16)->u8{println!("fallback-factory");fallback}
fn mapper_factory()->fn(u8)->u8{println!("mapper-factory");mapper}
fn main(){println!("{}",receiver(true).map_or_else(fallback_factory(),mapper_factory()));println!("{}",receiver(false).map_or_else(fallback_factory(),mapper_factory()));}
`},
 {name:'unknown Option fallback result finishes from actual mapper source',sourceVersionRange,singleCallOwners:[],stdout:'0\n8\n',source:`
fn choose(value:Option<u8>)->u8{value.map_or_else(||0,|number|number+1)}
fn main(){println!("{}",choose(None));println!("{}",choose(Some(7)));}
`},
 {name:'unknown Result fallback finishes while retaining its real error parameter',sourceVersionRange,singleCallOwners:[],stdout:'5\n0\n8\n',source:`
fn choose(value:Result<u8,u16>)->u8{value.map_or_else(|error|{println!("{}",error);0},|number|number+1)}
fn main(){println!("{}",choose(Err(5)));println!("{}",choose(Ok(7)));}
`},
 {name:'map_or literal default inherits actual mapper result without Unit guess',sourceVersionRange,singleCallOwners:[],stdout:'0\n8\n',source:`
fn choose(value:Option<u8>)->u8{value.map_or(0,|number|number+1)}
fn main(){println!("{}",choose(None));println!("{}",choose(Some(7)));}
`},
 {name:'typed fallback supplies complete pending mapper result',sourceVersionRange,singleCallOwners:[],stdout:'9\n1\n',source:`
fn choose(value:Option<u8>)->u16{value.map_or_else(||9u16,|_|1)}
fn main(){println!("{}",choose(None));println!("{}",choose(Some(7)));}
`},
 {name:'tuple payload predicate retains actual destructured by-value types',sourceVersionRange,singleCallOwners:[],stdout:'true\nfalse\nfalse\n',source:`
fn choose(value:Option<(u8,bool)>)->bool{value.is_some_and(|(number,flag)|flag&&number>3)}
fn main(){println!("{}",choose(Some((7,true))));println!("{}",choose(Some((7,false))));println!("{}",choose(None));}
`},
 {name:'nominal Copy error predicate keeps its actual datatype owner',sourceVersionRange,singleCallOwners:[],stdout:'true\nfalse\nfalse\n',source:`
#[derive(Clone,Copy)]enum Failure{Missing,Present}
fn missing(value:Failure)->bool{match value{Failure::Missing=>true,Failure::Present=>false}}
fn main(){println!("{}",Err::<u8,Failure>(Failure::Missing).is_err_and(missing));println!("{}",Err::<u8,Failure>(Failure::Present).is_err_and(missing));println!("{}",Ok::<u8,Failure>(7).is_err_and(missing));}
`},
];
export const rustStandardSumPredicateRefusals=[
 {name:'owned predicate cannot coerce numeric callback output to Boolean',sourceVersionRange,reasonIncludes:'source',source:'fn number(value:u8)->u8{value}fn wrong(value:Option<u8>)->bool{value.is_some_and(number)}fn main(){}'},
 {name:'owned predicate retains actual by-value parameter over a borrowed callback',sourceVersionRange,source:'fn borrowed(value:&u8)->bool{*value>0}fn wrong(value:Option<u8>)->bool{value.is_some_and(borrowed)}fn main(){}'},
 {name:'Result error predicate cannot use successful payload parameter type',sourceVersionRange,source:'fn wrong_parameter(value:u8)->bool{value>0}fn wrong(value:Result<u8,u16>)->bool{value.is_err_and(wrong_parameter)}fn main(){}'},
 {name:'map_or retains actual eager default result owner',sourceVersionRange,source:'fn mapper(value:u8)->u8{value}fn wrong(value:Option<u8>)->u8{value.map_or(0u16,mapper)}fn main(){}'},
 {name:'map_or_else retains usize owner against equal-width u64 sibling',sourceVersionRange,source:'fn fallback()->u64{0u64}fn mapper(value:u8)->usize{usize::from(value)}fn wrong(value:Option<u8>)->usize{value.map_or_else(fallback,mapper)}fn main(){}'},
 {name:'unresolved callback siblings do not receive guessed enclosing Unit or scalar type',sourceVersionRange,reasonIncludes:'guessed Unit',source:'fn later(value:Option<u8>)->u8{value.map_or_else(||0,|_|1)}fn main(){}'},
 {name:'filter keeps its actual borrowed predicate and lifetime obligation',sourceVersionRange,reasonIncludes:'borrowed predicate',source:'fn later(value:Option<u8>)->Option<u8>{value.filter(|number|*number>0)}fn main(){}'},
];
