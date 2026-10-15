// A finite operation certificate over the COMPLETE actual production token stream.
// The production parser, not this recognizer, constructs and checks the core.
import {tokenize} from '../../js/src/translation/lexer.js';
import {parseRustTokenStream} from '../../js/src/translation/rust.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {ownedLocalType,ownedLocalPosition} from '../../js/src/translation/rust-owned-local-domains.js';
import {checkRustSourceSignature} from '../../js/src/translation/rust-source-signatures.js';
import {acceptRustPureUnitName,acceptRustPureUnitNameCharacter} from '../../js/src/translation/rust-pure-unit-call-rules.js';
import {acceptRustOwnedLocalTextInput} from '../../js/src/translation/rust-owned-local-state-rules.js';
import {readRustTextValueProjectionDomain,acceptRustTextValueProjectionOperation} from './rules.mjs';
export const fail=reason=>{throw new TypeError('Rust text value projection: '+reason);};
export const tokenValue=token=>token.declaration?.name??token.binding?.name??token.literal?.value??token.value;
export function textPositionDomain(position){
 if(!position||!ownedLocalType(position.type)||position.type.kind!=='string'||!Array.isArray(position.wrappers)||position.wrappers.some(layer=>layer!=='reference')||position.lifetimes.some(lifetime=>lifetime!==null)||position.owner.arguments.length)fail('a text position requires its actual direct standard owner and elided immutable reference layers');
 const standard=ownedLocalPosition({...position,wrappers:[],lifetimes:[]}),slice=position.owner.kind==='primitive'&&position.owner.path.length===1&&position.owner.path[0]==='str';
 const domain=readRustTextValueProjectionDomain(standard,slice,position.wrappers.length);if(!domain)fail('this actual text owner or reference depth needs a separate interpretation');return domain;
}
export function parseTextProjectionCore(tokens,policy){
 const actual=tokens.map(token=>({kind:token.kind,value:tokenValue(token),raw:token.raw,start:token.span.start,end:token.span.end,...(token.kind==='number'?{suffix:token.suffix,floating:token.floating}:{})}));
 return checkProgram(parseRustTokenStream(actual,{sourceSignatureModel:policy.signatureModel}));
}
export function tokenizeTextProjection(source){
 if(typeof source!=='string'||!acceptRustOwnedLocalTextInput(source))fail('the complete Rust source requires primitive Unicode scalar text');
 const tokens=[],gaps=[];let end=0;
 for(const token of tokenize(source,'Rust').tokens){const units=[];for(let at=end;at<token.start;at++)units.push(source.charCodeAt(at));gaps.push({span:{start:end,end:token.start},units});tokens.push({kind:token.kind,value:token.value,raw:source.slice(token.start,token.end),span:{start:token.start,end:token.end},...(token.kind==='number'?{suffix:token.suffix??'',floating:Boolean(token.floating)}:{})});end=token.end;}
 return{tokens,gaps,span:{start:0,end:source.length}};
}
export function readTextProjectionSource(tokens,program,policy,{bound=true}={}){
 let at=0;const functions=[],roles=[],seen=new Set();
 const current=()=>tokens[at],take=()=>{if(at>=tokens.length)fail('the complete source ended inside an operation');return tokens[at++];};
 const fixed=value=>{const token=take();if(token.value!==value||['declaration','binding','literal'].some(role=>Object.hasOwn(token,role)))fail('the actual source expected fixed syntax '+value);return token;};
 const operation=(op,parts)=>({kind:'source operation',op,parts,span:{start:parts[0].span.start,end:parts.at(-1).span.end}});
 const name=token=>{const value=tokenValue(token),valid=value=>typeof value==='string'&&acceptRustPureUnitName(value)&&!policy.reservedNames.includes(value)&&[...value].every((letter,index)=>acceptRustPureUnitNameCharacter(letter.codePointAt(0),index===0));if(token.kind!=='identifier'||!valid(value)||!valid(token.raw))fail('the actual source identifier has no unshadowed closed owner');return value;};
 const role=(token,kind,owner)=>{if(bound?token[kind]!==owner:token.value!==owner.name&&kind!=='literal')fail('a source token lost its actual '+kind+' owner');roles.push({token,kind,owner});};
 const parts=[];
 while(current()?.value==='pub'||current()?.value==='fn'&&tokens[at+1]?.value!=='main'){
  const header=[];if(current().value==='pub')header.push(fixed('pub'));header.push(fixed('fn'));const nameToken=take(),fullName=name(nameToken),entry=program.declarations.get(fullName);
  if(!entry||entry.fullName!==fullName||entry.name!==fullName||entry.modulePath.length||seen.has(entry))fail('the complete source declaration is not its actual unique flat owner');seen.add(entry);role(nameToken,'declaration',entry);header.push(nameToken,fixed('('));
  const positions=[],scope=new Map();let parameter=0;
  while(current().value!==')'){
   if(parameter>=policy.maximumParameters)fail('the text interface exceeds its explicit parameter bound');const token=take(),parameterName=name(token),actual=entry.params[parameter],position=entry.sourceSignature?.parameters[parameter];
   if(!actual||actual.name!==parameterName||scope.has(parameterName))fail('a source parameter differs from its actual ordered checked owner');role(token,'binding',actual);header.push(token,fixed(':'));
   const reference=current().value==='&';if(reference)header.push(fixed('&'));const type=take();if(type.kind!=='identifier'||!['String','str'].includes(type.value)||['declaration','binding','literal'].some(key=>Object.hasOwn(type,key))||!reference&&type.value==='str')fail('only actual String, &String and &str source positions are admitted');header.push(type);
   const domain=textPositionDomain(position),expected=type.value==='str'?policy.domains.slice:reference?policy.domains.reference:policy.domains.owned;if(domain!==expected)fail('the actual source leaf type differs from its checked owner');
   const row={input:actual,position,domain};scope.set(parameterName,row);positions.push(row);parameter++;if(current().value!==',')break;header.push(fixed(','));
  }
  if(entry.params.length!==parameter)fail('the actual source header lost an ordered parameter');header.push(fixed(')'),fixed('->'),fixed('String'));
  checkRustSourceSignature(entry.sourceSignature,entry.params.map(param=>param.type),entry.ret,'Rust');
  if(entry.sourceSignature.model!==policy.signatureModel||entry.sourceSignature.lifetimeBindings.length||textPositionDomain(entry.sourceSignature.result)!==policy.domains.owned)fail('the actual result must be an unborrowed unshadowed standard String');
  const open=fixed('{'),bodyParts=[];let label,input=null,position=null,literal=null,domain;
  const atom=()=>{const token=take();bodyParts.push(token);if(token.kind==='string'){
    literal=entry.body;if(literal.k!=='lit'||literal.type.kind!=='string'||typeof literal.value!=='string'||!acceptRustOwnedLocalTextInput(literal.value))fail('the actual text literal has no Unicode scalar checked value owner');role(token,'literal',literal);domain=policy.domains.literal;return;
   }
   const parameterName=name(token),row=scope.get(parameterName);if(!row)fail('the actual source value is not a referenced header input');input=row.input;position=row.position;domain=row.domain;role(token,'binding',input);
  };
  if(current().value==='String'){
   for(const syntax of['String','::','from','('])bodyParts.push(fixed(syntax));atom();bodyParts.push(fixed(')'));label='standard String construction from borrowed text';
  }else{
   atom();if(current().value==='.'){
    bodyParts.push(fixed('.'));const method=take();if(!['clone','to_owned','to_string'].includes(method.value)||['declaration','binding','literal'].some(key=>Object.hasOwn(method,key)))fail('the actual method has no selected standard operation');bodyParts.push(method,fixed('('),fixed(')'));
    label=method.value==='clone'?'standard String clone':method.value==='to_owned'?'standard borrowed str owned construction':'standard borrowed str display construction';
   }else label='owned standard String move';
  }
  if(!acceptRustTextValueProjectionOperation(label,domain))fail('the actual source receiver/argument owner cannot produce the selected owned String operation');
  const leaf=operation(label,bodyParts),close=fixed('}'),body=operation('text value body',[open,leaf,close]),node=operation('text value function',[...header,body]);parts.push(node);
  functions.push({declaration:entry,signature:entry.sourceSignature,resultPosition:entry.sourceSignature.result,operationNode:leaf,coreValue:entry.body,input,position,literal});
 }
 if(!functions.length||seen.size!==program.declarations.size||program.items.length!==functions.length)fail('the whole source must represent every actual checked text function');
 const main=[fixed('fn'),fixed('main'),fixed('('),fixed(')'),fixed('{'),fixed('}')];parts.push(operation('independent empty main',main));const eof=take();if(eof.kind!=='eof'||at!==tokens.length)fail('the source interpretation must consume every token of the complete module');parts.push(eof);
 return{operations:operation('complete standard Rust text value module',parts),functions,roles};
}
