import {readRequiredTextFact} from './fact-column-rules.js';
/** Actual ordinary source owners only; carried input is still a refusal.
 * @param {string} term @param {boolean} comment @param {number} role @returns {boolean} */
export function acceptRustValueModuleItem(term,comment,role){return role!==3&&(comment||term==='function_item'||term==='const_item'||term==='type_item'||term==='attribute_item'||term==='inner_attribute_item');}
/** @param {string} term @returns {string} */
export function readRustValueModuleOwnerKind(term){return term==='const_item'?'const':term==='function_item'?'fn':'';}
/** Full original header, including ordered attributes and actual visibility.
 * @param {string[]} kinds @param {string[]} values @returns {number} */
export function readRustPublicConstantNamePosition(kinds,values){
 if(kinds.length!==values.length)return -1;
 let position=0;
 while(position<values.length&&readRequiredTextFact(kinds,position)==='punct'&&readRequiredTextFact(values,position)==='#'){
  position+=1;if(position<values.length&&readRequiredTextFact(kinds,position)==='punct'&&readRequiredTextFact(values,position)==='!')position+=1;
  if(position>=values.length||readRequiredTextFact(kinds,position)!=='punct'||readRequiredTextFact(values,position)!=='[')return -1;
  let depth=1;position+=1;
  while(position<values.length&&depth>0){if(readRequiredTextFact(kinds,position)==='punct'){if(readRequiredTextFact(values,position)==='[')depth+=1;else if(readRequiredTextFact(values,position)===']')depth-=1;}position+=1;}
  if(depth!==0)return -1;
 }
 if(position+3>=values.length||readRequiredTextFact(kinds,position)!=='identifier'||readRequiredTextFact(values,position)!=='pub')return -1;
 position+=1;
 if(readRequiredTextFact(kinds,position)!=='identifier'||readRequiredTextFact(values,position)!=='const'||readRequiredTextFact(kinds,position+1)!=='identifier')return -1;
 return position+1;
}
/** Reserve against every actual emitted identifier, including helper names.
 * @param {number} position @param {string[]} reserved @returns {string} */
export function readRustModuleValueExportBinding(position,reserved){
 let name='ml_source_constant_export_'+position;
 while(reserved.includes(name))name+='_';
 return name;
}
/** A source constant exports its value; exported keyword names stay aliases.
 * @param {string} source @param {string} getter @param {string} binding @returns {string} */
export function renderRustModuleValueExport(source,getter,binding){return 'const '+binding+' = '+getter+'();\nexport { '+binding+' as '+source+' };';}
/** The source App discriminator is preserved; a bare constant is still a read.
 * @param {string} head @param {boolean} local @param {string} declaration
 * @param {string} value @returns {boolean} */
export function requireCallableConstantApplicationCapture(head,local,declaration,value){return head==='name'&&!local&&declaration==='const'&&value==='function';}
/** Allocate the complete interface together against all actual target names.
 * @param {string[]} kinds @param {string[]} reserved @returns {string[]} */
export function planRustModuleValueExportBindings(kinds,reserved){
 const occupied=new Set(reserved),bindings=[];
 for(let position=0;position<kinds.length;position+=1){
  if(readRequiredTextFact(kinds,position)!=='const'){bindings.push('');continue;}
  let name='ml_source_constant_export_'+position;
  while(occupied.has(name))name+='_';
  // Different numeric positions have disjoint underscore-suffix families.
  bindings.push(name);
 }
 return bindings;
}
