// Complete original sources and independent contracts only; never parse them.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,declarations,body,value='7',ret='i64',abort=null){cases.push({name,module:'string_pattern_case_'+cases.length,source:declarations+'pub fn result()->'+ret+'{'+body+'}',value,abort});}
const selected='pub fn selected(text:&str)->i64{match text{"first"=>3,"second"=>5,_=>7}}';
for(const [input,value]of [['first','3'],['second','5'],['other','7']])add('ordered string literal arm '+input,selected,'selected('+JSON.stringify(input)+')',value);
add('string alternatives share one ordered body','pub fn selected(text:&str)->i64{match text{"first"|"second"=>7,_=>0}}','selected("second")');
add('literal guards do not suppress later unguarded arms','pub fn selected(text:&str,flag:bool)->i64{match text{"first" if flag=>3,"first"=>7,_=>0}}','selected("first",false)');
add('selected literal guard chooses its own body','pub fn selected(text:&str,flag:bool)->i64{match text{"first" if flag=>7,"first"=>3,_=>0}}','selected("first",true)');
add('unmatched string alternatives never evaluate their guards','fn fail()->bool{panic!("unmatched string guard")}pub fn selected(text:&str)->i64{match text{"first" if fail()=>3,_=>7}}','selected("other")');
add('selected string guard preserves its original abort','fn fail()->bool{panic!("selected string guard")}pub fn selected(text:&str)->i64{match text{"first" if fail()=>3,_=>7}}','selected("first")',null,'i64','selected string guard');
add('an earlier selected arm keeps later string guards lazy','fn fail()->bool{panic!("later string guard")}pub fn selected(text:&str)->i64{match text{"first"=>7,"first" if fail()=>3,_=>0}}','selected("first")');
add('the scrutinee retains its actual failure before arm selection','fn fail()->&\'static str{panic!("string scrutinee")}','match fail(){"first"=>3,_=>7}',null,'i64','string scrutinee');
add('empty and Unicode strings retain exact scalar contents','pub fn selected(text:&str)->i64{match text{""=>1,"é"=>3,"é"=>5,"😀"=>7,_=>0}}','selected("😀")');
add('decomposed and composed text remain different literals','pub fn selected(text:&str)->i64{match text{"é"=>3,"é"=>7,_=>0}}','selected("é")');
add('raw string patterns keep embedded quotes','pub fn selected(text:&str)->i64{match text{r##"quoted " text"##=>7,_=>0}}','selected(r##"quoted " text"##)');
add('literal matching retains a complete borrowed reference layer','pub fn selected(text:&&str)->i64{match text{"first"=>7,_=>0}}','selected(&"first")');
add('string aliases keep actual input ownership rather than literal provenance','pub fn selected(text:&str)->usize{match text{label@"first"=>label.len(),_=>0}}','selected("first")','5','usize');
add('string alias guards retain actual arm scope','pub fn selected(text:&str)->i64{match text{label@"first" if label.len()==5=>7,_=>0}}','selected("first")');
add('conditional string patterns reuse literal semantics','pub fn selected(text:&str)->i64{if let "first"=text{7}else{0}}','selected("first")');
add('conditional string alternatives retain unmatched fallback','pub fn selected(text:&str)->i64{if let "first"|"second"=text{3}else{7}}','selected("other")');
add('nested constructor literals retain their actual string field owner','enum Text{Value(&\'static str),None}','match Text::Value("first"){Text::Value("first")=>7,_=>0}');
add('nested constructor aliases keep their actual field ownership','enum Text{Value(&\'static str),None}','match Text::Value("first"){Text::Value(label@"first")=>label.len(),_=>0}','5','usize');
const error='Rust string pattern: a literal pattern requires its proven immutable borrowed str source owner';
const refusals=[
 {name:'owned String is not a borrowed str pattern scrutinee',source:'pub fn result()->i64{match String::from("first"){"first"=>7,_=>0}}fn main(){}',kind:'unsupported',reason:error},
 {name:'a scalar cannot acquire string pattern ownership',source:'pub fn result()->i64{match 3i64{"first"=>7,_=>0}}fn main(){}',kind:'unsupported',reason:error}
];
const base={kind:'string',model:'string slice',proven:true,layers:1,references:true,ownerKind:'primitive',ownerPath:'str',argumentsCount:0,collecting:false};
const premises=[{...base,expected:''}];
for(const change of [{kind:'int'},{model:'owned string'},{proven:false},{layers:0},{layers:1.5},{references:false},{ownerKind:'named'},{ownerPath:'other::str'},{argumentsCount:1}])premises.push({...base,...change,expected:error.slice('Rust string pattern: '.length)});
premises.push({...base,layers:3,expected:''},{...base,proven:false,collecting:true,expected:''},{...base,kind:'int',collecting:true,expected:error.slice('Rust string pattern: '.length)});
writeProjectData(new URL('../../parity/fixtures/rust-string-patterns.lino',import.meta.url),{schemaVersion:1,model:'ordinary Rust borrowed string literal patterns 1',retainSourceTypeOwners:true,cases,refusals,premises,tokens:[{token:'string',expected:true},{token:'byteString',expected:false},{token:'number',expected:false},{token:'identifier',expected:false}],source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64'},references:['https://doc.rust-lang.org/reference/patterns.html#literal-patterns','https://doc.rust-lang.org/reference/expressions/match-expr.html#match-guards'],assertions:['literalArmsUseCommonEqualityAndExactStringContents','actualBorrowedStringSourceOwnerRequired','orderedOrArmsGuardsAndOriginalAbruptCompletions','aliasesPreserveActualScrutineeAndNestedFieldOwners','bothGraphCodecsAndFourTargetEmissions','independentOriginalAndGeneratedNativePrograms'],boundary:'Unexecuted original controls. Shared source policy selects literal token and source-owner admission; the existing common match algebra handles semantics. Byte/C/character patterns, full match ergonomics, borrowed output interfaces, mutable/interior state and full Rust translation remain requirements. No construction receipt establishes execution evidence.'});
