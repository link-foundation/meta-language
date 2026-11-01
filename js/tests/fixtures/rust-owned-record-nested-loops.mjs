// Independent literal source contracts; endpoint execution is deferred.
const sourceVersionRange='>=1.90.0 <1.100.0';
const head='struct State{text:String,outer:u8,inner:u8}fn run()->u8{let mut state=State{text:String::from("keep"),outer:2u8,inner:0u8};';
const tail='state.outer}fn main(){println!("{}",run());}';
export const rustOwnedRecordNestedLoopDecisions=[
 {operation:'readRustOwnedRecordNestedLoopError',arguments:[true,0,false],expected:''},
 {operation:'readRustOwnedRecordNestedLoopError',arguments:[false,0,false],expected:'a nested loop needs its actual complete finite source admission'},
 {operation:'readRustOwnedRecordNestedLoopError',arguments:[true,-1,false],expected:'a nested loop needs its actual complete finite source admission'},
 {operation:'readRustOwnedRecordNestedLoopError',arguments:[true,1,false],expected:'a nested function return needs its complete enclosing loop exit continuation'},
 {operation:'readRustOwnedRecordNestedLoopError',arguments:[true,0,true],expected:'a nested loop cannot write its enclosing progress counter'},
 {operation:'readRustOwnedRecordNestedLoopDepthError',arguments:[8],expected:''},
 {operation:'readRustOwnedRecordNestedLoopDepthError',arguments:[9],expected:'the actual nested finite loops exceed their source depth budget'},
 {operation:'readRustOwnedRecordNestedLoopDepthError',arguments:[-1],expected:'the actual nested finite loops exceed their source depth budget'},
 {operation:'readRustOwnedRecordNestedCompletionError',arguments:[true,false],expected:''},
 {operation:'readRustOwnedRecordNestedCompletionError',arguments:[false,false],expected:'a nested loop needs its actual complete finite source admission'},
 {operation:'readRustOwnedRecordNestedCompletionError',arguments:[true,true],expected:'a nested loop cannot write its enclosing progress counter'},
 {operation:'readRustOwnedRecordLoopFootprintError',arguments:[100000,16777216],expected:''},
 {operation:'readRustOwnedRecordLoopFootprintError',arguments:[100001,0],expected:'the complete loop write footprint exceeds its source budget'},
 {operation:'readRustOwnedRecordLoopFootprintError',arguments:[0,16777217],expected:'the complete loop write footprint exceeds its source budget'},
];
export const rustOwnedRecordNestedLoopOriginals=[
 {name:'nested exact u8 loops advance independent record fields',sourceVersionRange,stdout:'1\n0\n1\n0\n0\n',source:head+'while state.outer>0u8{state.outer=state.outer-1u8;state.inner=2u8;while state.inner>0u8{state.inner=state.inner-1u8;println!("{}",state.inner);}}'+tail},
 {name:'an inner break exits only its nearest loop',sourceVersionRange,stdout:'2\n2\n0\n',source:head+'while state.outer>0u8{state.outer=state.outer-1u8;state.inner=3u8;while state.inner>0u8{if state.inner==2u8{break;}state.inner=state.inner-1u8;}println!("{}",state.inner);}'+tail},
 {name:'an inner continue retains its own exact progress',sourceVersionRange,stdout:'0\n0\n0\n',source:head+'while state.outer>0u8{state.outer=state.outer-1u8;state.inner=2u8;while state.inner>0u8{state.inner=state.inner-1u8;if state.inner>0u8{continue;}println!("{}",state.inner);}}'+tail},
 {name:'an inner function return crosses both actual loop continuations',sourceVersionRange,stdout:'9\n',source:head+'while state.outer>0u8{state.outer=state.outer-1u8;state.inner=1u8;while state.inner>0u8{return 9u8;}}'+tail},
 {name:'an inner owned return preserves its authentic selected field producer',sourceVersionRange,stdout:'keep\n',source:'struct State{text:String,outer:u8,inner:u8}fn run()->String{let mut state=State{text:String::from("keep"),outer:2u8,inner:0u8};while state.outer>0u8{state.outer=state.outer-1u8;state.inner=1u8;while state.inner>0u8{return state.text;}}String::from("zero")}fn main(){println!("{}",run());}'},
];
export const rustOwnedRecordNestedLoopPreviousRefusals=[
 {name:'an inner counter cannot supply a variable number of outer decrements',sourceVersionRange,originalCompilerOutcome:'accept',source:head+'while state.outer>0u8{state.inner=1u8;while state.inner>0u8{state.inner=state.inner-1u8;state.outer=state.outer-1u8;}}'+tail},
 {name:'an inner function return retains its separate enclosing continuation boundary',sourceVersionRange,originalCompilerOutcome:'accept',source:head+'while state.outer>0u8{state.outer=state.outer-1u8;state.inner=1u8;while state.inner>0u8{return 9u8;}}'+tail},
 {name:'nested loops do not establish an absent outer progress step',sourceVersionRange,originalCompilerOutcome:'accept',source:head+'while state.outer>0u8{state.inner=1u8;while state.inner>0u8{state.inner=state.inner-1u8;}}'+tail},
];
export const rustOwnedRecordNestedLoopRefusalHistory=rustOwnedRecordNestedLoopPreviousRefusals.map((control,previousIndex)=>({previousFixture:'rustOwnedRecordNestedLoopRefusals',previousIndex,control}));
export const rustOwnedRecordNestedLoopRefusals=rustOwnedRecordNestedLoopRefusalHistory.filter(row=>row.previousIndex!==1).map(row=>({...row.control,previousFixture:row.previousFixture,previousIndex:row.previousIndex}));
