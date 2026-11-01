// Independently authored owned bind/fallback contracts; no execution evidence.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustOwnedStandardSumBindDecisions=[
 {operation:'readRustOwnedStandardSumCallbackArity',arguments:['std::option::Option','and_then'],expected:1},
 {operation:'readRustOwnedStandardSumCallbackArity',arguments:['std::option::Option','or_else'],expected:1},
 {operation:'readRustOwnedStandardSumCallbackArity',arguments:['std::result::Result','and_then'],expected:1},
 {operation:'readRustOwnedStandardSumCallbackArity',arguments:['std::result::Result','or_else'],expected:1},
 {operation:'readRustOwnedStandardSumCallbackParameter',arguments:['std::option::Option','and_then'],expected:0},
 {operation:'readRustOwnedStandardSumCallbackParameter',arguments:['std::option::Option','or_else'],expected:-1},
 {operation:'readRustOwnedStandardSumCallbackParameter',arguments:['std::result::Result','and_then'],expected:0},
 {operation:'readRustOwnedStandardSumCallbackParameter',arguments:['std::result::Result','or_else'],expected:1},
 {operation:'readRustOwnedStandardSumCallbackResult',arguments:['std::option::Option','and_then'],expected:'bound sum'},
 {operation:'readRustOwnedStandardSumCallbackResult',arguments:['std::result::Result','or_else'],expected:'bound sum'},
 {operation:'planRustOwnedStandardSumCallbackArguments',arguments:['std::option::Option','and_then'],expected:[]},
 {operation:'planRustOwnedStandardSumCallbackArguments',arguments:['std::result::Result','or_else'],expected:[]},
 {operation:'readRustOwnedStandardSumCallbackRetainedArgument',arguments:['std::option::Option','and_then'],expected:-1},
 {operation:'readRustOwnedStandardSumCallbackRetainedArgument',arguments:['std::option::Option','or_else'],expected:0},
 {operation:'readRustOwnedStandardSumCallbackRetainedArgument',arguments:['std::result::Result','and_then'],expected:1},
 {operation:'readRustOwnedStandardSumCallbackRetainedArgument',arguments:['std::result::Result','or_else'],expected:0},
 {operation:'readRustOwnedStandardSumCallbackBoundResultError',arguments:['std::option::Option','std::option::Option',0,true],expected:''},
 {operation:'readRustOwnedStandardSumCallbackBoundResultError',arguments:['std::result::Result','std::result::Result',0,true],expected:''},
 {operation:'readRustOwnedStandardSumCallbackBoundResultError',arguments:['std::option::Option','std::result::Result',0,true],expected:'the callback must return its actual direct same-family sum and preserve every retained argument identity'},
 {operation:'readRustOwnedStandardSumCallbackBoundResultError',arguments:['std::result::Result','std::result::Result',0,false],expected:'the callback must return its actual direct same-family sum and preserve every retained argument identity'},
 {operation:'readRustOwnedStandardSumCallbackBoundResultError',arguments:['std::option::Option','std::option::Option',1,true],expected:'the callback must return its actual direct same-family sum and preserve every retained argument identity'},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::option::Option','Some','and_then'],expected:['','invoke']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::option::Option','None','and_then'],expected:['None','empty']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::option::Option','Some','or_else'],expected:['Some','payload']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::option::Option','None','or_else'],expected:['','invoke']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::result::Result','Ok','and_then'],expected:['','invoke']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::result::Result','Err','and_then'],expected:['Err','payload']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::result::Result','Ok','or_else'],expected:['Ok','payload']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::result::Result','Err','or_else'],expected:['','invoke']},
];
export const rustOwnedStandardSumBindOriginals=[
 {name:'owned Option and_then invokes a named producer once for Some',sourceVersionRange,stdout:'chain\n2\n',source:`
fn chain(text:String)->Option<Vec<u8>>{println!("chain");Some(vec![7u8,9u8])}
fn main(){let value:Option<String>=Some(String::from("text"));let result=value.and_then(chain);let length=match result{Some(values)=>values.len(),None=>0usize};println!("{}",length);}
`},
 {name:'owned Option and_then retains a nullary None and skips its producer',sourceVersionRange,stdout:'false\n',source:`
fn chain(text:String)->Option<Vec<u8>>{println!("unused");Some(vec![7u8])}
fn main(){let value:Option<String>=None;let result=value.and_then(chain);println!("{}",result.is_some());}
`},
 {name:'owned Option or_else keeps T and skips its callback for Some',sourceVersionRange,stdout:'text\n',source:`
fn fallback()->Option<String>{println!("unused");Some(String::from("other"))}
fn main(){let value:Option<String>=Some(String::from("text"));let result=value.or_else(fallback);println!("{}",result.unwrap_or(String::from("none")));}
`},
 {name:'owned Option or_else contextual None keeps its actual String argument',sourceVersionRange,stdout:'fallback\nfalse\n',source:`
fn main(){let value:Option<String>=None;let result=value.or_else(||{println!("fallback");None});println!("{}",result.is_some());}
`},
 {name:'owned Result and_then changes success while retaining the exact error owner',sourceVersionRange,stdout:'chain\n2\n',source:`
fn chain(text:String)->Result<Vec<u8>,String>{println!("chain");Ok(vec![7u8,9u8])}
fn main(){let value:Result<String,String>=Ok(String::from("text"));let result=value.and_then(chain);let length=match result{Ok(values)=>values.len(),Err(error)=>error.len()};println!("{}",length);}
`},
 {name:'owned Result and_then forwards an error without invoking its callback',sourceVersionRange,stdout:'bad\n',source:`
fn chain(text:String)->Result<Vec<u8>,String>{println!("unused");Ok(vec![7u8])}
fn main(){let value:Result<String,String>=Err(String::from("bad"));let result=value.and_then(chain);let message=match result{Ok(_)=>String::from("good"),Err(error)=>error};println!("{}",message);}
`},
 {name:'owned Result or_else changes error type while retaining the exact success owner',sourceVersionRange,stdout:'recover\nbad\n',source:`
fn recover(error:String)->Result<String,u8>{println!("recover");Ok(error)}
fn main(){let value:Result<String,String>=Err(String::from("bad"));let result=value.or_else(recover);let message=match result{Ok(text)=>text,Err(_)=>String::from("none")};println!("{}",message);}
`},
 {name:'owned Result or_else forwards success without invoking the callback',sourceVersionRange,stdout:'text\n',source:`
fn recover(error:String)->Result<String,u8>{println!("unused");Ok(error)}
fn main(){let value:Result<String,String>=Ok(String::from("text"));let result=value.or_else(recover);let message=match result{Ok(text)=>text,Err(_)=>String::from("none")};println!("{}",message);}
`},
 {name:'and_then returns its actual callback sum without an extra constructor layer',sourceVersionRange,stdout:'7\ntext\n7\n',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let count=7u8;let result=value.and_then(move|text|{println!("{}",count);Some(text)});println!("{}",result.unwrap_or(String::from("other")));println!("{}",count);}
`},
];
export const rustOwnedStandardSumBindRefusals=[
 {name:'Option and_then requires an actual Option callback result family',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn wrong(text:String)->Result<String,u8>{Ok(text)}fn main(){let value:Option<String>=Some(String::from("text"));let result=value.and_then(wrong);}'},
 {name:'Result and_then cannot change the retained complete error identity',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn wrong(text:String)->Result<String,u8>{Ok(text)}fn main(){let value:Result<String,String>=Ok(String::from("text"));let result=value.and_then(wrong);}'},
 {name:'Result or_else cannot change the retained complete success identity',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn wrong(error:String)->Result<Vec<u8>,u8>{Ok(vec![7u8])}fn main(){let value:Result<String,String>=Err(String::from("bad"));let result=value.or_else(wrong);}'},
 {name:'Option or_else cannot change its payload argument',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn wrong()->Option<Vec<u8>>{Some(vec![7u8])}fn main(){let value:Option<String>=None;let result=value.or_else(wrong);}'},
 {name:'owned bind still refuses an actual owned FnOnce capture',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn main(){let value:Option<String>=Some(String::from("text"));let fallback=String::from("other");let result=value.and_then(move|text|Some(fallback));}'},
 {name:'owned bind consumes its receiver even when its callback is skipped',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn chain(text:String)->Option<String>{Some(text)}fn main(){let value:Option<String>=None;let result=value.and_then(chain);println!("{}",value.is_some());}'},
];
