// Source-owned finite-loop admission tokens. The reader adds a token only
// after checking actual counter syntax, progress and every continuing exit.
import {unsupported} from './diagnostics.js';
import {readRustOwnedRecordStateSlots,readRustOwnedRecordStateLayout} from './rust-owned-record-state.js';
import {readRustOwnedRecordEffectSource,selectRustOwnedRecordLoopStateSlots} from './rust-owned-record-effects.js';
import {readRustOwnedRecordLoopOperation,readRustOwnedRecordLoopCounterError,readRustOwnedRecordLoopProgressError,readRustOwnedRecordLoopWriteError,readRustOwnedRecordLoopInvariantError,readRustOwnedRecordLoopExpansionError,readRustOwnedRecordLoopExpansionReservation,readRustOwnedRecordLoopChunkName,readRustOwnedRecordLoopChunkSize,planRustOwnedRecordLoopIterations,readRustOwnedRecordLoopExitDisposition} from './rust-owned-record-loop-rules.js';
import {readRustOwnedRecordStateFieldIdentity} from './rust-owned-record-state-rules.js';
import {testRustOwnedRecordFieldDescendant} from './rust-owned-record-update-rules.js';
import {rustOwnedRecordFieldState} from './rust-owned-record-updates.js';
import {withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {sealFiniteRecordLoopHelper} from './finite-record-loop-helpers.js';
import {readRustRecordSourceType} from './rust-records.js';
import {readRustOrdinaryCompletionOwnershipSnapshot,cloneRustOrdinaryCompletionOwnershipSnapshot,readRustOrdinaryLoopCompletion} from './rust-ordinary-completions.js';
import {readRustOwnedRecordEffectChildren} from './rust-owned-record-effect-rules.js';
import {acceptRustExpressionCompletionInertLeaf,readRustExpressionCompletionPacketOperation} from './rust-expression-completion-rules.js';
import {measureRustStandardSumSourceMetadata} from './rust-standard-sum-source-metadata.js';
import {readRustStandardSumSourceMetadataReservation} from './rust-standard-sum-source-metadata-rules.js';
import {readRustOwnedRecordNestedLoopError,readRustOwnedRecordNestedLoopDepthError,readRustOwnedRecordNestedCompletionError,readRustOwnedRecordLoopFootprintError,readRustOwnedRecordLoopResidualCountError} from './rust-owned-record-loop-rules.js';
import {readRustStandardSumTrySourceBranches} from './rust-standard-sum-completions.js';
import {hasRustExpressionCompletion} from './rust-expression-completions.js';
import {attachRustFiniteLoopStateTransform} from './rust-source-state-transforms.js';
const admitted=new WeakSet();
export function hasRustOwnedRecordLoopAdmission(value){return admitted.has(value);}
export function assertRustOwnedRecordLoopAdmission(value){if(!hasRustOwnedRecordLoopAdmission(value))throw unsupported('finite record loop helper','the helper needs its actual admitted source counter and exit-state derivation',value?.span);}
const fail=(reason,span)=>{throw unsupported('Rust finite record loop',reason,span);};
function inventory(parser){return parser.ownedRecordFiniteLoops??={entries:[],helpers:[],nodes:0};}
export function readRustOwnedRecordFiniteLoopSource(parser,node){if(node?.k!=='math'||node.op!==readRustOwnedRecordLoopOperation()||node.args?.length!==2)return null;const ordinal=Number(node.name);return Number.isSafeInteger(ordinal)&&ordinal>=0&&String(ordinal)===node.name?parser.ownedRecordFiniteLoops?.entries[ordinal]??null:null;}
function numeral(node,value){return node?.k==='num'&&String(node.value)===String(value)&&(!node.type||node.type.kind==='fixed'&&node.type.bits===8&&!node.type.signed);}
// A source field place must be the actual catalog projection over an actual
// state read. An immutable alias that happens to retain a place ID is not a
// progress-counter read.
function place(parser,node){
 const source=readRustOwnedRecordEffectSource(parser,node);if(source?.form==='read')return {slot:source.slot,indices:[],origin:source.slot.origin};
 if(node?.k!=='match'||node.scrutinees.length!==1||node.rows.length!==1)return null;const row=node.rows[0],pattern=row.patterns?.[0];if(row.patterns?.length!==1||pattern?.k!=='ctor'||row.body?.k!=='name'||row.body.path.length!==1)return null;
 const parent=place(parser,node.scrutinees[0]);if(!parent)return null;const actual=readRustOwnedRecordStateLayout(parser,parent.origin);if(!actual||pattern.path.join('.')!=='crate.'+actual.layout.constructor||pattern.args.length!==actual.entry.fields.length)return null;
 if(pattern.args.some(pattern=>!['bind','wild'].includes(pattern.k)))return null;const names=pattern.args.map(pattern=>pattern.k==='bind'?pattern.name:null),bound=names.filter(name=>name!==null);if(new Set(bound).size!==bound.length)return null;const index=names.indexOf(row.body.path[0]);return index<0?null:{slot:parent.slot,indices:[...parent.indices,index],origin:actual.entry.fields[index]};
}
function counter(parser,guard){if(guard?.k!=='binary'||!numeral(guard.right,0))fail('the loop head needs its actual pure counter comparison with zero',guard?.span);const selected=place(parser,guard.left);if(!selected||selected.indices.length===0)fail('the loop head needs an actual mutable record field place',guard.span);const origin=selected.origin,owner=origin.expandedSourceOwner??origin.sourceOwner,error=readRustOwnedRecordLoopCounterError(origin.type?.kind??'',origin.type?.bits??0,origin.type?.signed===true,origin.wrappers.length,owner?.kind==='primitive'&&owner.arguments.length===0?owner.path.join('::'):'',guard.op);if(!origin.proven||error)fail(error||'the loop counter needs its actual complete source proof',guard.span);return {...selected,target:readRustOwnedRecordStateFieldIdentity(selected.slot.root,selected.indices)};}
function samePlace(left,right){return left&&left.slot.lexical===right.slot.lexical&&left.indices.length===right.indices.length&&left.indices.every((index,at)=>index===right.indices[at]);}
function overlapping(left,right){return left===right||testRustOwnedRecordFieldDescendant(left,right)||testRustOwnedRecordFieldDescendant(right,left);}
function writeFootprint(parser,node){const arena=inventory(parser),nodesBefore=arena.footprintNodes??0,textBefore=arena.footprintUnits??0,result=new Set(),pending=[node];let work=0,text=0;const add=target=>{if(result.has(target))return;const error=readRustOwnedRecordLoopFootprintError(nodesBefore+result.size+1,textBefore+text+target.length);if(error)fail(error,node.span);text+=target.length;result.add(target);};while(pending.length){const value=pending.pop();if(!value||typeof value!=='object')continue;if(++work>100000)fail('the complete loop write footprint exceeds its source budget',node.span);const nested=readRustOwnedRecordFiniteLoopSource(parser,value);if(nested){assertRustOwnedRecordLoopAdmission(nested);for(const target of nested.writes)add(target);continue;}const source=readRustOwnedRecordEffectSource(parser,value);if(source?.form==='write')add(source.target);for(const child of Object.values(value))if(child&&typeof child==='object')pending.push(child);}arena.footprintNodes=nodesBefore+result.size;arena.footprintUnits=textBefore+text;return [...result].sort();}
function counts(parser,node,selected,budget){
 if(++budget.nodes>100000)fail('the complete record loop exits exceed their finite source budget',node?.span);
 const nested=readRustOwnedRecordFiniteLoopSource(parser,node);if(nested){const error=readRustOwnedRecordNestedLoopError(admitted.has(nested),nested.returns?.length??-1,nested.writes?.some(target=>overlapping(target,selected.target))??true);if(error)fail(error,node.span);return [0];}
 const source=readRustOwnedRecordEffectSource(parser,node);
 if(source?.form==='write'){
  const overlaps=source.target===selected.target||testRustOwnedRecordFieldDescendant(source.target,selected.target)||testRustOwnedRecordFieldDescendant(selected.target,source.target),rhs=node.args[0].value;
  const exact=source.target===selected.target&&rhs?.k==='binary'&&rhs.op==='sub'&&numeral(rhs.right,1)&&samePlace(place(parser,rhs.left),selected),error=readRustOwnedRecordLoopWriteError(overlaps,exact);if(error)fail(error,node.span);
  return counts(parser,rhs,selected,budget).map(count=>count+(exact?1:0));
 }
 if(node?.k==='lambda'||node?.k==='math'&&readRustExpressionCompletionPacketOperation(node.op))fail('nested callable and packet effects need their separate progress derivation',node.span);
 const add=(left,right)=>{if(left.length*right.length>256)fail('the complete record loop exits exceed their finite source budget',node.span);return left.flatMap(first=>right.map(second=>first+second));};
 if(node?.k==='if')return add(counts(parser,node.cond,selected,budget),[...counts(parser,node.then,selected,budget),...counts(parser,node.else,selected,budget)]);
 if(node?.k==='binary'&&['and','or'].includes(node.op))return add(counts(parser,node.left,selected,budget),[0,...counts(parser,node.right,selected,budget)]);
 if(node?.k==='match'||node?.k==='match1'){let result=[0];for(const value of node.k==='match'?node.scrutinees:[node.scrutinee])result=add(result,counts(parser,value,selected,budget));const branches=(node.k==='match'?node.rows:node.cases).flatMap(row=>counts(parser,row.body,selected,budget));return add(result,branches);}
 if(node?.k==='rustStandardSumTry'||node?.k==='propositionValue')fail('this loop body needs its separate residual or quantified exit derivation',node.span);
 let result=[0];for(const value of expressionChildren(node))result=add(result,counts(parser,value,selected,budget));return result;
}
function expressionChildren(node){if(!node||typeof node!=='object')return [];switch(node.k){case'let':return[node.value,node.body];case'print':return[node.expr,node.body];case'app':return[node.fn,...node.args];case'binary':return[node.left,node.right];case'unary':case'cast':return[node.arg];case'tuple':case'array':return node.items.map(item=>item?.k?item:item.value);case'tupleIndex':case'field':return[node.object];case'math':return node.args.map(arg=>arg.value);case'name':case'num':case'str':case'bool':case'unit':return[];default:{const roles=readRustOwnedRecordEffectChildren(node.k);if(!roles.length)fail('the loop body needs complete ordered source roles',node.span);return roles.flatMap(role=>Array.isArray(node[role])?node[role].map(child=>child?.k?child:child.value):[node[role]]);}}}
function boundNames(pattern,result){if(['bind','bindOrCtor','alias','natSucc'].includes(pattern?.k))result.add(pattern.name);for(const child of pattern?.args??[])boundNames(child,result);if(pattern?.pattern)boundNames(pattern.pattern,result);}
function closed(parser,node,path,bound=new Set(),budget={nodes:0}){if(++budget.nodes>100000)fail('the loop capture inventory exceeds its source budget',node?.span);if(node?.k==='rustStandardSumTry'){if(!readRustStandardSumTrySourceBranches(parser,node))fail('the loop residue needs its actual parsed question-mark source',node.span);closed(parser,node.operand,path,bound,budget);return;}if(node?.k==='name'){if(node.path.length===1&&bound.has(node.path[0]))return;const actual=parser.origins.lookupValueOwner(node.path,path);if(!actual||!['fn','ctor'].includes(actual.k))fail('a loop chunk needs explicit complete immutable capture parameters before capturing this local',node.span);return;}if(node?.k==='let'){closed(parser,node.value,path,bound,budget);closed(parser,node.body,path,new Set([...bound,node.name]),budget);return;}if(node?.k==='if'){for(const child of [node.cond,node.then,node.else])closed(parser,child,path,bound,budget);return;}if(node?.k==='match'||node?.k==='match1'){for(const child of node.k==='match'?node.scrutinees:[node.scrutinee])closed(parser,child,path,bound,budget);for(const row of node.k==='match'?node.rows:node.cases){const inner=new Set(bound);for(const pattern of row.patterns??[row.pattern])boundNames(pattern,inner);closed(parser,row.body,path,inner,budget);}return;}for(const child of expressionChildren(node))closed(parser,child,path,bound,budget);}
function measure(root){let nodes=0,text=0;const pending=[root];while(pending.length){const value=pending.pop();if(typeof value==='string')text+=value.length;else if(value&&typeof value==='object'){if(++nodes>100000)fail('the complete finite record loop expansion exceeds its source budget');for(const[key,child]of Object.entries(value)){text+=key.length;pending.push(child);}}if(text>16777216)fail('the complete finite record loop expansion exceeds its source budget');}return {nodes,text};}
function completedBodyAhead(parser){let depth=0,work=0;for(let index=parser.cursor.index;index<parser.cursor.tokens.length;index+=1){const token=parser.cursor.tokens[index];if(++work>100000)fail('the complete loop body exceeds its source inspection budget',token);if(token.kind==='identifier'&&['break','continue','return'].includes(token.raw)||token.kind==='punct'&&token.value==='?')return true;if(token.value==='{')depth+=1;if(token.value==='}'&&--depth===0)return false;}return false;}
function combineExpressionPaths(head,tail,span){const result=[];for(const first of head){if(first.kind==='return'){if(result.length>=256)fail('the complete record loop exits exceed their finite source budget',span);result.push(first);continue;}for(const second of tail){if(result.length>=256)fail('the complete record loop exits exceed their finite source budget',span);result.push({...second,count:first.count+second.count});}}return result;}
function alternateExpressionPaths(branches,span){const result=[];for(const branch of branches){if(result.length+branch.length>256)fail('the complete record loop exits exceed their finite source budget',span);result.push(...branch);}return result;}
function expressionPaths(parser,node,selected,budget,inspect=true){
 if(inspect&&!hasRustExpressionCompletion(node))return counts(parser,node,selected,budget).map(count=>({kind:'value',count}));
 if(++budget.nodes>100000)fail('the complete record loop exits exceed their finite source budget',node?.span);const descend=value=>expressionPaths(parser,value,selected,budget,false),chain=(left,right)=>combineExpressionPaths(left,right,node.span);
 if(node.k==='rustStandardSumTry'){const branch=readRustStandardSumTrySourceBranches(parser,node),error=readRustOwnedRecordLoopResidualCountError(branch?.residuals?.length??0);if(error)fail(error,node.span);return chain(descend(node.operand),[{kind:'value',count:0},...branch.residuals.map(leaf=>({kind:'return',count:0,origin:leaf.origin,ownership:leaf.ownership,leaf}))]);}
 const source=readRustOwnedRecordEffectSource(parser,node);if(source?.form==='write'){const rhs=node.args[0].value,overlaps=overlapping(source.target,selected.target),exact=source.target===selected.target&&rhs?.k==='binary'&&rhs.op==='sub'&&numeral(rhs.right,1)&&samePlace(place(parser,rhs.left),selected),error=readRustOwnedRecordLoopWriteError(overlaps,exact);if(error)fail(error,node.span);return descend(rhs).map(path=>path.kind==='return'?path:{...path,count:path.count+(exact?1:0)});}
 const nested=readRustOwnedRecordFiniteLoopSource(parser,node);if(nested)return counts(parser,node,selected,budget).map(count=>({kind:'value',count}));
 if(node.k==='lambda'||node.k==='propositionValue'||node.k==='math'&&readRustExpressionCompletionPacketOperation(node.op))fail('this loop body needs its separate callable or quantified exit derivation',node.span);
 if(node.k==='if')return chain(descend(node.cond),alternateExpressionPaths([descend(node.then),descend(node.else)],node.span));
 if(node.k==='binary'&&['and','or'].includes(node.op))return chain(descend(node.left),alternateExpressionPaths([[{kind:'value',count:0}],descend(node.right)],node.span));
 if(node.k==='match'||node.k==='match1'){let head=[{kind:'value',count:0}];for(const value of node.k==='match'?node.scrutinees:[node.scrutinee])head=chain(head,descend(value));return chain(head,alternateExpressionPaths((node.k==='match'?node.rows:node.cases).map(row=>descend(row.body)),node.span));}
 let result=[{kind:'value',count:0}];for(const value of expressionChildren(node))result=chain(result,descend(value));return result;
}
function paths(parser,tree,selected,budget={nodes:0}){
 if(++budget.nodes>100000)fail('the complete record loop exits exceed their finite source budget',tree.span);
 const expression=value=>expressionPaths(parser,value,selected,budget),combine=(head,tail)=>combineExpressionPaths(head,tail,tree.span);
 if(tree.node==='loop'){const child=readRustOwnedRecordFiniteLoopSource(parser,tree.expression),error=readRustOwnedRecordNestedCompletionError(admitted.has(child),child?.writes?.some(target=>overlapping(target,selected.target))??true);if(error)fail(error,tree.span);return [...paths(parser,tree.continuation,selected,budget),...tree.exits.map(leaf=>({kind:'return',count:0,origin:leaf.origin,ownership:leaf.ownership,leaf}))];}
 if(tree.origin)return expression(tree.expression).map(path=>path.kind==='return'?path:{kind:tree.node,count:path.count,origin:tree.origin,ownership:tree.ownership,leaf:tree});
 if(tree.node==='binding')return combine(expression(tree.binding.value),paths(parser,tree.body,selected,budget));
 if(tree.node==='match')return combine(expression(tree.scrutinee),alternateExpressionPaths(tree.arms.map(arm=>paths(parser,arm.body,selected,budget)),tree.span));
 if(tree.node==='conditional'){if(tree.stages)fail('loop chain conditions need their complete selected-stage progress derivation',tree.span);return combine(expression(tree.cond),[...paths(parser,tree.then,selected,budget),...paths(parser,tree.otherwise,selected,budget)]);}
 if(tree.node==='sequence'){const first=paths(parser,tree.first,selected,budget),rest=paths(parser,tree.rest,selected,budget),result=[];for(const path of first){if(path.kind!=='value'){result.push(path);continue;}for(const tail of rest){if(result.length>=256)fail('the complete record loop exits exceed their finite source budget',tree.span);result.push({...tail,count:path.count+tail.count});}}return result;}
 fail('a completed loop needs every actual source completion path',tree.span);
}
function retainPaths(parser,exits){return exits.map(exit=>{const cost=measureRustStandardSumSourceMetadata([exit.origin]),reservation=readRustStandardSumSourceMetadataReservation(parser.standardSumSourceMaterializedNodes??0,parser.standardSumSourceMaterializedUnits??0,cost[0],cost[1]);if(reservation.length!==2)fail('the complete loop exit origin exceeds its materialization budget',exit.leaf.expression.span);parser.standardSumSourceMaterializedNodes=reservation[0];parser.standardSumSourceMaterializedUnits=reservation[1];return {...exit,origin:structuredClone(exit.origin),ownership:cloneRustOrdinaryCompletionOwnershipSnapshot(parser,exit.ownership)};});}
function closedTree(parser,tree,path,bound=new Set()){
 if(tree.node==='loop'){assertRustOwnedRecordLoopAdmission(readRustOwnedRecordFiniteLoopSource(parser,tree.expression));return;}
 if(tree.origin){closed(parser,tree.expression,path,bound);return;}
 if(tree.node==='binding'){closed(parser,tree.binding.value,path,bound);closedTree(parser,tree.body,path,new Set([...bound,tree.binding.name]));return;}
 if(tree.node==='match'){closed(parser,tree.scrutinee,path,bound);for(const arm of tree.arms){const inner=new Set(bound);for(const pattern of arm.patterns)boundNames(pattern,inner);closedTree(parser,arm.body,path,inner);}return;}
 if(tree.node==='conditional'){if(tree.stages)fail('loop chain captures need their complete selected-stage source scope',tree.span);closed(parser,tree.cond,path,bound);closedTree(parser,tree.then,path,bound);closedTree(parser,tree.otherwise,path,bound);return;}
 if(tree.node==='sequence'){closedTree(parser,tree.first,path,bound);closedTree(parser,tree.rest,path,bound);return;}
 fail('a completed loop needs every actual source capture scope',tree.span);
}
function invariant(parser,before,after,slots,span){const keys=new Set([...before.keys(),...after.keys()]);for(const key of keys){if(!before.has(key)&&!slots.some(slot=>testRustOwnedRecordFieldDescendant(slot.root,key)))continue;const error=readRustOwnedRecordLoopInvariantError(rustOwnedRecordFieldState(before,key),rustOwnedRecordFieldState(after,key));if(error)fail(error,span);}}
export function readRustOwnedRecordFiniteLoop(parser,path){
 const c=parser.cursor,start=c.peek();if(start.kind!=='identifier'||start.value!=='while'||start.raw!=='while')return null;let slots=readRustOwnedRecordStateSlots(parser);if(slots.length===0)return null;c.next();
 const guard=withRustStandardSumSourceContext(parser,null,()=>parser.expr(path,{noStruct:true})),before=readRustOrdinaryCompletionOwnershipSnapshot(parser),bodyStart=c.index;
 const oldDepth=parser.ownedRecordFiniteLoopDepth??0,depthError=readRustOwnedRecordNestedLoopDepthError(oldDepth+1);if(depthError)fail(depthError,{start:start.start,end:start.end});let completion,body;parser.ownedRecordFiniteLoopDepth=oldDepth+1;try{completion=completedBodyAhead(parser)&&parser.standardSumReturnOrigin?.type?readRustOrdinaryLoopCompletion(parser,path):null;body=completion?.expression??parser.block(path);}finally{parser.ownedRecordFiniteLoopDepth=oldDepth;}
 const end=c.peek(-1),span={start:start.start,end:end.end};
 const node={k:'math',op:readRustOwnedRecordLoopOperation(),name:String(inventory(parser).entries.length),args:[{value:guard,spread:false},{value:body,spread:false}],block:true,span};
 if(parser.origins.collecting){parser.origins.standardSumSourceNeedsFinalAdmission=true;inventory(parser).entries.push({provisional:true,span,slots});return parser.origins.mark(node,parser.origins.typeOrigin({kind:'unit'},path));}
 if(c.tokens.slice(bodyStart,c.index).some(token=>token.kind==='identifier'&&['loop','for',...(!completion?['return','break','continue']:[])].includes(token.raw)))fail('this finite loop body needs its explicit nested or completed exit-state consumer',span);
 const selected=counter(parser,guard),origin=completion?null:parser.origins.origin(body,path);if(!completion&&(!origin.proven||origin.type?.kind!=='unit'||origin.wrappers.length))fail('the finite record loop body must have its actual Unit result',body.span);
 const exits=completion?retainPaths(parser,paths(parser,completion.tree,selected)):counts(parser,body,selected,{nodes:0}).map(count=>({kind:'value',count,ownership:readRustOrdinaryCompletionOwnershipSnapshot(parser)})),progress=readRustOwnedRecordLoopProgressError(exits.map(exit=>exit.kind),exits.map(exit=>exit.count));if(progress)fail(progress,span);closed(parser,guard,path);if(completion)closedTree(parser,completion.tree,path);else closed(parser,body,path);
 for(const exit of exits)if(exit.kind==='value'||exit.kind==='continue')invariant(parser,before,exit.ownership,slots,span);
 slots=selectRustOwnedRecordLoopStateSlots(parser,{k:'tuple',items:[guard,body],span},slots);
 const size=measure({guard,body,types:slots.map(slot=>slot.origin.type)}),arena=inventory(parser),reservation=readRustOwnedRecordLoopExpansionReservation(size.nodes,size.text);if(reservation.length!==2)fail('the complete finite record loop expansion exceeds its source budget',span);const error=readRustOwnedRecordLoopExpansionError(arena.nodes,reservation[0],reservation[1],255);if(error)fail(error,span);arena.nodes+=reservation[0]*255;
 const admission={span,counterType:selected.origin.type,slots,path:[...path],result:parser.standardSumReturnOrigin,controlOwner:completion?.owner??null,returns:completion?.returns??[],returnLeaves:exits.filter(exit=>exit.kind==='return').map(exit=>exit.leaf),exits,writes:writeFootprint(parser,body)};admitted.add(admission);arena.entries.push(admission);parser.origins.restoreOwnedBindings(before);for(const exit of exits)if(exit.kind==='break')parser.origins.joinOwnedBindings(parser.origins.snapshotOwnedBindings(),exit.ownership,true,true);return parser.origins.mark(node,parser.origins.typeOrigin({kind:'unit'},path));
}
export function lowerRustOwnedRecordLoopCompletion(parser,node,destination,terminal,sourceResult){
 const source=readRustOwnedRecordFiniteLoopSource(parser,node);assertRustOwnedRecordLoopAdmission(source);if(!source.controlOwner)fail('a loop return needs its actual completion carrier',node.span);source.destination=destination;
 const result=parser.freshProductBinding(),control=parser.freshProductBinding(),owner=['crate',...source.controlOwner],reference=name([result]);
 const continuation=()=>terminal?{k:'unit'}:{k:'app',fn:name([...destination,'StatementContinuation']),args:[]};
 if(terminal)parser.origins.argument(parser.origins.origin({k:'unit'},source.path),sourceResult,node.span);
 const cases=[{pattern:{k:'ctor',path:[...owner,'FunctionResult'],binds:[result]},body:terminal?reference:{k:'app',fn:name([...destination,'FunctionResult']),args:[reference]}},...['StatementContinuation','LoopBreak','LoopContinue'].map(variant=>({pattern:{k:'ctor',path:[...owner,variant],binds:[]},body:continuation()}))];
 return {k:'let',name:control,value:node,body:{k:'match1',scrutinee:name([control]),cases}};
}
function copyExpression(parser,node){const result=structuredClone(node),pending=[[node,result]];while(pending.length){const[first,second]=pending.pop();if(!first||typeof first!=='object')continue;if(first.k)parser.origins.reuseExpressionOrigin(first,second);for(const key of Object.keys(first))if(first[key]&&typeof first[key]==='object')pending.push([first[key],second[key]]);}return result;}
const name=path=>({k:'name',path});
// The factory consumes only an authenticated reader token. Each chunk is an
// acyclic, explicit-state function; no recursion bound or runtime fuel exists.
export function lowerRustOwnedRecordFiniteLoop(parser,node,environment,next,compile){
 const source=readRustOwnedRecordFiniteLoopSource(parser,node);assertRustOwnedRecordLoopAdmission(source);const arena=inventory(parser),ordinal=Number(node.name),slots=source.slots,keys=slots.map(slot=>slot.lexical);
 if(source.returns.length&&!source.destination)fail('a returning loop needs its actual enclosing ordinary completion consumer',node.span);
 if(keys.some(key=>!environment.has(key)))fail('the finite loop lost an actual initialized state root',node.span);
 const iterations=planRustOwnedRecordLoopIterations(source.counterType.bits),width=readRustOwnedRecordLoopChunkSize(),chunks=Math.ceil(iterations.length/width),cached=source.helperNames,names=cached??[];
 if(!cached)for(let index=0;index<chunks;index+=1){let collision=0,value;do{value=readRustOwnedRecordLoopChunkName(ordinal,index,collision++);}while(parser.reservedNames.has(value));parser.reservedNames.add(value);names.push(value);}
 const types=slots.map(slot=>readRustRecordSourceType(parser,slot.origin.type,slot.origin.path)),controlOwner=source.controlOwner?['crate',...source.controlOwner]:null,resultType=controlOwner?{kind:'named',path:controlOwner}:{kind:'unit'},ret={kind:'tuple',elements:[resultType,...types]};
 const control=(variant,args=[])=>({k:'app',fn:name([...controlOwner,variant]),args});
 const packet=(state,value=controlOwner?control('StatementContinuation'):{k:'unit'})=>({k:'tuple',items:[value,...keys.map(key=>name([state.get(key).name]))]});
 if(!cached)source.helperNames=names;
 if(!cached)for(let chunk=0;chunk<chunks;chunk+=1){
  const params=types.map(type=>({name:parser.freshProductBinding(),type})),initial=new Map(keys.map((key,index)=>[key,{slot:slots[index],name:params[index].name}])),count=Math.min(width,iterations.length-chunk*width);
  const step=(index,state)=>{
   if(index===count)return chunk+1<chunks?{k:'app',fn:name(['crate',...source.path,names[chunk+1]]),args:keys.map(key=>name([state.get(key).name]))}:packet(state);
   const guard=copyExpression(parser,node.args[0].value),body=copyExpression(parser,node.args[1].value);
   return compile(guard,state,{kind:'bool'},(condition,after)=>({k:'if',cond:condition,then:compile(body,new Map(after),resultType,(value,done)=>{
    if(!controlOwner){const rest=step(index+1,done);return acceptRustExpressionCompletionInertLeaf(value.k)?rest:{k:'let',name:parser.freshProductBinding(),type:{kind:'unit'},value,body:rest};}
    const decision=parser.freshProductBinding(),returned=parser.freshProductBinding(),reference=name([decision]),cases=['FunctionResult','StatementContinuation','LoopBreak','LoopContinue'].map(variant=>{const disposition=readRustOwnedRecordLoopExitDisposition(variant),completed=disposition==='return'||disposition==='break';return {pattern:{k:'ctor',path:[...controlOwner,variant],binds:disposition==='return'?[returned]:[]},body:{k:'tuple',items:[{k:'bool',value:completed},disposition==='return'?control('FunctionResult',[name([returned])]):control('StatementContinuation')]}};});
    return {k:'let',name:decision,value:{k:'match1',scrutinee:value,cases},body:{k:'if',cond:{k:'tupleIndex',object:reference,index:0},then:packet(done,{k:'tupleIndex',object:reference,index:1}),else:step(index+1,done)}};
   }),else:packet(after)}));
  };
  const fn=attachRustFiniteLoopStateTransform(parser,{k:'fn',name:names[chunk],params,ret,body:step(0,initial),generated:true,sourceFiniteRecordLoopHelper:true,span:source.span},source,controlOwner?{kind:'named',path:controlOwner}:null);sealFiniteRecordLoopHelper(fn,source);arena.helpers.push({path:source.path,declaration:fn});
 }
 const resultName=parser.freshProductBinding(),reference=name([resultName]),after=new Map(environment),bindings=[];
 keys.forEach((key,index)=>{const fresh=parser.freshProductBinding();bindings.push({name:fresh,value:{k:'tupleIndex',object:reference,index:index+1}});after.set(key,{slot:slots[index],name:fresh});});
 let result=next(source.destination?{k:'tupleIndex',object:reference,index:0}:{k:'unit',span:node.span},after);for(let index=bindings.length-1;index>=0;index-=1)result={k:'let',...bindings[index],body:result};return {k:'let',name:resultName,type:ret,value:{k:'app',fn:name(['crate',...source.path,names[0]]),args:keys.map(key=>name([environment.get(key).name]))},body:result,span:node.span};
}
export function appendRustOwnedRecordFiniteLoopHelpers(parser,program){const helpers=parser.ownedRecordFiniteLoops?.helpers??[];const append=(items,path)=>[...items.map(item=>item.k==='module'?{...item,items:append(item.items,[...path,item.name])}:item),...helpers.filter(row=>row.path.length===path.length&&row.path.every((value,index)=>value===path[index])).map(row=>row.declaration)];return helpers.length?{...program,items:append(program.items,[])}:program;}
