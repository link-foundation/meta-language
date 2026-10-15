// Private explicit whole-source checking. Generic public APIs alone do not
// validate this experimental source interpretation, which lives in provenance.
import {isDeepStrictEqual} from 'node:util';
import {readRuntimeProjectData} from '../../js/src/project-data-loader.js';
import {checkOwnedProgramContainers} from '../../js/src/translation/owned-program-containers.js';
import {acceptRustOwnedLocalContainer,acceptRustOwnedLocalTextInput,renderRustOwnedLocalTextInput} from '../../js/src/translation/rust-owned-local-state-rules.js';
import {acceptSourceSpanOffset} from '../../js/src/source-span-rules.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {checkProgramDeclarationOwners} from '../../js/src/typed-program-declaration-owners.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {rustString} from '../../js/src/translation/rust-string-literals.js';
import {DecoratorSet} from '../../js/src/decorators.js';
import {tokenize} from '../../js/src/translation/lexer.js';
import {acceptGeneratedOwnedRustModel,acceptGeneratedOwnedRustCodeUnit} from './rules.mjs';
import {parseGeneratedOwnedRust,readGeneratedOwnedRust,readGeneratedOwnedRustCookedLiteral,tokenName} from './parser.mjs';
import {acceptGeneratedOwnedRustValueEditModel,acceptGeneratedOwnedRustLiteralEditOwner,acceptGeneratedOwnedRustSignOwner} from './value-edit-rules.mjs';
import {encodeGeneratedOwnedRust,decodeGeneratedOwnedRust} from './codec.mjs';
import {readGeneratedOwnedRustTrivia} from './trivia.mjs';
export const GENERATED_OWNED_RUST_POLICY=readRuntimeProjectData(new URL('./data/policy.lino',import.meta.url));
export const GENERATED_OWNED_RUST_VALUE_EDIT_POLICY=readRuntimeProjectData(new URL('./data/value-edit-policy.lino',import.meta.url));
const policy=GENERATED_OWNED_RUST_POLICY,own=(value,name)=>Object.hasOwn(value,name);
const fail=reason=>{throw new TypeError('Generated whole owned Rust proof: '+reason);};
function sourcePolicy(model){
 if(acceptGeneratedOwnedRustModel(model))return policy;
 if(acceptGeneratedOwnedRustValueEditModel(model)){const edits=GENERATED_OWNED_RUST_VALUE_EDIT_POLICY;return{...policy,model:edits.model,literalValueEdits:edits,fields:{...policy.fields,proof:[...policy.fields.proof,edits.proofField]}};}
 return null;
}
function fields(value,names,exact=true){if(!value||typeof value!=='object'||Array.isArray(value)||value instanceof Map||Object.keys(value).some(name=>!names.includes(name))||exact&&Object.keys(value).length!==names.length)fail('unrecognized or missing owned fields');}
function interval(span,empty=false){fields(span,['start','end']);if(typeof span.start!=='number'||typeof span.end!=='number'||!acceptSourceSpanOffset(span.start,Object.is(span.start,-0))||!acceptSourceSpanOffset(span.end,Object.is(span.end,-0))||(empty?span.end<span.start:span.end<=span.start))fail('the actual source interval is malformed');}
function data(program,proof){
 const root={sourceGeneratedOwnedRust:proof,program};checkOwnedProgramContainers(root,{acceptContainer:acceptRustOwnedLocalContainer,fail,selectedFields:['sourceGeneratedOwnedRust']});
 const active=new WeakSet(),seen=new WeakSet();let visits=0;
 const visit=(value,depth=0)=>{if(value===null||typeof value!=='object'){if(['undefined','function','symbol','bigint'].includes(typeof value)||typeof value==='number'&&!Number.isFinite(value))fail('the whole source graph contains a non-data scalar');return;}if(active.has(value))fail('the whole source graph contains a structural cycle');if(seen.has(value))return;if(depth>256||++visits>100_000)fail('the whole source graph exceeds its data bound');seen.add(value);active.add(value);for(const child of value instanceof Map?[...value].flat():Object.values(value))visit(child,depth+1);active.delete(value);};visit(root);
}
function compare(left,right,identities){if(Object.is(left,right))return true;if(identities.has(right))return false;if(!left||!right||typeof left!=='object'||typeof right!=='object'||Array.isArray(left)!==Array.isArray(right))return Object.is(left,right);const a=Object.keys(left),b=Object.keys(right);return a.length===b.length&&b.every(name=>own(left,name)&&compare(left[name],right[name],identities));}
function operationNodes(root,label){const result=[];let visits=0;const visit=node=>{if(++visits>100_000)fail('the operation graph exceeds its traversal bound');if(node?.kind==='source operation'){fields(node,policy.fields.operation);if(node.op===label)result.push(node);if(!Array.isArray(node.parts))fail('an operation lost its ordered parts');node.parts.forEach(visit);}};visit(root);return result;}
function surfaceMap(surface,program){
 const result=new Map();
 function walk(source,node){result.set(source,node);if(source.k==='toString'){walk(source.arg,node);return;}if(source.k==='app'){source.args.forEach((value,index)=>walk(value,node.args[index]));return;}if(source.k==='tuple'){source.items.forEach((value,index)=>walk(value,node.items[index]));return;}
  for(const field of ['value','body','cond','then','else','left','right','object'])if(source[field]){if(!node[field])fail('the actual checked expression lost a source role');walk(source[field],node[field]);}}
 for(const item of surface.items){const entry=program.declarations.get(item.name);if(!entry)fail('the actual source helper lost its checked declaration');result.set(item,entry);walk(item.body,entry.body);}return result;
}
function checkCst(proof,selected){
 if(!Array.isArray(proof.tokens)||!Array.isArray(proof.gaps)||proof.tokens.length!==proof.gaps.length||!proof.tokens.length||proof.tokens.length>100_000||new Set(proof.tokens).size!==proof.tokens.length)fail('the whole source requires every distinct token and trivia interval');
 let text='',end=0;
 for(const[index,token]of proof.tokens.entries()){
  fields(token,policy.fields.token,false);interval(token.span,token.kind==='eof');const gap=proof.gaps[index];fields(gap,policy.fields.gap);interval(gap.span,true);
  const roles=['declaration','binding','literal'].filter(role=>own(token,role));if(roles.length>1||roles.length===0&&!own(token,'value')||roles.length&&own(token,'value')||roles.some(role=>role!=='literal')&&token.kind!=='identifier')fail('a token has contradictory owned value roles');
  fields(token,['kind','raw','span',roles[0]??'value',...(token.kind==='number'?['suffix','floating']:[])]);
  if(gap.span.start!==end||gap.span.end!==token.span.start||!Array.isArray(gap.units)||gap.units.length!==gap.span.end-gap.span.start||typeof token.raw!=='string'||token.raw.length!==token.span.end-token.span.start)fail('the source CST has omitted or invented code units');
  for(const unit of gap.units)if(typeof unit!=='number'||!acceptGeneratedOwnedRustCodeUnit(unit,Object.is(unit,-0)))fail('the trivia contains an invalid code unit');
  readGeneratedOwnedRustTrivia(gap.units,gap.span.start,policy);
  text+=gap.units.map(unit=>String.fromCharCode(unit)).join('')+token.raw;end=token.span.end;
 }
 if(proof.span.start!==0||end!==proof.span.end||text.length!==proof.span.end)fail('the complete CST does not cover its actual source');
 if(!acceptRustOwnedLocalTextInput(text))fail('the actual Rust source contains unpaired UTF16 values');
 const lexical=tokenize(text,'Rust').tokens;if(lexical.length!==proof.tokens.length)fail('the trivia or literal spelling changes active source tokens');
 for(const[index,original]of lexical.entries()){
  const token=proof.tokens[index];if(original.kind!==token.kind||original.start!==token.span.start||original.end!==token.span.end||text.slice(original.start,original.end)!==token.raw||token.kind==='number'&&(original.suffix!==token.suffix||Boolean(original.floating)!==token.floating)||!['declaration','binding','literal'].some(role=>own(token,role))&&original.value!==token.value)fail('the stored source token differs from the actual whole-source lexer');
  if(token.kind==='string'){if(!token.raw.startsWith('"'))fail('only actual generated cooked strings have an interpretation');readGeneratedOwnedRustCookedLiteral(token.raw,selected);}
 }
 return new Set(proof.tokens);
}
function literalEdits(parsed,mapped,operations=parsed.operations){return parsed.literals.filter(row=>row.node&&acceptGeneratedOwnedRustLiteralEditOwner(mapped.get(row.surface).type.kind,row.node.op)).map(row=>{
 const node=operationNodes(operations,row.node.op).find(node=>node.parts.length===row.node.parts.length&&node.parts.every((part,index)=>part===row.node.parts[index]));
 if(!node)fail('an editable literal lost its actual source operation');
 return{operationNode:node,literal:mapped.get(row.surface),signToken:row.sourceSign};
});}
function operationAssumptions(proof){return[...proof.assumptions,...proof.valueAssumptions].map(row=>({id:row.id,statement:row.meaning,details:['actual complete source operation '+row.operationNode.op+' at '+row.operationNode.span.start+'..'+row.operationNode.span.end]}));}
export function checkGeneratedOwnedRust(program,proof){
 data(program,proof);const selected=sourcePolicy(proof.model);if(!selected)fail('the source interpretation or actual worker identity is inconsistent');fields(proof,selected.fields.proof);interval(proof.span,true);
 if(proof.language!==selected.language||proof.observationModel!==selected.observationModel||proof.worker!==program.main)fail('the source interpretation or actual worker identity is inconsistent');
 checkProgramDeclarationOwners(program);if(program.sourceLanguage!=='Rust'||program.main?.effects?.length||program.imports?.length||program.externals?.length||['referenceSemantics','utf16Strings','outputThreaded','abortsThreaded'].some(field=>own(program,field)&&program[field]!==false))fail('the actual closed source context acquired effects, imports or an independent string/runtime model');
 const identities=checkCst(proof,selected),catalog=new Map([...program.declarations].map(([name,entry])=>[name,entry]));
 const parsed=readGeneratedOwnedRust(proof.tokens,selected,{program,catalog});
 if(!compare(proof.operations,parsed.operations,identities))fail('the actual operation graph differs from the complete source tokens');
 const expected=checkProgram(parsed.surface);if(!isDeepStrictEqual(program,expected)||!isDeepStrictEqual([...program.declarations.keys()],[...expected.declarations.keys()]))fail('the actual core differs from its entire checked source interpretation or canonical declaration order');
 const mapped=surfaceMap(parsed.surface,program);
 for(const row of parsed.helperRows){if(row.name.declaration!==mapped.get(row.surface))fail('a source header lost its actual declaration identity');for(const[index,param]of row.parameters.entries())if(param.row.token.binding!==row.name.declaration.params[index])fail('a source parameter lost its actual checked binder');}
 for(const row of parsed.bindings)if(row.surface?.k==='let'&&row.token.binding!==mapped.get(row.surface))fail('a source local lost its actual checked binder');
 for(const row of parsed.literals)if(row.token.literal!==mapped.get(row.surface))fail('a source literal lost its actual checked expression');
 if(selected.literalValueEdits){
  const expected=literalEdits(parsed,mapped,proof.operations);
  if(!Array.isArray(proof.literalEdits)||proof.literalEdits.length!==expected.length)fail('every editable String and i32 literal requires its actual operation and sign owners');
  for(const[index,row]of proof.literalEdits.entries()){
   fields(row,selected.literalValueEdits.fields);const actual=expected[index],minus=row.signToken!==null&&identities.has(row.signToken)&&row.signToken.kind==='punct'&&row.signToken.raw==='-'&&row.operationNode?.parts[0]===row.signToken;
   if(row.operationNode!==actual.operationNode||row.literal!==actual.literal||row.signToken!==actual.signToken||!acceptGeneratedOwnedRustLiteralEditOwner(row.literal.type.kind,row.operationNode.op)||!acceptGeneratedOwnedRustSignOwner(actual.signToken!==null,minus))fail('a literal edit lost its actual checked literal, source operation or unary-minus token');
  }
 }
 if(!Array.isArray(proof.assumptions)||proof.assumptions.length!==policy.runtimeAssumptions.length)fail('the actual runtime lost an explicit successful-operation assumption');
 for(const[index,assumption]of proof.assumptions.entries()){
  fields(assumption,policy.fields.assumption);const expected=policy.runtimeAssumptions[index],nodes=operationNodes(proof.operations,expected.operation);
  if(nodes.length!==1||assumption.id!==expected.id||assumption.meaning!==expected.meaning||assumption.operationNode!==nodes[0])fail('a runtime assumption lost its actual owned operation');
 }
 const valueNodes=policy.valueAssumption.operations.flatMap(label=>operationNodes(proof.operations,label));
 if(!Array.isArray(proof.valueAssumptions)||proof.valueAssumptions.length!==valueNodes.length)fail('the owned String operations lost their separate value-success conditions');
 for(const[index,assumption]of proof.valueAssumptions.entries()){fields(assumption,policy.fields.assumption);if(assumption.id!==policy.valueAssumption.id||assumption.meaning!==policy.valueAssumption.meaning||assumption.operationNode!==valueNodes[index])fail('an owned String success condition lost its actual operation');}
 const cloneNodes=operationNodes(proof.operations,'owned standard String clone');
 if(!Array.isArray(proof.clones)||proof.clones.length!==parsed.clones.length)fail('every actual standard String clone needs its owned proof');
 for(const[index,clone]of proof.clones.entries()){
  fields(clone,policy.fields.clone);if(clone.operationNode!==cloneNodes[index]||clone.receiver!==mapped.get(parsed.clones[index].receiver)||clone.receiver.type.kind!=='string'||!isDeepStrictEqual(clone.owner,policy.clone.owner)||clone.method!==policy.clone.method)fail('the clone owner or its actual typed receiver differs');
 }
 return parsed;
}
export function createGeneratedOwnedRust(source,{model=policy.model}={}){
 const selected=sourcePolicy(model);if(!selected)fail('select the explicit experimental whole owned Rust model');
 const parsed=parseGeneratedOwnedRust(source,selected),program=checkProgram(parsed.surface),proof=parsed.proof,mapped=surfaceMap(parsed.surface,program);
 const bind=(token,role,value)=>{delete token.value;token[role]=value;};
 for(const row of parsed.helperRows){bind(row.name,'declaration',mapped.get(row.surface));for(const[index,param]of row.parameters.entries())bind(param.row.token,'binding',mapped.get(row.surface).params[index]);}
 for(const row of parsed.bindings)if(row.surface?.k==='let')bind(row.token,'binding',mapped.get(row.surface));
 for(const row of parsed.uses)if(row.token)bind(row.token,'binding',row.entry.token.binding);
 for(const row of parsed.calls)bind(row.token,'declaration',program.declarations.get(tokenName(row.token)));
 for(const row of parsed.literals)bind(row.token,'literal',mapped.get(row.surface));
 for(const row of parsed.runtimeUses)bind(row.token,'binding',row.target);
 const bound=readGeneratedOwnedRust(proof.tokens,selected,{program,catalog:program.declarations}),boundMap=surfaceMap(bound.surface,program);
 proof.operations=bound.operations;proof.worker=program.main;
 proof.assumptions=policy.runtimeAssumptions.map(row=>({id:row.id,meaning:row.meaning,operationNode:bound.single(row.operation)}));
 proof.valueAssumptions=policy.valueAssumption.operations.flatMap(label=>operationNodes(proof.operations,label)).map(operationNode=>({id:policy.valueAssumption.id,meaning:policy.valueAssumption.meaning,operationNode}));
 proof.clones=bound.clones.map(row=>({operationNode:row.node,receiver:boundMap.get(row.receiver),owner:[...policy.clone.owner],method:policy.clone.method}));
 if(selected.literalValueEdits)proof.literalEdits=literalEdits(bound,boundMap);
 checkGeneratedOwnedRust(program,proof);return{program,proof};
}
export function createGeneratedOwnedRustLinks(source,options){const {program,proof}=createGeneratedOwnedRust(source,options);return encodeGeneratedOwnedRust(program,proof);}
export function decodeGeneratedOwnedRustLinks(network,root){const {program,proof}=decodeGeneratedOwnedRust(network,root);checkGeneratedOwnedRust(program,proof);return{program,proof};}
export function encodeGeneratedOwnedRustLinks(program,proof){checkGeneratedOwnedRust(program,proof);return encodeGeneratedOwnedRust(program,proof);}
function tokenText(token,index,proof,selected,signs){
 if(token.declaration)return token.declaration.name;if(token.binding)return token.binding.name??token.binding.value;
 if(signs?.has(token)){const literal=signs.get(token).literal,original='-'+proof.tokens[index+1].raw.slice(0,-3);if(BigInt(original).toString()!==literal.value&&!literal.value.startsWith('-'))return selected.literalValueEdits.integer.removedSignReplacement;}
 if(token.literal){
  if(token.literal.type.kind==='string')return readGeneratedOwnedRustCookedLiteral(token.raw,selected)===token.literal.value?token.raw:selected.literalValueEdits?rustString(token.literal.value):JSON.stringify(token.literal.value);
  if(token.literal.type.kind==='bool')return (token.raw==='true')===token.literal.value?token.raw:String(token.literal.value);
  const negative=index>0&&proof.tokens[index-1].raw==='-',original=(negative?'-':'')+token.raw.slice(0,-3);
  if(BigInt(original).toString()===token.literal.value)return token.raw;
  const value=negative?token.literal.value.replace(/^-/u,''):token.literal.value;return value+'i32';
 }
 return token.raw;
}
export function emitGeneratedOwnedRust(program,proof,target){
 checkGeneratedOwnedRust(program,proof);
 const selected=sourcePolicy(proof.model),assumptions=operationAssumptions(proof);
 if(target==='Rust'){
  const signs=selected.literalValueEdits?new Map(proof.literalEdits.filter(row=>row.signToken!==null).map(row=>[row.signToken,row])):null;
  const text=proof.tokens.map((token,index)=>proof.gaps[index].units.map(unit=>String.fromCharCode(unit)).join('')+tokenText(token,index,proof,selected,signs)).join('');
  if(selected.literalValueEdits){const rebound=createGeneratedOwnedRust(text,{model:proof.model});return{language:target,text,assumptions:operationAssumptions(rebound.proof)};}
  return{language:target,text,assumptions};
 }
 if(target!=='JavaScript')fail('only Rust source replay and JavaScript value execution are covered by the experiment');
 const emitted=emitJavaScript(program),guards=[renderRustOwnedLocalTextInput()];
 for(const[index,entry]of [...program.declarations.values()].entries()){
  const name=emitted.mappings.find(row=>row.kind==='function'&&row.source===entry.fullName)?.target;if(!name)fail('the actual target declaration has no emitter mapping');
  const saved='ml_generatedOwnedOriginal'+index,args='ml_generatedOwnedArgs'+index,lines=[`if(${args}.length!==${entry.params.length})throw new RangeError("generated owned argument arity");`];
  for(const[position,param]of entry.params.entries()){const arg=`${args}[${position}]`,condition=param.type.kind==='string'?`!ml_ownedTextInput(${arg})`:param.type.kind==='bool'?`typeof ${arg}!=="boolean"`:`typeof ${arg}!=="bigint"||${arg} < -2147483648n||${arg} > 2147483647n`;lines.push(`if(${condition})throw new RangeError("generated owned argument domain");`);}
  guards.push(`const ${saved}=${name};${name}=(...${args})=>{${lines.join('')}return ${saved}(...${args});};`);
 }
 const boundary='\nmain();\n';if(!emitted.text.endsWith(boundary)||emitted.text.split(boundary).length!==2)fail('the complete target lost its unique actual final entry invocation');
 const actions=[{op:'replace',field:'source',from:boundary,to:boundary+'\n'+guards.join('\n')+'\n'}];
 const decorators=new DecoratorSet([{id:'generated-owned-rust-target-input-guards',level:'emitter',order:0,when:[['format','JavaScript']],actions}]);
 // Reuse the shared decorator engine; the actual action graph is returned as
 // links data so the finite target transformation remains reviewable.
 const decorated=decorators.decorate('emitter',{format:'JavaScript',source:emitted.text});
 return{...emitted,text:decorated.source,assumptions:[...emitted.assumptions,...assumptions,policy.inputAssumption],decorators:decorators.toLino()};
}
export function emitGeneratedOwnedRustLinks(network,root,target){const {program,proof}=decodeGeneratedOwnedRustLinks(network,root);return emitGeneratedOwnedRust(program,proof,target);}
export function emitCanonicalGeneratedOwnedRust(program,proof){
 checkGeneratedOwnedRust(program,proof);const emitted=emitRust(program);
 // The canonical emitter creates new complete source operations and intervals.
 // Interpret that actual output before binding any conditions to its operations.
 const canonical=createGeneratedOwnedRust(emitted.text,{model:proof.model}),rebound=emitGeneratedOwnedRust(canonical.program,canonical.proof,'Rust');
 if(rebound.text!==emitted.text)fail('the canonical whole source lost its exact interpreted output');
 return{...emitted,assumptions:[...emitted.assumptions,...rebound.assumptions]};
}
