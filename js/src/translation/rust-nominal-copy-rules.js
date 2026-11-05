import {readRequiredNumberFact,readRequiredTextFact} from './fact-column-rules.js';
/** Nominal Copy is a source trait fact, never inferred merely from scalar fields.
 * Actual constructor fields form a finite dependency graph. Reference fields
 * carry an independent shared-reference Copy fact, without proving their target.
 * Output: status and owner flags; negative statuses retain the offending owner.
 * @param {string[]} owners @param {boolean[]} copies @param {boolean[]} clones
 * @param {number[]} starts @param {number[]} counts @param {number[]} targets
 * @param {boolean[]} leaves @returns {number[]} */
export function planRustNominalCopyOwners(owners,copies,clones,starts,counts,targets,leaves){
 const size=owners.length;
 if(copies.length!==size||clones.length!==size||starts.length!==size||counts.length!==size||targets.length!==leaves.length)return [-1];
 const accepted=[];let expectedStart=0;
 for(let owner=0;owner<size;owner+=1){
  const name=readRequiredTextFact(owners,owner),start=readRequiredNumberFact(starts,owner),count=readRequiredNumberFact(counts,owner);
  if(name===''||owners.indexOf(name)!==owner||!Number.isInteger(start)||start!==expectedStart||!Number.isInteger(count)||count<0||start+count>targets.length)return [-1,owner];
  if(copies[owner]&&!clones[owner])return [-2,owner];
  expectedStart+=count;
 }
 if(expectedStart!==targets.length)return [-1];
 for(let field=0;field<targets.length;field+=1){const target=readRequiredNumberFact(targets,field);if(!Number.isInteger(target)||target< -1||target>=size)return [-1];}
 for(let pass=0;pass<size;pass+=1){
  const before=accepted.length;
  for(let owner=0;owner<size;owner+=1){
   if(!copies[owner]||accepted.includes(owner))continue;
   let ready=true;const start=readRequiredNumberFact(starts,owner),count=readRequiredNumberFact(counts,owner);
   for(let at=start;at<start+count;at+=1){const target=readRequiredNumberFact(targets,at);if(!leaves[at]&&(target<0||!accepted.includes(target)))ready=false;}
   if(ready)accepted.push(owner);
  }
  if(accepted.length===before)break;
 }
 for(let owner=0;owner<size;owner+=1)if(copies[owner]&&!accepted.includes(owner))return [-3,owner];
 const result=[0];for(let owner=0;owner<size;owner+=1)result.push(accepted.includes(owner)?1:0);return result;
}

/** @param {string} kind @param {string[]} wrappers @param {boolean} primitiveCopy
 * @returns {boolean} */
export function acceptRustNominalCopyFieldLeaf(kind,wrappers,primitiveCopy){
 if(wrappers.length>0)return readRequiredTextFact(wrappers,0)==='reference';
 return primitiveCopy&&['bool','float','fixed','unit','function'].includes(kind);
}
