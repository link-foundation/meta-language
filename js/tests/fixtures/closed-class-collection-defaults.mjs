// Actual defaults stay source expressions and run on invocation, not during
// method environment installation. Expectations are independent literals.
const sourceVersionRange='ECMAScript 2024-2025; Node >=22.15 <23 || >=24 <25';
export const closedClassCollectionDefaultDecisions=[
 {operation:'readJavaScriptClosedClassParameterPositions',arguments:[['(','values','=','new','Set','(',')',')'],['punct','identifier','punct','identifier','identifier','punct','punct','punct'],0,7],expected:[-1,3]},
 {operation:'readJavaScriptClosedClassCollectionParameterPositions',arguments:[['(','values','=','new','Set','(',')',')'],['punct','identifier','punct','identifier','identifier','punct','punct','punct'],0,7],expected:[1]},
 {operation:'readJavaScriptClosedClassCollectionParameterPositions',arguments:[['(','values','=','new','Map','(',')',')'],['punct','identifier','punct','identifier','identifier','punct','punct','punct'],0,7],expected:[1]},
 {operation:'readJavaScriptClosedClassCollectionParameterPositions',arguments:[['(','values','=','new','Set','(','items',')',')'],['punct','identifier','punct','identifier','identifier','punct','identifier','punct','punct'],0,8],expected:[-1,3]},
 {operation:'acceptJavaScriptClosedClassOwnField',arguments:[['identity0','ident'],['identity0','ident'],'ident'],expected:true},
 {operation:'acceptJavaScriptClosedClassOwnField',arguments:[['identity1','ident'],['identity0','ident'],'ident'],expected:false},
 {operation:'acceptJavaScriptClosedClassOwnField',arguments:[['identity0','fresh'],['identity0','ident'],'fresh'],expected:false},
 {operation:'readJavaScriptClosedClassOwnFieldUseError',arguments:['recordField','',true,false,true,false,true],expected:''},
 {operation:'readJavaScriptClosedClassUseError',arguments:['recordField','',true,false,true,false],expected:'closed class instances require direct public field operations and receiver-retaining method calls; private slots, detached callables and indirect reflective lookups need the full object model'},
];
export const closedClassCollectionDefaultOriginals=[
 {name:'fresh empty Set constructor defaults are not shared',source:`class Bag {
  constructor(values = new Set()) { this.values = values; }
  add(value) { this.values.add(value); return this.values.size; }
  has(value) { return this.values.has(value); }
}
const first = new Bag(); const second = new Bag();
console.log(first.add('x')); console.log(second.has('x'));`,stdout:'1\nfalse\n',sourceVersionRange,singleCallOwners:[]},
 {name:'fresh empty Map constructor defaults retain actual key value owners',source:`class Table {
  constructor(values = new Map()) { this.values = values; }
  put(key, value) { this.values.set(key, value); return this.values.size; }
  get(key) { return this.values.get(key) ?? 0; }
}
const first = new Table(); const second = new Table();
console.log(first.put('x', 7)); console.log(second.get('x'));`,stdout:'1\n0\n',sourceVersionRange,singleCallOwners:[]},
 {name:'explicit owned collection suppresses default and preserves caller alias',source:`class Bag {
  constructor(values = new Set()) { this.values = values; }
  add(value) { this.values.add(value); return this.values.size; }
}
const original = new Set(['reserved']); const bag = new Bag(original);
console.log(bag.add('x')); console.log(original.has('x'));`,stdout:'2\ntrue\n',sourceVersionRange,singleCallOwners:[]},
 {name:'method collection defaults allocate on every invocation',source:`class Probe {
  constructor() { this.value = 0; }
  count(values = new Set()) { values.add('x'); return values.size; }
}
const probe = new Probe(); console.log(probe.count()); console.log(probe.count());`,stdout:'1\n1\n',sourceVersionRange,singleCallOwners:[]},
 {name:'ordinary stored callable field retains original unbound function value',source:`class Scope {
  constructor(ident, reserved = new Set()) { this.ident = ident; this.used = new Set(reserved); }
  fresh(name) { const candidate = this.ident(name); this.used.add(candidate); return candidate; }
  forward(name) { const ident = this.ident; return ident(name); }
}
function identity(name) { return name; }
const scope = new Scope(identity); console.log(scope.fresh('x')); console.log(scope.forward('y'));`,stdout:'x\ny\n',sourceVersionRange,singleCallOwners:[]},
];
export const closedClassCollectionDefaultRefusals=[
 {name:'earlier parameter Set cannot become standard collection owner',source:`class Bag {
 constructor(Set, values = new Set()) { this.values = values; }
 count() { return this.values.length; }
}
function custom() { return [7]; }
console.log(new Bag(custom).count());`,originalStdout:'1\n',sourceVersionRange},
 {name:'earlier parameter Map cannot become standard collection owner',source:`class Bag {
 constructor(Map, values = new Map()) { this.values = values; }
 count() { return this.values.length; }
}
function custom() { return [7]; }
console.log(new Bag(custom).count());`,originalStdout:'1\n',sourceVersionRange},
 {name:'module function Set is not standard Set in a default',source:`class Bag {
 constructor(values = new Set()) { this.values = values; }
 count() { return this.values.length; }
}
function Set() { return [7]; }
console.log(new Bag().count());`,originalStdout:'1\n',sourceVersionRange},
 {name:'local class Set keeps its actual owner before source rewriting',source:`class Set { constructor() { this.value = 7; } }
class Bag {
 constructor(values = new Set()) { this.values = values; }
 read() { return this.values.value; }
}
console.log(new Bag().read());`,originalStdout:'7\n',sourceVersionRange,reasonIncludes:'actual local class constructor'},
 {name:'detached prototype method is not an ordinary stored callable field',source:`class Scope {
 constructor(ident, reserved = new Set()) { this.ident = ident; this.used = new Set(reserved); }
 fresh(name) { this.used.add(name); return name; }
}
function identity(name) { return name; }
const scope = new Scope(identity); const fresh = scope.fresh;`,originalStdout:'',sourceVersionRange},
];
