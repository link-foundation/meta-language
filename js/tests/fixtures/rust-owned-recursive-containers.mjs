// Independently authored future assertions. No compiler/runtime observation.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustOwnedRecursiveContainerDecisions=[
 {operation:'acceptRustOwnedContainerSource',arguments:[true,90],expected:true},
 {operation:'acceptRustOwnedContainerSource',arguments:[true,99],expected:true},
 {operation:'acceptRustOwnedContainerSource',arguments:[true,100],expected:false},
 {operation:'acceptRustOwnedBoxTypeNode',arguments:[['box'],'library','std::boxed::Box',1],expected:true},
 {operation:'acceptRustOwnedBoxTypeNode',arguments:[['box'],'declaration','local::Box',1],expected:false},
 {operation:'acceptRustOwnedBoxTypeNode',arguments:[['reference','box'],'library','std::boxed::Box',1],expected:false},
 {operation:'acceptRustOwnedStandardSumBoxPayload',arguments:[['box','box'],0,'library','std::boxed::Box',1,2],expected:true},
 {operation:'acceptRustOwnedStandardSumBoxPayload',arguments:[['box','reference'],1,'library','std::boxed::Box',1,1],expected:false},
 {operation:'readRustOwnedBoxTypeError',arguments:[true,false,true,'owned',true],expected:''},
 {operation:'readRustOwnedBoxTypeError',arguments:[true,true,true,'',false],expected:''},
 {operation:'readRustOwnedBoxTypeError',arguments:[true,false,true,'',false],expected:'an owned Box source type needs its complete closed payload ownership graph'},
 {operation:'acceptRustOwnedBoxCallResult',arguments:[true,true,true,true,true],expected:true},
 {operation:'acceptRustOwnedBoxCallResult',arguments:[true,true,false,true,true],expected:false},
 {operation:'acceptRustOwnedBoxCallResult',arguments:[true,true,true,true,false],expected:false},
 {operation:'readRustOwnedBoxMoveError',arguments:[true,true,false],expected:''},
 {operation:'readRustOwnedBoxPayloadError',arguments:[false,true,true,false],expected:'stored Box payload references or callable captures need their complete lifetime/drop profile'},
 {operation:'readRustOwnedBoxPayloadError',arguments:[true,false,false,false],expected:''},
 {operation:'readRustOwnedContainerDepthError',arguments:[256,256],expected:''},
 {operation:'readRustOwnedContainerDepthError',arguments:[257,0],expected:'owned container source syntax and value lineage exceed their finite depth profile'},
 {operation:'readRustOwnedContainerDepthError',arguments:[NaN,0],expected:'owned container source syntax and value lineage exceed their finite depth profile'},
 {operation:'readRustOwnedContainerDepthError',arguments:[0,Infinity],expected:'owned container source syntax and value lineage exceed their finite depth profile'},
 {operation:'readRustOwnedContainerDepthError',arguments:[1.5,0],expected:'owned container source syntax and value lineage exceed their finite depth profile'},
 {operation:'readRustOwnedContainerRuntimeMetadataCost',arguments:['owner',4],expected:[1,9]},
 {operation:'readRustOwnedContainerRuntimeMetadataCost',arguments:['origin',0],expected:[16,256]},
];
export const rustOwnedRecursiveContainerOriginals=[
 {name:'a real Box allocation moves its String payload and explicit dereference returns that value',sourceVersionRange,stdout:'text\n',source:'fn main(){let value:Box<String>=Box::new(String::from("text"));let moved:String=*value;println!("{}",moved);}'},
 {name:'a copied scalar payload leaves its actual Box available for another dereference',sourceVersionRange,stdout:'7\n7\n',source:'fn main(){let value:Box<u8>=Box::new(7u8);println!("{}",*value);println!("{}",*value);}'},
 {name:'generic scalar Box allocations retain independent clone lineage without guessed scalar owners',sourceVersionRange,stdout:'7\n7\n',source:'fn main(){let value=Box::new(7);let other=value.clone();println!("{}",*value);println!("{}",*other);}'},
 {name:'ordinary Box return signatures require an actual recorded function call producer',sourceVersionRange,stdout:'text\n',source:'fn create()->Box<String>{Box::new(String::from("text"))}fn main(){let value:Box<String>=create();let moved:String=*value;println!("{}",moved);}'},
 {name:'standard sum field signatures retain the complete concrete Box owner',sourceVersionRange,stdout:'text\n',source:'fn main(){let value:Option<Box<String>>=Some(Box::new(String::from("text")));match value{Some(inner)=>println!("{}",*inner),None=>()}}'},
];
export const rustOwnedRecursiveContainerRefusals=[
 {name:'a Box owning String cannot be dereferenced by value twice',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn main(){let value:Box<String>=Box::new(String::from("text"));let first:String=*value;let second:String=*value;}'},
 {name:'a Box owning a Copy payload still cannot itself be moved twice',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn take(value:Box<u8>){}fn main(){let value:Box<u8>=Box::new(7u8);take(value);take(value);}'},
 {name:'an actual borrowed Box cannot move out its owned payload',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn main(){let value:Box<String>=Box::new(String::from("text"));let borrowed=&value;let moved:String=**borrowed;}'},
 {name:'stored Box references retain the separate lifetime and destructor boundary',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn main(){let value=7u8;let boxed=Box::new(&value);println!("{}",**boxed);}'},
 {name:'declaration Self does not admit an infinite direct datatype size',sourceVersionRange,originalCompilerOutcome:'reject',source:'struct Bad{value:Self}fn main(){}'},
];
// Exact next source obligations. Runtime recursive nominal admission remains
// gated until owned Vec production and complete formal mutual blocks land.
export const rustOwnedRecursiveContainerPendingOriginals=[
 {name:'declaration Self has a concrete guarded Box and empty Vec witness',sourceVersionRange,stdout:'0\n',source:'#[derive(Clone)]enum Shape{Unit,Array{element:Box<Self>},Tuple{elements:Vec<Self>}}fn main(){let shape=Shape::Array{element:Box::new(Shape::Tuple{elements:Vec::new()})};match shape{Shape::Array{element}=>match *element{Shape::Tuple{elements}=>println!("{}",elements.len()),_=>()},_=>()}}'},
 {name:'actual Expr and Node source modules retain their cross-module datatype identity',sourceVersionRange,stdout:'',source:'mod ir{pub struct Expr{pub node:nodes::Node}pub mod nodes{pub enum Node{Unit,Unary{arg:Box<super::Expr>},Array{items:Vec<super::Expr>}}}}fn main(){let value=ir::Expr{node:ir::nodes::Node::Unary{arg:Box::new(ir::Expr{node:ir::nodes::Node::Unit})}};}'},
];
