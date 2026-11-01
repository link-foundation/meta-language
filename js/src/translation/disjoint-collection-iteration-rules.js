import {readRequiredNumberFact,readRequiredTextFact} from './fact-column-rules.js';
// Source allocation sites, lexical binding edges and actual callable bodies
// decide noninterference. Names alone never denote distinct allocations.
import {readReferenceMethodResult} from './reference-method-rules.js';
import {readArrayCallbackResultForm} from './array-callback-rules.js';
import {readArrayStateMethodResult} from './array-state-method-rules.js';
import {readCollectionAccessResult} from './collection-access-rules.js';
/** @returns {string} */
export function readDisjointCollectionIterationSourceOperation(){return 'readClosedStandardCollectionIteration';}
/** @returns {string} */
export function readDisjointCollectionSpreadSourceOperation(){return 'readClosedStandardCollectionSpread';}
/** @param {number} ordinal @param {string} operation @returns {string} */
export function readDisjointCollectionIterationMarker(ordinal,operation){return '#closed collection iteration '+String(ordinal)+' '+operation;}
/** @param {number} ordinal @param {string} operation @returns {string} */
export function readDisjointCollectionSpreadMarker(ordinal,operation){return '#closed collection spread '+String(ordinal)+' '+operation;}
/** @param {string} operation @returns {string} */
export function readDisjointCollectionIterationMarkerRole(operation){return operation.startsWith('#closed collection iteration ')?'loop':operation.startsWith('#closed collection spread ')?'eager':'';}
/** Factory receivers and arbitrary computed getters need separate source
 * evaluation/provenance support; actual owned names and fields are enough for
 * the current own-shaped spread anchors. @param {string} shape @returns {boolean} */
export function acceptDisjointCollectionSpreadReceiver(shape){return shape==='name'||shape==='field';}
/** @param {boolean} known @param {number} externals @param {number} literals
 * @param {number} bindings @param {boolean} directory @param {boolean} publication
 * @returns {boolean} */
export function acceptDisjointCollectionIterationScope(known,externals,literals,bindings,directory,publication){return known&&externals===0&&literals===0&&bindings===0&&!directory&&!publication;}
/** Retained values-only admission; wider source item roles delegate to it.
 * @param {string} kind @param {string} operation @returns {boolean} */
export function acceptDisjointCollectionIterationOperation(kind,operation){return (kind==='map'||kind==='set')&&operation==='values';}
/** The initial values-only decision remains exact. The wider eager profile
 * consumes the same established collection item roles as ordinary access.
 * @param {string} kind @param {string} operation @returns {string} */
export function readDisjointCollectionIterationItemForm(kind,operation){
 if(acceptDisjointCollectionIterationOperation(kind,operation))return 'values';
 const form=readCollectionAccessResult(kind,operation);return form==='keys'||form==='entries'?form:'';
}
/** @param {string} kind @returns {boolean} */
export function readDisjointCollectionIterationIdentityKind(kind){return ['array','map','set','record','tuple','data','function','cell','option'].includes(kind);}
/** @param {string} kind @param {string} operation @returns {string} */
export function readDisjointCollectionIterationMethodEffect(kind,operation){
 if(readReferenceMethodResult(kind,operation)!==''&&operation!=='get'&&operation!=='has')return 'write';
 if(kind==='map'||kind==='set')return ['has','keys','values','entries'].includes(operation)||kind==='map'&&operation==='get'?'read':'unknown';
 if(kind==='array'){
  if(['sort','splice','fill','copyWithin'].includes(operation))return 'write';
  return readDisjointCollectionIterationCallback(operation)||['slice','concat','join','includes','indexOf','lastIndexOf','at','flat','toReversed','toSorted'].includes(operation)?'read':'unknown';
 }
 if(kind==='record')return readDisjointCollectionIterationWriteField(operation)>=0?'write':operation==='readJavaScriptRecordPropertyOrder'?'read':'unknown';
 // Scalar builtins are lowered to their checked scalar expression forms.
 // A newly registered method never inherits purity from its result type.
 return 'unknown';
}
/** Structural checked operations inspect their already evaluated operands.
 * Opaque Math operations intentionally have no default purity certificate.
 * @param {string} shape @returns {boolean} */
export function acceptDisjointCollectionIterationReadExpression(shape){return ['let','if','match','print','coalesce','some','flattenOption','unwrapOption','lit','unit','none','abort','impossible','lambda','cellRead','collectionSize','collectionRead','unary','binary','nullishNumber','optionalIndexNumber','truthy','strictEqual','optionalTypeOf','optionalText','optionalPredicate','optionalField','optionalProject','optionalIndex','stringPoints','toString','stringMap','stringTest','cast','append','index','length'].includes(shape);}
/** @param {string} operation @returns {number} */
export function readDisjointCollectionIterationWriteField(operation){
 const prefix='writeRecordField';if(!operation.startsWith(prefix))return -1;
 const text=operation.slice(prefix.length),index=Number(text);return Number.isInteger(index)&&index>=0&&String(index)===text?index:-1;
}
/** @param {string} operation @returns {boolean} */
export function readDisjointCollectionIterationCallback(operation){return readArrayCallbackResultForm(operation)!==''||operation==='sort'||operation==='toSorted';}
/** @param {string} role @returns {boolean} */
export function acceptDisjointCollectionIterationCallbackHead(role){return role==='function'||role==='function reference'||role==='variable';}
/** @param {number[]} facts @param {number} fact @returns {number[]} */
function addFact(facts,fact){return readRequiredNumberFact(facts,0)===-1||facts.includes(fact)?facts:facts.concat([fact]);}
/** @param {number[]} values @param {number} value @returns {number[]} */
function addValue(values,value){return values.includes(value)?values:values.concat([value]);}
/** @param {number[]} facts @param {number} tag @param {number} node
 * @param {number} width @param {number} span @returns {number[]} */
function readRoots(facts,tag,node,width,span){let result=[];for(const fact of facts)if(Math.floor(fact/span)===tag){const value=fact%span;if(Math.floor(value/width)===node)result=addValue(result,value%width-1);}return result;}
/** @param {number[]} facts @param {number} tag @param {number} node
 * @param {number[]} roots @param {number} width @param {number} span @returns {number[]} */
function addRoots(facts,tag,node,roots,width,span){let present=readRoots(facts,tag,node,width,span);for(const root of roots)present=addValue(present,root);if(present.length>32)return [-1];let result=facts;for(const root of roots)result=addFact(result,tag*span+node*width+root+1);return result;}
/** @param {number[]} starts @param {number[]} counts @param {string[]} labels
 * @param {number[]} targets @param {number} node @param {string} label @returns {number[]} */
function edges(starts,counts,labels,targets,node,label){let result=[];for(let at=readRequiredNumberFact(starts,node);at<readRequiredNumberFact(starts,node)+readRequiredNumberFact(counts,node);at+=1)if(textFact(labels,at)===label)result=result.concat([readRequiredNumberFact(targets,at)]);return result;}
/** @param {number[]} facts @param {number[]} roots @param {number} field
 * @param {number} fieldWidth @param {number} width @param {number} span @returns {number[]} */
function readField(facts,roots,field,fieldWidth,width,span){let result=[];for(const root of roots){if(root<0){result=addValue(result,-1);continue;}for(const fact of facts)if(Math.floor(fact/span)===2){const value=fact%span,owner=Math.floor(value/(fieldWidth*width)),actual=Math.floor(value/width)%fieldWidth;if(owner===root&&(actual===0||actual===field))result=addValue(result,value%width-1);}}return result;}
/** @param {number[]} facts @param {number[]} owners @param {number} field
 * @param {number[]} roots @param {number} fieldWidth @param {number} width
 * @param {number} span @returns {number[]} */
function addField(facts,owners,field,roots,fieldWidth,width,span){let result=facts;for(const owner of owners)if(owner>=0)for(const root of roots)result=addFact(result,2*span+(owner*fieldWidth+field)*width+root+1);return result;}
/** @param {number[]} facts @param {number[]} roots @param {number} destination
 * @param {number} fieldWidth @param {number} width @param {number} span @returns {number[]} */
function copyFields(facts,roots,destination,fieldWidth,width,span){let result=facts;for(const root of roots){if(root<0){result=addField(result,[destination],0,[-1],fieldWidth,width,span);continue;}for(const fact of facts)if(Math.floor(fact/span)===2){const value=fact%span;if(Math.floor(value/(fieldWidth*width))===root)result=addFact(result,2*span+destination*fieldWidth*width+value%(fieldWidth*width));}}return result;}
/** @param {number[]} facts @param {number} source @param {number} width
 * @param {number} span @returns {number[]} */
function called(facts,source,width,span){return readRoots(facts,3,source,width,span);}
/** @param {number[]} facts @param {number[]} initial @param {number} width
 * @param {number} span @returns {number[]} */
function reachable(facts,initial,width,span){let result=initial,progress=true;while(progress){progress=false;for(const owner of result)for(const target of called(facts,owner,width,span))if(!result.includes(target)){result=result.concat([target]);progress=true;}}return result;}
/** @param {number[]} left @param {number[]} right @returns {boolean} */
function separateRoots(left,right){return left.length>0&&right.length>0&&!left.includes(-1)&&!right.includes(-1)&&!left.some(root=>right.includes(root));}
/** @param {number[]} facts @param {string[]} roles @param {string[]} operations @param {string[]} kinds
 * @param {number[]} owners @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number[]} functions
 * @param {number[]} bodies @param {number[]} scopes @param {number} source
 * @param {boolean} allocation @param {number} main @param {number} width
 * @param {number} span @returns {boolean} */
function acceptsEffects(facts,roles,operations,kinds,owners,starts,counts,labels,targets,functions,bodies,scopes,source,allocation,main,width,span){
 const roots=readRoots(facts,1,source,width,span);
 if(allocation&&(roots.length===0||roots.some(root=>root<0||textFact(roles,root)!=='allocation'||textFact(kinds,root)!==textFact(kinds,source))))return false;
 const reached=reachable(facts,scopes,width,span);
 for(const scope of reached){const position=functions.indexOf(scope);if(position<0||readRequiredNumberFact(bodies,position)<0&&scope!==main)return false;}
 for(let node=0;node<roles.length;node+=1)if(reached.includes(readRequiredNumberFact(owners,node))){
  // Cell replacement can redirect an aliased receiver through another frame;
  // this first profile does not model temporal cell versions. Refuse it even
  // when the cell container and the collection have different checked kinds.
  if(textFact(roles,node)==='unknown'||textFact(roles,node)==='cell write')return false;
  if(textFact(roles,node)==='call'||textFact(roles,node)==='apply'){
   const actual=edges(starts,counts,labels,targets,node,'callee');let callees=[];
   if(textFact(roles,node)==='call')callees=actual;else if(actual.length)callees=readRoots(facts,1,readRequiredNumberFact(actual,0),width,span);
   if(callees.length===0||callees.some(target=>!functions.includes(target)))return false;
  }
  if(textFact(roles,node)==='method'){
   const object=firstNumber(edges(starts,counts,labels,targets,node,'object')),effect=readDisjointCollectionIterationMethodEffect(textFact(kinds,object),textFact(operations,node));
   if(effect==='unknown')return false;
   if(effect==='write'&&!['array','map','set','record','cell'].includes(textFact(kinds,object)))return false;
   if(effect==='write'&&(textFact(kinds,object)===textFact(kinds,source)&&!separateRoots(roots,readRoots(facts,1,object,width,span))))return false;
   const arguments_=edges(starts,counts,labels,targets,node,'argument');
   if(readDisjointCollectionIterationCallback(textFact(operations,node))&&arguments_.length){const callees=readRoots(facts,1,readRequiredNumberFact(arguments_,0),width,span);if(callees.length===0||callees.some(target=>!functions.includes(target)))return false;}
  }
 }
 return true;
}
/** @param {number[]} facts @param {number} owner @param {number[]} callees
 * @param {number[]} arguments_ @param {string} callback @param {number} receiver
 * @param {string[]} kinds @param {number[]} functions @param {number[]} parameterStarts
 * @param {number[]} parameterCounts @param {number[]} parameters @param {number[]} bodies
 * @param {number} main @param {number} width @param {number} span @returns {number[]} */
function bindCalls(facts,owner,callees,arguments_,callback,receiver,kinds,functions,parameterStarts,parameterCounts,parameters,bodies,main,width,span){
 let result=facts;
 for(const callee of callees){
  const position=functions.indexOf(callee);if(position<0)continue;
  result=addFact(result,4*span+callee);result=addRoots(result,3,owner,[callee],width,span);
  if(readRequiredNumberFact(bodies,position)<0&&callee!==main)result=addRoots(result,5,callee,[-1],width,span);
  for(let at=0;at<readRequiredNumberFact(parameterCounts,position);at+=1){
   const parameter=readRequiredNumberFact(parameters,readRequiredNumberFact(parameterStarts,position)+at);let roots=[];
   if(callback==='')roots=at<arguments_.length?readRoots(result,1,readRequiredNumberFact(arguments_,at),width,span):[];
   else if(readDisjointCollectionIterationIdentityKind(textFact(kinds,parameter))){
    if(callback==='reduce'&&at===0){roots=arguments_.length>1?readRoots(result,1,readRequiredNumberFact(arguments_,1),width,span):[];for(const root of readRoots(result,5,callee,width,span))roots=addValue(roots,root);}
    else if(at===(callback==='reduce'?3:2))roots=readRoots(result,1,receiver,width,span);
    else roots=[-1];
   }
   result=addRoots(result,1,parameter,roots,width,span);
  }
 }return result;
}
/** Bounded finite monotone reference/effect proof. The relation table uses
 * integer tags: 1 value aliases, 2 actual record fields, 3 calls, 4 reachable
 * callable bodies, 5 callable results. Unknown container contents stay -1.
 * Results: [error, snapshot count, snapshot nodes..., callback count, nodes...].
 * Error 1 is a resource/shape boundary, 2 unproved loop noninterference.
 * @param {string[]} roles @param {string[]} operations @param {string[]} kinds
 * @param {number[]} owners @param {number[]} starts @param {number[]} counts
 * @param {string[]} labels @param {number[]} targets @param {number[]} functions
 * @param {number[]} parameterStarts @param {number[]} parameterCounts
 * @param {number[]} parameters @param {number[]} bodies @param {number} main
 * @param {number[]} snapshots @param {number[]} callbacks @returns {number[]} */
export function proveDisjointCollectionIterations(roles,operations,kinds,owners,starts,counts,labels,targets,functions,parameterStarts,parameterCounts,parameters,bodies,main,snapshots,callbacks){
 const size=roles.length,width=size+1,fieldWidth=labels.length+1,span=size*fieldWidth*width+1;
 if(size<1||size>4096||labels.length>32768||operations.length!==size||kinds.length!==size||owners.length!==size||starts.length!==size||counts.length!==size||labels.length!==targets.length||functions.length!==parameterStarts.length||functions.length!==parameterCounts.length||functions.length!==bodies.length||!functions.includes(main)||snapshots.length%2!==0||callbacks.length%2!==0)return [1];
 for(let node=0;node<size;node+=1)if(!Number.isInteger(readRequiredNumberFact(starts,node))||!Number.isInteger(readRequiredNumberFact(counts,node))||!Number.isInteger(readRequiredNumberFact(owners,node))||readRequiredNumberFact(starts,node)<0||readRequiredNumberFact(counts,node)<0||readRequiredNumberFact(starts,node)+readRequiredNumberFact(counts,node)>targets.length||readRequiredNumberFact(owners,node)<-1||readRequiredNumberFact(owners,node)>=size)return [1];
 if(targets.some(target=>!Number.isInteger(target)||target<0||target>=size)||parameters.some(target=>!Number.isInteger(target)||target<0||target>=size))return [1];
 for(let at=0;at<functions.length;at+=1)if(!Number.isInteger(readRequiredNumberFact(functions,at))||!Number.isInteger(readRequiredNumberFact(parameterStarts,at))||!Number.isInteger(readRequiredNumberFact(parameterCounts,at))||!Number.isInteger(readRequiredNumberFact(bodies,at))||readRequiredNumberFact(functions,at)<0||readRequiredNumberFact(functions,at)>=size||readRequiredNumberFact(parameterStarts,at)<0||readRequiredNumberFact(parameterCounts,at)<0||readRequiredNumberFact(parameterStarts,at)+readRequiredNumberFact(parameterCounts,at)>parameters.length||readRequiredNumberFact(bodies,at)<-1||readRequiredNumberFact(bodies,at)>=size)return [1];
 if(snapshots.some(node=>!Number.isInteger(node)||node<0||node>=size)||callbacks.some(node=>!Number.isInteger(node)||node<0||node>=size))return [1];
 let facts=[4*span+main],work=0,settled=false,seeded=false;
 for(let pass=0;pass<128&&!settled;pass+=1){
  const before=facts.length;
  for(let node=0;node<size;node+=1){
   work+=(facts.length+1)*(readRequiredNumberFact(counts,node)+4);if(readRequiredNumberFact(facts,0)===-1||work>268435456||facts.length>65536)return [1];
   if(textFact(roles,node)==='function')facts=addRoots(facts,1,node,[node],width,span);
   if(!facts.includes(4*span+readRequiredNumberFact(owners,node)))continue;
   const role=textFact(roles,node),object=firstNumber(edges(starts,counts,labels,targets,node,'object')),value=firstNumber(edges(starts,counts,labels,targets,node,'value')),arguments_=edges(starts,counts,labels,targets,node,'argument');
   let roots=[];
   if(role==='allocation'||role==='record'||role==='array'||role==='tuple'||role==='data'||role==='snapshot'||role==='eager snapshot')roots=[node];
   if(role==='variable'||role==='binding'||role==='read')roots=value<0?[]:readRoots(facts,1,value,width,span);
   if(role==='field'){const actual=readRoots(facts,1,object,width,span);work+=(facts.length+1)*actual.length;if(work>268435456)return [1];roots=readField(facts,actual,labels.indexOf(textFact(operations,node))+1,fieldWidth,width,span);}
   if(role==='function reference')roots=edges(starts,counts,labels,targets,node,'callee');
   if(role==='value')for(const child of edges(starts,counts,labels,targets,node,'result'))for(const root of readRoots(facts,1,child,width,span))roots=addValue(roots,root);
   if(role==='call'||role==='apply'){
    const callee=edges(starts,counts,labels,targets,node,'callee');let actual=callee;
    if(role==='apply')actual=callee.length?readRoots(facts,1,readRequiredNumberFact(callee,0),width,span):[];
    facts=bindCalls(facts,readRequiredNumberFact(owners,node),actual,arguments_,'',-1,kinds,functions,parameterStarts,parameterCounts,parameters,bodies,main,width,span);
    for(const target of actual)for(const root of readRoots(facts,5,target,width,span))roots=addValue(roots,root);
   }
   if(role==='method'){
    if(readReferenceMethodResult(textFact(kinds,object),textFact(operations,node))==='receiver'||textFact(kinds,object)==='array'&&(readArrayStateMethodResult(textFact(operations,node))==='receiver'||textFact(operations,node)==='sort'))roots=readRoots(facts,1,object,width,span);
    // A method result is not a fresh allocation merely because it has Array
    // type (reduce/find/at can return existing nested handles). Actual fresh
    // referenceAllocate nodes establish new outer identities independently.
    else if(readDisjointCollectionIterationIdentityKind(textFact(kinds,node)))roots=[-1];
    if(readDisjointCollectionIterationCallback(textFact(operations,node))&&arguments_.length)facts=bindCalls(facts,readRequiredNumberFact(owners,node),readRoots(facts,1,readRequiredNumberFact(arguments_,0),width,span),arguments_,textFact(operations,node),object,kinds,functions,parameterStarts,parameterCounts,parameters,bodies,main,width,span);
   }
   if((role==='unknown'||role==='opaque read')&&readDisjointCollectionIterationIdentityKind(textFact(kinds,node)))roots=[-1];
   if(role!=='function'&&readDisjointCollectionIterationIdentityKind(textFact(kinds,node)))facts=addRoots(facts,1,node,roots,width,span);
   if(role==='allocation'){const actual=readRoots(facts,1,value,width,span);work+=(facts.length+1)*actual.length;if(work>268435456)return [1];facts=copyFields(facts,actual,node,fieldWidth,width,span);}
   if(role==='record'||role==='tuple'||role==='data'||role==='method')for(let at=readRequiredNumberFact(starts,node);at<readRequiredNumberFact(starts,node)+readRequiredNumberFact(counts,node);at+=1)if(textFact(labels,at).startsWith('field:')||textFact(labels,at).startsWith('item:')){
    const actual=role==='method'?readRoots(facts,1,object,width,span):[node];facts=addField(facts,actual,labels.indexOf(textFact(labels,at))+1,readRoots(facts,1,readRequiredNumberFact(targets,at),width,span),fieldWidth,width,span);
   }
  }
  for(let at=0;at<functions.length;at+=1)if(readRequiredNumberFact(bodies,at)>=0&&facts.includes(4*span+readRequiredNumberFact(functions,at)))facts=addRoots(facts,5,readRequiredNumberFact(functions,at),readRoots(facts,1,readRequiredNumberFact(bodies,at),width,span),width,span);
  if(facts.length===before){
   if(!seeded){seeded=true;for(const list of [snapshots,callbacks])for(let at=0;at<list.length;at+=2){const owner=readRequiredNumberFact(owners,readRequiredNumberFact(list,at));if(owner>=0&&!facts.includes(4*span+owner)){facts=addFact(facts,4*span+owner);const position=functions.indexOf(owner);if(position>=0)for(let parameter=readRequiredNumberFact(parameterStarts,position);parameter<readRequiredNumberFact(parameterStarts,position)+readRequiredNumberFact(parameterCounts,position);parameter+=1)if(readDisjointCollectionIterationIdentityKind(textFact(kinds,readRequiredNumberFact(parameters,parameter))))facts=addRoots(facts,1,readRequiredNumberFact(parameters,parameter),[-1],width,span);}}}
   settled=facts.length===before;
  }
 }
 if(!settled)return [1];
 let accepted=[];
 for(let at=0;at<snapshots.length;at+=2){
  const snapshot=readRequiredNumberFact(snapshots,at),source=readRequiredNumberFact(snapshots,at+1);let aliases=[snapshot],scopes=[];
  // A registered array spread consumes the standard iterator synchronously at
  // this operand. No callback or later sibling runs inside that consumption.
  // It needs actual allocation provenance, not a fabricated callable loop.
  if(textFact(roles,snapshot)==='eager snapshot'){
   if(!acceptsEffects(facts,roles,operations,kinds,owners,starts,counts,labels,targets,functions,bodies,[],source,true,main,width,span))return [2,snapshot];
   accepted=accepted.concat([snapshot]);continue;
  }
  for(let node=0;node<size;node+=1)if(textFact(roles,node)==='allocation'){const child=firstNumber(edges(starts,counts,labels,targets,node,'value'));if(child>=0&&readRoots(facts,1,child,width,span).includes(snapshot))aliases=addValue(aliases,node);}
  for(let node=0;node<size;node+=1)if(textFact(roles,node)==='call'||textFact(roles,node)==='apply'){
   const actual=edges(starts,counts,labels,targets,node,'argument');if(actual.some(argument=>readRoots(facts,1,argument,width,span).some(root=>aliases.includes(root)))){
    const targets_=edges(starts,counts,labels,targets,node,'callee'),callees=textFact(roles,node)==='call'?targets_:targets_.length?readRoots(facts,1,readRequiredNumberFact(targets_,0),width,span):[];for(const callee of callees)scopes=addValue(scopes,callee);
   }
  }
  if(scopes.length===0||!acceptsEffects(facts,roles,operations,kinds,owners,starts,counts,labels,targets,functions,bodies,scopes,source,true,main,width,span))return [2,snapshot];
  accepted=accepted.concat([snapshot]);
 }
 let safe=[];
 for(let at=0;at<callbacks.length;at+=2){
  const node=readRequiredNumberFact(callbacks,at),source=readRequiredNumberFact(callbacks,at+1),callback=firstNumber(edges(starts,counts,labels,targets,node,'argument'));
  if(callback<0||!acceptDisjointCollectionIterationCallbackHead(textFact(roles,callback)))continue;
  const scopes=readRoots(facts,1,callback,width,span);
  if(scopes.length&&acceptsEffects(facts,roles,operations,kinds,owners,starts,counts,labels,targets,functions,bodies,scopes,source,false,main,width,span))safe=safe.concat([node]);
 }
 return [0,accepted.length].concat(accepted,[safe.length],safe);
}

/** @param {number[]} values @returns {number} */
function firstNumber(values){return values.length===0?-1:readRequiredNumberFact(values,0);}
/** @param {string[]} values @param {number} index @returns {string} */
function textFact(values,index){return index<0||index>=values.length?'':readRequiredTextFact(values,index);}
