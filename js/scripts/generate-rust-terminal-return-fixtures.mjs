// Read and append declarations only; no source parser or oracle is invoked.
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const legacy=readProjectData(new URL('../../parity/fixtures/rust-completions.lino',import.meta.url)).legacy[0];
const inputs=[
 ['terminal integer return','pub fn result()->i64{return 7;}','7'],
 ['terminal value without semicolon','pub fn result()->i64{return 8}','8'],
 ['inert trailing semicolons','pub fn result()->i64{let value=9;return value;;;}', '9'],
 ['returned conditional expression','pub fn result()->i64{return if true{10}else{11};}','10'],
 ['nested block remains a value','pub fn result()->i64{let value=2;return {let value=12;value};}','12'],
 ['forward returned call','pub fn result()->i64{return later();}fn later()->i64{return 13;}','13'],
 ['returned named struct','struct Record{value:i64}fn create()->Record{return Record{value:14};}pub fn result()->i64{create().value}','14'],
 ['returned tuple value','fn pair()->(i64,i64){return (2,3);}pub fn result()->i64{let(first,second)=pair();first*10+second}','23'],
 ['returned string retains ownership','fn text()->String{return String::from("fifteen");}pub fn result()->usize{text().len()}','7'],
 ['punctuation inside strings cannot end a return','fn text()->String{return String::from("};");}pub fn result()->usize{text().len()}','2'],
 ['returned parenthesized value','pub fn result()->i64{return (16);}','16'],
 ['original default refusal becomes an actual value',legacy.source+' pub fn result()->u64{f(17)}','17'],
 ['terminal expression spans growing token windows','pub fn result()->i64{return '+Array.from({length:200},()=> '1').join('+')+';}','200'],
];
const cases=inputs.map(([name,source,value],index)=>({name,module:'terminal_return_case_'+index,source,value,abort:null}));
writeProjectData(new URL('../../parity/fixtures/rust-terminal-returns.lino',import.meta.url),{
 schemaVersion:1,model:'Rust terminal function return value source projection 1',sourceLanguage:'Rust',cases,
 source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n'),
 originalRefusal:legacy,previousAssertion:'default parseRust/checkProgram throws the original kind/reason/span',predecessorRevision:'ed58b081',
 refusals:[
  {name:'earlier value return keeps its continuation obligation',source:'fn result()->i64{return 1;let later=2;later}fn main(){}',kind:'unsupported',reason:'early return: write the result as the tail expression of the block'},
  {name:'nested return cannot become an initializer value',source:'fn result()->i64{let value={return 1;};value}fn main(){}',kind:'unsupported',reason:'early return: write the result as the tail expression of the block'},
 ],
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://doc.rust-lang.org/reference/expressions/return-expr.html ; https://doc.rust-lang.org/reference/expressions/block-expr.html',
 boundary:'Future original Rust/emitted JavaScript/native values, exact source parse parity, complete typed definition/program graphs and four emissions. No observations. Only actual terminal function-boundary returns are projected to values. Earlier/nested returns, unreachable statement preservation, general completions/borrows/drops and every preceding requirement remain.',
});
