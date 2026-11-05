// Source/token declarations only; never invoke the lexer or a translated product.
import{writeProjectData}from'./project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft to author declarations');
const cases=[
 ['raw function and parameter','fn r#match(r#type:i64)->i64{r#type} pub fn result()->i64{r#match(1)}','1'],
 ['raw Boolean keyword variable','fn read(r#true:bool)->bool{r#true} pub fn result()->i64{if read(false){1}else{2}}','2'],
 ['raw Boolean pattern binding','fn read(value:bool)->i64{match value{r#true=>if r#true{1}else{3}}} pub fn result()->i64{read(false)}','3'],
 ['raw datatype and field','struct r#type{r#match:i64} fn make()->r#type{r#type{r#match:4}} pub fn result()->i64{make().r#match}','4'],
 ['raw module','mod r#type{pub fn value()->i64{5}} pub fn result()->i64{r#type::value()}','5'],
 ['raw inherent method','struct Point{value:i64} impl Point{fn r#type(&self)->i64{self.value}} pub fn result()->i64{Point{value:6}.r#type()}','6'],
 ['edition 2024 reserved keyword','fn read(r#gen:i64)->i64{r#gen} pub fn result()->i64{read(7)}','7'],
 ['raw let keyword binding','pub fn result()->i64{let r#let=8;r#let}','8'],
 ['raw field pun','struct Point{r#type:i64} fn make(r#type:i64)->Point{Point{r#type}} pub fn result()->i64{make(9).r#type}','9'],
 ['raw pattern field pun','struct Point{r#type:i64} pub fn result()->i64{match (Point{r#type:10}){Point{r#type}=>r#type}}','10'],
 ['raw enum variant and field','enum Event{r#match{r#type:i64}} pub fn result()->i64{match (Event::r#match{r#type:11}){Event::r#match{r#type}=>r#type}}','11'],
 ['raw string stays a string','pub fn result()->i64{if r#"r#type"# == "r#type"{12}else{13}}','12'],
 ['raw non-keyword identity','struct r#Marker; fn read(_:Marker)->i64{13} pub fn result()->i64{read(r#Marker)}','13'],
 ['raw standard macro','pub fn result()->i64{r#panic!("raw macro failure")}',null,'raw macro failure'],
];
const rows=cases.map(([name,source,value,abort],index)=>({name,module:'raw_case_'+index,source,value,abort:abort??null}));
const lexicalCases=[
 {source:'r#match match r#new r#type',tokens:[['r#match','identifier','match',false],['match','identifier','match',true],['r#new','identifier','new',true],['r#type','identifier','type',false]]},
 {source:'r#true r#gen plain',tokens:[['r#true','identifier','true',false],['r#gen','identifier','gen',false],['plain','identifier','plain',true]]},
 {source:'r#panic!',tokens:[['r#panic!','macro','panic',false]]},
 {source:'r#"r#type"#',tokens:[['r#"r#type"#','string','r#type',false]]},
].map(row=>{
 let offset=0;
 return{source:row.source,tokens:row.tokens.map(([raw,kind,value,matchesValue])=>{
  const start=row.source.indexOf(raw,offset);if(start<0)throw new TypeError('literal token declaration is absent');
  const end=start+raw.length;offset=end;return{raw,kind,value,matchesValue,start,end};
 })};
});
const lexicalErrors=['_','crate','self','Self','super'].map(name=>({source:'r#'+name,kind:'syntax',reason:'invalid Rust raw identifier r#'+name,span:{start:0,end:name.length+2}}));
const refusals=lexicalErrors.map(row=>({name:'reserved raw '+row.source,source:'pub fn result()->i64{let '+row.source+'=1;1} fn main(){}',kind:row.kind,reason:row.reason}));
writeProjectData(new URL('../../parity/fixtures/rust-raw-identifiers.lino',import.meta.url),{
 schemaVersion:1,model:'Rust ASCII raw identifier and keyword meaning 1',cases:rows,refusals,lexicalCases,lexicalErrors,
 source:rows.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 basis:'https://doc.rust-lang.org/reference/identifiers.html#raw-identifiers; https://doc.rust-lang.org/reference/keywords.html#reserved-keywords',
 boundary:'Execute every source, value/abort, exact token/value/raw/span/cursor, forbidden-name, whole graph, text/binary codec, four-target and original/emitted Rust assertion in both runtimes. These are declarations, not observations. Unicode XID/NFC, older edition keyword profiles, generic/trait/raw lifetime and complete emitted-source reimport remain separate obligations.',
});
