// Deferred independent literals only; no imports or represented execution.
export const rustStandardSumConversionDecisions = [
 {operation:'readRustStandardSumConversionArity',arguments:['std::option::Option','ok_or'],expected:1},
 {operation:'readRustStandardSumConversionArity',arguments:['std::option::Option','ok_or_else'],expected:1},
 {operation:'readRustStandardSumConversionArity',arguments:['std::result::Result','ok'],expected:0},
 {operation:'readRustStandardSumConversionArity',arguments:['std::result::Result','err'],expected:0},
 {operation:'readRustStandardSumConversionArity',arguments:['std::result::Result','map_err'],expected:1},
 {operation:'readRustStandardSumConversionArity',arguments:['std::option::Option','map_err'],expected:-1},
 {operation:'readRustStandardSumConversionArity',arguments:['local::Result','ok'],expected:-1},
 {operation:'readRustStandardSumConversionOwner',arguments:['std::option::Option','ok_or'],expected:'std::result::Result'},
 {operation:'readRustStandardSumConversionOwner',arguments:['std::result::Result','ok'],expected:'std::option::Option'},
 {operation:'readRustStandardSumConversionOwner',arguments:['std::result::Result','err'],expected:'std::option::Option'},
 {operation:'readRustStandardSumConversionOwner',arguments:['std::result::Result','map_err'],expected:'std::result::Result'},
 {operation:'planRustStandardSumConversionArguments',arguments:['std::option::Option','ok_or'],expected:[0,-1]},
 {operation:'planRustStandardSumConversionArguments',arguments:['std::option::Option','ok_or_else'],expected:[0,-1]},
 {operation:'planRustStandardSumConversionArguments',arguments:['std::result::Result','ok'],expected:[0]},
 {operation:'planRustStandardSumConversionArguments',arguments:['std::result::Result','err'],expected:[1]},
 {operation:'planRustStandardSumConversionArguments',arguments:['std::result::Result','map_err'],expected:[0,-1]},
 {operation:'readRustStandardSumConversionCallbackArgument',arguments:['std::option::Option','ok_or'],expected:-2},
 {operation:'readRustStandardSumConversionCallbackArgument',arguments:['std::option::Option','ok_or_else'],expected:-1},
 {operation:'readRustStandardSumConversionCallbackArgument',arguments:['std::result::Result','map_err'],expected:1},
 {operation:'readRustStandardSumConversionConstructor',arguments:['std::option::Option','Some','ok_or'],expected:'Ok'},
 {operation:'readRustStandardSumConversionConstructor',arguments:['std::option::Option','None','ok_or'],expected:'Err'},
 {operation:'readRustStandardSumConversionConstructor',arguments:['std::option::Option','Some','ok_or_else'],expected:'Ok'},
 {operation:'readRustStandardSumConversionConstructor',arguments:['std::option::Option','None','ok_or_else'],expected:'Err'},
 {operation:'readRustStandardSumConversionConstructor',arguments:['std::result::Result','Ok','ok'],expected:'Some'},
 {operation:'readRustStandardSumConversionConstructor',arguments:['std::result::Result','Err','ok'],expected:'None'},
 {operation:'readRustStandardSumConversionConstructor',arguments:['std::result::Result','Ok','err'],expected:'None'},
 {operation:'readRustStandardSumConversionConstructor',arguments:['std::result::Result','Err','err'],expected:'Some'},
 {operation:'readRustStandardSumConversionConstructor',arguments:['std::result::Result','Ok','map_err'],expected:'Ok'},
 {operation:'readRustStandardSumConversionConstructor',arguments:['std::result::Result','Err','map_err'],expected:'Err'},
 {operation:'readRustStandardSumConversionValue',arguments:['std::option::Option','Some','ok_or'],expected:'payload'},
 {operation:'readRustStandardSumConversionValue',arguments:['std::option::Option','None','ok_or'],expected:'argument'},
 {operation:'readRustStandardSumConversionValue',arguments:['std::option::Option','Some','ok_or_else'],expected:'payload'},
 {operation:'readRustStandardSumConversionValue',arguments:['std::option::Option','None','ok_or_else'],expected:'invoke callback'},
 {operation:'readRustStandardSumConversionValue',arguments:['std::result::Result','Ok','ok'],expected:'payload'},
 {operation:'readRustStandardSumConversionValue',arguments:['std::result::Result','Err','ok'],expected:'empty'},
 {operation:'readRustStandardSumConversionValue',arguments:['std::result::Result','Ok','err'],expected:'empty'},
 {operation:'readRustStandardSumConversionValue',arguments:['std::result::Result','Err','err'],expected:'payload'},
 {operation:'readRustStandardSumConversionValue',arguments:['std::result::Result','Ok','map_err'],expected:'payload'},
 {operation:'readRustStandardSumConversionValue',arguments:['std::result::Result','Err','map_err'],expected:'invoke callback'},
 {operation:'readRustStandardSumConversionValue',arguments:['std::option::Option','Err','ok_or'],expected:''},
 {operation:'readRustStandardSumConversionCallbackError',arguments:['function',1,1,0,true,true],expected:''},
 {operation:'readRustStandardSumConversionCallbackError',arguments:['function',0,1,0,true,true],expected:'a sum conversion callback needs its actual direct Copy callable and complete parameter/result source origins'},
 {operation:'readRustStandardSumConversionCallbackError',arguments:['function',1,1,1,true,true],expected:'a sum conversion callback needs its actual direct Copy callable and complete parameter/result source origins'},
 {operation:'readRustStandardSumConversionCallbackError',arguments:['function',1,1,0,true,false],expected:'a sum conversion callback needs its actual direct Copy callable and complete parameter/result source origins'},
 {operation:'readRustStandardSumConversionSourceVersionRange',arguments:['std::option::Option','ok_or'],expected:'>=1.90.0 <1.100.0'},
 {operation:'readRustStandardSumConversionSourceVersionError',arguments:['std::option::Option','ok_or',90],expected:''},
 {operation:'readRustStandardSumConversionSourceVersionError',arguments:['std::result::Result','map_err',99],expected:''},
 {operation:'readRustStandardSumConversionSourceVersionError',arguments:['std::option::Option','ok_or_else',89],expected:'standard sum conversion source semantics are uncovered outside Rust >=1.90.0 <1.100.0'},
 {operation:'readRustStandardSumConversionSourceVersionError',arguments:['std::result::Result','err',100],expected:'standard sum conversion source semantics are uncovered outside Rust >=1.90.0 <1.100.0'},
];

export const rustStandardSumConversionOriginals = [
 {name:'ok_or captures receiver then eager error on both branches',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'receiver\nerror\n7\nreceiver\nerror\n0\n',source:`
fn receiver(value: Option<u8>) -> Option<u8> { println!("receiver"); value }
fn error() -> u16 { println!("error"); 9 }
fn show(value: Result<u8,u16>) -> u8 { match value { Ok(number) => number, Err(error) => if error > 0 { 0 } else { 1 } } }
fn main() {
    println!("{}", show(receiver(Some(7)).ok_or(error())));
    println!("{}", show(receiver(None).ok_or(error())));
}
`},
 {name:'ok_or_else constructs callable eagerly and invokes only absence',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'receiver\nfactory\n7\nreceiver\nfactory\nfallback\n0\n',source:`
fn receiver(value: Option<u8>) -> Option<u8> { println!("receiver"); value }
fn fallback() -> u16 { println!("fallback"); 9 }
fn factory() -> fn() -> u16 { println!("factory"); fallback }
fn show(value: Result<u8,u16>) -> u8 { match value { Ok(number) => number, Err(_) => 0 } }
fn main() {
    println!("{}", show(receiver(Some(7)).ok_or_else(factory())));
    println!("{}", show(receiver(None).ok_or_else(factory())));
}
`},
 {name:'Result ok and err retain the selected actual child owners',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'7\n0\n0\n9\n',source:`
fn main() {
    println!("{}", Ok::<u8,u16>(7).ok().unwrap_or(0u8));
    println!("{}", Err::<u8,u16>(9).ok().unwrap_or(0u8));
    println!("{}", Ok::<u8,u16>(7).err().unwrap_or(0u16));
    println!("{}", Err::<u8,u16>(9).err().unwrap_or(0u16));
}
`},
 {name:'discarded Result child still evaluates its original producer',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'success\ntrue\nfailure\ntrue\n',source:`
fn success() -> u8 { println!("success"); 7 }
fn failure() -> u16 { println!("failure"); 9 }
fn main() {
    println!("{}", Ok::<u8,u16>(success()).err().is_none());
    println!("{}", Err::<u8,u16>(failure()).ok().is_none());
}
`},
 {name:'map_err factory evaluates on both branches and maps only error',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'factory\n7\nfactory\nmapped\ntrue\n',source:`
fn mapper(error: u16) -> bool { println!("mapped"); error > 3 }
fn factory() -> fn(u16) -> bool { println!("factory"); mapper }
fn main() {
    println!("{}", Ok::<u8,u16>(7).map_err(factory()).unwrap_or(0u8));
    println!("{}", Err::<u8,u16>(9).map_err(factory()).err().unwrap_or(false));
}
`},
 {name:'parameter-only lambda preserves actual error context and new Boolean result',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'true\n11\n1\n',source:`
fn main() {
    println!("{}", Err::<u8,u16>(9).map_err(|error| error > 3).err().unwrap_or(false));
    println!("{}", None::<u8>.ok_or_else(|| 11u16).err().unwrap_or(0u16));
    println!("{}", Some::<u8>(1).ok_or_else(|| 11u16).ok().unwrap_or(0u8));
}
`},
 {name:'From mapper uses real input proof without an invented result slot',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'7\n',source:`
fn main() {
    println!("{}", Err::<bool,u8>(7).map_err(u16::from).err().unwrap_or(0u16));
}
`},
 {name:'sum aliases retain full conversion family and payload owners',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'7\n9\n',source:`
type Maybe = Option<u8>;
type Outcome = Result<u8,u16>;
fn convert(value: Maybe) -> Outcome { value.ok_or(9u16) }
fn main() {
    println!("{}", convert(Some(7)).ok().unwrap_or(0u8));
    println!("{}", convert(None).err().unwrap_or(0u16));
}
`},
 {name:'family-changing postfix does not inherit unrelated constructor result context',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'7\n',source:`
fn changed() -> Result<u8,u16> { Some(7u8).ok_or(5u16) }
fn main() { println!("{}", changed().ok().unwrap_or(0u8)); }
`},
 {name:'discarded local conversion does not infer error from enclosing Option result',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'discarded error\ntrue\n',source:`
fn changed() -> Option<bool> {
    let _ = Some(7u8).ok_or({ println!("discarded error"); 5u16 });
    Some(true)
}
fn main() { println!("{}", changed().unwrap_or(false)); }
`},
 {name:'explicit callback result remains its actual declared type',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'11\n11\n',source:`
fn main() {
    println!("{}", None::<u8>.ok_or_else(|| -> u16 { 11 }).err().unwrap_or(0u16));
    println!("{}", Err::<bool,u8>(9).map_err(|error| -> u16 { if error > 3 { 11 } else { 0 } }).err().unwrap_or(0u16));
}
`},
];

export const rustStandardSumConversionRefusals = [
 {name:'map_err must receive the actual error parameter',source:'fn main() { println!("{}", Err::<u8,u16>(9).map_err(|error: u8| error).is_err()); }'},
 {name:'ok_or_else is zero-argument and cannot receive a payload mapper',source:'fn main() { println!("{}", None::<u8>.ok_or_else(|value: u8| value).is_err()); }'},
 {name:'ok_or requires its eager error expression',source:'fn main() { println!("{}", Some::<u8>(7).ok_or().is_ok()); }'},
 {name:'unconstrained unsuffixed callback error needs real result inference',source:'fn main() { let value = None::<u8>.ok_or_else(|| 9); println!("{}", value.is_err()); }'},
 {name:'non-Copy callback error requires full owned result semantics',source:'fn main() { println!("{}", None::<u8>.ok_or_else(|| String::from("error")).is_err()); }'},
 {name:'borrowed error arguments require a separate indirect sum profile',source:'fn main() { println!("{}", None::<u8>.ok_or(&9u16).is_err()); }'},
 {name:'mutable captures retain their existing source refusal',source:'fn main() { let mut number = 0u16; println!("{}", None::<u8>.ok_or_else(|| { number = number + 1; number }).is_err()); }'},
];
