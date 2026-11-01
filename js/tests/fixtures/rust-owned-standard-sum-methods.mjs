// Independent literals authored without execution. Runtime/compiler cells are
// future controls, not observations; native policy generation remains deferred.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustOwnedStandardSumMethodDecisions=[
 {operation:'readRustOwnedStandardSumMethodReceiverMode',arguments:['std::option::Option','ok_or','copy'],expected:'copy'},
 {operation:'readRustOwnedStandardSumMethodReceiverMode',arguments:['std::option::Option','ok_or','owned'],expected:'owned'},
 {operation:'readRustOwnedStandardSumMethodReceiverMode',arguments:['std::result::Result','ok','copy'],expected:''},
 {operation:'readRustOwnedStandardSumMethodReceiverMode',arguments:['std::option::Option','unwrap_or','copy'],expected:''},
 {operation:'acceptRustOwnedStandardSumMethodCallProduction',arguments:['owned',true,true,true,0,false],expected:true},
 {operation:'acceptRustOwnedStandardSumMethodCallProduction',arguments:['owned',true,false,true,0,false],expected:false},
 {operation:'acceptRustOwnedStandardSumMethodCallProduction',arguments:['owned',false,true,true,0,false],expected:false},
 {operation:'acceptRustOwnedStandardSumMethodCallProduction',arguments:['owned',true,true,false,0,false],expected:false},
 {operation:'acceptRustOwnedStandardSumMethodCallProduction',arguments:['owned',true,true,true,1,false],expected:false},
 {operation:'acceptRustOwnedStandardSumMethodCallProduction',arguments:['owned',true,true,true,0,true],expected:false},
 {operation:'acceptRustOwnedStandardSumMethodCallProduction',arguments:['copy',true,true,true,0,false],expected:false},
 {operation:'readRustOwnedStandardSumMethodArity',arguments:['std::option::Option','unwrap_or'],expected:1},
 {operation:'readRustOwnedStandardSumMethodArity',arguments:['std::option::Option','ok_or'],expected:1},
 {operation:'readRustOwnedStandardSumMethodArity',arguments:['std::result::Result','unwrap_or'],expected:1},
 {operation:'readRustOwnedStandardSumMethodArity',arguments:['std::result::Result','ok'],expected:0},
 {operation:'readRustOwnedStandardSumMethodArity',arguments:['std::result::Result','err'],expected:0},
 {operation:'readRustOwnedStandardSumMethodArity',arguments:['std::option::Option','unwrap_or_else'],expected:-1},
 {operation:'readRustOwnedStandardSumMethodArity',arguments:['user::Option','unwrap_or'],expected:-1},
 {operation:'readRustOwnedStandardSumMethodVersionError',arguments:['std::option::Option','unwrap_or',90],expected:''},
 {operation:'readRustOwnedStandardSumMethodVersionError',arguments:['std::result::Result','ok',99],expected:''},
 {operation:'readRustOwnedStandardSumMethodVersionError',arguments:['std::option::Option','unwrap_or',89],expected:'the consuming owned standard sum method source version is uncovered'},
 {operation:'readRustOwnedStandardSumMethodVersionError',arguments:['std::result::Result','err',100],expected:'the consuming owned standard sum method source version is uncovered'},
 {operation:'readRustOwnedStandardSumMethodResult',arguments:['std::option::Option','unwrap_or'],expected:'payload'},
 {operation:'readRustOwnedStandardSumMethodResult',arguments:['std::option::Option','ok_or'],expected:'std::result::Result'},
 {operation:'readRustOwnedStandardSumMethodResult',arguments:['std::result::Result','ok'],expected:'std::option::Option'},
 {operation:'readRustOwnedStandardSumMethodResult',arguments:['std::result::Result','err'],expected:'std::option::Option'},
 {operation:'planRustOwnedStandardSumMethodArguments',arguments:['std::option::Option','ok_or'],expected:[0,-1]},
 {operation:'planRustOwnedStandardSumMethodArguments',arguments:['std::result::Result','ok'],expected:[0]},
 {operation:'planRustOwnedStandardSumMethodArguments',arguments:['std::result::Result','err'],expected:[1]},
 {operation:'planRustOwnedStandardSumMethodBranch',arguments:['std::option::Option','Some','unwrap_or'],expected:['','payload']},
 {operation:'planRustOwnedStandardSumMethodBranch',arguments:['std::option::Option','None','unwrap_or'],expected:['','argument']},
 {operation:'planRustOwnedStandardSumMethodBranch',arguments:['std::result::Result','Ok','unwrap_or'],expected:['','payload']},
 {operation:'planRustOwnedStandardSumMethodBranch',arguments:['std::result::Result','Err','unwrap_or'],expected:['','argument']},
 {operation:'planRustOwnedStandardSumMethodBranch',arguments:['std::option::Option','Some','ok_or'],expected:['Ok','payload']},
 {operation:'planRustOwnedStandardSumMethodBranch',arguments:['std::option::Option','None','ok_or'],expected:['Err','argument']},
 {operation:'planRustOwnedStandardSumMethodBranch',arguments:['std::result::Result','Ok','ok'],expected:['Some','payload']},
 {operation:'planRustOwnedStandardSumMethodBranch',arguments:['std::result::Result','Err','ok'],expected:['None','empty']},
 {operation:'planRustOwnedStandardSumMethodBranch',arguments:['std::result::Result','Ok','err'],expected:['None','empty']},
 {operation:'planRustOwnedStandardSumMethodBranch',arguments:['std::result::Result','Err','err'],expected:['Some','payload']},
 {operation:'planRustOwnedStandardSumMethodBranch',arguments:['std::option::Option','Other','unwrap_or'],expected:[]},
 {operation:'readRustOwnedStandardSumMethodCaptureError',arguments:['owned',0,false,true,'old','new',true,true],expected:''},
 {operation:'readRustOwnedStandardSumMethodCaptureError',arguments:['owned',0,false,true,'','new',true,true],expected:''},
 {operation:'readRustOwnedStandardSumMethodCaptureError',arguments:['owned',0,false,true,'same','same',true,true],expected:'the consuming value needs its actual move into a fresh tracked owned temporary'},
 {operation:'readRustOwnedStandardSumMethodCaptureError',arguments:['owned',0,false,true,'','',true,true],expected:'the consuming value needs its actual move into a fresh tracked owned temporary'},
 {operation:'readRustOwnedStandardSumMethodCaptureError',arguments:['owned',0,true,true,'old','new',true,true],expected:'the consuming value needs its actual move into a fresh tracked owned temporary'},
 {operation:'readRustOwnedStandardSumMethodCaptureError',arguments:['owned',1,false,true,'old','new',true,true],expected:'the consuming value needs its actual move into a fresh tracked owned temporary'},
 {operation:'readRustOwnedStandardSumMethodCaptureError',arguments:['owned',0,false,false,'old','new',true,true],expected:'the consuming value needs its actual move into a fresh tracked owned temporary'},
 {operation:'readRustOwnedStandardSumMethodCaptureError',arguments:['owned',0,false,true,'old','new',false,true],expected:'the consuming value needs its actual move into a fresh tracked owned temporary'},
 {operation:'readRustOwnedStandardSumMethodCaptureError',arguments:['owned',0,false,true,'old','new',true,false],expected:'the consuming value needs its actual move into a fresh tracked owned temporary'},
 {operation:'acceptRustOwnedStandardSumMethodArgument',arguments:['copy'],expected:true},
 {operation:'acceptRustOwnedStandardSumMethodArgument',arguments:['owned'],expected:true},
 {operation:'acceptRustOwnedStandardSumMethodArgument',arguments:['reference'],expected:false},
 {operation:'acceptRustOwnedStandardSumMethodProduction',arguments:['owned',true,true],expected:true},
 {operation:'acceptRustOwnedStandardSumMethodProduction',arguments:['copy',true,true],expected:false},
 {operation:'acceptRustOwnedStandardSumMethodProduction',arguments:['owned',false,true],expected:false},
 {operation:'acceptRustOwnedStandardSumMethodProduction',arguments:['owned',true,false],expected:false},
];
export const rustOwnedStandardSumMethodOriginals=[
 {name:'Copy Some ok_or evaluates and moves an owned function-produced error eagerly',sourceVersionRange,stdout:'error\n7\ntrue\n',source:`
fn error()->String{println!("error");String::from("bad")}
fn main(){let value:Option<u8>=Some(7u8);let result=value.ok_or(error());let number=match result{Ok(number)=>number,Err(_)=>0u8};println!("{}",number);println!("{}",value.is_some());}
`},
 {name:'Copy None ok_or preserves the complete owned error type',sourceVersionRange,stdout:'bad\n',source:`
fn main(){let value:Option<u8>=None;let error=String::from("bad");let result=value.ok_or(error);let message=match result{Ok(_)=>String::from("good"),Err(message)=>message};println!("{}",message);}
`},
 {name:'owned Some unwrap_or evaluates its receiver then unused default exactly once',sourceVersionRange,stdout:'receiver\ndefault\ntext\n',source:`
fn make()->Option<String>{println!("receiver");Some(String::from("text"))}
fn fallback()->String{println!("default");String::from("other")}
fn main(){let text=make().unwrap_or(fallback());println!("{}",text);}
`},
 {name:'owned None unwrap_or transfers its eager fallback',sourceVersionRange,stdout:'other\n',source:`
fn main(){let value:Option<String>=None;let fallback=String::from("other");let text=value.unwrap_or(fallback);println!("{}",text);}
`},
 {name:'owned Result unwrap_or selects success or eager fallback without Debug',sourceVersionRange,stdout:'good\nother\n',source:`
fn main(){let good:Result<String,String>=Ok(String::from("good"));let bad:Result<String,String>=Err(String::from("bad"));println!("{}",good.unwrap_or(String::from("other")));println!("{}",bad.unwrap_or(String::from("other")));}
`},
 {name:'Copy success with an owned error remains Copy after unwrap_or',sourceVersionRange,stdout:'7\n9\n',source:`
fn main(){let good:Result<u8,String>=Ok(7u8);let bad:Result<u8,String>=Err(String::from("bad"));println!("{}",good.unwrap_or(9u8));println!("{}",bad.unwrap_or(9u8));}
`},
 {name:'owned vector payload and fallback transfer as distinct eager values',sourceVersionRange,stdout:'2\n1\n',source:`
fn main(){let some:Option<Vec<u8>>=Some(vec![7u8,8u8]);let none:Option<Vec<u8>>=None;let first=some.unwrap_or(vec![9u8]);let second=none.unwrap_or(vec![9u8]);println!("{}",first.len());println!("{}",second.len());}
`},
 {name:'owned Option ok_or preserves both String owners',sourceVersionRange,stdout:'text\nerror\n',source:`
fn reveal(value:Result<String,String>)->String{match value{Ok(text)=>text,Err(error)=>error}}
fn main(){let some:Option<String>=Some(String::from("text"));let none:Option<String>=None;println!("{}",reveal(some.ok_or(String::from("error"))));println!("{}",reveal(none.ok_or(String::from("error"))));}
`},
 {name:'owned Result ok and err retain independent constructor presence',sourceVersionRange,stdout:'true\nfalse\nfalse\ntrue\n',source:`
fn main(){let a:Result<String,String>=Ok(String::from("good"));let b:Result<String,String>=Err(String::from("bad"));let c:Result<String,String>=Ok(String::from("good"));let d:Result<String,String>=Err(String::from("bad"));println!("{}",a.ok().is_some());println!("{}",b.ok().is_some());println!("{}",c.err().is_some());println!("{}",d.err().is_some());}
`},
 {name:'Result ok may produce a Copy Option while discarding its owned error',sourceVersionRange,stdout:'true\nfalse\n',source:`
fn main(){let a:Result<u8,String>=Ok(7u8);let b:Result<u8,String>=Err(String::from("bad"));println!("{}",a.ok().is_some());println!("{}",b.ok().is_some());}
`},
 {name:'nested owned sum result remains a real fresh producer',sourceVersionRange,stdout:'text\n',source:`
fn main(){let value:Option<Option<String>>=Some(Some(String::from("text")));let fallback:Option<String>=None;let inner=value.unwrap_or(fallback);let text=inner.unwrap_or(String::from("other"));println!("{}",text);}
`},
];
export const rustOwnedStandardSumMethodRefusals=[
 {name:'Copy Some ok_or still moves its unselected owned error argument',sourceVersionRange,originalCompilerOutcome:'reject',source:`
fn main(){let value:Option<u8>=Some(7u8);let error=String::from("bad");let result=value.ok_or(error);println!("{}",error);}
`},
 {name:'Copy None ok_or moves its selected owned error argument',sourceVersionRange,originalCompilerOutcome:'reject',source:`
fn main(){let value:Option<u8>=None;let error=String::from("bad");let result=value.ok_or(error);println!("{}",error);}
`},
 {name:'self has moved before the unwrap_or fallback reads it',sourceVersionRange,originalCompilerOutcome:'reject',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let text=value.unwrap_or(match value{Some(text)=>text,None=>String::from("other")});println!("{}",text);}
`},
 {name:'unused owned fallback is moved even when Some is selected',sourceVersionRange,originalCompilerOutcome:'reject',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let fallback=String::from("other");let text=value.unwrap_or(fallback);println!("{}",fallback);}
`},
 {name:'owned receiver cannot be used after unwrap_or',sourceVersionRange,originalCompilerOutcome:'reject',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let text=value.unwrap_or(String::from("other"));println!("{}",value.is_some());}
`},
 {name:'consuming unwrap_or cannot move an owned sum through shared reference',sourceVersionRange,originalCompilerOutcome:'reject',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let text=(&value).unwrap_or(String::from("other"));println!("{}",text);}
`},
 {name:'ok_or receiver moves before its eager error expression',sourceVersionRange,originalCompilerOutcome:'reject',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let result=value.ok_or(match value{Some(text)=>text,None=>String::from("error")});}
`},
 {name:'ok_or moves its eager error even in the success case',sourceVersionRange,originalCompilerOutcome:'reject',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let error=String::from("error");let result=value.ok_or(error);println!("{}",error);}
`},
 {name:'Result ok consumes its actual owned root',sourceVersionRange,originalCompilerOutcome:'reject',source:`
fn main(){let value:Result<String,String>=Ok(String::from("text"));let result=value.ok();println!("{}",value.is_ok());}
`},
];
// Preserve the former refusal and its exact input. The callback consumer now
// supplies the previously missing delayed invocation and ownership semantics.
export const rustOwnedStandardSumMethodPreviousBoundaries=[{
 previousCollection:'rustOwnedStandardSumMethodRefusals',previousIndex:9,
 previousDiagnosticKinds:['syntax','type','unsupported'],
 previousTranslationExpectation:'refuse callback outside eager consuming method profile',
 row:{name:'owned callable fallback remains outside this eager method batch',sourceVersionRange,originalCompilerOutcome:'accept',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let text=value.unwrap_or_else(||String::from("other"));println!("{}",text);}
`},
}];
rustOwnedStandardSumMethodOriginals.push({...rustOwnedStandardSumMethodPreviousBoundaries[0].row,
 stdout:'text\n',singleCallOwners:[],
 originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',outcome:'accept',category:'valid Rust source'},
 expectationHistory:[{previousCollection:'rustOwnedStandardSumMethodRefusals',previousIndex:9,
 translationExpectation:'refuse callback outside eager consuming method profile'}]});
