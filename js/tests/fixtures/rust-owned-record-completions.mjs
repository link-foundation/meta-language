// Independent original programs for the later compiler/runtime matrix.
const sourceVersionRange='>=1.90.0 <1.100.0';
const record='struct Record{text:String,number:u8}';
const initial='let mut value=Record{text:String::from("old"),number:1u8};';
export const rustOwnedRecordCompletionDecisions=[
 {operation:'testRustOrdinaryCompletionFallsThrough',arguments:['value'],expected:true},
 {operation:'testRustOrdinaryCompletionFallsThrough',arguments:['mixed'],expected:true},
 {operation:'testRustOrdinaryCompletionFallsThrough',arguments:['returned'],expected:false},
 {operation:'testRustOrdinaryCompletionFallsThrough',arguments:['aborted'],expected:false},
 {operation:'readRustOrdinaryCompletionSnapshotError',arguments:[99999,1,16777215,1],expected:''},
 {operation:'readRustOrdinaryCompletionSnapshotError',arguments:[100000,1,0,0],expected:'the actual completion exit snapshots exceed their finite source budget'},
 {operation:'readRustOrdinaryCompletionSnapshotError',arguments:[0,0,16777216,1],expected:'the actual completion exit snapshots exceed their finite source budget'},
];
export const rustOwnedRecordCompletionOriginals=[
 {name:'returning branch publishes its result without moving the continuing root',sourceVersionRange,stdout:'early\nlate\n',source:record+'fn choose(flag:bool)->Record{'+initial+'if flag{value.text=String::from("early");return value;}value.text=String::from("late");value}fn main(){let early=choose(true);let late=choose(false);println!("{}",early.text);println!("{}",late.text);}'},
 {name:'early scalar result retains the selected state before its return',sourceVersionRange,stdout:'7\n9\n',source:record+'fn choose(flag:bool)->u8{'+initial+'if flag{value.number=7u8;return value.number;}value.number=9u8;value.number}fn main(){println!("{}",choose(true));println!("{}",choose(false));}'},
 {name:'question mark assignment writes only after actual successful payload',sourceVersionRange,stdout:'rhs\nfalse\nrhs\n5\n5\n',source:record+'fn change(input:Option<u8>)->Option<u8>{'+initial+'value.number={println!("rhs");input?};println!("{}",value.number);Some(value.number)}fn main(){let absent=change(None);println!("{}",absent.is_some());let present=change(Some(5u8));println!("{}",present.unwrap_or(0u8));}'},
 {name:'result residual bypasses the write and its following print',sourceVersionRange,stdout:'false\n6\ntrue\n',source:record+'fn change(input:Result<u8,u16>)->Result<u8,u16>{'+initial+'value.number=input?;println!("{}",value.number);Ok(value.number)}fn main(){let absent=change(Err(3u16));println!("{}",absent.is_ok());let present=change(Ok(6u8));println!("{}",present.is_ok());}'},
 {name:'lazy assertion retains exact nominal structural equality',sourceVersionRange,stdout:'2\n',source:record+'#[derive(Clone,Copy,PartialEq)]struct Number{number:u8}fn main(){'+initial+'assert!((Number{number:1u8}==Number{number:1u8})&&{value.number=2u8;true});println!("{}",value.number);}'},
 {name:'short circuit assertion skips state and output of the unselected operand',sourceVersionRange,stdout:'1\n',source:record+'fn main(){'+initial+'assert!(true||{println!("unselected");value.number=9u8;false});println!("{}",value.number);}'},
 {name:'let else completed fallback does not contaminate successful ownership',sourceVersionRange,stdout:'fallback\nselected\n',source:record+'fn choose(input:Option<u8>)->Record{'+initial+'let Some(number)=input else{value.text=String::from("fallback");return value;};value.text=String::from("selected");value.number=number;value}fn main(){let absent=choose(None);let present=choose(Some(5u8));println!("{}",absent.text);println!("{}",present.text);}'},
 {name:'borrowed return alternatives retain their own genuine lifetime state',sourceVersionRange,stdout:'left\nright\n',source:'fn choose<\'a>(flag:bool,left:&\'a str,right:&\'a str)->&\'a str{if flag{return left;}right}fn main(){println!("{}",choose(true,"left","right"));println!("{}",choose(false,"left","right"));}'},
 {name:'failed owned place pattern retains its unmoved receiver inside else',sourceVersionRange,stdout:'true\n',source:'fn main(){let value:Option<String>=None;if let Some(text)=value{println!("{}",text);}else{println!("{}",value.is_none());};}'},
];
export const rustOwnedRecordCompletionRefusals=[
 {name:'returned reference cannot escape a completed local state scope',sourceVersionRange,originalCompilerOutcome:'reject',source:record+'fn invalid(flag:bool)->&\'static String{'+initial+'if flag{return &value.text;}&value.text}fn main(){}'},
 {name:'continuing alternative cannot use a root moved on its own path',sourceVersionRange,originalCompilerOutcome:'reject',source:record+'fn invalid(flag:bool)->u8{'+initial+'if flag{return 7u8;}let moved=value;value.number=9u8;value.number}fn main(){}'},
 {name:'question mark does not repair a field before evaluating its RHS',sourceVersionRange,originalCompilerOutcome:'reject',source:record+'fn invalid(input:Option<u8>)->Option<u8>{'+initial+'let moved=value.text;value.text={let number=input?;value.text};Some(value.number)}fn main(){}'},
 {name:'failed pattern does not undo an actual scrutinee factory argument move',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn absent(input:String)->Option<u8>{None}fn invalid()->u8{let input=String::from("moved");if let Some(number)=absent(input){return number;}println!("{}",input);0u8}fn main(){}'},
 {name:'failure after a successful owned pattern retains its earlier move',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn invalid(value:Option<String>){if let Some(text)=value&&false{println!("{}",text);}else{println!("{}",value.is_none());};}fn main(){}'},
];
