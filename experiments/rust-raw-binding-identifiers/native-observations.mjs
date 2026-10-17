// CI-only source-language oracle; importing or planning never starts a process.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/index.js';
import {fixedBounds} from '../../js/src/translation/types.js';
import {createSourceNativeObservationRunner} from '../source-native-observations.mjs';
import {
 RUST_RAW_BINDING_IDENTIFIER_POLICY as policy,createRustRawBindingIdentifierLinks,
 decodeRustRawBindingIdentifierLinks,encodeRustRawBindingIdentifierLinks,
 checkRustRawBindingIdentifierInvocation,emitRustRawBindingIdentifiers,
} from './model.mjs';
import {validName,readRawBindingIdentifierSyntax} from './syntax.mjs';

export const RUST_RAW_BINDING_IDENTIFIER_MODEL_COMMIT='612faac96d60d01f0e31d19a4869add446d5da85';
const modelDigests={
 'model.mjs':'86c80174c000183a4c807a077f307452ee355ecf7b326f953a5e17c7c071278e',
 'syntax.mjs':'158f2ac2a50ef9e9113391a340f59a1fc17c5a5cab889a35aefa045be9668784',
 'rules.mjs':'9e54e5bd9e1cf0a3366f55ef31cb4c4f86c6b3aabc0ef9939b07a3f1926932d3',
 'data/policy.lino':'1c6ed55cde0a20886d25ba6761aa54885f659726f1ec6ca6094aedc33aa0c33d',
 'data/fixtures.lino':'575ad1992ab8359c95e2e15d8f043890942a03b6de4c738c71433d137705b25a',
};
const checkedPlans=new WeakSet();
const sha256=value=>createHash('sha256').update(value).digest('hex');
function textBytes(value){
 if(typeof value!=='string'||!value.isWellFormed())throw new TypeError('oracle source must be primitive Unicode scalar text');
 return Buffer.from(value,'utf8');
}
function freeze(value){if(value&&typeof value==='object'){Object.values(value).forEach(freeze);Object.freeze(value);}return value;}
export function verifyRustRawBindingIdentifierModel(read=readFileSync){
 return Object.entries(modelDigests).map(([file,expected])=>{
  const actual=sha256(read(new URL(file,import.meta.url)));assert.equal(actual,expected,'the independently reviewed model changed: '+file);
  return{file,sha256:actual};
 });
}
export function rustRawBindingIdentifierNativeMode(environment=process.env){
 if(environment.RUST_RAW_BINDING_IDENTIFIER_NATIVE!=='1')return false;
 if(environment.CI!=='true')throw new TypeError('RUST_RAW_BINDING_IDENTIFIER_NATIVE=1 requires CI=true; native compilation is forbidden locally');
 return true;
}
function archiveSource(source){
 const packet=createRustRawBindingIdentifierLinks(source),text=LinkNetwork.fromLino(packet.network.toLino());
 return decodeRustRawBindingIdentifierLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(text)),packet.root);
}
function archiveEdit({program,proof}){
 const packet=encodeRustRawBindingIdentifierLinks(program,proof),text=LinkNetwork.fromLino(packet.network.toLino());
 return decodeRustRawBindingIdentifierLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(text)),packet.root);
}
function primitive(type){
 if(type.kind==='bool'){assert.deepEqual(type,{kind:'bool'});return'bool';}
 assert.deepEqual(type,{kind:'fixed',bits:32,signed:true},'the consumer requires the actual direct signed i32 type');
 return'i32';
}
function argumentValue(type,value){
 if(primitive(type)==='bool'){assert.equal(typeof value,'boolean');return value;}
 assert.equal(typeof value,'string','fixture i32 inputs use canonical decimal strings');assert.match(value,/^(?:0|-?[1-9][0-9]*)$/u);
 const integer=BigInt(value),bounds=fixedBounds(type);assert.ok(integer>=bounds.min&&integer<=bounds.max,'fixture i32 must satisfy its actual source bounds');return integer;
}
function canonicalValue(type,value){
 if(primitive(type)==='bool'){assert.equal(typeof value,'boolean');return value?'true':'false';}
 assert.equal(typeof value,'bigint');const bounds=fixedBounds(type);assert.ok(value>=bounds.min&&value<=bounds.max);return value.toString();
}
function publicTarget(program,proof,name){
 const syntax=readRawBindingIdentifierSyntax(proof,policy,program);
 const invocationPremises=syntax.functions.map(row=>{
  const declaration=row.target.checkedOwner,token=proof.tokens.indexOf(row.target.group.tokens[0]);
  assert.equal(proof.tokens[token-1]?.value,'fn');assert.equal(proof.tokens[token-2]?.value,'pub','the external oracle requires the actual public header');
  const premise=proof.inputAssumptions.find(premise=>premise.declaration===declaration);
  assert.ok(premise&&premise.parameterOwners===declaration.params,'the actual invocation premise owns the ordered parameter array');
  return{declarationName:declaration.fullName,parameterModes:declaration.params.map(parameter=>primitive(parameter.type)),resultMode:primitive(declaration.ret)};
 });
 const entry=program.declarations.get(name);assert.ok(entry&&entry.k==='fn'&&entry.name===name&&entry.fullName===name&&program.items.includes(entry));
 assert.ok(validName(entry.name,true),'the actual canonical declaration must permit safe raw Rust spelling');
 return{entry,invocationPremises};
}
function consumerSource(entry,observations){
 const modes=entry.params.map(parameter=>primitive(parameter.type)),result=primitive(entry.ret);
 const blocks=observations.map(({args},index)=>{
  const arguments_=args.map((value,slot)=>{
   const actual=argumentValue(entry.params[slot].type,value),text=canonicalValue(entry.params[slot].type,actual);
   return`let argument_${slot}:${modes[slot]}=${text}${modes[slot]==='i32'?'i32':''};`;
  });
  return`{\n${arguments_.join('\n')}\nlet observed:${result}=oracle_source::r#${entry.name}(${modes.map((_,slot)=>'argument_'+slot).join(',')});\nwrite!(&mut line,"${index}:${result}:{observed}\\n").expect("format actual primitive observation");\n}`;
 });
 return`extern crate oracle_source;\nuse std::fmt::Write as _;\nfn main(){\nlet mut line=String::new();\n${blocks.join('\n')}\nprint!("{line}");\n}\n`;
}
function preparedEntry(id,kind,fixtureId,source,name,observations,originalReceipt){
 const {program,proof}=archiveSource(source),replay=emitRustRawBindingIdentifiers(program,proof,'Rust').text;
 assert.deepEqual(textBytes(replay),textBytes(source),'the complete original Rust source must survive text and binary Links byte for byte');
 const {entry,invocationPremises}=publicTarget(program,proof,name),javascript=emitRustRawBindingIdentifiers(program,proof,'JavaScript').text,context={};
 vm.runInNewContext(javascript,context,{timeout:1000});assert.equal(typeof context[name],'function','observe the actual emitted JavaScript declaration');
 const expected=observations.map(({args,value})=>{
  const actual=args.map((arg,index)=>argumentValue(entry.params[index].type,arg));
  const premise=checkRustRawBindingIdentifierInvocation(program,proof,entry,actual);
  assert.equal(premise.declaration,entry);assert.equal(premise.parameterOwners,entry.params);
  const independent=argumentValue(entry.ret,value),observed=context[name](...actual);
  assert.equal(observed,independent,'the actual complete emitted JavaScript must agree with the independent fixture value');
  return{args:[...args],value,type:primitive(entry.ret),canonical:canonicalValue(entry.ret,observed)};
 });
 const consumer=consumerSource(entry,expected),sourceBytes=textBytes(source);
 return{id,kind,fixtureId,source,sourceBytes:sourceBytes.length,sourceSha256:sha256(sourceBytes),originalReceipt,
  replaySha256:sha256(textBytes(replay)),archiveFormats:['Links Notation text','binary links'],functionName:entry.name,
  parameterModes:entry.params.map(parameter=>primitive(parameter.type)),resultMode:primitive(entry.ret),invocationPremises,
  consumerSource:consumer,consumerSha256:sha256(consumer),javaScriptSha256:sha256(javascript),observations:expected};
}
const receipt=source=>({sourceBytes:textBytes(source).length,sourceSha256:sha256(textBytes(source))});
function editedSources(fixtures){
 const row=fixtures.cases.find(row=>row.id==='two actual parameter owners');
 let {program,proof}=archiveSource(row.source);
 const target=program.items[0].params[1],use=proof.occurrences[0];
 use.token.binding=target;use.alias=proof.aliases.find(alias=>alias.checkedOwner===target);use.checkedExpression.name=target.name;
 ({program,proof}=archiveEdit({program,proof}));
 const retarget=emitRustRawBindingIdentifiers(program,proof,'Rust').text;
 const expected=[{args:row.arguments,value:row.arguments[1]}],original=receipt(row.source);
 const entries=[preparedEntry('parameter-retarget','coherent parameter retarget',row.id,retarget,row.name,expected,original)];
 program.items[0].params[1].name='loop';proof.occurrences[0].checkedExpression.name='loop';
 ({program,proof}=archiveEdit({program,proof}));
 entries.push(preparedEntry('parameter-keyword-rename','coherent parameter keyword rename',row.id,emitRustRawBindingIdentifiers(program,proof,'Rust').text,row.name,expected,original));
 const forward=fixtures.cases.find(row=>row.id==='actual forward declaration occurrence');
 ({program,proof}=archiveSource(forward.source));
 const callee=program.declarations.get('type'),call=proof.occurrences.find(row=>row.alias.checkedOwner===callee);
 assert.ok(call&&call.checkedExpression.k==='call');callee.name='loop';callee.fullName='loop';call.checkedExpression.fn=callee.fullName;
 program.declarations=new Map(program.items.map(item=>[item.fullName,item]));
 ({program,proof}=archiveEdit({program,proof}));
 entries.push(preparedEntry('callee-keyword-rename','coherent actual callee keyword rename',forward.id,emitRustRawBindingIdentifiers(program,proof,'Rust').text,forward.name,[{args:forward.arguments,value:forward.value}],receipt(forward.source)));
 return entries;
}
/** Planning performs only checked source/Links interpretation and actual JavaScript observations. */
export function planRustRawBindingIdentifierNative({fixtures=readProjectData(new URL('./data/fixtures.lino',import.meta.url)),includeEdits=true}={}){
 const modelSources=verifyRustRawBindingIdentifierModel();assert.deepEqual(fixtures,readProjectData(new URL('./data/fixtures.lino',import.meta.url)),'the oracle requires the unchanged independently frozen fixture records');
 assert.equal(fixtures.cases.length,7,'all seven complete original fixture modules are mandatory');
 const entries=fixtures.cases.map(row=>preparedEntry(row.id,'complete unmodified original',row.id,row.source,row.name,[{args:row.arguments,value:row.value}],receipt(row.source)));
 if(includeEdits)entries.push(...editedSources(fixtures));
 const plan=freeze({model:fixtures.model,modelCommit:RUST_RAW_BINDING_IDENTIFIER_MODEL_COMMIT,modelSources,originalCount:7,entries});checkedPlans.add(plan);return plan;
}
function parseObservations(output,expected){
 const lines=output.split('\n');assert.equal(lines.pop(),'','the native primitive stream must end with a newline');assert.equal(lines.length,expected.length,'every actual invocation must be reported exactly once');
 return lines.map((line,index)=>{
  const match=/^(0|[1-9][0-9]*):(bool|i32):(true|false|0|-?[1-9][0-9]*)$/u.exec(line);assert.ok(match,'the native stream must contain canonical typed primitive text');
  assert.equal(Number(match[1]),index,'native observations must retain actual invocation order');assert.equal(match[2],expected[index].type,'the actual result type cannot be erased');
  const type=match[2]==='bool'?{kind:'bool'}:{kind:'fixed',bits:32,signed:true};
  const value=match[2]==='bool'?(assert.ok(['true','false'].includes(match[3])),match[3]==='true'):argumentValue(type,match[3]);
  assert.equal(canonicalValue(type,value),expected[index].canonical,'actual original Rust differs from emitted JavaScript and the independent fixture');
  return{arguments:expected[index].args,resultType:match[2],canonical:match[3],value:match[2]==='i32'?value.toString():value};
 });
}
export const runRustRawBindingIdentifierNative=createSourceNativeObservationRunner({
 nativeMode:rustRawBindingIdentifierNativeMode,checkedPlan:plan=>checkedPlans.has(plan),textBytes,parseObservations,
 entryReceipt:entry=>({functionName:entry.functionName,parameterModes:entry.parameterModes,resultMode:entry.resultMode,invocationPremises:entry.invocationPremises}),
 temporaryPrefix:'rust-raw-binding-original-',failureLabel:'Original Rust raw binding observations failed',
});
