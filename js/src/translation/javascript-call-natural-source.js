// Shared source construction and admission. No represented expression runs.
import {compileJavaScriptCallSource} from './javascript-call-source.js';
import {readJavaScriptSourceExceptionOperation} from './source-exception-rules.js';

/** @param {string} message @returns {JavaScriptCallSource} */
export function constructJavaScriptLiteralErrorSource(message){
 /** @type {number[]} */ const units=[];
 for(let index=0;index<message.length;index+=1)units.push(message.charCodeAt(index));
 return {$:'sourcePrimitive',operation:readJavaScriptSourceExceptionOperation('Error'),arguments:[{$:'sourceConstant',value:{$:'text',units}},{$:'sourceConstant',value:{$:'boolean',value:true}}]};
}
/** @param {number} identity @returns {JavaScriptCallSource} */
export function constructJavaScriptNaturalNegativeSource(identity){
 return {$:'sourceNumberBinary',operator:'lessThan',left:{$:'sourcePrimitive',operation:'compare integers',arguments:[{$:'sourceBinding',identity},{$:'sourceConstant',value:{$:'integer',value:0n}}]},right:{$:'sourceConstant',value:{$:'number',value:0}}};
}
/** @param {number} identity @param {string} message @param {JavaScriptCallSource} body @returns {JavaScriptCallSource} */
export function constructJavaScriptNaturalGuardSource(identity,message,body){
 return {$:'sourceIf',condition:constructJavaScriptNaturalNegativeSource(identity),yes:constructJavaScriptLiteralErrorSource(message),no:body};
}
/** @param {number} identity @param {string} message @param {boolean} clamp @returns {JavaScriptCallSource} */
export function constructJavaScriptNaturalCastSource(identity,message,clamp){
 return {$:'sourceIf',condition:constructJavaScriptNaturalNegativeSource(identity),yes:clamp?{$:'sourceConstant',value:{$:'integer',value:0n}}:constructJavaScriptLiteralErrorSource(message),no:{$:'sourceBinding',identity}};
}
/** @param {number} left @param {number} right @returns {JavaScriptCallSource} */
export function constructJavaScriptNaturalDifferenceSource(left,right){
 return {$:'sourceIf',condition:{$:'sourceNumberBinary',operator:'lessThan',left:{$:'sourcePrimitive',operation:'compare integers',arguments:[{$:'sourceBinding',identity:left},{$:'sourceBinding',identity:right}]},right:{$:'sourceConstant',value:{$:'number',value:0}}},yes:{$:'sourceConstant',value:{$:'integer',value:0n}},no:{$:'sourcePrimitive',operation:'subtract integers',arguments:[{$:'sourceBinding',identity:left},{$:'sourceBinding',identity:right}]}};
}
/** @param {number} identity @returns {JavaScriptCallSource} */
export function constructJavaScriptNaturalPredecessorSource(identity){
 return {$:'sourcePrimitive',operation:'subtract integers',arguments:[{$:'sourceBinding',identity},{$:'sourceConstant',value:{$:'integer',value:1n}}]};
}
/** @param {number} identity @param {JavaScriptCallSource} zero @param {JavaScriptCallSource} succ @returns {JavaScriptCallSource} */
export function constructJavaScriptNaturalMatchSource(identity,zero,succ){
 return {$:'sourceIf',condition:{$:'sourceStrictEqual',left:{$:'sourceBinding',identity},right:{$:'sourceConstant',value:{$:'integer',value:0n}}},yes:zero,no:succ};
}
/** The adapter supplies canonical decimal spellings of actual global realm IDs.
 * @param {string[]} realms @returns {boolean} */
export function testJavaScriptNaturalSourceRealm(realms){
 if(realms.length===0)return false;
 const first=realms[0]??'';
 for(const realm of realms)if(realm!==first)return false;
 return true;
}
/** -1 is refusal; valid positions remain in complete source guard order.
 * @param {string[]} types @param {number[]} orders @param {boolean} oneRealm @returns {number[]} */
export function planJavaScriptNaturalGuardOrder(types,orders,oneRealm){
 if(types.length!==orders.length||types.length>4096||types.length>0&&!oneRealm)return [-1];
 /** @type {number[]} */ const positions=[];
 for(let order=0;order<orders.length;order+=1){
  let found=-1;
  for(let index=0;index<orders.length;index+=1)if((orders[index]??-1)===order){if(found!==-1)return [-1];found=index;}
  if(found<0||(types[found]??'')!=='nat')return [-1];
  positions.push(found);
 }
 return positions;
}
/** Zero refuses; one widens; two clamps; three retains a checked source guard.
 * @param {string} from @param {string} to @param {string} flavor @param {boolean} retainedMessage @param {number} order @param {boolean} oneRealm @returns {number} */
export function readJavaScriptNaturalCastPlan(from,to,flavor,retainedMessage,order,oneRealm){
 if(from==='nat'&&to==='int'&&flavor==='exact')return 1;
 if(from!=='int'||to!=='nat')return 0;
 if(flavor==='clamp')return 2;
 return flavor==='checked'&&retainedMessage&&Number.isSafeInteger(order)&&order>=0&&oneRealm?3:0;
}
/** @param {string} pattern @returns {number} */
export function readJavaScriptNaturalPatternBinding(pattern){
 return pattern==='natSucc'?2:pattern==='bind'?1:pattern==='natZero'||pattern==='wild'?0:-1;
}
/** Complete natural branches, including one permitted default branch.
 * @param {string} kind @param {string[]} patterns @returns {number[]} */
export function planJavaScriptNaturalPatternBranches(kind,patterns){
 if(kind!=='nat'||patterns.length>3)return [];
 let zero=-1,succ=-1,fallback=-1;
 for(let index=0;index<patterns.length;index+=1){
  const pattern=patterns[index]??'';
  if(pattern==='natZero'){if(zero!==-1)return [];zero=index;}
  else if(pattern==='natSucc'){if(succ!==-1)return [];succ=index;}
  else if(pattern==='wild'||pattern==='bind'){if(fallback!==-1)return [];fallback=index;}
  else return [];
 }
 if(zero===-1)zero=fallback;
 if(succ===-1)succ=fallback;
 return zero<0||succ<0?[]:[zero,succ];
}
/** Keep construction and its actual compiler available in one nominal model.
 * @param {JavaScriptCallSource} source @param {number[]} parameters @returns {JavaScriptCallSourceCompilation} */
export function compileJavaScriptNaturalSource(source,parameters){return compileJavaScriptCallSource(source,parameters);}
