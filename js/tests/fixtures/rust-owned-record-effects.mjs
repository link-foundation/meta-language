// Independent source declarations for future execution; no observations.
const sourceVersionRange='>=1.90.0 <1.100.0';
const record='struct Record{text:String,number:u8}';
const initial='let mut value=Record{text:String::from("old"),number:1u8};';
export const rustOwnedRecordEffectDecisions=[
 {operation:'readRustOwnedRecordEffectForm',arguments:['math','rustOwnedRecordStateRead'],expected:'read state'},
 {operation:'readRustOwnedRecordEffectForm',arguments:['math','rustOwnedRecordStateWrite'],expected:'write state'},
 {operation:'readRustOwnedRecordEffectForm',arguments:['if',''],expected:'conditional'},
 {operation:'readRustOwnedRecordEffectForm',arguments:['binary','and'],expected:'short circuit'},
 {operation:'readRustOwnedRecordEffectForm',arguments:['match',''],expected:'match'},
 {operation:'readRustOwnedRecordEffectChildren',arguments:['app'],expected:['fn','args']},
 {operation:'readRustOwnedRecordEffectChildren',arguments:['method'],expected:['object','args']},
 {operation:'readRustOwnedRecordEffectChildren',arguments:['math'],expected:['args']},
 {operation:'readRustOwnedRecordEffectChildren',arguments:['lambda'],expected:[]},
 {operation:'readRustOwnedRecordEffectOperation',arguments:['read'],expected:'rustOwnedRecordStateRead'},
 {operation:'readRustOwnedRecordEffectOperation',arguments:['write'],expected:'rustOwnedRecordStateWrite'},
 {operation:'readRustOwnedRecordEffectOperation',arguments:['unknown'],expected:''},
 {operation:'readRustOwnedRecordEffectLimit',arguments:['depth'],expected:256},
 {operation:'acceptRustOwnedRecordEffectScope',arguments:[true,true],expected:true},
 {operation:'acceptRustOwnedRecordEffectScope',arguments:[false,true],expected:false},
 {operation:'acceptRustOwnedRecordEffectScope',arguments:[true,false],expected:false},
];
export const rustOwnedRecordEffectOriginals=[
 {name:'initializer state is available to the following main effect',sourceVersionRange,stdout:'true\nnew\n',source:record+'fn main(){'+initial+'let flag:bool={value.text=String::from("new");true};println!("{}",flag);println!("{}",value.text);}'},
 {name:'ordinary call operands retain their left to right snapshots',sourceVersionRange,stdout:'1\n2\n2\n',source:record+'fn show(first:u8,second:u8){println!("{}",first);println!("{}",second);}fn main(){'+initial+'show(value.number,{value.number=2u8;value.number});println!("{}",value.number);}'},
 {name:'earlier effectful operands execute before later state writes',sourceVersionRange,stdout:'first\nsecond\n1\n2\n',source:record+'fn first(value:u8)->u8{println!("first");value}fn second()->u8{println!("second");2u8}fn show(first:u8,second:u8){println!("{}",first);println!("{}",second);}fn main(){'+initial+'show(first(value.number),{value.number=second();value.number});}'},
 {name:'tuple operands retain distinct actual state versions',sourceVersionRange,stdout:'1\n7\n7\n',source:record+'fn main(){'+initial+'let pair:(u8,u8)=(value.number,{value.number=7u8;value.number});println!("{}",pair.0);println!("{}",pair.1);println!("{}",value.number);}'},
 {name:'false and skips the entire right state transition',sourceVersionRange,stdout:'false\n1\n',source:record+'fn main(){'+initial+'let result:bool=false&&{value.number=9u8;true};println!("{}",result);println!("{}",value.number);}'},
 {name:'true or skips the entire right state transition',sourceVersionRange,stdout:'true\n1\n',source:record+'fn main(){'+initial+'let result:bool=true||{value.number=9u8;false};println!("{}",result);println!("{}",value.number);}'},
 {name:'selected short circuit operand publishes its state',sourceVersionRange,stdout:'true\n9\n',source:record+'fn main(){'+initial+'let result:bool=true&&{value.number=9u8;true};println!("{}",result);println!("{}",value.number);}'},
 {name:'if initializer joins result and surviving record state',sourceVersionRange,stdout:'3\n3\n',source:record+'fn main(){'+initial+'let result:u8=if true{value.number=3u8;value.number}else{value.number=4u8;value.number};println!("{}",result);println!("{}",value.number);}'},
 {name:'match initializer publishes only the selected arm state',sourceVersionRange,stdout:'4\n4\n',source:record+'fn main(){'+initial+'let result:u8=match 2u8{1u8=>{value.number=3u8;value.number},_=>{value.number=4u8;value.number}};println!("{}",result);println!("{}",value.number);}'},
 {name:'function body transports local state through nested value contexts',sourceVersionRange,stdout:'8\nnew\n',source:record+'fn changed()->Record{'+initial+'let number:u8={value.text=String::from("new");8u8};value.number=number;value}fn main(){let value=changed();println!("{}",value.number);println!("{}",value.text);}'},
 {name:'main structural assertions preserve both operand evaluation order and data equality',sourceVersionRange,stdout:'2\n',source:record+'#[derive(Clone,Copy,PartialEq)]struct Number{number:u8}fn main(){'+initial+'assert_eq!(Number{number:1u8},{value.number=2u8;Number{number:1u8}});println!("{}",value.number);}'},
 {name:'main Boolean assertion evaluates its actual state block once',sourceVersionRange,stdout:'5\n',source:record+'fn main(){'+initial+'assert!({value.number=5u8;value.number==5u8});println!("{}",value.number);}'},
];
export const rustOwnedRecordEffectRefusals=[
 {name:'ordered expression cannot move an expired whole record',sourceVersionRange,originalCompilerOutcome:'reject',source:record+'fn main(){'+initial+'let old=value;let next={value.number=2u8;value};}'},
 {name:'earlier argument loan remains active during a later overlapping write',sourceVersionRange,originalCompilerOutcome:'reject',source:record+'fn observe(first:&String,second:u8){}fn main(){'+initial+'observe(&value.text,{value.text=String::from("new");2u8});}'},
 {name:'a conditional write cannot restore a moved field on its skipped path',sourceVersionRange,originalCompilerOutcome:'reject',source:record+'fn main(){'+initial+'let moved=value.text;let ignored:bool=false&&{value.text=String::from("new");true};println!("{}",value.text);}'},
 {name:'stateful captured closure retains its actual FnMut boundary',sourceVersionRange,originalCompilerOutcome:'accept',source:record+'fn main(){'+initial+'let change=||{value.number=2u8;};}'},
 {name:'logical assertion state writes await proposition continuations',sourceVersionRange,originalCompilerOutcome:'accept',source:record+'fn main(){'+initial+'assert!(true&&{value.number=2u8;true});}'},
];
// Exact previous valid boundary input is retained byte for byte as an active
// positive above; this history is a declaration, not an observed result.
export const rustOwnedRecordEffectRefusalHistory=[{fixture:'rustOwnedRecordStateRefusals',index:7,name:'uncomposed initializer state effects retain their valid source',sourceVersionRange,originalCompilerOutcome:'accept',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};let flag={value.left=String::from("new");true};println!("{}",flag);}'}];
export const rustOwnedRecordEffectRetainedOriginals=[{...rustOwnedRecordEffectRefusalHistory[0],name:'the retained initializer boundary now has an explicit ordered continuation',stdout:'true\n'}];
