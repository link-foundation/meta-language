// Independent complete originals and archive edits. These are future controls,
// never observations from authoring this source batch.
const sourceVersionRange='ECMAScript 2024-2025; Node >=22.15 <23 || >=24 <25';
export const closedClassOwnerDecisions=[
 {operation:'acceptJavaScriptClassOwnerObservation',arguments:['closure slots','identity'],expected:true},
 {operation:'acceptJavaScriptClassOwnerObservation',arguments:['closure slots','structural'],expected:false},
 {operation:'acceptJavaScriptClassOwnerObservation',arguments:['prototype helpers','identity'],expected:false},
 {operation:'acceptJavaScriptClassOwnerObservation',arguments:['prototype helpers','structural'],expected:false},
 {operation:'acceptJavaScriptClassOwnerObservation',arguments:['unknown','identity'],expected:false},
 {operation:'readJavaScriptClassOwnerInference',arguments:['Box','',false,true],expected:'right'},
 {operation:'readJavaScriptClassOwnerInference',arguments:['','Box',true,false],expected:'left'},
 {operation:'readJavaScriptClassOwnerInference',arguments:['Box','Other',false,false],expected:'error'},
 {operation:'readJavaScriptClassRecordOwner',arguments:[['Box','Other'],['brand0','brand1'],['brand0','value']],expected:'Box'},
 {operation:'readJavaScriptClassRecordOwner',arguments:[['Box','Other'],['brand0','brand1'],['brand0','brand1']],expected:'#ambiguous'},
];
export const closedClassOwnerOriginals=[
 {name:'legacy closure-slot alias identity remains admitted',source:`class Box { constructor(value) { this.value = value; } read() { return this.value; } }
const object = new Box(3); const alias = object; console.log(object === alias); console.log(alias.read());`,stdout:'true\n3\n',sourceVersionRange,singleCallOwners:[],prototypeMethodsExpected:false},
 {name:'legacy closure-slot fresh instances remain distinct',source:`class Box { constructor(value) { this.value = value; } read() { return this.value; } }
const first = new Box(3); const second = new Box(3); console.log(first === second);`,stdout:'false\n',sourceVersionRange,singleCallOwners:[],prototypeMethodsExpected:false},
 {name:'legacy empty class keeps reference identity',source:`class Empty {} const first = new Empty(); const alias = first; const second = new Empty(); console.log(first === alias); console.log(first === second);`,stdout:'true\nfalse\n',sourceVersionRange,singleCallOwners:[],prototypeMethodsExpected:false},
 {name:'prototype data owners survive nested results and archives',source:`class Box {
 constructor(value) { this.value = value; }
 read() { return this.value; }
 child() { return new Box(this.value); }
}
function compute() { const box = new Box(1); return box.read(); }
function plain() { return { value: 4 }; }
function wrapper() { return { nested: new Box(2) }; }
function nested() { return wrapper(); }
console.log(compute());`,stdout:'1\n',sourceVersionRange,singleCallOwners:[],prototypeMethodsExpected:true},
];
// Each name is consumed by the independent mutation adapter in both host
// matrices. No malformed input disappears when serde rejects it earlier.
export const closedClassOwnerArchiveEdits=[
 'dropped witness','null witness','empty witness','wrong model','wrong version',
 'wrong constructor reference','wrong helper reference','wrong brand',
 'empty owner','null owner','numeric owner','erased owner','retagged plain record',
 'direct debug observation','nested debug observation','prototype identity observation',
];
export const closedClassOwnerGraphEdits=['restored direct debug observation','restored nested debug observation','retargeted constructor edge'];
