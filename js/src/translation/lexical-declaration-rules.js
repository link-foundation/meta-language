// Early errors use actual parsed BoundNames, never declaration lookahead counts.
import {readRequiredNumberFact,readRequiredTextFact} from './fact-column-rules.js';
/** Declaration kinds: 0 ordinary function, 1 lexical. A function/function
 * pair remains governed by its source-type profile; a lexical collision is
 * an early error in both Script and Module. Parameters occupy the immediately
 * enclosing callable frame, not every descendant block.
 * @param {number[]} parents @param {number[]} boundaries
 * @param {number[]} parameterScopes @param {string[]} parameterNames
 * @param {number[]} declarationScopes @param {string[]} declarationNames
 * @param {number[]} declarationKinds @returns {number[]} */
export function findJavaScriptLexicalDeclarationConflict(parents,boundaries,parameterScopes,parameterNames,declarationScopes,declarationNames,declarationKinds){
 if(parents.length!==boundaries.length||parameterScopes.length!==parameterNames.length||declarationScopes.length!==declarationNames.length||declarationKinds.length!==declarationNames.length)throw new RangeError('inconsistent actual lexical declaration columns');
 for(let index=0;index<declarationNames.length;index+=1){
  const scope=readRequiredNumberFact(declarationScopes,index),name=readRequiredTextFact(declarationNames,index),kind=readRequiredNumberFact(declarationKinds,index);
  if(scope<0||scope%1!==0||scope>=parents.length||(kind!==0&&kind!==1))throw new RangeError('invalid actual lexical declaration frame or kind');
  for(let previous=0;previous<index;previous+=1){
   if(readRequiredNumberFact(declarationScopes,previous)===scope&&readRequiredTextFact(declarationNames,previous)===name&&(kind===1||readRequiredNumberFact(declarationKinds,previous)===1))return [index,0];
  }
  const parent=readRequiredNumberFact(parents,scope);
  if(kind===1&&readRequiredNumberFact(boundaries,scope)===0&&parent>=0&&readRequiredNumberFact(boundaries,parent)>0){
   for(let parameter=0;parameter<parameterNames.length;parameter+=1)if(readRequiredNumberFact(parameterScopes,parameter)===parent&&readRequiredTextFact(parameterNames,parameter)===name)return [index,1];
  }
 }
 return [];
}
