// Independent complete source inputs; assertions are deferred, not observations.
const sourceVersionRange='>=1.90.0 <1.100.0';
const record='struct Record{text:String,count:u8}';
const initial='let mut value=Record{text:String::from("kept"),count:start};';
export const rustOwnedRecordLoopExitOriginals=[
 {name:'early scalar function return exits the finite loop and enclosing callable',sourceVersionRange,stdout:'9\n7\n',source:record+'fn choose(start:u8)->u8{'+initial+'while value.count>0u8{if value.count==2u8{return 7u8;}value.count=value.count-1u8;}9u8}fn main(){println!("{}",choose(0u8));println!("{}",choose(3u8));}'},
 {name:'break needs no decrement on its completed path',sourceVersionRange,stdout:'2\n0\n',source:record+'fn stop(start:u8)->u8{'+initial+'while value.count>0u8{if value.count==2u8{break;}value.count=value.count-1u8;}value.count}fn main(){println!("{}",stop(3u8));println!("{}",stop(0u8));}'},
 {name:'continue preserves one real decrement and skips the remaining body',sourceVersionRange,stdout:'last\n0\n',source:record+'fn finish(start:u8)->u8{'+initial+'while value.count>0u8{value.count=value.count-1u8;if value.count>0u8{continue;}println!("last");}value.count}fn main(){println!("{}",finish(3u8));}'},
 {name:'owned return field is moved only on the actual completed function path',sourceVersionRange,stdout:'early\nkept\nkept\n',source:record+'fn choose(start:u8)->String{'+initial+'while value.count>0u8{if value.count==2u8{println!("early");return value.text;}value.count=value.count-1u8;}value.text}fn main(){println!("{}",choose(3u8));println!("{}",choose(0u8));}'},
 {name:'break may move one owned field and leave a separate Copy field usable',sourceVersionRange,stdout:'1\n',source:record+'fn remaining(start:u8)->u8{'+initial+'while value.count>0u8{let taken=value.text;break;}value.count}fn main(){println!("{}",remaining(1u8));}'},
];
export const rustOwnedRecordLoopExitRefusals=[
 {name:'continue without a decrement has no continuing progress proof',sourceVersionRange,originalCompilerOutcome:'accept',source:record+'fn missing(start:u8){'+initial+'while value.count>0u8{continue;}}fn main(){}'},
 {name:'owned field moved on a continuing iteration is not restored by a counter decrement',sourceVersionRange,originalCompilerOutcome:'reject',source:record+'fn invalid(start:u8)->u8{'+initial+'while value.count>0u8{let taken=value.text;value.count=value.count-1u8;continue;}value.count}fn main(){}'},
 {name:'whole record remains unavailable after a possibly partially moving break',sourceVersionRange,originalCompilerOutcome:'reject',source:record+'fn invalid(start:u8)->Record{'+initial+'while value.count>0u8{let taken=value.text;break;}value}fn main(){}'},
 {name:'a loop function result cannot borrow a completed local record scope',sourceVersionRange,originalCompilerOutcome:'reject',source:record+'fn invalid(start:u8)->&\'static String{'+initial+'while value.count>0u8{if value.count==2u8{return &value.text;}value.count=value.count-1u8;}&value.text}fn main(){}'},
];
