// Independent source/decision contracts, not execution observations.
const sourceVersionRange='>=1.90.0 <1.100.0';
export const rustOrdinaryMatchCompletionDecisions=[
 {operation:'readRustOrdinaryCompletionConstruction',arguments:['match',true],expected:'match'},
 {operation:'readRustOrdinaryCompletionConstruction',arguments:['match',false],expected:'match'},
 {operation:'readRustOrdinaryMatchCompletionPresence',arguments:[['identifier','identifier','punct','identifier','punct'],['match','value','{','return','}']],expected:1},
 {operation:'readRustOrdinaryMatchCompletionPresence',arguments:[['identifier','identifier','punct','string','punct'],['match','value','{','return','}']],expected:0},
 {operation:'readRustOrdinaryMatchCompletionPresence',arguments:[['identifier','identifier','punct','identifier','punct'],['match','value','{','r#return','}']],expected:0},
 {operation:'readRustOrdinaryMatchCompletionPresence',arguments:[['identifier','identifier','punct','punct','punct'],['match','value','{','?','}']],expected:1},
 {operation:'readRustOrdinaryMatchCompletionPresence',arguments:[['identifier','identifier','punct','identifier'],['match','value','{','return']],expected:-1},
 {operation:'readRustOrdinaryMatchCompletionPresence',arguments:[['identifier'],['r#match']],expected:0},
 {operation:'readRustOrdinaryMatchArmCountError',arguments:[1],expected:''},
 {operation:'readRustOrdinaryMatchArmCountError',arguments:[256],expected:''},
 {operation:'readRustOrdinaryMatchArmCountError',arguments:[257],expected:'the complete ordinary match arms exceed their source budget'},
 {operation:'readRustOrdinaryMatchArmCountError',arguments:[0],expected:'the complete ordinary match arms exceed their source budget'},
 {operation:'readRustOrdinaryCompletionAlternativeKind',arguments:['broken','value'],expected:'mixed'},
 {operation:'readRustOrdinaryCompletionAlternativeKind',arguments:['returned','continued'],expected:'completed loop alternatives'},
];
export const rustOrdinaryMatchCompletionOriginals=[
 {name:'an unbraced returned match arm shares the actual function result',sourceVersionRange,stdout:'7\n9\n',source:'fn choose(flag:bool)->u8{match flag{true=>return 7u8,false=>9u8}}fn main(){println!("{}",choose(true));println!("{}",choose(false));}'},
 {name:'a completed owning arm does not move a following alternative',sourceVersionRange,stdout:'text\ntext\n',source:'fn choose(flag:bool)->String{let text=String::from("text");match flag{true=>return text,false=>{}}text}fn main(){println!("{}",choose(true));println!("{}",choose(false));}'},
 {name:'actual standard constructor patterns transfer only their selected owning field',sourceVersionRange,stdout:'hello\nempty\n',source:'fn choose(value:Option<String>)->String{match value{Some(text)=>return text,None=>String::from("empty")}}fn main(){println!("{}",choose(Some(String::from("hello"))));println!("{}",choose(None));}'},
 {name:'a match break exits its nearest finite loop',sourceVersionRange,stdout:'2\n',source:'struct State{text:String,count:u8}fn run()->u8{let mut state=State{text:String::from("keep"),count:3u8};while state.count>0u8{match state.count==2u8{true=>break,false=>{state.count=state.count-1u8;}}}state.count}fn main(){println!("{}",run());}'},
 {name:'a match continue retains progress before the selected arm',sourceVersionRange,stdout:'last\n0\n',source:'struct State{text:String,count:u8}fn run()->u8{let mut state=State{text:String::from("keep"),count:3u8};while state.count>0u8{state.count=state.count-1u8;match state.count>0u8{true=>continue,false=>{println!("last");}}}state.count}fn main(){println!("{}",run());}'},
];
export const rustOrdinaryMatchCompletionRefusals=[
 {name:'completed arms retain exhaustive constructor coverage',sourceVersionRange,originalCompilerOutcome:'reject',source:'fn invalid(flag:bool)->u8{match flag{true=>return 7u8}}fn main(){}'},
 {name:'guarded completed arms retain the explicit fallthrough boundary',sourceVersionRange,originalCompilerOutcome:'accept',source:'fn choose(value:u8)->u8{match value{number if number>0u8=>return number,_=>0u8}}fn main(){}'},
 {name:'a continuing match arm still needs its exact loop progress',sourceVersionRange,originalCompilerOutcome:'accept',source:'struct State{text:String,count:u8}fn run()->u8{let mut state=State{text:String::from("keep"),count:3u8};while state.count>0u8{match state.count==2u8{true=>break,false=>{}}}state.count}fn main(){}'},
];
