// Future declarations only. No parser, fixture, oracle or product executes.
import{writeProjectData}from'./project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['matched enum payload','enum Value{Some(i64),None} pub fn result()->i64{if let Value::Some(x)=Value::Some(41){x}else{0}}','41'],
 ['unmatched enum payload','enum Value{Some(i64),None} pub fn result()->i64{if let Value::Some(x)=Value::None{x}else{42}}','42'],
 ['outer name remains in else scope','enum Value{Some(i64),None} pub fn result()->i64{let x=43;if let Value::Some(x)=Value::None{x}else{x}}','43'],
 ['inner name has matched scope','enum Value{Some(i64),None} pub fn result()->i64{let x=3;if let Value::Some(x)=Value::Some(44){x}else{x}}','44'],
 ['or-pattern success','enum Value{First(i64),Second(i64),Empty} pub fn result()->i64{if let Value::First(x)|Value::Second(x)=Value::Second(45){x}else{0}}','45'],
 ['else if let chain','enum Value{Some(i64),None} pub fn result()->i64{if let Value::Some(x)=Value::None{x}else if let Value::Some(y)=Value::Some(46){y}else{0}}','46'],
 ['tuple literal pattern','pub fn result()->i64{if let (x,true)=(47,true){x}else{0}}','47'],
 ['tuple literal mismatch','pub fn result()->i64{if let (x,true)=(1,false){x}else{48}}','48'],
 ['nominal record fields','struct Point{x:i64,y:bool} pub fn result()->i64{if let Point{x,y:true}=(Point{x:49,y:true}){x}else{0}}','49'],
 ['tuple rest pattern','pub fn result()->i64{if let (first,..,last)=(5,3,0){first*10+last}else{0}}','50'],
 ['conditional owned result','struct Point{x:i64} enum Value{Some(i64),None} fn make()->Point{if let Value::Some(x)=Value::Some(51){Point{x}}else{Point{x:0}}} pub fn result()->i64{make().x}','51'],
 ['borrowed payload','enum Value{Some(i64),None} fn read(p:&Value)->i64{if let Value::Some(x)=p{*x}else{0}} pub fn result()->i64{read(&Value::Some(52))}','52'],
 ['selected arm failure','enum Value{Some(i64),None} pub fn result()->i64{if let Value::Some(x)=Value::Some(1){panic!("matched arm")}else{panic!("fallback arm")}}',null,'matched arm'],
 ['scrutinee failure before arms','enum Value{Some(i64),None} fn source()->Value{panic!("scrutinee first")} pub fn result()->i64{if let Value::Some(x)=source(){panic!("matched later")}else{panic!("fallback later")}}',null,'scrutinee first'],
 ['fully bound pattern alias','enum Value{Some(i64),None} pub fn result()->i64{if let whole@Value::Some(x)=Value::Some(53){match whole{Value::Some(y)=>y,Value::None=>0}}else{0}}','53'],
];
inputs.push(['exact former conditional chain input','enum Value{Some(i64),None} fn f(p:Value)->i64{if let Value::Some(x)=p&&x>0{x}else{0}} fn main(){} pub fn result()->i64{f(Value::Some(3))+f(Value::None)}','3']);
const cases=inputs.map(([name,source,value,abort],index)=>({name,module:'pattern_condition_case_'+index,source,value,abort:abort??null}));
const refusals=[
 {name:'conditional pattern needs both value branches',source:'enum Value{Some(i64),None} fn f(p:Value)->i64{if let Value::Some(x)=p{x}} fn main(){}',kind:'unsupported',reason:'if without else: the value of an if must be defined on both branches'},
 {name:'or-pattern bindings have equal ownership scope',source:'enum Value{First(i64),Second(i64)} fn f(p:Value)->i64{if let Value::First(x)|Value::Second(y)=p{0}else{1}} fn main(){}',kind:'unsupported',reason:'or-pattern ownership: every alternative must bind the same names with the same source ownership'},
];
writeProjectData(new URL('../../parity/fixtures/rust-pattern-conditions.lino',import.meta.url),{
 schemaVersion:1,model:'owned nominal Rust record field order 1',cases,refusals,
 source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://doc.rust-lang.org/reference/expressions/if-expr.html#if-let-patterns ; conditional bindings belong to matched branches; original outer names remain available in else.',
 boundary:'Future paired full program/definition graphs, both codecs, four emissions, original/emitted Rust values and exact refusals. No observations. Conditional chains now have their distinct shared continuation program and immutable predecessor controls; missing-else unit/effect statements, let-else, loops and full move/borrow/drop retain their remaining requirements.',
});
