// Protected join uses ordinary Gets and shared coercion; no host Array body runs.
import {appendJavaScriptObjectCoercion} from './javascript-object-coercion-call-source.js';
/** @param {string} operation @returns {boolean} */
export function testJavaScriptArrayJoinOperation(operation){return operation==='Array.join';}
/**
 * Actual realm roots are required for ToObject, including String own UTF16 slots.
 * arrayJoinEnter/Leave are frame-owned VM instructions: all actual Objects enter
 * the active receiver directory under the selected V8 compatibility profile.
 * Unwinding releases this frame's token, including conversion/getter throws.
 * @param {bigint} identity @param {bigint} realm
 * @param {import('./javascript-object-call-source.js').JavaScriptPrimitiveWrapperRoots|undefined} roots
 * @returns {JavaScriptCallFunction|undefined}
 */
export function compileJavaScriptArrayJoinFunction(identity,realm,roots){
 // 0=original receiver, 1=boxed Object, 2=length, 3=separator,
 // 4=index, 5=result, 6=element, 7=key/type, 8=condition,
 // 9=zero, 10=one, 11=undefined, 12=null, 13=prototype, 14=guard/key.
 /** @type {JavaScriptCallInstruction[]} */const code=[];
 if(!appendJavaScriptObjectCoercion(code,0,1,7,14,8,13,roots))return undefined;
 code.push({$:'constant',destination:7,value:{$:'text',units:[108,101,110,103,116,104]}},{$:'ordinaryGet',destination:2,object:1,key:7,receiver:1},{$:'primitive',destination:2,operation:'coerce primitive number',arguments:[2]},{$:'primitive',destination:2,operation:'read array like length',arguments:[2]},{$:'primitive',destination:2,operation:'require V8 array join length',arguments:[2]},{$:'argument',destination:3,index:0},{$:'constant',destination:11,value:{$:'undefined'}},{$:'strictEqual',destination:8,left:3,right:11});
 const separatorBranch=code.length;code.push({$:'branch',condition:8,yes:0,no:separatorBranch+1},{$:'primitive',destination:3,operation:'convert primitive text',arguments:[3]});
 const separatorDone=code.length;code.push({$:'jump',target:0});
 const comma=code.length;code.push({$:'constant',destination:3,value:{$:'text',units:[44]}});
 const ready=code.length;code[separatorBranch]={$:'branch',condition:8,yes:comma,no:separatorBranch+1};code[separatorDone]={$:'jump',target:ready};
 code.push({$:'constant',destination:5,value:{$:'text',units:[]}},{$:'constant',destination:9,value:{$:'number',value:0}},{$:'constant',destination:10,value:{$:'number',value:1}},{$:'constant',destination:12,value:{$:'null'}},{$:'copy',destination:4,source:9},{$:'numberBinary',destination:8,left:2,right:9,operator:'equal'});
 const empty=code.length;code.push({$:'branch',condition:8,yes:0,no:empty+1},{$:'arrayJoinEnter',destination:14,receiver:1});
 const guard=code.length;code.push({$:'branch',condition:14,yes:guard+1,no:0});
 const loop=code.length;code.push({$:'numberBinary',destination:8,left:4,right:2,operator:'lessThan'});
 const exhausted=code.length;code.push({$:'branch',condition:8,yes:exhausted+1,no:0},{$:'numberBinary',destination:8,left:4,right:9,operator:'equal'});
 const first=code.length;code.push({$:'branch',condition:8,yes:first+2,no:first+1},{$:'primitive',destination:5,operation:'concatenate text',arguments:[5,3]},{$:'propertyKey',destination:7,input:4,realm},{$:'ordinaryGet',destination:6,object:1,key:7,receiver:1},{$:'strictEqual',destination:8,left:6,right:11});
 const undefinedElement=code.length;code.push({$:'branch',condition:8,yes:0,no:undefinedElement+1},{$:'strictEqual',destination:8,left:6,right:12});
 const nullElement=code.length;code.push({$:'branch',condition:8,yes:0,no:nullElement+1},{$:'primitive',destination:6,operation:'convert primitive text',arguments:[6]},{$:'primitive',destination:5,operation:'concatenate text',arguments:[5,6]});
 const advance=code.length;code.push({$:'numberBinary',destination:4,left:4,right:10,operator:'add'},{$:'jump',target:loop});
 const leave=code.length;code.push({$:'arrayJoinLeave',receiver:1});
 const finish=code.length;code.push({$:'return',source:5});
 code[empty]={$:'branch',condition:8,yes:finish,no:empty+1};code[guard]={$:'branch',condition:14,yes:guard+1,no:finish};code[exhausted]={$:'branch',condition:8,yes:exhausted+1,no:leave};code[undefinedElement]={$:'branch',condition:8,yes:advance,no:undefinedElement+1};code[nullElement]={$:'branch',condition:8,yes:advance,no:nullElement+1};
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:15,captures:[],instructions:code};
}
