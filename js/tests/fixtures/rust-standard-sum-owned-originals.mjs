// Independent complete source contracts for ordinary ownership transport.
// Authored only: no original, translation or control has been executed.
const sourceVersionRange='>=1.90.0 <1.100.0';
// Literal compiler expectations, not observations. Both selected compilers
// must check every classified original without executing a refused program.
const invalidOwnershipCompilerExpectation={model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',outcome:'reject',category:'invalid Rust ownership'};
const validSourceCompilerExpectation={model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',outcome:'accept',category:'valid Rust source'};
export const rustStandardSumOwnedOriginals=[
 {name:'owned String constructor moves its actual payload through a consuming parameter',sourceVersionRange,singleCallOwners:[],stdout:'text\n',source:`
fn reveal(value:Option<String>)->String{match value{Some(text)=>text,None=>String::from("none")}}
fn main(){let text=String::from("text");let value:Option<String>=Some(text);println!("{}",reveal(value));}
`},
 {name:'nullary Option String remains a non-Copy source type',sourceVersionRange,singleCallOwners:[],stdout:'none\n',source:`
fn reveal(value:Option<String>)->String{match value{Some(text)=>text,None=>String::from("none")}}
fn main(){let value:Option<String>=None;println!("{}",reveal(value));}
`},
 {name:'actual function result retains fresh owned sum production',sourceVersionRange,singleCallOwners:[],stdout:'made\n',source:`
fn make()->Option<String>{Some(String::from("made"))}
fn reveal(value:Option<String>)->String{match value{Some(text)=>text,None=>String::from("none")}}
fn main(){let value=make();println!("{}",reveal(value));}
`},
 {name:'both Result String branches retain their independent actual owners',sourceVersionRange,singleCallOwners:[],stdout:'good\nbad\n',source:`
fn reveal(value:Result<String,String>)->String{match value{Ok(text)=>text,Err(error)=>error}}
fn main(){println!("{}",reveal(Ok(String::from("good"))));println!("{}",reveal(Err(String::from("bad"))));}
`},
 {name:'owned vector payload moves into its case binding before a shared length read',sourceVersionRange,singleCallOwners:[],stdout:'2\n0\n',source:`
fn count(value:Option<Vec<u8>>)->usize{match value{Some(values)=>values.len(),None=>0usize}}
fn main(){let values=vec![7u8,9u8];println!("{}",count(Some(values)));println!("{}",count(None));}
`},
 {name:'nested owned sum patterns preserve every independent constructor',sourceVersionRange,singleCallOwners:[],stdout:'good\nbad\nnone\n',source:`
fn reveal(value:Option<Result<String,String>>)->String{match value{Some(Ok(text))=>text,Some(Err(error))=>error,None=>String::from("none")}}
fn main(){println!("{}",reveal(Some(Ok(String::from("good")))));println!("{}",reveal(Some(Err(String::from("bad")))));println!("{}",reveal(None));}
`},
 {name:'borrowed pattern projection retains the actual root loan before a later consuming call',sourceVersionRange,singleCallOwners:[],stdout:'5\ncafé\n',source:`
fn count(value:&Option<String>)->usize{match value{Some(text)=>text.len(),None=>0usize}}
fn reveal(value:Option<String>)->String{match value{Some(text)=>text,None=>String::from("none")}}
fn main(){let value:Option<String>=Some(String::from("café"));println!("{}",count(&value));println!("{}",reveal(value));}
`},
 {name:'nonbinding constructor inspection does not move an owned payload',sourceVersionRange,singleCallOwners:[],stdout:'true\ntext\n',source:`
fn reveal(value:Option<String>)->String{match value{Some(text)=>text,None=>String::from("none")}}
fn main(){let value:Option<String>=Some(String::from("text"));let present=match value{Some(_)=>true,None=>false};println!("{}",present);println!("{}",reveal(value));}
`},
 {name:'guarded shared projection does not acquire an owned field',sourceVersionRange,singleCallOwners:[],stdout:'true\nfalse\n',source:`
fn present(value:&Option<String>)->bool{match value{Some(text) if text.len()>0usize=>true,_=>false}}
fn main(){let value:Option<String>=Some(String::from("text"));let absent:Option<String>=None;println!("{}",present(&value));println!("{}",present(&absent));}
`},
];
export const rustStandardSumOwnedRefusals=[
 {name:'a constructor consumes the original owned String binding',sourceVersionRange,originalCompilerExpectation:invalidOwnershipCompilerExpectation,source:'fn main(){let text=String::from("text");let value:Option<String>=Some(text);println!("{}",text);}'},
 {name:'a consuming sum parameter cannot receive the same moved root twice',sourceVersionRange,originalCompilerExpectation:invalidOwnershipCompilerExpectation,source:'fn consume(value:Option<String>)->usize{match value{Some(text)=>text.len(),None=>0usize}}fn main(){let value:Option<String>=Some(String::from("text"));println!("{}",consume(value));println!("{}",consume(value));}'},
 {name:'a consuming field pattern invalidates its original root',sourceVersionRange,originalCompilerExpectation:invalidOwnershipCompilerExpectation,source:'fn consume(value:Option<String>)->usize{match value{Some(text)=>text.len(),None=>0usize}}fn main(){let value:Option<String>=Some(String::from("text"));let count=match value{Some(text)=>text.len(),None=>0usize};println!("{}",consume(value));}'},
 {name:'an explicit reference pattern cannot move String out of a shared sum',sourceVersionRange,originalCompilerExpectation:invalidOwnershipCompilerExpectation,source:'fn bad(value:&Option<String>)->String{match value{&Some(text)=>text,&None=>String::from("none")}}fn main(){}'},
 {name:'a shared root loan cannot survive moving its owned sum',sourceVersionRange,originalCompilerExpectation:invalidOwnershipCompilerExpectation,source:'fn consume(value:Option<String>)->usize{match value{Some(text)=>text.len(),None=>0usize}}fn inspect(value:&Option<String>)->bool{match value{Some(_)=>true,None=>false}}fn main(){let value:Option<String>=Some(String::from("text"));let borrowed=&value;println!("{}",consume(value));println!("{}",inspect(borrowed));}'},
];
// Preserve the exact former source row, coordinate, compiler contract and
// previous interpreter oracle when the new deferred-move consumer admits it.
export const rustStandardSumOwnedPreviousBoundaries=[{
 previousCollection:'rustStandardSumOwnedRefusals',previousIndex:5,
 previousDiagnosticKinds:['unsupported','type'],
 previousTranslationExpectation:'refuse immediate owned pattern guard',
 row:{name:'guarded by-value field moves retain their fallthrough obligation',sourceVersionRange,originalCompilerExpectation:validSourceCompilerExpectation,source:'fn bad(value:Option<String>)->usize{match value{Some(text) if text.len()>0usize=>text.len(),_=>0usize}}fn main(){}'},
}];
rustStandardSumOwnedOriginals.push({...rustStandardSumOwnedPreviousBoundaries[0].row,
 stdout:'',singleCallOwners:[],expectationHistory:[{
  previousCollection:rustStandardSumOwnedPreviousBoundaries[0].previousCollection,
  previousIndex:rustStandardSumOwnedPreviousBoundaries[0].previousIndex,
  translationExpectation:rustStandardSumOwnedPreviousBoundaries[0].previousTranslationExpectation,
  reason:'owned sum guards need their delayed payload-move and fallthrough model',
 }]});
