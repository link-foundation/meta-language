// Faithful graph-role projection only: no source grammar or operation selection.
import {readRuntimeProjectData} from '../../js/src/project-data-loader.js';
import {checkRustSourceSignature} from '../../js/src/translation/rust-source-signatures.js';
import {GENERATED_OWNED_RUST_POLICY,checkGeneratedOwnedRustData,checkGeneratedOwnedRustFields as fields,checkGeneratedOwnedRustCst,checkGeneratedOwnedRustInterval} from '../generated-owned-rust/model.mjs';
import {RUST_TEXT_VALUE_PROJECTION_POLICY,createRustTextValueProjection} from '../rust-text-value-projection/model.mjs';
import {decodeGeneratedOwnedRust,encodeGeneratedOwnedRust} from '../generated-owned-rust/codec.mjs';
import {checkRustTextCertificate} from './rules.mjs';
export const RUST_TEXT_CERTIFICATE_POLICY=readRuntimeProjectData(new URL('./data/policy.lino',import.meta.url));
const policy=RUST_TEXT_CERTIFICATE_POLICY,source=RUST_TEXT_VALUE_PROJECTION_POLICY;
const fail=reason=>{throw new TypeError('Portable Rust text certificate: '+reason);};
const shapeType=type=>fields(type,['kind']);
export function projectRustTextCertificate(program,proof){
 checkGeneratedOwnedRustData(program,proof);fields(proof,source.fields.proof);checkGeneratedOwnedRustInterval(proof.span,true);
 // The old generic data/CST helpers certify descriptors and the actual complete
 // lexer/trivia. They do not choose this model's grammar, operation or owners.
 checkGeneratedOwnedRustCst(proof,{...GENERATED_OWNED_RUST_POLICY,literalTokenMode:source.literalTokenMode});
 fields(program,['schemaVersion','sourceLanguage','items','main','declarations']);fields(program.main,['effects','span']);checkGeneratedOwnedRustInterval(program.main.span,true);
 if(!(program.declarations instanceof Map)||![program.items,program.main.effects,proof.functions,proof.inputAssumptions,proof.valueAssumptions].every(Array.isArray))fail('a graph role lost its actual ordered container shape');
 const objects=new Map(),kinds=[0],symbols=new Map(policy.symbols.map((value,index)=>[value,index+1])),strings=[null,...policy.symbols];
 const identity=(value,kind=9)=>{if(value===null)return 0;if(value===undefined||typeof value!=='object')return -1;let id=objects.get(value);if(id===undefined){id=kinds.length;objects.set(value,id);kinds.push(kind);}else if(kind!==9){if(kinds[id]!==9&&kinds[id]!==kind)fail('an actual object occupies contradictory structural record roles');kinds[id]=kind;}return id;};
 const symbol=value=>{if(typeof value!=='string')return -1;let id=symbols.get(value);if(id===undefined){id=strings.length;symbols.set(value,id);strings.push(value);}return id;};
 const nullValue=value=>value===null?0:value===undefined?-1:identity(value);
 const out={context:[],kinds,tokens:[],declarations:[],parameters:[],positions:[],paths:[],layers:[],cores:[],functions:[],operations:[],parts:[],inputs:[],allocations:[],names:[],units:[],items:[]};
 const operationCodes=new Map(policy.operations.map(row=>[row.name,row.code]));for(const[key,value]of Object.entries(policy.operationCodes))operationCodes.set({function:'text value function',body:'text value body',main:'independent empty main',root:'complete standard Rust text value module'}[key],value);
 const walkOperation=node=>{if(node?.kind!=='source operation')return identity(node);const id=identity(node,7);if(out.operations.some((value,index)=>index%6===0&&value===id))return id;fields(node,['kind','op','parts','span']);checkGeneratedOwnedRustInterval(node.span,true);if(!Array.isArray(node.parts))fail('an operation lost its structural ordered container');const offset=out.parts.length;for(const part of node.parts)out.parts.push(identity(part));out.operations.push(id,operationCodes.get(node.op)??-1,node.span.start,node.span.end,offset,node.parts.length);for(const part of node.parts)if(part?.kind==='source operation')walkOperation(part);return id;};
 for(const token of proof.tokens){const roles=['declaration','binding','literal'].filter(key=>Object.hasOwn(token,key)),role=roles[0]??null,target=role?token[role]:null;out.tokens.push(identity(token,1),symbol(token.kind),role?0:symbol(token.value),role?policy.roles[role]:0,identity(target),symbol(role==='declaration'?target?.name:role==='binding'?target?.name:role==='literal'?target?.value:token.value),symbol(token.raw),token.span.start,token.span.end);}
 const position=(value,signature,ordinal)=>{
  fields(value,['type','wrappers','lifetimes','owner']);shapeType(value.type);fields(value.owner,['kind','path','arguments']);const offset=out.paths.length;for(const part of value.owner.path)out.paths.push(symbol(part));const start=out.layers.length;
  for(let index=0;index<Math.max(value.wrappers.length,value.lifetimes.length);index++)out.layers.push(symbol(value.wrappers[index]),value.lifetimes[index]===null?0:value.lifetimes[index]===undefined?-1:symbol(value.lifetimes[index]));
  out.positions.push(identity(value,5),signature,ordinal,symbol(value.type.kind),symbol(value.owner.kind),offset,value.owner.path.length,start,value.wrappers.length,value.lifetimes.length,value.owner.arguments.length);
 };
 for(const[key,entry]of program.declarations){
  fields(entry,['k','name','params','ret','body','span','sourceSignature','fullName','modulePath','recursive','decreasing']);checkGeneratedOwnedRustInterval(entry.span,true);shapeType(entry.ret);if(!Array.isArray(entry.params)||!Array.isArray(entry.modulePath))fail('a declaration lost an ordered structural role');
  const id=identity(entry,2),signature=identity(entry.sourceSignature,4),core=identity(entry.body,6),parameterStart=out.parameters.length;
  checkRustSourceSignature(entry.sourceSignature,entry.params.map(param=>param.type),entry.ret,program.sourceLanguage);
  for(const[index,param]of entry.params.entries()){fields(param,['name','type','guard','default']);shapeType(param.type);out.parameters.push(identity(param,3),id,index,symbol(param.name),symbol(param.type.kind),nullValue(param.guard),nullValue(param.default),identity(entry.sourceSignature.parameters[index]));}
  for(const[index,value]of entry.sourceSignature.parameters.entries())position(value,signature,index);position(entry.sourceSignature.result,signature,-1);
  fields(entry.body,entry.body.k==='var'?['k','name','type','span']:entry.body.k==='lit'?['k','value','type','span']:['k','arg','type','span']);shapeType(entry.body.type);checkGeneratedOwnedRustInterval(entry.body.span,true);
  out.cores.push(core,id,symbol(entry.body.k),symbol(entry.body.type.kind),symbol(Object.hasOwn(entry.body,'name')?entry.body.name:entry.body.value),entry.body.span.start,entry.body.span.end);
  out.declarations.push(id,symbol(entry.name),symbol(entry.fullName),entry.modulePath.length,signature,core,parameterStart,entry.params.length,symbol(entry.ret.kind),entry.recursive===false?0:entry.recursive===true?1:-1,nullValue(entry.decreasing),entry.span.start,entry.span.end,symbol(key),symbol(entry.sourceSignature.model),symbol(entry.sourceSignature.language),entry.sourceSignature.lifetimeBindings.length,symbol(entry.k),entry.sourceSignature.parameters.length,identity(entry.sourceSignature.result));
 }
 for(const item of program.items)out.items.push(identity(item));
 for(const row of proof.functions){fields(row,source.fields.function);out.functions.push(...[row.declaration,row.signature,row.resultPosition,row.operationNode,row.coreValue,row.input,row.position,row.literal].map(value=>identity(value)));}
 for(const[key,rows]of[['inputs',proof.inputAssumptions],['allocations',proof.valueAssumptions]])for(const row of rows){fields(row,source.fields.assumption);out[key].push(symbol(row.id),symbol(row.meaning),identity(row.declaration),identity(row.operationNode),identity(row.coreValue),identity(row.input),identity(row.position));}
 const root=walkOperation(proof.operations);out.context.push(program.schemaVersion,symbol(program.sourceLanguage),identity(program.main,8),program.main.effects.length,program.items.length,program.declarations.size,identity(proof.worker),symbol(proof.model),symbol(proof.language),symbol(proof.observationModel),root,proof.span.start,proof.span.end,program.main.span.start,program.main.span.end);
 for(const value of strings){const start=out.units.length;if(value===null){out.names.push(start,0);continue;}for(let at=0;at<value.length;at++)out.units.push(value.charCodeAt(at));out.names.push(start,value.length);out.units.push(0);}
 return out;
}
export function evaluateRustTextCertificate(table){checkGeneratedOwnedRustData(table,null);fields(table,['context','kinds','tokens','declarations','parameters','positions','paths','layers','cores','functions','operations','parts','inputs','allocations','names','units','items']);return checkRustTextCertificate(...['context','kinds','tokens','declarations','parameters','positions','paths','layers','cores','functions','operations','parts','inputs','allocations','names','units','items'].map(name=>table[name]));}
export function checkPortableRustTextCertificate(program,proof){const table=projectRustTextCertificate(program,proof),decision=evaluateRustTextCertificate(table);if(decision[0]!==1)fail('the complete generated certificate kernel refused at '+decision.join(','));return{table,decision};}
export function createPortableRustTextCertificate(source,options){const value=createRustTextValueProjection(source,options);return{...value,...checkPortableRustTextCertificate(value.program,value.proof)};}
export function encodePortableRustTextCertificate(program,proof){checkPortableRustTextCertificate(program,proof);return encodeGeneratedOwnedRust(program,proof);}
export function decodePortableRustTextCertificate(network,root){const value=decodeGeneratedOwnedRust(network,root);return{...value,...checkPortableRustTextCertificate(value.program,value.proof)};}
