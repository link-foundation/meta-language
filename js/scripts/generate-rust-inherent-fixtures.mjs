// Source/input declarations only. No interpreted or compiled product executes.
import{writeProjectData}from'./project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft to author declarations');
const cases=[
 ['associated constructor','struct Point{x:i64,y:i64} impl Point{fn new(x:i64,y:i64)->Self{Self{y,x}}} pub fn result()->i64{let p=Point::new(2,7);p.x*10+p.y}', '27'],
 ['shared self receiver','struct Point{x:i64,y:i64} impl Point{fn sum(&self)->i64{self.x+self.y}} pub fn result()->i64{Point{x:3,y:7}.sum()}', '10'],
 ['owned self receiver','struct Point{x:i64,y:i64} impl Point{fn sum(self)->i64{self.x+self.y}} pub fn result()->i64{Point{x:8,y:7}.sum()}', '15'],
 ['borrowed source receiver','struct Point{x:i64,y:i64} impl Point{fn sum(&self)->i64{self.x+self.y}} fn read(p:&Point)->i64{p.sum()} pub fn result()->i64{read(&Point{x:5,y:6})}', '11'],
 ['explicit associated method call','struct Point{x:i64,y:i64} impl Point{fn sum(&self)->i64{self.x+self.y}} pub fn result()->i64{Point::sum(&Point{x:4,y:8})}', '12'],
 ['aliased owner','struct Point{x:i64} use self::Point as Alias; impl Alias{fn new(x:i64)->Self{Self{x}} fn read(&self)->i64{self.x}} pub fn result()->i64{Alias::new(13).read()}', '13'],
 ['multiple implementation blocks','struct Point{x:i64} impl Point{fn new(x:i64)->Self{Self{x}}} impl Point{fn read(&self)->i64{self.x}} pub fn result()->i64{Point::new(14).read()}', '14'],
 ['forward method declaration','pub fn result()->i64{Point::new(15).read()} struct Point{x:i64} impl Point{fn new(x:i64)->Self{Self{x}} fn read(&self)->i64{self.x}}', '15'],
 ['implementation before datatype','impl Point{fn new(x:i64)->Self{Self{x}} fn read(&self)->i64{self.x}} struct Point{x:i64} pub fn result()->i64{Point::new(16).read()}', '16'],
 ['owned Self result','struct Point{x:i64} impl Point{fn identity(self)->Self{self} fn read(&self)->i64{self.x}} pub fn result()->i64{Point{x:17}.identity().read()}', '17'],
 ['Self associated path','struct Point{x:i64} impl Point{fn seed()->i64{18} fn new()->Self{Self{x:Self::seed()}} fn read(&self)->i64{self.x}} pub fn result()->i64{Point::new().read()}', '18'],
 ['chained methods','struct Point{x:i64} impl Point{fn scale(&self,factor:i64)->Self{Self{x:self.x*factor}} fn read(&self)->i64{self.x}} pub fn result()->i64{Point{x:7}.scale(3).read()}', '21'],
 ['actual lexical module','fn read()->i64{3} struct Point{x:i64} impl Point{fn read(&self)->i64{read()+self.x}} pub fn result()->i64{Point{x:4}.read()}', '7'],
 ['explicit source module','fn seed()->i64{19} struct Point{x:i64} impl Point{fn new()->Self{Self{x:self::seed()}} fn read(&self)->i64{self.x}} pub fn result()->i64{Point::new().read()}', '19'],
 ['source parent module','fn seed()->i64{20} mod owner{struct Point{x:i64} impl Point{fn new()->Self{Self{x:super::seed()}} fn read(&self)->i64{self.x}} pub fn read()->i64{Point::new().read()}} pub fn result()->i64{owner::read()}', '20'],
 ['distinct actual method owners','struct Left{x:i64} struct Right{x:i64} impl Left{fn read(&self)->i64{self.x}} impl Right{fn read(&self)->i64{self.x}} pub fn result()->i64{Left{x:2}.read()*10+Right{x:3}.read()}', '23'],
 ['enum inherent method','enum Event{Point{x:i64,y:i64},Empty} impl Event{fn read(self)->i64{match self{Self::Empty=>0,Self::Point{y,x}=>x*10+y}}} pub fn result()->i64{Event::Point{x:2,y:4}.read()}', '24'],
 ['receiver failure before argument','struct Point{x:i64} impl Point{fn add(&self,value:i64)->i64{self.x+value}} fn receiver()->Point{panic!("receiver first")} fn argument()->i64{panic!("argument later")} pub fn result()->i64{receiver().add(argument())}', null,'receiver first'],
 ['method argument failure order','struct Point{x:i64} impl Point{fn add(&self,left:i64,right:i64)->i64{self.x+left+right}} fn left()->i64{panic!("left first")} fn right()->i64{panic!("right later")} pub fn result()->i64{Point{x:1}.add(left(),right())}', null,'left first'],
 ['unit receiver retains variable','struct Marker; impl Marker{fn identity(self)->Self{self} fn read(&self)->i64{25}} pub fn result()->i64{Marker.identity().read()}', '25'],
 ['tuple receiver field','struct Pair(i64,i64); impl Pair{fn read(&self)->i64{self.0*10+self.1}} pub fn result()->i64{Pair(2,6).read()}', '26'],
 ['contextual fixed fields in Self','struct Pair{left:u8,right:u16} impl Pair{fn new()->Self{Self{right:7,left:2}} fn read(&self)->i64{self.left as i64*10+self.right as i64}} pub fn result()->i64{Pair::new().read()}', '27'],
];
const rows=cases.map(([name,source,value,abort],index)=>({name,module:'inherent_case_'+index,source,value,abort:abort??null}));
const refusals=[
 ['generic implementation','struct Point{x:i64} impl<T> Point{fn f()->i64{1}} fn main(){}','generic Rust implementation: type parameters require their complete trait and ownership interpretation'],
 ['trait implementation','struct Point{x:i64} impl SomeTrait for Point{fn f()->i64{1}} fn main(){}','generic or trait Rust implementation: traits and predicates need their complete dispatch interpretation'],
 ['mutable receiver','struct Point{x:i64} impl Point{fn read(&mut self)->i64{self.x}} fn main(){}','Rust mutable or explicit lifetime receiver: receiver mutation and named regions need an explicit ownership interpretation'],
 ['named lifetime receiver','struct Point{x:i64} impl Point{fn read<\'a>(&\'a self)->i64{self.x}} fn main(){}','Rust mutable or explicit lifetime receiver: receiver mutation and named regions need an explicit ownership interpretation'],
 ['explicit receiver type','struct Point{x:i64} impl Point{fn read(self:Self)->i64{self.x}} fn main(){}','Rust explicit receiver type: explicit receiver types need their complete arbitrary-self interpretation'],
 ['associated constant','struct Point{x:i64} impl Point{const VALUE:i64=1;} fn main(){}','Rust associated item: associated constants, types, macros and modified functions need their own source interpretation'],
 ['implementation in another source module','mod owner{pub struct Point{pub x:i64}} impl owner::Point{fn read(&self)->i64{self.x}} fn main(){}','Rust implementation location: cross-module implementation placement requires its explicit namespace mapping'],
 ['unknown nominal owner','impl Unknown{fn read()->i64{1}} fn main(){}','Rust implementation owner: an inherent implementation needs its actual source datatype'],
 ['owned method through borrowed receiver','struct Point{x:i64} impl Point{fn read(self)->i64{self.x}} fn read(p:&Point)->i64{p.read()} fn main(){}','Rust inherent receiver: the method needs its actual owned or single immutable borrowed receiver'],
 ['boxed receiver needs dereference model','struct Point{x:i64} impl Point{fn read(&self)->i64{self.x}} pub fn result()->i64{Box::new(Point{x:1}).read()} fn main(){}','Rust inherent receiver: the method needs its actual owned or single immutable borrowed receiver'],
].map(([name,source,reason])=>({name,source,kind:'unsupported',reason}));
refusals.push({name:'closed signature profile needs composition',source:'struct Point{x:i64} impl Point{fn read(&self)->i64{self.x}} fn main(){}',kind:'unsupported',reason:'Rust inherent profile composition: closed source models need an explicit method composition proof',sourceSignatureModel:'Rust source function signature 1'});
writeProjectData(new URL('../../parity/fixtures/rust-inherent.lino',import.meta.url),{
 schemaVersion:1,model:'pure owned Rust inherent function and immutable receiver 1',cases:rows,refusals,
 source:rows.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 boundary:'Execute every declared ownership, namespace, construction, method, receiver/argument-order, error and codec cell in both runtimes; compare full graphs, all four emissions and original/emitted Rust. No execution observations are authored. Trait/generic dispatch, mutable/boxed/multiple-reference receivers, cross-module implementations, arbitrary-self/region models, associated constants/types/macros and closed-profile composition remain separate obligations.',
});
