// Author declarations only; no original or translated code executes.
import{writeProjectData}from'./project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['enum payload success','enum Value{Some(i64),None} pub fn result()->i64{let Value::Some(x)=Value::Some(61) else{panic!("not matched")};x}','61'],
 ['enum payload fallback','enum Value{Some(i64),None} pub fn result()->i64{let Value::Some(x)=Value::None else{panic!("fallback matched")};x}',null,'fallback matched'],
 ['refutable tuple pattern','pub fn result()->i64{let (x,true)=(62,true) else{panic!("not matched")};x}','62'],
 ['tuple mismatch fallback','pub fn result()->i64{let (x,true)=(1,false) else{panic!("tuple fallback")};x}',null,'tuple fallback'],
 ['record literal pattern','struct Point{x:i64,flag:bool} pub fn result()->i64{let Point{x,flag:true}=(Point{x:63,flag:true}) else{panic!("not matched")};x}','63'],
 ['nested tuple record pattern','struct Point{value:(i64,bool)} pub fn result()->i64{let Point{value:(x,true)}=(Point{value:(64,true)}) else{panic!("not matched")};x}','64'],
 ['borrowed payload remains borrowed','enum Value{Some(i64),None} fn read(p:&Value)->i64{let Value::Some(x)=p else{panic!("not matched")};*x} pub fn result()->i64{read(&Value::Some(65))}','65'],
 ['tuple rest local with fallback','pub fn result()->i64{let (first,..,last)=(6,2,6) else{panic!("not matched")};first*10+last}','66'],
 ['tuple variant rest with fallback','enum Value{Some(i64,i64,i64),None} pub fn result()->i64{let Value::Some(first,..,last)=Value::Some(6,2,7) else{panic!("not matched")};first*10+last}','67'],
 ['irrefrutable name retains source value','pub fn result()->i64{let x=68 else{panic!("unreachable")};x}','68'],
 ['constant condition without names','pub fn result()->i64{let true=true else{panic!("not matched")};69}','69'],
 ['fallback uses outer name','enum Value{Some(i64),None} pub fn result()->i64{let x=70;let Value::Some(x)=Value::None else{if x==70{panic!("outer scope")}else{panic!("wrong scope")}};x}',null,'outer scope'],
 ['initializer fails before fallback','enum Value{Some(i64),None} fn source()->Value{panic!("initializer first")} pub fn result()->i64{let Value::Some(x)=source() else{panic!("fallback later")};x}',null,'initializer first'],
 ['annotated owner','enum Value{Some(u8),None} pub fn result()->i64{let Value::Some(x):Value=Value::Some(71) else{panic!("not matched")};x as i64}','71'],
 ['parenthesized lazy Boolean initializer','pub fn result()->i64{let true=(true&&true) else{panic!("not matched")};72}','72'],
];
const cases=inputs.map(([name,source,value,abort],index)=>({name,module:'let_else_case_'+index,source,value,abort:abort??null}));
const refusals=[
 {name:'fallback must diverge',source:'enum Value{Some(i64),None} fn read(p:Value)->i64{let Value::Some(x)=p else{0};x} fn main(){}',kind:'unsupported',reason:'Rust let-else fallback: the fallback needs a represented diverging completion'},
 {name:'block-ending initializer needs parentheses',source:'struct Point{x:i64} fn read()->i64{let Point{x}=Point{x:1} else{panic!("not matched")};x} fn main(){}',kind:'unsupported',reason:'Rust let-else initializer: a block-ending initializer must be parenthesized'},
 {name:'lazy Boolean initializer needs parentheses',source:'fn read()->i64{let true=true&&true else{panic!("not matched")};0} fn main(){}',kind:'unsupported',reason:'Rust let-else initializer: a lazy Boolean initializer must be parenthesized'},
];
writeProjectData(new URL('../../parity/fixtures/rust-let-else.lino',import.meta.url),{
 schemaVersion:1,model:'owned nominal Rust record field order 1',cases,refusals,
 source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://doc.rust-lang.org/reference/statements.html#let-statements ; fallback divergence and initializer grammar restrictions remain explicit source requirements.',
 boundary:'Future paired full graph/codec/four-emission/original-Rust value and exact-error controls. No observations. Early return, resource/drop effects, full outlives and divergence inference, loops and source profile composition retain their own requirements.',
});
