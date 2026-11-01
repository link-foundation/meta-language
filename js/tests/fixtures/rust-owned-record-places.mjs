// Independent place syntax and complete Rust inputs; execution is deferred.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustOwnedRecordPlaceDecisions=[
 {operation:'readRustOwnedRecordPlaceDepthLimit',arguments:[],expected:256},
 {operation:'readRustOwnedRecordPlaceTokenCount',arguments:[['name','}']],expected:1},
 {operation:'readRustOwnedRecordPlaceTokenCount',arguments:[['(','name',')','}']],expected:3},
 {operation:'readRustOwnedRecordPlaceTokenCount',arguments:[['(','name','.','name',')','.','name','}']],expected:7},
 {operation:'readRustOwnedRecordPlaceTokenCount',arguments:[['name','.','number','}']],expected:3},
 {operation:'readRustOwnedRecordPlaceTokenCount',arguments:[['(','(','name',')',')','}']],expected:5},
 {operation:'readRustOwnedRecordPlaceTokenCount',arguments:[['name','(',')','}']],expected:-1},
 {operation:'readRustOwnedRecordPlaceTokenCount',arguments:[['{','name','}','}']],expected:-1},
 {operation:'readRustOwnedRecordPlaceTokenCount',arguments:[['(','name','}']],expected:-1},
 {operation:'readRustOwnedRecordPlaceTokenCount',arguments:[['name','.','}']],expected:-1},
 {operation:'readRustOwnedRecordPlaceTokenCount',arguments:[['number','}']],expected:-1},
 {operation:'readRustOwnedRecordPlaceTokenCount',arguments:[['name','other','name','}']],expected:-1},
];
export const rustOwnedRecordPlaceOriginals=[
 {name:'parenthesized partial place preserves an unselected owning field',sourceVersionRange,stdout:'left\nright\n',source:'struct Pair{left:String,right:String}fn main(){let first=Pair{left:String::from("left"),right:String::from("right")};let second=Pair{left:String::from("new"),..(first)};let third=Pair{right:String::from("other"),..((first))};println!("{}",third.left);println!("{}",second.right);}'},
 {name:'nested owning place selects only actual omitted child fields',sourceVersionRange,stdout:'left\nright\n7\n',source:'struct Pair{left:String,right:String}struct Outer{inner:Pair,number:u8}fn main(){let first=Outer{inner:Pair{left:String::from("left"),right:String::from("right")},number:7u8};let second=Pair{left:String::from("new"),..first.inner};println!("{}",first.inner.left);println!("{}",second.right);println!("{}",first.number);}'},
 {name:'parentheses around nested place boundaries retain exact field identity',sourceVersionRange,stdout:'left\nright\n',source:'struct Pair{left:String,right:String}struct Outer{inner:Pair}fn main(){let first=Outer{inner:Pair{left:String::from("left"),right:String::from("right")}};let second=Pair{left:String::from("new"),..((first).inner)};let third=Pair{right:String::from("other"),..(first.inner)};println!("{}",third.left);println!("{}",second.right);}'},
 {name:'nested branch absence inherits real available ancestor states',sourceVersionRange,stdout:'left\n7\n',source:'struct Pair{left:String,right:String}struct Outer{inner:Pair,number:u8}fn main(){let first=Outer{inner:Pair{left:String::from("left"),right:String::from("right")},number:7u8};if true{let second=Pair{left:String::from("new"),..first.inner};}let third=Pair{right:String::from("other"),..first.inner};println!("{}",third.left);println!("{}",first.number);}'},
 {name:'parenthesized whole available value expression keeps ordinary base behavior',sourceVersionRange,stdout:'new\nright\n',source:'struct Pair{left:String,right:String}fn base()->Pair{Pair{left:String::from("left"),right:String::from("right")}}fn main(){let value=Pair{left:String::from("new"),..(base())};println!("{}",value.left);println!("{}",value.right);}'},
 {name:'replacing nested partial field copies only a remaining outer scalar',sourceVersionRange,stdout:'new\n7\n',source:'struct Pair{left:String,right:String}struct Outer{inner:Pair,number:u8}fn main(){let first=Outer{inner:Pair{left:String::from("left"),right:String::from("right")},number:7u8};let second=Pair{left:String::from("new"),..first.inner};let third=Outer{inner:Pair{left:String::from("new"),right:String::from("other")},..(first)};println!("{}",third.inner.left);println!("{}",third.number);}'},
];
export const rustOwnedRecordPlaceRefusals=[
 {name:'parentheses do not restore a moved field',sourceVersionRange,originalCompilerOutcome:'reject',source:'struct Pair{left:String,right:String}fn main(){let first=Pair{left:String::from("left"),right:String::from("right")};let second=Pair{left:String::from("new"),..first};let third=Pair{left:String::from("other"),..(first)};}'},
 {name:'a nested partially moved owner cannot escape as a whole',sourceVersionRange,originalCompilerOutcome:'reject',source:'struct Pair{left:String,right:String}struct Outer{inner:Pair}fn main(){let first=Outer{inner:Pair{left:String::from("left"),right:String::from("right")}};let second=Pair{left:String::from("new"),..first.inner};let again=first;}'},
 {name:'nested maybe moved child stays unavailable after branches',sourceVersionRange,originalCompilerOutcome:'reject',source:'struct Pair{left:String,right:String}struct Outer{inner:Pair}fn main(){let first=Outer{inner:Pair{left:String::from("left"),right:String::from("right")}};if true{let second=Pair{left:String::from("new"),..first.inner};}println!("{}",first.inner.right);}'},
 {name:'arbitrary whole value blocks cannot launder a partial place',sourceVersionRange,originalCompilerOutcome:'reject',source:'struct Pair{left:String,right:String}fn main(){let first=Pair{left:String::from("left"),right:String::from("right")};let second=Pair{left:String::from("new"),..first};let third=Pair{right:String::from("other"),..{first}};}'},
];
