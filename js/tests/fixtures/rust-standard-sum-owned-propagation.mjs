// Source-authored controls. Their compiler/runtime outcomes remain unobserved.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustStandardSumOwnedPropagationDecisions=[
 {operation:'acceptRustStandardSumOwnedQuestionContext',arguments:['std::option::Option','std::option::Option',true,'owned',0,'',''],expected:true},
 {operation:'acceptRustStandardSumOwnedQuestionContext',arguments:['std::result::Result','std::result::Result',true,'owned',0,'actual error String','actual error String'],expected:true},
 {operation:'acceptRustStandardSumOwnedQuestionContext',arguments:['std::result::Result','std::result::Result',true,'owned',0,'actual error u8','actual error u16'],expected:false},
 {operation:'acceptRustStandardSumOwnedQuestionContext',arguments:['std::option::Option','std::result::Result',true,'owned',0,'','actual error String'],expected:false},
 {operation:'acceptRustStandardSumOwnedQuestionContext',arguments:['std::option::Option','std::option::Option',false,'owned',0,'',''],expected:false},
 {operation:'acceptRustStandardSumOwnedQuestionContext',arguments:['std::option::Option','std::option::Option',true,'copy',0,'',''],expected:false},
 {operation:'acceptRustStandardSumOwnedQuestionContext',arguments:['std::option::Option','std::option::Option',true,'owned',1,'',''],expected:false},
 {operation:'acceptRustStandardSumOwnedQuestionContext',arguments:['std::option::Option','std::option::Option',true,'',0,'',''],expected:false},
 {operation:'acceptRustStandardSumOwnedQuestionContext',arguments:['std::result::Result','std::result::Result',true,'owned',0,'',''],expected:false},
 {operation:'acceptRustStandardSumOwnedQuestionContext',arguments:['local::Option','local::Option',true,'owned',0,'',''],expected:false},
 {operation:'acceptRustStandardSumQuestionFamily',arguments:['std::option::Option','std::option::Option','',''],expected:true},
 {operation:'acceptRustStandardSumQuestionFamily',arguments:['std::result::Result','std::result::Result','a','a'],expected:true},
 {operation:'acceptRustStandardSumQuestionFamily',arguments:['std::result::Result','std::result::Result','a','b'],expected:false},
 {operation:'acceptRustStandardSumQuestionContext',arguments:['std::option::Option','std::option::Option',true,false,'',''],expected:false},
 {operation:'acceptRustStandardSumQuestionContext',arguments:['std::option::Option','std::option::Option',true,true,'',''],expected:true},
];
export const rustStandardSumOwnedPropagationOriginals=[
 {name:'owned Option propagation transfers its actual successful String field',sourceVersionRange,stdout:'text\n0\n',source:'fn keep(value:Option<String>)->Option<String>{let text=value?;Some(text)}fn main(){println!("{}",keep(Some(String::from("text"))).unwrap_or_default());println!("{}",keep(None).unwrap_or_default().len());}'},
 {name:'owned Result propagation preserves actual String errors while changing success',sourceVersionRange,stdout:'4\nerror\n',source:'fn length(value:Result<String,String>)->Result<usize,String>{let text=value?;Ok(text.len())}fn main(){let a=length(Ok(String::from("text")));let b=length(Err(String::from("error")));println!("{}",a.unwrap_or(0usize));let message=match b{Ok(_)=>String::from("wrong"),Err(error)=>error};println!("{}",message);}'},
 {name:'stacked owned Option propagation transfers each selected nested field',sourceVersionRange,stdout:'text\nfalse\nfalse\n',source:'fn keep(value:Option<Option<String>>)->Option<String>{Some(value??)}fn main(){println!("{}",keep(Some(Some(String::from("text")))).unwrap_or_default());println!("{}",keep(Some(None)).is_some());println!("{}",keep(None).is_some());}'},
 {name:'owned vector propagation creates the actual successful vector binding',sourceVersionRange,stdout:'2\nfalse\n',source:'fn keep(value:Option<Vec<u8>>)->Option<Vec<u8>>{let values=value?;Some(values)}fn main(){println!("{}",keep(Some(vec![7u8,9u8])).unwrap_or_default().len());println!("{}",keep(None).is_some());}'},
 {name:'owned propagation evaluates its receiver once and stops before later effects',sourceVersionRange,stdout:'receiver\nafter\ntext\nreceiver\nfalse\n',source:'fn make(present:bool)->Option<String>{println!("receiver");if present{Some(String::from("text"))}else{None}}fn keep(present:bool)->Option<String>{let text=make(present)?;println!("after");Some(text)}fn main(){println!("{}",keep(true).unwrap_or_default());println!("{}",keep(false).is_some());}'},
 {name:'owned error propagation skips a later owning producer',sourceVersionRange,stdout:'false\n',source:'fn later()->String{println!("later");String::from("other")}fn keep(value:Result<String,String>)->Result<String,String>{let text=value?;let other=later();Ok(text)}fn main(){println!("{}",keep(Err(String::from("error"))).is_ok());}'},
 {name:'owned propagation retains actual residual conversion obligation',sourceVersionRange,originalCompilerOutcome:'accept',stdout:'',previousBoundary:'owned residual primitive conversion boundary',source:'fn later(value:Result<String,u8>)->Result<String,u16>{Ok(value?)}fn main(){}'},
];
export const rustStandardSumOwnedPropagationRefusals=[
 {name:'owned propagation consumes its actual original receiver',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn wrong(value:Option<String>)->Option<String>{let text=value?;println!("{}",value.is_some());Some(text)}fn main(){}'},
 {name:'owned propagated payload cannot be consumed twice',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn wrong(value:Option<String>)->Option<String>{let text=value?;let result=Some(text);println!("{}",text);result}fn main(){}'},
 {name:'owned propagation cannot consume a sum through a reference',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn wrong(value:Option<String>)->Option<String>{let text=(&value)?;Some(text)}fn main(){}'},
 {name:'owned propagation cannot retain a live borrowed read of its moved root',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn wrong(value:Option<String>)->Option<String>{let borrowed=&value;let text=value?;println!("{}",borrowed.is_some());Some(text)}fn main(){}'},
];
export const rustStandardSumOwnedPropagationPreviousBoundaries=[{
 name:'owned residual primitive conversion boundary',previousCollection:'rustStandardSumOwnedPropagationRefusals',previousIndex:3,previousDiagnosticKinds:['syntax','type','unsupported'],sourceVersionRange,originalCompilerOutcome:'accept',
 row:{name:'owned propagation retains actual residual conversion obligation',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn later(value:Result<String,u8>)->Result<String,u16>{Ok(value?)}fn main(){}'},
 extension:'Separate complete primitive From conversion with actual owned operand consumption. Old owned identity-error controls remain exact; no observation is recorded.',
}];
