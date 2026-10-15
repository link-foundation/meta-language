// Parse the entire ordered source. No selected function text is extracted.
import {tokenize} from '../../js/src/translation/lexer.js';
import {unsupported} from '../../js/src/translation/diagnostics.js';
import {sameType} from '../../js/src/translation/types.js';
import {ownedLocalType,ownedLocalPair} from '../../js/src/translation/rust-owned-local-domains.js';
import {createRustSourceSignature,readRustSourceTypeOwner} from '../../js/src/translation/rust-source-signatures.js';
import {acceptRustPureUnitName,acceptRustPureUnitNameCharacter} from '../../js/src/translation/rust-pure-unit-call-rules.js';
import {acceptRustRuntimeShellReservedName,acceptRustRuntimeShellStackShift} from '../../js/src/translation/rust-runtime-shell-rules.js';
import {acceptGeneratedOwnedRustClone,acceptGeneratedOwnedRustLiteralCodePoint,acceptGeneratedOwnedRustEqualityGrouping} from './rules.mjs';
import {acceptRustOwnedLocalTextInput} from '../../js/src/translation/rust-owned-local-state-rules.js';
import {readRustStringContinuationDraft} from '../issue-199-rust-string-continuation-adapter.mjs';
import {readGeneratedOwnedRustTrivia} from './trivia.mjs';
const fail=(reason,span)=>{throw unsupported('generated whole owned Rust',reason,span);};
const range=(first,last)=>({start:first.span.start,end:last.span.end});
export const tokenName=token=>token.declaration?.name??token.binding?.name??token.binding?.value??token.literal?.value??token.value;
export function readGeneratedOwnedRustCookedLiteral(raw,policy){
 if(!policy.literalValueEdits)return readRustStringContinuationDraft(raw).value;
 const tokens=tokenize(raw,'Rust').tokens,token=tokens[0];
 if(tokens.length!==2||token.kind!=='string'||token.start!==0||token.end!==raw.length||!raw.startsWith('"')||tokens[1].kind!=='eof')fail('the literal needs one complete actual Rust cooked-string token');
 return token.value;
}
export function readGeneratedOwnedRust(tokens,policy,{program=null,catalog=null}={}){
 let index=0,visits=0;const helperRows=[],bindings=[],uses=[],calls=[],literals=[],clones=[],captures=new Map(),runtimeUses=[];
 const current=()=>tokens[index],take=()=>{if(++visits>100_000||!current())fail('the complete source exceeds its operation limit',current()?.span);return tokens[index++];};
 const literal=value=>{const token=take();if(token.value!==value||token.declaration||token.binding||token.literal)fail('the complete grammar expected '+value,token.span);return token;};
 const operation=(op,parts)=>({kind:'source operation',op,parts,span:range(parts[0],parts.at(-1))});
 const ordinary=(name,local=false)=>typeof name==='string'&&(acceptRustPureUnitName(name)||local&&new RegExp(policy.generatedBindingPattern,'u').test(name))&&acceptRustRuntimeShellReservedName(name)&&!policy.reservedOwners.includes(name)&&[...name].every((letter,at)=>acceptRustPureUnitNameCharacter(letter.codePointAt(0),at===0));
 function binder(local=false){const token=take();if(token.kind!=='identifier'||!ordinary(tokenName(token),local)||!ordinary(token.raw,local))fail('the actual identifier needs a separate owner interpretation',token.span);return token;}
 function readType(){
  if(current().value==='('){literal('(');const left=readType();literal(',');const right=readType();literal(')');const type={kind:'tuple',elements:[left.type,right.type]};if(!ownedLocalPair(type))fail('only the ordered owned String/i32 product is admitted',current().span);return{type,wrappers:[],sourceLifetimes:[],sourceOwner:readRustSourceTypeOwner(ownerAdapter,['('],[left.sourceOwner,right.sourceOwner],type)};}
  const token=take(),type=policy.domains[token.value];if(token.kind!=='identifier'||!type||token.binding||token.declaration||token.literal)fail('only actual direct standard String, i32 and Boolean types are admitted',token.span);
  return{type:structuredClone(type),wrappers:[],sourceLifetimes:[],sourceOwner:readRustSourceTypeOwner(ownerAdapter,[token.value],[],type)};
 }
 const ownerAdapter={modulePath:[],resolve:path=>path,origins:{lookup:()=>null}};
 const actualLiteral=(token,kind)=>{
  if(program){const node=token.literal;if(!node||node.k!=='lit'||node.type.kind!==kind)fail('a literal lost its actual checked value owner',token.span);return node.value;}
  return token.value;
 };
 function expression(scope,depth=0){
  if(depth>128)fail('the complete value exceeds its recursive operation limit',current()?.span);
  let result;
  if(current().value==='if'){
   const first=literal('if'),condition=expression(scope,depth+1),then=block(new Map(scope),depth+1),otherwise=literal('else'),alternate=block(new Map(scope),depth+1);
   if(condition.type.kind!=='bool'||!sameType(then.type,alternate.type))fail('the actual conditional domains differ',first.span);
   result={surface:{k:'if',cond:condition.surface,then:then.surface,else:alternate.surface,span:range(first,alternate.node)},type:then.type,node:operation('owned conditional value',[first,condition.node,then.node,otherwise,alternate.node])};
  }else if(current().value==='{')result=block(new Map(scope),depth+1);
  else if(current().value==='('){
   const open=literal('('),left=expression(scope,depth+1);
   if(current().value===','){const comma=literal(','),right=expression(scope,depth+1),close=literal(')'),type={kind:'tuple',elements:[left.type,right.type]};if(!ownedLocalPair(type))fail('the actual ordered product has different value domains',open.span);result={surface:{k:'tuple',items:[left.surface,right.surface],span:range(open,close)},type,node:operation('owned ordered product',[open,left.node,comma,right.node,close])};}
   else{const close=literal(')');result={...left,node:operation('owned parenthesized value',[open,left.node,close])};}
  }else if(current().value==='String'){
   const start=literal('String'),separator=literal('::'),method=literal('from'),open=literal('('),token=take();
   if(token.kind!=='string'||!token.raw.startsWith('"'))fail('the standard String constructor requires its actual cooked literal argument',token.span);
   readGeneratedOwnedRustCookedLiteral(token.raw,policy);
   const value=actualLiteral(token,'string'),surface={k:'str',value,span:token.span},close=literal(')');
   if(!acceptRustOwnedLocalTextInput(value)||!policy.literalValueEdits&&[...value].some(letter=>!acceptGeneratedOwnedRustLiteralCodePoint(letter.codePointAt(0))))fail('the literal requires Unicode scalar text and its supported Rust quoting domain',token.span);
   result={surface,type:{kind:'string'},node:operation('owned standard String literal construction',[start,separator,method,open,token,close])};literals.push({token,surface,node:result.node,sourceSign:null});
  }else if(current().value==='crate'){
   const first=literal('crate'),separator=literal('::'),token=binder(),open=literal('('),args=[],parts=[first,separator,token,open];
   while(current().value!==')'){const value=expression(scope,depth+1);args.push(value);parts.push(value.node);if(current().value!==',')break;parts.push(literal(','));}
   parts.push(literal(')'));const name=tokenName(token),target=catalog?.get(name);
   if(scope.has(name)||catalog&&(!target||!ownedLocalPair(target.ret)||target.params.length!==2||!sameType(target.params[0].type,policy.domains.String)||!sameType(target.params[1].type,policy.domains.i32))||args.length!==2||!sameType(args[0].type,policy.domains.String)||!sameType(args[1].type,policy.domains.i32))fail('a product call needs its actual closed declared String/i32 target',token.span);
   if(program&&(!token.declaration||program.declarations.get(token.declaration.fullName)!==token.declaration||token.declaration.name!==name))fail('a call token references an unowned declaration',token.span);
   result={surface:{k:'app',fn:{k:'name',path:['crate',name],span:range(first,token)},args:args.map(value=>value.surface),span:range(first,parts.at(-1))},type:target?.ret??{kind:'tuple',elements:args.map(value=>value.type)},node:operation('owned declared product call',parts)};
   calls.push({token,surface:result.surface,result});
  }else if(current().raw==='true'||current().raw==='false'){
   const token=take(),value=program?actualLiteral(token,'bool'):token.value==='true',surface={k:'bool',value,span:token.span};if(typeof value!=='boolean')fail('a Boolean token has a different primitive domain',token.span);literals.push({token,surface});result={surface,type:{kind:'bool'},node:operation('owned Boolean literal',[token])};
  }else if(current().kind==='number'||current().value==='-'){
   const parts=[];if(current().value==='-')parts.push(literal('-'));const token=take();parts.push(token);
   const spelling=token.literal?token.raw.slice(0,-3):token.value;
   if(token.kind!=='number'||token.suffix!=='i32'||token.floating||!/^\d+$/u.test(spelling))fail('the literal requires its actual signed i32 suffix and value',token.span);
   const value=program?actualLiteral(token,'fixed'):(parts.length===2?'-':'')+token.value;if(typeof value!=='string'||!/^[-]?\d+$/u.test(value)||BigInt(value)<-2147483648n||BigInt(value)>2147483647n)fail('the actual integer literal exceeds signed i32',token.span);
   if(parts.length===2&&!value.startsWith('-')&&BigInt(value)!==0n&&!policy.literalValueEdits)fail('a negative source operation cannot become a positive value without a new sign interpretation',token.span);
   const surface={k:'num',value:value.replace(/^-/u,''),negative:value.startsWith('-'),type:policy.domains.i32,span:range(parts[0],token)};result={surface,type:policy.domains.i32,node:operation('owned signed i32 literal',parts)};literals.push({token,surface,node:result.node,sourceSign:parts.length===2?parts[0]:null});
  }else{
   const token=binder(true),name=tokenName(token),entry=scope.get(name);
   if(!entry||program&&token.binding!==entry.token.binding)fail('a value token lost its actual lexical owner',token.span);
   const surface={k:'name',path:[name],span:token.span};result={surface,type:entry.type,node:operation('owned lexical value',[token]),read:{token,type:entry.type,cloned:false,projected:false}};uses.push({...result.read,surface,entry,read:result.read});
  }
  while(current()?.value==='.'){
   const dot=literal('.'),token=take();
   if(token.kind==='number'){
    if(!ownedLocalPair(result.type)||!['0','1'].includes(token.value)||token.suffix||token.floating||result.surface.k!=='name')fail('the actual field needs its original ordered local product',token.span);
    result.read.projected=true;const field=Number(token.value),surface={k:'tupleIndex',object:result.surface,index:field,span:range(result.node,token)},type=result.type.elements[field];
    result={surface,type,node:operation('owned product field',[result.node,dot,token]),read:{token,type,cloned:false,projected:false}};if(type.kind==='string')uses.push({surface,read:result.read});
   }else{
    const open=literal('('),close=literal(')');
    if(token.value!==policy.clone.method||!acceptGeneratedOwnedRustClone(true,result.type.kind==='string',policy.clone.receiverKinds.includes(result.surface.k)&&!result.read?.cloned,0))fail('only an actual unshadowed zero-argument standard String clone is admitted',token.span);
    result.read.cloned=true;clones.push({receiver:result.surface,node:operation('owned standard String clone',[result.node,dot,token,open,close])});result={...result,node:clones.at(-1).node};
   }
  }
  if(current()?.value==='=='){
   const operator=literal('=='),right=expression(scope,depth+1);
   if(!acceptGeneratedOwnedRustEqualityGrouping(right.surface.k==='binary',right.node.op==='owned parenthesized value'))fail('an actual nested comparison requires its Rust parentheses',operator.span);
   if(!ownedLocalType(result.type)||result.type.kind==='string'||!sameType(result.type,right.type))fail('equality requires actual Boolean or signed i32 operands',operator.span);
   result={surface:{k:'binary',op:'eq',left:result.surface,right:right.surface,span:range(result.node,right.node)},type:{kind:'bool'},node:operation('owned scalar equality',[result.node,operator,right.node])};
  }
  return result;
 }
 function block(scope,depth){
  const open=literal('{'),statements=[],parts=[open];
  while(current().value==='let'){
   const first=literal('let'),name=binder(true);if(scope.has(tokenName(name)))fail('an immutable local duplicates or shadows a source binder',name.span);const equal=literal('='),value=expression(scope,depth+1),semicolon=literal(';');
   const node=operation('owned immutable local',[first,name,equal,value.node,semicolon]),row={token:name,type:value.type,surface:null,value};scope.set(tokenName(name),row);bindings.push(row);statements.push(row);parts.push(node);
   if(value.surface.k==='app')value.completeInitializer=true;
  }
  if(current().value==='}')fail('a value body needs one actual final expression',current().span);
  const tail=expression(scope,depth+1);parts.push(tail.node);const close=literal('}');parts.push(close);let surface=tail.surface;
  for(const row of statements.toReversed()){row.surface={k:'let',name:tokenName(row.token),value:row.value.surface,body:surface,span:range(row.token,row.value.node)};surface=row.surface;}
  return{surface,type:tail.type,node:operation('owned value block',parts)};
 }
 function helper(){
  const begin=index,parts=[literal('pub'),literal('fn')],name=binder();parts.push(name,literal('('));const parameters=[],scope=new Map();
  while(current().value!==')'){
   const token=binder(),colon=literal(':'),start=index,origin=readType(),parameter={name:tokenName(token),type:origin.type};if(scope.has(parameter.name))fail('the actual parameter owner is duplicated',token.span);parts.push(token,colon,...tokens.slice(start,index));const row={token,type:origin.type,surface:parameter};parameters.push({...parameter,origin,row});scope.set(parameter.name,row);bindings.push(row);if(current().value!==',')break;parts.push(literal(','));
  }
  parts.push(literal(')'),literal('->'));const typeStart=index,result=readType();parts.push(...tokens.slice(typeStart,index));
  if(result.type.kind!=='string'&&!ownedLocalPair(result.type))fail('only String or ordered String/i32 results are admitted',name.span);
  const beforeCalls=calls.length,body=block(scope,0);parts.push(body.node);
  if(!sameType(body.type,result.type)||ownedLocalPair(result.type)&&(body.surface.k!=='tuple'||calls.length!==beforeCalls||parameters.length!==2||!sameType(parameters[0].type,policy.domains.String)||!sameType(parameters[1].type,policy.domains.i32)))fail('the actual helper result differs from its closed owned interface',name.span);
  if(!ownedLocalPair(result.type)&&calls.length-beforeCalls!==1)fail('a generated String caller requires one actual complete product call initializer',name.span);
  const signature=createRustSourceSignature('Rust source function signature 1',parameters.map(parameter=>parameter.origin),result,[]);
  const surface={k:'fn',name:tokenName(name),params:parameters.map(({name,type})=>({name,type})),ret:result.type,body:body.surface,span:range(tokens[begin],tokens[index-1]),sourceSignature:signature};
  if(program&&(!name.declaration||program.declarations.get(name.declaration.fullName)!==name.declaration||name.declaration.name!==surface.name))fail('the header lost its actual declaration identity',name.span);
  const row={name,surface,parameters,node:operation('owned generated value helper',parts)};helperRows.push(row);return row.node;
 }
 function match(descriptor){
  if(typeof descriptor==='string')return literal(descriptor);
  if(descriptor.operation)return operation(descriptor.operation,descriptor.pattern.map(match));
  if(descriptor.bind){const token=take(),name=tokenName(token);if(token.kind!=='identifier'||(descriptor.fixed!==undefined?name!==descriptor.fixed:!acceptRustPureUnitName(name)))fail('the runtime binder differs from its shared closed grammar',token.span);if(token.binding||token.declaration||token.literal)fail('the runtime binder must retain its original token identity',token.span);captures.set(descriptor.bind,token);return token;}
  if(descriptor.reference){const token=take(),target=captures.get(descriptor.reference);if(!target||token.kind!=='identifier'||(program?token.binding!==target:token.value!==target.value))fail('a runtime occurrence lost its actual token binder',token.span);runtimeUses.push({token,target});return token;}
  if(descriptor.string!==undefined){const token=take();if(token.kind!=='string'||token.value!==descriptor.string||token.literal||token.binding||token.declaration)fail('the runtime literal differs from its actual shared operation',token.span);return token;}
  if(descriptor.number){const token=take();if(token.kind!=='number'||token.suffix||token.floating||!acceptRustRuntimeShellStackShift(Number(token.value)))fail('the runtime stack shift differs from its bounded source role',token.span);return token;}
  if(descriptor.repeat){if(current().value!==descriptor.until)fail('the generated worker requires its actual empty Unit body',current().span);return{kind:'source operation',op:descriptor.repeat,parts:[],span:{...current().span,end:current().span.start}};}
  fail('an unknown shared runtime descriptor needs a separate source rule',current()?.span);
 }
 const parts=[match(policy.syntax.attribute)];while(current().value==='pub')parts.push(helper());
 if(!helperRows.length||new Set(helperRows.map(row=>tokenName(row.name))).size!==helperRows.length)fail('the whole module requires its actual distinct value declarations',current().span);
 const worker=match(policy.syntax.worker);parts.push(worker,match(policy.syntax.entry));const eof=take();if(eof.kind!=='eof'||index!==tokens.length)fail('the source interpretation must consume the complete artifact',eof.span);parts.push(eof);
 for(const use of uses){if(use.read.type.kind==='string'&&!use.read.cloned||use.read.type.kind==='tuple'&&!use.read.projected)fail('every actual String read must retain its standard clone and each tuple read its field role',use.read.token.span);}
 for(const call of calls)if(!call.result.completeInitializer)fail('the actual product call must be a complete once-evaluated local initializer',call.token.span);
 const operations=operation('complete generated owned Rust module',parts),surface={language:'Rust',items:helperRows.map(row=>row.surface),main:{effects:[],span:worker.span}};
 const single=label=>{const found=[];const visit=node=>{if(node?.kind==='source operation'){if(node.op===label)found.push(node);node.parts.forEach(visit);}};visit(operations);if(found.length!==1)fail('the complete module requires one actual '+label,operations.span);return found[0];};
 const shift=single('unsigned stack size shift');if(shift.parts[1].span.end!==shift.parts[2].span.start)fail('the actual stack shift operator must remain contiguous',shift.span);
 return{surface,operations,helperRows,bindings,uses,calls,literals,clones,runtimeUses,single};
}
export function parseGeneratedOwnedRust(source,policy){
 if(!acceptRustOwnedLocalTextInput(source))fail('Rust source must contain only Unicode scalar text');
 const lexical=tokenize(source,'Rust'),tokens=[],gaps=[];let end=0;
 for(const token of lexical.tokens){if(!['identifier','punct','number','string','eof'].includes(token.kind))fail('the lexical construct has no generated owned interpretation',{start:token.start,end:token.end});const units=[];for(let at=end;at<token.start;at++)units.push(source.charCodeAt(at));readGeneratedOwnedRustTrivia(units,end,policy);gaps.push({span:{start:end,end:token.start},units});tokens.push({kind:token.kind,value:token.value,raw:source.slice(token.start,token.end),span:{start:token.start,end:token.end},...(token.kind==='number'?{suffix:token.suffix??'',floating:Boolean(token.floating)}:{})});end=token.end;}
 const discovery=readGeneratedOwnedRust(tokens,policy),catalog=new Map(discovery.helperRows.map(row=>[tokenName(row.name),row.surface]));const parsed=readGeneratedOwnedRust(tokens,policy,{catalog});
 return{...parsed,proof:{model:policy.model,language:policy.language,observationModel:policy.observationModel,span:{start:0,end:source.length},tokens,gaps,operations:parsed.operations,assumptions:[],valueAssumptions:[],worker:null,clones:[]}};
}
