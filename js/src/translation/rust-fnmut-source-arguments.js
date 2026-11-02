// Actual shared Box deref coercion for a selected source callback. Runtime
// values already use the established Box erasure; only its proven source
// position changes. Mutation/moves through that reference remain unavailable.
import {unsupported} from './diagnostics.js';
import {acceptRustOwnedBoxTypeNode} from './rust-owned-recursive-datatype-rules.js';
import {testRustFnMutSourcePosition} from './rust-fnmut-source-positions.js';
export function prepareRustFnMutSourceCallbackArgument(parser,node,expected,path){
 const actual=parser.origins.origin(node,path);if(!actual.wrappers?.includes('box'))return node;
 const fail=reason=>{throw unsupported('Rust source FnMut shared Box argument',reason,node.span);};
 if(parser.fnMutSourceModel!=='Rust closed source FnMut specializations 1'||!parser.fnMutSourceSpecialization?.runtime||expected?.wrappers.length!==1||expected.wrappers[0]!=='reference'||actual.wrappers[0]!=='reference'||actual.wrappers.length>256||actual.wrappers.slice(1).some(layer=>layer!=='box'))fail('callback deref coercion needs one actual shared reference followed only by canonical owned Box layers');
 const project=initial=>{let owner=initial;for(let at=1;at<actual.wrappers.length;at++){if(!owner||!Array.isArray(owner.path)||!Array.isArray(owner.arguments)||!acceptRustOwnedBoxTypeNode(['box'],owner.kind,owner.path.join('::'),owner.arguments.length))fail('every dereferenced source layer needs its complete actual standard Box owner');owner=owner.arguments[0];}return structuredClone(owner);};
 const projected={...structuredClone(actual),wrappers:['reference'],sourceLifetimes:(actual.sourceLifetimes??[]).slice(0,1),sourceOwner:project(actual.sourceOwner),...(actual.expandedSourceOwner?{expandedSourceOwner:project(actual.expandedSourceOwner)}:{}),sourceBoxLayers:[],sourceBorrowedPlace:true};
 if(!testRustFnMutSourcePosition(parser,projected,expected))fail('the projected shared referent retains the selected callback complete type, owner and lifetime');
 parser.origins.argument(actual,expected,node.span);return parser.origins.mark({...node},projected);
}
