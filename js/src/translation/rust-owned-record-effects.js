import {testRustStandardCollectionSourceLoopNode,readRustStandardCollectionSourceLoopOrigin,lowerRustStandardCollectionSourceLoop} from './rust-standard-collection-source-loops.js';
// One ordered expression/state algebra. All states are actual immutable SSA
// snapshots in the common core; source ownership remains checked during read.
import {unsupported} from './diagnostics.js';
import {installRustOwnedRecordEffectTransport,readRustOwnedRecordStateAssignment,readRustOwnedRecordStateDeclarations,reconstructRustOwnedRecordState} from './rust-owned-record-state.js';
import {readRustOwnedRecordEffectForm,readRustOwnedRecordEffectChildren,readRustOwnedRecordEffectOperation,readRustOwnedRecordEffectLimit} from './rust-owned-record-effect-rules.js';
import {readRustExpressionCompletionPacketOperation,acceptRustExpressionCompletionInertLeaf,readRustExpressionCompletionShortCircuitValue} from './rust-expression-completion-rules.js';
import {readRustRecordSourceType} from './rust-records.js';
import {expandSurfacePropositionValue} from './proposition-values.js';
import {readRustOwnedRecordFiniteLoopSource,hasRustOwnedRecordLoopAdmission,lowerRustOwnedRecordFiniteLoop,appendRustOwnedRecordFiniteLoopHelpers} from './rust-owned-record-loops.js';
import {readRustOwnedRecordLoopOperation} from './rust-owned-record-loop-rules.js';
import {readRustExclusiveSourceWholeAssignment,readRustExclusiveSourceNode,testRustExclusiveSourcePrivateNode} from './rust-exclusive-source-calls.js';
import {readRustExclusiveSourceCompoundAssignment} from './rust-exclusive-source-compounds.js';
import {projectRustOwnedRecordStatePlace} from './rust-owned-record-state.js';
import {attachRustSourceStateTransition} from './rust-source-state-transitions.js';
import {testRustOwnedRecordFieldDescendant} from './rust-owned-record-update-rules.js';
import {retainRustSourceRegionState} from './rust-source-region-composition.js';
import {createRustSourcePlaceCallTransform} from './rust-source-place-transforms.js';
import {createRustSourcePlaceReplacement} from './rust-source-place-replacements.js';
const fail=(reason,span)=>{throw unsupported('Rust ordered record state',reason,span);};
function effects(parser){
 if(parser.ownedRecordEffects)return parser.ownedRecordEffects;
 const context={reads:[],writes:[],inspection:{nodes:0},work:0};parser.ownedRecordEffects=context;
 installRustOwnedRecordEffectTransport(parser,{
  read(slot,node){if(context.reads.length+context.writes.length>=readRustOwnedRecordEffectLimit('work'))fail('the actual state operations exceed their source budget',node.span);const index=context.reads.length;context.reads.push(slot);return parser.origins.mark({k:'math',op:readRustOwnedRecordEffectOperation('read'),name:String(index),args:[],span:node.span},slot.origin);},
  write(info){if(context.reads.length+context.writes.length>=readRustOwnedRecordEffectLimit('work'))fail('the actual state operations exceed their source budget',info.span);const index=context.writes.length;context.writes.push(info);return parser.origins.mark({k:'math',op:readRustOwnedRecordEffectOperation('write'),name:String(index),args:[{value:info.value,spread:false}],span:info.span},parser.origins.typeOrigin({kind:'unit'},info.path));},
 });return context;
}
export function initializeRustOwnedRecordEffects(parser){effects(parser);}
export function readRustOwnedRecordEffectExpression(parser,path,options,readOrdinary){effects(parser);return readRustExclusiveSourceCompoundAssignment(parser,path)??readRustExclusiveSourceWholeAssignment(parser,path)??readRustOwnedRecordStateAssignment(parser,path)?.value??readOrdinary();}
export function beginRustOwnedRecordShortCircuit(parser){return parser.ownedRecordEffects?parser.origins.snapshotOwnedBindings():null;}
export function endRustOwnedRecordShortCircuit(parser,before){if(before!==null)parser.origins.joinOwnedBindings(before,parser.origins.snapshotOwnedBindings(),true,true);}
export function readRustOwnedRecordEffectSource(parser,node){if(node?.k!=='math'||!parser.ownedRecordEffects)return null;const index=Number(node.name);if(!Number.isSafeInteger(index)||index<0||String(index)!==node.name)return null;if(node.op===readRustOwnedRecordEffectOperation('read')&&node.args.length===0){const slot=parser.ownedRecordEffects.reads[index];return slot?{form:'read',slot}:null;}if(node.op===readRustOwnedRecordEffectOperation('write')&&node.args.length===1){const source=parser.ownedRecordEffects.writes[index];return source?{form:'write',...source}:null;}return null;}
// Select from the real outer inventory, never from caller-supplied root names.
// Head-only values are evaluated separately by the ordered for-loop consumer.
// Unknown private operations retain the whole inventory rather than hiding an
// effect. Nested source loops are inspected through their actual operands.
export function selectRustOwnedRecordLoopStateSlots(parser,node,slots){
 const selected=new Set(),pending=[node],seen=new Set(),names=new Map(slots.map(slot=>[slot.currentName,slot.lexical]));
 while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;seen.add(value);if(seen.size>readRustOwnedRecordEffectLimit('inspection'))fail('loop state selection exceeds its complete source inspection budget',node?.span);
  if(value.sourceStateTransition)selected.add(value.sourceStateTransition.binder.identity);
  if(value.k==='name'&&value.path.length===1&&names.has(value.path[0]))selected.add(names.get(value.path[0]));
  if(marker(value)){
   const ordinary=readRustOwnedRecordEffectSource(parser,value),exclusive=readRustExclusiveSourceNode(parser,value);
   if(ordinary)selected.add(ordinary.slot.lexical);
   else if(exclusive?.kind==='write')selected.add(exclusive.info.slot.lexical);
   else if(exclusive?.kind==='call'){for(const borrow of exclusive.info.borrows)selected.add(borrow.slot.lexical);for(let index=0;index<value.args.length;index++)if(!exclusive.info.borrows.some(borrow=>borrow.index===index))pending.push(value.args[index].value);continue;}
   else if(!readRustStandardCollectionSourceLoopOrigin(parser,value)&&!readRustOwnedRecordFiniteLoopSource(parser,value))return slots;
  }
  for(const[key,child]of Object.entries(value))if(!['sourcePosition','sourceStateTransition','type'].includes(key)&&child&&typeof child==='object')pending.push(child);
 }
 return slots.filter(slot=>selected.has(slot.lexical));
}
// Exact outer-state write projection. Unknown private operations conservatively
// retain every state slot; controller metadata is never a caller admission flag.
function changedStateRoots(parser,node){
 const changed=new Set(),pending=[node],seen=new Set();while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;seen.add(value);if(seen.size>readRustOwnedRecordEffectLimit('inspection'))fail('state write projection exceeds its actual expression budget',node?.span);
  if(value.sourceStateTransition?.operation==='write')changed.add(value.sourceStateTransition.binder.identity);
  if(marker(value)){
   const ordinary=readRustOwnedRecordEffectSource(parser,value),exclusive=readRustExclusiveSourceNode(parser,value);
   if(ordinary?.form==='write')changed.add(ordinary.slot.lexical);
   else if(ordinary?.form==='read'){}
   else if(exclusive?.kind==='write')changed.add(exclusive.info.slot.lexical);
   else if(exclusive?.kind==='call'){for(const borrow of exclusive.info.borrows)changed.add(borrow.slot.lexical);for(let index=0;index<value.args.length;index++)if(!exclusive.info.borrows.some(borrow=>borrow.index===index))pending.push(value.args[index].value);continue;}
   else {const loop=readRustStandardCollectionSourceLoopOrigin(parser,value)??readRustOwnedRecordFiniteLoopSource(parser,value);if(!loop)return null;const body=readRustStandardCollectionSourceLoopOrigin(parser,value)?value.args[1].value:value;for(const slot of selectRustOwnedRecordLoopStateSlots(parser,body,loop.slots))changed.add(slot.lexical);}
  }
  for(const child of Object.values(value))if(child&&typeof child==='object')pending.push(child);
 }return changed;
}
// Source-derived overwrite liveness. Intervening effects remain in place;
// only a later complete assignment before any access can kill a join slot.
function killedBeforeNextAccess(parser,node,continuation,environment){
 if(!environment.size||!['if','match','match1','binary'].includes(node.k)||node.k==='binary'&&!['and','or'].includes(node.op))return null;
 const first=new Map(),killed=new Set();let work=0,complete=true;
 function accesses(expression){
  const found=new Set(),pending=[expression],seen=new Set();while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;seen.add(value);if(++work>readRustOwnedRecordEffectLimit('inspection'))fail('overwrite liveness exceeds its source inspection budget',node.span);
   if(value.k==='abort'||value.k==='rustStandardSumTry'||value.k==='math'&&readRustExpressionCompletionPacketOperation(value.op))return null;
   if(value.sourceStateTransition)found.add(value.sourceStateTransition.binder.identity);
   if(marker(value)){
    const ordinary=readRustOwnedRecordEffectSource(parser,value),exclusive=readRustExclusiveSourceNode(parser,value);
    if(ordinary)found.add(ordinary.slot.lexical);
    else if(exclusive?.kind==='write')found.add(exclusive.info.slot.lexical);
    else if(exclusive?.kind==='call'){for(const borrow of exclusive.info.borrows)found.add(borrow.slot.lexical);for(let index=0;index<value.args.length;index++)if(!exclusive.info.borrows.some(borrow=>borrow.index===index))pending.push(value.args[index].value);continue;}
    else {
     const ordered=readRustStandardCollectionSourceLoopOrigin(parser,value),loop=ordered??readRustOwnedRecordFiniteLoopSource(parser,value);
     if(!loop||!ordered&&!hasRustOwnedRecordLoopAdmission(loop)||loop.returns?.length)return null;
     // A borrowed iterator's lease is an access even when its item pattern
     // copies a scalar or ignores every element. The head/body operands below
     // supply ordinary dependencies; the actual lease supplies hidden loans.
     if(ordered&&!ordered.view.owned){const loan=ordered.view.loan;if(!loan)return null;const identities=new Set([loan.sourceBindingIdentity,loan.sourcePlaceBindingIdentity,...(loan.sourceValueBorrowBindings??[]),...(loan.sourceLoanBindingIdentities??[]),...(loan.sourcePlaceLoanBindingIdentities??[])].filter(Boolean));if(!identities.size)return null;for(const slot of ordered.slots)for(const identity of identities){if(++work>readRustOwnedRecordEffectLimit('inspection'))fail('iterator lease liveness exceeds its source inspection budget',node.span);if(identity===slot.root||testRustOwnedRecordFieldDescendant(slot.root,identity)){found.add(slot.lexical);break;}}}
    }
   }
   for(const child of Object.values(value))if(child&&typeof child==='object')pending.push(child);
  }return found;
 }
 let current=continuation,index=current.index??0,body=current.body,scopeComplete=false;const scopes=new Set(),contexts=new Set([current]);
 for(;;){
  const effect=current.effects?current.effects[index++]:body;
  if(!effect){scopeComplete=Boolean(current.effects||current.complete);if(!current.next)break;if(++work>readRustOwnedRecordEffectLimit('inspection'))fail('source continuation chain exceeds its finite budget',node.span);current=current.next;if(contexts.has(current))fail('source continuation descriptors cannot cycle',node.span);contexts.add(current);index=current.index??0;body=current.body;scopes.clear();continue;}
  if(!current.effects){if(scopes.has(effect))fail('a source lexical continuation cannot cycle',node.span);scopes.add(effect);body=['let','print'].includes(effect.k)?effect.body:null;}
  if(++work>readRustOwnedRecordEffectLimit('inspection'))fail('overwrite continuation exceeds its source inspection budget',node.span);
  const expression=effect.k==='let'?effect.value:effect.k==='print'?effect.expr:effect,write=readRustExclusiveSourceNode(parser,expression),whole=write?.kind==='write'&&write.info.fields.length===0&&write.info.layouts.length===0;
  const used=accesses(whole?expression.args[0].value:expression);if(used===null){complete=false;break;}
  for(const root of used)if(environment.has(root)&&!first.has(root))first.set(root,'access');
  if(whole){const root=write.info.slot.lexical;if(environment.has(root)&&!first.has(root)){first.set(root,'overwrite');if(environment.get(root).slot.exclusiveMode!=='exclusive')killed.add(root);}}
  if(first.size===environment.size)break;
 }
 if(complete&&scopeComplete)for(const[root,entry]of environment)if(!first.has(root)&&entry.slot.exclusiveMode!=='exclusive')killed.add(root);
 return killed.size?{node,roots:killed,continuation}:null;
}
function selectedStateKeys(parser,node,environment){const changed=changedStateRoots(parser,node);return [...environment.keys()].filter(key=>changed===null||changed.has(key));}
function requireUnchangedState(before,after,selected,node){const written=new Set(selected);if(before.size!==after.size||[...before].some(([key,prior])=>!after.has(key)||after.get(key).slot!==prior.slot||!written.has(key)&&after.get(key).name!==prior.name))fail('state write projection must retain every actual unchanged outer slot',node?.span);}
function marker(node){return testRustStandardCollectionSourceLoopNode(node)||testRustExclusiveSourcePrivateNode(node)||node?.k==='math'&&(node.op===readRustOwnedRecordEffectOperation('read')||node.op===readRustOwnedRecordEffectOperation('write')||node.op===readRustOwnedRecordLoopOperation());}
function contains(node,callables=false,budget={nodes:0}){const pending=[node],seen=new Set();while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;if(++budget.nodes>readRustOwnedRecordEffectLimit('inspection'))fail('the actual state expression exceeds its inspection budget',node?.span);seen.add(value);if(marker(value))return true;if(!callables&&value.k==='lambda')continue;for(const child of Object.values(value))if(child&&typeof child==='object')pending.push(child);}return false;}
function typeOf(parser,node,path,expected=null,seen=new Set()){
 if(expected)return expected;if(!node||seen.has(node))return null;seen.add(node);
 const origin=parser.origins.recordedExpressionOrigin(node)??parser.origins.recordedExpressionType(node)??parser.origins.origin(node,path);if(origin.proven&&origin.type)return readRustRecordSourceType(parser,origin.type,origin.path);
 if(node.k==='unit')return {kind:'unit'};if(node.k==='bool')return {kind:'bool'};if(node.k==='str')return {kind:'string'};if(node.k==='num'&&node.type)return node.type;
 if(node.k==='propositionValue')return {kind:'bool'};
 if(node.k==='lambda'&&node.ret&&node.params.every(parameter=>parameter.type))return {kind:'function',params:node.params.map(parameter=>parameter.type),ret:node.ret};
 if(node.k==='tuple'){const elements=node.items.map(value=>typeOf(parser,value,path,null,new Set(seen)));return elements.every(Boolean)?{kind:'tuple',elements}:null;}
 if(node.k==='tupleIndex'){const tuple=typeOf(parser,node.object,path,null,new Set(seen));return tuple?.kind==='tuple'?tuple.elements[node.index]??null:null;}
 if(node.k==='let'||node.k==='print')return typeOf(parser,node.body,path,null,seen);
 if(node.k==='cast')return node.to;
 if(['toString','show','stringMap'].includes(node.k))return {kind:'string'};
 if(['stringTest','booleanValue','arrayIsArray'].includes(node.k)||node.k==='binary'&&['eq','ne','lt','le','gt','ge','and','or'].includes(node.op)||node.k==='unary'&&node.op==='not')return {kind:'bool'};
 const callee=node.k==='app'?node.fn:node;
 if(callee?.k==='name')for(const entry of effects(parser).data??[]){const names=['crate',...entry.path,entry.declaration.name];if(callee.path.length!==names.length+1||!names.every((name,index)=>name===callee.path[index]))continue;const ctor=entry.declaration.ctors.find(ctor=>ctor.name===callee.path.at(-1));if(ctor&&ctor.fields.length===(node.k==='app'?node.args.length:0))return {kind:'named',path:names};}
 if(node.k==='if'){const first=typeOf(parser,node.then,path,null,new Set(seen)),second=typeOf(parser,node.else,path,null,new Set(seen));if(node.then.k==='abort')return second;if(node.else.k==='abort')return first;if(first&&JSON.stringify(first)===JSON.stringify(second))return first;}
 if(node.k==='match'||node.k==='match1'){let result=null;for(const row of node.k==='match'?node.rows:node.cases){if(row.body.k==='abort')continue;const actual=typeOf(parser,row.body,path,null,new Set(seen));if(!actual||result&&JSON.stringify(result)!==JSON.stringify(actual))return null;result=actual;}return result;}
 return null;
}
function childSlots(node,roles){const result=[];for(const role of roles){const value=node[role];if(Array.isArray(value))value.forEach((child,index)=>{const wrapped=child&&typeof child==='object'&&!child.k&&child.value&&typeof child.value==='object';result.push({role,index,child:wrapped?child.value:child,wrapper:wrapped?child:null});});else if(value&&typeof value==='object')result.push({role,index:null,child:value,wrapper:null});}return result;}
function alpha(parser,node,environment,declarations,path,budget){
 if(++budget.nodes>readRustOwnedRecordEffectLimit('work'))fail('state lexical renaming exceeds its source budget',node.span);
 const mark=value=>parser.origins.reuseExpressionOrigin(node,node.sourcePlaceTransform?{...value,sourcePlaceTransform:{...node.sourcePlaceTransform,slots:node.sourcePlaceTransform.slots.map(slot=>({...slot,input:environment.get(slot.input)??slot.input})),...(Array.isArray(node.sourcePlaceTransform.operands)?{operands:node.sourcePlaceTransform.operands.map(operand=>({...operand,input:environment.get(operand.input)??operand.input}))}:{})}}:value);
 if(node.k==='name')return node.path.length===1&&environment.has(node.path[0])?mark({...node,path:[environment.get(node.path[0])]}):node;
 if(node.k==='propositionValue'){if(!['eq','ne','lt','le','gt','ge'].includes(node.prop.p))return alpha(parser,expandSurfacePropositionValue(node.prop),environment,declarations,path,budget);return mark({...node,prop:{...node.prop,left:alpha(parser,node.prop.left,environment,declarations,path,budget),right:alpha(parser,node.prop.right,environment,declarations,path,budget)}});}
 if(node.k==='let'){const name=parser.freshProductBinding(),inner=new Map(environment);inner.set(node.name,name);const slot=readRustOwnedRecordStateDeclarations(parser).get(node.name);if(slot)declarations.set(name,slot);return mark({...node,name,value:alpha(parser,node.value,environment,declarations,path,budget),body:alpha(parser,node.body,inner,declarations,path,budget)});}
 if(node.k==='lambda'){const inner=new Map(environment),params=node.params.map(parameter=>{const name=parser.freshProductBinding();inner.set(parameter.name,name);return {...parameter,name};});return mark({...node,params,captures:(node.captures??[]).map(name=>environment.get(name)??name),body:alpha(parser,node.body,inner,declarations,path,budget)});}
 const renamePattern=(pattern,scope)=>{if(pattern.k==='bind'||pattern.k==='bindOrCtor'||pattern.k==='alias'||pattern.k==='natSucc'){const name=parser.freshProductBinding();scope.set(pattern.name,name);pattern={...pattern,name};}if(pattern.binds)pattern={...pattern,binds:pattern.binds.map(name=>{if(name===null)return null;const next=parser.freshProductBinding();scope.set(name,next);return next;})};if(pattern.args)pattern={...pattern,args:pattern.args.map(child=>renamePattern(child,scope))};if(pattern.pattern)pattern={...pattern,pattern:renamePattern(pattern.pattern,scope)};if(pattern.inner)pattern={...pattern,inner:renamePattern(pattern.inner,scope)};return pattern;};
 if(node.k==='match'||node.k==='match1'){const rows=node.k==='match'?node.rows:node.cases,changed=rows.map(row=>{const inner=new Map(environment),patterns=node.k==='match'?row.patterns.map(pattern=>renamePattern(pattern,inner)):null,pattern=node.k==='match1'?renamePattern(row.pattern,inner):null;return {...row,...(patterns?{patterns}:{pattern}),body:alpha(parser,row.body,inner,declarations,path,budget)};});return mark({...node,...(node.k==='match'?{scrutinees:node.scrutinees.map(value=>alpha(parser,value,environment,declarations,path,budget)),rows:changed}:{scrutinee:alpha(parser,node.scrutinee,environment,declarations,path,budget),cases:changed})});}
 const roles=node.k==='if'?['cond','then','else']:node.k==='print'?['expr','body']:readRustOwnedRecordEffectChildren(node.k),result={...node};for(const role of roles)if(Array.isArray(node[role]))result[role]=[...node[role]];for(const slot of childSlots(node,roles)){const value=alpha(parser,slot.child,environment,declarations,path,budget);if(slot.index===null)result[slot.role]=value;else result[slot.role][slot.index]=slot.wrapper?{...slot.wrapper,value}:value;}return mark(result);
}
function lower(parser,node,path,declarations,initial=new Map(),expected=null,finished=(value,environment)=>value,discarded=null,completeScope=false,sourceFuture=null){
 const joinKills=new WeakMap();if(discarded)joinKills.set(discarded.node,discarded.roots);
 const arena=effects(parser),has=(node,callables=false)=>contains(node,callables,arena.inspection);
 const mark=(value,original)=>{const result=parser.origins.reuseExpressionOrigin(original,value);if(original.sourcePosition&&!Object.hasOwn(result,'sourcePosition'))result.sourcePosition=original.sourcePosition;if(original.sourceStateTransition&&!Object.hasOwn(result,'sourceStateTransition'))result.sourceStateTransition=original.sourceStateTransition;return result;};
 function capture(value,origin,next){if(acceptRustExpressionCompletionInertLeaf(value.k))return next(value);const want=typeOf(parser,value,path,origin);if(!want)fail('an earlier evaluated operand needs its actual complete type before a later state effect',value.span);const name=parser.freshProductBinding(),reference=mark({k:'name',path:[name]},value);return {k:'let',name,type:want,value,body:next(reference),span:value.span};}
 function packet(value,environment,keys,resultType,span){const items=[value,...keys.map(key=>({k:'name',path:[environment.get(key).name]}))];return {k:'tuple',items,span};}
 function phi(original,environment,build,next,depth,want,join=original){const dead=joinKills.get(join)??new Set(),keys=selectedStateKeys(parser,original,environment).filter(key=>!dead.has(key)),resultType=typeOf(parser,original,path,want);if(!resultType)fail('a branch state join needs its actual complete result type or actual enclosing annotation',original.span);const tuple={kind:'tuple',elements:[resultType,...keys.map(key=>readRustRecordSourceType(parser,environment.get(key).slot.origin.type,environment.get(key).slot.origin.path))]},built=build((result,branch)=>{requireUnchangedState(environment,branch,[...keys,...dead],original);return packet(result,branch,keys,resultType,original.span);}),value={...built},name=parser.freshProductBinding(),reference={k:'name',path:[name]},bindings=[],joined=new Map(environment);delete value.sourcePosition;delete value.sourceStateTransition;for(let index=0;index<keys.length;index+=1){const fresh=parser.freshProductBinding();bindings.push({name:fresh,value:{k:'tupleIndex',object:reference,index:index+1}});joined.set(keys[index],{...environment.get(keys[index]),name:fresh});}let body=next(mark({k:'tupleIndex',object:reference,index:0},original),joined);for(const binding of bindings.toReversed())body={k:'let',...binding,body};return {k:'let',name,type:tuple,value,body,span:original.span};}
 function ordered(original,roles,environment,next,depth,want){const slots=childSlots(original,roles),selected=new Map();const step=(at,current)=>{if(at===slots.length){const rebuilt={...original};for(const role of roles)if(Array.isArray(original[role]))rebuilt[role]=[...original[role]];for(const [slot,value]of selected)if(slot.index===null)rebuilt[slot.role]=value;else rebuilt[slot.role][slot.index]=slot.wrapper?{...slot.wrapper,value}:value;return next(mark(rebuilt,original),current);}const slot=slots[at],later=slots.slice(at+1).some(item=>has(item.child));return visit(slot.child,current,(value,after)=>{const resume=actual=>{selected.set(slot,actual);return step(at+1,after);};return later?capture(value,null,resume):resume(value);},depth+1,null);};return step(0,environment);}
 function exclusiveCall(original,info,environment,next,depth){
  if(retainRustSourceRegionState(parser)&&info.borrows.some(borrow=>borrow.fields.length||borrow.payloadLayers>0))return projectedExclusiveCall(original,info,environment,next,depth);
  const arguments_=[],step=(index,current)=>{
   if(index<original.args.length){const borrow=info.borrows.find(row=>row.index===index);if(borrow){const root=current.get(borrow.slot.lexical);if(!root||root.slot.root!==borrow.slot.root)fail('an exclusive argument needs its actual live caller state slot',original.span);const selected=projectRustOwnedRecordStatePlace(parser,borrow.layouts,{k:'name',path:[root.name]},original.span);return capture(selected,readRustRecordSourceType(parser,borrow.field.type,borrow.field.path),held=>{arguments_.push(held);return step(index+1,current);});}
    return visit(original.args[index].value,current,(value,after)=>{if((info.parameters[index].sourceFnMutParameter||info.parameters[index].sourceCompilerCallback)&&value.k==='lambda'){arguments_.push(value);return step(index+1,after);}return capture(value,readRustRecordSourceType(parser,info.parameters[index].type,info.parameters[index].path),held=>{arguments_.push(held);return step(index+1,after);});},depth+1,info.parameters[index].type);
   }
   const packetName=parser.freshProductBinding(),packet={k:'name',path:[packetName]},bindings=[],updated=new Map(current),type={kind:'tuple',elements:[readRustRecordSourceType(parser,info.result.type,info.result.path),...info.borrows.map(row=>readRustRecordSourceType(parser,row.field.type,row.field.path))]};
   for(let at=0;at<info.borrows.length;at++){const borrow=info.borrows[at],prior=updated.get(borrow.slot.lexical),name=parser.freshProductBinding(),replacement={k:'tupleIndex',object:packet,index:at+1},value=reconstructRustOwnedRecordState(parser,path,borrow.layouts,{k:'name',path:[prior.name]},replacement,original.span);bindings.push({name,type:readRustRecordSourceType(parser,borrow.slot.origin.type,borrow.slot.origin.path),value});updated.set(borrow.slot.lexical,{slot:prior.slot,name});}
   let body=next(mark({k:'tupleIndex',object:packet,index:0},original),updated);for(const binding of bindings.toReversed())body={k:'let',...binding,body};return {k:'let',name:packetName,type,value:{k:'app',fn:{k:'name',path:info.callee},args:arguments_,span:original.span},body,span:original.span};
  };return step(0,environment);
 }
 function projectedExclusiveCall(original,info,environment,next,depth){
  const args=[],step=(index,current)=>{
   if(index<original.args.length){if(info.borrows.some(borrow=>borrow.index===index)){args.push(null);return step(index+1,current);}return visit(original.args[index].value,current,(value,after)=>{if(info.parameters[index].sourceFnMutParameter||info.parameters[index].sourceCompilerCallback){if(value.k!=='lambda')fail('a projected selected callback retains its actual inline source body',original.span);args.push(value);return step(index+1,after);}const local=parser.freshProductBinding(),reference=parser.origins.reuseExpressionOrigin(value,{k:'name',path:[local],...(value.sourcePosition?{sourcePosition:value.sourcePosition}:{})});args.push(reference);return {k:'let',name:local,type:readRustRecordSourceType(parser,info.parameters[index].type,info.parameters[index].path),value,body:step(index+1,after)};},depth+1,info.parameters[index].type);}
   const transformed=createRustSourcePlaceCallTransform(parser,info,current,args),packet=parser.freshProductBinding(),reference={k:'name',path:[packet]},updated=new Map(current),bindings=transformed.roots.map((root,index)=>{const local=parser.freshProductBinding();updated.set(root.slot.lexical,{slot:root.slot,name:local});return {name:local,value:{k:'tupleIndex',object:reference,index:index+1}};});let body=next(mark({k:'tupleIndex',object:reference,index:0},original),updated);for(const binding of bindings.toReversed())body={k:'let',...binding,body};return {k:'let',name:packet,value:transformed.node,body,span:original.span};
  };return step(0,environment);
 }
 function projectedIntrinsicWrite(original,info,environment,next,depth){
  const operation=info.intrinsic.operation,value=original.args[0].value,operand=operation==='clear string'?null:operation==='append string text'&&value.k==='binary'&&value.op==='concat'?value.right:operation==='append vector element'&&value.k==='array'&&value.items.length===2&&value.items[0].spread===true&&value.items[1].spread===false?value.items[1].value:null;
  if(operation!=='clear string'&&!operand)fail('a projected intrinsic retains its actual source operand and standard construction',original.span);
  const complete=(after,held)=>{const prior=after.get(info.slot.lexical);if(!prior||prior.slot.root!==info.slot.root)fail('a projected replacement retains its actual latest source root',original.span);const packet=parser.freshProductBinding(),current=parser.freshProductBinding(),updated=new Map(after);updated.set(info.slot.lexical,{slot:prior.slot,name:current});return {k:'let',name:packet,value:createRustSourcePlaceReplacement(parser,info,prior,held),body:{k:'let',name:current,value:{k:'tupleIndex',object:{k:'name',path:[packet]},index:1},body:next(mark({k:'tupleIndex',object:{k:'name',path:[packet]},index:0},original),updated)},span:original.span};};
  if(!operand)return complete(environment,null);return visit(operand,environment,(actual,after)=>{const local=parser.freshProductBinding();return {k:'let',name:local,type:readRustRecordSourceType(parser,info.intrinsic.operand.type,info.intrinsic.operand.path),value:actual,body:complete(after,local)};},depth+1,info.intrinsic.operand.type);
 }
 function visit(value,environment,next,depth,want,future=null){if(++arena.work>readRustOwnedRecordEffectLimit('work')||depth>readRustOwnedRecordEffectLimit('depth'))fail('ordered state construction exceeds its finite source budget',value.span);
  if(future&&!joinKills.has(value)){const dead=killedBeforeNextAccess(parser,value,future,environment);if(dead)joinKills.set(value,dead.roots);}
  if(value.k==='lambda'){const body=visit(value.body,new Map(),result=>result,depth+1,value.ret);return next(mark({...value,body},value),environment);}
  if(!has(value,true))return next(value,environment);
  const exclusive=readRustExclusiveSourceNode(parser,value);
  if(exclusive?.kind==='call')return exclusiveCall(value,exclusive.info,environment,next,depth);
  if(exclusive?.kind==='write'){
   const info=exclusive.info,prior=environment.get(info.slot.lexical);if(!prior||prior.slot.root!==info.slot.root)fail('exclusive whole-place writes retain their actual lexical state slot',value.span);
   if(info.intrinsic&&(info.fields.length||info.intrinsic.referent)&&retainRustSourceRegionState(parser))return projectedIntrinsicWrite(value,info,environment,next,depth);
   return visit(value.args[0].value,environment,(rhs,after)=>capture(rhs,readRustRecordSourceType(parser,info.field.type,info.field.path),held=>{const name=parser.freshProductBinding(),current=after.get(info.slot.lexical),snapshot=attachRustSourceStateTransition(parser,reconstructRustOwnedRecordState(parser,path,info.layouts,{k:'name',path:[current.name]},held,value.span),info.slot,'write'),updated=new Map(after);updated.set(info.slot.lexical,{slot:current.slot,name});return {k:'let',name,type:readRustRecordSourceType(parser,info.slot.origin.type,info.slot.origin.path),value:snapshot,body:next({k:'unit',span:value.span},updated)};}),depth+1,info.field.type);
  }
  if(testRustExclusiveSourcePrivateNode(value))fail('an exclusive source private operation lacks its actual selected parser continuation',value.span);
  if(value.sourceRecordPlace&&value.k==='match'&&value.scrutinees.length===1)return visit(value.scrutinees[0],environment,(owner,after)=>next(mark({...value,scrutinees:[owner]},value),after),depth+1,value.sourceRecordPlace.root.type);
  if(readRustStandardCollectionSourceLoopOrigin(parser,value))return lowerRustStandardCollectionSourceLoop(parser,value,environment,next,(source,initial,expected,finished)=>{const renamed=alpha(parser,source,new Map(),declarations,path,arena.inspection);return lower(parser,renamed,path,declarations,initial,expected,finished);});
  if(readRustOwnedRecordFiniteLoopSource(parser,value))return lowerRustOwnedRecordFiniteLoop(parser,value,environment,next,(source,initial,expected,finished)=>{const renamed=alpha(parser,source,new Map(),declarations,path,arena.inspection);return lower(parser,renamed,path,declarations,initial,expected,finished);});
  const form=readRustOwnedRecordEffectForm(value.k,value.op??'');
  if(form==='proposition comparison')return ordered({k:'tuple',items:[value.prop.left,value.prop.right]},['items'],environment,(pair,after)=>next(mark({...value,prop:{...value.prop,left:pair.items[0],right:pair.items[1]}},value),after),depth,null);
  if(form==='read state'){const slot=arena.reads[Number(value.name)],actual=slot?environment.get(slot.lexical):null;if(!actual)fail('a mutable read needs its actual initialized lexical state in this callable',value.span);return next(mark({k:'name',path:[actual.name]},value),environment);}
  if(form==='write state'){const info=arena.writes[Number(value.name)];if(!info||!environment.has(info.slot.lexical))fail('a mutable write cannot leave its actual lexical declaration scope',value.span);return visit(value.args[0].value,environment,(rhs,after)=>{const previous=after.get(info.slot.lexical),rhsName=parser.freshProductBinding(),stateName=parser.freshProductBinding(),updated=new Map(after);updated.set(info.slot.lexical,{slot:info.slot,name:stateName});const snapshot=attachRustSourceStateTransition(parser,reconstructRustOwnedRecordState(parser,path,info.layouts,{k:'name',path:[previous.name]},{k:'name',path:[rhsName]},value.span),info.slot,'write'),body=next({k:'unit',span:value.span},updated);return {k:'let',name:rhsName,type:readRustRecordSourceType(parser,info.field.type,info.field.path),value:rhs,body:{k:'let',name:stateName,type:readRustRecordSourceType(parser,info.slot.origin.type,info.slot.origin.path),value:snapshot,body},span:value.span};},depth+1,info.field.type);}
  if(form==='binding'){
   const continuation={body:value.body,next:future},discarded=killedBeforeNextAccess(parser,value.value,continuation,environment),previous=joinKills.get(value.value);if(discarded)joinKills.set(value.value,discarded.roots);
   try{return visit(value.value,environment,(rhs,after)=>{const inner=new Map(after),slot=declarations.get(value.name);if(slot)inner.set(slot.lexical,{slot,name:value.name});return mark({...value,value:rhs,body:visit(value.body,inner,(result,done)=>{const outer=new Map(done);if(slot)outer.delete(slot.lexical);return next(result,outer);},depth+1,want,future)},value);},depth+1,value.type??null,continuation);}finally{if(discarded){if(previous)joinKills.set(value.value,previous);else joinKills.delete(value.value);}}
  }
  if(form==='print')return visit(value.expr,environment,(expr,after)=>{
   const body=visit(value.body,after,(result,done)=>{
    if(!value.sourcePosition&&!value.sourceStateTransition)return next(result,done);
    const type=typeOf(parser,value.body,path)??typeOf(parser,result,path);if(!type)fail('a print result retains its complete source type before continuation',value.span);
    const name=parser.freshProductBinding(),held=parser.origins.reuseExpressionOrigin(result,{k:'name',path:[name],span:value.span});
    return {k:'let',name,type,value:result,body:next({...held,...(value.sourcePosition?{sourcePosition:value.sourcePosition}:{}),...(value.sourceStateTransition?{sourceStateTransition:value.sourceStateTransition}:{})},done),span:value.span};
   },depth+1,want,future),printed=mark({...value,expr,body},value);delete printed.sourcePosition;delete printed.sourceStateTransition;return printed;
  },depth+1,null);
  if(form==='conditional')return visit(value.cond,environment,(condition,after)=>phi(value,after,branch=>mark({...value,cond:condition,then:visit(value.then,new Map(after),branch,depth+1,want,future),else:visit(value.else,new Map(after),branch,depth+1,want,future)},value),next,depth,want),depth+1,{kind:'bool'});
  if(form==='short circuit')return visit(value.left,environment,(left,after)=>phi(value,after,branch=>{const short=readRustExpressionCompletionShortCircuitValue(value.op),done=branch({k:'bool',value:short},new Map(after)),right=visit(value.right,new Map(after),branch,depth+1,{kind:'bool'},future);return mark({k:'if',cond:left,then:short?done:right,else:short?right:done,span:value.span},value);},next,depth,{kind:'bool'}),depth+1,{kind:'bool'});
  if(form==='match')return ordered(value,value.k==='match1'?['scrutinee']:['scrutinees'],environment,(selected,after)=>phi(value,after,branch=>value.k==='match1'?mark({...selected,cases:value.cases.map(row=>({...row,body:visit(row.body,new Map(after),branch,depth+1,want,future)}))},value):mark({...selected,rows:value.rows.map(row=>({...row,body:visit(row.body,new Map(after),branch,depth+1,want,future)}))},value),next,depth,want,value),depth,null);
  if(form==='packet'){const operation=readRustExpressionCompletionPacketOperation(value.op);if(operation==='result')return packetVisit(value.args[0].value,1,environment,next,after=>visit(value.args[1].value,after,next,depth+1,want),depth+1,want);if(operation==='success')return visit(value.args[0].value,environment,(child,after)=>next(mark({...value,args:[{...value.args[0],value:child}]},value),after),depth+1,null);}
  const roles=readRustOwnedRecordEffectChildren(value.k);if(roles.length)return ordered(value,roles,environment,next,depth,want);
  fail('the actual source expression form needs its complete ordered state roles',value.span);
 }
 function packetVisit(value,levels,environment,next,failed,depth,want){if(++arena.work>readRustOwnedRecordEffectLimit('work')||depth>readRustOwnedRecordEffectLimit('depth'))fail('conditional state packets exceed their source budget',value.span);const role=value.k==='math'?readRustExpressionCompletionPacketOperation(value.op):'';if(role==='success')return levels===1?visit(value.args[0].value,environment,next,depth+1,want):packetVisit(value.args[0].value,levels-1,environment,next,failed,depth+1,want);if(role==='failure')return failed(environment);if(role==='result')return packetVisit(value.args[0].value,levels+1,environment,next,after=>packetVisit(value.args[1].value,levels,after,next,failed,depth+1,want),depth+1,want);if(value.k==='if')return visit(value.cond,environment,(cond,after)=>mark({...value,cond,then:packetVisit(value.then,levels,new Map(after),next,failed,depth+1,want),else:packetVisit(value.else,levels,new Map(after),next,failed,depth+1,want)},value),depth+1,{kind:'bool'});if(value.k==='let')return visit(value.value,environment,(rhs,after)=>{const inner=new Map(after),slot=declarations.get(value.name);if(slot)inner.set(slot.lexical,{slot,name:value.name});return mark({...value,value:rhs,body:packetVisit(value.body,levels,inner,(result,done)=>{const outer=new Map(done);if(slot)outer.delete(slot.lexical);return next(result,outer);},failed,depth+1,want)},value);},depth+1,value.type??null);if(value.k==='match'||value.k==='match1')return ordered(value,value.k==='match1'?['scrutinee']:['scrutinees'],environment,(selected,after)=>value.k==='match1'?mark({...selected,cases:value.cases.map(row=>({...row,body:packetVisit(row.body,levels,new Map(after),next,failed,depth+1,want)}))},value):mark({...selected,rows:value.rows.map(row=>({...row,body:packetVisit(row.body,levels,new Map(after),next,failed,depth+1,want)}))},value),depth,null);fail('the conditional state packet needs its actual complete producer',value.span);}
 return visit(node,initial,finished,0,expected,sourceFuture??discarded?.continuation??(completeScope?{body:null,complete:true}:null));
}
function lowerMain(parser,main,budget){
 const inventory=new Map(),names=new Map(),output=[];let environment=new Map(),followingEffects={effects:main.effects,index:0};
 const rename=node=>alpha(parser,node,names,inventory,[],budget);
 // Main retains its effect algebra. A value/state packet completes before
 // the following effect, including the original structural assertion.
 const evaluate=(node,expected=null)=>{
  const discarded=killedBeforeNextAccess(parser,node,followingEffects,environment),keys=selectedStateKeys(parser,node,environment).filter(key=>!discarded?.roots.has(key));
  if(keys.length===0)return lower(parser,node,[],inventory,environment,expected,(result,after)=>{requireUnchangedState(environment,after,keys,node);return result;},discarded,false,followingEffects);
  const packetName=parser.freshProductBinding(),reference={k:'name',path:[packetName]};
  const value=lower(parser,node,[],inventory,environment,expected,(result,after)=>{requireUnchangedState(environment,after,keys,node);return {k:'tuple',items:[result,...keys.map(key=>({k:'name',path:[after.get(key).name]}))]};},discarded,false,followingEffects);
  const resultType=typeOf(parser,node,[],expected),type=resultType?{kind:'tuple',elements:[resultType,...keys.map(key=>readRustRecordSourceType(parser,environment.get(key).slot.origin.type,environment.get(key).slot.origin.path))]}:null;
  output.push({k:'let',name:packetName,...(type?{type}:{}),value});
  const actual=parser.origins.origin(node,[]),selected=parser.origins.mark({k:'tupleIndex',object:reference,index:0,...(node.sourcePosition?{sourcePosition:node.sourcePosition}:{})},actual),after=new Map(environment);
  keys.forEach((key,index)=>{const name=parser.freshProductBinding();output.push({k:'let',name,value:{k:'tupleIndex',object:reference,index:index+1}});after.set(key,{...environment.get(key),name});});environment=after;return selected;
 };
 const pureRead=node=>{
  if(marker(node)){if(node.op!==readRustOwnedRecordEffectOperation('read'))fail('a lazy assertion operand needs its explicit ordered source continuation',node.span);const slot=effects(parser).reads[Number(node.name)],actual=slot&&environment.get(slot.lexical);if(!actual)fail('an assertion read needs its actual initialized lexical state',node.span);return parser.origins.mark({k:'name',path:[actual.name]},parser.origins.origin(node,[]));}
  if(!node||typeof node!=='object')return node;
  if(Array.isArray(node))return node.map(pureRead);
  return Object.fromEntries(Object.entries(node).map(([key,value])=>[key,value&&typeof value==='object'?pureRead(value):value]));
 };
 const assertion=prop=>{
  if(prop.p==='bool')return {...prop,expr:evaluate(rename(prop.expr),{kind:'bool'})};
  if(['eq','ne','lt','le','gt','ge'].includes(prop.p)){
   const left=rename(prop.left),right=rename(prop.right),leftType=typeOf(parser,left,[]),rightType=typeOf(parser,right,[]),type=leftType??rightType;
   const pair=evaluate({k:'tuple',items:[left,right]},type?{kind:'tuple',elements:[type,type]}:null);
   return {...prop,left:{k:'tupleIndex',object:pair,index:0},right:{k:'tupleIndex',object:pair,index:1}};
  }
  // Assertions have their own structural equality and quantifier semantics.
  // Pure state reads can remain in that algebra; writes under logical or
  // quantified propositions await a proposition continuation consumer.
  if(hasWrite(prop))return {p:'bool',expr:evaluate(rename(propositionExpression(prop)),{kind:'bool'})};return pureRead(renameProp(prop));
 };
 const hasWrite=node=>{const pending=[node];let count=0;while(pending.length){const value=pending.pop();if(!value||typeof value!=='object')continue;if(++count>readRustOwnedRecordEffectLimit('inspection'))fail('the assertion exceeds its source inspection budget');if(marker(value)&&value.op===readRustOwnedRecordEffectOperation('write'))return true;pending.push(...Object.values(value));}return false;};
 const propositionExpression=expandSurfacePropositionValue;
 const renameProp=prop=>prop.p==='bool'?{...prop,expr:rename(prop.expr)}:['eq','ne','lt','le','gt','ge'].includes(prop.p)?{...prop,left:rename(prop.left),right:rename(prop.right)}:prop.p==='not'?{...prop,arg:renameProp(prop.arg)}:['and','or','implies'].includes(prop.p)?{...prop,left:renameProp(prop.left),right:renameProp(prop.right)}:prop;
 for(const [effectIndex,effect]of main.effects.entries()){
  followingEffects={effects:main.effects,index:effectIndex+1};
  if(effect.k==='let'){
   const old=effect.name,name=parser.freshProductBinding(),slot=readRustOwnedRecordStateDeclarations(parser).get(old),value=evaluate(rename(effect.value),effect.type??null);
   output.push({...effect,name,value});names.set(old,name);if(slot){inventory.set(name,slot);environment.set(slot.lexical,{slot,name});}
  }else if(effect.k==='print')output.push({...effect,expr:evaluate(rename(effect.expr))});
  else if(effect.k==='assert')output.push({...effect,prop:assertion(effect.prop)});
  else fail('the actual main effect needs its existing ordered consumer',effect.span);
 }
 return {...main,effects:output};
}
// Opt-in adapters use this same state algebra; they cannot supply replacement
// runtime semantics. The caller owns source checks and the complete parameter
// state environment, and consumes all resulting state through its continuation.
export function lowerRustOwnedRecordEffectBody(parser,node,path,declarations,initial,expected,finished){
 if(!parser?.origins||!parser.ownedRecordEffects||!(declarations instanceof Map)||!(initial instanceof Map)||!Array.isArray(path)||path.some(part=>typeof part!=='string')||typeof finished!=='function')fail('an adapted body needs the actual parser state inventory and completion continuation',node?.span);
 const renamed=alpha(parser,node,new Map(),declarations,path,{nodes:0});return lower(parser,renamed,path,declarations,initial,expected,finished,null,true);
}
export function finishRustOwnedRecordEffects(parser,program){
 if(parser.origins.collecting&&parser.ownedRecordFiniteLoops?.entries.some(row=>row.provisional))return program;
 if(!parser.ownedRecordEffects||!contains(program,true))return program;const inventory=new Map(),budget={nodes:0};
 const data=[];const collect=(items,path)=>{for(const item of items)if(item.k==='module')collect(item.items,[...path,item.name]);else if(item.k==='data')data.push({path,declaration:item});};collect(program.items,[]);effects(parser).data=data;
 const expression=(node,path,expected=null)=>{if(!contains(node,true))return node;const renamed=alpha(parser,node,new Map(),inventory,path,budget);return lower(parser,renamed,path,inventory,new Map(),expected,undefined,null,true);};
 const items=(values,path)=>values.map(item=>item.k==='module'?{...item,items:items(item.items,[...path,item.name])}:item.k==='fn'?{...item,body:expression(item.body,path,item.ret)}:item);
 const result=appendRustOwnedRecordFiniteLoopHelpers(parser,{...program,items:items(program.items,[]),...(program.main?{main:lowerMain(parser,program.main,budget)}:{})});
 if(contains(result,true))fail('a private record state operation escaped its ordered consumer',program.main?.span);
 return result;
}
