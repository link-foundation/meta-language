// Independent deferred source literals; no imports, generation or execution.
export const rustStandardOptionPanicMethodDecisions = [
 {operation:'readRustStandardSumMethodArity',arguments:['std::option::Option','unwrap'],expected:0},
 {operation:'readRustStandardSumMethodArity',arguments:['std::option::Option','expect'],expected:1},
 {operation:'readRustStandardSumMethodArity',arguments:['std::result::Result','unwrap'],expected:-1},
 {operation:'readRustStandardSumMethodArity',arguments:['std::result::Result','expect'],expected:-1},
 {operation:'readRustStandardSumMethodResult',arguments:['std::option::Option','unwrap'],expected:'payload'},
 {operation:'readRustStandardSumMethodBranch',arguments:['std::option::Option','Some','expect'],expected:'payload'},
 {operation:'readRustStandardSumMethodBranch',arguments:['std::option::Option','None','unwrap'],expected:'panic'},
 {operation:'readRustStandardSumMethodPanicMessageRole',arguments:['std::option::Option','unwrap'],expected:'standard message'},
 {operation:'readRustStandardSumMethodPanicMessageRole',arguments:['std::option::Option','expect'],expected:'supplied message'},
 {operation:'readRustStandardSumMethodPanicMessageRole',arguments:['std::result::Result','expect'],expected:''},
 {operation:'readRustStandardSumMethodSourceVersionRange',arguments:['std::option::Option','unwrap'],expected:'>=1.90.0 <1.100.0'},
 {operation:'readRustStandardSumMethodSourceVersionRange',arguments:['std::result::Result','unwrap'],expected:''},
 {operation:'readRustStandardSumMethodSourceVersionError',arguments:['std::option::Option','unwrap',90],expected:''},
 {operation:'readRustStandardSumMethodSourceVersionError',arguments:['std::option::Option','expect',99],expected:''},
 {operation:'readRustStandardSumMethodSourceVersionError',arguments:['std::option::Option','expect',89],expected:'Option unwrap and expect source semantics are uncovered outside Rust >=1.90.0 <1.100.0'},
 {operation:'readRustStandardSumMethodSourceVersionError',arguments:['std::option::Option','unwrap',100],expected:'Option unwrap and expect source semantics are uncovered outside Rust >=1.90.0 <1.100.0'},
 {operation:'readRustOptionPanicMessage',arguments:['unwrap','ignored'],expected:'called `Option::unwrap()` on a `None` value'},
 {operation:'readRustOptionPanicMessage',arguments:['expect','raw {unformatted}: λ'],expected:'raw {unformatted}: λ'},
 {operation:'acceptRustStandardSumOwnedPanicReceiver',arguments:['std::option::Option','unwrap','owned'],expected:true},
 {operation:'acceptRustStandardSumOwnedPanicReceiver',arguments:['std::option::Option','expect','owned'],expected:true},
 {operation:'acceptRustStandardSumOwnedPanicReceiver',arguments:['std::option::Option','unwrap','copy'],expected:false},
 {operation:'acceptRustStandardSumOwnedPanicReceiver',arguments:['std::option::Option','map','owned'],expected:false},
 {operation:'acceptRustStandardSumOwnedPanicReceiver',arguments:['std::result::Result','unwrap','owned'],expected:false},
 {operation:'acceptRustStandardSumOwnedPanicReceiver',arguments:['local::Option','expect','owned'],expected:false},
];

// Panic expectations are raw common panic payloads, not compiler-specific
// stderr prefixes, backtraces, hook locations or evidence of source execution.
export const rustStandardOptionPanicMethodOriginals = [
 {name:'present unwrap and expect retain actual payload',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'7\ntrue\n',source:`
fn main() {
    println!("{}", Some::<u8>(7).unwrap());
    println!("{}", Some::<bool>(true).expect("unused {literal}"));
}
`},
 {name:'absent unwrap keeps exact standard message',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'before\n',panic:'called `Option::unwrap()` on a `None` value',source:`
fn main() {
    println!("before");
    println!("{}", None::<u8>.unwrap());
    println!("after");
}
`},
 {name:'expect raw message has no formatting or Debug suffix',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'before\n',panic:'raw {unformatted}: λ',source:`
fn main() {
    println!("before");
    println!("{}", None::<u8>.expect("raw {unformatted}: λ"));
    println!("after");
}
`},
 {name:'present expect evaluates message after receiver once',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'receiver\nmessage\n7\n',source:`
fn receiver() -> Option<u8> { println!("receiver"); Some(7) }
fn main() {
    println!("{}", receiver().expect({ println!("message"); "unused" }));
}
`},
 {name:'absent expect evaluates message before panic',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'receiver\nmessage\n',panic:'computed {literal}',source:`
fn receiver() -> Option<u8> { println!("receiver"); None }
fn main() {
    println!("{}", receiver().expect({ println!("message"); "computed {literal}" }));
    println!("after");
}
`},
];

export const rustStandardOptionPanicMethodRefusals = [
 {name:'unwrap has no source arguments',source:'fn main() { println!("{}", Some::<u8>(7).unwrap(1)); }'},
 {name:'expect requires an actual immutable str source',source:'fn main() { println!("{}", Some::<u8>(7).expect(1u8)); }'},
 {name:'Result unwrap still needs its actual Debug error model',source:'fn main() { println!("{}", Ok::<u8,u8>(7).unwrap()); }'},
 {name:'Result expect still needs its actual Debug error model',source:'fn main() { println!("{}", Ok::<u8,u8>(7).expect("message")); }'},
];
rustStandardOptionPanicMethodOriginals.push(
 {name:'owned Option unwrap and expect transfer String and vector payloads',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'text\n2\n',source:`
fn main() {let text:Option<String>=Some(String::from("text"));let values:Option<Vec<u8>>=Some(vec![7u8,9u8]);println!("{}",text.unwrap());println!("{}",values.expect("unused").len());}
`},
 {name:'absent owned Option unwrap keeps the standard panic',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'before\n',panic:'called `Option::unwrap()` on a `None` value',source:`
fn main() {let value:Option<String>=None;println!("before");println!("{}",value.unwrap());println!("after");}
`},
 {name:'absent owned Option expect keeps a literal panic message',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'before\n',panic:'owned {literal}: λ',source:`
fn main() {let value:Option<String>=None;println!("before");println!("{}",value.expect("owned {literal}: λ"));println!("after");}
`},
 {name:'owned expect moves receiver before evaluating message once',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'receiver\nmessage\ntext\n',source:`
fn receiver()->Option<String>{println!("receiver");Some(String::from("text"))}fn main() {println!("{}",receiver().expect({println!("message");"unused"}));}
`});
rustStandardOptionPanicMethodRefusals.push(
 {name:'owned Option unwrap cannot reuse its moved receiver',source:'fn main() {let value:Option<String>=Some(String::from("text"));let text=value.unwrap();println!("{}",value.is_some());}'},
 {name:'owned Option expect moves before message reads receiver',source:'fn main() {let value:Option<String>=Some(String::from("text"));let text=value.expect({println!("{}",value.is_some());"message"});}'});

// Primary std endpoint references supplied by the parent source review.
export const rustStandardOptionPanicMethodSources = [
 {version:'1.90.0',url:'https://raw.githubusercontent.com/rust-lang/rust/1.90.0/library/core/src/option.rs',unwrapFailedLine:2067,expectFailedLine:2074},
 {version:'1.99.0',url:'https://raw.githubusercontent.com/rust-lang/rust/1.99.0/library/core/src/option.rs',unwrapFailedLine:2181,expectFailedLine:2189},
];
