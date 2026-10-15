// The entire actual module is read through a closed linked grammar. The CST
// retains all token spellings and trivia code units, including return/ASI facts.
import {tokenize} from '../../js/src/translation/lexer.js';
import {unsupported} from '../../js/src/translation/diagnostics.js';
import {acceptJavaScriptLineTerminator} from '../../js/src/translation/javascript-line-terminator-rules.js';
import {acceptGeneratedJavaScriptUnitName} from './rules.mjs';
const fail=(detail,span)=>{throw unsupported('generated JavaScript Unit source',detail,span);};
const word=name=>typeof name==='string'&&/^[A-Za-z_][A-Za-z0-9_]*$/u.test(name);
export function readGeneratedJavaScriptUnitName(name,policy,local=false){return word(name)&&acceptGeneratedJavaScriptUnitName(name,local&&new RegExp(policy.generatedLocalPattern,'u').test(name));}
export function readGeneratedJavaScriptUnitTrivia(units,span){
 let index=0,lineBreak=false,lineCommentAtEnd=false;
 while(index<units.length){const code=units[index];
  if([9,32].includes(code)||acceptJavaScriptLineTerminator(code)){if(acceptJavaScriptLineTerminator(code))lineBreak=true;index++;continue;}
  if(code===47&&units[index+1]===47){index+=2;while(index<units.length&&!acceptJavaScriptLineTerminator(units[index]))index++;if(index===units.length)lineCommentAtEnd=true;continue;}
  if(code===47&&units[index+1]===42){index+=2;let closed=false;while(index<units.length){if(acceptJavaScriptLineTerminator(units[index]))lineBreak=true;if(units[index]===42&&units[index+1]===47){index+=2;closed=true;break;}index++;}if(!closed)fail('the owned trivia contains an unclosed block comment',span);continue;}
  fail('the owned trivia contains an unrepresented active source character',span);
 }
 return {lineBreak,lineCommentAtEnd};
}
export function readGeneratedJavaScriptUnitSyntax(tokens,gaps,policy,{program=null}={}){
 let index=0;const captures=new Map(),helpers=new Map(),helperRows=[],statements=[],helperUses=[],localBindings=[],entryUses=[];
 const value=token=>token.declaration?.name??token.binding?.name??token.entryTarget?.name??token.value;
 const current=()=>tokens[index];
 const span=()=>current()?.span;
 const take=()=>{const token=current();if(!token)fail('the complete module has no next owned token',span());index++;return token;};
 const range=(first,last)=>({start:first.span.start,end:last.span.end});
 const single=(node,label)=>{const found=[];const visit=node=>{if(node?.kind==='source operation'){if(node.op===label)found.push(node);node.parts.forEach(visit);}};visit(node);if(found.length!==1)fail('the complete module requires one owned '+label,node.span);return found[0];};
 function match(descriptor){
  if(typeof descriptor==='string'){const token=take(),kind=word(descriptor)?'identifier':'punct';if(token.kind!==kind||token.value!==descriptor||token.declaration||token.binding||token.entryTarget)fail('the owned module grammar expected '+descriptor,token.span);return token;}
  if(descriptor.operation){const start=index,parts=descriptor.pattern.map(match),first=tokens[start]??tokens.at(-1),last=index>start?tokens[index-1]:first,node={kind:'source operation',op:descriptor.operation,parts,span:index>start?range(first,last):{start:first.span.start,end:first.span.start}};
   if(node.op==='owned generated Unit helper'){const name=captures.get('helper'),row={name,node,value:single(node,'owned null Unit return').parts[1]};helpers.set(value(name),row);helperRows.push(row);}
   if(node.op==='owned generated Unit binding')statements.push({name:captures.get('local'),node,call:single(node,'owned generated Unit invocation')});return node;
  }
  if(descriptor.bind){const token=take(),name=value(token),role=descriptor.bind;if(token.kind!=='identifier'||(role==='entry'?name!==policy.entryName:!readGeneratedJavaScriptUnitName(name,policy,role==='local')))fail('the actual '+role+' binding is outside the closed generated identifier domain',token.span);
   if(role==='helper'){if(program?!token.declaration||program.declarations.get(token.declaration.fullName)!==token.declaration:token.value===undefined||token.declaration||token.binding||token.entryTarget)fail('the helper header must own its actual declaration',token.span);if(helpers.has(name))fail('the actual helper binding is duplicated',token.span);}
   if(role==='local'){if(program?!program.main.effects.includes(token.binding):token.value===undefined||token.declaration||token.binding||token.entryTarget)fail('the local token must own its actual entry effect',token.span);localBindings.push(token);}
   if(role==='entry'){if(program?token.entryTarget!==program.main.sourceGeneratedJavaScriptEntry:token.value!==policy.entryName||token.declaration||token.binding||token.entryTarget)fail('the entry header must own the actual module entry identity',token.span);entryUses.push(token);}
   captures.set(role,token);return token;
  }
  if(descriptor.helperReference){const token=take(),row=helpers.get(value(token));if(token.kind!=='identifier'||!row||(program?token.declaration!==row.name.declaration:token.value===undefined||token.declaration||token.binding||token.entryTarget))fail('the actual call must target an owned Unit helper',token.span);helperUses.push({token,target:row.name});return token;}
  if(descriptor.entryReference){const token=take(),target=captures.get('entry');if(token.kind!=='identifier'||!target||value(token)!==value(target)||(program?token.entryTarget!==target.entryTarget:token.value!==policy.entryName||token.declaration||token.binding||token.entryTarget))fail('the final actual invocation must target the owned entry definition',token.span);entryUses.push(token);return token;}
  if(descriptor.directive){const token=take();if(token.kind!=='string'||token.value!=='use strict'||!policy.strictDirectives.includes(token.raw))fail('the actual directive must have an unescaped strict-mode spelling',token.span);return token;}
  if(descriptor.nullUnit){const position=index,token=take();if(token.kind!=='identifier'||token.value!=='null'||token.raw!=='null'||readGeneratedJavaScriptUnitTrivia(gaps[position].units,gaps[position].span).lineBreak)fail('the actual helper must return the singleton null Unit value without an intervening line terminator',token.span);return token;}
  if(descriptor.repeat){const start=index,parts=[];while(descriptor.whileHelper?current()?.value==='function'&&value(tokens[index+1])!==policy.entryName:value(current())!==descriptor.until){const before=index;parts.push(match(policy.syntax[descriptor.rule]));if(index<=before||index>=tokens.length)fail('the complete module operation sequence is not closed',span());}const first=tokens[start]??tokens.at(-1),last=index>start?tokens[index-1]:first;return {kind:'source operation',op:descriptor.repeat,parts,span:index>start?range(first,last):{start:first.span.start,end:first.span.start}};}
  if(descriptor.eof){const token=take();if(token.kind!=='eof'||token.value!==''||index!==tokens.length)fail('the interpretation must consume the entire actual module',token.span);return token;}
  fail('the linked generated module grammar has an unknown operation',span());
 }
 const operations=match(policy.syntax.module),entry=single(operations,'owned generated Unit entry');
 const names=new Set();for(const token of localBindings){const name=value(token);if(names.has(name)||helpers.has(name)||name===policy.entryName)fail('an actual entry local duplicates or shadows an owned helper or entry binding',token.span);names.add(name);}
 const items=helperRows.map(row=>({k:'fn',name:value(row.name),params:[],ret:{kind:'unit'},body:{k:'unit',span:row.value.span},span:row.node.span}));
 const effects=statements.map(row=>({k:'let',name:value(row.name),type:{kind:'unit'},constant:true,value:{k:'app',fn:{k:'name',path:[value(row.call.parts[0])],span:row.call.parts[0].span},args:[],span:row.call.span},span:row.node.span}));
 return {operations,helperRows,statements,helperUses,localBindings,entryUses,surface:{language:'JavaScript',items,main:{effects,span:entry.span}},single:label=>single(operations,label)};
}
export function parseGeneratedJavaScriptUnit(source,policy){
 const lexical=tokenize(source,'JavaScript'),tokens=[],gaps=[];let end=0;
 for(const token of lexical.tokens){if(!['identifier','punct','string','eof'].includes(token.kind))fail('the actual lexical construct needs a separate generated Unit interpretation',{start:token.start,end:token.end});const units=[];for(let offset=end;offset<token.start;offset++)units.push(source.charCodeAt(offset));gaps.push({span:{start:end,end:token.start},units});tokens.push({kind:token.kind,value:token.value,raw:token.raw,span:{start:token.start,end:token.end}});end=token.end;}
 const parsed=readGeneratedJavaScriptUnitSyntax(tokens,gaps,policy),proof={model:policy.model,language:policy.language,observationModel:policy.observationModel,valueEncoding:policy.valueEncoding,span:{start:0,end:source.length},tokens,gaps,operations:parsed.operations};
 return {...parsed,proof};
}
