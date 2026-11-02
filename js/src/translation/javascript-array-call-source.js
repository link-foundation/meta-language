// Array [[Call]] uses its actual active Array owner as newTarget. The generic
// Construct kernel owns all allocation, length, prototype and element semantics.
/** @param {bigint} identity @param {bigint} realm @returns {JavaScriptCallFunction} */
export function compileJavaScriptArrayCallFunction(identity,realm){
 /** @type {JavaScriptCallInstruction[]} */const instructions=[
  {$:'constant',destination:1,value:{$:'object',identity,callable:true}},
  {$:'argumentCount',destination:2},
  {$:'constant',destination:3,value:{$:'number',value:0}},
  {$:'constant',destination:4,value:{$:'number',value:1}},
  {$:'argumentListOpen',slot:0},
  {$:'numberBinary',destination:5,left:3,right:2,operator:'lessThan'},
  {$:'branch',condition:5,yes:7,no:11},
  {$:'argumentAt',destination:6,index:3},
  {$:'argumentListAppend',slot:0,value:6},
  {$:'numberBinary',destination:3,left:3,right:4,operator:'add'},
  {$:'jump',target:5},
  {$:'argumentListCall',destination:7,slot:0,callee:1,receiver:1,mode:'construct'},
  {$:'return',source:7}
 ];
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:8,captures:[],instructions};
}
