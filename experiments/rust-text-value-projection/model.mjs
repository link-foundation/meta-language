// Explicit private interpretation. Ordinary public Links emission does not
// enforce this separately selected provenance certificate.
import {isDeepStrictEqual} from 'node:util';
import {readRuntimeProjectData} from '../../js/src/project-data-loader.js';
import {checkProgramDeclarationOwners} from '../../js/src/typed-program-declaration-owners.js';
import {acceptRustOwnedLocalTextInput,renderRustOwnedLocalTextInput} from '../../js/src/translation/rust-owned-local-state-rules.js';
import {rustString} from '../../js/src/translation/rust-string-literals.js';
import {tokenize} from '../../js/src/translation/lexer.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {DecoratorSet} from '../../js/src/decorators.js';
import {GENERATED_OWNED_RUST_POLICY,checkGeneratedOwnedRustData,checkGeneratedOwnedRustFields as fields,compareGeneratedOwnedRustData as compare,checkGeneratedOwnedRustCst,checkGeneratedOwnedRustInterval} from '../generated-owned-rust/model.mjs';
import {encodeGeneratedOwnedRust,decodeGeneratedOwnedRust} from '../generated-owned-rust/codec.mjs';
import {acceptRustTextValueProjectionModel} from './rules.mjs';
import {fail,tokenValue,tokenizeTextProjection,parseTextProjectionCore,readTextProjectionSource} from './source.mjs';
export const RUST_TEXT_VALUE_PROJECTION_POLICY=readRuntimeProjectData(new URL('./data/policy.lino',import.meta.url));
const policy=RUST_TEXT_VALUE_PROJECTION_POLICY;
const own=(value,key)=>Object.hasOwn(value,key);
const cstPolicy={...GENERATED_OWNED_RUST_POLICY,literalTokenMode:policy.literalTokenMode};
function actualOperation(expected,operations){const node=operations.parts.find(node=>node?.kind==='source operation'&&node.op==='text value function'&&node.parts.at(-1)?.parts[1]?.parts.length===expected.parts.length&&node.parts.at(-1).parts[1].parts.every((part,index)=>part===expected.parts[index]))?.parts.at(-1).parts[1];if(!node)fail('the actual text operation lost its token occurrence');return node;}
function facts(parsed,operations){return parsed.functions.map(row=>({...row,operationNode:actualOperation(row.operationNode,operations)}));}
function assumption(descriptor,row){return{id:descriptor.id,meaning:descriptor.meaning,declaration:row.declaration,operationNode:row.operationNode,coreValue:row.coreValue,input:row.input,position:row.position};}
function conditions(rows){return{inputAssumptions:rows.filter(row=>row.input!==null).map(row=>assumption(policy.inputAssumption,row)),valueAssumptions:rows.filter(row=>policy.operations.find(operation=>operation.name===row.operationNode.op).allocation).map(row=>assumption(policy.valueAssumption,row))};}
function checkRows(actual,expected,names){if(!Array.isArray(actual)||actual.length!==expected.length)fail('the source operation facts or conditions have missing or duplicated rows');for(const[index,row]of actual.entries()){fields(row,names);for(const name of names)if(row[name]!==expected[index][name])fail('an actual source operation/header/core identity or condition differs at '+name);}}
export function checkRustTextValueProjection(program,proof){
 checkGeneratedOwnedRustData(program,proof);fields(proof,policy.fields.proof);checkGeneratedOwnedRustInterval(proof.span,true);
 if(!acceptRustTextValueProjectionModel(proof.model)||proof.language!==policy.language||proof.observationModel!==policy.observationModel||proof.worker!==program.main)fail('select the explicit source interpretation and actual empty main owner');
 checkProgramDeclarationOwners(program);
 if(program.sourceLanguage!=='Rust'||program.main.effects.length||program.imports?.length||program.externals?.length||['referenceSemantics','utf16Strings','outputThreaded','abortsThreaded'].some(key=>own(program,key)&&program[key]!==false))fail('this source context contains an independent effect, input or runtime model');
 const identities=checkGeneratedOwnedRustCst(proof,cstPolicy),parsed=readTextProjectionSource(proof.tokens,program,policy),expected=parseTextProjectionCore(proof.tokens,policy);
 if(!isDeepStrictEqual(program,expected)||!isDeepStrictEqual([...program.declarations.keys()],[...expected.declarations.keys()]))fail('the actual checked core differs from its complete source token interpretation');
 if(!compare(proof.operations,parsed.operations,identities))fail('the source operation graph differs from its actual complete tokens');
 const rows=facts(parsed,proof.operations);checkRows(proof.functions,rows,policy.fields.function);const required=conditions(rows);for(const key of['inputAssumptions','valueAssumptions'])checkRows(proof[key],required[key],policy.fields.assumption);
 return parsed;
}
export function createRustTextValueProjection(source,{model=policy.model}={}){
 if(!acceptRustTextValueProjectionModel(model))fail('select the explicit experimental text source model');
 const lexical=tokenizeTextProjection(source),program=parseTextProjectionCore(lexical.tokens,policy),initial=readTextProjectionSource(lexical.tokens,program,policy,{bound:false});
 for(const{token,kind,owner}of initial.roles){delete token.value;token[kind]=owner;}
 const parsed=readTextProjectionSource(lexical.tokens,program,policy),rows=facts(parsed,parsed.operations),proof={model,language:policy.language,observationModel:policy.observationModel,...lexical,operations:parsed.operations,functions:rows,worker:program.main,...conditions(rows)};
 checkRustTextValueProjection(program,proof);return{program,proof};
}
export function encodeRustTextValueProjection(program,proof){checkRustTextValueProjection(program,proof);return encodeGeneratedOwnedRust(program,proof);}
export function createRustTextValueProjectionLinks(source,options){const value=createRustTextValueProjection(source,options);return encodeRustTextValueProjection(value.program,value.proof);}
export function decodeRustTextValueProjection(network,root){const value=decodeGeneratedOwnedRust(network,root);checkRustTextValueProjection(value.program,value.proof);return value;}
function emissionConditions(proof){return[...proof.inputAssumptions,...proof.valueAssumptions].map(row=>({id:row.id,statement:row.meaning,details:['actual declaration '+row.declaration.fullName,'actual source operation '+row.operationNode.op+' at '+row.operationNode.span.start+'..'+row.operationNode.span.end,...(row.input?['actual input '+row.input.name]:[])]}));}
function replayToken(token){if(token.declaration||token.binding)return tokenValue(token);if(token.literal){if(typeof token.literal.value!=='string'||!acceptRustOwnedLocalTextInput(token.literal.value))fail('an edited literal requires Unicode scalar text');const original=tokenize(token.raw,'Rust').tokens[0];return original.value===token.literal.value?token.raw:rustString(token.literal.value);}return token.raw;}
export function emitRustTextValueProjection(program,proof,target){
 checkRustTextValueProjection(program,proof);
 if(target==='Rust'){
  const text=proof.tokens.map((token,index)=>proof.gaps[index].units.map(unit=>String.fromCharCode(unit)).join('')+replayToken(token)).join(''),fresh=createRustTextValueProjection(text,{model:proof.model});
  return{language:target,text,assumptions:emissionConditions(fresh.proof)};
 }
 if(target!=='JavaScript')fail('only complete Rust source replay and JavaScript text value execution are selected');
 const emitted=emitJavaScript(program),guards=[renderRustOwnedLocalTextInput()];
 for(const[index,entry]of [...program.declarations.values()].entries()){
  const name=emitted.mappings.find(row=>row.kind==='function'&&row.source===entry.fullName)?.target;if(!name)fail('the actual target function mapping is absent');
  const saved='ml_textProjectionOriginal'+index,args='ml_textProjectionArguments'+index;
  guards.push(`const ${saved}=${name};${name}=(...${args})=>{if(${args}.length!==${entry.params.length}||${args}.some(value=>!ml_ownedTextInput(value)))throw new RangeError("text projection input domain");return ${saved}(...${args});};`);
 }
 const boundary='\nmain();\n';if(!emitted.text.endsWith(boundary)||emitted.text.split(boundary).length!==2)fail('the complete target has no unique actual final entry invocation');
 const decorators=new DecoratorSet([{id:'rust-text-value-projection-input-guards',level:'emitter',order:0,when:[['format','JavaScript']],actions:[{op:'replace',field:'source',from:boundary,to:boundary+'\n'+guards.join('\n')+'\n'}]}]);
 return{...emitted,text:decorators.decorate('emitter',{format:'JavaScript',source:emitted.text}).source,assumptions:[...emitted.assumptions,...emissionConditions(proof),GENERATED_OWNED_RUST_POLICY.inputAssumption],decorators:decorators.toLino()};
}
export function emitRustTextValueProjectionLinks(network,root,target){const{program,proof}=decodeRustTextValueProjection(network,root);return emitRustTextValueProjection(program,proof,target);}
// The actual whole generic artifact is exposed for explicit boundary tests;
// its runtime wrapper is not stripped or granted this leaf-source proof.
export function emitGenericRustTextValueProjection(program,proof){checkRustTextValueProjection(program,proof);return emitRust(program);}
