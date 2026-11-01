// Real engine declaration bootstrap and bounded semantic driver for future
// controls. Defining classes/functions does not execute any represented body.
import assert from 'node:assert/strict';
import {createContext,runInContext} from 'node:vm';
import {beginJavaScriptErrorSourceClosure,advanceJavaScriptErrorSourceClosure,resumeJavaScriptErrorSourceClosure} from '../../src/translation/javascript-error-closure.js';
import {resumeJavaScriptCallObjectAllocation} from '../../src/translation/javascript-call-machine.js';

export function createActualErrorSourceDeclarations(capture){
 const context=createContext({});
 runInContext(capture.origins.map(row=>capture.source.slice(row.start,row.end)).join('\n'),context,{filename:capture.identifier});
 const definitions=runInContext('({'+capture.plan.functions.map(row=>row.name).join(',')+'})',context);
 const realm=runInContext('({errors:{Error,TypeError,RangeError,ReferenceError,SyntaxError,URIError,EvalError},objectPrototype:Object.prototype,toPrimitive:Symbol.toPrimitive,getPrimitive:function(key){"use strict";return this[key];},assign:function(key,value){"use strict";this[key]=value;return value;}})',context);
 return {definitions,realm};
}

export function executeErrorSourceClosure(linked,exportName,args,{construct=false,abrupt=false}={}){
 const embedding=linked.embedding;
 let transition=beginJavaScriptErrorSourceClosure(linked,exportName,args.map(embedding.encode),{construct});
 for(let step=0;step<4096;step++){
  if(transition.$==='machineContinue')transition=advanceJavaScriptErrorSourceClosure(linked,transition,{steps:65536,objectIdentities:[embedding.allocateIdentity()]});
  else if(transition.$==='machineAllocateObject')transition=resumeJavaScriptCallObjectAllocation(embedding.module.program,transition.machine,embedding.allocateIdentity());
  else if(['machineAllocateErrorValue','machineInvokeErrorIntrinsic','machineAllocateError','machineAllocateSourceError'].includes(transition.$))transition=resumeJavaScriptErrorSourceClosure(linked,transition);
  else{assert.equal(transition.$,abrupt?'machineThrow':'machineCompleted',transition.reason);embedding.adoptHeap(transition.heap);return {transition,value:embedding.decode(transition.value)};}
 }
 assert.fail('complete imported Error source execution exceeded its explicit control budget');
}
