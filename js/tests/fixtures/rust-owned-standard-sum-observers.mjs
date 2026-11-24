// Independent complete originals and literal expectations, not observations.
// Source signature expectation: Option::{is_some,is_none} and
// Result::{is_ok,is_err} take &self throughout Rust >=1.90.0 <1.100.0.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustOwnedStandardSumObserverDecisions=[
 {operation:'acceptRustOwnedStandardSumObserverProfile',arguments:['owned',0],expected:true},
 {operation:'acceptRustOwnedStandardSumObserverProfile',arguments:['copy',0],expected:false},
 {operation:'acceptRustOwnedStandardSumObserverProfile',arguments:['copy',1],expected:true},
 {operation:'acceptRustOwnedStandardSumObserverProfile',arguments:['',1],expected:false},
 {operation:'readRustOwnedStandardSumObserverArity',arguments:['std::option::Option','is_some'],expected:0},
 {operation:'readRustOwnedStandardSumObserverArity',arguments:['std::option::Option','is_none'],expected:0},
 {operation:'readRustOwnedStandardSumObserverArity',arguments:['std::result::Result','is_ok'],expected:0},
 {operation:'readRustOwnedStandardSumObserverArity',arguments:['std::result::Result','is_err'],expected:0},
 {operation:'readRustOwnedStandardSumObserverArity',arguments:['std::option::Option','is_ok'],expected:-1},
 {operation:'readRustOwnedStandardSumObserverArity',arguments:['local::Option','is_some'],expected:-1},
 {operation:'readRustOwnedStandardSumObserverBranch',arguments:['std::option::Option','Some','is_some'],expected:'true'},
 {operation:'readRustOwnedStandardSumObserverBranch',arguments:['std::option::Option','None','is_some'],expected:'false'},
 {operation:'readRustOwnedStandardSumObserverBranch',arguments:['std::option::Option','Some','is_none'],expected:'false'},
 {operation:'readRustOwnedStandardSumObserverBranch',arguments:['std::option::Option','None','is_none'],expected:'true'},
 {operation:'readRustOwnedStandardSumObserverBranch',arguments:['std::result::Result','Ok','is_ok'],expected:'true'},
 {operation:'readRustOwnedStandardSumObserverBranch',arguments:['std::result::Result','Err','is_ok'],expected:'false'},
 {operation:'readRustOwnedStandardSumObserverBranch',arguments:['std::result::Result','Ok','is_err'],expected:'false'},
 {operation:'readRustOwnedStandardSumObserverBranch',arguments:['std::result::Result','Err','is_err'],expected:'true'},
 {operation:'readRustOwnedStandardSumObserverSourceError',arguments:[true,0,true,0,true,true,false,false,true],expected:''},
 {operation:'readRustOwnedStandardSumObserverSourceError',arguments:[true,0,true,0,true,false,true,false,false],expected:''},
 {operation:'readRustOwnedStandardSumObserverSourceError',arguments:[true,1,true,1,true,false,false,false,false],expected:''},
 {operation:'readRustOwnedStandardSumObserverSourceError',arguments:[true,2,true,2,true,false,false,false,true],expected:''},
 {operation:'readRustOwnedStandardSumObserverSourceError',arguments:[true,0,true,0,true,false,false,true,true],expected:''},
 {operation:'readRustOwnedStandardSumObserverSourceError',arguments:[true,0,true,0,true,false,true,true,false],expected:''},
 {operation:'readRustOwnedStandardSumObserverSourceError',arguments:[true,0,true,0,true,false,false,false,false],expected:'an owned sum observer needs an available source root or an actual owned producer'},
 {operation:'readRustOwnedStandardSumObserverSourceError',arguments:[true,0,true,0,true,false,false,true,false],expected:'an owned sum observer cannot inspect an unproven borrowed source place'},
 {operation:'readRustOwnedStandardSumObserverSourceError',arguments:[true,1,true,0,true,false,false,false,true],expected:'an owned sum observer needs retained immutable reference and lifetime provenance'},
 {operation:'readRustOwnedStandardSumObserverSourceError',arguments:[true,1,false,1,true,false,false,false,true],expected:'an owned sum observer needs retained immutable reference and lifetime provenance'},
 {operation:'readRustOwnedStandardSumObserverSourceError',arguments:[false,0,true,0,true,true,false,false,true],expected:'an owned sum observer needs its actual checked ownership and source origin'},
 {operation:'readRustOwnedStandardSumObserverSourceError',arguments:[true,0,true,0,false,true,false,false,true],expected:'an owned sum observer needs its actual checked ownership and source origin'},
 {operation:'readRustOwnedStandardSumObserverVersionError',arguments:['std::option::Option','is_some',90],expected:''},
 {operation:'readRustOwnedStandardSumObserverVersionError',arguments:['std::result::Result','is_err',99],expected:''},
 {operation:'readRustOwnedStandardSumObserverVersionError',arguments:['std::option::Option','is_some',100],expected:'owned standard sum observer source semantics are uncovered outside Rust >=1.90.0 <1.100.0'},
];
export const rustOwnedStandardSumObserverOriginals=[
 {name:'repeated owned Option String observers preserve the later consuming match',sourceVersionRange,singleCallOwners:[],stdout:'true\ntrue\nfalse\ntext\nfalse\nfalse\ntrue\nnone\n',source:`
fn inspect(value:Option<String>)->String{println!("{}",value.is_some());println!("{}",value.is_some());println!("{}",value.is_none());match value{Some(text)=>text,None=>String::from("none")}}
fn main(){println!("{}",inspect(Some(String::from("text"))));println!("{}",inspect(None));}
`},
 {name:'repeated Result String observers preserve each original owned field',sourceVersionRange,singleCallOwners:[],stdout:'true\nfalse\ntrue\ngood\nfalse\ntrue\nfalse\nbad\n',source:`
fn inspect(value:Result<String,String>)->String{println!("{}",value.is_ok());println!("{}",value.is_err());println!("{}",value.is_ok());match value{Ok(text)=>text,Err(error)=>error}}
fn main(){println!("{}",inspect(Ok(String::from("good"))));println!("{}",inspect(Err(String::from("bad"))));}
`},
 {name:'owned vector Option observers preserve payload for a consuming parameter',sourceVersionRange,singleCallOwners:[],stdout:'true\nfalse\n2\nfalse\ntrue\n0\n',source:`
fn count(value:Option<Vec<u8>>)->usize{match value{Some(values)=>values.len(),None=>0usize}}
fn inspect(value:Option<Vec<u8>>)->usize{println!("{}",value.is_some());println!("{}",value.is_none());count(value)}
fn main(){println!("{}",inspect(Some(vec![7u8,9u8])));println!("{}",inspect(None));}
`},
 {name:'shared owned sum parameter observers retain caller root availability',sourceVersionRange,singleCallOwners:[],stdout:'true\nfalse\ntext\nfalse\ntrue\nnone\n',source:`
fn inspect(value:&Option<String>)->bool{value.is_some()&&value.is_some()&&!value.is_none()}
fn reveal(value:Option<String>)->String{match value{Some(text)=>text,None=>String::from("none")}}
fn choose(value:Option<String>)->String{let borrowed=&value;println!("{}",inspect(borrowed));println!("{}",borrowed.is_none());reveal(value)}
fn main(){println!("{}",choose(Some(String::from("text"))));println!("{}",choose(None));}
`},
 {name:'nested owned sums observe only outer discriminant and preserve later nested match',sourceVersionRange,singleCallOwners:[],stdout:'true\nfalse\ngood\ntrue\nfalse\nbad\nfalse\ntrue\nnone\n',source:`
fn inspect(value:Option<Result<String,String>>)->String{println!("{}",value.is_some());println!("{}",value.is_none());match value{Some(Ok(text))=>text,Some(Err(error))=>error,None=>String::from("none")}}
fn main(){println!("{}",inspect(Some(Ok(String::from("good")))));println!("{}",inspect(Some(Err(String::from("bad")))));println!("{}",inspect(None));}
`},
 {name:'shared Result with distinct owned payload families preserves both roots',sourceVersionRange,singleCallOwners:[],stdout:'true\nfalse\n4\nfalse\ntrue\n2\n',source:`
fn inspect(value:&Result<String,Vec<u8>>)->bool{value.is_ok()&&value.is_ok()}
fn choose(value:Result<String,Vec<u8>>)->usize{println!("{}",inspect(&value));println!("{}",value.is_err());match value{Ok(text)=>text.len(),Err(values)=>values.len()}}
fn main(){println!("{}",choose(Ok(String::from("text"))));println!("{}",choose(Err(vec![7u8,9u8])));}
`},
 {name:'temporary owned producer evaluates once before its observer',sourceVersionRange,singleCallOwners:['make'],singleCallFunction:'choose',stdout:'make\ntrue\nmake\nfalse\n',source:`
fn make(present:bool)->Option<String>{println!("make");if present{Some(String::from("text"))}else{None}}
fn choose(present:bool)->bool{make(present).is_some()}
fn main(){println!("{}",choose(true));println!("{}",choose(false));}
`},
 {name:'borrowed-place observer does not move its shared payload or root',sourceVersionRange,singleCallOwners:[],stdout:'true\nfalse\ntext\n',source:`
fn reveal(value:Option<String>)->String{match value{Some(text)=>text,None=>String::from("none")}}
fn main(){let value:Option<String>=Some(String::from("text"));let borrowed=&value;println!("{}",(*borrowed).is_some());println!("{}",borrowed.is_none());println!("{}",reveal(value));}
`},
 {name:'shared Copy sum projected from owned parent retains its transitive root loan',sourceVersionRange,singleCallOwners:[],stdout:'true\ntrue\npresent\nfalse\nfalse\nabsent\nfalse\nfalse\nbad\n',source:`
fn inspect(value:&Result<Option<u8>,String>)->bool{match value{Ok(inner)=>inner.is_some()&&!inner.is_none(),Err(_)=>false}}
fn reveal(value:Result<Option<u8>,String>)->String{match value{Ok(Some(_))=>String::from("present"),Ok(None)=>String::from("absent"),Err(error)=>error}}
fn choose(value:Result<Option<u8>,String>)->String{println!("{}",inspect(&value));println!("{}",inspect(&value));reveal(value)}
fn main(){println!("{}",choose(Ok(Some(7))));println!("{}",choose(Ok(None)));println!("{}",choose(Err(String::from("bad"))));}
`},
];
export const rustOwnedStandardSumObserverRefusals=[
 {name:'owned observer cannot revive a consumed sum root',sourceVersionRange,source:'fn consume(value:Option<String>)->usize{match value{Some(text)=>text.len(),None=>0usize}}fn main(){let value:Option<String>=Some(String::from("text"));println!("{}",consume(value));println!("{}",value.is_some());}'},
 {name:'borrowed observer cannot outlive moving its actual owner',sourceVersionRange,source:'fn consume(value:Option<String>)->usize{match value{Some(text)=>text.len(),None=>0usize}}fn main(){let value:Option<String>=Some(String::from("text"));let borrowed=&value;println!("{}",borrowed.is_some());println!("{}",consume(value));println!("{}",borrowed.is_none());}'},
 {name:'owned observer cannot keep a local root alive past its drop scope',sourceVersionRange,source:'fn main(){let borrowed={let value:Option<String>=Some(String::from("text"));&value};println!("{}",borrowed.is_some());}'},
 {name:'nonconsuming observer does not authorize moving payload from shared sum',sourceVersionRange,source:'fn wrong(value:&Option<String>)->String{let present=value.is_some();match value{&Some(text)=>text,&None=>String::from("none")}}fn main(){}'},
 {name:'observer after consuming nested field cannot recover outer owner',sourceVersionRange,source:'fn main(){let value:Option<Result<String,String>>=Some(Ok(String::from("text")));let text=match value{Some(Ok(text))=>text,Some(Err(error))=>error,None=>String::from("none")};println!("{}",value.is_some());}'},
 {name:'zero argument observer arity cannot discard an eager owned expression',sourceVersionRange,source:'fn main(){let value:Option<String>=None;println!("{}",value.is_none(String::from("wrong")));}'},
 {name:'method spelling does not change the actual standard sum family',sourceVersionRange,source:'fn main(){let value:Option<String>=None;println!("{}",value.is_ok());}'},
 {name:'borrowed place inspection does not permit a following consuming method',sourceVersionRange,source:'fn consume(value:Option<String>)->usize{match value{Some(text)=>text.len(),None=>0usize}}fn main(){let value:Option<String>=Some(String::from("text"));let borrowed=&value;println!("{}",(*borrowed).is_some());println!("{}",consume(*borrowed));}'},
 {name:'shared nested Copy observer cannot outlive consuming its transitive owned parent',sourceVersionRange,source:'fn consume(value:Result<Option<u8>,String>)->bool{match value{Ok(_)=>true,Err(_)=>false}}fn choose(value:Result<Option<u8>,String>)->bool{let Ok(inner)=&value else{return false;};let before=inner.is_some();let moved=consume(value);inner.is_none()}fn main(){}'},
];
