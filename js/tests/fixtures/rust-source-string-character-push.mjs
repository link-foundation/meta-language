// Complete literal source obligations, not execution observations.
export const rustSourceStringCharacterPushVersions={rust:'>=1.90.0 <1.100.0',node:'>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0'};
export const rustSourceStringCharacterPushOriginals=[
 {name:'ascii',source:'fn main(){let mut value=String::from("a");value.push(\'b\');println!("{}",value=="ab");}',stdout:'true\n'},
 {name:'non-ascii-scalar',source:'fn main(){let mut value=String::from("a");value.push(\'é\');println!("{}",value=="aé");}',stdout:'true\n'},
 {name:'supplementary-scalar',source:'fn main(){let mut value=String::from("a");value.push(\'😀\');println!("{}",value=="a😀");}',stdout:'true\n'},
 {name:'zero-character',source:'fn main(){let mut value=String::from("a");value.push(\'\\0\');println!("{}",value=="a\\0");}',stdout:'true\n'},
 {name:'newline-character',source:'fn main(){let mut value=String::from("a");value.push(\'\\n\');println!("{}",value=="a\\n");}',stdout:'true\n'},
 {name:'unicode-escape',source:'fn main(){let mut value=String::from("a");value.push(\'\\u{1f600}\');println!("{}",value=="a😀");}',stdout:'true\n'},
 {name:'quote-character',source:'fn main(){let mut value=String::from("");value.push(\'\\\'\');println!("{}",value=="\'");}',stdout:'true\n'},
 {name:'backslash-character',source:'fn main(){let mut value=String::from("");value.push(\'\\\\\');println!("{}",value=="\\\\");}',stdout:'true\n'},
 {name:'successive-calls',source:'fn main(){let mut value=String::from("");value.push(\'a\');value.push(\'😀\');value.push(\'é\',);println!("{}",value=="a😀é");}',stdout:'true\n'},
 {name:'clear-then-push',source:'fn main(){let mut value=String::from("old");value.clear();value.push(\'x\');println!("{}",value=="x");}',stdout:'true\n'},
 {name:'actual-record-field-place',source:'struct Buffer{text:String,count:u8} fn main(){let mut value=Buffer{text:String::from("a"),count:7};value.text.push(\'b\');println!("{}",value.text=="ab"&&value.count==7);}',stdout:'true\n'},
 {name:'exclusive-method-receiver',source:'struct Buffer{text:String} impl Buffer{fn append(&mut self){self.text.push(\'😀\');}} fn main(){let mut value=Buffer{text:String::from("a")};value.append();println!("{}",value.text=="a😀");}',stdout:'true\n'},
 {name:'zero-byte-boundary-prefix',source:'fn main(){let mut value=String::from("rest");value.insert(0,\'x\');println!("{}",value=="xrest");}',stdout:'true\n'},
 {name:'zero-explicit-usize-boundary',source:'fn main(){let mut value=String::from("é");value.insert(0usize,\'😀\');println!("{}",value=="😀é");}',stdout:'true\n'},
 {name:'empty-prefix-then-append',source:'fn main(){let mut value=String::from("");value.insert(0,\'a\');value.push(\'b\');println!("{}",value=="ab");}',stdout:'true\n'},
 {name:'prefix-record-field',source:'struct Buffer{text:String,count:u8} fn main(){let mut value=Buffer{text:String::from("rest"),count:7};value.text.insert(0,\'x\');println!("{}",value.text=="xrest"&&value.count==7);}',stdout:'true\n'},
];
export const rustSourceStringCharacterPushRefusals=[
 {name:'immutable-place',source:'fn main(){let value=String::from("");value.push(\'a\');}'},
 {name:'byte-is-not-character',source:'fn main(){let mut value=String::from("");value.push(b\'a\');}'},
 {name:'string-is-not-character',source:'fn main(){let mut value=String::from("");value.push("a");}'},
 {name:'integer-is-not-character',source:'fn main(){let mut value=String::from("");value.push(97u32);}'},
 {name:'missing-operand',source:'fn main(){let mut value=String::from("");value.push();}'},
 {name:'extra-operand',source:'fn main(){let mut value=String::from("");value.push(\'a\',\'b\');}'},
 {name:'nonzero-byte-index-needs-full-boundary-interpretation',source:'fn main(){let mut value=String::from("abc");value.insert(1,\'x\');}'},
 {name:'zero-in-wrong-integer-domain',source:'fn main(){let mut value=String::from("abc");value.insert(0u8,\'x\');}'},
 {name:'invalid-utf8-boundary-is-not-zero-prefix',source:'fn main(){let mut value=String::from("é");value.insert(1,\'x\');}'},
];
