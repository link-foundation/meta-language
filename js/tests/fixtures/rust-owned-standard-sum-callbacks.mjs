// Independent source literals and policy expectations; no observations.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustOwnedStandardSumCallbackDecisions=[
 {operation:'acceptRustOwnedStandardSumCallbackClosureCopy',arguments:[['owned'],'owned',0,0],expected:true},
 {operation:'acceptRustOwnedStandardSumCallbackClosureCopy',arguments:[['owned'],'copy',0,0],expected:true},
 {operation:'acceptRustOwnedStandardSumCallbackClosureCopy',arguments:[['owned'],'owned',1,0],expected:false},
 {operation:'acceptRustOwnedStandardSumCallbackClosureCopy',arguments:[['owned'],'owned',0,1],expected:false},
 {operation:'acceptRustOwnedStandardSumCallbackClosureCopy',arguments:[['borrowed'],'owned',0,0],expected:false},
 {operation:'acceptRustOwnedStandardSumCallbackClosureCopy',arguments:[['owned'],'',0,0],expected:false},
 {operation:'readRustOwnedStandardSumCallbackArity',arguments:['std::option::Option','unwrap_or_else'],expected:1},
 {operation:'readRustOwnedStandardSumCallbackArity',arguments:['std::option::Option','ok_or_else'],expected:1},
 {operation:'readRustOwnedStandardSumCallbackArity',arguments:['std::option::Option','map'],expected:1},
 {operation:'readRustOwnedStandardSumCallbackArity',arguments:['std::result::Result','map_err'],expected:1},
 // Retained original policy input. The subsequent owned bind profile admits
 // this operation; the earlier capture-free map batch returned -1.
 {operation:'readRustOwnedStandardSumCallbackArity',arguments:['std::result::Result','and_then'],expected:1,previousExpected:-1,previousProfile:'owned unwrap_or_else/ok_or_else/map/map_err source batch'},
 {operation:'readRustOwnedStandardSumCallbackVersionError',arguments:['std::option::Option','map',90],expected:''},
 {operation:'readRustOwnedStandardSumCallbackVersionError',arguments:['std::result::Result','map_err',99],expected:''},
 {operation:'readRustOwnedStandardSumCallbackVersionError',arguments:['std::option::Option','map',89],expected:'the owned standard sum callback source version is uncovered'},
 {operation:'readRustOwnedStandardSumCallbackVersionError',arguments:['std::result::Result','map_err',100],expected:'the owned standard sum callback source version is uncovered'},
 {operation:'readRustOwnedStandardSumCallbackParameter',arguments:['std::option::Option','unwrap_or_else'],expected:-1},
 {operation:'readRustOwnedStandardSumCallbackParameter',arguments:['std::option::Option','map'],expected:0},
 {operation:'readRustOwnedStandardSumCallbackParameter',arguments:['std::result::Result','unwrap_or_else'],expected:1},
 {operation:'readRustOwnedStandardSumCallbackParameter',arguments:['std::result::Result','map'],expected:0},
 {operation:'readRustOwnedStandardSumCallbackParameter',arguments:['std::result::Result','map_err'],expected:1},
 {operation:'readRustOwnedStandardSumCallbackResult',arguments:['std::option::Option','unwrap_or_else'],expected:'payload'},
 {operation:'readRustOwnedStandardSumCallbackResult',arguments:['std::option::Option','ok_or_else'],expected:'std::result::Result'},
 {operation:'planRustOwnedStandardSumCallbackArguments',arguments:['std::option::Option','ok_or_else'],expected:[0,-1]},
 {operation:'planRustOwnedStandardSumCallbackArguments',arguments:['std::option::Option','map'],expected:[-1]},
 {operation:'planRustOwnedStandardSumCallbackArguments',arguments:['std::result::Result','map'],expected:[-1,1]},
 {operation:'planRustOwnedStandardSumCallbackArguments',arguments:['std::result::Result','map_err'],expected:[0,-1]},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::option::Option','Some','unwrap_or_else'],expected:['','payload']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::option::Option','None','unwrap_or_else'],expected:['','invoke']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::option::Option','Some','ok_or_else'],expected:['Ok','payload']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::option::Option','None','ok_or_else'],expected:['Err','invoke']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::option::Option','Some','map'],expected:['Some','invoke']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::option::Option','None','map'],expected:['None','empty']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::result::Result','Ok','unwrap_or_else'],expected:['','payload']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::result::Result','Err','unwrap_or_else'],expected:['','invoke']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::result::Result','Ok','map'],expected:['Ok','invoke']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::result::Result','Err','map'],expected:['Err','payload']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::result::Result','Ok','map_err'],expected:['Ok','payload']},
 {operation:'planRustOwnedStandardSumCallbackBranch',arguments:['std::result::Result','Err','map_err'],expected:['Err','invoke']},
 {operation:'readRustOwnedStandardSumCallbackClosureHeaderError',arguments:[true,['value'],['owned'],'owned'],expected:''},
 {operation:'readRustOwnedStandardSumCallbackClosureHeaderError',arguments:[true,['value'],['owned'],'copy'],expected:''},
 {operation:'readRustOwnedStandardSumCallbackClosureHeaderError',arguments:[true,['value'],['borrowed'],'owned'],expected:'the callback parameter needs its actual direct Copy or supported owned source domain'},
 {operation:'readRustOwnedStandardSumCallbackClosureHeaderError',arguments:[true,[],[],''],expected:'the callback result needs its actual direct Copy or supported owned source domain'},
 {operation:'readRustOwnedStandardSumCallbackClosureHeaderError',arguments:[true,['same','same'],['copy','owned'],'copy'],expected:'callback parameters need distinct source bindings'},
 {operation:'readRustOwnedStandardSumCallbackCaptureError',arguments:[0,0],expected:''},
 {operation:'readRustOwnedStandardSumCallbackCaptureError',arguments:[1,0],expected:'owned-domain callbacks currently need actual capture-free source closures'},
 {operation:'readRustOwnedStandardSumCallbackCaptureError',arguments:[0,1],expected:'owned-domain callbacks currently need actual capture-free source closures'},
 {operation:'readRustOwnedStandardSumCallbackSourceError',arguments:['function',1,1,0,true,'owned'],expected:''},
 {operation:'readRustOwnedStandardSumCallbackSourceError',arguments:['function',1,1,0,true,'copy'],expected:''},
 {operation:'readRustOwnedStandardSumCallbackSourceError',arguments:['function',1,1,0,false,'owned'],expected:'the callback needs its actual Copy callable and complete movable parameter/result contracts'},
 {operation:'readRustOwnedStandardSumCallbackSourceError',arguments:['function',0,1,0,true,'owned'],expected:'the callback needs its actual Copy callable and complete movable parameter/result contracts'},
];
export const rustOwnedStandardSumCallbackOriginals=[
 {name:'new owned callback profile retains the former eager-batch closure boundary input',sourceVersionRange,stdout:'text\n',source:'fn main(){let value:Option<String>=Some(String::from("text"));let text=value.unwrap_or_else(||String::from("other"));println!("{}",text);}'},
 {name:'owned None invokes its capture-free fallback only when selected',sourceVersionRange,stdout:'fallback\nother\n',source:`
fn main(){let value:Option<String>=None;let text=value.unwrap_or_else(||{println!("fallback");String::from("other")});println!("{}",text);}
`},
 {name:'named owned Result fallback moves its actual error into its result',sourceVersionRange,stdout:'bad\n',source:`
fn fallback(error:String)->String{error}
fn main(){let value:Result<String,String>=Err(String::from("bad"));println!("{}",value.unwrap_or_else(fallback));}
`},
 {name:'named owning callback factory runs eagerly while fallback invocation is skipped',sourceVersionRange,stdout:'factory\ntext\n',source:`
fn fallback()->String{println!("fallback");String::from("other")}
fn factory()->fn()->String{println!("factory");fallback}
fn main(){let value:Option<String>=Some(String::from("text"));println!("{}",value.unwrap_or_else(factory()));}
`},
 {name:'owned Option map accepts a named Copy callable with owned input and Copy result',sourceVersionRange,stdout:'4\n',source:`
fn length(text:String)->usize{text.len()}
fn main(){let value:Option<String>=Some(String::from("text"));let result=value.map(length);println!("{}",result.unwrap_or(0usize));}
`},
 {name:'capture-free identity closure moves its owned parameter into an owned result',sourceVersionRange,stdout:'text\n',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let result=value.map(|text|text);println!("{}",result.unwrap_or(String::from("other")));}
`},
 {name:'owned Result map preserves its independent failure owner',sourceVersionRange,stdout:'bad\n',source:`
fn identity(text:String)->String{println!("mapper");text}
fn main(){let value:Result<String,String>=Err(String::from("bad"));let mapped=value.map(identity);let text=match mapped{Ok(text)=>text,Err(error)=>error};println!("{}",text);}
`},
 {name:'owned map_err moves only the selected error through its named callable',sourceVersionRange,stdout:'mapper\n2\n',source:`
fn code(error:String)->Vec<u8>{println!("mapper");vec![7u8,9u8]}
fn main(){let value:Result<String,String>=Err(String::from("bad"));let mapped=value.map_err(code);let length=match mapped{Ok(text)=>text.len(),Err(values)=>values.len()};println!("{}",length);}
`},
 {name:'ok_or_else invokes its capture-free error producer only for None',sourceVersionRange,stdout:'error\ntrue\nfalse\n',source:`
fn main(){let some:Option<String>=Some(String::from("text"));let none:Option<String>=None;let a=some.ok_or_else(||{println!("unused");String::from("bad")});let b=none.ok_or_else(||{println!("error");String::from("bad")});println!("{}",a.is_ok());println!("{}",b.is_ok());}
`},
 {name:'owned vector parameter can be consumed by a capture-free scalar mapper',sourceVersionRange,stdout:'2\n',source:`
fn main(){let value:Option<Vec<u8>>=Some(vec![7u8,9u8]);let result=value.map(|values|values.len());println!("{}",result.unwrap_or(0usize));}
`},
 {name:'mapped nested owned sum preserves its distinct constructor layer',sourceVersionRange,stdout:'text\n',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let nested=value.map(|text|Some(text));let fallback:Option<String>=None;let inner=nested.unwrap_or(fallback);println!("{}",inner.unwrap_or(String::from("other")));}
`},
];
export const rustOwnedStandardSumCallbackRefusals=[
 {name:'an owned capture needs the separate FnOnce environment model',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn main(){let value:Option<String>=None;let fallback=String::from("other");let text=value.unwrap_or_else(move||fallback);println!("{}",text);}'},
 {name:'a Copy capture is outside this capture-free owned callback batch',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn main(){let value:Option<String>=Some(String::from("text"));let extra=7u8;let result=value.map(|text|{println!("{}",extra);text});}'},
 {name:'a callback cannot consume its owned parameter twice',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn take(text:String)->usize{text.len()}fn main(){let value:Option<String>=Some(String::from("text"));let result=value.map(|text|{let first=take(text);take(text)});}'},
 {name:'self moves before reading the callback factory expression',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn fallback()->String{String::from("other")}fn main(){let value:Option<String>=Some(String::from("text"));let text=value.unwrap_or_else({println!("{}",value.is_some());fallback});}'},
 {name:'named callback input must keep its actual String owner',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn wrong(values:Vec<u8>)->usize{values.len()}fn main(){let value:Option<String>=Some(String::from("text"));let result=value.map(wrong);}'},
 {name:'fallback callable must return the complete actual payload type',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn wrong()->Vec<u8>{vec![7u8]}fn main(){let value:Option<String>=None;let text=value.unwrap_or_else(wrong);}'},
 {name:'a consuming callback method cannot move through a borrowed root',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn identity(text:String)->String{text}fn main(){let value:Option<String>=Some(String::from("text"));let result=(&value).map(identity);}'},
];
