// Independent deferred literals. No source, control or oracle executes here.
export const rustStandardSumNestedDecisions = [
 {operation:'readRustStandardSumNestedArity',arguments:['std::option::Option','flatten'],expected:0},
 {operation:'readRustStandardSumNestedArity',arguments:['std::option::Option','transpose'],expected:0},
 {operation:'readRustStandardSumNestedArity',arguments:['std::result::Result','transpose'],expected:0},
 {operation:'readRustStandardSumNestedArity',arguments:['std::result::Result','flatten'],expected:-1},
 {operation:'readRustStandardSumNestedProfile',arguments:['std::option::Option','flatten','std::option::Option'],expected:'option flatten'},
 {operation:'readRustStandardSumNestedProfile',arguments:['std::option::Option','transpose','std::result::Result'],expected:'option result transpose'},
 {operation:'readRustStandardSumNestedProfile',arguments:['std::result::Result','transpose','std::option::Option'],expected:'result option transpose'},
 {operation:'readRustStandardSumNestedProfile',arguments:['std::option::Option','transpose','std::option::Option'],expected:''},
 {operation:'readRustStandardSumNestedProfile',arguments:['local::Option','flatten','std::option::Option'],expected:''},
 {operation:'planRustStandardSumNestedResult',arguments:['option flatten'],expected:['inner']},
 {operation:'planRustStandardSumNestedResult',arguments:['option result transpose'],expected:['inner success','std::option::Option','inner error','std::result::Result']},
 {operation:'planRustStandardSumNestedResult',arguments:['result option transpose'],expected:['inner success','outer error','std::result::Result','std::option::Option']},
 {operation:'planRustStandardSumNestedBranch',arguments:['option flatten','std::option::Option','None',false],expected:['result None']},
 {operation:'planRustStandardSumNestedBranch',arguments:['option flatten','std::option::Option','Some',false],expected:['match inner']},
 {operation:'planRustStandardSumNestedBranch',arguments:['option flatten','std::option::Option','None',true],expected:['result None']},
 {operation:'planRustStandardSumNestedBranch',arguments:['option flatten','std::option::Option','Some',true],expected:['payload','result Some']},
 {operation:'planRustStandardSumNestedBranch',arguments:['option result transpose','std::option::Option','None',false],expected:['child None','result Ok']},
 {operation:'planRustStandardSumNestedBranch',arguments:['option result transpose','std::result::Result','Ok',true],expected:['payload','child Some','result Ok']},
 {operation:'planRustStandardSumNestedBranch',arguments:['option result transpose','std::result::Result','Err',true],expected:['payload','result Err']},
 {operation:'planRustStandardSumNestedBranch',arguments:['result option transpose','std::result::Result','Err',false],expected:['payload','child Err','result Some']},
 {operation:'planRustStandardSumNestedBranch',arguments:['result option transpose','std::option::Option','None',true],expected:['result None']},
 {operation:'planRustStandardSumNestedBranch',arguments:['result option transpose','std::option::Option','Some',true],expected:['payload','child Ok','result Some']},
 {operation:'readRustStandardSumNestedConstructionPosition',arguments:['result Some'],expected:-1},
 {operation:'readRustStandardSumNestedConstructionPosition',arguments:['child Err'],expected:0},
 {operation:'readRustStandardSumNestedConstructionPosition',arguments:['payload'],expected:-2},
 {operation:'readRustStandardSumNestedConstructionVariant',arguments:['child Some'],expected:'Some'},
 {operation:'readRustStandardSumNestedSourceVersionRange',arguments:['std::option::Option','flatten'],expected:'>=1.90.0 <1.100.0'},
 {operation:'readRustStandardSumNestedSourceVersionError',arguments:['std::option::Option','transpose',90],expected:''},
 {operation:'readRustStandardSumNestedSourceVersionError',arguments:['std::result::Result','transpose',99],expected:''},
 {operation:'readRustStandardSumNestedSourceVersionError',arguments:['std::option::Option','flatten',89],expected:'nested sum source semantics are uncovered outside Rust >=1.90.0 <1.100.0'},
 {operation:'readRustStandardSumNestedSourceVersionError',arguments:['std::result::Result','transpose',100],expected:'nested sum source semantics are uncovered outside Rust >=1.90.0 <1.100.0'},
];

export const rustStandardSumNestedOriginals = [
 {name:'flatten retains all actual outer and inner absence cases',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'0\n0\n7\n',source:`
fn flatten(value: Option<Option<u8>>) -> Option<u8> { value.flatten() }
fn main() {
    println!("{}", flatten(None).unwrap_or(0u8));
    println!("{}", flatten(Some(None)).unwrap_or(0u8));
    println!("{}", flatten(Some(Some(7))).unwrap_or(0u8));
}
`},
 {name:'flatten removes exactly one actual sum layer',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'1\n0\n2\n',source:`
fn classify(value: Option<Option<u8>>) -> u8 { match value { None => 0, Some(None) => 1, Some(Some(_)) => 2 } }
fn main() {
    let first: Option<Option<Option<u8>>> = Some(Some(None));
    let second: Option<Option<Option<u8>>> = Some(None);
    let third: Option<Option<Option<u8>>> = Some(Some(Some(7)));
    println!("{}", classify(first.flatten()));
    println!("{}", classify(second.flatten()));
    println!("{}", classify(third.flatten()));
}
`},
 {name:'Option Result transpose distinguishes None success and error',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'0\n7\n9\n',source:`
fn flip(value: Option<Result<u8,u16>>) -> Result<Option<u8>,u16> { value.transpose() }
fn show(value: Result<Option<u8>,u16>) -> u8 {
    match value { Ok(None) => 0, Ok(Some(number)) => number, Err(error) => if error > 0 { 9 } else { 1 } }
}
fn main() {
    println!("{}", show(flip(None)));
    println!("{}", show(flip(Some(Ok(7)))));
    println!("{}", show(flip(Some(Err(9)))));
}
`},
 {name:'Result Option transpose distinguishes absent success and error',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'0\n7\n9\n',source:`
fn flip(value: Result<Option<u8>,u16>) -> Option<Result<u8,u16>> { value.transpose() }
fn show(value: Option<Result<u8,u16>>) -> u8 {
    match value { None => 0, Some(Ok(number)) => number, Some(Err(error)) => if error > 0 { 9 } else { 1 } }
}
fn main() {
    println!("{}", show(flip(Ok(None))));
    println!("{}", show(flip(Ok(Some(7)))));
    println!("{}", show(flip(Err(9))));
}
`},
 {name:'nested operations capture each receiver once',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'inner\nouter\n7\noption receiver\n9\nresult receiver\n9\n',source:`
fn inner() -> Option<u8> { println!("inner"); Some(7) }
fn outer(value: Option<Option<u8>>) -> Option<Option<u8>> { println!("outer"); value }
fn option_receiver(value: Option<Result<u8,u16>>) -> Option<Result<u8,u16>> { println!("option receiver"); value }
fn result_receiver(value: Result<Option<u8>,u16>) -> Result<Option<u8>,u16> { println!("result receiver"); value }
fn main() {
    println!("{}", outer(Some(inner())).flatten().unwrap_or(0u8));
    println!("{}", option_receiver(Some(Err(9))).transpose().err().unwrap_or(0u16));
    println!("{}", result_receiver(Err(9)).transpose().unwrap_or(Ok::<u8,u16>(0)).err().unwrap_or(0u16));
}
`},
 {name:'nominal Copy payload and error retain complete declared owners',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'7\n0\n9\n',source:`
#[derive(Clone,Copy)] struct Value { number: u8 }
#[derive(Clone,Copy)] struct Failure { number: u16 }
type Nested = Option<Result<Value,Failure>>;
fn flip(value: Nested) -> Result<Option<Value>,Failure> { value.transpose() }
fn show(value: Result<Option<Value>,Failure>) -> u8 {
    match value { Ok(Some(value)) => value.number, Ok(None) => 0, Err(error) => if error.number > 0 { 9 } else { 1 } }
}
fn main() {
    println!("{}", show(flip(Some(Ok(Value { number: 7 })))));
    println!("{}", show(flip(None)));
    println!("{}", show(flip(Some(Err(Failure { number: 9 })))));
}
`},
 {name:'implicit Some payload proof precedes same-family ambient result',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'7\n',source:`
fn changed() -> Option<u8> { Some(Some(7u8)).flatten() }
fn main() { println!("{}", changed().unwrap_or(0u8)); }
`},
 {name:'both transpose directions return original nested constructor meanings',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'0\n7\n9\n',source:`
fn round_trip(value: Option<Result<u8,u16>>) -> Option<Result<u8,u16>> { value.transpose().transpose() }
fn show(value: Option<Result<u8,u16>>) -> u8 {
    match value { None => 0, Some(Ok(number)) => number, Some(Err(error)) => if error > 0 { 9 } else { 1 } }
}
fn main() {
    println!("{}", show(round_trip(None)));
    println!("{}", show(round_trip(Some(Ok(7)))));
    println!("{}", show(round_trip(Some(Err(9)))));
}
`},
];

export const rustStandardSumNestedRefusals = [
 {name:'flatten requires an actual inner standard Option',source:'fn main() { println!("{}", Some::<u8>(7).flatten().is_some()); }'},
 {name:'Option transpose requires actual inner Result',source:'fn main() { println!("{}", Some::<Option<u8>>(Some(7)).transpose().is_some()); }'},
 {name:'Result transpose requires actual inner Option',source:'fn main() { println!("{}", Ok::<Result<u8,u16>,u32>(Ok(7)).transpose().is_some()); }'},
 {name:'flatten has no source arguments',source:'fn main() { println!("{}", Some::<Option<u8>>(Some(7)).flatten(1u8).is_some()); }'},
 {name:'borrowed receiver retains the separate indirection obligation',source:'fn main() { let value: Option<Option<u8>> = Some(Some(7)); println!("{}", (&value).flatten().is_some()); }'},
];
export const rustStandardSumNestedPreviousBoundaries=[{
 previousCollection:'rustStandardSumNestedRefusals',previousIndex:5,
 previousDiagnosticKinds:['syntax','type','unsupported'],
 previousTranslationExpectation:'refuse non-Copy nested ownership',
 row:{name:'non-Copy nested payload retains its full ownership obligation',source:'fn main() { let value: Option<Option<String>> = Some(Some(String::from("value"))); println!("{}", value.flatten().is_some()); }'},
}];
rustStandardSumNestedOriginals.push({...rustStandardSumNestedPreviousBoundaries[0].row,
 sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'true\n',singleCallOwners:[],
 originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',outcome:'accept',category:'valid Rust source'},
 expectationHistory:[{previousCollection:'rustStandardSumNestedRefusals',previousIndex:5,translationExpectation:'refuse non-Copy nested ownership'}]},
 {name:'owned flatten preserves payload and both empty branches',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'value\nfalse\nfalse\n',source:`
fn main(){let a:Option<Option<String>>=Some(Some(String::from("value")));let b:Option<Option<String>>=Some(None);let c:Option<Option<String>>=None;println!("{}",a.flatten().unwrap_or(String::from("other")));println!("{}",b.flatten().is_some());println!("{}",c.flatten().is_some());}
`},
 {name:'owned Option transpose preserves each constructor and error owner',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'value\nerror\nfalse\n',source:`
fn main(){let a:Option<Result<String,String>>=Some(Ok(String::from("value")));let b:Option<Result<String,String>>=Some(Err(String::from("error")));let c:Option<Result<String,String>>=None;let first=match a.transpose(){Ok(Some(text))=>text,_=>String::from("wrong")};let second=match b.transpose(){Err(text)=>text,_=>String::from("wrong")};println!("{}",first);println!("{}",second);println!("{}",c.transpose().ok().flatten().is_some());}
`},
 {name:'owned Result transpose retains Copy success and owned error branches',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'7\nerror\nfalse\n',source:`
fn main(){let a:Result<Option<u8>,String>=Ok(Some(7u8));let b:Result<Option<u8>,String>=Err(String::from("error"));let c:Result<Option<u8>,String>=Ok(None);let number=match a.transpose(){Some(Ok(value))=>value,_=>0u8};let error=match b.transpose(){Some(Err(text))=>text,_=>String::from("wrong")};println!("{}",number);println!("{}",error);println!("{}",c.transpose().is_some());}
`},
 {name:'owned nested function receiver evaluates once',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'receiver\nvalue\n',source:`
fn make()->Option<Option<String>>{println!("receiver");Some(Some(String::from("value")))}fn main(){println!("{}",make().flatten().unwrap_or(String::from("other")));}
`});
rustStandardSumNestedRefusals.push(...[
 {name:'flatten consumes its actual owned receiver',source:'fn main(){let value:Option<Option<String>>=Some(Some(String::from("text")));let inner=value.flatten();println!("{}",value.is_some());}'},
 {name:'transpose consumes its actual owned receiver',source:'fn main(){let value:Result<Option<u8>,String>=Ok(Some(7u8));let flipped=value.transpose();println!("{}",value.is_ok());}'},
 {name:'owned flatten cannot move through shared reference',source:'fn main(){let value:Option<Option<String>>=Some(Some(String::from("text")));let inner=(&value).flatten();}'},
 {name:'owned nested result does not grant a second payload move',source:'fn main(){let value:Option<Option<String>>=Some(Some(String::from("text")));let inner=value.flatten();let text=inner.unwrap_or(String::from("other"));let again=inner.unwrap_or(String::from("another"));}'}
].map(row=>({...row,sourceVersionRange:'>=1.90.0 <1.100.0',
 originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',outcome:'reject',category:'invalid Rust ownership'}})));
