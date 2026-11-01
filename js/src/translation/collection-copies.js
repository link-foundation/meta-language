// The existing solver owns all type terms; these are mechanical eager snapshot
// transports, not iterator implementations or replacement collection runtimes.
import {unsupported} from './diagnostics.js';
import {acceptReadOnlyReferenceCallback} from './reference-callbacks.js';
import {readCollectionCopyInferenceRole,readCollectionCopySourceNormalization,acceptCollectionCopyMapperSnapshot,readCollectionCopyMapperError} from './collection-copy-rules.js';
function settle(inference,constraint,settled){
 const input=inference.resolve?inference.resolve(constraint.input):constraint.input;
 const role=readCollectionCopyInferenceRole(constraint.kind,input?.kind??'',settled);
 if(role==='wait')return false;
 if(role==='copy'){
  inference.unify(constraint.input,constraint.result,constraint.span);
  inference.collectionCopies.set(constraint.node,constraint.snapshot);
 }else inference.unify(constraint.input,constraint.array,constraint.span);
 return true;
}
export function inferCollectionCopyInput(inference,node,input,expectedArray,result,snapshot){
 const constraint={node,kind:node.k,input,array:expectedArray,result,snapshot,span:node.span};
 if(!settle(inference,constraint,false))inference.collectionConstructionInputs.push(constraint);
}
export function solveCollectionCopyInputs(inference,settled=false){
 let progress=false;const waiting=[];
 for(const constraint of inference.collectionConstructionInputs){if(settle(inference,constraint,settled))progress=true;else waiting.push(constraint);}
 inference.collectionConstructionInputs=waiting;return progress;
}
export function normalizeCollectionCopyInputs(inference){
 for(const[node,snapshot]of inference.collectionCopies){
  const field=node.k==='set'?'items':'entries',object=node[field],type=inference.surface(snapshot,node.span);
  const wrapper={k:readCollectionCopySourceNormalization(),args:[object],type,span:object.span??node.span};
  node[field]=wrapper;inference.expressionTypes.set(wrapper,type);
 }
}
export function checkCollectionCopySnapshotCallbacks(program){
 const pending=[program],seen=new Set();
 while(pending.length){
  const node=pending.pop();if(!node||typeof node!=='object'||seen.has(node))continue;seen.add(node);
  const snapshot=node.k==='method'&&node.op==='map'&&node.object?.k==='collectionItems'?node.object:null;
  if(snapshot&&acceptCollectionCopyMapperSnapshot(snapshot.object.type.kind,snapshot.op)){
   const callback=node.args[0],error=readCollectionCopyMapperError(true,acceptReadOnlyReferenceCallback(callback,program.declarations));if(error)throw unsupported('collection snapshot callback',error,callback?.span??node.span);
  }
  pending.push(...(node instanceof Map?[...node.values()]:Object.entries(node).filter(([key])=>!['type','domain','span','modulePath'].includes(key)).map(([,value])=>value)));
 }
}
