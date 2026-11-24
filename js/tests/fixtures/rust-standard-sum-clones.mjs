// Independent ordinary-value source controls, not runtime observations.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustStandardSumCloneDecisions=[
 {operation:'readRustStandardSumCloneSourceError',arguments:['std::option::Option','copy',0,false,false,false,false],expected:''},
 {operation:'readRustStandardSumCloneSourceError',arguments:['std::option::Option','owned',0,false,true,true,false],expected:''},
 {operation:'readRustStandardSumCloneSourceError',arguments:['std::result::Result','owned',0,false,true,false,true],expected:''},
 {operation:'readRustStandardSumCloneSourceError',arguments:['std::option::Option','owned',0,false,true,false,false],expected:'standard sum clone needs an available actual owner or an actual fresh producer, independently of its type'},
 {operation:'readRustStandardSumCloneSourceError',arguments:['std::option::Option','owned',0,false,false,false,true],expected:'standard sum clone needs an available actual owner or an actual fresh producer, independently of its type'},
 {operation:'readRustStandardSumCloneSourceError',arguments:['std::option::Option','',0,false,true,true,true],expected:'standard sum clone needs an available actual owner or an actual fresh producer, independently of its type'},
 {operation:'readRustStandardSumCloneSourceError',arguments:['local::Option','owned',0,false,true,true,true],expected:'standard sum clone needs its actual direct ordinary value source'},
 {operation:'readRustStandardSumCloneSourceError',arguments:['std::option::Option','owned',1,false,true,true,true],expected:'standard sum clone needs its actual direct ordinary value source'},
 {operation:'readRustStandardSumCloneSourceError',arguments:['std::option::Option','owned',0,true,true,true,true],expected:'standard sum clone needs its actual direct ordinary value source'},
];
export const rustStandardSumCloneOriginals=[
 {name:'owned Option clone preserves both independent source owners',sourceVersionRange,stdout:'text\ntext\n',source:'fn main(){let original:Option<String>=Some(String::from("text"));let cloned=original.clone();println!("{}",original.unwrap_or_default());println!("{}",cloned.unwrap_or_default());}'},
 {name:'owned Result clone preserves actual error payload and original receiver',sourceVersionRange,stdout:'error\nerror\n',source:'fn main(){let original:Result<String,String>=Err(String::from("error"));let cloned=original.clone();println!("{}",original.err().unwrap_or_default());println!("{}",cloned.err().unwrap_or_default());}'},
 {name:'nested vector clone keeps both actual constructor layers',sourceVersionRange,stdout:'2\n2\n',source:'fn main(){let original:Option<Option<Vec<u8>>>=Some(Some(vec![7u8,9u8]));let cloned=original.clone();println!("{}",original.flatten().unwrap_or_default().len());println!("{}",cloned.flatten().unwrap_or_default().len());}'},
 {name:'Copy standard sum clone keeps ordinary repeated source reads',sourceVersionRange,stdout:'7\n7\ntrue\n',source:'fn main(){let original:Option<u8>=Some(7u8);let cloned=original.clone();println!("{}",cloned.unwrap_or_default());println!("{}",original.unwrap_or_default());println!("{}",original.is_some());}'},
 {name:'cloning a fresh owned producer evaluates that producer once',sourceVersionRange,stdout:'receiver\ntext\n',source:'fn make()->Option<String>{println!("receiver");Some(String::from("text"))}fn main(){println!("{}",make().clone().unwrap_or_default());}'},
];
export const rustStandardSumCloneRefusals=[
 {name:'standard sum clone cannot read an already moved original',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn main(){let value:Option<String>=Some(String::from("text"));let text=value.unwrap_or_default();let cloned=value.clone();}'},
 {name:'standard sum clone retains its exact zero-argument signature',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn main(){let value:Option<String>=Some(String::from("text"));let cloned=value.clone(7u8);}'},
 {name:'cloned owning result cannot be consumed twice',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn main(){let value:Option<String>=Some(String::from("text"));let cloned=value.clone();let first=cloned.unwrap_or_default();let second=cloned.unwrap_or_default();}'},
];
