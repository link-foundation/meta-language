// Complete independent originals and stdout expectations; never observations.
const sourceVersionRange='>=1.90.0 <1.100.0';
const validSourceCompilerExpectation={model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',outcome:'accept',category:'valid Rust source'};
export const rustStandardSumGuardRegistrationDecisions=[
 {operation:'acceptRustOwnedBindingRegistration',arguments:[true,false],expected:true},
 {operation:'acceptRustOwnedBindingRegistration',arguments:[true,true],expected:false},
 {operation:'acceptRustOwnedBindingRegistration',arguments:[false,false],expected:false},
 {operation:'acceptRustOwnedBindingRegistration',arguments:[false,true],expected:false},
];
export const rustStandardSumGuardDecisions=[
 {operation:'readRustStandardSumGuardMode',arguments:[true,true],expected:'deferred guard'},
 {operation:'readRustStandardSumGuardMode',arguments:[false,true],expected:'ordinary'},
 {operation:'readRustStandardSumGuardMode',arguments:[true,false],expected:'ordinary'},
 {operation:'readRustStandardSumDeferredGuardPatternError',arguments:[1,0],expected:''},
 {operation:'readRustStandardSumDeferredGuardPatternError',arguments:[2,0],expected:'guarded owned sum alternatives need their delayed move agreement model'},
 {operation:'readRustStandardSumDeferredGuardPatternError',arguments:[1,1],expected:'guarded owned sum whole-value aliases need their delayed payload move model'},
 {operation:'readRustStandardSumGuardBindingDisposition',arguments:[true,true],expected:'readonly'},
 {operation:'readRustStandardSumGuardBindingDisposition',arguments:[true,false],expected:'readonly'},
 {operation:'readRustStandardSumGuardBindingDisposition',arguments:[false,true],expected:'transfer'},
 {operation:'readRustStandardSumGuardBindingDisposition',arguments:[false,false],expected:'plain'},
];
export const rustStandardSumGuardOriginals=[
 {name:'owned guard false leaves payload available to a later matching arm',sourceVersionRange,singleCallOwners:[],stdout:'guard\nfallback\ntext\nguard\nbody\nlonger\nnone\n',source:`
fn inspect(text:&String)->bool{println!("guard");text.len()>4usize}
fn choose(value:Option<String>)->String{match value{Some(text) if inspect(&text)=>{println!("body");text},Some(text)=>{println!("fallback");text},None=>String::from("none")}}
fn main(){println!("{}",choose(Some(String::from("text"))));println!("{}",choose(Some(String::from("longer"))));println!("{}",choose(None));}
`},
 {name:'false owned field guard permits consuming original root inside fallthrough arm',sourceVersionRange,singleCallOwners:[],stdout:'guard\ntext\nguard\nlonger\nnone\n',source:`
fn inspect(text:&String)->bool{println!("guard");text.len()>4usize}
fn reveal(value:Option<String>)->String{match value{Some(text)=>text,None=>String::from("none")}}
fn choose(value:Option<String>)->String{match value{Some(text) if inspect(&text)=>text,_=>reveal(value)}}
fn main(){println!("{}",choose(Some(String::from("text"))));println!("{}",choose(Some(String::from("longer"))));println!("{}",choose(None));}
`},
 {name:'true guard commits owned vector field only on its body path',sourceVersionRange,singleCallOwners:[],stdout:'guard\nbody\n2\nguard\nfallback\n0\n0\n',source:`
fn inspect(values:&Vec<u8>)->bool{println!("guard");values.len()>0usize}
fn count(values:Vec<u8>)->usize{println!("body");values.len()}
fn choose(value:Option<Vec<u8>>)->usize{match value{Some(values) if inspect(&values)=>count(values),Some(values)=>{println!("fallback");values.len()},None=>0usize}}
fn main(){println!("{}",choose(Some(vec![7u8,9u8])));println!("{}",choose(Some(Vec::<u8>::new())));println!("{}",choose(None));}
`},
 {name:'readonly owned guard repeated reads preserve original bindings for body',sourceVersionRange,singleCallOwners:[],stdout:'read\nread\ntext\nread\nnone\n',source:`
fn read(text:&String)->bool{println!("read");text.len()>0usize}
fn choose(value:Option<String>)->String{match value{Some(text) if read(&text)&&read(&text)=>text,_=>String::from("none")}}
fn main(){println!("{}",choose(Some(String::from("text"))));println!("{}",choose(Some(String::from(""))));}
`},
 {name:'nonbinding guarded owned inspection preserves root for later consuming match',sourceVersionRange,singleCallOwners:[],stdout:'true\ntext\nfalse\nnone\n',source:`
fn reveal(value:Option<String>)->String{match value{Some(text)=>text,None=>String::from("none")}}
fn choose(value:Option<String>)->String{let present=match value{Some(_) if true=>true,_=>false};println!("{}",present);reveal(value)}
fn main(){println!("{}",choose(Some(String::from("text"))));println!("{}",choose(None));}
`},
 {name:'temporary owned sum guard reads then transfers actual producer payload',sourceVersionRange,singleCallOwners:['make'],singleCallFunction:'choose',stdout:'make\nguard\ntext\nmake\nguard\nempty\n',source:`
fn make(present:bool)->Option<String>{println!("make");if present{Some(String::from("text"))}else{Some(String::from(""))}}
fn inspect(text:&String)->bool{println!("guard");text.len()>0usize}
fn choose(present:bool)->String{match make(present){Some(text) if inspect(&text)=>text,Some(text)=>{let count=text.len();String::from("empty")},None=>String::from("none")}}
fn main(){println!("{}",choose(true));println!("{}",choose(false));}
`},
 {name:'fresh nested producer guard observes owned child without fabricated owner IDs',sourceVersionRange,singleCallOwners:['make'],singleCallFunction:'choose',stdout:'make\ntext\nmake\nnone\n',source:`
fn make(present:bool)->Option<Option<String>>{println!("make");if present{Some(Some(String::from("text")))}else{Some(None)}}
fn reveal(value:Option<String>)->String{match value{Some(text)=>text,None=>String::from("none")}}
fn choose(present:bool)->String{match make(present){Some(inner) if inner.is_some()&&!inner.is_none()=>reveal(inner),Some(inner)=>reveal(inner),None=>String::from("none")}}
fn main(){println!("{}",choose(true));println!("{}",choose(false));}
`},
 {name:'nominal Copy guards keep existing immediate source ownership behavior',sourceVersionRange,singleCallOwners:[],stdout:'7\n0\n',source:`
#[derive(Clone,Copy)]struct Mark{number:u8}
fn choose(value:Option<Mark>)->u8{match value{Some(mark) if mark.number>0u8=>mark.number,_=>0u8}}
fn main(){println!("{}",choose(Some(Mark{number:7u8})));println!("{}",choose(None));}
`},
 {name:'historical guarded by-value input now uses deferred guard consumer',sourceVersionRange,singleCallOwners:[],stdout:'',originalCompilerExpectation:validSourceCompilerExpectation,expectationHistory:[{fixture:'rust-standard-sum-owned-originals.mjs',name:'guarded by-value field moves retain their fallthrough obligation',translationExpectation:'refuse immediate owned pattern guard',reason:'owned sum guards need their delayed payload-move and fallthrough model'}],source:'fn bad(value:Option<String>)->usize{match value{Some(text) if text.len()>0usize=>text.len(),_=>0usize}}fn main(){}'},
 {name:'temporary readonly guard stays borrowed through nested block scope entry',sourceVersionRange,singleCallOwners:['make'],singleCallFunction:'choose',stdout:'make\nread\ntext\nmake\nread\nempty\n',source:`
fn make(present:bool)->Option<String>{println!("make");if present{Some(String::from("text"))}else{Some(String::from(""))}}
fn choose(present:bool)->String{match make(present){Some(text) if {println!("read");text.len()>0usize}=>text,Some(text)=>{let count=text.len();String::from("empty")},None=>String::from("none")}}
fn main(){println!("{}",choose(true));println!("{}",choose(false));}
`},
];
export const rustStandardSumGuardRefusals=[
 {name:'readonly guard binding cannot pass its owned String field by value',sourceVersionRange,source:'fn consume(text:String)->bool{text.len()>0usize}fn wrong(value:Option<String>)->usize{match value{Some(text) if consume(text)=>text.len(),_=>0usize}}fn main(){}'},
 {name:'readonly guard binding cannot move through standard String From',sourceVersionRange,source:'fn wrong(value:Option<String>)->usize{match value{Some(text) if String::from(text).len()>0usize=>text.len(),_=>0usize}}fn main(){}'},
 {name:'readonly nested guard cannot consume actual owned child sum',sourceVersionRange,source:'fn consume(value:Option<String>)->bool{match value{Some(_)=>true,None=>false}}fn wrong(value:Option<Option<String>>)->usize{match value{Some(inner) if consume(inner)=>0usize,_=>0usize}}fn main(){}'},
 {name:'guard cannot consume an outside owned root and pretend fallthrough unchanged',sourceVersionRange,source:'fn consume(text:String)->bool{text.len()>0usize}fn wrong(value:Option<String>,outside:String)->usize{match value{Some(text) if consume(outside)=>text.len(),_=>0usize}}fn main(){}'},
 {name:'guarded whole value alias keeps explicit delayed payload agreement obligation',sourceVersionRange,reasonIncludes:'whole-value aliases',source:'fn wrong(value:Option<String>)->usize{match value{whole@Some(text) if text.len()>0usize=>text.len(),_=>0usize}}fn main(){}'},
 {name:'guarded owned or alternatives retain delayed move agreement obligation',sourceVersionRange,reasonIncludes:'alternatives',source:'fn wrong(value:Result<String,String>)->usize{match value{Ok(text)|Err(text) if text.len()>0usize=>text.len(),_=>0usize}}fn main(){}'},
 {name:'true owned body transfer does not allow later whole root reuse',sourceVersionRange,source:'fn consume(value:Option<String>)->usize{match value{Some(text)=>text.len(),None=>0usize}}fn wrong(value:Option<String>)->usize{let count=match value{Some(text) if text.len()>0usize=>text.len(),_=>0usize};consume(value)}fn main(){}'},
 {name:'temporary readonly guard block cannot allocate an owned identity then move field',sourceVersionRange,source:'fn make()->Option<String>{Some(String::from("text"))}fn consume(text:String)->bool{text.len()>0usize}fn wrong()->usize{match make(){Some(text) if {consume(text)}=>text.len(),_=>0usize}}fn main(){}'},
];
