// Source-owned finite-loop admission tokens. The reader adds a token only
// after checking actual counter syntax, progress and every continuing exit.
import {unsupported} from './diagnostics.js';
import {readRustOwnedRecordStateSlots,readRustOwnedRecordStateLayout} from './rust-owned-record-state.js';
import {readRustOwnedRecordEffectSource} from './rust-owned-record-effects.js';
import {readRustOwnedRecordLoopOperation,readRustOwnedRecordLoopCounterError,readRustOwnedRecordLoopProgressError,readRustOwnedRecordLoopWriteError,readRustOwnedRecordLoopInvariantError,readRustOwnedRecordLoopExpansionError,readRustOwnedRecordLoopChunkName,readRustOwnedRecordLoopChunkSize,planRustOwnedRecordLoopIterations} from './rust-owned-record-loop-rules.js';
import {readRustOwnedRecordStateFieldIdentity} from './rust-owned-record-state-rules.js';
import {testRustOwnedRecordFieldDescendant} from './rust-owned-record-update-rules.js';
import {rustOwnedRecordFieldState} from './rust-owned-record-updates.js';
import {withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {sealFiniteRecordLoopHelper} from './finite-record-loop-helpers.js';
import {readRustRecordSourceType} from './rust-records.js';
import {readRustOrdinaryCompletionOwnershipSnapshot} from './rust-ordinary-completions.js';
import {readRustOwnedRecordEffectChildren} from './rust-owned-record-effect-rules.js';
import {acceptRustExpressionCompletionInertLeaf,readRustExpressionCompletionPacketOperation} from './rust-expression-completion-rules.js';
const admitted=new WeakSet();
export function assertRustOwnedRecordLoopAdmission(value){if(!admitted.has(value))throw unsupported('finite record loop helper','the helper needs its actual admitted source counter and exit-state derivation',value?.span);}
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
 const names=pattern.args.map(pattern=>pattern.k==='bind'?pattern.name:null);if(names.some(name=>name===null)||new Set(names).size!==names.length)return null;const index=names.indexOf(row.body.path[0]);return index<0?null:{slot:parent.slot,indices:[...parent.indices,index],origin:actual.entry.fields[index]};
}
function counter(parser,guard){if(guard?.k!=='binary'||!numeral(guard.right,0))fail('the loop head needs its actual pure counter comparison with zero',guard?.span);const selected=place(parser,guard.left);if(!selected||selected.indices.length===0)fail('the loop head needs an actual mutable record field place',guard.span);const origin=selected.origin,owner=origin.expandedSourceOwner??origin.sourceOwner,error=readRustOwnedRecordLoopCounterError(origin.type?.kind??'',origin.type?.bits??0,origin.type?.signed===true,origin.wrappers.length,owner?.kind==='primitive'&&owner.arguments.length===0?owner.path.join('::'):'',guard.op);if(!origin.proven||error)fail(error||'the loop counter needs its actual complete source proof',guard.span);return {...selected,target:readRustOwnedRecordStateFieldIdentity(selected.slot.root,selected.indices)};}
function samePlace(left,right){return left&&left.slot.lexical===right.slot.lexical&&left.indices.length===right.indices.length&&left.indices.every((index,at)=>index===right.indices[at]);}
function counts(parser,node,selected,budget){
 if(++budget.nodes>100000)fail('the complete record loop exits exceed their finite source budget',node?.span);
 const source=readRustOwnedRecordEffectSource(parser,node);
 if(source?.form==='write'){
  const overlaps=source.target===selected.target||testRustOwnedRecordFieldDescendant(source.target,selected.target)||testRustOwnedRecordFieldDescendant(selected.target,source.target),rhs=node.args[0].value;
  const exact=source.target===selected.target&&rhs?.k==='binary'&&rhs.op==='sub'&&numeral(rhs.right,1)&&samePlace(place(parser,rhs.left),selected),error=readRustOwnedRecordLoopWriteError(overlaps,exact);if(error)fail(error,node.span);
  return counts(parser,rhs,selected,budget).map(count=>count+(exact?1:0));
 }
 if(node?.k==='lambda'||readRustOwnedRecordFiniteLoopSource(parser,node)||node?.k==='math'&&readRustExpressionCompletionPacketOperation(node.op))fail('nested callable, packet and loop effects need their separate progress derivation',node.span);
 const add=(left,right)=>{if(left.length*right.length>256)fail('the complete record loop exits exceed their finite source budget',node.span);return left.flatMap(first=>right.map(second=>first+second));};
 if(node?.k==='if')return add(counts(parser,node.cond,selected,budget),[...counts(parser,node.then,selected,budget),...counts(parser,node.else,selected,budget)]);
 if(node?.k==='binary'&&['and','or'].includes(node.op))return add(counts(parser,node.left,selected,budget),[0,...counts(parser,node.right,selected,budget)]);
 if(node?.k==='match'||node?.k==='match1'){let result=[0];for(const value of node.k==='match'?node.scrutinees:[node.scrutinee])result=add(result,counts(parser,value,selected,budget));const branches=(node.k==='match'?node.rows:node.cases).flatMap(row=>counts(parser,row.body,selected,budget));return add(result,branches);}
 if(node?.k==='rustStandardSumTry'||node?.k==='propositionValue')fail('this loop body needs its separate residual or quantified exit derivation',node.span);
 let result=[0];for(const value of expressionChildren(node))result=add(result,counts(parser,value,selected,budget));return result;
}
function expressionChildren(node){if(!node||typeof node!=='object')return [];switch(node.k){case'let':return[node.value,node.body];case'print':return[node.expr,node.body];case'app':return[node.fn,...node.args];case'binary':return[node.left,node.right];case'unary':case'cast':return[node.arg];case'tuple':case'array':return node.items.map(item=>item?.k?item:item.value);case'tupleIndex':case'field':return[node.object];case'math':return node.args.map(arg=>arg.value);case'name':case'num':case'str':case'bool':case'unit':return[];default:{const roles=readRustOwnedRecordEffectChildren(node.k);if(!roles.length)fail('the loop body needs complete ordered source roles',node.span);return roles.flatMap(role=>Array.isArray(node[role])?node[role].map(child=>child?.k?child:child.value):[node[role]]);}}}
function boundNames(pattern,result){if(['bind','bindOrCtor','alias','natSucc'].includes(pattern?.k))result.add(pattern.name);for(const child of pattern?.args??[])boundNames(child,result);if(pattern?.pattern)boundNames(pattern.pattern,result);}
function closed(parser,node,path,bound=new Set(),budget={nodes:0}){if(++budget.nodes>100000)fail('the loop capture inventory exceeds its source budget',node?.span);if(node?.k==='name'){if(node.path.length===1&&bound.has(node.path[0]))return;const actual=parser.origins.lookupValueOwner(node.path,path);if(!actual||!['fn','ctor'].includes(actual.k))fail('a loop chunk needs explicit complete immutable capture parameters before capturing this local',node.span);return;}if(node?.k==='let'){closed(parser,node.value,path,bound,budget);closed(parser,node.body,path,new Set([...bound,node.name]),budget);return;}if(node?.k==='if'){for(const child of [node.cond,node.then,node.else])closed(parser,child,path,bound,budget);return;}if(node?.k==='match'||node?.k==='match1'){for(const child of node.k==='match'?node.scrutinees:[node.scrutinee])closed(parser,child,path,bound,budget);for(const row of node.k==='match'?node.rows:node.cases){const inner=new Set(bound);for(const pattern of row.patterns??[row.pattern])boundNames(pattern,inner);closed(parser,row.body,path,inner,budget);}return;}for(const child of expressionChildren(node))closed(parser,child,path,bound,budget);}
function measure(root){let nodes=0,text=0;const pending=[root];while(pending.length){const value=pending.pop();if(typeof value==='string')text+=value.length;else if(value&&typeof value==='object'){if(++nodes>100000)fail('the complete finite record loop expansion exceeds its source budget');for(const[key,child]of Object.entries(value)){text+=key.length;pending.push(child);}}if(text>16777216)fail('the complete finite record loop expansion exceeds its source budget');}return {nodes,text};}
export function readRustOwnedRecordFiniteLoop(parser,path){
 const c=parser.cursor,start=c.peek();if(start.kind!=='identifier'||start.value!=='while'||start.raw!=='while')return null;const slots=readRustOwnedRecordStateSlots(parser);if(slots.length===0)return null;c.next();
 const guard=withRustStandardSumSourceContext(parser,null,()=>parser.expr(path,{noStruct:true})),before=readRustOrdinaryCompletionOwnershipSnapshot(parser),bodyStart=c.index,body=parser.block(path),end=c.peek(-1),span={start:start.start,end:end.end};
 const node={k:'math',op:readRustOwnedRecordLoopOperation(),name:String(inventory(parser).entries.length),args:[{value:guard,spread:false},{value:body,spread:false}],block:true,span};
 if(parser.origins.collecting){parser.origins.standardSumSourceNeedsFinalAdmission=true;inventory(parser).entries.push({provisional:true,span,slots});return parser.origins.mark(node,parser.origins.typeOrigin({kind:'unit'},path));}
 if(c.tokens.slice(bodyStart,c.index).some(token=>token.kind==='identifier'&&['return','break','continue','while','loop','for'].includes(token.raw)))fail('this finite loop body needs its explicit nested or completed exit-state consumer',span);
 const selected=counter(parser,guard),origin=parser.origins.origin(body,path);if(!origin.proven||origin.type?.kind!=='unit'||origin.wrappers.length)fail('the finite record loop body must have its actual Unit result',body.span);
 const paths=counts(parser,body,selected,{nodes:0}),progress=readRustOwnedRecordLoopProgressError(paths.map(()=>'value'),paths);if(progress)fail(progress,span);closed(parser,guard,path);closed(parser,body,path);
 const after=readRustOrdinaryCompletionOwnershipSnapshot(parser),keys=new Set([...before.keys(),...after.keys()]);for(const key of keys){if(!before.has(key)&&!slots.some(slot=>testRustOwnedRecordFieldDescendant(slot.root,key)))continue;const first=rustOwnedRecordFieldState(before,key),last=rustOwnedRecordFieldState(after,key),error=readRustOwnedRecordLoopInvariantError(first,last);if(error)fail(error,span);}
 const size=measure({guard,body,types:slots.map(slot=>slot.origin.type)}),arena=inventory(parser),error=readRustOwnedRecordLoopExpansionError(arena.nodes,size.nodes*2+128,size.text*2,255);if(error)fail(error,span);arena.nodes+=(size.nodes*2+128)*255;
 const admission={span,counterType:selected.origin.type,slots,path:[...path]};admitted.add(admission);arena.entries.push(admission);parser.origins.restoreOwnedBindings(before);return parser.origins.mark(node,parser.origins.typeOrigin({kind:'unit'},path));
}
function copyExpression(parser,node){const result=structuredClone(node),pending=[[node,result]];while(pending.length){const[first,second]=pending.pop();if(!first||typeof first!=='object')continue;if(first.k)parser.origins.reuseExpressionOrigin(first,second);for(const key of Object.keys(first))if(first[key]&&typeof first[key]==='object')pending.push([first[key],second[key]]);}return result;}
const name=path=>({k:'name',path});
// The factory consumes only an authenticated reader token. Each chunk is an
// acyclic, explicit-state function; no recursion bound or runtime fuel exists.
export function lowerRustOwnedRecordFiniteLoop(parser,node,environment,next,compile){
 const source=readRustOwnedRecordFiniteLoopSource(parser,node);assertRustOwnedRecordLoopAdmission(source);const arena=inventory(parser),ordinal=Number(node.name),slots=source.slots,keys=slots.map(slot=>slot.lexical);
 if(keys.some(key=>!environment.has(key)))fail('the finite loop lost an actual initialized state root',node.span);
 const iterations=planRustOwnedRecordLoopIterations(source.counterType.bits),width=readRustOwnedRecordLoopChunkSize(),chunks=Math.ceil(iterations.length/width),names=[];
 for(let index=0;index<chunks;index+=1){let collision=0,value;do{value=readRustOwnedRecordLoopChunkName(ordinal,index,collision++);}while(parser.reservedNames.has(value));parser.reservedNames.add(value);names.push(value);}
 const types=slots.map(slot=>readRustRecordSourceType(parser,slot.origin.type,slot.origin.path)),ret={kind:'tuple',elements:[{kind:'unit'},...types]};
 const packet=state=>({k:'tuple',items:[{k:'unit'},...keys.map(key=>name([state.get(key).name]))]});
 for(let chunk=0;chunk<chunks;chunk+=1){
  const params=types.map(type=>({name:parser.freshProductBinding(),type})),initial=new Map(keys.map((key,index)=>[key,{slot:slots[index],name:params[index].name}])),count=Math.min(width,iterations.length-chunk*width);
  const step=(index,state)=>{
   if(index===count)return chunk+1<chunks?{k:'app',fn:name(['crate',...source.path,names[chunk+1]]),args:keys.map(key=>name([state.get(key).name]))}:packet(state);
   const guard=copyExpression(parser,node.args[0].value),body=copyExpression(parser,node.args[1].value);
   return compile(guard,state,{kind:'bool'},(condition,after)=>({k:'if',cond:condition,then:compile(body,new Map(after),{kind:'unit'},(value,done)=>{const rest=step(index+1,done);return acceptRustExpressionCompletionInertLeaf(value.k)?rest:{k:'let',name:parser.freshProductBinding(),type:{kind:'unit'},value,body:rest};}),else:packet(after)}));
  };
  const fn={k:'fn',name:names[chunk],params,ret,body:step(0,initial),generated:true,sourceFiniteRecordLoopHelper:true,span:source.span};sealFiniteRecordLoopHelper(fn,source);arena.helpers.push({path:source.path,declaration:fn});
 }
 const resultName=parser.freshProductBinding(),reference=name([resultName]),after=new Map(environment),bindings=[];
 keys.forEach((key,index)=>{const fresh=parser.freshProductBinding();bindings.push({name:fresh,value:{k:'tupleIndex',object:reference,index:index+1}});after.set(key,{slot:slots[index],name:fresh});});
 let result=next({k:'unit',span:node.span},after);for(let index=bindings.length-1;index>=0;index-=1)result={k:'let',...bindings[index],body:result};return {k:'let',name:resultName,type:ret,value:{k:'app',fn:name(['crate',...source.path,names[0]]),args:keys.map(key=>name([environment.get(key).name]))},body:result,span:node.span};
}
export function appendRustOwnedRecordFiniteLoopHelpers(parser,program){const helpers=parser.ownedRecordFiniteLoops?.helpers??[];const append=(items,path)=>[...items.map(item=>item.k==='module'?{...item,items:append(item.items,[...path,item.name])}:item),...helpers.filter(row=>row.path.length===path.length&&row.path.every((value,index)=>value===path[index])).map(row=>row.declaration)];return helpers.length?{...program,items:append(program.items,[])}:program;}
