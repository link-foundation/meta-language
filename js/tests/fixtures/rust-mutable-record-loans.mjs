// Deferred contracts. Literal Rust inputs and outputs are independently authored;
// selected compiler endpoints remain future oracles, never observations here.
const sourceVersionRange='>=1.90.0 <1.100.0';
const declaration='struct Record{text:String,count:u8}';
const root='let mut record=Record{text:String::from("text"),count:1u8};';
export const rustMutableRecordLoanDecisions=[
 {operation:'acceptRustMutableRecordValueLoans',arguments:[true,false,true,0],expected:true},
 {operation:'acceptRustMutableRecordValueLoans',arguments:[false,true,false,0],expected:true},
 {operation:'acceptRustMutableRecordValueLoans',arguments:[false,true,true,0],expected:false},
 {operation:'acceptRustMutableRecordValueLoans',arguments:[false,false,true,1],expected:true},
 {operation:'selectRustMutableRecordLoanRoot',arguments:['7','lexical 8',['record field 7 1']],expected:true},
 {operation:'selectRustMutableRecordLoanRoot',arguments:['7','lexical 8',['record field 70 1']],expected:false},
 {operation:'selectRustMutableRecordLoanRoot',arguments:['7','lexical 8',['lexical 8']],expected:true},
 {operation:'readRustMutableRecordLoanIdentity',arguments:[9],expected:'mutable record loan 9'},
 {operation:'readRustMutableRecordLoanIdentity',arguments:[-1],expected:''},
 {operation:'readRustMutableRecordLoanIdentity',arguments:[1.5],expected:''},
 {operation:'readRustMutableRecordLoanBudgetError',arguments:[0,0,100000,16777216],expected:''},
 {operation:'readRustMutableRecordLoanBudgetError',arguments:[1,0,100000,0],expected:'the actual mutable record value loans exceed their source budget'},
 {operation:'readRustMutableRecordLoanBudgetError',arguments:[0,1,0,16777216],expected:'the actual mutable record value loans exceed their source budget'},
 {operation:'acceptRustMutableRecordLoanEscape',arguments:[true,true,true],expected:true},
 {operation:'acceptRustMutableRecordLoanEscape',arguments:[false,true,true],expected:false},
 {operation:'acceptRustMutableRecordLoanEscape',arguments:[true,false,true],expected:false},
 {operation:'acceptRustMutableRecordLoanEscape',arguments:[true,true,false],expected:false},
 {operation:'readRustMutableRecordLoanMoveError',arguments:[true,true],expected:'an owned move overlaps an actual surviving mutable record value loan'},
 {operation:'readRustMutableRecordLoanMoveError',arguments:[true,false],expected:''},
 {operation:'readRustMutableRecordLoanMoveError',arguments:[false,true],expected:''},
];
export const rustMutableRecordLoanOriginals=[
 {name:'an owned text field borrow permits changing a disjoint Copy field',sourceVersionRange,stdout:'text\n2\n',source:declaration+'fn main(){'+root+'let hold=&record.text;record.count=2u8;println!("{}",hold);println!("{}",record.count);}'},
 {name:'a reference to one field permits changing a disjoint field',sourceVersionRange,stdout:'1\nchanged\n',source:declaration+'fn main(){'+root+'let hold=&record.count;record.text=String::from("changed");println!("{}",*hold);println!("{}",record.text);}'},
 {name:'leaving an inner reference scope ends its loan',sourceVersionRange,stdout:'1\n2\n',source:declaration+'fn main(){'+root+'{let hold=&record.count;println!("{}",*hold);}record.count=2u8;println!("{}",record.count);}'},
 {name:'a completed call releases an unreturned reference argument',sourceVersionRange,stdout:'1\n2\n',source:declaration+'fn observe(value:&u8){println!("{}",*value);}fn main(){'+root+'observe(&record.count);record.count=2u8;println!("{}",record.count);}'},
 {name:'a later call argument can change a field disjoint from an earlier loan',sourceVersionRange,stdout:'1\n3\nchanged\n',source:declaration+'fn observe(value:&u8,other:u8){println!("{}",*value);println!("{}",other);}fn main(){'+root+'observe(&record.count,{record.text=String::from("changed");3u8});println!("{}",record.text);}'},
 {name:'a completed shared receiver call releases its whole root loan',sourceVersionRange,stdout:'1\n2\n',source:declaration+'impl Record{fn observe(&self){println!("{}",self.count);}}fn main(){'+root+'record.observe();record.count=2u8;println!("{}",record.count);}'},
 {name:'moving a disjoint owned field preserves a Copy field reference',sourceVersionRange,stdout:'text\n1\n',source:declaration+'fn main(){'+root+'let hold=&record.count;let text=record.text;println!("{}",text);println!("{}",*hold);}'},
];
export const rustMutableRecordLoanRefusals=[
 {name:'shadowing a live outer reference cannot hide its loan',sourceVersionRange,originalCompilerOutcome:'reject',source:declaration+'fn main(){'+root+'let hold=&record.count;{let hold=0u8;record.count=2u8;}println!("{}",*hold);}'},
 {name:'shadowing a live outer reference cannot permit moving its root',sourceVersionRange,originalCompilerOutcome:'reject',source:declaration+'fn main(){'+root+'let hold=&record.count;{let hold=0u8;let moved=record;println!("{}",moved.count);}println!("{}",*hold);}'},
 {name:'an earlier temporary argument loan survives later argument evaluation',sourceVersionRange,originalCompilerOutcome:'reject',source:declaration+'fn observe(value:&u8,other:u8){println!("{}",*value);println!("{}",other);}fn main(){'+root+'observe(&record.count,{record.count=2u8;3u8});}'},
 {name:'a shared receiver borrows the whole root before arguments',sourceVersionRange,originalCompilerOutcome:'reject',source:declaration+'impl Record{fn observe(&self,other:u8){println!("{}",self.count);println!("{}",other);}}fn main(){'+root+'record.observe({record.count=2u8;3u8});}'},
 {name:'an inner block returning a reference retains its outer field loan',sourceVersionRange,originalCompilerOutcome:'reject',source:declaration+'fn main(){'+root+'let hold={&record.count};record.count=2u8;println!("{}",*hold);}'},
 {name:'a reference to a local mutable record cannot escape the function',sourceVersionRange,originalCompilerOutcome:'reject',source:declaration+'fn invalid()->&u8{'+root+'&record.count}fn main(){}'},
];
