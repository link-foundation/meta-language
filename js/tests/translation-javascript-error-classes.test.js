// Deferred complete-source consumer controls. None ran during source drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createContext,runInContext} from 'node:vm';
import {captureJavaScriptErrorClassModule} from '../src/translation/javascript-error-classes.js';
import {createJavaScriptErrorRealm} from '../src/translation/javascript-error-realm.js';
import {beginJavaScriptErrorModule,advanceJavaScriptErrorModule,resumeJavaScriptErrorModule} from '../src/translation/javascript-error-module.js';
import {resumeJavaScriptCallObjectAllocation} from '../src/translation/javascript-call-machine.js';
import {executeJavaScriptCallPrimitive} from '../src/translation/javascript-call-primitives.js';
import * as rules from '../src/translation/javascript-error-construction-rules.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {readCollectionCopyConcreteControl} from './support/collection-copy-concrete.js';
import {createControlDiagnostics,finishControlDiagnostics,requireControlProduct as product} from './support/control-diagnostics.js';
import {javascriptErrorClassDecisions as decisions,javascriptErrorClassAllocationDecisions as allocations,javascriptErrorClassOriginals as originals,javascriptErrorClassRefusals as refusals,javascriptErrorClassRequiredHostBoundaries as boundaries,translationErrorSource} from './fixtures/javascript-error-classes.mjs';
const profile='actual intrinsic Error subclasses 1';
function versions(){const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node);}
function actual(capture){
 // Evaluate only captured declaration syntax to obtain authentic class values
 // and prototypes. No represented body runs during this bootstrap. The
 // original oracle below invokes the original separately and explicitly.
 const context=createContext({});
 const source=capture.origins.map(row=>capture.source.slice(row.start,row.end)).join('\n');
 runInContext(source,context,{filename:capture.identifier});
 const definitions=runInContext('({'+capture.plan.functions.map(row=>row.name).join(',')+'})',context);
 const realm=runInContext('({errors:{Error,TypeError,RangeError,ReferenceError,SyntaxError,URIError,EvalError},objectPrototype:Object.prototype,toPrimitive:Symbol.toPrimitive,getPrimitive:function(key){"use strict";return this[key];},assign:function(key,value){"use strict";this[key]=value;return value;}})',context);
 return {definitions,realm};
}
function run(embedding,row){
 const identity=embedding.module.capture.plan.functions.find(definition=>definition.name===row.entry).identity;
 let transition=beginJavaScriptErrorModule(embedding.module,embedding.heap(),identity,row.args.map(embedding.encode),{construct:row.construct});
 for(let count=0;count<4096;count++){
  if(transition.$==='machineContinue')transition=advanceJavaScriptErrorModule(embedding.module,transition,{steps:65536,objectIdentities:[embedding.allocateIdentity()]});
  else if(transition.$==='machineAllocateObject')transition=resumeJavaScriptCallObjectAllocation(embedding.module.program,transition.machine,embedding.allocateIdentity());
  else if(['machineAllocateErrorValue','machineInvokeErrorIntrinsic','machineAllocateError','machineAllocateSourceError'].includes(transition.$))transition=resumeJavaScriptErrorModule(embedding.module,transition,embedding.authority);
  else{assert.equal(transition.$,'machineCompleted',transition.reason);embedding.adoptHeap(transition.heap);return embedding.decode(transition.value);}
 }
 assert.fail('complete Error execution exceeded its explicit control budget');
}
function fields(value,expected){
 const result={};for(const key of Object.keys(expected)){
  if(key==='ownMessage')result[key]=Object.hasOwn(value,'message');
  else if(key==='rendered')result[key]=value.toString();
  else if(expected[key]&&typeof expected[key]==='object'){result[key]={};for(const field of Object.keys(expected[key]))result[key][field]=value[key][field];}
  else result[key]=value[key];
 }return result;
}
test('actual diagnostics module source remains byte exact',()=>assert.equal(readFileSync(new URL('../src/translation/diagnostics.js',import.meta.url),'utf8'),translationErrorSource));
test('Error construction source decisions and actual semantic object discriminator',()=>{
 for(const row of [...decisions,...allocations])assert.deepEqual(rules[row.operation](...row.arguments),row.expected,row.operation);
 assert.deepEqual(executeJavaScriptCallPrimitive('test object value',[{$:'object',identity:1n,callable:true}]),{$:'primitiveValue',value:{$:'boolean',value:true}});
 assert.deepEqual(executeJavaScriptCallPrimitive('test object value',[{$:'null'}]),{$:'primitiveValue',value:{$:'boolean',value:false}});
});
for(const row of originals)test(row.name,()=>{
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt;
 attempt('active version coverage',versions);attempt('default CST, exact source bytes and both codecs',()=>readCollectionCopyConcreteControl(row.source));
 const capture=attempt('actual source/CST capture',()=>captureJavaScriptErrorClassModule(row.source,{profile,identifier:row.name+'.js'})),original=attempt('original actual declarations',()=>actual(product(capture,'actual source/CST capture'))),supplied=attempt('independent runtime actual declarations',()=>actual(product(capture,'actual source/CST capture')));
 const expected=attempt('original constructor/helper result',()=>{const declarations=product(original,'original actual declarations').definitions;return row.construct?Reflect.construct(declarations[row.entry],row.args):Reflect.apply(declarations[row.entry],undefined,row.args);});
 attempt('original literal fields',()=>assert.deepEqual(fields(product(expected,'original constructor/helper result'),row.expected),row.expected));
 const embedding=attempt('actual realm and compiled source module',()=>createJavaScriptErrorRealm(product(capture,'actual source/CST capture'),product(supplied,'independent runtime actual declarations'))),value=attempt('portable constructor/helper execution',()=>run(product(embedding,'actual realm and compiled source module'),row));
 attempt('portable literal fields',()=>assert.deepEqual(fields(product(value,'portable constructor/helper execution'),row.expected),row.expected));
 const owner=attempt('actual source constructor owner',()=>product(capture,'actual source/CST capture').plan.functions.find(definition=>definition.$==='errorConstructorPlan'));
 attempt('actual newTarget prototype',()=>assert.equal(Object.getPrototypeOf(product(value,'portable constructor/helper execution')),product(supplied,'independent runtime actual declarations').definitions[product(owner,'actual source constructor owner').name].prototype));
 attempt('actual instanceof brand',()=>assert.ok(product(value,'portable constructor/helper execution') instanceof product(supplied,'independent runtime actual declarations').definitions[product(owner,'actual source constructor owner').name]));
 attempt('actual own message descriptor',()=>assert.equal(Reflect.getOwnPropertyDescriptor(product(value,'portable constructor/helper execution'),'message')?.enumerable??false,false));
 // This is the actual live lazy stack, read only after the source assignments.
 attempt('live lazy stack first line',()=>{const live=product(value,'portable constructor/helper execution');assert.equal(typeof live.stack,'string');assert.equal(live.stack.split('\n')[0],row.expected.rendered);});
 const again=attempt('second call execution',()=>run(product(embedding,'actual realm and compiled source module'),row));attempt('fresh actual receiver identity',()=>assert.notEqual(product(again,'second call execution'),product(value,'portable constructor/helper execution')));
 if(row.args.length<4)attempt('fresh call-time default details',()=>assert.notEqual(product(again,'second call execution').details,product(value,'portable constructor/helper execution').details));
 finishControlDiagnostics({name:row.name,diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}});
});
for(const row of refusals)test(row.name,()=>{versions();readCollectionCopyConcreteControl(row.source);assert.throws(()=>captureJavaScriptErrorClassModule(row.source,{profile,identifier:row.name+'.js'}));});
test('ordinary class profiles retain their separate Error inheritance gate',async()=>{
 const {parseJavaScript}=await import('../src/translation/javascript.js');assert.throws(()=>parseJavaScript(translationErrorSource,{referenceSemantics:true}));
});
test('transport cannot substitute constructor operands before any actual host effect',()=>{
 versions();const row=originals[0],capture=captureJavaScriptErrorClassModule(row.source,{profile,identifier:'request.js'}),embedding=createJavaScriptErrorRealm(capture,actual(capture)),sourceIdentity=capture.plan.functions.find(row=>row.name==='TranslationError').identity;
 let request=beginJavaScriptErrorModule(embedding.module,embedding.heap(),sourceIdentity,row.args.map(embedding.encode),{construct:true});
 for(let step=0;step<256&&request.$!=='machineAllocateErrorValue';step++){
  if(request.$==='machineContinue')request=advanceJavaScriptErrorModule(embedding.module,request,{steps:65536,objectIdentities:[embedding.allocateIdentity()]});
  else if(request.$==='machineAllocateObject')request=resumeJavaScriptCallObjectAllocation(embedding.module.program,request.machine,embedding.allocateIdentity());
  else assert.fail('unexpected transition before super '+request.$);
 }
 assert.equal(request.$,'machineAllocateErrorValue');let calls=0;const authority={perform(){calls++;throw new Error('an invalid request must not execute');}};
 assert.throws(()=>resumeJavaScriptErrorModule(embedding.module,{...request,message:{$:'text',units:[120]}},authority),/cannot substitute/u);assert.equal(calls,0);
});
test('native Error coercion cannot execute an original represented callback body',()=>{
 versions();const row=boundaries.find(row=>row.source),capture=captureJavaScriptErrorClassModule(row.source,{profile,identifier:'callback.js'}),supplied=actual(capture),embedding=createJavaScriptErrorRealm(capture,supplied);
 const name={[Symbol.toPrimitive]:supplied.definitions.forbidden};
 const value=run(embedding,{entry:'CallbackError',construct:true,args:[name]});
 assert.throws(()=>run(embedding,{entry:'render',construct:false,args:[value]}),error=>error instanceof TypeError&&error.message.includes('portable source-call continuation'));
});
test('every Error semantic source kernel keeps codecs and all four target obligations',()=>{
 const diagnostic=createControlDiagnostics(),attempt=diagnostic.attempt;
 for(const name of ['javascript-error-construction-program','javascript-error-class-program','javascript-error-class-source-program']){
  const packet=attempt(name+'/source graph',()=>createSourceDefinitionProgramLinks('meta-language/source-program/'+name));
  const variants=[attempt(name+'/original',()=>product(packet,name+'/source graph').network),attempt(name+'/text',()=>LinkNetwork.fromLino(product(packet,name+'/source graph').network.toLino())),attempt(name+'/binary',()=>decodeLinkNetworkBinary(encodeLinkNetworkBinary(product(packet,name+'/source graph').network)))];
  for(let index=0;index<variants.length;index++)for(const target of ['JavaScript','Rust','Lean','Rocq'])attempt(name+'/'+index+'/'+target,()=>{const measured=product(packet,name+'/source graph');assert.deepEqual(emitTypedProgramLinks(product(variants[index],name+'/'+index+' graph'),measured.root,target),emitTypedProgramLinks(measured.network,measured.root,target));});
 }
 finishControlDiagnostics({name:'Error semantic source kernels',diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}});
});
