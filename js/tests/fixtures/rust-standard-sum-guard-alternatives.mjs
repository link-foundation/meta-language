// Independent complete source/stdout contracts for the expanded deferred mode.
// These are declarations for future controls, never executed observations.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustStandardSumGuardAlternativeDecisions=[
 {operation:'readRustStandardSumGuardAlternativePatternError',arguments:[1,0],expected:''},
 {operation:'readRustStandardSumGuardAlternativePatternError',arguments:[2,0],expected:''},
 {operation:'readRustStandardSumGuardAlternativePatternError',arguments:[2,2],expected:''},
 {operation:'readRustStandardSumGuardAlternativePatternError',arguments:[0,0],expected:'guarded owned sum patterns need their actual finite alternative and alias counts'},
 {operation:'readRustStandardSumGuardAlternativePatternError',arguments:[1,-1],expected:'guarded owned sum patterns need their actual finite alternative and alias counts'},
 {operation:'readRustStandardSumGuardAlternativeBindingError',arguments:[true,true,true],expected:''},
 {operation:'readRustStandardSumGuardAlternativeBindingError',arguments:[false,true,true],expected:'guarded owned sum alternatives must bind the same complete actual source types'},
 {operation:'readRustStandardSumGuardAlternativeBindingError',arguments:[true,false,true],expected:'guarded owned sum alternatives must bind the same complete actual source types'},
 {operation:'readRustStandardSumGuardAlternativeBindingError',arguments:[true,true,false],expected:'guarded owned sum alternatives must retain the same actual root loans and lifetime provenance'},
 {operation:'readRustStandardSumGuardAliasTransferError',arguments:[1,1],expected:''},
 {operation:'readRustStandardSumGuardAliasTransferError',arguments:[1,0],expected:''},
 {operation:'readRustStandardSumGuardAliasTransferError',arguments:[2,1],expected:''},
 {operation:'readRustStandardSumGuardAliasTransferError',arguments:[1,2],expected:'guarded owned sum whole-value aliases with owned field bindings need their partial-move agreement model'},
];
export const rustStandardSumGuardAlternativeOriginals=[
 {name:'disjoint owned Result alternatives retain actual field owner across guard fallthrough',sourceVersionRange,singleCallOwners:[],stdout:'guard\nfallback\ntext\nguard\nfallback\nbad\nguard\nbody\nlonger\nguard\nbody\nfailure\n',source:`
fn inspect(text:&String)->bool{println!("guard");text.len()>4usize}
fn reveal(value:Result<String,String>)->String{match value{Ok(text)=>text,Err(error)=>error}}
fn choose(value:Result<String,String>)->String{match value{Ok(text)|Err(text) if inspect(&text)=>{println!("body");text},_=>{println!("fallback");reveal(value)}}}
fn main(){println!("{}",choose(Ok(String::from("text"))));println!("{}",choose(Err(String::from("bad"))));println!("{}",choose(Ok(String::from("longer"))));println!("{}",choose(Err(String::from("failure"))));}
`},
 {name:'overlapping owned alternatives repeat false guard without premature payload moves',sourceVersionRange,singleCallOwners:[],stdout:'guard\nguard\nfallback\ntext\nguard\nbody\nlonger\nnone\n',source:`
fn inspect(text:&String)->bool{println!("guard");text.len()>4usize}
fn choose(value:Option<String>)->String{match value{Some(text)|Some(text) if inspect(&text)=>{println!("body");text},Some(text)=>{println!("fallback");text},None=>String::from("none")}}
fn main(){println!("{}",choose(Some(String::from("text"))));println!("{}",choose(Some(String::from("longer"))));println!("{}",choose(None));}
`},
 {name:'whole owned alias with wildcard fields stays readonly in guard and transfers once in body',sourceVersionRange,singleCallOwners:[],stdout:'text\nnone\n',source:`
fn reveal(value:Option<String>)->String{match value{Some(text)=>text,None=>String::from("none")}}
fn choose(value:Option<String>)->String{match value{whole@Some(_) if whole.is_some()&&!whole.is_none()=>reveal(whole),_=>reveal(value)}}
fn main(){println!("{}",choose(Some(String::from("text"))));println!("{}",choose(None));}
`},
 {name:'whole owned alias retains independent Copy field after actual whole transfer',sourceVersionRange,singleCallOwners:[],stdout:'7\npresent\npresent\nbad\n',source:`
fn reveal(value:Result<u8,String>)->String{match value{Ok(_)=>String::from("present"),Err(error)=>error}}
fn choose(value:Result<u8,String>)->String{match value{whole@Ok(number) if number>0u8&&whole.is_ok()=>{let text=reveal(whole);println!("{}",number);text},_=>reveal(value)}}
fn main(){println!("{}",choose(Ok(7)));println!("{}",choose(Ok(0)));println!("{}",choose(Err(String::from("bad"))));}
`},
 {name:'temporary whole alias guard retains actual producer proof and one evaluation',sourceVersionRange,singleCallOwners:['make'],singleCallFunction:'choose',stdout:'make\ntext\nmake\nnone\n',source:`
fn make(present:bool)->Option<String>{println!("make");if present{Some(String::from("text"))}else{None}}
fn reveal(value:Option<String>)->String{match value{Some(text)=>text,None=>String::from("none")}}
fn choose(present:bool)->String{match make(present){whole@Some(_) if whole.is_some()=>reveal(whole),_=>String::from("none")}}
fn main(){println!("{}",choose(true));println!("{}",choose(false));}
`},
 {name:'overlapping whole aliases repeat borrowed guard while preserving original fallthrough root',sourceVersionRange,singleCallOwners:[],stdout:'guard\nguard\ntext\nnone\n',source:`
fn inspect(value:&Option<String>)->bool{println!("guard");value.is_none()}
fn reveal(value:Option<String>)->String{match value{Some(text)=>text,None=>String::from("none")}}
fn choose(value:Option<String>)->String{match value{whole@Some(_)|whole@Some(_) if inspect(&whole)=>reveal(whole),_=>reveal(value)}}
fn main(){println!("{}",choose(Some(String::from("text"))));println!("{}",choose(None));}
`},
 {name:'nested safe alias commits one actual owned sum field after guard',sourceVersionRange,singleCallOwners:[],stdout:'text\nnone\nnone\n',source:`
fn reveal(value:Option<String>)->String{match value{Some(text)=>text,None=>String::from("none")}}
fn choose(value:Option<Option<String>>)->String{match value{Some(inner@Some(_)) if inner.is_some()=>reveal(inner),Some(inner)=>reveal(inner),None=>String::from("none")}}
fn main(){println!("{}",choose(Some(Some(String::from("text")))));println!("{}",choose(Some(None)));println!("{}",choose(None));}
`},
 {name:'historical guarded owned or alternatives now retain actual delayed move agreement',sourceVersionRange,singleCallOwners:[],stdout:'',expectationHistory:[{fixture:'rust-standard-sum-guards.mjs',name:'guarded owned or alternatives retain delayed move agreement obligation',translationExpectation:'refuse incomplete delayed alternative move model'}],source:'fn wrong(value:Result<String,String>)->usize{match value{Ok(text)|Err(text) if text.len()>0usize=>text.len(),_=>0usize}}fn main(){}'},
];
export const rustStandardSumGuardAlternativeRefusals=[
 {name:'guarded alternatives cannot unify different actual owned payload types',sourceVersionRange,reasonIncludes:'same complete actual source types',source:'fn wrong(value:Result<String,Vec<u8>>)->usize{match value{Ok(item)|Err(item) if item.len()>0usize=>item.len(),_=>0usize}}fn main(){}'},
 {name:'guarded alternatives must retain every actual binding name',sourceVersionRange,reasonIncludes:'same names',source:'fn wrong(value:Result<String,String>)->usize{match value{Ok(left)|Err(right) if left.len()>0usize=>left.len(),_=>0usize}}fn main(){}'},
 {name:'whole alias cannot also bind its actual owned field by value',sourceVersionRange,reasonIncludes:'partial-move agreement',source:'fn wrong(value:Option<String>)->usize{match value{whole@Some(text) if text.len()>0usize=>text.len(),_=>0usize}}fn main(){}'},
 {name:'nested whole alias cannot also bind its actual owned subfield',sourceVersionRange,reasonIncludes:'partial-move agreement',source:'fn wrong(value:Option<Option<String>>)->usize{match value{Some(inner@Some(text)) if text.len()>0usize=>text.len(),_=>0usize}}fn main(){}'},
 {name:'whole alias is a readonly guard place rather than a by-value callable argument',sourceVersionRange,source:'fn consume(value:Option<String>)->bool{match value{Some(_)=>true,None=>false}}fn wrong(value:Option<String>)->usize{match value{whole@Some(_) if consume(whole)=>0usize,_=>0usize}}fn main(){}'},
 {name:'expanded alternatives cannot move guard field while attempting either alternative',sourceVersionRange,source:'fn consume(text:String)->bool{text.len()>0usize}fn wrong(value:Result<String,String>)->usize{match value{Ok(text)|Err(text) if consume(text)=>text.len(),_=>0usize}}fn main(){}'},
];
