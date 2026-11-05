// Independent complete originals and literal output. No source program runs
// while this fixture is authored. Existing immediate-consumer fixtures remain.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustArrayLookupOptionOriginals=[
 {name:'stored copied get keeps present absent and repeated independent values',sourceVersionRange,singleCallOwners:[],stdout:'7\n0\n7\n',source:`
fn pick(values: &[u8], index: usize) -> Option<u8> { values.get(index).copied() }
fn reveal(value: Option<u8>) -> u8 { match value { Some(number) => number, None => 0 } }
fn main() { let values=vec![7u8,9u8]; let present=pick(&values,0usize); let absent=pick(&values,9usize); println!("{}",reveal(present)); println!("{}",reveal(absent)); println!("{}",reveal(present)); }
`},
 {name:'stored copied first and last retain empty branch without invalid subtraction',sourceVersionRange,singleCallOwners:[],stdout:'7\n9\n0\n0\n',source:`
fn first(values: &[u8]) -> Option<u8> { values.first().copied() }
fn last(values: &[u8]) -> Option<u8> { values.last().copied() }
fn reveal(value: Option<u8>) -> u8 { match value { Some(number) => number, None => 0 } }
fn main() { let values=vec![7u8,9u8]; let empty: Vec<u8>=vec![]; println!("{}",reveal(first(&values))); println!("{}",reveal(last(&values))); println!("{}",reveal(first(&empty))); println!("{}",reveal(last(&empty))); }
`},
 {name:'copied lookup captures receiver before index and exactly once',sourceVersionRange,singleCallOwners:['values','index'],singleCallFunction:'step',stdout:'receiver\nindex\nafter\n7\n',source:`
fn values() -> Vec<u8> { println!("receiver"); vec![7u8] }
fn index() -> usize { println!("index"); 0usize }
fn step() -> Option<u8> { let answer=values().get(index()).copied(); println!("after"); answer }
fn reveal(value: Option<u8>) -> u8 { match value { Some(number) => number, None => 0 } }
fn main() { println!("{}",reveal(step())); }
`},
 {name:'stored copied lookup question returns before a failed continuation',sourceVersionRange,singleCallOwners:[],stdout:'before\n0\nbefore\nafter\n8\n',source:`
fn step(values: &[u8], index: usize) -> Option<u8> { println!("before"); let number=values.get(index).copied()?; println!("after"); Some(number+1) }
fn reveal(value: Option<u8>) -> u8 { match value { Some(number) => number, None => 0 } }
fn main() { let values=vec![7u8]; println!("{}",reveal(step(&values,8usize))); println!("{}",reveal(step(&values,0usize))); }
`},
 {name:'copied lookup lifts an actual Option usize index before the result question',sourceVersionRange,singleCallOwners:['index'],singleCallFunction:'step',stdout:'before\nindex\n0\nbefore\nindex\nafter\n7\n',source:`
fn index(flag:bool)->Option<usize>{println!("index");if flag{Some(0usize)}else{None}}
fn step(values:&[u8],flag:bool)->Option<u8>{println!("before");let number=values.get(index(flag)?).copied()?;println!("after");Some(number)}
fn reveal(value:Option<u8>)->u8{match value{Some(number)=>number,None=>0}}
fn main(){let values=vec![7u8];println!("{}",reveal(step(&values,false)));println!("{}",reveal(step(&values,true)));}
`},
 {name:'copied lookup map changes actual payload and keeps absence',sourceVersionRange,singleCallOwners:[],stdout:'mapper\ntrue\nfalse\n',source:`
fn truth(value:u8)->bool { println!("mapper"); value>0 }
fn step(values:&[u8])->Option<bool> { values.first().copied().map(truth) }
fn reveal(value:Option<bool>)->bool { match value {Some(flag)=>flag,None=>false} }
fn main(){let values=vec![7u8];let empty:Vec<u8>=vec![];println!("{}",reveal(step(&values)));println!("{}",reveal(step(&empty)));}
`},
 {name:'copied lookup and_then retains actual newly owned result',sourceVersionRange,singleCallOwners:[],stdout:'mapper\n7\n0\n',source:`
fn widen(value:u8)->Option<u16> { println!("mapper"); Some(u16::from(value)) }
fn step(values:&[u8])->Option<u16> { values.last().copied().and_then(widen) }
fn reveal(value:Option<u16>)->u16 { match value {Some(number)=>number,None=>0} }
fn main(){let values=vec![7u8];let empty:Vec<u8>=vec![];println!("{}",reveal(step(&values)));println!("{}",reveal(step(&empty)));}
`},
 {name:'copied Unit presence remains distinct from absent',sourceVersionRange,singleCallOwners:[],stdout:'true\nfalse\n',source:`
fn read(values:&[()])->Option<()> { values.first().copied() }
fn present(value:Option<()>)->bool {match value {Some(())=>true,None=>false}}
fn main(){let values=vec![()];let empty:Vec<()>=vec![];println!("{}",present(read(&values)));println!("{}",present(read(&empty)));}
`},
];
export const rustArrayLookupOptionRefusals=[
 {name:'a stored copied lookup index retains its usize source requirement',sourceVersionRange,source:'fn bad(values:&[u8], index:u64)->Option<u8>{values.get(index).copied()}fn main(){}',reasonIncludes:'usize'},
 {name:'a reference lookup cannot masquerade as a stored owned sum',sourceVersionRange,source:'fn bad(values:&[u8])->Option<u8>{values.first()}fn main(){}',reasonIncludes:'immediate standard'},
 {name:'copied String does not claim scalar Copy meaning',sourceVersionRange,source:'fn bad(values:&[String])->u8{let value=values.first().copied();0}fn main(){}',reasonIncludes:'Copy'},
 {name:'stored cloned String still needs its separate owned interpretation',sourceVersionRange,source:'fn bad(values:&[String])->u8{let value=values.first().cloned();0}fn main(){}',reasonIncludes:'immediate standard'},
 {name:'copied lookup retains concrete payload owner against mismatched result',sourceVersionRange,source:'fn bad(values:&[u8])->Option<bool>{values.first().copied()}fn main(){}'},
];
export const rustArrayLookupOptionLegacyConsumers=['is_some','is_none','is_some_and','is_none_or','unwrap','expect','unwrap_or','unwrap_or_else','unwrap_or_default','map_or','map_or_else'];
