// Shared closed source-domain selections; hosts provide actual operands/types.
import {readArrayCallbackResultForm,readArrayCallbackParameterRole} from './array-callback-rules.js';
/** @param {string} name @returns {string} */
export function readJavaScriptOwnPropertyBuiltinOperation(name){return name==='Object.hasOwn'?'testJavaScriptOwnedProperty':'';}
/** @param {string} kind @param {string} element @returns {string} */
export function readJavaScriptOwnPropertyReceiverForm(kind,element){
 if(kind==='unit'||kind==='option'&&element==='unit')return 'absent';
 if(kind==='option')return 'guarded';
 if(kind==='record')return 'record';
 if(kind==='array'||kind==='tuple'||kind==='string')return 'indexed';
 if(kind==='bool'||kind==='float'||kind==='int'||kind==='nat'||kind==='map'||kind==='set')return 'empty';
 return 'refuse';
}
/** @returns {string} */
export function readJavaScriptObjectCoercionError(){return 'Cannot convert undefined or null to object';}
/** @returns {string} */
export function readJavaScriptArrayPropertyIndexLimit(){return '4294967295';}
/** One actual source hasOwn condition selects only its true branch; an actual
 * unary negation selects the other branch. @param {string} kind
 * @param {string} operation @param {boolean} negated @returns {number} */
export function readJavaScriptOwnPropertyGuardArm(kind,operation,negated){return kind==='math'&&operation==='testJavaScriptOwnedProperty'?(negated?-1:1):0;}
/** Facts belong to immutable functional binders with closed record shapes.
 * Mutable source cells and effectful/missing/extra operands supply no fact.
 * @param {number} count @param {boolean} spread @param {boolean} localObject
 * @param {boolean} localKey @param {string} object @param {string} key
 * @param {boolean} open @returns {boolean} */
export function acceptJavaScriptOwnPropertyGuardBindings(count,spread,localObject,localKey,object,key,open){return count===2&&!spread&&localObject&&localKey&&object==='record'&&key==='string'&&!open;}
/** @param {string} object @param {boolean} open @param {string} key
 * @param {boolean} guarded @returns {boolean} */
export function acceptJavaScriptGuardedRecordIndex(object,open,key,guarded){return object==='record'&&!open&&key==='string'&&guarded;}
/** Exact canonical positions for host adaptation of actual named projections.
 * @param {number} count @returns {number[]} */
export function readJavaScriptGuardedRecordProjectionPositions(count){
 if(!Number.isSafeInteger(count)||count<0||count>100000)throw new RangeError('invalid actual guarded record field count');
 let result=[];for(let index=count-1;index>=0;index-=1)result=result.concat([index]);return result;
}
/** Use the existing complete callback role definitions, not an independent
 * native parameter convention. @param {string} operation @returns {number} */
export function readJavaScriptOwnKeyCallbackPosition(operation){
 if(readArrayCallbackResultForm(operation)==='')return -1;
 for(let position=0;position<4;position+=1)if(readArrayCallbackParameterRole(operation,position)==='element')return position;
 return -1;
}
/** @param {string} operation @returns {number} */
export function readJavaScriptOwnKeyCallbackReceiverPosition(operation){
 if(readArrayCallbackResultForm(operation)==='')return -1;
 for(let position=0;position<4;position+=1)if(readArrayCallbackParameterRole(operation,position)==='receiver')return position;
 return -1;
}
/** Only the immediate actual own-key producer supplies callback membership.
 * @param {string} expression @param {string} operation @param {number} count
 * @param {boolean} spread @param {boolean} local @param {string} owner
 * @param {boolean} open @param {string} callback @returns {boolean} */
export function acceptJavaScriptOwnKeyCallbackSource(expression,operation,count,spread,local,owner,open,callback){return expression==='math'&&operation==='enumerateJavaScriptObjectKeys'&&count===1&&!spread&&local&&owner==='record'&&!open&&callback==='lambda';}
/** Exposing the live key array can replace keys before later visits.
 * @param {boolean} ownerShadowed @param {boolean} receiverExposed
 * @param {string} key @returns {boolean} */
export function acceptJavaScriptOwnKeyCallbackBinding(ownerShadowed,receiverExposed,key){return !ownerShadowed&&!receiverExposed&&key==='string';}
