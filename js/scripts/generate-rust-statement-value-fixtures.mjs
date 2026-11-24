// Source/value declarations only. No source parser, runtime, oracle or tests.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['discarded direct integer call','fn value()->i64{7}pub fn result()->i64{value();8}','8'],
 ['discarded forward integer call','pub fn result()->i64{later();9}fn later()->i64{7}','9'],
 ['discarded Boolean call','fn value()->bool{false}pub fn result()->i64{value();10}','10'],
 ['discarded owned string construction','fn ignore(){String::from("text");}pub fn result()->i64{ignore();11}','11'],
 ['discarded owned string call','fn value()->String{String::from("text")}pub fn result()->i64{value();12}','12'],
 ['discarded typed conditional value','pub fn result()->i64{if true{7i64}else{8i64};19}','19'],
 ['discarded explicit fixed literal','pub fn result()->i64{7i64;13}','13'],
 ['discarded typed local','pub fn result()->i64{let value=7i64;value;14}','14'],
 ['discarded Boolean comparison','pub fn result()->i64{1i64<2i64;15}','15'],
 ['Unit conditional statement without a semicolon','fn empty(){}fn invoke(flag:bool){if flag{empty();}else{empty();}empty();}pub fn result()->i64{invoke(true);invoke(false);16}','16'],
 ['Unit block statement without a semicolon','fn empty(){}fn invoke(){{empty();}empty();}pub fn result()->i64{invoke();17}','17'],
 ['Unit conditional with forward declarations','fn invoke(flag:bool){if flag{later();}else{later();}later();}fn later(){}pub fn result()->i64{invoke(true);18}','18'],
 ['discarded result precedes a guarded return','fn value()->i64{7}fn choose(flag:bool)->i64{value();if flag{return 8;}9}pub fn result()->i64{choose(true)*10+choose(false)}','89'],
 ['discarded result preserves source name hygiene','fn value()->i64{7}pub fn result()->i64{let ml_tuple_value0=19;value();ml_tuple_value0}','19'],
];
const cases=inputs.map(([name,source,value],index)=>({name,module:'statement_value_'+index,source,value,abort:null}));
writeProjectData(new URL('../../parity/fixtures/rust-statement-values.lino',import.meta.url),{
 schemaVersion:1,model:'ordinary Rust direct primitive discarded values and constrained Unit block statements 1',sourceLanguage:'Rust',cases,
 source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n'),refusals:[],
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://doc.rust-lang.org/reference/statements.html#expression-statements ; https://doc.rust-lang.org/reference/expressions/block-expr.html',
 boundary:'Future original Rust/JavaScript/native values, complete source ASTs, definition/program codecs and four emissions. No observations. Semicolon discards currently use known direct primitive source types; missing-semicolon blocks carry an actual Unit constraint and direct-source facts. General discarded nominal/reference values, default numeric inference/overflow, mixed value joins, custom drop/borrow/mutable contexts and every earlier obligation remain.',
});
