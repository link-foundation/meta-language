// Shared ToObject bytecode authoring. Actual Objects retain identity; primitive
// wrappers use the selected realm roots and the shared internal-slot allocator.
/** @param {JavaScriptCallInstruction[]} code @param {number} input
 * @param {number} destination @param {number} typeRegister @param {number} keyRegister
 * @param {number} conditionRegister @param {number} prototypeRegister
 * @param {JavaScriptPrimitiveWrapperRoots|undefined} roots @returns {boolean} */
export function appendJavaScriptObjectCoercion(code,input,destination,typeRegister,keyRegister,conditionRegister,prototypeRegister,roots){
 if(roots?.$!=='primitiveWrapperRoots')return false;
 const choices=[{kind:'string',prototype:roots.stringPrototype},{kind:'number',prototype:roots.numberPrototype},{kind:'boolean',prototype:roots.booleanPrototype},{kind:'bigint',prototype:roots.integerPrototype},{kind:'symbol',prototype:roots.symbolPrototype}];
 if([roots.objectPrototype,...choices.map(row=>row.prototype)].some(value=>value?.$!=='object'||value.callable))return false;
 code.push({$:'primitive',destination:conditionRegister,operation:'require object coercible',arguments:[input]},{$:'primitive',destination:conditionRegister,operation:'test object value',arguments:[input]});
 const objectBranch=code.length;code.push({$:'branch',condition:conditionRegister,yes:0,no:objectBranch+1},{$:'primitive',destination:typeRegister,operation:'read semantic type',arguments:[input]});const jumps=[];
 for(const choice of choices){const units=[];for(let index=0;index<choice.kind.length;index++)units.push(choice.kind.charCodeAt(index));code.push({$:'constant',destination:keyRegister,value:{$:'text',units}},{$:'strictEqual',destination:conditionRegister,left:typeRegister,right:keyRegister});const branch=code.length;code.push({$:'branch',condition:conditionRegister,yes:branch+1,no:branch+3},{$:'constant',destination:prototypeRegister,value:choice.prototype});jumps.push(code.length);code.push({$:'jump',target:0});}
 code.push({$:'constant',destination:prototypeRegister,value:roots.objectPrototype});const allocation=code.length;code.push({$:'primitiveWrapper',destination,input,prototype:prototypeRegister});const finishJump=code.length;code.push({$:'jump',target:0});const unchanged=code.length;code.push({$:'copy',destination,source:input});const end=code.length;
 code[objectBranch]={$:'branch',condition:conditionRegister,yes:unchanged,no:objectBranch+1};code[finishJump]={$:'jump',target:end};for(const jump of jumps)code[jump]={$:'jump',target:allocation};return true;
}
