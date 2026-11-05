import {readRequiredNumberFact,readRequiredTextFact} from './fact-column-rules.js';
import {readCollectionConstructionField as field,readCollectionConstructionElement as element} from './collection-construction-rules.js';
/** @param {string} kind @param {string} proof @returns {string} */
export function readJavaScriptClosedClassObservationOperands(kind,proof){
 if(['eq','ne','lt','le','gt','ge'].includes(proof))return 'left right';
 if(proof==='forall')return 'binders';
 if(kind==='toString'||kind==='optionalText')return 'arg';
 if(kind==='strictEqual')return 'left right';
 if(kind==='print'||kind==='output')return 'expr';
 return '';
}
/** The original two-column public decision remains exact for its existing
 * native callers. @param {string} kind @param {string} proof
 * @param {string} operation @returns {string} */
export function readJavaScriptClosedClassObservationOperationOperands(kind,proof,operation){return kind==='binary'&&['eq','ne'].includes(operation)?'left right':readJavaScriptClosedClassObservationOperands(kind,proof);}
/** A bounded query through actual type children and declared nominal fields.
 * A recursive declaration is visited once; missing type ownership refuses.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} root
 * @param {number} declarations @param {string[]} brands @returns {boolean} */
export function containsJavaScriptClosedClassBrand(texts,starts,counts,labels,targets,root,declarations,brands){
 if(brands.length===0)return false;
 let pending=[root],seen=[];
 while(pending.length){
  if(seen.length>=100000||pending.length>100000)return true;
  const node=readRequiredNumberFact(pending,pending.length-1);pending=pending.slice(0,-1);
  if(node<0||node>=starts.length)return true;if(seen.includes(node))continue;seen=seen.concat([node]);
  const owner=field(starts,counts,labels,targets,node,'kind');if(owner<0)return true;
  const kind=readRequiredTextFact(texts,owner);
  if(kind==='record'){
   const fields=field(starts,counts,labels,targets,node,'fields');if(fields<0)return true;
   for(let at=0;at<readRequiredNumberFact(counts,fields);at+=1){const row=element(starts,counts,targets,fields,at),name=field(starts,counts,labels,targets,row,'name');if(name<0)return true;if(brands.includes(readRequiredTextFact(texts,name)))return true;pending=pending.concat([field(starts,counts,labels,targets,row,'type')]);}
  }else if(['array','option','set','cell'].includes(kind))pending=pending.concat([field(starts,counts,labels,targets,node,'element')]);
  else if(kind==='map')pending=pending.concat([field(starts,counts,labels,targets,node,'key'),field(starts,counts,labels,targets,node,'value')]);
  else if(kind==='tuple'||kind==='union'||kind==='function'){
   const children=field(starts,counts,labels,targets,node,kind==='tuple'?'elements':kind==='union'?'alternatives':'params');if(children<0)return true;
   for(let at=0;at<readRequiredNumberFact(counts,children);at+=1)pending=pending.concat([element(starts,counts,targets,children,at)]);
   if(kind==='function')pending=pending.concat([field(starts,counts,labels,targets,node,'ret')]);
  }else if(kind==='data'){
   const name=field(starts,counts,labels,targets,node,'name');if(name<0||declarations<0)return true;let found=false;
   for(let at=0;at<readRequiredNumberFact(counts,declarations);at+=1){const row=element(starts,counts,targets,declarations,at),fullName=field(starts,counts,labels,targets,row,'fullName');if(fullName<0||readRequiredTextFact(texts,fullName)!==readRequiredTextFact(texts,name))continue;
    const constructors=field(starts,counts,labels,targets,row,'ctors');if(constructors<0)return true;found=true;
    for(let ctor=0;ctor<readRequiredNumberFact(counts,constructors);ctor+=1){const fields=field(starts,counts,labels,targets,element(starts,counts,targets,constructors,ctor),'fields');if(fields<0)return true;for(let index=0;index<readRequiredNumberFact(counts,fields);index+=1)pending=pending.concat([field(starts,counts,labels,targets,element(starts,counts,targets,fields,index),'type')]);}
   }if(!found)return true;
  }else if(!['unit','bool','float','int','nat','fixed','string'].includes(kind))return true;
 }return false;
}
