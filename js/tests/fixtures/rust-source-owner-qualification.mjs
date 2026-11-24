// Complete source inputs and independent expectations; no observations.
const sourceVersionRange='>=1.90.0 <1.100.0';
const declarations='struct Later{number:u8}mod domain{pub fn one()->Later{Later{text:String::from("forward")}}pub struct Later{pub text:String}}';
export const rustSourceOwnerQualificationOriginals=[
 {name:'forward standard sum owners retain their declaration namespace under a caller shadow',sourceVersionRange,stdout:'forward\n',source:'struct Later{number:u8}mod domain{pub fn make()->Option<Later>{Some(Later{text:String::from("forward")})}pub struct Later{pub text:String}}fn read(value:Option<domain::Later>)->String{match value{Some(inner)=>inner.text,None=>String::from("none")}}fn main(){println!("{}",read(domain::make()));}'},
 {name:'a real foreign function result becomes a caller vector without changing its nominal owner',sourceVersionRange,stdout:'1\n',source:declarations+'fn take(values:Vec<domain::Later>){println!("{}",values.len());}fn main(){let values=vec![domain::one()];take(values);}'},
 {name:'Box preserves its outer owner while qualifying a nested vector element',sourceVersionRange,stdout:'1\n',source:declarations+'fn take(value:Box<Vec<domain::Later>>){let values=*value;println!("{}",values.len());}fn main(){take(Box::new(vec![domain::one()]));}'},
 {name:'contextual empty vectors keep the actual forward declaration owner',sourceVersionRange,stdout:'0\n',source:'struct Later{number:u8}mod domain{pub fn empty()->Vec<Later>{Vec::new()}pub struct Later{pub text:String}}fn main(){let values:Vec<domain::Later>=domain::empty();println!("{}",values.len());}'},
];
export const rustSourceOwnerQualificationRefusals=[
 {name:'same-spelled declarations in distinct scopes are not interchangeable vector elements',sourceVersionRange,originalCompilerOutcome:'reject',source:declarations+'fn take(values:Vec<Later>){}fn main(){take(vec![domain::one()]);}'},
 {name:'a returned vector cannot relabel a foreign declaration as its caller shadow',sourceVersionRange,originalCompilerOutcome:'reject',source:declarations+'fn invalid(values:Vec<domain::Later>)->Vec<Later>{values}fn main(){}'},
];
