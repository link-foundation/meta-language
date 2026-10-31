// Deferred independent literals only. No imports, parser calls or execution.
export const rustStandardSumMethodDecisions = [
 {operation:'readRustStandardSumMethodArity',arguments:['std::option::Option','is_some'],expected:0},
 {operation:'readRustStandardSumMethodArity',arguments:['std::result::Result','is_err'],expected:0},
 {operation:'readRustStandardSumMethodArity',arguments:['std::option::Option','unwrap_or'],expected:1},
 {operation:'readRustStandardSumMethodArity',arguments:['std::result::Result','map'],expected:1},
 {operation:'readRustStandardSumMethodArity',arguments:['std::option::Option','is_ok'],expected:-1},
 {operation:'readRustStandardSumMethodArity',arguments:['local::Option','is_some'],expected:-1},
 {operation:'readRustStandardSumMethodArity',arguments:['std::result::Result','unwrap'],expected:-1},
 {operation:'readRustStandardSumMethodCallbackArgument',arguments:['std::option::Option','unwrap_or_else'],expected:-1},
 {operation:'readRustStandardSumMethodCallbackArgument',arguments:['std::result::Result','unwrap_or_else'],expected:1},
 {operation:'readRustStandardSumMethodCallbackArgument',arguments:['std::result::Result','and_then'],expected:0},
 {operation:'readRustStandardSumMethodCallbackArgument',arguments:['std::result::Result','unwrap_or'],expected:-2},
 {operation:'readRustStandardSumMethodResult',arguments:['std::option::Option','is_none'],expected:'boolean'},
 {operation:'readRustStandardSumMethodResult',arguments:['std::option::Option','unwrap_or_else'],expected:'payload'},
 {operation:'readRustStandardSumMethodResult',arguments:['std::result::Result','map'],expected:'mapped sum'},
 {operation:'readRustStandardSumMethodResult',arguments:['std::option::Option','and_then'],expected:'bound sum'},
 {operation:'readRustStandardSumMethodBranch',arguments:['std::option::Option','None','is_none'],expected:'true'},
 {operation:'readRustStandardSumMethodBranch',arguments:['std::option::Option','Some','is_none'],expected:'false'},
 {operation:'readRustStandardSumMethodBranch',arguments:['std::result::Result','Ok','unwrap_or'],expected:'payload'},
 {operation:'readRustStandardSumMethodBranch',arguments:['std::result::Result','Err','unwrap_or'],expected:'default'},
 {operation:'readRustStandardSumMethodBranch',arguments:['std::option::Option','None','unwrap_or_else'],expected:'invoke fallback'},
 {operation:'readRustStandardSumMethodBranch',arguments:['std::result::Result','Err','unwrap_or_else'],expected:'invoke fallback'},
 {operation:'readRustStandardSumMethodBranch',arguments:['std::option::Option','Some','map'],expected:'construct mapped'},
 {operation:'readRustStandardSumMethodBranch',arguments:['std::result::Result','Err','map'],expected:'forward failure'},
 {operation:'readRustStandardSumMethodBranch',arguments:['std::result::Result','Ok','and_then'],expected:'invoke mapper'},
 {operation:'readRustStandardSumMethodBranch',arguments:['std::result::Result','Some','map'],expected:''},
 {operation:'readRustStandardSumMethodCallbackError',arguments:['function',1,1,0,true,true],expected:''},
 {operation:'readRustStandardSumMethodCallbackError',arguments:['function',0,1,0,true,true],expected:'a standard sum callback needs its complete direct Copy callable and actual parameter/result source types'},
];

export const rustStandardSumMethodOriginals = [
 {name:'named mapper retains its actual Boolean result',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'true\nfalse\ntrue\n',source:`
fn predicate(value: u8) -> bool { value > 3 }
fn bound(value: u8) -> Option<bool> { Some(value > 3) }
fn main() {
    println!("{}", Some::<u8>(7).map(predicate).unwrap_or(false));
    println!("{}", None::<u8>.map(predicate).unwrap_or(false));
    println!("{}", Some::<u8>(7).and_then(bound).unwrap_or(false));
}
`},
 {name:'eager defaults and lazy invocation preserve source order',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'receiver\ndefault\n7\nreceiver\nfactory\n7\nreceiver\nfactory\ndefault\n9\n',source:`
fn receiver(value: Option<u8>) -> Option<u8> { println!("receiver"); value }
fn fallback() -> u8 { println!("default"); 9 }
fn factory() -> fn() -> u8 { println!("factory"); fallback }
fn main() {
    println!("{}", receiver(Some(7)).unwrap_or(fallback()));
    println!("{}", receiver(Some(7)).unwrap_or_else(factory()));
    println!("{}", receiver(None).unwrap_or_else(factory()));
}
`},
 {name:'Result callbacks retain exact error parameter and branch',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'7\nfallback\n9\ntrue\nfalse\n',source:`
fn fallback(error: u16) -> u8 { println!("fallback"); if error > 0 { 9 } else { 0 } }
fn predicate(value: u8) -> bool { value > 3 }
fn main() {
    println!("{}", Ok::<u8,u16>(7).unwrap_or_else(fallback));
    println!("{}", Err::<u8,u16>(5).unwrap_or_else(fallback));
    println!("{}", Ok::<u8,u16>(7).map(predicate).unwrap_or(false));
    println!("{}", Err::<u8,u16>(5).map(predicate).unwrap_or(false));
}
`},
];
