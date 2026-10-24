// Author source and expectation declarations only; no product executes.
import{writeProjectData}from'./project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['named fields','struct Point{x:i64,y:i64} pub fn result()->i64{let Point{x,y}=Point{x:4,y:2};x*10+y}','42'],
 ['reordered named fields','struct Point{x:i64,y:i64} pub fn result()->i64{let Point{y:second,x:first}=Point{y:3,x:5};first*10+second}','53'],
 ['partial named fields','struct Point{x:i64,y:i64} pub fn result()->i64{let Point{y,..}=Point{x:1,y:7};y}','7'],
 ['discard all fields','struct Point{x:i64,y:i64} pub fn result()->i64{let Point{..}=Point{x:1,y:2};8}','8'],
 ['tuple struct fields','struct Pair(i64,i64); pub fn result()->i64{let Pair(left,right)=Pair(6,4);left*10+right}','64'],
 ['tuple brace fields','struct Pair(i64,i64); pub fn result()->i64{let Pair{1:right,0:left}=Pair(7,5);left*10+right}','75'],
 ['unit struct pattern','struct Marker; pub fn result()->i64{let Marker=Marker;9}','9'],
 ['empty named pattern','struct Marker{} pub fn result()->i64{let Marker{}=Marker{};10}','10'],
 ['nested records','struct Inner{x:i64} struct Outer{left:Inner,right:Inner} pub fn result()->i64{let Outer{left:Inner{x:a},right:Inner{x:b}}=Outer{left:Inner{x:8},right:Inner{x:6}};a*10+b}','86'],
 ['single enum variant','enum Event{Value{x:i64,y:i64}} pub fn result()->i64{let Event::Value{y,x}=Event::Value{x:9,y:7};x*10+y}','97'],
 ['single tuple enum variant','enum Event{Value(i64,i64)} pub fn result()->i64{let Event::Value(a,b)=Event::Value(1,8);a*10+b}','18'],
 ['qualified owner and annotation','mod owner{pub struct Point{pub x:u8,pub y:u16}} use self::owner::Point as Pair; pub fn result()->i64{let Pair{x,y}:Pair=Pair{x:2,y:9};x as i64*10+y as i64}','29'],
 ['binding shadows previous scope','struct Point{x:i64} pub fn result()->i64{let x=7;let Point{x}=Point{x:x+1};x}','8'],
 ['hygienic internal bundle','struct Point{x:i64,y:i64} pub fn result()->i64{let ml_tuple_value0=3;let ml_tuple_value1=4;let Point{x,y}=Point{x:ml_tuple_value0,y:ml_tuple_value1};x*10+y}','34'],
 ['initializer failure still occurs','struct Point{x:i64} fn value()->Point{panic!("initializer first")} pub fn result()->i64{let Point{..}=value();panic!("tail later")}',null,'initializer first'],
 ['owned string field','struct Text{value:String} pub fn result()->i64{let Text{value}=Text{value:String::from("abc")};if value==String::from("abc"){35}else{0}}','35'],
 ['borrowed pattern field','struct Point{x:i64} fn read(p:&Point)->i64{let Point{x}=p;*x} pub fn result()->i64{read(&Point{x:36})}','36'],
 ['nested field keeps nominal owner','struct Inner{x:i64} struct Outer{inner:Inner} pub fn result()->i64{let Outer{inner}=Outer{inner:Inner{x:37}};inner.x}','37'],
 ['source order across bindings','struct Point{x:i64} pub fn result()->i64{let Point{x:first}=Point{x:3};let Point{x:second}=Point{x:first+5};first*10+second}','38'],
 ['forward constructor declaration','pub fn result()->i64{let Pair(a,b)=Pair(3,9);a*10+b} struct Pair(i64,i64);','39'],
];
const cases=inputs.map(([name,source,value,abort],index)=>({name,module:'binding_case_'+index,source,value,abort:abort??null}));
const reason='refutable product binding: only immutable names, ignored fields and anonymous tuple products are admitted';
const refusals=[
 {name:'multiple enum variants remain refutable',source:'enum Event{Value(i64),Empty} fn read(e:Event)->i64{let Event::Value(x)=e;x} fn main(){}',kind:'unsupported',reason},
 {name:'literal field remains refutable',source:'struct Point{x:i64} fn read(p:Point)->i64{let Point{x:1}=p;0} fn main(){}',kind:'unsupported',reason},
 {name:'duplicate local name',source:'struct Point{x:i64,y:i64} fn read(p:Point)->i64{let Point{x:a,y:a}=p;a} fn main(){}',kind:'unsupported',reason:'duplicate tuple binding: each name occurs only once in a destructuring pattern'},
 {name:'whole record alias needs move proof',source:'struct Point{x:i64} fn read(p:Point)->i64{let Point{x:a@x}=p;a} fn main(){}',kind:'unsupported',reason:'Rust nominal binding alias: whole-value aliases require their complete move and borrow interpretation'},
];
writeProjectData(new URL('../../parity/fixtures/rust-record-bindings.lino',import.meta.url),{
 schemaVersion:1,model:'owned nominal Rust record field order 1',cases,refusals,
 source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://doc.rust-lang.org/reference/statements.html#let-statements ; https://doc.rust-lang.org/reference/patterns.html#refutability ; single-constructor recursive patterns are irrefutable; the source value is evaluated once before binding names.',
 boundary:'Future paired complete program/definition graphs, both codecs, four emissions, independent original/emitted Rust values and exact refusals. No observations. Tuple fields nested inside nominal patterns, ref/mut/@ patterns and broader move/borrow/drop semantics require further source work.',
});
