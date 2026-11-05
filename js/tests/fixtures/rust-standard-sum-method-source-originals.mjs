// Independent original programs retained separately from shared policy controls.
// No imports, execution or observations during source drafting.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustStandardSumMethodSourceOriginals=[
 {name:'Option observers retain both constructor branches',sourceVersionRange,stdout:'true\nfalse\nfalse\ntrue\n',singleCallOwners:[],source:`
fn present()->Option<u8>{Some(7)} fn absent()->Option<u8>{None}
fn main(){println!("{}",present().is_some());println!("{}",present().is_none());println!("{}",absent().is_some());println!("{}",absent().is_none());}
`},
 {name:'Result observers retain both constructor branches',sourceVersionRange,stdout:'true\nfalse\nfalse\ntrue\n',singleCallOwners:[],source:`
fn good()->Result<u8,u16>{Ok(7)} fn bad()->Result<u8,u16>{Err(5)}
fn main(){println!("{}",good().is_ok());println!("{}",good().is_err());println!("{}",bad().is_ok());println!("{}",bad().is_err());}
`},
 {name:'unwrap_or evaluates the receiver then the default even on success',sourceVersionRange,stdout:'receiver\ndefault\n7\nabsent\ndefault\n9\n',singleCallOwners:[],source:`
fn present()->Option<u8>{println!("receiver");Some(7)}
fn absent()->Option<u8>{println!("absent");None}
fn fallback()->u8{println!("default");9}
fn main(){println!("{}",present().unwrap_or(fallback()));println!("{}",absent().unwrap_or(fallback()));}
`},
 {name:'Option unwrap_or_else invokes only a selected fallback',sourceVersionRange,stdout:'7\nfallback\n9\n',singleCallOwners:[],source:`
fn present()->Option<u8>{Some(7)} fn absent()->Option<u8>{None}
fn fallback()->u8{println!("fallback");9}
fn main(){println!("{}",present().unwrap_or_else(||fallback()));println!("{}",absent().unwrap_or_else(||fallback()));}
`},
 {name:'Result unwrap_or_else passes the original error owner to its callback',sourceVersionRange,stdout:'7\n5\n9\n',singleCallOwners:[],source:`
fn good()->Result<u8,u16>{Ok(7)} fn bad()->Result<u8,u16>{Err(5)}
fn recover(error:u16)->u8{println!("{}",error);9}
fn main(){println!("{}",good().unwrap_or_else(|error|recover(error)));println!("{}",bad().unwrap_or_else(|error|recover(error)));}
`},
 {name:'fallback factories evaluate once while callback invocation stays lazy',sourceVersionRange,stdout:'factory\n7\nfactory\ncallback\n9\n',singleCallOwners:[],source:`
fn present()->Option<u8>{Some(7)} fn absent()->Option<u8>{None}
fn fallback()->u8{println!("callback");9}
fn factory()->fn()->u8{println!("factory");fallback}
fn main(){println!("{}",present().unwrap_or_else(factory()));println!("{}",absent().unwrap_or_else(factory()));}
`},
 {name:'map changes the payload type through an actual named callable',sourceVersionRange,stdout:'mapper\ntrue\nfalse\n',singleCallOwners:[],source:`
fn truth(value:u8)->bool{println!("mapper");value>0}
fn changed(value:Option<u8>)->Option<bool>{value.map(truth)}
fn main(){println!("{}",changed(Some(7)).unwrap_or(false));println!("{}",changed(None).unwrap_or(false));}
`},
 {name:'a named mapper result is inferred without inventing Unit',sourceVersionRange,stdout:'true\n',singleCallOwners:[],source:`
fn truth(value:u8)->bool{value>0}
fn main(){let changed=Some(7u8).map(truth);println!("{}",changed.unwrap_or(false));}
`},
 {name:'and_then preserves absent input and actual distinct success payload',sourceVersionRange,stdout:'mapper\n7\n9\n',singleCallOwners:[],source:`
fn widen(value:u8)->Option<u16>{println!("mapper");Some(u16::from(value))}
fn changed(value:Option<u8>)->Option<u16>{value.and_then(widen)}
fn main(){println!("{}",changed(Some(7)).unwrap_or(9));println!("{}",changed(None).unwrap_or(9));}
`},
 {name:'generic Result aliases compose with a nominal Copy error and map',sourceVersionRange,stdout:'true\nfalse\n',singleCallOwners:[],source:`
#[derive(Clone,Copy)] enum Failure{Missing}
type Work<T>=std::result::Result<T,Failure>;
fn truth(value:u8)->bool{value>0}
fn changed(value:Work<u8>)->Work<bool>{value.map(truth)}
fn main(){println!("{}",changed(Ok(7)).unwrap_or(false));println!("{}",changed(Err(Failure::Missing)).unwrap_or(false));}
`},
];
export const rustStandardSumMethodContextOriginals=[
 {name:'actual constructor payload precedes a family-preserving mapper result context',sourceVersionRange,stdout:'true\n',singleCallOwners:[],source:`
fn truth(value:u8)->bool{value>0u8}
fn step()->Option<bool>{Some(7u8).map(truth)}
fn main(){println!("{}",step().unwrap_or(false));}
`},
 {name:'discarded mapper output ignores an unrelated enclosing return type',sourceVersionRange,stdout:'true\n7\n',singleCallOwners:[],source:`
fn step()->Option<u8>{let mapped=Some(7u8).map(|value|value>0u8);println!("{}",mapped.unwrap_or(false));Some(7u8)}
fn main(){println!("{}",step().unwrap_or(0u8));}
`},
 {name:'discarded binding callback keeps its actual sum result owner',sourceVersionRange,stdout:'true\n7\n',singleCallOwners:[],source:`
fn truth(value:u8)->Option<bool>{Some(value>0u8)}
fn step()->Option<u8>{let mapped=Some(7u8).and_then(truth);println!("{}",mapped.unwrap_or(false));Some(7u8)}
fn main(){println!("{}",step().unwrap_or(0u8));}
`},
];
export const rustStandardSumMethodSourceRefusals=[
 {name:'an and_then callback cannot change the Result error owner',sourceVersionRange,source:'fn wrong(value:u8)->Result<u16,u8>{Ok(u16::from(value))}fn changed(value:Result<u8,u16>)->Result<u16,u16>{value.and_then(wrong)}fn main(){}'},
 {name:'a mapper needs the actual receiver payload parameter type',sourceVersionRange,source:'fn wrong(value:bool)->u8{if value{1}else{0}}fn changed(value:Option<u8>)->Option<u8>{value.map(wrong)}fn main(){}'},
 {name:'observer methods cannot swap Option and Result families',sourceVersionRange,source:'fn wrong(value:Option<u8>)->bool{value.is_ok()}fn main(){}'},
 {name:'a lazy fallback retains its original payload result type',sourceVersionRange,source:'fn wrong(value:Option<u8>)->u8{value.unwrap_or_else(||true)}fn main(){}'},
];
