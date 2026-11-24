import {readRequiredTextFact,readRequiredNumberFact,readRequiredBooleanFact} from './fact-column-rules.js';
/** @param {string} kind @param {string} operator @param {boolean} atom
 * @param {boolean} literal @param {number} children @returns {boolean} */
export function acceptRustUsizeIndexExpressionNode(kind,operator,atom,literal,children){
 if(kind==='binary')return children===2&&(operator==='add'||operator==='sub'||operator==='mul'||operator==='div'||operator==='rem');
 if(kind==='if')return children===2;
 if(kind==='match')return children>0;
 return children===0&&(atom||literal||kind==='abort');
}
/** Actual AST ancestry is transport; recursive admission is shared here.
 * @param {string[]} kinds @param {string[]} operators @param {boolean[]} atoms
 * @param {boolean[]} literals @param {number[]} parents @returns {string} */
export function readRustUsizeIndexExpressionError(kinds,operators,atoms,literals,parents){
 const size=kinds.length;
 if(size<1||size>4096)return 'the index expression exceeds its source node limit';
 if(operators.length!==size||atoms.length!==size||literals.length!==size||parents.length!==size)return 'unaligned index expression source columns';
 for(let index=0;index<size;index+=1){
  const parent=readRequiredNumberFact(parents,index);
  if(!Number.isInteger(parent)||index===0&&parent!==-1||index>0&&(parent<0||parent>=index))return 'an index expression needs one root and actual earlier parents';
  let ancestor=parent,depth=0;
  while(ancestor>=0){depth+=1;if(depth>128)return 'the index expression exceeds its source depth limit';ancestor=readRequiredNumberFact(parents,ancestor);}
 }
 // Append-only postorder facts avoid alias-sensitive storage writes. The
 // 4096-node bound limits all child-directory visits to 8386560 pairs.
 const accepted=[];
 for(let remaining=size;remaining>0;remaining-=1){
  const index=remaining-1;
  let count=0,childrenAccepted=true;
  for(let child=index+1;child<size;child+=1)if(readRequiredNumberFact(parents,child)===index){count+=1;childrenAccepted=childrenAccepted&&readRequiredBooleanFact(accepted,size-child-1);}
  accepted.push(childrenAccepted&&acceptRustUsizeIndexExpressionNode(readRequiredTextFact(kinds,index),readRequiredTextFact(operators,index),readRequiredBooleanFact(atoms,index),readRequiredBooleanFact(literals,index),count));
 }
 return readRequiredBooleanFact(accepted,size-1)?'':'the complete index expression needs actual usize atoms or contextually inferred integer operands';
}
