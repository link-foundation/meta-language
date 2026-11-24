// Complete supplied modules. The selected Scope body is exact; these inputs
// do not claim that all declarations of the original ir.js module translate.
import {scopeClassSource} from './closed-class-prototype-methods.mjs';
const versions={node:'>=22.15 <23 || >=24 <25',language:'ECMAScript 2024-2025',rust:'>=1.90 <1.100'};
const profileError='closed class composition needs its exact supplied dependency, resolved alias and shared reference profiles';
export const sourceLibraryClassSourceAnchors=[{path:'js/src/translation/ir.js',source:'export '+scopeClassSource},{path:'examples/closed-source-class-composition/scope.js',source:'export '+scopeClassSource}];
export const sourceLibraryClassDecisions=[
 {operation:'readSourceLibraryClassProfileError',arguments:['',false,false,false],expected:''},
 {operation:'readSourceLibraryClassProfileError',arguments:['closed local class declarations 1',true,true,true],expected:''},
 {operation:'readSourceLibraryClassProfileError',arguments:['closed local class declarations 1',false,true,true],expected:profileError},
 {operation:'readSourceLibraryClassProfileError',arguments:['closed local class declarations 1',true,false,true],expected:profileError},
 {operation:'readSourceLibraryClassProfileError',arguments:['closed local class declarations 1',true,true,false],expected:profileError},
 {operation:'readSourceLibraryClassProfileError',arguments:['unknown class profile',true,true,true],expected:profileError},
 {operation:'readSourceLibraryClassBindingKind',arguments:['class','identifier','Scope',false],expected:'class'},
 {operation:'readSourceLibraryClassBindingKind',arguments:['const','identifier','run',true],expected:'const'},
 {operation:'readSourceLibraryClassBindingKind',arguments:['class','identifier','class',false],expected:''},
 {operation:'acceptSourceLibraryClassExport',arguments:['class','identifier','Scope',false],expected:true},
 {operation:'acceptSourceLibraryClassExport',arguments:['default','identifier','Scope',false],expected:false},
 {operation:'acceptSourceLibraryClassDeclarationTerm',arguments:['class_declaration','class'],expected:true},
 {operation:'acceptSourceLibraryClassDeclarationTerm',arguments:['class','class'],expected:false},
 {operation:'readSourceLibraryClassDeclarationOwner',arguments:[['Scope'],[10],[90],['Scope'],20,80],expected:'Scope'},
 {operation:'readSourceLibraryClassDeclarationOwner',arguments:[['Scope'],[10],[90],['Other'],20,80],expected:''},
 {operation:'acceptSourceLibraryClassHelperCall',arguments:[['Scope'],'Scope',['actualHelper'],'actualHelper','Scope'],expected:true},
 {operation:'acceptSourceLibraryClassHelperCall',arguments:[[],'Scope',['actualHelper'],'actualHelper','Scope'],expected:false},
 {operation:'acceptSourceLibraryClassHelperCall',arguments:[['Scope'],'Scope',['actualHelper'],'different','Scope'],expected:false},
 {operation:'acceptSourceLibraryClassStaticCall',arguments:[['Scope'],'Scope',['actualStatic'],'actualStatic'],expected:true},
 {operation:'acceptSourceLibraryClassStaticCall',arguments:[[],'Scope',['actualStatic'],'actualStatic'],expected:false},
 {operation:'projectSourceLibraryClassEditInterval',arguments:[[4],[9],[2],7,9],expected:[10,12]},
 {operation:'projectSourceLibraryClassEditInterval',arguments:[[4],[9],[2],4,5],expected:[4,9]},
 {operation:'projectSourceLibraryClassEditInterval',arguments:[[4],[10],[0],4,6],expected:[4,12]},
 {operation:'projectSourceLibraryClassEditInterval',arguments:[[9,4],[10,6],[1,1],1,3],expected:[]},
];
const box=`export class Box { constructor(value) { this.value = value; } read() { return this.value; } child() { return new Box(this.value); } }`;
export const sourceLibraryClassOriginals=[
 {name:'actual Scope import alias retains child self-reference and fresh defaults',versions,entry:'entry.js',arguments:['x'],expected:'x/x_2/x_2',files:[
  {path:'scope.js',source:`import { renderSourceTargetIdentifier } from './identifier.js';\nexport ${scopeClassSource}`},
  {path:'identifier.js',source:`function acceptSourcePrivateIdentifier(name) { return name.startsWith('ml_'); }
export function renderSourceTargetIdentifier(name) {
  return acceptSourcePrivateIdentifier(name) ? 'source_' + name : name;
}`},
  {path:'entry.js',source:`import { Scope as NameScope } from './scope.js';
function identity(name) { return name; }
export function run(name) { const root = new NameScope(identity); const first = root.fresh(name); const child = root.child(); return first + '/' + child.fresh(name) + '/' + root.fresh(name); }`},
 ]},
 {name:'two same-spelled exported classes retain separate constructor owners',versions,entry:'entry.js',arguments:[],expected:'left/right',files:[
  {path:'left.js',source:box},{path:'right.js',source:box},
  {path:'entry.js',source:`import { Box as LeftBox } from './left.js'; import { Box as RightBox } from './right.js'; export function run() { return new LeftBox('left').child().read() + '/' + new RightBox('right').child().read(); }`},
 ]},
 {name:'import substitution leaves a same-spelled parameter in its own scope',versions,entry:'entry.js',arguments:[],expected:'shadow/value',files:[
  {path:'box.js',source:box},
  {path:'entry.js',source:`import { Box as ValueBox } from './box.js'; function label(ValueBox) { return ValueBox; } export function run() { return label('shadow') + '/' + new ValueBox('value').read(); }`},
 ]},
 {name:'actual imported factory exposes its receiver owner without a constructor import',versions,entry:'entry.js',arguments:[],expected:'factory',files:[
  {path:'box.js',source:box+`\nexport function make(value) { return new Box(value); }`},
  {path:'entry.js',source:`import { make as create } from './box.js'; export function run() { return create('factory').child().read(); }`},
 ]},
 {name:'entry class collision preserves dependency class self references',versions,entry:'entry.js',arguments:[],expected:'local/imported',files:[
  {path:'box.js',source:box},
  {path:'entry.js',source:`import { Box as ImportedBox } from './box.js'; class Box { constructor(value) { this.value = value; } read() { return this.value; } child() { return new Box(this.value); } } export function run() { return new Box('local').child().read() + '/' + new ImportedBox('imported').child().read(); }`},
 ]},
 {name:'transitive composed receivers retain field and method ownership',versions,entry:'entry.js',arguments:[],expected:'outer',files:[
  {path:'box.js',source:box},
  {path:'state.js',source:`import { Box as ValueBox } from './box.js'; export class State { constructor(value) { this.box = new ValueBox(value); } read() { return this.box.read(); } child() { return new State(this.box.read()); } }`},
  {path:'entry.js',source:`import { State as BackendState } from './state.js'; export function run() { return new BackendState('outer').child().read(); }`},
 ]},
 {name:'constructor collection defaults allocate separately on each imported call',versions,entry:'entry.js',arguments:[],expected:1,files:[
  {path:'bag.js',source:`export class Bag { constructor(values = new Set()) { this.values = new Set(values); } size() { return this.values.size; } child() { return new Bag(this.values); } }`},
  {path:'entry.js',source:`import { Bag } from './bag.js'; export function run() { const first = new Bag(); first.values.add('x'); const second = new Bag(); return first.size() + second.size(); }`},
 ]},
 {name:'an unrelated private Set function cannot capture another module builtin default',versions,entry:'entry.js',arguments:[],expected:'x',files:[
  {path:'scope.js',source:`import { renderSourceTargetIdentifier } from './identifier.js';\nexport ${scopeClassSource}`},
  {path:'identifier.js',source:`export function renderSourceTargetIdentifier(name) { return name; }`},
  {path:'entry.js',source:`import { Scope as NameScope } from './scope.js'; function Set(value) { return value; } function identity(name) { return name; } export function run() { return new NameScope(identity).fresh('x'); }`},
 ]},
 {name:'entry projection retains only reachable actual class witness rows',versions,entry:'entry.js',arguments:[],expected:'used',retainedClasses:['Box'],files:[
  {path:'box.js',source:box+`\nexport class Unused { constructor() { this.value = 7; } read() { return this.value; } child() { return new Unused(); } }`},
  {path:'entry.js',source:`import { Box } from './box.js'; export function run() { return new Box('used').read(); }`},
 ]},
 {name:'imported static helpers retain their actual class lexical module',versions,entry:'entry.js',arguments:[],expected:'static/x/instance',files:[
  {path:'box.js',source:`function decorate(value) { return 'static/' + value; } export class Box { constructor() { this.value = 'instance'; } read() { return this.value; } child() { return new Box(); } static render(value) { return decorate(value); } }`},
  {path:'entry.js',source:`import { Box as Helpers } from './box.js'; export function run() { return Helpers.render('x') + '/' + new Helpers().read(); }`},
 ]},
];
export const sourceLibraryClassRefusals=[
 {name:'module constants do not acquire an accidental shared environment',versions,entry:'entry.js',error:'unrepresented module state',files:[{path:'entry.js',source:`const seed = 3; export class Box { constructor(value) { this.value = value; } read() { return this.value + seed; } } export function run() { return new Box(1).read(); }`}]},
 {name:'namespace imports retain the exact named-binding boundary',versions,entry:'entry.js',error:'imports need',files:[{path:'box.js',source:box},{path:'entry.js',source:`import * as boxes from './box.js'; export function run() { return new boxes.Box('x').read(); }`}]},
 {name:'default class exports retain their unsupported source boundary',versions,entry:'entry.js',error:'exports need',files:[{path:'entry.js',source:`export default class Box { constructor(value) { this.value = value; } }`}]},
 {name:'class re-exports require complete resolved export ownership',versions,entry:'entry.js',error:'exports need',files:[{path:'box.js',source:box},{path:'entry.js',source:`export { Box } from './box.js';`}]},
 {name:'cyclic class modules retain initialization refusal',versions,entry:'entry.js',error:'acyclic',files:[{path:'box.js',source:`import { run } from './entry.js'; ${box}`},{path:'entry.js',source:`import { Box } from './box.js'; export function run() { return new Box('x').read(); }`}]},
 {name:'unimported class names cannot be captured by concatenation',versions,entry:'entry.js',error:'new Box',previousDraftError:'cannot capture',files:[{path:'box.js',source:box+` export function scalar() { return 1; }`},{path:'entry.js',source:`import { scalar } from './box.js'; export function run() { return new Box('x').read(); }`}]},
 {name:'static initialization cannot be reordered into a declaration-only class closure',versions,entry:'entry.js',error:'static initialization',files:[{path:'entry.js',source:`export class Box { static value = 3; } export function run() { return Box.value; }`}]},
 {name:'inherited class evaluation retains its existing closed descriptor boundary',versions,entry:'entry.js',error:'ordinary named base-class',files:[{path:'entry.js',source:`class Base {} export class Box extends Base {} export function run() { return new Box(); }`}]},
 {name:'public entry spelling cannot silently change to protect a foreign builtin',versions,entry:'entry.js',error:'public entry binding',files:[{path:'bag.js',source:`export class Bag { constructor(values = new Set()) { this.values = new Set(values); } size() { return this.values.size; } child() { return new Bag(this.values); } }`},{path:'entry.js',source:`import { Bag } from './bag.js'; export function Set(value) { return value; } export function run() { return new Bag().size(); }`}]},
];
