// Eager consumption has its own shared role; no loop invocation is fabricated.
const sourceVersionRange='ECMAScript 2024-2025; Node >=22.15 <23 || >=24 <25';
export const disjointCollectionSpreadSourceAnchors=[{path:'js/src/translation/emit-common.js',source:'return [...this.assumptions.values()];'},{path:'js/src/translation/emit-common.js',source:'return [...this.encodings.values()];'}];
export const disjointCollectionSpreadDecisions=[
 {operation:'readDisjointCollectionIterationMarkerRole',arguments:['#closed collection iteration 0 values'],expected:'loop'},
 {operation:'readDisjointCollectionIterationMarkerRole',arguments:['#closed collection spread 0 values'],expected:'eager'},
 {operation:'readDisjointCollectionIterationMarkerRole',arguments:['values'],expected:''},
 {operation:'acceptDisjointCollectionSpreadReceiver',arguments:['name'],expected:true},
 {operation:'acceptDisjointCollectionSpreadReceiver',arguments:['field'],expected:true},
 {operation:'acceptDisjointCollectionSpreadReceiver',arguments:['app'],expected:false},
 {operation:'acceptDisjointCollectionSpreadReceiver',arguments:['call'],expected:false},
 {operation:'acceptDisjointCollectionSpreadReceiver',arguments:['method'],expected:false},
 {operation:'acceptDisjointCollectionSpreadReceiver',arguments:['index'],expected:false},
 {operation:'acceptDisjointCollectionIterationScope',arguments:[true,1,0,0,false,false],expected:false},
 {operation:'acceptDisjointCollectionIterationScope',arguments:[true,0,0,1,false,false],expected:false},
 {operation:'acceptDisjointCollectionIterationScope',arguments:[false,0,0,0,false,false],expected:false},
];
export const disjointCollectionSpreadOriginals=[
 {name:'owned source producer executes once before its eager name capture',source:`function make() { console.log('receiver'); return new Map([['a', 1], ['b', 2]]); }
function collect() {
  const source = make();
  source.set('a', 1);
  return [...source.values()].join(',');
}
console.log(collect());`,stdout:'receiver\n1,2\n',sourceVersionRange,singleCallFunction:'collect',singleCallOwners:['make']},
 {name:'own-shaped assumption and encoding Map values become independent outer snapshots',source:`class EmitState {
  constructor() {
    this.assumptions = new Map([['a', 'first'], ['b', 'second']]);
    this.encodings = new Map([['x', 'one'], ['y', 'two']]);
  }
  assumptionList() { return [...this.assumptions.values()]; }
  encodingList() { return [...this.encodings.values()]; }
  change() { this.assumptions.set('b', 'changed'); this.encodings.set('y', 'changed'); }
}
const state = new EmitState();
const assumptions = state.assumptionList();
const encodings = state.encodingList();
state.change();
console.log(assumptions.join(','));
console.log(encodings.join(','));`,stdout:'first,second\none,two\n',sourceVersionRange,singleCallOwners:[]},
 {name:'eager spread captures before later sibling Map mutation',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const result = [...source.values(), source.set('b', 9).size];
  return result.join(',') + ':' + (source.get('b') ?? 0);
}
console.log(collect());`,stdout:'1,2,2:9\n',sourceVersionRange,singleCallOwners:[]},
 {name:'earlier sibling runs before snapshot and later sibling runs afterward',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const result = [source.set('a', 7).size, ...source.values(), source.set('b', 9).size];
  return result.join(',') + ':' + (source.get('b') ?? 0);
}
console.log(collect());`,stdout:'2,7,2,2:9\n',sourceVersionRange,singleCallOwners:[]},
 {name:'two eager spreads capture at their own positions around mutation',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const result = [...source.values(), source.set('b', 9).size, ...source.values()];
  return result.join(',');
}
console.log(collect());`,stdout:'1,2,2,1,9\n',sourceVersionRange,singleCallOwners:[]},
 {name:'Map keys spread preserves insertion order and an independent outer array',source:`function collect() {
  const source = new Map([['b', 2], ['a', 1]]);
  const result = [...source.keys()];
  source.delete('a'); source.set('c', 3);
  return result.join(',');
}
console.log(collect());`,stdout:'b,a\n',sourceVersionRange,singleCallOwners:[]},
 {name:'Map entries spread retains independent actual String Number pairs',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const result = [...source.entries()];
  source.set('b', 9);
  return result.map(pair => pair[0] + ':' + pair[1]).join(',');
}
console.log(collect());`,stdout:'a:1,b:2\n',sourceVersionRange,singleCallOwners:[]},
 {name:'Set keys and values spreads retain original elements and order',source:`function collect() {
  const source = new Set([2, 1]);
  const keys = [...source.keys()];
  const values = [...source.values()];
  source.delete(1); source.add(3);
  return keys.join(',') + ':' + values.join(',');
}
console.log(collect());`,stdout:'2,1:2,1\n',sourceVersionRange,singleCallOwners:[]},
 {name:'Set entries spread retains both same-element pair positions',source:`function collect() {
  const source = new Set([2, 1]);
  const result = [...source.entries()];
  source.delete(1); source.add(3);
  return result.map(pair => pair[0] + pair[1]).join(',');
}
console.log(collect());`,stdout:'4,2\n',sourceVersionRange,singleCallOwners:[]},
 {name:'eager values snapshot retains nested array reference aliases',source:`function collect() {
  const source = new Map([['a', [1, 2]], ['b', [3, 4]]]);
  const result = [...source.values()];
  result[0].fill(9);
  const first = source.get('a') ?? [0];
  return first.join(',');
}
console.log(collect());`,stdout:'9,9\n',sourceVersionRange,singleCallOwners:[]},
];
export const disjointCollectionSpreadRefusals=[
 {name:'stored iterator remains live rather than an eager source marker',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const iterator = source.values();
  source.set('b', 9);
  return [...iterator].join(',');
}
console.log(collect());`,originalStdout:'1,9\n',sourceVersionRange},
 {name:'receiver factory stays outside owned name field source profile',source:`function make() { console.log('receiver'); return new Map([['a', 1], ['b', 2]]); }
function collect() {
  const mutation = new Map([['seed', 0]]);
  mutation.set('value', 1);
  return [...make().values()].join(',');
}
console.log(collect());`,originalStdout:'receiver\n1,2\n',sourceVersionRange},
 {name:'nonzero argument values call retains exact signature boundary',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  source.set('a', 1);
  return [...source.values(0)].join(',');
}
console.log(collect());`,originalStdout:'1,2\n',sourceVersionRange},
 {name:'computed receiver lookup lacks actual allocation provenance',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const holders = [source];
  source.set('a', 1);
  return [...holders[0].values()].join(',');
}
console.log(collect());`,originalStdout:'1,2\n',sourceVersionRange},
];
