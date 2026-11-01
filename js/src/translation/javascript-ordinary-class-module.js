// Actual closed-class declarations bootstrap function/prototype identity only.
// No class constructor, method or helper body runs in this host adapter.
import {compileFunction} from 'node:vm';
import {checkJavaScriptOrdinaryClassModule} from './javascript-ordinary-class-source.js';
import {createJavaScriptErrorRealm} from './javascript-error-realm.js';
import {beginJavaScriptErrorModule} from './javascript-error-module.js';
import {linkJavaScriptCollectionRealm} from './javascript-collection-realm.js';

export function createJavaScriptOrdinaryClassDeclarations(input){
 const capture=checkJavaScriptOrdinaryClassModule(input),definitions=Object.create(null),classes=new Map();
 function create(origin){const source=capture.source.slice(origin.start,origin.end),value=compileFunction('"use strict";return ('+source+'\n);',[])();if(Function.prototype.toString.call(value)!==source)throw new TypeError('a source declaration must retain its complete original text');return value;}
 for(const row of capture.classes){const origin=capture.origins.find(origin=>origin.identity===row.binding&&origin.constructor===true);if(!origin)throw new TypeError('an actual class declaration requires its captured constructor owner');const value=create(origin);if(Reflect.getPrototypeOf(value)!==Function.prototype||Reflect.getPrototypeOf(value.prototype)!==Object.prototype)throw new TypeError('the ordinary source class profile requires its actual base constructor and prototype');classes.set(row.binding,value);Object.defineProperty(definitions,String(row.binding),{value,enumerable:true});}
 for(const origin of capture.origins){
  if(origin.constructor===true)continue;let value;
  if(origin.classBinding!==undefined){const owner=classes.get(origin.classBinding),descriptor=owner&&Reflect.getOwnPropertyDescriptor(owner.prototype,origin.name);if(!descriptor||!Object.hasOwn(descriptor,'value')||typeof descriptor.value!=='function'||descriptor.enumerable||!descriptor.configurable||!descriptor.writable)throw new TypeError('an ordinary instance method requires its actual standard own prototype descriptor');value=descriptor.value;if(Function.prototype.toString.call(value)!==capture.source.slice(origin.start,origin.end))throw new TypeError('an ordinary method must retain its actual complete source member');}
  else value=create(origin);
  Object.defineProperty(definitions,String(origin.identity),{value,enumerable:true});
 }
 const realm={objectPrototype:Object.prototype,toPrimitive:Symbol.toPrimitive,errors:{Error,TypeError,RangeError,ReferenceError,SyntaxError,URIError,EvalError},getPrimitive:function sourceClassPrimitiveGet(key){'use strict';return this[key];},assign:function sourceClassAssign(key,value){'use strict';this[key]=value;return value;}};
 return {capture,definitions,realm};
}

/** Pure class modules publish only when every actual imported/native source
 * operation has an executable implementation. Collection-bearing modules use
 * the same bootstrap plus the separately supplied shared collection linker;
 * they must not fall back to original host Set/Map or class bodies. */
export function prepareJavaScriptOrdinaryClassModule(input,{realmIdentity,declarationWitness=null}={}){
 if(typeof realmIdentity!=='bigint'||realmIdentity<=0n||realmIdentity>18446744073709551615n)throw new TypeError('ordinary source classes require an explicit actual realm identity');
 const authority=createJavaScriptOrdinaryClassDeclarations(input),capture=authority.capture;
 if(capture.cells.length)throw new TypeError('this ordinary class module needs its actual cell initialization before publication');
 const embedding=createJavaScriptErrorRealm(capture,{...authority,identity:realmIdentity,deferModule:true,declarationWitness}),collection=linkJavaScriptCollectionRealm(embedding,capture.capabilities,{realmIdentity});embedding.finishSourceCapabilities(collection.references,[]);
 return {capture,embedding,collection,
  callable(name){const definitions=capture.plan.functions.filter(row=>row.name===name&&capture.plan.exports.includes(row.identity));if(definitions.length!==1)throw new TypeError('a source callable value requires its actual public declaration');const owner=embedding.module.registry.owners.find(row=>row.source===definitions[0].identity);return {$:'object',identity:owner.identity,callable:true};},
  begin(name,args){collection.verify();const definitions=capture.plan.functions.filter(row=>row.name===name&&capture.plan.exports.includes(row.identity)&&(row.$==='errorStatementFunctionPlan'||row.$==='errorMutableStatementFunctionPlan'));if(definitions.length!==1)throw new TypeError('entry calls require one actual ordinary public function; construct a class through its actual source new expression');return beginJavaScriptErrorModule(embedding.module,embedding.heap(),definitions[0].identity,args);}
 };
}
