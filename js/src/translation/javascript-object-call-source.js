// Object Call selects actual realm prototypes before shared wrapper allocation.
/** @typedef {{$:'primitiveWrapperRoots',objectPrototype:JavaScriptSemanticValue,stringPrototype:JavaScriptSemanticValue,numberPrototype:JavaScriptSemanticValue,booleanPrototype:JavaScriptSemanticValue,integerPrototype:JavaScriptSemanticValue,symbolPrototype:JavaScriptSemanticValue}} JavaScriptPrimitiveWrapperRoots */
/** @param {bigint} identity @param {bigint} realm @param {JavaScriptPrimitiveWrapperRoots|undefined} roots @returns {JavaScriptCallFunction|undefined} */
export function compileJavaScriptObjectCallFunction(identity,realm,roots){
 if(roots?.$!=='primitiveWrapperRoots')return undefined;
 const choices=[{kind:'string',prototype:roots.stringPrototype},{kind:'number',prototype:roots.numberPrototype},{kind:'boolean',prototype:roots.booleanPrototype},{kind:'bigint',prototype:roots.integerPrototype},{kind:'symbol',prototype:roots.symbolPrototype}];
 if([roots.objectPrototype,...choices.map(row=>row.prototype)].some(value=>value?.$!=='object'||value.callable))return undefined;
 /** @type {JavaScriptCallInstruction[]} */const code=[{$:'argument',destination:1,index:0},{$:'primitive',destination:3,operation:'test object value',arguments:[1]},{$:'branch',condition:3,yes:0,no:3},{$:'primitive',destination:2,operation:'read semantic type',arguments:[1]}];const jumps=[];
 for(const choice of choices){const units=[];for(let at=0;at<choice.kind.length;at+=1)units.push(choice.kind.charCodeAt(at));code.push({$:'constant',destination:3,value:{$:'text',units}},{$:'strictEqual',destination:4,left:2,right:3});const branch=code.length;code.push({$:'branch',condition:4,yes:branch+1,no:branch+3},{$:'constant',destination:5,value:choice.prototype});jumps.push(code.length);code.push({$:'jump',target:0});}
 code.push({$:'constant',destination:5,value:roots.objectPrototype});const allocation=code.length;code.push({$:'primitiveWrapper',destination:2,input:1,prototype:5},{$:'return',source:2});const unchanged=code.length;code.push({$:'return',source:1});code[2]={$:'branch',condition:3,yes:unchanged,no:3};for(const jump of jumps)code[jump]={$:'jump',target:allocation};
 return {$:'callFunction',identity,realm,receiverMode:'primitive intrinsic',lexicalReceiver:{$:'undefined'},parameters:0,registers:6,captures:[],instructions:code};
}
