// Explicit successor profile. Existing function/class-only refused inputs
// remain in their original fixtures and still exercise their original gates.
import {scopeClassSource} from './closed-class-prototype-methods.mjs';
const versions={node:'>=22.15 <23 || >=24 <25',language:'ECMAScript 2024-2025',rust:'>=1.90 <1.100'};
const reExports='resolved named module providers 1';
const error='named re-export composition needs actual supplied dependencies and resolved aliases';
export const sourceLibraryClassReExportDecisions=[
 {operation:'readSourceLibraryReExportProfileError',arguments:['',false,false],expected:''},
 {operation:'readSourceLibraryReExportProfileError',arguments:[reExports,true,true],expected:''},
 {operation:'readSourceLibraryReExportProfileError',arguments:[reExports,false,true],expected:error},
 {operation:'readSourceLibraryReExportProfileError',arguments:[reExports,true,false],expected:error},
 {operation:'readSourceLibraryReExportProfileError',arguments:['unknown',true,true],expected:error},
 {operation:'acceptSourceLibraryReExportName',arguments:['identifier','Scope'],expected:true},
 {operation:'acceptSourceLibraryReExportName',arguments:['identifier','default'],expected:false},
 {operation:'acceptSourceLibraryReExportName',arguments:['string','Scope'],expected:false},
 {operation:'acceptSourceLibraryEntryExportOwner',arguments:['Scope','Scope'],expected:true},
 {operation:'acceptSourceLibraryEntryExportOwner',arguments:['Scope','privateProvider'],expected:false},
 {operation:'acceptSourceLibraryEntryExportOwner',arguments:['',''],expected:false},
];
export const sourceLibraryClassReExportSourceAnchors=[{path:'js/src/translation/rust-scalar-loops.js',source:"export {acceptRustScalarLoopModel} from './scalar-loop-rules.js';"}];
const box=`export class Box { constructor(value) { this.value = value; } read() { return this.value; } child() { return new Box(this.value); } }`;
export const sourceLibraryClassReExportOriginals=[
 {name:'complete Scope constructor and child self references cross an aliased facade',versions,reExports,entry:'entry.js',arguments:['x'],expected:'x/x_2',files:[
  {path:'scope.js',source:`export ${scopeClassSource}\nfunction renderSourceTargetIdentifier(name) { return name; }`},
  {path:'facade.js',source:`export { Scope as NameScope } from './scope.js';`},
  {path:'entry.js',source:`import { NameScope as Renamer } from './facade.js'; function identity(name) { return name; } export function run(name) { const root = new Renamer(identity); const first = root.fresh(name); return first + '/' + root.child().fresh(name); }`},
 ]},
 {name:'transitive re-export aliases resolve one actual class provider',versions,reExports,entry:'entry.js',arguments:[],expected:'transitive',files:[
  {path:'box.js',source:box},
  {path:'facade.js',source:`export { Box as PublicBox } from './box.js';`},
  {path:'bridge.js',source:`export { PublicBox as ValueBox } from './facade.js';`},
  {path:'entry.js',source:`import { ValueBox } from './bridge.js'; export function run() { return new ValueBox('transitive').child().read(); }`},
 ]},
 {name:'facade public aliases preserve separate same-spelled class declarations',versions,reExports,entry:'entry.js',arguments:[],expected:'left/right',files:[
  {path:'left.js',source:box},{path:'right.js',source:box},
  {path:'facade.js',source:`export { Box as LeftBox } from './left.js'; export { Box as RightBox } from './right.js';`},
  {path:'entry.js',source:`import { LeftBox, RightBox } from './facade.js'; export function run() { return new LeftBox('left').read() + '/' + new RightBox('right').read(); }`},
 ]},
 {name:'the actual scalar-loop named re-export form retains its function provider',versions,reExports,classOwnersExpected:false,entry:'entry.js',arguments:[],expected:'yes',files:[
  {path:'rules.js',source:`export function acceptRustScalarLoopModel(model) {return model === "" || model === "closed primitive scalar loop 1" || model === "closed declaration scalar loop 1";}`},
  {path:'facade.js',source:`export { acceptRustScalarLoopModel } from './rules.js';`},
  {path:'entry.js',source:`import { acceptRustScalarLoopModel as check } from './facade.js'; export function run() { return check('closed primitive scalar loop 1') ? 'yes' : 'no'; }`},
 ]},
 {name:'facade substitution preserves a same-spelled local parameter',versions,reExports,entry:'entry.js',arguments:[],expected:'shadow/value',files:[
  {path:'box.js',source:box},{path:'facade.js',source:`export { Box as ValueBox } from './box.js';`},
  {path:'entry.js',source:`import { ValueBox } from './facade.js'; function label(ValueBox) { return ValueBox; } export function run() { return label('shadow') + '/' + new ValueBox('value').child().read(); }`},
 ]},
];
export const sourceLibraryClassReExportRefusals=[
 {name:'public re-export aliases retain their actual wrapper requirement',versions,reExports,entry:'entry.js',error:'public re-export needs its actual callable or constructor export wrapper',files:[{path:'box.js',source:box},{path:'entry.js',source:`export { Box as AliasBox } from './box.js';`}]},
 {name:'default re-export providers retain their separate source profile',versions,reExports,entry:'entry.js',error:'ordinary provider and public identifiers',files:[{path:'box.js',source:`export default class Box { constructor(value) { this.value = value; } }`},{path:'entry.js',source:`export { default as Box } from './box.js';`}]},
 {name:'star namespace exports do not invent a complete public owner set',versions,reExports,entry:'entry.js',error:'exports need',files:[{path:'box.js',source:box},{path:'entry.js',source:`export * from './box.js';`}]},
 {name:'duplicate re-export public owners cannot select an arbitrary class',versions,reExports,entry:'entry.js',error:'unique actual public owners',files:[{path:'left.js',source:box},{path:'right.js',source:box},{path:'entry.js',source:`export { Box } from './left.js'; export { Box } from './right.js';`}]},
 {name:'missing re-export declarations retain the actual provider failure',versions,reExports,entry:'entry.js',error:'no actual exported owner',files:[{path:'box.js',source:box},{path:'entry.js',source:`export { Missing } from './box.js';`}]},
 {name:'cyclic re-export providers cannot manufacture a declaration owner',versions,reExports,entry:'entry.js',error:'acyclic',files:[{path:'facade.js',source:`export { Box } from './entry.js';`},{path:'entry.js',source:`export { Box } from './facade.js';`}]},
];
