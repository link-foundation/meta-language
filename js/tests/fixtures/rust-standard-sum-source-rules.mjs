// Independent literal inputs for a future Links fixture author. No imports,
// represented-program execution, policy invocation or expected-value generation.
// These controls have not run; the new source policy is not frontend support.
export const rustStandardSumSourceDecisions = [
 {operation:'readRustStandardSumSourceOwner',arguments:['std::option::Option',true,0],expected:'std::option::Option'},
 {operation:'readRustStandardSumSourceOwner',arguments:['core::result::Result',true,0],expected:'std::result::Result'},
 {operation:'readRustStandardSumSourceOwner',arguments:['Option',true,0],expected:''},
 {operation:'readRustStandardSumSourceOwner',arguments:['local::Option',true,0],expected:''},
 {operation:'readRustStandardSumSourceOwner',arguments:['std::option::Option',false,0],expected:''},
 {operation:'readRustStandardSumSourceOwner',arguments:['std::option::Option',true,1],expected:''},
 {operation:'readRustStandardSumSourceArgumentCount',arguments:['std::option::Option'],expected:1},
 {operation:'readRustStandardSumSourceArgumentCount',arguments:['std::result::Result'],expected:2},
 {operation:'readRustStandardSumSourceArgumentCount',arguments:['local::Result'],expected:-1},
 {operation:'acceptRustStandardSumSourceArguments',arguments:['std::option::Option',['checked nested Option<u8>'],[true]],expected:true},
 {operation:'acceptRustStandardSumSourceArguments',arguments:['std::result::Result',['checked u8','checked u8'],[true,true]],expected:true},
 {operation:'acceptRustStandardSumSourceArguments',arguments:['std::result::Result',['checked u8'],[true]],expected:false},
 {operation:'acceptRustStandardSumSourceArguments',arguments:['std::result::Result',['checked u8',''],[true,true]],expected:false},
 {operation:'acceptRustStandardSumSourceArguments',arguments:['std::result::Result',['checked u8','checked String'],[true,false]],expected:false},
 {operation:'acceptRustStandardSumSourceArguments',arguments:['local::Option',['checked u8'],[true]],expected:false},
 {operation:'readRustStandardSumSourceConstructors',arguments:['std::option::Option'],expected:['None','Some']},
 {operation:'readRustStandardSumSourceConstructors',arguments:['std::result::Result'],expected:['Ok','Err']},
 {operation:'readRustStandardSumSourceConstructors',arguments:['local::Result'],expected:[]},
 {operation:'readRustStandardSumSourceFieldArgument',arguments:['std::option::Option','None'],expected:-1},
 {operation:'readRustStandardSumSourceFieldArgument',arguments:['std::option::Option','Some'],expected:0},
 {operation:'readRustStandardSumSourceFieldArgument',arguments:['std::result::Result','Ok'],expected:0},
 {operation:'readRustStandardSumSourceFieldArgument',arguments:['std::result::Result','Err'],expected:1},
 {operation:'readRustStandardSumSourceFieldArgument',arguments:['std::option::Option','Err'],expected:-2},
 {operation:'readRustStandardSumSourceConstructorArity',arguments:['std::option::Option','None'],expected:0},
 {operation:'readRustStandardSumSourceConstructorArity',arguments:['std::option::Option','Some'],expected:1},
 {operation:'readRustStandardSumSourceConstructorArity',arguments:['std::result::Result','Err'],expected:1},
 {operation:'readRustStandardSumSourceConstructorArity',arguments:['std::result::Result','None'],expected:-1},
 {operation:'acceptRustStandardSumQuestionContext',arguments:['std::option::Option','std::option::Option',true,true,'',''],expected:true},
 {operation:'acceptRustStandardSumQuestionContext',arguments:['std::result::Result','std::result::Result',true,true,'checked u8','checked u8'],expected:true},
 {operation:'acceptRustStandardSumQuestionContext',arguments:['std::result::Result','std::result::Result',true,true,'checked u8','checked u16'],expected:false},
 {operation:'acceptRustStandardSumQuestionContext',arguments:['std::result::Result','std::result::Result',true,true,'',''],expected:false},
 {operation:'acceptRustStandardSumQuestionContext',arguments:['std::option::Option','std::result::Result',true,true,'',''],expected:false},
 {operation:'acceptRustStandardSumQuestionContext',arguments:['std::option::Option','std::option::Option',false,true,'',''],expected:false},
 {operation:'acceptRustStandardSumQuestionContext',arguments:['std::option::Option','std::option::Option',true,false,'',''],expected:false},
 {operation:'acceptRustStandardSumQuestionContext',arguments:['local::Result','local::Result',true,true,'checked u8','checked u8'],expected:false},
 {operation:'readRustStandardSumQuestionBranch',arguments:['std::option::Option','Some'],expected:'continue field'},
 {operation:'readRustStandardSumQuestionBranch',arguments:['std::option::Option','None'],expected:'return absent'},
 {operation:'readRustStandardSumQuestionBranch',arguments:['std::result::Result','Ok'],expected:'continue field'},
 {operation:'readRustStandardSumQuestionBranch',arguments:['std::result::Result','Err'],expected:'return error field'},
 {operation:'readRustStandardSumQuestionBranch',arguments:['std::result::Result','Some'],expected:''},
];

// Source-version selection and original compilation belong to future CI.
// Each complete original has literal expected stdout and declared source std
// range. The new policy does not parse these programs or assert their admission.
export const rustStandardSumSourceOriginals = [
 {name:'nested absence stays distinct',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'0\n1\n2\n',source:`
fn classify(value: Option<Option<u8>>) -> u8 {
    match value { None => 0, Some(None) => 1, Some(Some(_)) => 2 }
}
fn main() {
    println!("{}", classify(None));
    println!("{}", classify(Some(None)));
    println!("{}", classify(Some(Some(7))));
}
`},
 {name:'option propagation skips continuation',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'before\n0\nbefore\nafter\n8\n',source:`
fn step(value: Option<u8>) -> Option<u8> {
    println!("before");
    let number = value?;
    println!("after");
    Some(number + 1)
}
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() {
    println!("{}", reveal(step(None)));
    println!("{}", reveal(step(Some(7))));
}
`},
 {name:'result propagation preserves error payload',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'before\n5\nbefore\nafter\n8\n',source:`
fn step(value: Result<u8, u8>) -> Result<u8, u8> {
    println!("before");
    let number = value?;
    println!("after");
    Ok(number + 1)
}
fn reveal(value: Result<u8, u8>) -> u8 { match value { Ok(number) => number, Err(error) => error } }
fn main() {
    println!("{}", reveal(step(Err(5))));
    println!("{}", reveal(step(Ok(7))));
}
`},
 {name:'guarded present arm needs later coverage',sourceVersionRange:'>=1.90.0 <1.100.0',stdout:'0\n2\n7\n',source:`
fn classify(value: Option<u8>) -> u8 {
    match value { Some(number) if number > 3 => number, Some(_) => 2, None => 0 }
}
fn main() {
    println!("{}", classify(None));
    println!("{}", classify(Some(1)));
    println!("{}", classify(Some(7)));
}
`},
];

export const rustStandardSumSourceRefusals = [
 {name:'missing absent case',source:`fn bad(value: Option<u8>) -> u8 { match value { Some(number) => number } } fn main() {}`},
 {name:'guard does not cover present constructor',source:`fn bad(value: Option<u8>) -> u8 { match value { Some(number) if number > 0 => number, None => 0 } } fn main() {}`},
 {name:'missing error case',source:`fn bad(value: Result<u8,u8>) -> u8 { match value { Ok(number) => number } } fn main() {}`},
 {name:'question has incompatible return family',source:`fn bad(value: Option<u8>) -> Result<u8,u8> { let number = value?; Ok(number) } fn main() {}`},
 // This source is valid Rust: From<u8> for u16 exists. Only the deliberately
 // narrow identity-error policy refuses; a later conversion model must retain it.
 {name:'error conversion needs its own policy',source:`fn later(value: Result<u8,u8>) -> Result<u8,u16> { let number = value?; Ok(number) } fn main() {}`},
];
