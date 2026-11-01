// Complete independent originals and literal stdout. These are source drafts,
// never observations. No previously retained input/oracle is replaced.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustStandardSumCombinatorDecisions=[
 {operation:'readRustStandardSumCombinatorArity',arguments:['std::option::Option','and'],expected:1},
 {operation:'readRustStandardSumCombinatorArity',arguments:['std::option::Option','xor'],expected:1},
 {operation:'readRustStandardSumCombinatorArity',arguments:['std::result::Result','or_else'],expected:1},
 {operation:'readRustStandardSumCombinatorArity',arguments:['std::result::Result','xor'],expected:-1},
 {operation:'readRustStandardSumCombinatorArity',arguments:['local::Option','or'],expected:-1},
 {operation:'readRustStandardSumCombinatorCaptureRoles',arguments:['or'],expected:['receiver','argument']},
 {operation:'readRustStandardSumCombinatorCaptureRoles',arguments:['or_else'],expected:['receiver','callback']},
 {operation:'acceptRustStandardSumCombinatorReceiverContext',arguments:['std::option::Option','or'],expected:true},
 {operation:'acceptRustStandardSumCombinatorReceiverContext',arguments:['std::option::Option','and'],expected:false},
 {operation:'acceptRustStandardSumCombinatorReceiverContext',arguments:['std::result::Result','or'],expected:false},
 {operation:'readRustStandardSumCombinatorRetainedArguments',arguments:['std::option::Option','and'],expected:[]},
 {operation:'readRustStandardSumCombinatorRetainedArguments',arguments:['std::option::Option','xor'],expected:[0]},
 {operation:'readRustStandardSumCombinatorRetainedArguments',arguments:['std::result::Result','and'],expected:[1]},
 {operation:'readRustStandardSumCombinatorRetainedArguments',arguments:['std::result::Result','or_else'],expected:[0]},
 {operation:'readRustStandardSumCombinatorCallbackArgument',arguments:['std::option::Option','or_else'],expected:-1},
 {operation:'readRustStandardSumCombinatorCallbackArgument',arguments:['std::result::Result','or_else'],expected:1},
 {operation:'planRustStandardSumCombinatorRightArguments',arguments:['std::result::Result','and','Ok'],expected:['argument','left error']},
 {operation:'planRustStandardSumCombinatorRightArguments',arguments:['std::result::Result','or','Err'],expected:['left value','argument']},
 {operation:'planRustStandardSumCombinatorRightArguments',arguments:['std::result::Result','or','Ok'],expected:[]},
 {operation:'readRustStandardSumCombinatorBranch',arguments:['std::option::Option','Some','and',''],expected:'right'},
 {operation:'readRustStandardSumCombinatorBranch',arguments:['std::option::Option','None','and',''],expected:'construct failure'},
 {operation:'readRustStandardSumCombinatorBranch',arguments:['std::option::Option','Some','or',''],expected:'left'},
 {operation:'readRustStandardSumCombinatorBranch',arguments:['std::option::Option','None','or_else',''],expected:'callback'},
 {operation:'readRustStandardSumCombinatorBranch',arguments:['std::option::Option','Some','xor','None'],expected:'left'},
 {operation:'readRustStandardSumCombinatorBranch',arguments:['std::option::Option','None','xor','Some'],expected:'right'},
 {operation:'readRustStandardSumCombinatorBranch',arguments:['std::option::Option','Some','xor','Some'],expected:'construct failure'},
 {operation:'readRustStandardSumCombinatorBranch',arguments:['std::option::Option','None','xor','None'],expected:'construct failure'},
 {operation:'readRustStandardSumCombinatorBranch',arguments:['std::result::Result','Err','and',''],expected:'construct failure'},
 {operation:'readRustStandardSumCombinatorBranch',arguments:['std::result::Result','Ok','or',''],expected:'construct success'},
 {operation:'readRustStandardSumCombinatorBranch',arguments:['std::result::Result','Err','or_else',''],expected:'callback'},
 {operation:'readRustStandardSumCombinatorConstructor',arguments:['std::option::Option',false],expected:'None'},
 {operation:'readRustStandardSumCombinatorConstructor',arguments:['std::result::Result',true],expected:'Ok'},
 {operation:'readRustStandardSumCombinatorVersionError',arguments:['std::result::Result','and',90],expected:''},
 {operation:'readRustStandardSumCombinatorVersionError',arguments:['std::option::Option','xor',99],expected:''},
 {operation:'readRustStandardSumCombinatorVersionError',arguments:['std::option::Option','or',100],expected:'standard sum combinator source semantics are uncovered outside Rust >=1.90.0 <1.100.0'},
];
export const rustStandardSumCombinatorOriginals=[
 {name:'Option and evaluates right even when left is absent and changes payload type',sourceVersionRange,singleCallOwners:['left','right'],singleCallFunction:'choose',stdout:'left\nright\n9\nleft\nright\n0\n0\n',source:`
fn left(flag:bool)->Option<u8>{println!("left");if flag{Some(7)}else{None}}
fn right()->Option<u16>{println!("right");Some(9)}
fn reveal(value:Option<u16>)->u16{match value{Some(number)=>number,None=>0}}
fn choose(flag:bool)->u16{reveal(left(flag).and(right()))}
fn main(){println!("{}",choose(true));println!("{}",choose(false));println!("{}",reveal(Some::<u8>(7).and(None::<u16>)));}
`},
 {name:'Option or evaluates unused right operand once before selecting left',sourceVersionRange,singleCallOwners:['left','right'],singleCallFunction:'choose',stdout:'left\nright\n7\nleft\nright\n9\n7\n0\n',source:`
fn left(flag:bool)->Option<u8>{println!("left");if flag{Some(7)}else{None}}
fn right()->Option<u8>{println!("right");Some(9)}
fn reveal(value:Option<u8>)->u8{match value{Some(number)=>number,None=>0}}
fn choose(flag:bool)->u8{reveal(left(flag).or(right()))}
fn main(){println!("{}",choose(true));println!("{}",choose(false));println!("{}",reveal(Some::<u8>(7).or(None)));println!("{}",reveal(None::<u8>.or(None)));}
`},
 {name:'Option xor retains all four actual presence combinations',sourceVersionRange,singleCallOwners:[],stdout:'7\n9\n0\n0\n',source:`
fn choose(left:Option<u8>,right:Option<u8>)->u8{match left.xor(right){Some(number)=>number,None=>0}}
fn main(){println!("{}",choose(Some(7),None));println!("{}",choose(None,Some(9)));println!("{}",choose(Some(7),Some(9)));println!("{}",choose(None,None));}
`},
 {name:'Option or_else evaluates factory while invoking only selected fallback',sourceVersionRange,singleCallOwners:['left','factory'],singleCallFunction:'choose',stdout:'left\nfactory\n7\nleft\nfactory\nfallback\n9\n',source:`
fn left(flag:bool)->Option<u8>{println!("left");if flag{Some(7)}else{None}}
fn fallback()->Option<u8>{println!("fallback");Some(9)}
fn factory()->fn()->Option<u8>{println!("factory");fallback}
fn reveal(value:Option<u8>)->u8{match value{Some(number)=>number,None=>0}}
fn choose(flag:bool)->u8{reveal(left(flag).or_else(factory()))}
fn main(){println!("{}",choose(true));println!("{}",choose(false));}
`},
 {name:'Result and keeps original error and evaluates right on every constructor',sourceVersionRange,singleCallOwners:['left','right'],singleCallFunction:'choose',stdout:'left\nright\n9\nleft\nright\n5\nleft\nright\n6\nleft\nright\n5\n',source:`
fn left(flag:bool)->Result<u8,u8>{println!("left");if flag{Ok(7)}else{Err(5)}}
fn right(flag:bool)->Result<u16,u8>{println!("right");if flag{Ok(9)}else{Err(6)}}
fn choose(first:bool,second:bool)->u16{match left(first).and(right(second)){Ok(number)=>number,Err(error)=>u16::from(error)}}
fn main(){println!("{}",choose(true,true));println!("{}",choose(false,true));println!("{}",choose(true,false));println!("{}",choose(false,false));}
`},
 {name:'Result or changes error family but preserves successful payload type',sourceVersionRange,singleCallOwners:['left','right'],singleCallFunction:'choose',stdout:'left\nright\n7\nleft\nright\n7\nleft\nright\n9\nleft\nright\n6\n',source:`
fn left(flag:bool)->Result<u8,u8>{println!("left");if flag{Ok(7)}else{Err(5)}}
fn right(flag:bool)->Result<u8,u16>{println!("right");if flag{Ok(9)}else{Err(6)}}
fn choose(first:bool,second:bool)->u16{match left(first).or(right(second)){Ok(number)=>u16::from(number),Err(error)=>error}}
fn main(){println!("{}",choose(true,true));println!("{}",choose(true,false));println!("{}",choose(false,true));println!("{}",choose(false,false));}
`},
 {name:'Result or_else factory preserves input error and actual new error family',sourceVersionRange,singleCallOwners:['left','factory'],singleCallFunction:'choose',stdout:'left\nfactory\n7\nleft\nfactory\nrecover\n6\n',source:`
fn left(flag:bool)->Result<u8,u8>{println!("left");if flag{Ok(7)}else{Err(5)}}
fn recover(error:u8)->Result<u8,u16>{println!("recover");Err(u16::from(error)+1)}
fn factory()->fn(u8)->Result<u8,u16>{println!("factory");recover}
fn choose(flag:bool)->u16{match left(flag).or_else(factory()){Ok(number)=>u16::from(number),Err(error)=>error}}
fn main(){println!("{}",choose(true));println!("{}",choose(false));}
`},
 {name:'Result and direct Ok receives only its real missing error owner',sourceVersionRange,singleCallOwners:[],stdout:'9\n5\n',source:`
fn choose(value:Result<u8,u8>)->Result<u16,u8>{value.and(Ok(9u16))}
fn reveal(value:Result<u16,u8>)->u16{match value{Ok(number)=>number,Err(error)=>u16::from(error)}}
fn main(){println!("{}",reveal(choose(Ok(7))));println!("{}",reveal(choose(Err(5))));}
`},
 {name:'Result or direct Err receives only its real missing successful owner',sourceVersionRange,singleCallOwners:[],stdout:'7\n9\n',source:`
fn choose(value:Result<u8,u8>)->Result<u8,u16>{value.or(Err(9u16))}
fn reveal(value:Result<u8,u16>)->u16{match value{Ok(number)=>u16::from(number),Err(error)=>error}}
fn main(){println!("{}",reveal(choose(Ok(7))));println!("{}",reveal(choose(Err(5))));}
`},
 {name:'partial Result constructors keep actual imported value namespace aliases',sourceVersionRange,singleCallOwners:[],stdout:'9\n5\n7\n11\n',source:`
use std::result::Result::Ok as Success;
use std::result::Result::Err as Failure;
fn next(value:Result<u8,u8>)->Result<u16,u8>{value.and(Success(9u16))}
fn fallback(value:Result<u8,u8>)->Result<u8,u16>{value.or(Failure(11u16))}
fn reveal_left(value:Result<u16,u8>)->u16{match value{Ok(number)=>number,Err(error)=>u16::from(error)}}
fn reveal_right(value:Result<u8,u16>)->u16{match value{Ok(number)=>u16::from(number),Err(error)=>error}}
fn main(){println!("{}",reveal_left(next(Ok(7))));println!("{}",reveal_left(next(Err(5))));println!("{}",reveal_right(fallback(Ok(7))));println!("{}",reveal_right(fallback(Err(5))));}
`},
 {name:'actual source Span optional fallback preserves nominal Copy owner',sourceVersionRange,singleCallOwners:[],stdout:'3\n9\n0\n',source:`
#[derive(Clone,Copy)]struct Span{start:usize,end:usize}
fn choose(left:Option<Span>,right:Option<Span>)->usize{match left.or(right){Some(place)=>place.start,None=>0usize}}
fn main(){let first=Span{start:3usize,end:7usize};let last=Span{start:9usize,end:11usize};println!("{}",choose(Some(first),Some(last)));println!("{}",choose(None,Some(last)));println!("{}",choose(None,None));}
`},
 {name:'Unit xor presence stays separate from absent Unit',sourceVersionRange,singleCallOwners:[],stdout:'true\ntrue\nfalse\nfalse\n',source:`
fn choose(left:Option<()>,right:Option<()>)->bool{left.xor(right).is_some()}
fn main(){println!("{}",choose(Some(()),None));println!("{}",choose(None,Some(())));println!("{}",choose(Some(()),Some(())));println!("{}",choose(None,None));}
`},
 {name:'Option or_else actual receiver constrains absent callback result',sourceVersionRange,singleCallOwners:[],stdout:'7\n0\n',source:`
fn choose(value:Option<u8>)->Option<u8>{value.or_else(||None)}
fn reveal(value:Option<u8>)->u8{match value{Some(number)=>number,None=>0}}
fn main(){println!("{}",reveal(choose(Some(7))));println!("{}",reveal(choose(None)));}
`},
];
export const rustStandardSumCombinatorRefusals=[
 {name:'Option or cannot change actual successful payload owner',sourceVersionRange,source:'fn wrong(left:Option<u8>,right:Option<u16>)->Option<u8>{left.or(right)}fn main(){}'},
 {name:'Option xor cannot change actual successful payload owner',sourceVersionRange,source:'fn wrong(left:Option<u8>,right:Option<bool>)->Option<u8>{left.xor(right)}fn main(){}'},
 {name:'Result and cannot silently convert retained error owner',sourceVersionRange,source:'fn wrong(left:Result<u8,u8>,right:Result<u16,u16>)->Result<u16,u8>{left.and(right)}fn main(){}'},
 {name:'Result or cannot silently convert retained successful owner',sourceVersionRange,source:'fn wrong(left:Result<u8,u8>,right:Result<u16,u16>)->Result<u8,u16>{left.or(right)}fn main(){}'},
 {name:'Result or_else callback keeps original error parameter owner',sourceVersionRange,source:'fn callback(error:u8)->Result<u8,u16>{Err(u16::from(error))}fn wrong(value:Result<u8,u16>)->Result<u8,u16>{value.or_else(callback)}fn main(){}'},
 {name:'Result or_else cannot change retained successful payload owner',sourceVersionRange,source:'fn callback(error:u8)->Result<u16,u16>{Err(u16::from(error))}fn wrong(value:Result<u8,u8>)->Result<u8,u16>{value.or_else(callback)}fn main(){}'},
 {name:'Option or_else cannot change successful payload owner',sourceVersionRange,source:'fn callback()->Option<u16>{Some(9)}fn wrong(value:Option<u8>)->Option<u8>{value.or_else(callback)}fn main(){}'},
 {name:'and cannot swap actual Option and Result families',sourceVersionRange,source:'fn wrong(left:Option<u8>,right:Result<u8,u8>)->Option<u8>{left.and(right)}fn main(){}'},
 {name:'partial constructor does not replace an actual shadowing function',sourceVersionRange,source:'fn Ok(value:u16)->Result<u16,u16>{Err(value)}fn wrong(left:Result<u8,u8>)->Result<u16,u8>{left.and(Ok(9u16))}fn main(){}'},
 {name:'partial constructor cannot guess numeric successful type',sourceVersionRange,reasonIncludes:'actual complete immutable Copy source type',source:'fn later(value:Result<u8,u8>)->Result<u16,u8>{value.and(Ok(9))}fn main(){}'},
 {name:'partial constructor exact one-field arity remains mandatory',sourceVersionRange,reasonIncludes:'one field',source:'fn wrong(value:Result<u8,u8>)->Result<u16,u8>{value.and(Ok(9u16,10u16))}fn main(){}'},
 {name:'partial constructor scope does not cross error-changing member operations',sourceVersionRange,reasonIncludes:'complete direct right operand',source:'fn change(error:u16)->u8{u8::try_from(error).unwrap_or(0)}fn later(value:Result<u8,u8>)->Result<u16,u8>{value.and(Ok(9u16).map_err(change))}fn main(){}'},
 {name:'Result or_else retains unestablished generic callback result as an obligation',sourceVersionRange,source:'fn later(value:Result<u8,u8>)->Result<u8,u16>{value.or_else(|error|Err(u16::from(error)))}fn main(){}'},
];
rustStandardSumCombinatorDecisions.push(
 {operation:'acceptRustStandardSumCombinatorOwnership',arguments:['copy','or_else'],expected:true},
 {operation:'acceptRustStandardSumCombinatorOwnership',arguments:['owned','and'],expected:true},
 {operation:'acceptRustStandardSumCombinatorOwnership',arguments:['owned','or'],expected:true},
 {operation:'acceptRustStandardSumCombinatorOwnership',arguments:['owned','xor'],expected:true},
 {operation:'acceptRustStandardSumCombinatorOwnership',arguments:['owned','or_else'],expected:false},
 {operation:'acceptRustStandardSumCombinatorOwnership',arguments:['reference','and'],expected:false},
 {operation:'acceptRustStandardSumCombinatorOwnership',arguments:['','or'],expected:false},
 {operation:'acceptRustStandardSumCombinatorOwnership',arguments:['copy','unknown'],expected:false});
rustStandardSumCombinatorOriginals.push(
 {name:'owned Option or reconstructs a selected payload without root reuse',sourceVersionRange,singleCallOwners:[],stdout:'left\nright\nfalse\n',source:`
fn reveal(value:Option<String>)->String{value.unwrap_or(String::from("empty"))}
fn main(){let a:Option<String>=Some(String::from("left"));let b:Option<String>=None;let c:Option<String>=None;println!("{}",reveal(a.or(Some(String::from("right")))));println!("{}",reveal(b.or(Some(String::from("right")))));println!("{}",c.or(None).is_some());}
`},
 {name:'owned Option xor reconstructs exactly the one present payload',sourceVersionRange,singleCallOwners:[],stdout:'left\nright\nfalse\nfalse\n',source:`
fn choose(a:Option<String>,b:Option<String>)->Option<String>{a.xor(b)}
fn main(){println!("{}",choose(Some(String::from("left")),None).unwrap_or(String::from("empty")));println!("{}",choose(None,Some(String::from("right"))).unwrap_or(String::from("empty")));println!("{}",choose(Some(String::from("left")),Some(String::from("right"))).is_some());println!("{}",choose(None,None).is_some());}
`},
 {name:'owned Option and changes payload roles with eager right evaluation',sourceVersionRange,singleCallOwners:[],stdout:'right\n7\nright\nfalse\n',source:`
fn next()->Option<u8>{println!("right");Some(7u8)}
fn main(){let a:Option<String>=Some(String::from("left"));let b:Option<String>=None;println!("{}",a.and(next()).unwrap_or(0u8));println!("{}",b.and(next()).is_some());}
`},
 {name:'Copy Option and can introduce an actual owned result',sourceVersionRange,singleCallOwners:[],stdout:'right\ntrue\n',source:`
fn main(){let a:Option<u8>=Some(7u8);let b:Option<String>=Some(String::from("right"));println!("{}",a.and(b).unwrap_or(String::from("empty")));println!("{}",a.is_some());}
`},
 {name:'owned Result and constrains direct success from actual retained error',sourceVersionRange,singleCallOwners:[],stdout:'next\nerror\n',source:`
fn reveal(value:Result<String,String>)->String{match value{Ok(text)=>text,Err(text)=>text}}
fn main(){let a:Result<u8,String>=Ok(7u8);let b:Result<u8,String>=Err(String::from("error"));println!("{}",reveal(a.and(Ok(String::from("next")))));println!("{}",reveal(b.and(Ok(String::from("unused")))));}
`},
 {name:'owned Result or changes the actual error while retaining success',sourceVersionRange,singleCallOwners:[],stdout:'left\n9\n',source:`
fn main(){let a:Result<String,String>=Ok(String::from("left"));let b:Result<String,String>=Err(String::from("error"));let first=a.or(Err(9u8));let second=b.or(Err(9u8));println!("{}",first.unwrap_or(String::from("wrong")));let number=match second{Ok(_)=>0u8,Err(value)=>value};println!("{}",number);}
`},
 {name:'owned eager combinator moves receiver then evaluates right once',sourceVersionRange,singleCallOwners:[],stdout:'left\nright\nleft\n',source:`
fn left()->Option<String>{println!("left");Some(String::from("left"))}fn right()->Option<String>{println!("right");Some(String::from("right"))}fn main(){println!("{}",left().or(right()).unwrap_or(String::from("wrong")));}
`});
rustStandardSumCombinatorRefusals.push(...[
 {name:'owned eager combinator moves self before the right expression reads it',source:'fn main(){let a:Option<String>=Some(String::from("left"));let result=a.or(match a{Some(text)=>Some(text),None=>None});}'},
 {name:'owned eager combinator consumes an unselected right operand',source:'fn main(){let a:Option<String>=Some(String::from("left"));let b:Option<String>=Some(String::from("right"));let result=a.or(b);println!("{}",b.is_some());}'},
 {name:'owned eager combinator cannot reuse its receiver',source:'fn main(){let a:Option<String>=Some(String::from("left"));let b:Option<String>=None;let result=a.xor(b);println!("{}",a.is_some());}'},
 {name:'owned combinator cannot consume a shared receiver',source:'fn main(){let a:Option<String>=Some(String::from("left"));let b:Option<String>=None;let result=(&a).or(b);}'},
 {name:'owned constrained constructor transfers its actual payload',source:'fn main(){let a:Result<u8,String>=Ok(7u8);let payload=String::from("right");let result=a.and(Ok(payload));println!("{}",payload);}'}
].map(row=>({...row,sourceVersionRange,originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',outcome:'reject',category:'invalid Rust ownership'}})));
