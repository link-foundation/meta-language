// Independent policy literals and complete originals. No execution observations.
const expansion='the complete finite record loop expansion exceeds its source budget';
const progress='the complete record loop exits exceed their finite source budget';
export const rustOwnedRecordLoopDecisions=[
 {operation:'acceptRustOwnedRecordLoopCount',arguments:[0],expected:true},
 {operation:'acceptRustOwnedRecordLoopCount',arguments:[255],expected:true},
 {operation:'acceptRustOwnedRecordLoopCount',arguments:[-1],expected:false},
 {operation:'acceptRustOwnedRecordLoopCount',arguments:[0.5],expected:false},
 {operation:'acceptRustOwnedRecordLoopCount',arguments:[9007199254740992],expected:false},
 {operation:'readRustOwnedRecordLoopProgressError',arguments:[['value','break'],[1,0]],expected:''},
 {operation:'readRustOwnedRecordLoopProgressError',arguments:[['continue'],[0]],expected:'every continuing record loop path must decrement its actual counter exactly once'},
 {operation:'readRustOwnedRecordLoopProgressError',arguments:[['break'],[-1]],expected:progress},
 {operation:'readRustOwnedRecordLoopExpansionError',arguments:[0,100,20,255],expected:''},
 {operation:'readRustOwnedRecordLoopExpansionError',arguments:[0,100001,0,0],expected:expansion},
 {operation:'readRustOwnedRecordLoopExpansionError',arguments:[0,1,0,256],expected:expansion},
 {operation:'readRustOwnedRecordLoopChunkName',arguments:[0,1,0],expected:'ml_finite_record_loop_chunk_0_1'},
 {operation:'readRustOwnedRecordLoopChunkName',arguments:[-1,0,0],expected:''},
];
// Nonfinite numbers and signed zero have no lossless JSON number transport.
// Keep their literal source controls separate from the shared Links fixtures.
export const rustOwnedRecordLoopMalformedNumbers=[
 {operation:'acceptRustOwnedRecordLoopCount',arguments:[NaN],expected:false},
 {operation:'acceptRustOwnedRecordLoopCount',arguments:[Infinity],expected:false},
 {operation:'acceptRustOwnedRecordLoopCount',arguments:[-Infinity],expected:false},
 {operation:'acceptRustOwnedRecordLoopCount',arguments:[-0],expected:false},
 {operation:'readRustOwnedRecordLoopProgressError',arguments:[['break'],[NaN]],expected:progress},
 {operation:'readRustOwnedRecordLoopExpansionError',arguments:[NaN,0,0,0],expected:expansion},
 {operation:'readRustOwnedRecordLoopExpansionError',arguments:[0,Infinity,0,0],expected:expansion},
 {operation:'readRustOwnedRecordLoopExpansionError',arguments:[0,0,-0,0],expected:expansion},
 {operation:'readRustOwnedRecordLoopExpansionError',arguments:[0,0,0,-Infinity],expected:expansion},
 {operation:'readRustOwnedRecordLoopChunkName',arguments:[NaN,0,0],expected:''},
 {operation:'readRustOwnedRecordLoopChunkName',arguments:[0,Infinity,0],expected:''},
 {operation:'readRustOwnedRecordLoopChunkName',arguments:[0,0,-0],expected:''},
];
export const rustOwnedRecordLoopPreservedOriginals=[
 {name:'ordinary user function with generated chunk prefix stays ordinary',sourceVersionRange:'>=1.90.0 <1.100.0',source:'fn ml_finite_record_loop_chunk_user()->u8{7u8}fn main(){println!("{}",ml_finite_record_loop_chunk_user());}',stdout:'7\n'},
];
const sourceVersionRange='>=1.90.0 <1.100.0';
const record='struct Record{text:String,count:u8}';
export const rustOwnedRecordLoopOriginals=[
 {name:'zero one and complete u8 domain use actual progress rather than fuel',sourceVersionRange,stdout:'0\n0\n0\n',source:record+'fn drain(start:u8)->u8{let mut value=Record{text:String::from("kept"),count:start};while value.count>0u8{value.count=value.count-1u8;}value.count}fn main(){println!("{}",drain(0u8));println!("{}",drain(1u8));println!("{}",drain(255u8));}'},
 {name:'each conditional continuing branch has its own actual decrement',sourceVersionRange,stdout:'0\n',source:record+'fn drain()->u8{let mut value=Record{text:String::from("kept"),count:3u8};while value.count!=0u8{if value.count>1u8{value.count=value.count-1u8;}else{value.count=value.count-1u8;}}value.count}fn main(){println!("{}",drain());}'},
 {name:'iteration output evaluates before the selected counter replacement',sourceVersionRange,stdout:'2\n1\n0\n',source:record+'fn drain()->u8{let mut value=Record{text:String::from("kept"),count:2u8};while value.count>0u8{println!("{}",value.count);value.count=value.count-1u8;}value.count}fn main(){println!("{}",drain());}'},
 {name:'noninert Unit body tail is evaluated on every selected iteration',sourceVersionRange,stdout:'step\nstep\n0\n',source:record+'fn announce(){println!("step");}fn drain()->u8{let mut value=Record{text:String::from("kept"),count:2u8};while value.count>0u8{value.count=value.count-1u8;announce()}value.count}fn main(){println!("{}",drain());}'},
];
export const rustOwnedRecordLoopRefusals=[
 {name:'wider counter does not inherit the u8 finite domain',sourceVersionRange,originalCompilerOutcome:'accept',source:'struct Record{text:String,count:u16}fn drain(){let mut value=Record{text:String::from("kept"),count:1u16};while value.count>0u16{value.count=value.count-1u16;}}fn main(){drain();}'},
 {name:'a missing branch decrement has no complete progress derivation',sourceVersionRange,originalCompilerOutcome:'accept',source:record+'fn drain(){let mut value=Record{text:String::from("kept"),count:1u8};while value.count>0u8{if value.count>1u8{value.count=value.count-1u8;}}}fn main(){}'},
 {name:'a copied old counter is not a current counter place witness',sourceVersionRange,originalCompilerOutcome:'accept',source:record+'fn drain(){let mut value=Record{text:String::from("kept"),count:1u8};while value.count>0u8{let old=value.count;value.count=old-1u8;}}fn main(){drain();}'},
 {name:'two writes cannot substitute for exactly one nonzero decrement',sourceVersionRange,originalCompilerOutcome:'accept',source:record+'fn drain(){let mut value=Record{text:String::from("kept"),count:2u8};while value.count>0u8{value.count=value.count-1u8;value.count=value.count-1u8;}}fn main(){drain();}'},
];
