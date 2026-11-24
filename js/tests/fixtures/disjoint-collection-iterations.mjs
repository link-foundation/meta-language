// Independently written complete inputs and literal expectations. No draft
// execution observations; old general live-iterator refusals remain binding.
const sourceVersionRange='ECMAScript 2024-2025; Node >=22.15 <23 || >=24 <25';
export const disjointCollectionIterationSourceAnchors=[{path:'js/src/translation/emit-common.js',source:'for (const entry of this.program.declarations.values()) {'},{path:'js/src/translation/emit-common.js',source:'this.moduleNames.set(key, claim(`${moduleSpace}:${parent}`, moduleName(segment)));'}];
export const disjointCollectionIterationDecisions=[
 {operation:'readDisjointCollectionIterationMethodEffect',arguments:['map','set'],expected:'write'},
 {operation:'readDisjointCollectionIterationMethodEffect',arguments:['set','add'],expected:'write'},
 {operation:'readDisjointCollectionIterationMethodEffect',arguments:['array','fill'],expected:'write'},
 {operation:'readDisjointCollectionIterationMethodEffect',arguments:['record','writeRecordField0'],expected:'write'},
 {operation:'readDisjointCollectionIterationMethodEffect',arguments:['map','get'],expected:'read'},
 {operation:'readDisjointCollectionIterationMethodEffect',arguments:['array','map'],expected:'read'},
 {operation:'readDisjointCollectionIterationMethodEffect',arguments:['cell','write'],expected:'unknown'},
 {operation:'readDisjointCollectionIterationMethodEffect',arguments:['future reference','set'],expected:'unknown'},
 {operation:'readDisjointCollectionIterationMethodEffect',arguments:['map','futureOperation'],expected:'unknown'},
 {operation:'readDisjointCollectionIterationMethodEffect',arguments:['record','set'],expected:'unknown'},
 {operation:'readDisjointCollectionIterationMethodEffect',arguments:['set','get'],expected:'unknown'},
 {operation:'readDisjointCollectionIterationMethodEffect',arguments:['record','writeRecordField00'],expected:'unknown'},
 {operation:'acceptDisjointCollectionIterationReadExpression',arguments:['binary'],expected:true},
 {operation:'acceptDisjointCollectionIterationReadExpression',arguments:['cellRead'],expected:true},
 {operation:'acceptDisjointCollectionIterationReadExpression',arguments:['cellWrite'],expected:false},
 {operation:'acceptDisjointCollectionIterationReadExpression',arguments:['referenceWrite'],expected:false},
 {operation:'acceptDisjointCollectionIterationReadExpression',arguments:['math'],expected:false},
 {operation:'acceptDisjointCollectionIterationReadExpression',arguments:['futureCheckedOperation'],expected:false},
 {operation:'acceptDisjointCollectionIterationCallbackHead',arguments:['function reference'],expected:true},
 {operation:'acceptDisjointCollectionIterationCallbackHead',arguments:['call'],expected:false},
 {operation:'acceptDisjointCollectionIterationScope',arguments:[true,0,0,0,false,false],expected:true},
 {operation:'acceptDisjointCollectionIterationScope',arguments:[true,0,0,0,false,true],expected:false},
 {operation:'acceptDisjointCollectionIterationOperation',arguments:['map','values'],expected:true},
 {operation:'acceptDisjointCollectionIterationOperation',arguments:['map','entries'],expected:false},
];
export const disjointCollectionIterationOriginals=[
 {name:'distinct Map allocations retain value iteration order',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const destination = new Map([['seed', 0]]);
  for (const value of source.values()) destination.set('value', value);
  return destination.get('value') ?? 0;
}
console.log(collect());`,stdout:'2\n',sourceVersionRange,singleCallOwners:[]},
 {name:'actual aliases through record fields preserve disjoint allocation roots',source:`function collect() {
  const state = { source: new Map([['a', 1], ['b', 2]]), destination: new Map([['seed', 0]]) };
  const alias = state;
  for (const value of alias.source.values()) state.destination.set('value', value);
  return alias.destination.get('value') ?? 0;
}
console.log(collect());`,stdout:'2\n',sourceVersionRange,singleCallOwners:[]},
 {name:'known helper arguments carry actual distinct receiver identities',source:`function put(destination, value) { destination.set('value', value); }
function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const destination = new Map([['seed', 0]]);
  for (const value of source.values()) put(destination, value);
  return destination.get('value') ?? 0;
}
console.log(collect());`,stdout:'2\n',sourceVersionRange,singleCallOwners:[]},
 {name:'Set values may mutate a different actual Set allocation',source:`function collect() {
  const source = new Set([1, 2]);
  const destination = new Set([0]);
  for (const value of source.values()) destination.add(value);
  return destination.size;
}
console.log(collect());`,stdout:'3\n',sourceVersionRange,singleCallOwners:[]},
 {name:'own-shaped closed EmitState fields and nested array mapper write other maps',source:`class EmitState {
  constructor() {
    this.program = { declarations: new Map([['a', { name: 'a', modulePath: ['root', 'leaf'] }]]) };
    this.names = new Map([['seed', 'seed']]);
    this.moduleNames = new Map([['seed', 'seed']]);
  }
  assign() {
    for (const entry of this.program.declarations.values()) {
      const segments = entry.modulePath.map(segment => {
        this.moduleNames.set(segment, segment);
        return segment;
      });
      this.names.set(entry.name, segments.join('.'));
    }
    return this.names.get('a') ?? '';
  }
}
const state = new EmitState();
console.log(state.assign());`,stdout:'root.leaf\n',sourceVersionRange,singleCallOwners:[]},
 {name:'array callback may write a Map while retaining original array slots',source:`function collect() {
  const values = [1, 2];
  const names = new Map([['seed', 0]]);
  const result = values.map(value => { names.set('value', value); return value + 1; });
  return result.join(',') + ':' + (names.get('value') ?? 0);
}
console.log(collect());`,stdout:'2,3:2\n',sourceVersionRange,singleCallOwners:[]},
 {name:'Array.from mapper may mutate a disjoint actual Map',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const names = new Map([['seed', 0]]);
  const result = Array.from(source, pair => { names.set('value', pair[1]); return pair[1]; });
  return result.join(',') + ':' + (names.get('value') ?? 0);
}
console.log(collect());`,stdout:'1,2:2\n',sourceVersionRange,singleCallOwners:[]},
];
export const disjointCollectionIterationRefusals=[
 {name:'same Map alias requires live future value reads',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const alias = source;
  for (const value of source.values()) { console.log(value); alias.set('b', 9); }
}
collect();`,originalStdout:'1\n9\n',sourceVersionRange,reasonIncludes:'disjoint'},
 {name:'different field names do not prove different Map identities',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const state = { source, destination: source };
  for (const value of state.source.values()) { console.log(value); state.destination.set('b', 9); }
}
collect();`,originalStdout:'1\n9\n',sourceVersionRange,reasonIncludes:'disjoint'},
 {name:'known helper preserves a same receiver alias',source:`function put(destination) { destination.set('b', 9); }
function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  for (const value of source.values()) { console.log(value); put(source); }
}
collect();`,originalStdout:'1\n9\n',sourceVersionRange,reasonIncludes:'disjoint'},
 {name:'Map payload lookup does not invent a disjoint allocation proof',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const holders = new Map([['target', source]]);
  for (const value of source.values()) { console.log(value); const target = holders.get('target') ?? source; target.set('b', 9); }
}
collect();`,originalStdout:'1\n9\n',sourceVersionRange,reasonIncludes:'disjoint'},
 {name:'mutable array callback still requires live slots',source:`const values = [1, 2];
const result = values.map((value, index) => { if (index === 0) values.fill(9); return value; });
console.log(result.join(','));`,originalStdout:'1,9\n',sourceVersionRange},
 {name:'array indexing does not relabel a stored Map handle as its array owner',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const holders = [source];
  for (const value of source.values()) { console.log(value); holders[0].set('b', 9); }
}
collect();`,originalStdout:'1\n9\n',sourceVersionRange,reasonIncludes:'disjoint'},
 {name:'reduce result is not a fresh disjoint reference merely because it is an array',source:`const source = [1, 2];
const holders = [source];
const target = holders.reduce((previous, value) => value, source);
const result = source.map((value, index) => { if (index === 0) target.fill(9); return value; });
console.log(result.join(','));`,originalStdout:'1,9\n',sourceVersionRange},
 {name:'opaque checked Math is not an effect certificate',source:`function collect() {
  const source = new Map([['a', 1]]);
  const destination = new Map([['seed', 0]]);
  for (const value of source.values()) destination.set('value', Math.floor(value));
  return destination.get('value') ?? 0;
}
console.log(collect());`,originalStdout:'1\n',sourceVersionRange,reasonIncludes:'disjoint'},
 {name:'escaping mutable cell replacement cannot redirect a proved receiver',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  let destination = new Map([['seed', 0]]);
  const change = () => { destination = source; };
  for (const value of source.values()) { change(); console.log(value); destination.set('b', 9); }
}
collect();`,originalStdout:'1\n9\n',sourceVersionRange},
];
