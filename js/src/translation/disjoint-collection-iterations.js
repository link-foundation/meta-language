// Host transport only: lexical, allocation and effect decisions are shared rules.
import {createSurfaceReferenceFacts} from './surface-reference-facts.js';
import {unsupported} from './diagnostics.js';
import {projectDisjointCollectionIterationFacts,readDisjointCollectionProjectionRole,readDisjointCollectionProjectionOperation,readDisjointCollectionProjectionKind,readDisjointCollectionProjectionEdge} from './disjoint-collection-projection-rules.js';
import {proveDisjointCollectionIterations,readDisjointCollectionIterationMarkerRole} from './disjoint-collection-iteration-rules.js';

export function readProgramDisjointCollectionIterationProof(program,markers=[]){
 const facts=createSurfaceReferenceFacts({declarations:[...program.declarations.values()],main:program.main});
 const plan=projectDisjointCollectionIterationFacts(facts.texts,facts.starts,facts.counts,facts.labels,facts.targets,facts.root,markers);
 if(plan[0]!==0)return {error:1,arrayCallbacks:false,collectionMappers:false};
 const [,size,edgeCount,functionCount,parameterCount,main,snapshotCount,callbackCount]=plan;
 let at=8;const roles=[],operations=[],kinds=[],owners=[],edgeRows=[],functions=[],bodies=[],parameterRows=[],snapshots=[],callbacks=[];
 for(let node=0;node<size;node+=1){const role=plan[at++],operation=plan[at++],kind=plan[at++];roles.push(readDisjointCollectionProjectionRole(role));operations.push(readDisjointCollectionProjectionOperation(role,operation,facts.texts[operation]??''));kinds.push(readDisjointCollectionProjectionKind(role,facts.texts[kind]??''));owners.push(plan[at++]);}
 for(let edge=0;edge<edgeCount;edge+=1){const source=plan[at++],label=plan[at++],auxiliary=plan[at++],target=plan[at++];edgeRows.push({source,label:readDisjointCollectionProjectionEdge(label,auxiliary,facts.texts[auxiliary]??''),target});}
 for(let fn=0;fn<functionCount;fn+=1){functions.push(plan[at++]);bodies.push(plan[at++]);}
 for(let parameter=0;parameter<parameterCount;parameter+=1)parameterRows.push({owner:plan[at++],slot:plan[at++]});
 for(let snapshot=0;snapshot<snapshotCount; snapshot+=1)snapshots.push(plan[at++],plan[at++]);
 for(let callback=0;callback<callbackCount;callback+=1)callbacks.push(plan[at++],plan[at++]);
 const starts=[],counts=[],labels=[],targets=[],parameterStarts=[],parameterCounts=[],parameters=[];
 for(let node=0;node<size;node+=1){starts.push(labels.length);for(const edge of edgeRows)if(edge.source===node){labels.push(edge.label);targets.push(edge.target);}counts.push(labels.length-starts[node]);}
 for(const fn of functions){parameterStarts.push(parameters.length);for(const parameter of parameterRows)if(parameter.owner===fn)parameters.push(parameter.slot);parameterCounts.push(parameters.length-parameterStarts[parameterStarts.length-1]);}
 const result=proveDisjointCollectionIterations(roles,operations,kinds,owners,starts,counts,labels,targets,functions,parameterStarts,parameterCounts,parameters,bodies,main,snapshots,callbacks);
 if(result[0]!==0)return {error:result[0],arrayCallbacks:false,collectionMappers:false};
 const safe=result.slice(3+result[1]);let arrayCallbacks=true,collectionMappers=true;
 for(let callback=0;callback<callbacks.length;callback+=2)if(!safe.includes(callbacks[callback])){if(kinds[callbacks[callback+1]]==='array')arrayCallbacks=false;else collectionMappers=false;}
 return {error:0,arrayCallbacks,collectionMappers};
}

// Markers originate solely at the source checker seam. They must all pass the
// actual final program proof before they become ordinary collection snapshots.
export function checkDisjointCollectionIterations(program,markers){
 if(markers.length===0)return;
 const proof=readProgramDisjointCollectionIterationProof(program,markers.map(marker=>marker.marker));
 if(proof.error)throw unsupported('closed collection iteration','the actual lowered loop needs bounded, known callable effects and disjoint collection allocations');
 const pending=[program],seen=new Set(),found=new Set();
 while(pending.length){const node=pending.pop();if(!node||typeof node!=='object'||seen.has(node))continue;seen.add(node);
  if(node.k==='collectionItems'&&typeof node.op==='string'&&readDisjointCollectionIterationMarkerRole(node.op)!==''){
   const marker=markers.find(marker=>marker.marker===node.op);if(!marker)throw unsupported('closed collection iteration','an unregistered private collection marker cannot escape checking');
   found.add(marker.marker);node.op=marker.operation;
  }
  pending.push(...(node instanceof Map?[...node.values()]:Object.entries(node).filter(([key])=>!['type','domain','span','modulePath'].includes(key)).map(([,value])=>value)));
 }
 if(found.size!==markers.length)throw unsupported('closed collection iteration','every source marker needs an actual checked loop consumer');
}
