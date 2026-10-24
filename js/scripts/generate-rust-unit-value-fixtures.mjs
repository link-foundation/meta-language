// Declare original Rust source controls only. Never compile or execute here.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['implicit unit function and value binding','fn empty(){} pub fn result()->i64{let _value=empty();7}','7'],
 ['explicit unit empty function','fn empty()->(){} pub fn result()->i64{let _value=empty();8}','8'],
 ['explicit unit expression','fn empty(){()} pub fn result()->i64{let _value=empty();9}','9'],
 ['terminal unit return','fn empty(){return;} pub fn result()->i64{let _value=empty();10}','10'],
 ['terminal unit return without semicolon','fn empty(){return} pub fn result()->i64{let _value=empty();11}','11'],
 ['empty nested block is unit','pub fn result()->i64{let _value={};12}','12'],
 ['empty conditional branches are unit','pub fn result()->i64{let _value=if true{}else{};13}','13'],
 ['unit argument and return forwarding','fn empty(){} fn forward()->(){empty()} fn accept(_value:())->i64{14} pub fn result()->i64{accept(forward())}','14'],
 ['unit after local binding','fn empty(){let _value=7i64;} pub fn result()->i64{let _value=empty();15}','15'],
];
inputs.push(
 ['ordered repeated Unit call statements','fn first(){} fn second(){} fn invoke(){first();second();first();} pub fn result()->i64{invoke();16}','16'],
 ['forward Unit declaration calls','fn invoke(){later();} fn later(){} pub fn result()->i64{invoke();17}','17'],
 ['empty statements have no effects','fn empty(){;;;();;;return;;;} pub fn result()->i64{empty();18}','18'],
 ['Unit call private-name hygiene','fn empty(){} pub fn result()->i64{let ml_tuple_value0=19;empty();ml_tuple_value0}','19'],
);
inputs.push(['Unit call statements in a main-shaped function','fn empty(){} fn main(){;;;empty();();} pub fn result()->i64{main();20}','20']);
const correctedAssertions=[
 {source:'fn noop(){}fn main(){}',previousAssertion:'parseRust throws function without a result: noop returns (); only value-returning functions are portable',previousSpan:{start:0,end:9},calls:[]},
 {source:'fn noop()->(){()}fn main(){noop();}',previousAssertion:'parseRust throws main statement: main may only print, bind and assert',calls:['noop']},
 {source:'complete recorded emitted Rust',previousAssertion:'parseRust throws function without a result: ml_main',currentAssertion:'parseRust reaches the unchanged std::env::args wrapper obligation'},
];
const cases=inputs.map(([name,source,value],index)=>({name,module:'unit_value_case_'+index,source,value,abort:null}));
const refusals=[
 {name:'unreachable Unit calls cannot move before abort',source:'fn empty(){} fn failure(){panic!("stop");empty();} fn main(){}',kind:'unsupported',reason:'unreachable continuation: statements after an abort are not removed by this model'},
 {name:'nested return retains nonlocal completion obligation',source:'fn empty(flag:bool){let _value=if flag{return;}else{()};panic!("later");} fn main(){}',kind:'unsupported',reason:'early return: write the result as the tail expression of the block'},
];
writeProjectData(new URL('../../parity/fixtures/rust-unit-values.lino',import.meta.url),{
 schemaVersion:1,correctedAssertions,predecessorRevision:'062d78c3',model:'Rust implicit Unit and empty block source values 1',sourceLanguage:'Rust',cases,refusals,
 mainSources:['fn empty(){}fn main(){empty();}','fn empty(){}fn main(){empty()}','fn main(){;;;();}'],
 source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n'),
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://doc.rust-lang.org/reference/items/functions.html ; https://doc.rust-lang.org/reference/expressions/block-expr.html ; https://doc.rust-lang.org/reference/expressions/return-expr.html',
 boundary:'Future original Rust and emitted JavaScript/native values, actual source parse parity and complete graphs/codecs/four-target emissions. No observations. Arbitrary nonlocal returns, discarded expression effects/destructors, source profile composition and all earlier requirements remain.',
});
