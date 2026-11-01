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
 {name:'non-Copy nested payload retains its full ownership obligation',source:'fn main() { let value: Option<Option<String>> = Some(Some(String::from("value"))); println!("{}", value.flatten().is_some()); }'},
];
