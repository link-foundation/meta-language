// Complete selected own Scope owner plus exact helper bodies. Import linking
// remains a separate source obligation; no inheritance anchor is invented.
const sourceVersionRange='ECMAScript 2024-2025; Node >=22.15 <23 || >=24 <25';
export const scopeClassSource=`class Scope {
  constructor(ident, reserved = new Set()) {
    this.ident = ident;
    this.used = new Set(reserved);
  }

  fresh(name) {
    name = renderSourceTargetIdentifier(name);
    // A suffixed name is legal on its own: \`end_2\` needs none of the quoting of Lean's \`«end»\`.
    let candidate = renderSourceTargetIdentifier(this.ident(name));
    for (let index = 2; this.used.has(candidate); index += 1) candidate = renderSourceTargetIdentifier(this.ident(\`\${name}_\${index}\`));
    this.used.add(candidate);
    return candidate;
  }

  child() {
    const scope = new Scope(this.ident);
    scope.used = new Set(this.used);
    return scope;
  }
}`;
const identifierHelpers=`function acceptSourcePrivateIdentifier(name) { return name.startsWith('ml_'); }
function renderSourceTargetIdentifier(name) {
  return acceptSourcePrivateIdentifier(name) ? 'source_' + name : name;
}
function identity(name) { return name; }`;
export const closedClassPrototypeSourceAnchors=[{path:'js/src/translation/ir.js',source:'export '+scopeClassSource},{path:'js/src/translation/source-identifier-rules.js',source:`export function renderSourceTargetIdentifier(name) {
  return acceptSourcePrivateIdentifier(name) ? 'source_' + name : name;
}`}];
export const closedClassPrototypeDecisions=[
 {operation:'readJavaScriptClosedClassMethodTarget',arguments:[['brand0','fresh','helper0','brand1','fresh','helper1'],['brand0','used'],'fresh'],expected:'helper0'},
 {operation:'readJavaScriptClosedClassMethodTarget',arguments:[['brand0','fresh','helper0','brand1','fresh','helper1'],['brand1','used'],'fresh'],expected:'helper1'},
 {operation:'readJavaScriptClosedClassMethodTarget',arguments:[['brand0','fresh','helper0'],['different','used'],'fresh'],expected:''},
 {operation:'readJavaScriptClosedClassMethodTarget',arguments:[['brand0','fresh','helper0'],['brand0','used'],'detached'],expected:''},
 {operation:'readJavaScriptClosedClassMethodTarget',arguments:[['brand0','fresh','helper0','brand0','fresh','helper1'],['brand0'],'fresh'],expected:''},
 {operation:'acceptJavaScriptClosedClassMethodDefinition',arguments:['ml_class_identity_0','fresh','ml_class_method_0_9','ml_class_receiver_0',['ml_class_identity_0','used'],true],expected:true},
 {operation:'acceptJavaScriptClosedClassMethodDefinition',arguments:['ml_class_identity_0','fresh','ml_class_method_1_9','ml_class_receiver_0',['ml_class_identity_0','used'],true],expected:false},
 {operation:'acceptJavaScriptClosedClassMethodDefinition',arguments:['ml_class_identity_0','fresh','ml_class_method_0_9','ml_class_receiver_0',['used'],true],expected:false},
 {operation:'acceptJavaScriptClosedClassMethodDefinition',arguments:['ml_class_identity_0','fresh','ml_class_method_0_9','ml_class_receiver_0',['ml_class_identity_0','used'],false],expected:false},
];
export const closedClassPrototypeObservationDecisions=[
 {operation:'readJavaScriptClosedClassObservationOperands',arguments:['toString',''],expected:'arg'},
 {operation:'readJavaScriptClosedClassObservationOperands',arguments:['strictEqual',''],expected:'left right'},
 {operation:'readJavaScriptClosedClassObservationOperands',arguments:['','eq'],expected:'left right'},
 {operation:'readJavaScriptClosedClassObservationOperands',arguments:['','forall'],expected:'binders'},
 {operation:'readJavaScriptClosedClassObservationOperands',arguments:['recordField',''],expected:''},
];
export const closedClassPrototypeOriginals=[
 {name:'complete actual Scope child owner has finite data and independent Set copies',source:scopeClassSource+'\n'+identifierHelpers+`
const root = new Scope(identity);
console.log(root.fresh('x'));
const child = root.child();
console.log(child.fresh('x'));
console.log(root.fresh('x'));
console.log(child.fresh('x'));
console.log(root.fresh('ml_private'));`,stdout:'x\nx_2\nx_2\nx_3\nsource_ml_private\n',sourceVersionRange,singleCallOwners:[],prototypeMethodsExpected:true},
 {name:'actual composition shape retains shared Scope receiver through backend and state',source:scopeClassSource+`
class EmitterState {
 constructor(scope) { this.scope = scope; }
 fresh(name) { return this.scope.fresh(name); }
 child() { return new EmitterState(this.scope.child()); }
}
class RustEmitter {
 constructor(state) { this.state = state; }
 fresh(name) { return this.state.fresh(name); }
 child() { return new RustEmitter(this.state.child()); }
}
`+identifierHelpers+`
const root = new RustEmitter(new EmitterState(new Scope(identity)));
console.log(root.fresh('x')); const child = root.child();
console.log(child.fresh('x')); console.log(root.fresh('x'));`,stdout:'x\nx_2\nx_2\n',sourceVersionRange,singleCallOwners:[],prototypeMethodsExpected:true},
 {name:'receiver constructor arguments and method effects run once in source order',source:`class Counter {
 constructor(value) { this.value = value; console.log('construct'); }
 read(extra) { console.log('method'); return this.value + extra; }
 child() { return new Counter(this.value); }
}
function receiver() { console.log('receiver'); return new Counter(2); }
function argument() { console.log('argument'); return 3; }
function compute() { return receiver().read(argument()); }
console.log(compute());`,stdout:'receiver\nconstruct\nargument\nmethod\n5\n',sourceVersionRange,singleCallFunction:'compute',singleCallOwners:['receiver','argument'],prototypeMethodsExpected:true},
 {name:'module helper lexical environment does not capture constructor parameter shadow',source:`class Counter {
 constructor(seed) { this.value = seed; }
 read() { return seed(); }
 child() { return new Counter(this.value); }
}
function seed() { return 7; }
console.log(new Counter(3).read());`,stdout:'7\n',sourceVersionRange,singleCallOwners:[],prototypeMethodsExpected:true},
 {name:'child instance data preserves nested array alias while outer receiver stays distinct',source:`class Box {
 constructor(items) { this.items = items; }
 child() { return new Box(this.items); }
 add(value) { this.items.push(value); return this.items.length; }
}
const root = new Box([1]); const child = root.child();
console.log(child.add(2)); console.log(root.items.join(','));`,stdout:'2\n1,2\n',sourceVersionRange,singleCallOwners:[],prototypeMethodsExpected:true},
];
export const closedClassPrototypeRefusals=[
 {name:'prototype helper receiver is not a detached callable value',source:`class Counter { constructor(value) { this.value = value; } read() { return this.value; } child() { return new Counter(this.value); } }
const root = new Counter(1); const read = root.read;`,originalStdout:'',sourceVersionRange},
 {name:'prototype method replacement is not a public data-field write',source:`class Counter { constructor(value) { this.value = value; } read() { return this.value; } child() { return new Counter(this.value); } }
const root = new Counter(1); root.read = () => 9; console.log(root.read());`,originalStdout:'9\n',sourceVersionRange},
 {name:'data-only class shape cannot expose its hidden brand through String',source:`class Counter { constructor(value) { this.value = value; } read() { return this.value; } child() { return new Counter(this.value); } }
const root = new Counter(1); console.log(String(root));`,originalStdout:'[object Object]\n',sourceVersionRange},
 {name:'nested class value cannot become ordinary structural print data',source:`class Counter { constructor(value) { this.value = value; } read() { return this.value; } child() { return new Counter(this.value); } }
const root = new Counter(1); console.log(String({ nested: root }));`,originalStdout:'[object Object]\n',sourceVersionRange},
 {name:'class aggregate identity observation keeps its explicit model boundary',source:`class Counter { constructor(value) { this.value = value; } read() { return this.value; } child() { return new Counter(this.value); } }
const root = new Counter(1); console.log(root === root);`,originalStdout:'true\n',sourceVersionRange},
];
