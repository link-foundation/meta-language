// Independent authored inputs. No observations or compiler claims.
// Current Reference rationale: match scrutinees are not extending expressions:
// https://doc.rust-lang.org/reference/destructors.html#temporary-lifetime-extension
// Exact 1.90/1.99 compiler controls remain pending CI execution.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustStandardSumOwnershipDecisions=[
 {operation:'readRustStandardSumOwnershipProfile',arguments:['std::option::Option',['copy']],expected:'copy'},
 {operation:'readRustStandardSumOwnershipProfile',arguments:['std::option::Option',['owned']],expected:'owned'},
 {operation:'readRustStandardSumOwnershipProfile',arguments:['std::result::Result',['owned','copy']],expected:'owned'},
 {operation:'readRustStandardSumOwnershipProfile',arguments:['std::result::Result',['copy','owned']],expected:'owned'},
 {operation:'readRustStandardSumOwnershipProfile',arguments:['std::result::Result',['owned','owned']],expected:'owned'},
 {operation:'readRustStandardSumOwnershipProfile',arguments:['std::result::Result',['copy','copy']],expected:'copy'},
 {operation:'readRustStandardSumOwnershipProfile',arguments:['Option',['owned']],expected:''},
 {operation:'readRustStandardSumOwnershipProfile',arguments:['std::option::Option',[]],expected:''},
 {operation:'readRustStandardSumOwnershipProfile',arguments:['std::option::Option',['reference']],expected:''},
 {operation:'readRustStandardSumOwnershipProfile',arguments:['std::result::Result',['owned']],expected:''},
 {operation:'acceptRustStandardSumOwnedArguments',arguments:['std::option::Option',['actual String'],['owned']],expected:true},
 {operation:'acceptRustStandardSumOwnedArguments',arguments:['std::result::Result',['actual u8','actual String'],['copy','owned']],expected:true},
 {operation:'acceptRustStandardSumOwnedArguments',arguments:['std::option::Option',['actual u8'],['copy']],expected:false},
 {operation:'acceptRustStandardSumOwnedArguments',arguments:['std::option::Option',[''],['owned']],expected:false},
 {operation:'acceptRustStandardSumOwnedArguments',arguments:['std::option::Option',[],['owned']],expected:false},
 {operation:'acceptRustStandardSumOwnedArguments',arguments:['user::Option',['actual String'],['owned']],expected:false},
 {operation:'readRustStandardSumOwnedConstructorError',arguments:[true,0,0],expected:''},
 {operation:'readRustStandardSumOwnedConstructorError',arguments:[true,1,1],expected:''},
 {operation:'readRustStandardSumOwnedConstructorError',arguments:[false,1,1],expected:'an owned sum construction needs its actual constructor owner and exact source field arguments'},
 {operation:'readRustStandardSumOwnedConstructorError',arguments:[true,1,0],expected:'an owned sum construction needs its actual constructor owner and exact source field arguments'},
 {operation:'readRustStandardSumOwnedFieldDisposition',arguments:[true,true,true],expected:'borrow'},
 {operation:'readRustStandardSumOwnedFieldDisposition',arguments:[true,false,true],expected:'move'},
 {operation:'readRustStandardSumOwnedFieldDisposition',arguments:[true,false,false],expected:'plain'},
 {operation:'readRustStandardSumOwnedFieldDisposition',arguments:[false,true,true],expected:'plain'},
 {operation:'readRustStandardSumOwnedGuardError',arguments:[true,true],expected:'owned sum guards need their delayed payload-move and fallthrough model'},
 {operation:'readRustStandardSumOwnedGuardError',arguments:[true,false],expected:''},
 {operation:'readRustStandardSumOwnedGuardError',arguments:[false,true],expected:''},
 {operation:'acceptRustStandardSumOwnedCallResult',arguments:[true,true,true],expected:true},
 {operation:'acceptRustStandardSumOwnedCallResult',arguments:[true,true,false],expected:false},
 {operation:'acceptRustStandardSumOwnedCallResult',arguments:[true,false,true],expected:false},
 {operation:'acceptRustStandardSumOwnedCallResult',arguments:[false,true,true],expected:false},
 {operation:'readRustStandardSumOwnedPatternTransferError',arguments:[true,false,false],expected:'an unbound owned pattern field needs its actual fresh parent producer contract'},
 {operation:'readRustStandardSumOwnedPatternTransferError',arguments:[true,true,false],expected:''},
 {operation:'readRustStandardSumOwnedPatternTransferError',arguments:[true,false,true],expected:''},
 {operation:'readRustStandardSumOwnedPatternTransferError',arguments:[false,false,false],expected:''},
 {operation:'readRustStandardSumOwnershipNodeLimit',arguments:[],expected:100000},
 {operation:'readRustStandardSumOwnershipDepthLimit',arguments:[],expected:256},
 {operation:'readRustStandardSumOwnershipVersionError',arguments:[90],expected:''},
 {operation:'readRustStandardSumOwnershipVersionError',arguments:[99],expected:''},
 {operation:'readRustStandardSumOwnershipVersionError',arguments:[89],expected:'owned standard sum source semantics are uncovered outside Rust >=1.90.0 <1.100.0'},
 {operation:'readRustStandardSumOwnershipVersionError',arguments:[100],expected:'owned standard sum source semantics are uncovered outside Rust >=1.90.0 <1.100.0'},
];
export const rustStandardSumOwnershipOriginals=[
 {name:'a field borrowed from a named root retains its enclosing scope',sourceVersionRange,stdout:'4\n',source:`
fn main(){let value:Option<String>=Some(String::from("text"));let fallback=String::from("none");let field=match &value{Some(text)=>text,None=>&fallback};println!("{}",field.len());}
`},
 {name:'a direct borrow initializer extends its sum temporary before later projection',sourceVersionRange,stdout:'4\n',source:`
fn main(){let root=&Some(String::from("text"));let fallback=String::from("none");let field=match root{Some(text)=>text,None=>&fallback};println!("{}",field.len());}
`},
 {name:'a fresh producer can hand its owned field to a consuming binding',sourceVersionRange,stdout:'4\n',source:`
fn make()->Option<String>{Some(String::from("text"))}
fn main(){let length=match make(){Some(text)=>text.len(),None=>0usize};println!("{}",length);}
`},
];
export const rustStandardSumOwnershipRefusals=[
 {name:'a match scrutinee borrow does not extend its fresh sum temporary',sourceVersionRange,source:`
fn main(){let fallback=String::from("none");let field=match &Some(String::from("text")){Some(text)=>text,None=>&fallback};println!("{}",field.len());}
`},
 {name:'a field borrow cannot escape the block owning its named sum',sourceVersionRange,source:`
fn main(){let fallback=String::from("none");let field={let value:Option<String>=Some(String::from("text"));match &value{Some(text)=>text,None=>&fallback}};println!("{}",field.len());}
`},
];
