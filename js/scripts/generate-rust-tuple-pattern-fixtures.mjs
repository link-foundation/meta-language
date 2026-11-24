// Source/expectation declarations only. Never execute behavioral products here.
import{writeProjectData}from'./project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['tuple projection match','pub fn result()->i64{match (4,2){(a,b)=>a*10+b}}','42'],
 ['tuple Boolean alternatives','pub fn result()->i64{match (4,true){(a,false)=>a,(a,true)=>a+7}}','11'],
 ['tuple literal priority','pub fn result()->i64{match (3,4){(3,4)=>12,(3,_)=>9,_=>0}}','12'],
 ['tuple literal fallback','pub fn result()->i64{match (3,5){(3,4)=>12,(3,x)=>x+8,_=>0}}','13'],
 ['tuple wildcard fallback','pub fn result()->i64{match (9,5){(3,4)=>12,_=>14}}','14'],
 ['nested tuple match','pub fn result()->i64{match ((2,3),(4,true)){((a,b),(c,true))=>a+b+c+6,_=>0}}','15'],
 ['tuple prefix rest','pub fn result()->i64{match (1,2,16){(..,last)=>last}}','16'],
 ['tuple suffix rest','pub fn result()->i64{match (17,2,3){(first,..)=>first}}','17'],
 ['tuple middle rest','pub fn result()->i64{match (8,2,3,10){(first,..,last)=>first+last}}','18'],
 ['tuple all rest','pub fn result()->i64{match (1,2,3){(..)=>19}}','19'],
 ['tuple zero-width rest','pub fn result()->i64{match (9,11){(first,..,last)=>first+last}}','20'],
 ['single tuple field','pub fn result()->i64{match (21,){(value,)=>value}}','21'],
 ['parenthesized binder','pub fn result()->i64{match 22{(value)=>value}}','22'],
 ['unit pattern','pub fn result()->i64{match (){()=>23}}','23'],
 ['tuple nested in named record let','struct Point{value:(u8,u16)} pub fn result()->i64{let Point{value:(a,b)}=Point{value:(2,4)};a as i64*10+b as i64}','24'],
 ['tuple nested in record match','struct Point{value:(i64,bool)} pub fn result()->i64{match (Point{value:(18,true)}){Point{value:(a,true)}=>a+7,_=>0}}','25'],
 ['borrowed tuple match','fn read(p:&(i64,i64))->i64{match p{(a,b)=>*a+*b}} pub fn result()->i64{read(&(12,14))}','26'],
 ['tuple receiver field scopes','struct Point{value:(i64,i64)} pub fn result()->i64{let Point{value:(a,b)}=Point{value:(13,14)};a+b}','27'],
 ['tuple internal field name hygiene','pub fn result()->i64{let ml_tuple_field1=10;let ml_tuple_field2=18;match (1,2){(a,b)=>ml_tuple_field1+ml_tuple_field2+a-b+1}}','28'],
 ['tuple initializer failure order','fn source()->(i64,i64){panic!("tuple source first")} pub fn result()->i64{match source(){(..)=>panic!("arm later")}}',null,'tuple source first'],
];
const cases=inputs.map(([name,source,value,abort],index)=>({name,module:'tuple_pattern_case_'+index,source,value,abort:abort??null}));
const refusals=[
 {name:'tuple pattern arity',source:'fn f(p:(i64,i64))->i64{match p{(a,)=>a}} fn main(){}',kind:'type',reason:'tuple pattern has the wrong type or number of fields'},
 {name:'tuple pattern type',source:'fn f(p:i64)->i64{match p{(a,)=>a}} fn main(){}',kind:'type',reason:'tuple pattern has the wrong type or number of fields'},
 {name:'multiple tuple rest positions',source:'fn f(p:(i64,i64))->i64{match p{(..,..)=>0}} fn main(){}',kind:'unsupported',reason:'tuple rest pattern: a tuple pattern has at most one rest position'},
 {name:'duplicate tuple pattern binding',source:'fn f(p:(i64,i64))->i64{match p{(a,a)=>a}} fn main(){}',kind:'unsupported',reason:'duplicate source pattern binding: each pattern binds a source name at most once'},
 {name:'tuple patterns need exhaustive literal coverage',source:'fn f(p:(i64,i64))->i64{match p{(1,_)=>1}} fn main(){}',kind:'type',reason:'non-exhaustive match'},
];
const versions={JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'};
writeProjectData(new URL('../../parity/fixtures/rust-tuple-patterns.lino',import.meta.url),{
 schemaVersion:1,model:'owned nominal Rust record field order 1',cases,refusals,versions,
 source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',
 sourceBasis:'https://doc.rust-lang.org/reference/patterns.html#tuple-patterns ; https://doc.rust-lang.org/reference/patterns.html#rest-patterns ; tuple matching expands into ordinary positional columns without new checked terms.',
 boundary:'Future paired complete definition/program graph, text/binary codec, four-emission, original/emitted Rust values and exact refusals. No observations. Full ownership/drop, tuple-struct rest and general source-pattern extensions remain separate requirements.',
});
const corrections=[
 ['tuple-struct','struct Pair(u32, bool); fn main() {}','Rust struct item','pub fn result()->i64{let Pair(value,flag)=Pair(4,true);if flag{value as i64}else{0}}','4'],
 ['tuple-match','fn main() { let pair = (1u32, true); let value = match pair { (value, _) => value }; }','tuple match pattern','pub fn result()->u32{let pair=(1u32,true);match pair{(value,_)=>value}}','1'],
];
for(const[name,source,messagePattern,caller,value]of corrections)writeProjectData(new URL('../../parity/fixtures/rust-product-correction-'+name+'.lino',import.meta.url),{
 schemaVersion:1,model:'owned nominal Rust record field order 1',versions,source:source+'\n'+caller+'\n',refusals:[],
 cases:[{name,module:'',source,value,abort:null,correctionKind:name,previousExpectation:{kind:'unsupported',messagePattern}}],
 correctionBasis:'https://doc.rust-lang.org/reference/items/structs.html ; https://doc.rust-lang.org/reference/patterns.html#tuple-patterns ; newly represented valid source forms replace obsolete translator limitations. Exact predecessor inputs and expectations remain mandatory alongside independent original Rust values and full paired graph controls.',
});
