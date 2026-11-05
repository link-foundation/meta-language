// Complete source/value/history declarations only. No parser, calls or tests.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['nested partial return joins its outer continuation','fn choose(first:bool,second:bool)->i64{if first{if second{return 1;}();}else{();}3}pub fn result()->i64{choose(true,true)*100+choose(true,false)*10+choose(false,true)}','133'],
 ['continuing first alternative cannot lose its continuation','fn empty(){}fn choose(first:bool,second:bool)->i64{if first{empty();}else if second{return 2;}3}pub fn result()->i64{choose(true,true)*100+choose(false,true)*10+choose(false,false)}','323'],
 ['branch-local shadows cannot capture the shared tail','fn choose(first:bool,second:bool)->i64{let value=20;if first{let value=7;if second{return value;}();}else{let value=9;();}value}pub fn result()->i64{choose(true,true)*100+choose(true,false)*10+choose(false,false)}','920'],
 ['multiple mixed joins preserve source ordering','fn choose(first:bool,second:bool)->i64{if first{if second{return 1;}();}else{();}if second{return 2;}3}pub fn result()->i64{choose(true,true)*100+choose(false,true)*10+choose(true,false)}','123'],
 ['owned strings return through the complete carrier','fn choose(first:bool,second:bool)->String{if first{if second{return String::from("s");}();}else{();}String::from("long")}pub fn result()->usize{choose(true,true).len()*100+choose(true,false).len()*10+choose(false,false).len()}','144'],
 ['nominal payload remains its original source type','struct Record{value:i64}fn choose(first:bool,second:bool)->Record{if first{if second{return Record{value:1};}();}else{();}Record{value:3}}pub fn result()->i64{choose(true,true).value*10+choose(true,false).value}','13'],
 ['continuing else alternative retains the tail','fn choose(flag:bool)->i64{if flag{return 1;}else{();}2}pub fn result()->i64{choose(true)*10+choose(false)}','12'],
 ['unselected nested abort stays lazy','fn choose(first:bool,second:bool)->i64{if first{if second{panic!("nested carrier");}();}else{return 2;}3}pub fn result()->i64{choose(true,false)*10+choose(false,false)}','32'],
 ['else-if chain shares exactly one outer continuation','fn choose(first:bool,second:bool,third:bool)->i64{if first{();}else if second{return 2;}else if third{return 3;}4}pub fn result()->i64{choose(true,true,true)*1000+choose(false,true,true)*100+choose(false,false,true)*10+choose(false,false,false)}','4234'],
 ['completed conditional with semicolon has no Unit fallthrough','fn choose(flag:bool)->i64{if flag{return 7;}else{return 9;};}pub fn result()->i64{choose(true)*10+choose(false)}','79'],
 ['false result is distinct from statement continuation','fn choose(first:bool,second:bool)->bool{if first{if second{return false;}();}true}pub fn result()->i64{let first=if choose(true,true){1}else{0};let second=if choose(true,false){2}else{0};first+second}','2'],
 ['zero result is distinct from statement continuation','fn choose(first:bool,second:bool)->i64{if first{if second{return 0;}();}7}pub fn result()->i64{choose(true,true)*10+choose(true,false)}','7'],
 ['source temporary names remain independent','fn choose(first:bool,second:bool)->i64{let ml_tuple_value0=17;if first{let ml_tuple_value1=23;if second{return ml_tuple_value1;}();}ml_tuple_value0}pub fn result()->i64{choose(true,true)*100+choose(true,false)}','2317'],
 ['actual nested module owns its carrier declaration','mod child{pub fn choose(first:bool,second:bool)->i64{if first{if second{return 1;}();}3}}pub fn result()->i64{child::choose(true,true)*10+child::choose(true,false)}','13'],
 ['branch binding scope remains outside the continuation matcher','fn choose(first:bool,second:bool)->i64{let value=5;if first{let value=9;if second{return value;}();}else{let value=7;();}let next=value+1;next}pub fn result()->i64{choose(true,true)*10+choose(true,false)}','96'],
];
const cases=inputs.map(([name,source,value],index)=>({name,module:'completion_carrier_'+index,source,value,abort:null}));
cases.push({name:'selected nested abort keeps original payload',module:'completion_carrier_abort',source:'fn choose(first:bool,second:bool)->i64{if first{if second{panic!("nested carrier");}();}else{return 2;}3}pub fn result()->i64{choose(true,true)}',value:null,abort:'nested carrier'});
const originalRefusals=readProjectData(new URL('../../parity/fixtures/rust-ordinary-returns.lino',import.meta.url)).originalRefusals;
const files=['js/src/translation/rust-ordinary-completions.js','rust/src/translation/rust/ordinary_completions.rs','js/scripts/generate-rust-ordinary-return-fixtures.mjs'].map(path=>{const source=execFileSync('git',['show','75496b5e:'+path],{encoding:'utf8'});return {path,source,sha256:createHash('sha256').update(source).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/rust-completion-carriers.lino',import.meta.url),{
 schemaVersion:1,model:'ordinary Rust explicit function result and statement continuation joins 1',sourceLanguage:'Rust',cases,
 source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n'),refusals:[],
 assertionCorrections:{predecessorRevision:'75496b5e',originalRefusals,files,currentAssertion:'Both exact former refusal sources are admitted, retain complete original/translated/native traces and all program/definition codecs/emissions; every original input and assertion remains in the complete prior sources.'},
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://doc.rust-lang.org/reference/expressions/return-expr.html ; https://doc.rust-lang.org/reference/expressions/if-expr.html ; https://doc.rust-lang.org/reference/expressions/block-expr.html',
 boundary:'Future independent original Rust/translated JavaScript/native values and aborts, source AST equality, complete definition/program archives and four emissions. No observations. Statement fallthrough currently needs direct Unit source evidence. General discarded value joins, nested expression returns, loops, mutable/borrow/drop contexts, selected-profile composition and all earlier obligations remain.',
});
