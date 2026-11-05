// Literal policy inputs and source data. Never invoke builders or compilers.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const body={$:'sourceConstant',value:{$:'integer',value:'7'}};
const guardOrders=[
 {name:'empty guards require no realm choice',types:[],orders:[],oneRealm:false,expected:[]},
 {name:'one complete natural guard',types:['nat'],orders:[0],oneRealm:true,expected:[0]},
 {name:'source guard order differs from parameter order',types:['nat','nat'],orders:[1,0],oneRealm:true,expected:[1,0]},
 {name:'lost guard position refuses',types:['nat'],orders:[1],oneRealm:true,expected:[-1]},
 {name:'duplicate positions refuse',types:['nat','nat'],orders:[0,0],oneRealm:true,expected:[-1]},
 {name:'other source domains refuse',types:['float'],orders:[0],oneRealm:true,expected:[-1]},
 {name:'incomplete columns refuse',types:['nat'],orders:[],oneRealm:true,expected:[-1]},
 {name:'cross realm guard policy refuses',types:['nat'],orders:[0],oneRealm:false,expected:[-1]},
 {name:'fractional source positions refuse',types:['nat'],orders:[0.5],oneRealm:true,expected:[-1]}
];
const casts=[
 {name:'exact widening keeps its value',from:'nat',to:'int',flavor:'exact',retainedMessage:false,order:-1,oneRealm:false,expected:1},
 {name:'clamping has its own zero branch',from:'int',to:'nat',flavor:'clamp',retainedMessage:false,order:-1,oneRealm:false,expected:2},
 {name:'complete source guard admits a checked conversion',from:'int',to:'nat',flavor:'checked',retainedMessage:true,order:0,oneRealm:true,expected:3},
 {name:'erased message policy refuses',from:'int',to:'nat',flavor:'checked',retainedMessage:false,order:0,oneRealm:true,expected:0},
 {name:'cross realm checked policy refuses',from:'int',to:'nat',flavor:'checked',retainedMessage:true,order:0,oneRealm:false,expected:0},
 {name:'unrepresented conversion refuses',from:'nat',to:'float',flavor:'exact',retainedMessage:true,order:0,oneRealm:true,expected:0},
 {name:'missing guard order refuses',from:'int',to:'nat',flavor:'checked',retainedMessage:true,order:-1,oneRealm:true,expected:0},
 {name:'fractional guard order refuses',from:'int',to:'nat',flavor:'checked',retainedMessage:true,order:0.5,oneRealm:true,expected:0}
];
const patterns=[
 {name:'both natural branches',kind:'nat',patterns:['natZero','natSucc'],expected:[0,1]},
 {name:'reversed source branches',kind:'nat',patterns:['natSucc','natZero'],expected:[1,0]},
 {name:'bound fallback retains the original subject',kind:'nat',patterns:['natZero','bind'],expected:[0,1]},
 {name:'one complete wildcard',kind:'nat',patterns:['wild'],expected:[0,0]},
 {name:'zero fallback follows the successor arm',kind:'nat',patterns:['natSucc','wild'],expected:[1,0]},
 {name:'duplicate zero refuses',kind:'nat',patterns:['natZero','natZero','natSucc'],expected:[]},
 {name:'duplicate fallback refuses',kind:'nat',patterns:['wild','bind'],expected:[]},
 {name:'missing successor refuses',kind:'nat',patterns:['natZero'],expected:[]},
 {name:'other scrutinee domain refuses',kind:'float',patterns:['wild'],expected:[]},
 {name:'foreign constructor refuses',kind:'nat',patterns:['ctor'],expected:[]}
];
const bindings=[{pattern:'natZero',expected:0},{pattern:'natSucc',expected:2},{pattern:'wild',expected:0},{pattern:'bind',expected:1},{pattern:'ctor',expected:-1}];
const realms=[{realms:[],expected:false},{realms:['1000','1000'],expected:true},{realms:['1000','1001'],expected:false},{realms:['18446744073709551615','18446744073709551615'],expected:true}];
const builders=[
 {name:'literal Error keeps unpaired units and NUL',operation:'error',messageUnits:[55296,0],parameters:[]},
 {name:'guard retains its bound subject and body',operation:'guard',identity:0,messageUnits:[110],body,parameters:[0]},
 {name:'clamping keeps a separate zero branch',operation:'cast',identity:0,messageUnits:[],clamp:true,parameters:[0]},
 {name:'checked cast retains its exact source Error',operation:'cast',identity:0,messageUnits:[55296],clamp:false,parameters:[0]},
 {name:'truncated difference retains both bound operands',operation:'difference',left:0,right:1,parameters:[0,1]},
 {name:'predecessor keeps its source binding',operation:'predecessor',identity:0,parameters:[0]},
 {name:'natural match retains both complete branches',operation:'match',identity:0,zero:body,succ:{$:'sourceBinding',identity:0},parameters:[0]}
];
writeProjectData(new URL('../../parity/fixtures/javascript-call-natural-source.lino',import.meta.url),{schemaVersion:1,model:'checked JavaScript natural source construction 1',nodeVersionRange:'>=22.15 <23 or >=24 <25',rustVersionRange:'>=1.90 <1.100',guardOrders,casts,patterns,bindings,realms,builders});
