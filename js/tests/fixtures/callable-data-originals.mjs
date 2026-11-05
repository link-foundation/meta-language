// Original source obligations; no compiler or runtime observations.
export const callableDataSourceVersionRange='>=1.90.0 <1.100.0';
export const callableDataOriginals=[
 {name:'Option stores extracts and invokes a named generic Mapper',source:'type Mapper<T,U>=fn(T)->U;fn widen(value:u8)->u16{u16::from(value)}fn main(){let map:Option<Mapper<u8,u16>>=Some(widen);match map{Some(read)=>println!("{}",read(7u8)),None=>println!("none")}}',stdout:'7\n'},
 {name:'Result keeps the actual callable success field and scalar error owner',source:'fn widen(value:u8)->u16{u16::from(value)}fn main(){let map:Result<fn(u8)->u16,u8>=Ok(widen);match map{Ok(read)=>println!("{}",read(7u8)),Err(error)=>println!("{}",error)}}',stdout:'7\n'},
 {name:'nominal callable data uses pattern extraction without equality or debug',source:'enum Callback{Value(fn(u8)->u16)}fn widen(value:u8)->u16{u16::from(value)}fn main(){let map=Callback::Value(widen);match map{Callback::Value(read)=>println!("{}",read(7u8))}}',stdout:'7\n'},
 {name:'acyclic nested nominal callable data retains field representation',source:'enum Callback{Value(fn(u8)->u16)}enum Wrapped{Value(Callback)}fn widen(value:u8)->u16{u16::from(value)}fn main(){let map=Wrapped::Value(Callback::Value(widen));match map{Wrapped::Value(Callback::Value(read))=>println!("{}",read(7u8))}}',stdout:'7\n'},
 {name:'self recursive callable chain retains an actual finite terminal constructor',source:'enum Chain{End,Link(fn(u8)->u8,Box<Chain>)}fn read(value:u8)->u8{value}fn main(){let chain=Chain::Link(read,Box::new(Chain::End));match chain{Chain::Link(callback,tail)=>println!("{}",callback(7u8)),Chain::End=>println!("empty")}}',stdout:'7\n'},
 {name:'self recursive default witness need not be the first constructor',source:'enum Chain{Link(fn(u8)->u8,Box<Chain>),End}fn read(value:u8)->u8{value}fn main(){let chain=Chain::Link(read,Box::new(Chain::End));match chain{Chain::Link(callback,tail)=>println!("{}",callback(7u8)),Chain::End=>println!("empty")}}',stdout:'7\n'},
];
export const callableDataRecursiveRefusals=[
 {name:'recursive occurrence in callable domain lacks Lean strict positivity',source:'enum Negative{End,More(fn(Negative)->u8)}fn main(){}'},
 {name:'positive recursive callable container without a finite constructor witness remains explicit',source:'enum Endless{More(fn(u8)->u8,Box<Endless>)}fn main(){}'},
 {name:'mutually recursive callable containers retain their independent backend obligation',source:'enum First{End,More(fn(u8)->u8,Box<Second>)}enum Second{More(Box<First>)}fn main(){}'},
];
export const callableDataObservationRefusals=[
 {name:'assert equality cannot observe an aggregate function identity',source:'fn read(value:u8)->u8{value}fn main(){let value:Option<fn(u8)->u8>=Some(read);assert_eq!(value,value);}'},
 {name:'debug printing cannot observe an aggregate function representation',source:'fn read(value:u8)->u8{value}fn main(){let value:Option<fn(u8)->u8>=Some(read);println!("{:?}",value);}'},
];
