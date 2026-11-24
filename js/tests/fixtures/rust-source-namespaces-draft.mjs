// Authored originals and exact expectations, deferred until the full draft.
// Namespace assertions are bounded by the active source range; no observations.
export const sourceNamespaceOriginals=[
 {name:'function Option does not shadow qualified type constructors',source:'fn Option()->u8{9u8} fn main(){let value:Option<u8>=Option::Some(7u8);println!("{}",value.unwrap_or(0u8));println!("{}",Option());}',stdout:'7\n9\n'},
 {name:'local value Option does not shadow qualified type constructors',source:'fn main(){let Option=9u8;let value:Option<u8>=Option::Some(7u8);println!("{}",value.unwrap_or(0u8));println!("{}",Option);}',stdout:'7\n9\n'},
 {name:'functions std and core do not shadow type namespace roots',source:'fn std()->u8{3u8} fn core()->u8{4u8} type Maybe<T>=std::option::Option<T>; fn main(){let value:Maybe<u8>=core::option::Option::Some(7u8);println!("{}",value.unwrap_or(0u8));println!("{}",std());println!("{}",core());}',stdout:'7\n3\n4\n'},
 {name:'module followed by function retains both std namespace entries',source:'mod std{pub fn value()->u8{3u8}} fn std()->u8{4u8} fn main(){let value:Option<u8>=Option::Some(7u8);println!("{}",value.unwrap_or(0u8));println!("{}",std::value());println!("{}",std());}',stdout:'7\n3\n4\n'},
 {name:'function followed by module retains both std namespace entries',source:'fn std()->u8{4u8} mod std{pub fn value()->u8{3u8}} fn main(){let value:Option<u8>=Option::Some(7u8);println!("{}",value.unwrap_or(0u8));println!("{}",std::value());println!("{}",std());}',stdout:'7\n3\n4\n'},
 {name:'imported type alias ignores local value homograph',source:'use std::option::Option as Maybe; fn main(){let Maybe=9u8;let value:Maybe<u8>=Maybe::Some(7u8);println!("{}",value.unwrap_or(0u8));println!("{}",Maybe);}',stdout:'7\n9\n'},
];
export const sourceNamespaceRefusals=[
 {name:'actual std module blocks canonical library root',source:'mod std{pub mod option{pub enum Option{Other}}} type Maybe<T>=std::option::Option<T>; fn main(){}'},
 {name:'actual core module blocks canonical library root',source:'mod core{pub mod option{pub enum Option{Other}}} type Maybe<T>=core::option::Option<T>; fn main(){}'},
 {name:'forward type root shadows standard Option',source:'fn main(){let value:Option<u8>=Option::Some(7u8);} mod Option{}'},
 {name:'bare Some function keeps value shadow',source:'fn Some(value:u8)->u8{value} fn main(){let value:Option<u8>=Some(7u8);println!("{}",value.unwrap_or(0u8));}'},
 {name:'bare Some local keeps value shadow',source:'fn main(){let Some=9u8;let value:Option<u8>=Some(7u8);println!("{}",value.unwrap_or(0u8));}'},
];
export const sourceNamespaceVersionRange='>=1.90.0 <1.100.0';
