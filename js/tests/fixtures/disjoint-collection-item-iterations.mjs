// Additive keys/entries controls. The earlier values inputs and restrictive
// initial-profile assertions are retained unchanged in their original fixture.
const sourceVersionRange='ECMAScript 2024-2025; Node >=22.15 <23 || >=24 <25';
export const disjointCollectionItemIterationDecisions=[
 {operation:'readDisjointCollectionIterationItemForm',arguments:['map','keys'],expected:'keys'},
 {operation:'readDisjointCollectionIterationItemForm',arguments:['map','values'],expected:'values'},
 {operation:'readDisjointCollectionIterationItemForm',arguments:['map','entries'],expected:'entries'},
 {operation:'readDisjointCollectionIterationItemForm',arguments:['set','keys'],expected:'keys'},
 {operation:'readDisjointCollectionIterationItemForm',arguments:['set','values'],expected:'values'},
 {operation:'readDisjointCollectionIterationItemForm',arguments:['set','entries'],expected:'entries'},
 {operation:'readDisjointCollectionIterationItemForm',arguments:['array','entries'],expected:''},
 {operation:'readDisjointCollectionIterationItemForm',arguments:['record','keys'],expected:''},
 {operation:'readDisjointCollectionIterationItemForm',arguments:['map','get'],expected:''},
 {operation:'readDisjointCollectionIterationItemForm',arguments:['map','futureIterator'],expected:''},
];
export const disjointCollectionItemIterationOriginals=[
 {name:'Map keys retain actual key type and insertion order',source:`function collect() {
  const source = new Map([['second', 2], ['first', 1]]);
  const destination = new Map([['seed', 0]]);
  for (const key of source.keys()) { console.log(key); destination.set(key, 1); }
  return destination.size;
}
console.log(collect());`,stdout:'second\nfirst\n3\n',sourceVersionRange,singleCallOwners:[]},
 {name:'Map entries destructure exact independent String and Number pair owners',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const destination = new Map([['seed', 0]]);
  for (const [key, value] of source.entries()) destination.set(key, value + 10);
  return destination.get('b') ?? 0;
}
console.log(collect());`,stdout:'12\n',sourceVersionRange,singleCallOwners:[]},
 {name:'Set keys are the original element in insertion order',source:`function collect() {
  const source = new Set([2, 1]);
  const destination = new Set([0]);
  for (const key of source.keys()) { console.log(key); destination.add(key); }
  return destination.size;
}
console.log(collect());`,stdout:'2\n1\n3\n',sourceVersionRange,singleCallOwners:[]},
 {name:'Set entries contain two copies of the same actual element',source:`function collect() {
  const source = new Set([2, 1]);
  const destination = new Set([0]);
  for (const [key, value] of source.entries()) destination.add(key + value);
  return Array.from(destination).join(',');
}
console.log(collect());`,stdout:'0,4,2\n',sourceVersionRange,singleCallOwners:[]},
 {name:'Map entry pair snapshot preserves nested array handle aliases',source:`function collect() {
  const source = new Map([['a', [1, 2]], ['b', [3, 4]]]);
  for (const [key, values] of source.entries()) values.fill(9);
  const first = source.get('a') ?? [0];
  return first.join(',');
}
console.log(collect());`,stdout:'9,9\n',sourceVersionRange,singleCallOwners:[]},
];
export const disjointCollectionItemIterationRefusals=[
 {name:'Map keys alias mutation needs live deleted and appended entries',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const alias = source;
  for (const key of source.keys()) { console.log(key); if (key === 'a') { alias.delete('b'); alias.set('c', 3); } }
}
collect();`,originalStdout:'a\nc\n',sourceVersionRange,reasonIncludes:'disjoint'},
 {name:'Map entries alias mutation changes the later actual pair value',source:`function collect() {
  const source = new Map([['a', 1], ['b', 2]]);
  const alias = source;
  for (const [key, value] of source.entries()) { console.log(key + ':' + value); if (key === 'a') alias.set('b', 9); }
}
collect();`,originalStdout:'a:1\nb:9\n',sourceVersionRange,reasonIncludes:'disjoint'},
 {name:'Set entries alias mutation needs live pair creation',source:`function collect() {
  const source = new Set([1, 2]);
  const alias = source;
  for (const [key, value] of source.entries()) { console.log(key + ',' + value); if (key === 1) { alias.delete(2); alias.add(3); } }
}
collect();`,originalStdout:'1,1\n3,3\n',sourceVersionRange,reasonIncludes:'disjoint'},
 {name:'nonzero argument entries call stays outside exact eager signature',source:`function collect() {
  const source = new Map([['a', 1]]);
  const destination = new Map([['seed', 0]]);
  for (const [key, value] of source.entries(0)) destination.set(key, value);
  return destination.get('a') ?? 0;
}
console.log(collect());`,originalStdout:'1\n',sourceVersionRange},
 {name:'stored keys iterator is not an eager syntactic for-of source',source:`function collect() {
  const source = new Map([['a', 1]]);
  const destination = new Map([['seed', 0]]);
  const iterator = source.keys();
  for (const key of iterator) destination.set(key, 1);
  return destination.get('a') ?? 0;
}
console.log(collect());`,originalStdout:'1\n',sourceVersionRange},
];
