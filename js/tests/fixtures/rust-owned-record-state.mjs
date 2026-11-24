// Independent source contracts, not observations. Current Reference describes
// the mechanism; exact selected-version compilation remains a deferred oracle.
export const rustOwnedRecordStateReferences=[
 'https://doc.rust-lang.org/reference/expressions/operator-expr.html#assignment-expressions',
 'https://doc.rust-lang.org/reference/expressions.html#evaluation-order-of-operands',
 'https://doc.rust-lang.org/reference/expressions/struct-expr.html#functional-update-syntax',
];
const sourceVersionRange='>=1.90.0 <1.100.0';
const mutable='a mutable record needs its actual mutable declaration and complete destructor-free owned lexical root';
export const rustOwnedRecordStateDecisions=[
 {operation:'readRustOwnedRecordMutableBindingError',arguments:[true,true,'owned',true,true,0],expected:''},
 {operation:'readRustOwnedRecordMutableBindingError',arguments:[true,false,'owned',true,true,0],expected:mutable},
 {operation:'readRustOwnedRecordMutableBindingError',arguments:[true,true,'copy',true,true,0],expected:mutable},
 {operation:'readRustOwnedRecordMutableBindingError',arguments:[true,true,'owned',false,true,0],expected:mutable},
 {operation:'readRustOwnedRecordStateWriteError',arguments:[true,true,true,'available','available',true,false],expected:''},
 {operation:'readRustOwnedRecordStateWriteError',arguments:[true,true,true,'available','moved',true,false],expected:''},
 {operation:'readRustOwnedRecordStateWriteError',arguments:[true,true,true,'available','available',true,true],expected:'field assignment overlaps an actual surviving whole or field loan'},
 {operation:'readRustOwnedRecordStateWriteError',arguments:[true,true,true,'moved','available',true,false],expected:'field reinitialization needs its still initialized record root and exact available or moved field'},
 {operation:'readRustOwnedRecordStateWriteError',arguments:[true,false,true,'available','available',true,false],expected:'a captured mutable record needs its separate callable environment state interpretation'},
 {operation:'readRustOwnedRecordStateWriteError',arguments:[true,true,false,'available','available',true,false],expected:'this expression needs an explicit record state result continuation before an outer place can be changed'},
 {operation:'readRustOwnedRecordStateWriteError',arguments:[false,true,true,'available','available',true,false],expected:'record field assignment needs its actual mutable root binding'},
 {operation:'readRustOwnedRecordStateWriteError',arguments:[true,true,true,'available','available',false,false],expected:'field assignment must preserve its actual complete source type and owner'},
 {operation:'testRustOwnedRecordStateLoanOverlap',arguments:['7','record field 7 0',['7','record field 7 1'],true],expected:false},
 {operation:'testRustOwnedRecordStateLoanOverlap',arguments:['7','record field 7 0',['record field 7 0'],false],expected:true},
 {operation:'testRustOwnedRecordStateLoanOverlap',arguments:['7','record field record field 7 0 1',['record field 7 0'],false],expected:true},
 {operation:'testRustOwnedRecordStateLoanOverlap',arguments:['7','record field 7 0',['7'],false],expected:true},
 {operation:'testRustOwnedRecordStateLoanOverlap',arguments:['7','record field 7 0',[],true],expected:true},
 {operation:'testRustOwnedRecordStateLoanOverlap',arguments:['7','record field 7 0',['70'],false],expected:false},
 {operation:'planRustOwnedRecordStatePacket',arguments:[0],expected:[0]},
 {operation:'planRustOwnedRecordStatePacket',arguments:[2],expected:[0,1,2]},
 {operation:'planRustOwnedRecordStatePacket',arguments:[257],expected:[]},
 {operation:'planRustOwnedRecordStateFields',arguments:[3,1],expected:[0,-1,2]},
 {operation:'planRustOwnedRecordStateFields',arguments:[0,0],expected:[]},
 {operation:'readRustOwnedRecordStateFieldIdentity',arguments:['7',[0,1]],expected:'record field record field 7 0 1'},
 {operation:'readRustOwnedRecordStateAssignmentTokenCount',arguments:[['name','.','name','=','other']],expected:3},
 {operation:'readRustOwnedRecordStateAssignmentTokenCount',arguments:[['(','name',')','.','name','=','other']],expected:5},
 {operation:'readRustOwnedRecordStateAssignmentTokenCount',arguments:[['name','=','other']],expected:-1},
 {operation:'readRustOwnedRecordStateStorageDisposition',arguments:['function',0,'copy'],expected:'refuse'},
 {operation:'readRustOwnedRecordStateStorageDisposition',arguments:['string',0,'owned'],expected:'leaf'},
 {operation:'readRustOwnedRecordStateStorageDisposition',arguments:['named',0,'owned'],expected:'children'},
 {operation:'readRustOwnedRecordStateStorageDisposition',arguments:['fixed',1,'copy'],expected:'refuse'},
 {operation:'acceptRustOwnedRecordStateStorageOwner',arguments:['library','std::boxed::Box'],expected:false},
 {operation:'acceptRustOwnedRecordStateStorageOwner',arguments:['library','std::string::String'],expected:true},
 {operation:'readRustOwnedRecordStateStorageError',arguments:[65536,256],expected:''},
 {operation:'readRustOwnedRecordStateStorageError',arguments:[65537,256],expected:'the complete record state storage exceeds its finite source budget'},
 {operation:'readRustOwnedRecordStateStorageDisposition',arguments:['array',0,'owned'],expected:'children'},
 {operation:'readRustOwnedRecordStateWriteError',arguments:[true,true,true,'available','possibly moved',true,false],expected:''},
];
export const rustOwnedRecordStateOriginals=[
 {name:'an actual mutable root replaces one owning field',sourceVersionRange,stdout:'new\nright\n',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};value.left=String::from("new");println!("{}",value.left);println!("{}",value.right);}'},
 {name:'reinitializing a moved field restores legal whole root transfer',sourceVersionRange,stdout:'left\nrestored\n',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};let selected=Pair{left:String::from("new"),..value};value.right=String::from("restored");let whole=value;println!("{}",whole.left);println!("{}",whole.right);}'},
 {name:'assignment moves its own RHS before reinitializing the same field',sourceVersionRange,stdout:'left\nright\n',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};value.left=value.left;println!("{}",value.left);println!("{}",value.right);}'},
 {name:'nested field reinitialization preserves the complete ancestor path',sourceVersionRange,stdout:'restored\n7\n',source:'struct Pair{left:String,right:String}struct Outer{inner:Pair,number:u8}fn main(){let mut value=Outer{inner:Pair{left:String::from("left"),right:String::from("right")},number:7u8};let selected=Pair{left:String::from("new"),..value.inner};(value.inner).right=String::from("restored");let whole=value;println!("{}",whole.inner.right);println!("{}",whole.number);}'},
 {name:'both branches return actual reinitialized root snapshots',sourceVersionRange,stdout:'then\n',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};let selected=Pair{left:String::from("new"),..value};let flag=true;if flag{value.right=String::from("then");}else{value.right=String::from("else");}println!("{}",value.right);}'},
 {name:'nested block statements return changed outer state to their continuation',sourceVersionRange,stdout:'changed\n',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};{value.right=String::from("changed");}println!("{}",value.right);}'},
 {name:'immutable scalar shadow cannot redirect an outer record carrier',sourceVersionRange,stdout:'9\nnew\n',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};{let value=9u8;println!("{}",value);}value.left=String::from("new");println!("{}",value.left);}'},
 {name:'distinct mutable declarations sharing a spelling keep fresh IR identities',sourceVersionRange,stdout:'inner changed\nouter\n',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("outer"),right:String::from("right")};{let mut value=Pair{left:String::from("inner"),right:String::from("right")};value.left=String::from("inner changed");println!("{}",value.left);}println!("{}",value.left);}'},
 {name:'a surviving disjoint field loan permits the other field replacement',sourceVersionRange,stdout:'left\nnew\n',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};let borrowed=&value.left;value.right=String::from("new");println!("{}",borrowed);println!("{}",value.right);}'},
 {name:'the RHS factory executes once before the reconstructed place is exposed',sourceVersionRange,stdout:'rhs\nnew\n',source:'struct Pair{left:String,right:String}fn replacement()->String{println!("rhs");String::from("new")}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};value.left=replacement();println!("{}",value.left);}'},
 {name:'owned vector field replacement preserves its actual element domain',sourceVersionRange,stdout:'1\nleft\n',source:'struct Entry{text:String,values:Vec<u8>}fn main(){let mut value=Entry{text:String::from("left"),values:vec![1u8,2u8]};value.values=vec![7u8];println!("{}",value.values.len());println!("{}",value.text);}'},
 {name:'a fully initialized mutable local can become the actual function result',sourceVersionRange,stdout:'new\nright\n',source:'struct Pair{left:String,right:String}fn changed()->Pair{let mut value=Pair{left:String::from("left"),right:String::from("right")};value.left=String::from("new");value}fn main(){let value=changed();println!("{}",value.left);println!("{}",value.right);}'},
 {name:'unconditional reinitialization restores a field possibly moved by a branch',sourceVersionRange,stdout:'restored\n',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};if true{let moved=value.right;}value.right=String::from("restored");println!("{}",value.right);}'},
];
// Preserve every old coordinate before the additive ordered-expression profile
// admits row seven. Current refusals retain explicit predecessor coordinates.
export const rustOwnedRecordStatePreviousRefusals=[
 {name:'immutable root cannot be reinitialized',sourceVersionRange,originalCompilerOutcome:'reject',source:'struct Pair{left:String,right:String}fn main(){let value=Pair{left:String::from("left"),right:String::from("right")};value.left=String::from("new");}'},
 {name:'wrong field source type cannot replace an owning String',sourceVersionRange,originalCompilerOutcome:'reject',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};value.left=7u8;}'},
 {name:'a same field loan prevents replacement before later reference use',sourceVersionRange,originalCompilerOutcome:'reject',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};let borrowed=&value.left;value.left=String::from("new");println!("{}",borrowed);}'},
 {name:'a whole record loan prevents overlapping field replacement',sourceVersionRange,originalCompilerOutcome:'reject',source:'struct Pair{left:String,right:String}fn observe(value:&Pair){}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};let borrowed=&value;value.left=String::from("new");observe(borrowed);}'},
 {name:'one branch reinitialization cannot restore a maybe moved field',sourceVersionRange,originalCompilerOutcome:'reject',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};let selected=Pair{left:String::from("new"),..value};if true{value.right=String::from("restored");}println!("{}",value.right);}'},
 {name:'a moved intermediate record cannot be reinitialized through a child',sourceVersionRange,originalCompilerOutcome:'reject',source:'struct Pair{left:String,right:String}struct Outer{inner:Pair}fn main(){let mut value=Outer{inner:Pair{left:String::from("left"),right:String::from("right")}};let moved=value.inner;value.inner.left=String::from("new");println!("{}",value.inner.right);}'},
 {name:'old mutable scalar boundary retains its exact complete input',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn main(){let mut value=1;}'},
 {name:'uncomposed initializer state effects retain their valid source',sourceVersionRange,originalCompilerOutcome:'accept',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};let flag={value.left=String::from("new");true};println!("{}",flag);}'},
 {name:'mutable captured environments retain the distinct FnMut boundary',sourceVersionRange,originalCompilerOutcome:'accept',source:'struct Pair{left:String,right:String}fn main(){let mut value=Pair{left:String::from("left"),right:String::from("right")};let mut change=||{value.left=String::from("new");};change();}'},
 {name:'Copy record mutation retains its separate existing state boundary',sourceVersionRange,originalCompilerOutcome:'accept',source:'#[derive(Clone,Copy)]struct Number{value:u8}fn main(){let mut value=Number{value:1u8};value.value=2u8;}'},
];
export const rustOwnedRecordStateRefusalHistory=rustOwnedRecordStatePreviousRefusals.map((control,previousIndex)=>({previousFixture:'rustOwnedRecordStateRefusals',previousIndex,control}));
export const rustOwnedRecordStateRefusals=rustOwnedRecordStateRefusalHistory.filter(row=>row.previousIndex!==7).map(row=>({...row.control,previousFixture:row.previousFixture,previousIndex:row.previousIndex}));
