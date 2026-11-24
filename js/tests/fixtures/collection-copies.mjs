// Independent whole programs and literal stdout, never recorded observations.
const sourceVersionRange='ECMAScript 2024-2025; Node >=22.15 <23 || >=24 <25';
export const collectionCopyTargets=['JavaScript','Rust','Lean','Rocq'];
export const collectionCopyTransports=['original','text','binary'];
export const collectionCopySourceAnchors=[
 {path:'js/src/translation/ir.js',source:'this.used = new Set(reserved);'},
 {path:'js/src/translation/ir.js',source:'scope.used = new Set(this.used);'},
 {path:'js/src/translation/ir.js',source:'const inner = new Map(env);'},
];
export const collectionCopyDecisions=[
 {operation:'readCollectionCopyInferenceRole',arguments:['map','map',false],expected:'copy'},
 {operation:'readCollectionCopyInferenceRole',arguments:['set','set',false],expected:'copy'},
 {operation:'readCollectionCopyInferenceRole',arguments:['map','array',false],expected:'legacy array'},
 {operation:'readCollectionCopyInferenceRole',arguments:['map','set',false],expected:'legacy array'},
 {operation:'readCollectionCopyInferenceRole',arguments:['set','record',false],expected:'legacy array'},
 {operation:'readCollectionCopyInferenceRole',arguments:['map','var',false],expected:'wait'},
 {operation:'readCollectionCopyInferenceRole',arguments:['map','var',true],expected:'legacy array'},
 {operation:'readCollectionCopySourceNormalization',arguments:[],expected:'arrayFrom'},
 {operation:'readCollectionCopySnapshotOperation',arguments:['map'],expected:'entries'},
 {operation:'readCollectionCopySnapshotOperation',arguments:['set'],expected:'values'},
 {operation:'readCollectionCopySnapshotOperation',arguments:['record'],expected:''},
 {operation:'acceptCollectionCopyArrayFromSnapshot',arguments:['map',true],expected:true},
 {operation:'acceptCollectionCopyArrayFromSnapshot',arguments:['set',true],expected:true},
 {operation:'acceptCollectionCopyArrayFromSnapshot',arguments:['map',false],expected:false},
 {operation:'acceptCollectionCopyArrayFromSnapshot',arguments:['array',true],expected:false},
 {operation:'readCollectionCopyMapperError',arguments:[true,true],expected:''},
 {operation:'readCollectionCopyMapperError',arguments:[false,false],expected:''},
 {operation:'readCollectionCopyMapperError',arguments:[true,false],expected:'a mutable collection snapshot mapper needs its complete readonly callback graph; factories and live iterator mutations need their original ordered iterator model'},
 {operation:'acceptCollectionCopyMapperSnapshot',arguments:['map','entries'],expected:true},
 {operation:'acceptCollectionCopyMapperSnapshot',arguments:['set','values'],expected:true},
 {operation:'acceptCollectionCopyMapperSnapshot',arguments:['array','values'],expected:false},
];
export const collectionCopyOriginals=[
 {name:'call-later Set copy allocates independent outer identity',sourceVersionRange,singleCallOwners:[],stdout:'false\ntrue\nfalse\ntrue\n',source:`
function copy(values){return new Set(values);}
const original=new Set([1,2]);const copied=copy(original);copied.add(3);
console.log(original.has(3));console.log(copied.has(3));console.log(original===copied);console.log(copied.has(1));
`},
 {name:'own source rename environment Map copy preserves original bindings',sourceVersionRange,singleCallOwners:[],stdout:'2\n9\nfalse\ntrue\n',source:`
function renameEnvironment(env){const inner=new Map(env);inner.set('second',9);return inner;}
const env=new Map([['first',1],['second',2]]);const inner=renameEnvironment(env);
console.log(env.get('second')??0);console.log(inner.get('second')??0);console.log(env===inner);console.log(inner.has('first'));
`},
 {name:'closed Scope shape copies actual reserved Set without sharing outer state',sourceVersionRange,singleCallOwners:[],stdout:'true\nfalse\ntrue\n',source:`
class Scope{constructor(reserved){this.used=new Set(reserved);}has(name){return this.used.has(name);}add(name){this.used.add(name);return this.used.has(name);}}
const reserved=new Set(['first']);const scope=new Scope(reserved);console.log(scope.add('second'));console.log(reserved.has('second'));console.log(scope.has('first'));
`},
 {name:'Map copy evaluates actual source factory once',sourceVersionRange,singleCallOwners:['make'],singleCallFunction:'copy',stdout:'input\n7\n',source:`
function make(){console.log('input');return new Map([['first',7]]);}
function copy(){return new Map(make());}const copied=copy();console.log(copied.get('first')??0);
`},
 {name:'Map copy retains nested array handles while isolating outer mutations',sourceVersionRange,singleCallOwners:[],stdout:'2\ntrue\nfalse\n',source:`
const child=[1];const original=new Map([['child',child]]);const copied=new Map(original);
const value=copied.get('child')??[];value.push(9);const prior=original.get('child')??[];
console.log(prior.length);console.log(value===child);copied.set('late',[3]);console.log(original.has('late'));
`},
 {name:'Set copy preserves SameValueZero normalization and insertion order',sourceVersionRange,singleCallOwners:[],stdout:'3\ntrue\ntrue\n2\ntrue\nfalse\n',source:`
const original=new Set([NaN,NaN,-0,0,2]);const copied=new Set(original);const values=Array.from(copied);
console.log(values.length);console.log(Number.isNaN(values[0]??0));console.log(Object.is(values[1]??9,0));console.log(values[2]??0);
copied.delete(0);console.log(original.has(0));console.log(copied.has(-0));
`},
 {name:'Map copy preserves replacement position and append order',sourceVersionRange,singleCallOwners:[],stdout:'first,second,third\n3\n2\n',source:`
const original=new Map([['first',1],['second',2],['first',3]]);const copied=new Map(original);copied.set('third',4);
console.log(Array.from(copied,pair=>pair[0]).join(','));console.log(copied.get('first')??0);console.log(original.size);
`},
 {name:'Map numeric key copy retains NaN and normalized signed zero',sourceVersionRange,singleCallOwners:[],stdout:'2\n2\n4\ntrue\n',source:`
const original=new Map([[-0,1],[0,2],[NaN,3],[NaN,4]]);const copied=new Map(original);
console.log(copied.size);console.log(copied.get(-0)??0);console.log(copied.get(NaN)??0);copied.delete(NaN);console.log(original.has(NaN));
`},
 {name:'Array from Map snapshots outer contents and retains actual nested handles',sourceVersionRange,singleCallOwners:[],stdout:'1\n2\n',source:`
const child=[1];const source=new Map([['child',child]]);const values=Array.from(source,pair=>pair[1]);
source.set('later',[9]);const first=values[0]??[];first.push(3);console.log(values.length);console.log((source.get('child')??[]).length);
`},
 {name:'Array from Set snapshots before source mutation',sourceVersionRange,singleCallOwners:[],stdout:'1,2\n3\n',source:`
const source=new Set([1,2]);const values=Array.from(source);source.add(3);console.log(values.join(','));console.log(source.size);
`},
 {name:'conditional call-later Map inputs resolve before the legacy array fallback',sourceVersionRange,singleCallOwners:[],stdout:'7\n9\nfalse\n',source:`
function choose(flag,left,right){return new Map(flag?left:right);}
const left=new Map([['key',7]]);const right=new Map([['key',9]]);const first=choose(true,left,right);const second=choose(false,left,right);
console.log(first.get('key')??0);console.log(second.get('key')??0);console.log(first===left);
`},
 {name:'named readonly mapper uses its actual checked function body',sourceVersionRange,singleCallOwners:[],stdout:'first,second\n',source:`
function key(pair){return pair[0];}const source=new Map([['first',1],['second',2]]);const copied=new Map(source);console.log(Array.from(copied,key).join(','));
`},
];
export const collectionCopyRefusals=[
 {name:'same constructor cannot invent a shared Map and array parameter type',sourceVersionRange,source:"function copy(values){return new Map(values);}const first=copy(new Map([['key',1]]));const second=copy([['key',2]]);"},
 {name:'Map copy does not turn primitive Set values into key value entries',sourceVersionRange,source:'const source=new Set([1,2]);const copied=new Map(source);'},
 {name:'Set copy does not admit structured Map entry identity implicitly',sourceVersionRange,source:"const source=new Map([['key',1]]);const copied=new Set(source);"},
 {name:'collection copy retains primitive key identity admission',sourceVersionRange,source:'const key={number:1};const source=new Map([[key,7]]);const copied=new Map(source);'},
 {name:'actual shadowing constructor remains a source owner refusal',sourceVersionRange,source:"function Map(values){return values;}const copied=new Map([['key',1]]);"},
 {name:'custom iterator remains outside same standard collection proof',sourceVersionRange,source:"const values={[Symbol.iterator]:function*(){yield ['key',1];}};const copied=new Map(values);"},
 {name:'live source iterator is not admitted by eager same kind copy support',sourceVersionRange,source:"const source=new Map([['key',1]]);const copied=new Map(source.entries());"},
 {name:'mutable Map mapper retains its ordered live iterator obligation',sourceVersionRange,reasonIncludes:'complete readonly callback graph',originalStdout:'2\n',expectationHistory:[{scope:'Array.from mutable standard Map with mapper',translationExpectation:'eager snapshot needs actual readonly proof; live mutation is not snapshot equivalent'}],source:"const source=new Map([['first',1]]);const values=Array.from(source,pair=>{source.set('second',2);return pair[0];});console.log(values.length);"},
 {name:'readonly returned mapper factory cannot snapshot before its source mutation',sourceVersionRange,reasonIncludes:'complete readonly callback graph',originalStdout:'factory\n2\n',expectationHistory:[{scope:'Array.from standard Map mapper factory',translationExpectation:'factory expression must finish before the original iterator starts; unknown returned callable requires operational source order'}],source:"function makeMapper(source){console.log('factory');source.set('second',2);return pair=>pair[0];}const source=new Map([['first',1]]);console.log(Array.from(source,makeMapper(source)).length);"},
];
