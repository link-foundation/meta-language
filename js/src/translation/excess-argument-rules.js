// Supplied JavaScript arguments all evaluate, including unused trailing values.
/** [consumedCount,...actual argument indices], or [] for the existing route.
 * @param {string} language @param {number} parameters @param {number} provided
 * @returns {number[]} */
export function planJavaScriptExcessArguments(language,parameters,provided){
 if(language!=='JavaScript'||!Number.isSafeInteger(parameters)||!Number.isSafeInteger(provided)||parameters<0||provided<=parameters||provided>1024)return [];
 let result=[parameters];for(let index=0;index<provided;index+=1)result=result.concat([index]);return result;
}
