// Real engine declaration bootstrap and bounded semantic driver for future
// controls. Defining classes/functions does not execute any represented body.
import assert from 'node:assert/strict';
import {createContext,runInContext} from 'node:vm';
import {URL as NativeURL} from 'node:url';
import {beginJavaScriptErrorSourceClosure} from '../../src/translation/javascript-error-closure.js';
import {runJavaScriptSourceTransition} from '../../src/node-source-modules.js';

export function createActualErrorSourceDeclarations(capture){
 const context=createContext({URL:NativeURL});
 runInContext(capture.origins.map(row=>capture.source.slice(row.start,row.end)).join('\n'),context,{filename:capture.identifier});
 const definitions=runInContext('({'+capture.plan.functions.map(row=>row.name).join(',')+'})',context);
 const realm=runInContext(`({
  errors:{Error,TypeError,RangeError,ReferenceError,SyntaxError,URIError,EvalError},
  objectPrototype:Object.prototype,URL,toPrimitive:Symbol.toPrimitive,
  getPrimitive:function(key){"use strict";return this[key];},
  assign:function(key,value){"use strict";this[key]=value;return value;},
  raiseUninitializedBinding:function(name){
   "use strict";
   if(!/^[A-Za-z_$][A-Za-z_0-9$]*$/.test(name))throw new TypeError("uncovered lexical binding name");
   return Function('"use strict"; return '+name+'; const '+name+'=undefined;')();
  }
 })`,context);
 return {definitions,realm};
}

export function executeErrorSourceClosure(linked,exportName,args,{construct=false,abrupt=false}={}){
 const embedding=linked.embedding;
 const execution=runJavaScriptSourceTransition(embedding,beginJavaScriptErrorSourceClosure(linked,exportName,args.map(embedding.encode),{construct}),{steps:65536}),transition=execution.transition;
 assert.equal(transition.$,abrupt?'machineThrow':'machineCompleted',transition.reason);return {transition,value:embedding.decode(transition.value)};
}
