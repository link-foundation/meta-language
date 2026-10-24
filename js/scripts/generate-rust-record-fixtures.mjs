// Source/input declarations only. No translated product or assertion executes.
import{writeProjectData}from'./project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft to author declarations');
const cases=[
 ['named order','pub struct Point { pub x:i64,pub y:i64 } pub fn result()->i64{let p=Point{y:7,x:2};p.x*10+p.y}', '27'],
 ['field puns','struct Point{x:i64,y:i64} pub fn result()->i64{let x=4;let y=3;let p=Point{y,x};p.x*10+p.y}', '43'],
 ['forward declaration','pub fn result()->i64{let p=Point{y:9,x:1};p.x*10+p.y} struct Point{x:i64,y:i64}', '19'],
 ['nested nominal fields','struct Inner{x:i64} struct Outer{left:Inner,right:Inner} pub fn result()->i64{let p=Outer{right:Inner{x:6},left:Inner{x:5}};p.left.x*10+p.right.x}', '56'],
 ['qualified alias','mod owner{pub struct Point{pub x:i64,pub y:i64}} use self::owner::Point as Pair; pub fn result()->i64{let p=Pair{y:8,x:3};p.x*10+p.y}', '38'],
 ['tuple struct','struct Point(i64,i64); pub fn result()->i64{let p=Point(6,2);p.0*10+p.1}', '62'],
 ['tuple braces','struct Point(i64,i64); pub fn result()->i64{let p=Point{1:4,0:8};p.0*10+p.1}', '84'],
 ['unit value','struct Marker; fn accept(_:Marker)->i64{7} pub fn result()->i64{accept(Marker)}', '7'],
 ['unit braces','struct Marker; fn accept(_:Marker)->i64{8} pub fn result()->i64{accept(Marker{})}', '8'],
 ['empty named struct','struct Marker{} fn accept(_:Marker)->i64{9} pub fn result()->i64{accept(Marker{})}', '9'],
 ['empty tuple struct','struct Marker(); fn accept(_:Marker)->i64{10} pub fn result()->i64{accept(Marker())}', '10'],
 ['hygienic construction','struct Point{x:i64,y:i64} pub fn result()->i64{let ml_record_value0=8;let ml_record_value1=7;let p=Point{y:ml_record_value1,x:ml_record_value0};p.x*10+p.y}', '87'],
 ['borrowed nominal owner','struct Point{x:i64,y:i64} fn read(p:&Point)->i64{p.y*10+p.x} pub fn result()->i64{let p=Point{x:3,y:9};read(&p)}', '93'],
 ['function result owner','struct Point{x:i64,y:i64} fn make()->Point{Point{y:1,x:7}} pub fn result()->i64{make().x*10+make().y}', '71'],
 ['distinct same-shaped owners','struct Left{x:i64} struct Right{x:i64} pub fn result()->i64{let a=Left{x:1};let b=Right{x:2};a.x*10+b.x}', '12'],
 ['first failure source order','struct Point{left:i64,right:i64} pub fn result()->i64{let p=Point{right:panic!("right first"),left:panic!("left later")};p.left}', null,'right first'],
];
const rows=cases.map(([name,source,value,abort],index)=>({name,module:'record_case_'+index,source,value,abort:abort??null}));
const refusals=[
 ['missing field','struct Point{x:i64,y:i64} fn f()->Point{Point{x:1}} fn main(){}','type','a Rust record construction has duplicate, unknown or missing fields'],
 ['duplicate field','struct Point{x:i64} fn f()->Point{Point{x:1,x:2}} fn main(){}','type','a Rust record construction has duplicate, unknown or missing fields'],
 ['unknown field','struct Point{x:i64} fn f()->Point{Point{x:1,y:2}} fn main(){}','type','a Rust record construction has duplicate, unknown or missing fields'],
 ['duplicate declaration field','struct Point{x:i64,x:i64} fn main(){}','type','a Rust record declares a duplicate field'],
 ['unknown projection','struct Point{x:i64} fn f(p:Point)->i64{p.y} fn main(){}','type','a Rust record has no field y'],
 ['generic owner needs ownership model','struct Point<T>{x:T} fn main(){}','unsupported','generic Rust record: record parameters and predicates need their complete ownership interpretation'],
 ['builtin shadow needs namespace model','struct String{x:i64} fn main(){}','unsupported','Rust record type owner: a builtin-shadowing record needs its complete namespace interpretation'],
 ['functional update needs move model','struct Point{x:i64,y:i64} fn f(p:Point)->Point{Point{x:2,..p}} fn main(){}','unsupported','Rust record update: update syntax needs complete move and field provenance'],
].map(([name,source,kind,reason])=>({name,source,kind,reason}));
writeProjectData(new URL('../../parity/fixtures/rust-records.lino',import.meta.url),{
 schemaVersion:1,model:'owned nominal Rust records 1',cases:rows,refusals,
 source:rows.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 boundary:'Execute every declared constructor, projection, owner, order and refusal cell in both runtimes; compare whole typed graphs, both codecs, four complete emissions, original Rust and emitted Rust. These are future declarations, not observations. Traits, layout, destructors, generic owners, field writes, functional update and patterns remain separate implementation obligations.',
});
