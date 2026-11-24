// Independent Copy-to-owned callback source inputs. Selected source context
// never supplies admission; complete callback parsing/checking remains required.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustStandardSumCopyOwnedCallbackDecisions=[
 {operation:'readRustOwnedStandardSumCallbackSelectionTokenLimit',arguments:[],expected:4096},
 {operation:'planRustOwnedStandardSumCallbackSelection',arguments:[['(','mapper',')'],['punctuation','identifier','punctuation']],expected:[1,1,2]},
 {operation:'planRustOwnedStandardSumCallbackSelection',arguments:[['(','factory','(',')',')'],['punctuation','identifier','punctuation','punctuation','punctuation']],expected:[2,1,2]},
 {operation:'planRustOwnedStandardSumCallbackSelection',arguments:[['(','module','::','mapper',')'],['punctuation','identifier','punctuation','identifier','punctuation']],expected:[1,1,4]},
 {operation:'planRustOwnedStandardSumCallbackSelection',arguments:[['(','|','value',':','u8','|','->','String','{'],['punctuation','punctuation','identifier','punctuation','identifier','punctuation','punctuation','identifier','punctuation']],expected:[3,7,8]},
 {operation:'planRustOwnedStandardSumCallbackSelection',arguments:[['(','move','||','->','std','::','string','::','String','{'],['punctuation','keyword','punctuation','punctuation','identifier','punctuation','identifier','punctuation','identifier','punctuation']],expected:[3,4,9]},
 {operation:'planRustOwnedStandardSumCallbackSelection',arguments:[['(','|','value','|','value',')'],['punctuation','punctuation','identifier','punctuation','identifier','punctuation']],expected:[]},
 {operation:'planRustOwnedStandardSumCallbackSelection',arguments:[['(','move','mapper',')'],['punctuation','keyword','identifier','punctuation']],expected:[]},
 {operation:'planRustOwnedStandardSumCallbackSelection',arguments:[['(','object','.','mapper',')'],['punctuation','identifier','punctuation','identifier','punctuation']],expected:[]},
 {operation:'planRustOwnedStandardSumCallbackSelection',arguments:[['(','||','->','&','String','{'],['punctuation','punctuation','punctuation','punctuation','identifier','punctuation']],expected:[]},
 {operation:'acceptRustOwnedStandardSumCallbackWrittenSelection',arguments:['library','std::string::String',false,false],expected:true},
 {operation:'acceptRustOwnedStandardSumCallbackWrittenSelection',arguments:['library','std::vec::Vec',false,false],expected:true},
 {operation:'acceptRustOwnedStandardSumCallbackWrittenSelection',arguments:['declaration','String',false,false],expected:false},
 {operation:'acceptRustOwnedStandardSumCallbackWrittenSelection',arguments:['library','std::string::String',true,false],expected:false},
 {operation:'acceptRustOwnedStandardSumCallbackWrittenSelection',arguments:['library','std::string::String',false,true],expected:false},
 {operation:'acceptRustOwnedStandardSumCallbackWrittenSelection',arguments:['primitive','u8',false,false],expected:false},
];
export const rustStandardSumCopyOwnedCallbackOriginals=[
 {name:'Copy Option map introduces an owned String and preserves the original receiver',sourceVersionRange,stdout:'7\ntext\ntrue\n',source:`
fn text(value:u8)->String{println!("{}",value);String::from("text")}
fn main(){let value:Option<u8>=Some(7u8);let result=value.map(text);println!("{}",result.unwrap_or(String::from("other")));println!("{}",value.is_some());}
`},
 {name:'Copy None map produces an owned None without invoking its mapper',sourceVersionRange,stdout:'other\n',source:`
fn text(value:u8)->String{println!("unused");String::from("text")}
fn main(){let value:Option<u8>=None;let result=value.map(text);println!("{}",result.unwrap_or(String::from("other")));}
`},
 {name:'Copy Result map retains its complete Copy error owner',sourceVersionRange,stdout:'text\ntrue\n',source:`
fn text(value:u8)->String{String::from("text")}
fn main(){let value:Result<u8,u8>=Ok(7u8);let result=value.map(text);println!("{}",result.unwrap_or(String::from("other")));println!("{}",value.is_ok());}
`},
 {name:'Copy Result map_err introduces an owned error and preserves the original receiver',sourceVersionRange,stdout:'9\nbad\ntrue\n',source:`
fn error(value:u8)->String{println!("{}",value);String::from("bad")}
fn main(){let value:Result<u8,u8>=Err(9u8);let result=value.map_err(error);println!("{}",result.err().unwrap_or(String::from("other")));println!("{}",value.is_err());}
`},
 {name:'Copy Option ok_or_else introduces its actual owning error contract',sourceVersionRange,stdout:'error\nbad\n',source:`
fn error()->String{println!("error");String::from("bad")}
fn main(){let value:Option<u8>=None;let result=value.ok_or_else(error);println!("{}",result.err().unwrap_or(String::from("other")));}
`},
 {name:'Copy Result and_then accepts an actual owning same-family result',sourceVersionRange,stdout:'text\n',source:`
fn chain(value:u8)->Result<String,u8>{Ok(String::from("text"))}
fn main(){let value:Result<u8,u8>=Ok(7u8);let result=value.and_then(chain);println!("{}",result.unwrap_or(String::from("other")));}
`},
 {name:'Copy Result or_else retains success while changing its error to String',sourceVersionRange,stdout:'bad\n',source:`
fn recover(value:u8)->Result<u8,String>{Err(String::from("bad"))}
fn main(){let value:Result<u8,u8>=Err(9u8);let result=value.or_else(recover);println!("{}",result.err().unwrap_or(String::from("other")));}
`},
 {name:'an explicitly typed owned lambda result is admitted through its full header',sourceVersionRange,stdout:'7\ntext\n',source:`
fn main(){let value:Option<u8>=Some(7u8);let result=value.map(|number:u8|->String{println!("{}",number);String::from("text")});println!("{}",result.unwrap_or(String::from("other")));}
`},
 {name:'an explicitly typed move lambda copies its environment independently of its owned result',sourceVersionRange,stdout:'9\ntext\n9\n',source:`
fn main(){let value:Option<u8>=Some(7u8);let count=9u8;let result=value.map(move|number:u8|->String{println!("{}",count);String::from("text")});println!("{}",result.unwrap_or(String::from("other")));println!("{}",count);}
`},
 {name:'an actual callback factory evaluates after its Copy receiver exactly once',sourceVersionRange,stdout:'receiver\nfactory\nmapper\ntext\n',source:`
fn make()->Option<u8>{println!("receiver");Some(7u8)}
fn text(value:u8)->String{println!("mapper");String::from("text")}
fn factory()->fn(u8)->String{println!("factory");text}
fn main(){let result=make().map(factory());println!("{}",result.unwrap_or(String::from("other")));}
`},
 {name:'the existing named Copy-result map remains on its original reader',sourceVersionRange,stdout:'7\n',source:`
fn identity(value:u8)->u8{value}
fn main(){let value:Option<u8>=Some(7u8);println!("{}",value.map(identity).unwrap_or(9u8));}
`},
 {name:'an explicitly typed vector output is an actual owned callback domain',sourceVersionRange,stdout:'2\n',source:`
fn main(){let value:Option<u8>=Some(7u8);let result=value.map(|number:u8|->Vec<u8>{vec![number,9u8]});let values=result.unwrap_or(vec![1u8]);println!("{}",values.len());}
`},
 {name:'an imported function alias selects its actual owning result declaration',sourceVersionRange,stdout:'text\n',source:`
mod helpers{pub fn text(value:u8)->String{String::from("text")}}
use helpers::text as mapper;
fn main(){let value:Option<u8>=Some(7u8);let result=value.map(mapper);println!("{}",result.unwrap_or(String::from("other")));}
`},
 {name:'a local Copy callable shadows an owning function of the same spelling',sourceVersionRange,stdout:'7\n',source:`
fn mapper(value:u8)->String{String::from("unused")}
fn identity(value:u8)->u8{value}
fn main(){let mapper:fn(u8)->u8=identity;let value:Option<u8>=Some(7u8);println!("{}",value.map(mapper).unwrap_or(9u8));}
`},
];
export const rustStandardSumCopyOwnedCallbackRefusals=[
 {name:'a selected owning named callback must still match the complete Copy input type',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn wrong(values:Vec<u8>)->String{String::from("text")}fn main(){let value:Option<u8>=Some(7u8);let result=value.map(wrong);}'},
 {name:'an explicit owned result cannot disguise an actual owned FnOnce environment',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn main(){let value:Option<u8>=Some(7u8);let text=String::from("text");let result=value.map(move|number:u8|->String{text});}'},
 {name:'an explicit owned result cannot discard the loan carried by a captured reference',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn main(){let value:Option<u8>=Some(7u8);let count=9u8;let borrowed=&count;let result=value.map(move|number:u8|->String{println!("{}",*borrowed);String::from("text")});}'},
 {name:'an owning Result callback cannot change and_then retained error identity',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn wrong(value:u8)->Result<String,u16>{Ok(String::from("text"))}fn main(){let value:Result<u8,u8>=Ok(7u8);let result=value.and_then(wrong);}'},
 {name:'a named function call returning String is not a callback factory',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn text()->String{String::from("text")}fn main(){let value:Option<u8>=Some(7u8);let result=value.map(text());}'},
];
