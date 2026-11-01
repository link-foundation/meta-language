// Independently authored source contracts; no observation or completion claim.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustOwnedStandardSumPredicateDecisions=[
 {operation:'readRustOwnedStandardSumPredicateResultError',arguments:['copy',true],expected:''},
 {operation:'readRustOwnedStandardSumPredicateResultError',arguments:['owned',true],expected:''},
 {operation:'readRustOwnedStandardSumPredicateResultError',arguments:['',true],expected:'an eager default or first callback must establish its actual complete Copy or supported owned result type'},
 {operation:'readRustOwnedStandardSumPredicateResultError',arguments:['copy',false],expected:'an eager default or first callback must establish its actual complete Copy or supported owned result type'},
 {operation:'readRustStandardSumPredicateArity',arguments:['std::option::Option','is_some_and'],expected:1},
 {operation:'readRustStandardSumPredicateArity',arguments:['std::option::Option','is_none_or'],expected:1},
 {operation:'readRustStandardSumPredicateArity',arguments:['std::result::Result','is_ok_and'],expected:1},
 {operation:'readRustStandardSumPredicateArity',arguments:['std::result::Result','is_err_and'],expected:1},
 {operation:'readRustStandardSumPredicateArity',arguments:['std::option::Option','map_or'],expected:2},
 {operation:'readRustStandardSumPredicateArity',arguments:['std::result::Result','map_or_else'],expected:2},
 {operation:'readRustStandardSumPredicateCaptureRoles',arguments:['is_some_and'],expected:['receiver','predicate']},
 {operation:'readRustStandardSumPredicateCaptureRoles',arguments:['map_or'],expected:['receiver','default','mapper']},
 {operation:'readRustStandardSumPredicateCaptureRoles',arguments:['map_or_else'],expected:['receiver','fallback','mapper']},
 {operation:'readRustStandardSumPredicateCallbackArgument',arguments:['std::option::Option','map_or','mapper'],expected:0},
 {operation:'readRustStandardSumPredicateCallbackArgument',arguments:['std::option::Option','map_or_else','fallback'],expected:-1},
 {operation:'readRustStandardSumPredicateCallbackArgument',arguments:['std::result::Result','map_or_else','fallback'],expected:1},
 {operation:'readRustStandardSumPredicateCallbackArgument',arguments:['std::result::Result','is_err_and','predicate'],expected:1},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::option::Option','Some','is_some_and'],expected:'predicate'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::option::Option','None','is_some_and'],expected:'false'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::result::Result','Ok','is_err_and'],expected:'false'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::result::Result','Err','is_err_and'],expected:'predicate'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::option::Option','Some','map_or'],expected:'mapper'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::option::Option','None','map_or'],expected:'default'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::result::Result','Ok','map_or_else'],expected:'mapper'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::result::Result','Err','map_or_else'],expected:'fallback'},
 {operation:'readRustStandardSumPredicateBranch',arguments:['std::option::Option','None','is_none_or'],expected:'true'},
 {operation:'readRustStandardSumPredicateVersionError',arguments:['std::option::Option','map_or',90],expected:''},
 {operation:'readRustStandardSumPredicateVersionError',arguments:['std::result::Result','is_err_and',99],expected:''},
 {operation:'readRustStandardSumPredicateVersionError',arguments:['std::option::Option','map_or_else',100],expected:'standard sum predicate and mapping source semantics are uncovered outside Rust >=1.90.0 <1.100.0'},
];
export const rustOwnedStandardSumPredicateOriginals=[
 {name:'owned is_some_and consumes its payload only in the selected invocation',sourceVersionRange,stdout:'predicate\ntrue\nfalse\n',source:`
fn nonempty(text:String)->bool{println!("predicate");text.len()>0usize}
fn main(){let some:Option<String>=Some(String::from("text"));let none:Option<String>=None;println!("{}",some.is_some_and(nonempty));println!("{}",none.is_some_and(nonempty));}
`},
 {name:'owned Result predicates select their actual success or error parameter',sourceVersionRange,stdout:'predicate\ntrue\nfalse\nfalse\npredicate\ntrue\n',source:`
fn nonempty(text:String)->bool{println!("predicate");text.len()>0usize}
fn main(){let a:Result<String,String>=Ok(String::from("text"));let b:Result<String,String>=Err(String::from("bad"));let c:Result<String,String>=Ok(String::from("text"));let d:Result<String,String>=Err(String::from("bad"));println!("{}",a.is_ok_and(nonempty));println!("{}",b.is_ok_and(nonempty));println!("{}",c.is_err_and(nonempty));println!("{}",d.is_err_and(nonempty));}
`},
 {name:'owned is_none_or keeps its existing shared nullary true branch',sourceVersionRange,stdout:'true\n',source:`
fn nonempty(text:String)->bool{println!("unused");text.len()>0usize}
fn main(){let value:Option<String>=None;println!("{}",value.is_none_or(nonempty));}
`},
 {name:'map_or evaluates owned receiver then eager default then selected mapper once',sourceVersionRange,stdout:'receiver\ndefault\nmapper\ntext\n',source:`
fn make()->Option<String>{println!("receiver");Some(String::from("text"))}
fn fallback()->String{println!("default");String::from("other")}
fn mapper(text:String)->String{println!("mapper");text}
fn main(){println!("{}",make().map_or(fallback(),mapper));}
`},
 {name:'map_or evaluates a mapper factory even when owned default is selected',sourceVersionRange,stdout:'default\nfactory\nother\n',source:`
fn fallback()->String{println!("default");String::from("other")}
fn mapper(text:String)->String{println!("unused");text}
fn factory()->fn(String)->String{println!("factory");mapper}
fn main(){let value:Option<String>=None;println!("{}",value.map_or(fallback(),factory()));}
`},
 {name:'map_or_else evaluates both factories in order and invokes only the selected mapper',sourceVersionRange,stdout:'fallback factory\nmapper factory\nmapper\ntext\n',source:`
fn fallback()->String{println!("unused");String::from("other")}
fn mapper(text:String)->String{println!("mapper");text}
fn fallback_factory()->fn()->String{println!("fallback factory");fallback}
fn mapper_factory()->fn(String)->String{println!("mapper factory");mapper}
fn main(){let value:Option<String>=Some(String::from("text"));println!("{}",value.map_or_else(fallback_factory(),mapper_factory()));}
`},
 {name:'Result map_or_else moves an owned error into its vector-producing fallback',sourceVersionRange,stdout:'fallback\n1\n',source:`
fn fallback(error:String)->Vec<u8>{println!("fallback");vec![7u8]}
fn mapper(text:String)->Vec<u8>{println!("unused");vec![7u8,9u8]}
fn main(){let value:Result<String,String>=Err(String::from("bad"));let result=value.map_or_else(fallback,mapper);println!("{}",result.len());}
`},
 {name:'an owned error does not change the Copy result of a typed eager map_or',sourceVersionRange,stdout:'7\n',source:`
fn identity(value:u8)->u8{value}
fn main(){let value:Result<u8,String>=Ok(7u8);println!("{}",value.map_or(9u8,identity));}
`},
 {name:'two explicit move callbacks independently copy the same scalar environment',sourceVersionRange,stdout:'true\ntext\ntrue\n',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let flag=true;let result=value.map_or_else(move||{println!("{}",flag);String::from("other")},move|text|{println!("{}",flag);text});println!("{}",result);println!("{}",flag);}
`},
 {name:'map_or preserves an owned nested Option result constructor layer',sourceVersionRange,stdout:'text\n',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let fallback:Option<String>=None;let result=value.map_or(fallback,|text|Some(text));println!("{}",result.unwrap_or(String::from("other")));}
`},
];
export const rustOwnedStandardSumPredicateRefusals=[
 {name:'a predicate receiver moves even when its callback is skipped',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn nonempty(text:String)->bool{text.len()>0usize}fn main(){let value:Option<String>=None;let selected=value.is_some_and(nonempty);println!("{}",value.is_some());}'},
 {name:'eager map_or default moves before the mapper expression reads it',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn mapper(text:String)->String{text}fn main(){let value:Option<String>=Some(String::from("text"));let fallback=String::from("other");let result=value.map_or(fallback,{println!("{}",fallback);mapper});}'},
 {name:'map_or_else receiver moves before the first callback expression',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn fallback()->String{String::from("other")}fn mapper(text:String)->String{text}fn main(){let value:Option<String>=Some(String::from("text"));let result=value.map_or_else({println!("{}",value.is_some());fallback},mapper);}'},
 {name:'both lazy callbacks must return the same complete actual type',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn fallback()->String{String::from("other")}fn wrong(text:String)->Vec<u8>{vec![7u8]}fn main(){let value:Option<String>=None;let result=value.map_or_else(fallback,wrong);}'},
 {name:'predicate callbacks cannot substitute truthy numeric results for bool',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn wrong(text:String)->u8{7u8}fn main(){let value:Option<String>=Some(String::from("text"));let result=value.is_some_and(wrong);}'},
 {name:'an owned predicate environment still needs FnOnce state transport',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn take(text:String)->usize{text.len()}fn main(){let value:Option<String>=Some(String::from("text"));let other=String::from("other");let result=value.is_some_and(move|text|take(other)>0usize);}'},
 {name:'an unresolved eager scalar needs its separate later-sibling inference transport',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn main(){let value:Option<String>=None;let result=value.map_or(0,|text|text.len());}'},
 {name:'an unresolved first callback needs its separate deferred owned-frame transport',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn main(){let value:Option<String>=None;let result=value.map_or_else(||0,|text|text.len());}'},
];
