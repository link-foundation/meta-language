// Complete finite constructor ownership and field-choice plans. These rules do
// not equate constructors by spelling: callers supply actual checked owners.
/** @param {string} owner @param {string} constructor @param {number} fields
 * @param {string[]} owners @param {string[]} constructors @param {number[]} arities
 * @returns {boolean} */
export function testConditionalConstructorOwners(owner,constructor,fields,owners,constructors,arities){
 if(owner===''||constructor===''||!Number.isSafeInteger(fields)||fields<0||fields>256)return false;
 if(owners.length<2||owners.length>256||constructors.length!==owners.length||arities.length!==owners.length)return false;
 for(let index=0;index<owners.length;index+=1)
  if(owners[index]!==owner||constructors[index]!==constructor||arities[index]!==fields)return false;
 return true;
}
/** Return one representative default leaf followed by every nondefault leaf.
 * Class identities come only from actual equal stable atoms or unique leaves;
 * equal names never classify calls, property reads or arbitrary expressions.
 * @param {number[]} classes @returns {number[]} */
export function planConditionalConstructorFieldSelection(classes){
 if(classes.length<2||classes.length>256)return [];
 for(const value of classes)if(!Number.isSafeInteger(value)||value<0||value>=classes.length)return [];
 let selected=0,maximum=0;
 for(let index=0;index<classes.length;index+=1){
  let count=0;
  for(const value of classes)if(value===classes[index])count+=1;
  if(count>maximum){selected=index;maximum=count;}
 }
 const result=[selected],fallback=classes[selected]?? -1;
 for(let index=0;index<classes.length;index+=1)if(classes[index]!==fallback)result.push(index);
 return result;
}
