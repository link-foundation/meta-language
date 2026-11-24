import {readRequiredNumberFact,readRequiredTextFact} from './fact-column-rules.js';
// Canonical checked graph -> lexical binding/allocation facts. Both hosts pass
// the existing primitive-column view of the actual checked declarations/main.
import {readCollectionConstructionField,readCollectionConstructionElement} from './collection-construction-rules.js';
import {readProgramExpressionOperandFields} from '../typed-program-link-rules.js';
import {readReferenceCallbackOperandFields} from './reference-method-rules.js';
import {readDisjointCollectionIterationWriteField,acceptDisjointCollectionIterationReadExpression,readDisjointCollectionIterationMarkerRole} from './disjoint-collection-iteration-rules.js';
import {readArrayCallbackResultForm} from './array-callback-rules.js';
import {acceptCollectionCopyMapperSnapshot} from './collection-copy-rules.js';
/** @param {number[][]} state @param {number} column @returns {number[]} */
function row(state,column){return state[column]??[];}
/** @param {number[][]} state @param {number} column @param {number[]} value @returns {number[][]} */
function replace(state,column,value){return state.map((previous,index)=>index===column?value:previous);}
/** @param {number[][]} state @param {number} column @param {number} value @returns {number[][]} */
function append(state,column,value){return replace(state,column,row(state,column).concat([value]));}
/** @param {number[][]} state @returns {number} */
function last(state){return readRequiredNumberFact(row(state,15),0);}
/** @param {number[][]} state @param {number} node @returns {number[][]} */
function finish(state,node){return replace(state,15,[node]);}
/** @param {number[][]} state @param {number} role @param {number} operation
 * @param {number} kind @param {number} owner @returns {number[][]} */
function make(state,role,operation,kind,owner){
 const identity=row(state,0).length;if(identity>=4096)return replace(state,16,[1]);
 return finish(append(append(append(append(state,0,role),1,operation),2,kind),3,owner),identity);
}
/** @param {number[][]} state @param {number} owner @param {number} label
 * @param {number} auxiliary @param {number} target @returns {number[][]} */
function edge(state,owner,label,auxiliary,target){return append(append(append(append(state,4,owner),5,label),6,auxiliary),7,target);}
/** @param {number[][]} state @param {number} owner @param {number} label @returns {number} */
function selectedEdge(state,owner,label){for(let at=0;at<row(state,4).length;at+=1)if(readRequiredNumberFact(row(state,4),at)===owner&&readRequiredNumberFact(row(state,5),at)===label)return readRequiredNumberFact(row(state,7),at);return -1;}
/** @param {string[]} texts @param {number} identity @returns {string} */
function text(texts,identity){return identity<0?'':readRequiredTextFact(texts,identity);}
/** @param {number[]} starts @param {number[]} counts @param {string[]} labels
 * @param {number[]} targets @param {number} node @param {string} name @returns {number} */
function field(starts,counts,labels,targets,node,name){return readCollectionConstructionField(starts,counts,labels,targets,node,name);}
/** @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} node
 * @param {string} name @returns {string} */
function word(texts,starts,counts,labels,targets,node,name){return text(texts,field(starts,counts,labels,targets,node,name));}
/** @param {number[]} starts @param {number[]} counts @param {string[]} labels
 * @param {number[]} targets @param {number} node @returns {number} */
function kind(starts,counts,labels,targets,node){return field(starts,counts,labels,targets,field(starts,counts,labels,targets,node,'type'),'kind');}
/** @param {number[][]} state @param {number} name @param {string[]} texts @returns {number} */
function symbol(state,name,texts){for(let at=row(state,13).length-1;at>=0;at-=1)if(text(texts,readRequiredNumberFact(row(state,13),at))===text(texts,name))return readRequiredNumberFact(row(state,14),at);return -1;}
/** @param {number[][]} state @param {number} name @param {string[]} texts @returns {number[][]} */
function callable(state,name,texts){
 const known=symbol(state,name,texts);if(known>=0)return finish(state,known);
 state=make(state,1,-1,-1,-1);const root=last(state);return finish(append(append(append(state,8,root),13,name),14,root),root);
}
/** @param {number[][]} state @param {number} source @param {number} owner
 * @param {number[]} names @param {number[]} bindings @param {string[]} texts
 * @param {number[]} starts @param {number[]} counts @param {string[]} labels
 * @param {number[]} targets @param {string[]} markers @param {number} depth
 * @returns {number[][]} */
function expression(state,source,owner,names,bindings,texts,starts,counts,labels,targets,markers,depth){
 if((readRequiredNumberFact(row(state,16),0))!==0)return state;
 if(depth>256)return replace(state,16,[1]);
 const shape=word(texts,starts,counts,labels,targets,source,'k'),type=kind(starts,counts,labels,targets,source),operation=field(starts,counts,labels,targets,source,'op');
 let role=0,operationValue=operation;
 if(shape==='var')role=3;
 else if(shape==='referenceAllocate'||shape==='cellAllocate')role=5;
 else if(shape==='referenceRead')role=6;
 else if(shape==='recordField'){role=7;operationValue=field(starts,counts,labels,targets,source,'field');}
 else if(shape==='tupleIndex'){role=8;operationValue=-2-Number(word(texts,starts,counts,labels,targets,source,'index'));}
 else if(shape==='functionRef')role=9;
 else if(shape==='call'||shape==='recur')role=10;
 else if(shape==='apply')role=11;
 else if(shape==='method')role=12;
 else if(shape==='record')role=13;
 else if(shape==='array'||shape==='map'||shape==='set'||shape==='collectionItems')role=14;
 else if(shape==='tuple')role=15;
 else if(shape==='ctor')role=16;
 else if(shape==='cellWrite')role=18;
 else if(['index','optionalIndex','collectionRead','cellRead','optionalField','optionalProject'].includes(shape))role=20;
 else if(!acceptDisjointCollectionIterationReadExpression(shape))role=19;
 if(shape==='lambda')role=1;
 const marker=markers.indexOf(text(texts,operation)),consumer=readDisjointCollectionIterationMarkerRole(text(texts,operation));if(shape==='collectionItems'&&marker>=0)role=consumer==='eager'?21:17;
 if(shape==='collectionItems'&&consumer!==''&&marker<0)return replace(state,16,[1]);
 state=make(state,role,operationValue,type,owner);const node=last(state);
 if(shape==='var'){
  const name=field(starts,counts,labels,targets,source,'name');let binding=-1;
  for(let at=names.length-1;at>=0;at-=1)if(text(texts,readRequiredNumberFact(names,at))===text(texts,name)){binding=readRequiredNumberFact(bindings,at);break;}
  if(binding>=0)state=edge(state,node,1,-1,binding);else state=replace(state,0,row(state,0).map((value,index)=>index===node?19:value));
  return finish(state,node);
 }
 if(shape==='functionRef'||shape==='call'||shape==='recur'){
  if(shape==='recur')state=edge(state,node,4,-1,owner);
  else{state=callable(state,field(starts,counts,labels,targets,source,'fn'),texts);state=edge(state,node,4,-1,last(state));}
 }
 if(shape==='let'){
  const value=field(starts,counts,labels,targets,source,'value');state=expression(state,value,owner,names,bindings,texts,starts,counts,labels,targets,markers,depth+1);const actual=last(state);
  state=make(state,4,-1,kind(starts,counts,labels,targets,value),owner);const binding=last(state);state=edge(state,binding,1,-1,actual);
  state=expression(state,field(starts,counts,labels,targets,source,'body'),owner,names.concat([field(starts,counts,labels,targets,source,'name')]),bindings.concat([binding]),texts,starts,counts,labels,targets,markers,depth+1);
  return finish(edge(state,node,3,-1,last(state)),node);
 }
 if(shape==='lambda'){
  state=append(state,8,node);let localNames=names,localBindings=bindings;const params=field(starts,counts,labels,targets,source,'params');
  for(let at=0;at<(count(counts,params));at+=1){const param=readCollectionConstructionElement(starts,counts,targets,params,at);state=make(state,2,-1,kind(starts,counts,labels,targets,param),node);const binding=last(state);state=append(append(state,9,node),10,binding);localNames=localNames.concat([field(starts,counts,labels,targets,param,'name')]);localBindings=localBindings.concat([binding]);}
  state=expression(state,field(starts,counts,labels,targets,source,'body'),node,localNames,localBindings,texts,starts,counts,labels,targets,markers,depth+1);
  return finish(append(append(state,11,node),12,last(state)),node);
 }
 if(shape==='match'){
  const scrutinee=field(starts,counts,labels,targets,source,'scrutinee');state=expression(state,scrutinee,owner,names,bindings,texts,starts,counts,labels,targets,markers,depth+1);const value=last(state),cases=field(starts,counts,labels,targets,source,'cases');
  for(let at=0;at<(count(counts,cases));at+=1){
   const arm=readCollectionConstructionElement(starts,counts,targets,cases,at),pattern=field(starts,counts,labels,targets,arm,'pattern'),form=word(texts,starts,counts,labels,targets,pattern,'k');let localNames=names,localBindings=bindings;
   if(form==='bind'){state=make(state,4,-1,kind(starts,counts,labels,targets,scrutinee),owner);const binding=last(state);state=edge(state,binding,1,-1,value);localNames=localNames.concat([field(starts,counts,labels,targets,pattern,'name')]);localBindings=localBindings.concat([binding]);}
   else if(form==='ctor'){
    const actual=field(starts,counts,labels,targets,pattern,'binds');
    for(let bind=0;bind<(count(counts,actual));bind+=1){const name=readCollectionConstructionElement(starts,counts,targets,actual,bind);if(text(texts,name)==='')continue;state=make(state,8,-2-bind,-1,owner);const binding=last(state);state=edge(state,binding,2,-1,value);localNames=localNames.concat([name]);localBindings=localBindings.concat([binding]);}
   }
   state=expression(state,field(starts,counts,labels,targets,arm,'body'),owner,localNames,localBindings,texts,starts,counts,labels,targets,markers,depth+1);state=edge(state,node,3,-1,last(state));
  }return finish(state,node);
 }
 if(shape==='record'||shape==='array'||shape==='tuple'||shape==='ctor'){
  const fieldName=shape==='record'?'entries':shape==='ctor'?'args':'items',values=field(starts,counts,labels,targets,source,fieldName);
  for(let at=0;at<(count(counts,values));at+=1){const item=readCollectionConstructionElement(starts,counts,targets,values,at),value=shape==='record'?field(starts,counts,labels,targets,item,'value'):item;state=expression(state,value,owner,names,bindings,texts,starts,counts,labels,targets,markers,depth+1);if(shape!=='array')state=edge(state,node,shape==='record'?6:7,shape==='record'?field(starts,counts,labels,targets,item,'name'):at,last(state));}
  if(shape==='record'){const order=field(starts,counts,labels,targets,source,'order');if(order>=0)state=expression(state,order,owner,names,bindings,texts,starts,counts,labels,targets,markers,depth+1);}
  return finish(state,node);
 }
 let operandFields=readReferenceCallbackOperandFields(shape,readProgramExpressionOperandFields(shape)).split(' ');
 for(const operand of operandFields){
  if(operand==='')continue;const value=field(starts,counts,labels,targets,source,operand);if(value<0)continue;
  if(operand==='args'){
   let first=-1;for(let at=0;at<(count(counts,value));at+=1){state=expression(state,readCollectionConstructionElement(starts,counts,targets,value,at),owner,names,bindings,texts,starts,counts,labels,targets,markers,depth+1);state=edge(state,node,5,-1,last(state));if(at===0)first=last(state);}
   if(shape==='method'&&first>=0){const index=readDisjointCollectionIterationWriteField(text(texts,operation)),object=field(starts,counts,labels,targets,source,'object'),fields=field(starts,counts,labels,targets,field(starts,counts,labels,targets,object,'type'),'fields');if(index>=0&&index<(count(counts,fields))){const descriptor=readCollectionConstructionElement(starts,counts,targets,fields,index);state=edge(state,node,6,field(starts,counts,labels,targets,descriptor,'name'),first);}}
  }else{
   state=expression(state,value,owner,names,bindings,texts,starts,counts,labels,targets,markers,depth+1);
   const label=operand==='fn'?4:operand==='reference'||operand==='value'&&shape==='referenceAllocate'||operand==='value'&&shape==='cellAllocate'?1:operand==='object'||operand==='cell'?2:3;
   state=edge(state,node,label,-1,last(state));
  }
 }
 if(role===17||role===21){const receiver=selectedEdge(state,node,2);state=append(append(state,17,node),18,receiver);}
 if(shape==='method'){
  const object=field(starts,counts,labels,targets,source,'object'),objectShape=word(texts,starts,counts,labels,targets,object,'k'),objectKind=text(texts,kind(starts,counts,labels,targets,object)),op=text(texts,operation);let protectedSource=-1;
  if(objectShape==='referenceRead'&&objectKind==='array'&&readArrayCallbackResultForm(op)!=='')protectedSource=selectedEdge(state,node,2);
  if(objectShape==='collectionItems'&&op==='map'){
   const source_=field(starts,counts,labels,targets,object,'object'),actualKind=text(texts,kind(starts,counts,labels,targets,source_)),actualOperation=word(texts,starts,counts,labels,targets,object,'op');
   if(acceptCollectionCopyMapperSnapshot(actualKind,actualOperation))protectedSource=selectedEdge(state,selectedEdge(state,node,2),2);
  }
  if(protectedSource>=0){state=append(append(state,19,node),20,protectedSource);}
 }
 return finish(state,node);
}
/** Role names are protocol values, never inferred source ownership.
 * @param {number} role @returns {string} */
export function readDisjointCollectionProjectionRole(role){return ['value','function','parameter','variable','binding','allocation','read','field','field','function reference','call','apply','method','record','array','tuple','data','snapshot','cell write','unknown','opaque read','eager snapshot'][role]??'unknown';}
/** @param {number} role @param {number} operation @param {string} text @returns {string} */
export function readDisjointCollectionProjectionOperation(role,operation,text){return role===7?'field:'+text:role===8?'item:'+String(-2-operation):text;}
/** @param {number} role @param {string} value @returns {string} */
export function readDisjointCollectionProjectionKind(role,value){return role===1||role===9?'function':value||'unknown';}
/** @param {number} role @param {number} auxiliary @param {string} text @returns {string} */
export function readDisjointCollectionProjectionEdge(role,auxiliary,text){return role===1?'value':role===2?'object':role===3?'result':role===4?'callee':role===5?'argument':role===6?'field:'+text:role===7?'item:'+String(auxiliary):'';}
/** Output encodes counts, main root, node rows (role/op/type/owner), edge rows
 * (source/label/auxiliary/target), functions (root/body), parameters (root/slot),
 * snapshot/source and callback/source pairs. Original text fact IDs stay exact.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number} root
 * @param {string[]} markers @returns {number[]} */
export function projectDisjointCollectionIterationFacts(texts,starts,counts,labels,targets,root,markers){
 if(root<0||root>=starts.length||starts.length>1048576||starts.length!==counts.length||labels.length!==targets.length)return [1];
 let state=[[],[],[],[],[],[],[],[],[],[],[],[],[],[],[],[-1],[0],[],[],[],[]];
 const declarations=field(starts,counts,labels,targets,root,'declarations');
 for(let at=0;at<(count(counts,declarations));at+=1){const declaration=readCollectionConstructionElement(starts,counts,targets,declarations,at);if(word(texts,starts,counts,labels,targets,declaration,'k')==='fn')state=callable(state,field(starts,counts,labels,targets,declaration,'fullName'),texts);}
 state=make(state,1,-1,-1,-1);const main=last(state);state=append(state,8,main);
 for(let at=0;at<(count(counts,declarations));at+=1){
  const declaration=readCollectionConstructionElement(starts,counts,targets,declarations,at);if(word(texts,starts,counts,labels,targets,declaration,'k')!=='fn')continue;
  const owner=symbol(state,field(starts,counts,labels,targets,declaration,'fullName'),texts),params=field(starts,counts,labels,targets,declaration,'params');let names=[],bindings=[];
  for(let parameter=0;parameter<(count(counts,params));parameter+=1){const actual=readCollectionConstructionElement(starts,counts,targets,params,parameter);state=make(state,2,-1,kind(starts,counts,labels,targets,actual),owner);const binding=last(state);state=append(append(state,9,owner),10,binding);names=names.concat([field(starts,counts,labels,targets,actual,'name')]);bindings=bindings.concat([binding]);}
  state=expression(state,field(starts,counts,labels,targets,declaration,'body'),owner,names,bindings,texts,starts,counts,labels,targets,markers,0);state=append(append(state,11,owner),12,last(state));
 }
 let names=[],bindings=[];const effects=field(starts,counts,labels,targets,field(starts,counts,labels,targets,root,'main'),'effects');
 for(let at=0;at<(count(counts,effects));at+=1){const effect=readCollectionConstructionElement(starts,counts,targets,effects,at),shape=word(texts,starts,counts,labels,targets,effect,'k');
  if(shape==='let'){const value=field(starts,counts,labels,targets,effect,'value');state=expression(state,value,main,names,bindings,texts,starts,counts,labels,targets,markers,0);const actual=last(state);state=make(state,4,-1,kind(starts,counts,labels,targets,value),main);const binding=last(state);state=edge(state,binding,1,-1,actual);names=names.concat([field(starts,counts,labels,targets,effect,'name')]);bindings=bindings.concat([binding]);}
  else if(shape==='print'||shape==='output')state=expression(state,field(starts,counts,labels,targets,effect,'expr'),main,names,bindings,texts,starts,counts,labels,targets,markers,0);
  else if(shape==='assert')return [1];
  else return [1];
 }
 if((readRequiredNumberFact(row(state,16),0))!==0||row(state,17).length!==markers.length)return [1];
 let result=[0,row(state,0).length,row(state,4).length,row(state,8).length,row(state,9).length,main,row(state,17).length,row(state,19).length];
 for(let at=0;at<row(state,0).length;at+=1)result=result.concat([readRequiredNumberFact(row(state,0),at),readRequiredNumberFact(row(state,1),at),readRequiredNumberFact(row(state,2),at),readRequiredNumberFact(row(state,3),at)]);
 for(let at=0;at<row(state,4).length;at+=1)result=result.concat([readRequiredNumberFact(row(state,4),at),readRequiredNumberFact(row(state,5),at),readRequiredNumberFact(row(state,6),at),readRequiredNumberFact(row(state,7),at)]);
 for(const owner of row(state,8)){const position=row(state,11).indexOf(owner);result=result.concat([owner,position<0?-1:readRequiredNumberFact(row(state,12),position)]);}
 for(let at=0;at<row(state,9).length;at+=1)result=result.concat([readRequiredNumberFact(row(state,9),at),readRequiredNumberFact(row(state,10),at)]);
 for(let at=0;at<row(state,17).length;at+=1)result=result.concat([readRequiredNumberFact(row(state,17),at),readRequiredNumberFact(row(state,18),at)]);
 for(let at=0;at<row(state,19).length;at+=1)result=result.concat([readRequiredNumberFact(row(state,19),at),readRequiredNumberFact(row(state,20),at)]);
 return result;
}

/** @param {number[]} values @param {number} index @returns {number} */
function count(values,index){return index<0||index>=values.length?0:readRequiredNumberFact(values,index);}
