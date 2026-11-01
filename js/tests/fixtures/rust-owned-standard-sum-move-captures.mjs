// Source-authored extension inputs, not observed outcomes. Old bare capture
// and owned FnOnce boundary inputs remain in the callback batch unchanged.
const sourceVersionRange='>=1.90.0 <1.100.0';
const bindingError='a moved callback capture needs its actual complete direct Copy value without borrowed-place or loan provenance';
export const rustOwnedStandardSumMoveCaptureDecisions=[
 {operation:'readRustOwnedStandardSumCallbackMoveCaptureError',arguments:[false,[],[],[],[],[],0],expected:''},
 {operation:'readRustOwnedStandardSumCallbackMoveCaptureError',arguments:[true,['flag'],['copy'],['actual bool'],[0],[false],0],expected:''},
 {operation:'readRustOwnedStandardSumCallbackMoveCaptureError',arguments:[false,['flag'],['copy'],['actual bool'],[0],[false],0],expected:'captured owned-domain callbacks need an explicit move source closure'},
 {operation:'readRustOwnedStandardSumCallbackMoveCaptureError',arguments:[true,['text'],['owned'],['actual String'],[0],[false],0],expected:bindingError},
 {operation:'readRustOwnedStandardSumCallbackMoveCaptureError',arguments:[true,['flag'],['copy'],[''],[0],[false],0],expected:bindingError},
 {operation:'readRustOwnedStandardSumCallbackMoveCaptureError',arguments:[true,['flag'],['copy'],['actual bool'],[1],[false],0],expected:bindingError},
 {operation:'readRustOwnedStandardSumCallbackMoveCaptureError',arguments:[true,['flag'],['copy'],['actual bool'],[0],[true],0],expected:bindingError},
 {operation:'readRustOwnedStandardSumCallbackMoveCaptureError',arguments:[true,['flag'],['copy'],['actual bool'],[0],[false],1],expected:'mutable callback captures need their actual environment state interpretation'},
 {operation:'readRustOwnedStandardSumCallbackMoveCaptureError',arguments:[true,['flag'],[],['actual bool'],[0],[false],0],expected:'the complete copied capture environment needs every actual source binding'},
 {operation:'readRustOwnedStandardSumCallbackMoveCaptureError',arguments:[true,['flag','flag'],['copy','copy'],['actual bool','actual bool'],[0,0],[false,false],0],expected:'copied callback captures need distinct actual lexical bindings'},
 {operation:'acceptRustOwnedStandardSumCallbackMoveClosureCopy',arguments:[['owned'],'owned',true,['flag'],['copy'],['actual bool'],[0],[false],0],expected:true},
 {operation:'acceptRustOwnedStandardSumCallbackMoveClosureCopy',arguments:[['owned'],'copy',true,['flag'],['copy'],['actual bool'],[0],[false],0],expected:true},
 {operation:'acceptRustOwnedStandardSumCallbackMoveClosureCopy',arguments:[['owned'],'owned',false,[],[],[],[],[],0],expected:true},
 {operation:'acceptRustOwnedStandardSumCallbackMoveClosureCopy',arguments:[['owned'],'owned',false,['flag'],['copy'],['actual bool'],[0],[false],0],expected:false},
 {operation:'acceptRustOwnedStandardSumCallbackMoveClosureCopy',arguments:[['owned'],'owned',true,['text'],['owned'],['actual String'],[0],[false],0],expected:false},
 {operation:'acceptRustOwnedStandardSumCallbackMoveClosureCopy',arguments:[['owned'],'owned',true,['flag'],['copy'],[''],[0],[false],0],expected:false},
 {operation:'readRustOwnedStandardSumCallbackImmediateCaptureError',arguments:[0,'name',false,false,0],expected:''},
 {operation:'readRustOwnedStandardSumCallbackImmediateCaptureError',arguments:[1,'lambda',true,true,1],expected:''},
 {operation:'readRustOwnedStandardSumCallbackImmediateCaptureError',arguments:[1,'name',true,true,1],expected:'a copied callback environment needs its actual explicit move Lambda source and span identity'},
 {operation:'readRustOwnedStandardSumCallbackImmediateCaptureError',arguments:[1,'lambda',false,true,1],expected:'a copied callback environment needs its actual explicit move Lambda source and span identity'},
 {operation:'readRustOwnedStandardSumCallbackImmediateCaptureError',arguments:[1,'lambda',true,false,1],expected:'a copied callback environment needs its actual explicit move Lambda source and span identity'},
 {operation:'readRustOwnedStandardSumCallbackImmediateCaptureError',arguments:[1,'lambda',true,true,2],expected:'a copied callback environment needs its actual explicit move Lambda source and span identity'},
];
export const rustOwnedStandardSumMoveCaptureOriginals=[
 {name:'actual derived nominal Copy evidence admits a complete named capture',sourceVersionRange,stdout:'7\ntext\n7\n',source:`
#[derive(Clone,Copy)]struct Tag{number:u8}
fn main(){let value:Option<String>=Some(String::from("text"));let tag:Tag=Tag{number:7u8};let result=value.map(move|text|{println!("{}",tag.number);text});println!("{}",result.unwrap_or(String::from("other")));println!("{}",tag.number);}
`},
 {name:'explicit move copies a scalar environment independently of owned mapper domains',sourceVersionRange,stdout:'7\ntext\n7\n',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let count=7u8;let result=value.map(move|text|{println!("{}",count);text});println!("{}",result.unwrap_or(String::from("other")));println!("{}",count);}
`},
 {name:'explicit move fallback copies a scalar capture but invokes only for None',sourceVersionRange,stdout:'7\nother\n7\n',source:`
fn main(){let value:Option<String>=None;let count=7u8;let text=value.unwrap_or_else(move||{println!("{}",count);String::from("other")});println!("{}",text);println!("{}",count);}
`},
 {name:'the Some branch skips a copied fallback environment invocation',sourceVersionRange,stdout:'text\n7\n',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let count=7u8;let text=value.unwrap_or_else(move||{println!("{}",count);String::from("other")});println!("{}",text);println!("{}",count);}
`},
 {name:'map_err copies a Boolean environment while transferring its owned error',sourceVersionRange,stdout:'true\nbad\ntrue\n',source:`
fn main(){let value:Result<String,String>=Err(String::from("bad"));let flag=true;let result=value.map_err(move|error|{println!("{}",flag);error});let text=match result{Ok(text)=>text,Err(error)=>error};println!("{}",text);println!("{}",flag);}
`},
 {name:'tuple capture retains its complete immutable product identity',sourceVersionRange,stdout:'7\ntext\ntrue\n',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let pair:(u8,bool)=(7u8,true);let result=value.map(move|text|{println!("{}",pair.0);text});println!("{}",result.unwrap_or(String::from("other")));println!("{}",pair.1);}
`},
 {name:'a canonical Copy sum capture remains distinct from owned input and output sums',sourceVersionRange,stdout:'true\ntext\ntrue\n',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let flag:Option<u8>=Some(7u8);let result=value.map(move|text|{println!("{}",flag.is_some());text});println!("{}",result.unwrap_or(String::from("other")));println!("{}",flag.is_some());}
`},
 {name:'actual lexical shadow selects the inner capture while preserving the outer binding',sourceVersionRange,stdout:'9\ntext\n7\n',source:`
fn main(){let count=7u8;let value:Option<String>=Some(String::from("text"));let result={let count=9u8;value.map(move|text|{println!("{}",count);text})};println!("{}",result.unwrap_or(String::from("other")));println!("{}",count);}
`},
];
export const rustOwnedStandardSumMoveCaptureRefusals=[
 {name:'implicit shared capture remains outside the explicit move environment profile',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn main(){let value:Option<String>=Some(String::from("text"));let count=7u8;let result=value.map(|text|{println!("{}",count);text});}'},
 {name:'a borrowed Copy scalar capture retains an unmodeled escaping loan',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn main(){let value:Option<String>=Some(String::from("text"));let count=7u8;let borrowed=&count;let result=value.map(move|text|{println!("{}",*borrowed);text});}'},
 {name:'an owned String environment still needs the separate FnOnce move model',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn main(){let value:Option<String>=Some(String::from("text"));let fallback=String::from("other");let result=value.map(move|text|fallback);}'},
 {name:'a captured environment cannot coerce to a function pointer',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn main(){let count=7u8;let callback:fn(String)->String=move|text|{println!("{}",count);text};}'},
 {name:'a borrowed owned field cannot be copied into a move closure environment',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn main(){let value:Option<String>=Some(String::from("text"));let borrowed=&value;let input:Option<String>=None;let result=input.unwrap_or_else(move||{println!("{}",borrowed.is_some());String::from("other")});}'},
];
