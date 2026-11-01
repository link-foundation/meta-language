// One ordered expression/state algebra. All states are actual immutable SSA
// snapshots in the common core; source ownership remains checked during read.
import {unsupported} from './diagnostics.js';
import {installRustOwnedRecordEffectTransport,readRustOwnedRecordStateAssignment,readRustOwnedRecordStateDeclarations,reconstructRustOwnedRecordState} from './rust-owned-record-state.js';
import {readRustOwnedRecordEffectForm,readRustOwnedRecordEffectChildren,readRustOwnedRecordEffectOperation,readRustOwnedRecordEffectLimit,acceptRustOwnedRecordAssertionComparison} from './rust-owned-record-effect-rules.js';
import {readRustExpressionCompletionPacketOperation,acceptRustExpressionCompletionInertLeaf,readRustExpressionCompletionShortCircuitValue} from './rust-expression-completion-rules.js';
import {readRustRecordSourceType} from './rust-records.js';
const fail=(reason,span)=>{throw unsupported('Rust ordered record state',reason,span);};
function effects(parser){
 if(parser.ownedRecordEffects)return parser.ownedRecordEffects;
 const context={reads:[],writes:[],inspection:{nodes:0},work:0};parser.ownedRecordEffects=context;
 installRustOwnedRecordEffectTransport(parser,{
  read(slot,node){if(context.reads.length+context.writes.length>=readRustOwnedRecordEffectLimit('work'))fail('the actual state operations exceed their source budget',node.span);const index=context.reads.length;context.reads.push(slot);return parser.origins.mark({k:'math',op:readRustOwnedRecordEffectOperation('read'),name:String(index),args:[],span:node.span},slot.origin);},
  write(info){if(context.reads.length+context.writes.length>=readRustOwnedRecordEffectLimit('work'))fail('the actual state operations exceed their source budget',info.span);const index=context.writes.length;context.writes.push(info);return parser.origins.mark({k:'math',op:readRustOwnedRecordEffectOperation('write'),name:String(index),args:[{value:info.value,spread:false}],span:info.span},parser.origins.typeOrigin({kind:'unit'},info.path));},
 });return context;
}
export function readRustOwnedRecordEffectExpression(parser,path,options,readOrdinary){effects(parser);return readRustOwnedRecordStateAssignment(parser,path)?.value??readOrdinary();}
export function beginRustOwnedRecordShortCircuit(parser){return parser.ownedRecordEffects?parser.origins.snapshotOwnedBindings():null;}
export function endRustOwnedRecordShortCircuit(parser,before){if(before!==null)parser.origins.joinOwnedBindings(before,parser.origins.snapshotOwnedBindings(),true,true);}
function marker(node){return node?.k==='math'&&(node.op===readRustOwnedRecordEffectOperation('read')||node.op===readRustOwnedRecordEffectOperation('write'));}
function contains(node,callables=false,budget={nodes:0}){const pending=[node],seen=new Set();while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;if(++budget.nodes>readRustOwnedRecordEffectLimit('inspection'))fail('the actual state expression exceeds its inspection budget',node?.span);seen.add(value);if(marker(value))return true;if(!callables&&value.k==='lambda')continue;for(const child of Object.values(value))if(child&&typeof child==='object')pending.push(child);}return false;}
function typeOf(parser,node,path,expected=null,seen=new Set()){
 if(expected)return expected;if(!node||seen.has(node))return null;seen.add(node);
 const origin=parser.origins.origin(node,path);if(origin.proven&&origin.type)return readRustRecordSourceType(parser,origin.type,origin.path);
 if(node.k==='unit')return {kind:'unit'};if(node.k==='bool')return {kind:'bool'};if(node.k==='str')return {kind:'string'};if(node.k==='num'&&node.type)return node.type;
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
 return null;
}
function childSlots(node,roles){const result=[];for(const role of roles){const value=node[role];if(Array.isArray(value))value.forEach((child,index)=>{const wrapped=child&&typeof child==='object'&&!child.k&&child.value&&typeof child.value==='object';result.push({role,index,child:wrapped?child.value:child,wrapper:wrapped?child:null});});else if(value&&typeof value==='object')result.push({role,index:null,child:value,wrapper:null});}return result;}
function alpha(parser,node,environment,declarations,path,budget){
 if(++budget.nodes>readRustOwnedRecordEffectLimit('work'))fail('state lexical renaming exceeds its source budget',node.span);
 const mark=value=>parser.origins.mark(value,parser.origins.origin(node,path));
 if(node.k==='name')return node.path.length===1&&environment.has(node.path[0])?mark({...node,path:[environment.get(node.path[0])]}):node;
 if(node.k==='let'){const name=parser.freshProductBinding(),inner=new Map(environment);inner.set(node.name,name);const slot=readRustOwnedRecordStateDeclarations(parser).get(node.name);if(slot)declarations.set(name,slot);return mark({...node,name,value:alpha(parser,node.value,environment,declarations,path,budget),body:alpha(parser,node.body,inner,declarations,path,budget)});}
 if(node.k==='lambda'){const inner=new Map(environment),params=node.params.map(parameter=>{const name=parser.freshProductBinding();inner.set(parameter.name,name);return {...parameter,name};});return mark({...node,params,captures:(node.captures??[]).map(name=>environment.get(name)??name),body:alpha(parser,node.body,inner,declarations,path,budget)});}
 const renamePattern=(pattern,scope)=>{if(pattern.k==='bind'||pattern.k==='bindOrCtor'||pattern.k==='alias'||pattern.k==='natSucc'){const name=parser.freshProductBinding();scope.set(pattern.name,name);pattern={...pattern,name};}if(pattern.binds)pattern={...pattern,binds:pattern.binds.map(name=>{if(name===null)return null;const next=parser.freshProductBinding();scope.set(name,next);return next;})};if(pattern.args)pattern={...pattern,args:pattern.args.map(child=>renamePattern(child,scope))};if(pattern.pattern)pattern={...pattern,pattern:renamePattern(pattern.pattern,scope)};if(pattern.inner)pattern={...pattern,inner:renamePattern(pattern.inner,scope)};return pattern;};
 if(node.k==='match'||node.k==='match1'){const rows=node.k==='match'?node.rows:node.cases,changed=rows.map(row=>{const inner=new Map(environment),patterns=node.k==='match'?row.patterns.map(pattern=>renamePattern(pattern,inner)):null,pattern=node.k==='match1'?renamePattern(row.pattern,inner):null;return {...row,...(patterns?{patterns}:{pattern}),body:alpha(parser,row.body,inner,declarations,path,budget)};});return mark({...node,...(node.k==='match'?{scrutinees:node.scrutinees.map(value=>alpha(parser,value,environment,declarations,path,budget)),rows:changed}:{scrutinee:alpha(parser,node.scrutinee,environment,declarations,path,budget),cases:changed})});}
 const roles=node.k==='if'?['cond','then','else']:node.k==='print'?['expr','body']:readRustOwnedRecordEffectChildren(node.k),result={...node};for(const role of roles)if(Array.isArray(node[role]))result[role]=[...node[role]];for(const slot of childSlots(node,roles)){const value=alpha(parser,slot.child,environment,declarations,path,budget);if(slot.index===null)result[slot.role]=value;else result[slot.role][slot.index]=slot.wrapper?{...slot.wrapper,value}:value;}return mark(result);
}
function lower(parser,node,path,declarations,initial=new Map(),expected=null,finished=(value,environment)=>value){
 const arena=effects(parser),has=(node,callables=false)=>contains(node,callables,arena.inspection);
 const mark=(value,original)=>parser.origins.mark(value,parser.origins.origin(original,path));
 function capture(value,origin,next){if(acceptRustExpressionCompletionInertLeaf(value.k))return next(value);const want=typeOf(parser,value,path,origin);if(!want)fail('an earlier evaluated operand needs its actual complete type before a later state effect',value.span);const name=parser.freshProductBinding(),reference=mark({k:'name',path:[name]},value);return {k:'let',name,type:want,value,body:next(reference),span:value.span};}
 function packet(value,environment,keys,resultType,span){const items=[value,...keys.map(key=>({k:'name',path:[environment.get(key).name]}))];return {k:'tuple',items,span};}
 function phi(original,environment,build,next,depth,want){const keys=[...environment.keys()],resultType=typeOf(parser,original,path,want);if(!resultType)fail('a branch state join needs its actual complete result type or actual enclosing annotation',original.span);const tuple={kind:'tuple',elements:[resultType,...keys.map(key=>readRustRecordSourceType(parser,environment.get(key).slot.origin.type,environment.get(key).slot.origin.path))]},value=build((result,branch)=>packet(result,branch,keys,resultType,original.span)),name=parser.freshProductBinding(),reference={k:'name',path:[name]},bindings=[],joined=new Map(environment);for(let index=0;index<keys.length;index+=1){const fresh=parser.freshProductBinding();bindings.push({name:fresh,value:{k:'tupleIndex',object:reference,index:index+1}});joined.set(keys[index],{...environment.get(keys[index]),name:fresh});}let body=next(mark({k:'tupleIndex',object:reference,index:0},original),joined);for(const binding of bindings.toReversed())body={k:'let',...binding,body};return {k:'let',name,type:tuple,value,body,span:original.span};}
 function ordered(original,roles,environment,next,depth,want){const slots=childSlots(original,roles),selected=new Map();const step=(at,current)=>{if(at===slots.length){const rebuilt={...original};for(const role of roles)if(Array.isArray(original[role]))rebuilt[role]=[...original[role]];for(const [slot,value]of selected)if(slot.index===null)rebuilt[slot.role]=value;else rebuilt[slot.role][slot.index]=slot.wrapper?{...slot.wrapper,value}:value;return next(mark(rebuilt,original),current);}const slot=slots[at],later=slots.slice(at+1).some(item=>has(item.child));return visit(slot.child,current,(value,after)=>{const resume=actual=>{selected.set(slot,actual);return step(at+1,after);};return later?capture(value,null,resume):resume(value);},depth+1,null);};return step(0,environment);}
 function visit(value,environment,next,depth,want){if(++arena.work>readRustOwnedRecordEffectLimit('work')||depth>readRustOwnedRecordEffectLimit('depth'))fail('ordered state construction exceeds its finite source budget',value.span);
  if(value.k==='lambda'){const body=visit(value.body,new Map(),result=>result,depth+1,value.ret);return next(mark({...value,body},value),environment);}
  if(!has(value,true))return next(value,environment);
  const form=readRustOwnedRecordEffectForm(value.k,value.op??'');
  if(form==='read state'){const slot=arena.reads[Number(value.name)],actual=slot?environment.get(slot.lexical):null;if(!actual)fail('a mutable read needs its actual initialized lexical state in this callable',value.span);return next(mark({k:'name',path:[actual.name]},value),environment);}
  if(form==='write state'){const info=arena.writes[Number(value.name)];if(!info||!environment.has(info.slot.lexical))fail('a mutable write cannot leave its actual lexical declaration scope',value.span);return visit(value.args[0].value,environment,(rhs,after)=>{const previous=after.get(info.slot.lexical),rhsName=parser.freshProductBinding(),stateName=parser.freshProductBinding(),updated=new Map(after);updated.set(info.slot.lexical,{slot:info.slot,name:stateName});const snapshot=reconstructRustOwnedRecordState(parser,path,info.layouts,{k:'name',path:[previous.name]},{k:'name',path:[rhsName]},value.span),body=next({k:'unit',span:value.span},updated);return {k:'let',name:rhsName,type:readRustRecordSourceType(parser,info.field.type,info.field.path),value:rhs,body:{k:'let',name:stateName,type:readRustRecordSourceType(parser,info.slot.origin.type,info.slot.origin.path),value:snapshot,body},span:value.span};},depth+1,info.field.type);}
  if(form==='binding')return visit(value.value,environment,(rhs,after)=>{const inner=new Map(after),slot=declarations.get(value.name);if(slot)inner.set(slot.lexical,{slot,name:value.name});return mark({...value,value:rhs,body:visit(value.body,inner,(result,done)=>{const outer=new Map(done);if(slot)outer.delete(slot.lexical);return next(result,outer);},depth+1,want)},value);},depth+1,value.type??null);
  if(form==='print')return visit(value.expr,environment,(expr,after)=>mark({...value,expr,body:visit(value.body,after,next,depth+1,want)},value),depth+1,null);
  if(form==='conditional')return visit(value.cond,environment,(condition,after)=>phi(value,after,branch=>mark({...value,cond:condition,then:visit(value.then,new Map(after),branch,depth+1,want),else:visit(value.else,new Map(after),branch,depth+1,want)},value),next,depth,want),depth+1,{kind:'bool'});
  if(form==='short circuit')return visit(value.left,environment,(left,after)=>phi(value,after,branch=>{const short=readRustExpressionCompletionShortCircuitValue(value.op),done=branch({k:'bool',value:short},new Map(after)),right=visit(value.right,new Map(after),branch,depth+1,{kind:'bool'});return mark({k:'if',cond:left,then:short?done:right,else:short?right:done,span:value.span},value);},next,depth,{kind:'bool'}),depth+1,{kind:'bool'});
  if(form==='match')return ordered(value,value.k==='match1'?['scrutinee']:['scrutinees'],environment,(selected,after)=>phi(value,after,branch=>value.k==='match1'?mark({...selected,cases:value.cases.map(row=>({...row,body:visit(row.body,new Map(after),branch,depth+1,want)}))},value):mark({...selected,rows:value.rows.map(row=>({...row,body:visit(row.body,new Map(after),branch,depth+1,want)}))},value),next,depth,want),depth,null);
  if(form==='packet'){const operation=readRustExpressionCompletionPacketOperation(value.op);if(operation==='result')return packetVisit(value.args[0].value,1,environment,next,after=>visit(value.args[1].value,after,next,depth+1,want),depth+1,want);if(operation==='success')return visit(value.args[0].value,environment,(child,after)=>next(mark({...value,args:[{...value.args[0],value:child}]},value),after),depth+1,null);}
  const roles=readRustOwnedRecordEffectChildren(value.k);if(roles.length)return ordered(value,roles,environment,next,depth,want);
  fail('the actual source expression form needs its complete ordered state roles',value.span);
 }
 function packetVisit(value,levels,environment,next,failed,depth,want){if(++arena.work>readRustOwnedRecordEffectLimit('work')||depth>readRustOwnedRecordEffectLimit('depth'))fail('conditional state packets exceed their source budget',value.span);const role=value.k==='math'?readRustExpressionCompletionPacketOperation(value.op):'';if(role==='success')return levels===1?visit(value.args[0].value,environment,next,depth+1,want):packetVisit(value.args[0].value,levels-1,environment,next,failed,depth+1,want);if(role==='failure')return failed(environment);if(role==='result')return packetVisit(value.args[0].value,levels+1,environment,next,after=>packetVisit(value.args[1].value,levels,after,next,failed,depth+1,want),depth+1,want);if(value.k==='if')return visit(value.cond,environment,(cond,after)=>mark({...value,cond,then:packetVisit(value.then,levels,new Map(after),next,failed,depth+1,want),else:packetVisit(value.else,levels,new Map(after),next,failed,depth+1,want)},value),depth+1,{kind:'bool'});if(value.k==='let')return visit(value.value,environment,(rhs,after)=>{const inner=new Map(after),slot=declarations.get(value.name);if(slot)inner.set(slot.lexical,{slot,name:value.name});return mark({...value,value:rhs,body:packetVisit(value.body,levels,inner,(result,done)=>{const outer=new Map(done);if(slot)outer.delete(slot.lexical);return next(result,outer);},failed,depth+1,want)},value);},depth+1,value.type??null);if(value.k==='match'||value.k==='match1')return ordered(value,value.k==='match1'?['scrutinee']:['scrutinees'],environment,(selected,after)=>value.k==='match1'?mark({...selected,cases:value.cases.map(row=>({...row,body:packetVisit(row.body,levels,new Map(after),next,failed,depth+1,want)}))},value):mark({...selected,rows:value.rows.map(row=>({...row,body:packetVisit(row.body,levels,new Map(after),next,failed,depth+1,want)}))},value),depth,null);fail('the conditional state packet needs its actual complete producer',value.span);}
 return visit(node,initial,finished,0,expected);
}
function lowerMain(parser,main,budget){
 const inventory=new Map(),names=new Map(),output=[];let environment=new Map();
 const rename=node=>alpha(parser,node,names,inventory,[],budget);
 // Main retains its effect algebra. A value/state packet completes before
 // the following effect, including the original structural assertion.
 const evaluate=(node,expected=null)=>{
  const keys=[...environment.keys()],packetName=parser.freshProductBinding(),reference={k:'name',path:[packetName]};
  const value=lower(parser,node,[],inventory,environment,expected,(result,after)=>({k:'tuple',items:[result,...keys.map(key=>({k:'name',path:[after.get(key).name]}))]}));
  const resultType=typeOf(parser,node,[],expected),type=resultType?{kind:'tuple',elements:[resultType,...keys.map(key=>readRustRecordSourceType(parser,environment.get(key).slot.origin.type,environment.get(key).slot.origin.path))]}:null;
  output.push({k:'let',name:packetName,...(type?{type}:{}),value});
  const actual=parser.origins.origin(node,[]),selected=parser.origins.mark({k:'tupleIndex',object:reference,index:0},actual),after=new Map(environment);
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
  const renamed=renameProp(prop);if(hasWrite(renamed))return {p:'bool',expr:evaluate(propositionExpression(renamed),{kind:'bool'})};return pureRead(renamed);
 };
 const hasWrite=node=>{const pending=[node];let count=0;while(pending.length){const value=pending.pop();if(!value||typeof value!=='object')continue;if(++count>readRustOwnedRecordEffectLimit('inspection'))fail('the assertion exceeds its source inspection budget');if(marker(value)&&value.op===readRustOwnedRecordEffectOperation('write'))return true;pending.push(...Object.values(value));}return false;};
 const propositionExpression=prop=>{
  if(prop.p==='bool')return prop.expr;
  if(['eq','ne','lt','le','gt','ge'].includes(prop.p)){const left=typeOf(parser,prop.left,[]),right=typeOf(parser,prop.right,[]);if(!acceptRustOwnedRecordAssertionComparison(left?.kind??right?.kind??''))fail('stateful structural assertions need the checked proposition value bridge',prop.span);return {k:'binary',op:prop.p,left:prop.left,right:prop.right,span:prop.span};}
  if(prop.p==='not')return {k:'unary',op:'not',arg:propositionExpression(prop.arg),span:prop.span};
  if(prop.p==='and'||prop.p==='or')return {k:'binary',op:prop.p,left:propositionExpression(prop.left),right:propositionExpression(prop.right),span:prop.span};
  if(prop.p==='implies')return {k:'binary',op:'or',left:{k:'unary',op:'not',arg:propositionExpression(prop.left)},right:propositionExpression(prop.right),span:prop.span};
  fail('a stateful quantified assertion needs its checked proposition continuation',prop.span);
 };
 const renameProp=prop=>prop.p==='bool'?{...prop,expr:rename(prop.expr)}:['eq','ne','lt','le','gt','ge'].includes(prop.p)?{...prop,left:rename(prop.left),right:rename(prop.right)}:prop.p==='not'?{...prop,arg:renameProp(prop.arg)}:['and','or','implies'].includes(prop.p)?{...prop,left:renameProp(prop.left),right:renameProp(prop.right)}:prop;
 for(const effect of main.effects){
  if(effect.k==='let'){
   const old=effect.name,name=parser.freshProductBinding(),slot=readRustOwnedRecordStateDeclarations(parser).get(old),value=evaluate(rename(effect.value),effect.type??null);
   output.push({...effect,name,value});names.set(old,name);if(slot){inventory.set(name,slot);environment.set(slot.lexical,{slot,name});}
  }else if(effect.k==='print')output.push({...effect,expr:evaluate(rename(effect.expr))});
  else if(effect.k==='assert')output.push({...effect,prop:assertion(effect.prop)});
  else fail('the actual main effect needs its existing ordered consumer',effect.span);
 }
 return {...main,effects:output};
}
export function finishRustOwnedRecordEffects(parser,program){
 if(!parser.ownedRecordEffects||!contains(program,true))return program;const inventory=new Map(),budget={nodes:0};
 const data=[];const collect=(items,path)=>{for(const item of items)if(item.k==='module')collect(item.items,[...path,item.name]);else if(item.k==='data')data.push({path,declaration:item});};collect(program.items,[]);effects(parser).data=data;
 const expression=(node,path,expected=null)=>{if(!contains(node,true))return node;const renamed=alpha(parser,node,new Map(),inventory,path,budget);return lower(parser,renamed,path,inventory,new Map(),expected);};
 const items=(values,path)=>values.map(item=>item.k==='module'?{...item,items:items(item.items,[...path,item.name])}:item.k==='fn'?{...item,body:expression(item.body,path,item.ret)}:item);
 const result={...program,items:items(program.items,[]),...(program.main?{main:lowerMain(parser,program.main,budget)}:{})};
 if(contains(result,true))fail('a private record state operation escaped its ordered consumer',program.main?.span);
 return result;
}
