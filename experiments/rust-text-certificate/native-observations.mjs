// CI-only oracle for the entire unchanged automatically translated Rust artifact.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {LinkNetwork,encodeLinkNetworkBinary,decodeLinkNetworkBinary,readStructuredDataLino} from '../../js/src/index.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {tokenize} from '../../js/src/translation/lexer.js';
import {createSourceNativeObservationRunner} from '../source-native-observations.mjs';
import {bundledTextCertificateSource} from './generate.mjs';
import {checkRustTextCertificate} from './rules.mjs';
import {RUST_TEXT_CERTIFICATE_POLICY as policy,createPortableRustTextCertificate,encodePortableRustTextCertificate,decodePortableRustTextCertificate} from './bridge.mjs';

export const RUST_TEXT_CERTIFICATE_MODEL_COMMIT='f4ac8664ece42a0299beae645c7fc22f14c4d793';
const modelDigests={
 'bridge.mjs':'c3121d3206ec0c6fafa1ac4506acc73012e245509b76c14bb77d696d11808e4d',
 'rules.mjs':'30f2e26a2f2b8d9b30de14a9ffed93a72c2b1e1edd6a95e717dafa36226e1f19',
 'generate.mjs':'1fe1bad0f741c614e442671cd670102bd4bcce36fa004199b3d74caf55e58e05',
 'data/policy.lino':'ee66558495d3eea1c022b1ca68f4e0a3ebba9e7ebb501001f8dff5a432ce8e14',
 'data/translation.lino':'fa557964c1f56949c0d4dc2b15e03c6f121a5e979f835e32f1c88d3d7c98adfa',
};
const checkedPlans=new WeakSet();
const sha256=value=>createHash('sha256').update(value).digest('hex');
function textBytes(value){if(typeof value!=='string'||!value.isWellFormed())throw new TypeError('oracle artifact must be primitive Unicode scalar text');return Buffer.from(value,'utf8');}
function freeze(value){if(value&&typeof value==='object'){Object.values(value).forEach(freeze);Object.freeze(value);}return value;}
function numberBits(value){
 assert.equal(typeof value,'number','only actual Number cells cross the Vec<f64> boundary');const bytes=Buffer.alloc(8);bytes.writeDoubleBE(value);return bytes.toString('hex');
}
function bitsNumber(bits){assert.match(bits,/^[0-9a-f]{16}$/u);return Buffer.from(bits,'hex').readDoubleBE();}
export function verifyRustTextCertificateModel(read=readFileSync){
 const records=Object.entries(modelDigests).map(([file,expected])=>{
  const actual=sha256(read(new URL(file,import.meta.url)));assert.equal(actual,expected,'the frozen portable certificate changed: '+file);return{file,sha256:actual};
 });
 for(const row of policy.protectedSources){
  const actual=sha256(read(new URL(row.path,import.meta.url)));assert.equal(actual,row.sha256,'the original text model/source/fixture changed: '+row.path);records.push({file:row.path,sha256:actual});
 }
 return records;
}
export function rustTextCertificateNativeMode(environment=process.env){
 if(environment.RUST_TEXT_CERTIFICATE_NATIVE!=='1')return false;
 if(environment.CI!=='true')throw new TypeError('RUST_TEXT_CERTIFICATE_NATIVE=1 requires CI=true; native compilation is forbidden locally');return true;
}
function artifactReplay(artifact){
 const original=readFileSync(new URL('./data/translation.lino',import.meta.url),'utf8');
 const network=decodeLinkNetworkBinary(encodeLinkNetworkBinary(LinkNetwork.fromLino(original)));
 const restored=readStructuredDataLino(network.toLino());assert.deepEqual(restored,artifact,'the complete translation data archive must survive text and binary Links');return restored;
}
function nativeHeader(source,order){
 const tokens=tokenize(source,'Rust').tokens;let depth=0;const occurrences=[];
 for(let index=0;index<tokens.length;index++){
  if(depth===0&&tokens[index].value==='pub'&&tokens[index+1]?.value==='fn'&&tokens[index+2]?.value==='check_rust_text_certificate')occurrences.push(index);
  if(tokens[index].kind==='punct'&&tokens[index].value==='{')depth++;
  if(tokens[index].kind==='punct'&&tokens[index].value==='}')depth--;
 }
 assert.equal(depth,0);assert.equal(occurrences.length,1,'the actual external callee must be one crate-root public function');
 const expected=['pub','fn','check_rust_text_certificate','('];
 order.forEach((name,index)=>{if(index)expected.push(',');expected.push(name,':','Vec','<','f64','>');});expected.push(')','->','Vec','<','f64','>','{');
 assert.deepEqual(tokens.slice(occurrences[0],occurrences[0]+expected.length).map(token=>token.value),expected,'the complete actual Rust artifact must expose the same ordered 17-vector ABI');
 return{functionName:'check_rust_text_certificate',parameterModes:order.map(()=> 'Vec<f64>'),resultMode:'Vec<f64>',tokenCount:tokens.length};
}
function kernelObservation(artifact){
 const program=checkProgram(parseJavaScript(bundledTextCertificateSource()));
 assert.deepEqual(program.items.map(item=>item.fullName),artifact.declarations,'all actual generated module declarations must remain in order');assert.equal(artifact.declarations.length,92);assert.equal(artifact.definitions.length,92);
 for(const definition of artifact.definitions)assert.ok(artifact.completeRust.includes(definition),'the complete artifact must contain every actual generated definition');
 const entry=program.declarations.get('checkRustTextCertificate');assert.ok(entry&&entry.k==='fn');
 const vector={kind:'array',element:{kind:'float'}};
 for(const parameter of entry.params){assert.deepEqual(parameter.type,vector);assert.equal(parameter.guard,null);assert.equal(parameter.default,null);}assert.deepEqual(entry.ret,vector);
 const order=entry.params.map(parameter=>parameter.name);assert.equal(order.length,17);assert.equal(new Set(order).size,17);
 const emitted=emitJavaScript(program),context={};vm.runInNewContext(emitted.text,context,{timeout:10000});
 const target=emitted.mappings.find(row=>row.kind==='function'&&row.source===entry.fullName)?.target;assert.equal(typeof context[target],'function');
 return{order,javascript:emitted.text,evaluate:table=>Array.from(context[target](...order.map(name=>table[name])))};
}
function originalTransports(fixtures,order){
 assert.equal(fixtures.cases.length,13);return fixtures.cases.map(row=>{
  assert.equal(textBytes(row.source).length,row.sourceBytes,row.id);assert.equal(sha256(textBytes(row.source)),row.sourceSha256,row.id);
  const value=createPortableRustTextCertificate(row.source),packet=encodePortableRustTextCertificate(value.program,value.proof);
  const text=LinkNetwork.fromLino(packet.network.toLino()),restored=decodePortableRustTextCertificate(decodeLinkNetworkBinary(encodeLinkNetworkBinary(text)),packet.root);
  assert.deepEqual(restored.table,value.table);assert.deepEqual(Object.keys(restored.table),order,'the actual bridge table order must match the checked kernel arguments');
  const width=name=>policy.layouts[name].length;
  const independent=[1,restored.table.tokens.length/width('token'),restored.table.declarations.length/width('declaration'),restored.table.inputs.length/width('condition'),restored.table.allocations.length/width('condition')];
  assert.deepEqual(restored.decision,independent);
  return{id:row.id,kind:'original graph transport',fixtureId:row.id,table:restored.table,independent,sourceReceipt:{sourceBytes:row.sourceBytes,sourceSha256:row.sourceSha256},archiveFormats:['Links Notation text','binary links']};
 });
}
function corruptedTransports(originals){
 const base=originals.find(row=>row.id==='equal-valued-distinct-owners');assert.ok(base);
 const column=(layout,name)=>{const index=policy.layouts[layout].indexOf(name);assert.ok(index>=0);return index;};
 const width=layout=>policy.layouts[layout].length,symbol=name=>{const index=policy.symbols.indexOf(name);assert.ok(index>=0);return index+1;};
 const cases=[];
 for(const[name,value]of[['negative-zero',-0],['nan',NaN],['positive-infinity',Infinity],['negative-infinity',-Infinity],['fraction',0.5],['negative-domain',-2],['unsafe-integer',9007199254740992],['subnormal',Number.MIN_VALUE]]){
  cases.push(['numeric '+name,table=>{table.context[column('context','sourceStart')]=value;}]);
 }
 for(const role of['signature','resultPosition','operation','core','input','position'])cases.push(['other actual owner '+role,table=>{const at=column('function',role),other=table.functions[width('function')+at];assert.notEqual(table.functions[at],other);table.functions[at]=other;}]);
 cases.push(
  ['input condition omitted',table=>{table.inputs.splice(0,width('condition'));}],
  ['allocation condition duplicated',table=>{table.allocations.push(...table.allocations.slice(0,width('condition')));}],
  ['input condition wrong meaning',table=>{table.inputs[column('condition','meaning')]=symbol('Rust');}],
  ['allocation condition other operation',table=>{table.allocations[column('condition','operation')]=table.allocations[width('condition')+column('condition','operation')];}],
  ['unused identity',table=>{table.kinds.push(policy.identityKinds.parameter);}],
  ['unused text unit',table=>{table.units.push(0);}],
  ['unused owner path',table=>{table.paths.push(symbol('Rust'));}],
  ['unused pointer layer',table=>{table.layers.push(0,0);}],
  ['unused operation part',table=>{table.parts.push(table.parts[0]);}],
  ['unused symbol storage',table=>{table.names.push(0,0);}],
  ['ordered items reversed',table=>{table.items.reverse();}],
  ['declaration map key other owner',table=>{table.declarations[column('declaration','mapKey')]=table.declarations[width('declaration')+column('declaration','name')];}],
  ['original token omitted',table=>{table.tokens.splice(0,width('token'));}],
 );
 assert.equal(cases.length,27,'the bounded corruption inventory is explicit');
 return cases.map(([id,mutate])=>{const table=structuredClone(base.table);mutate(table);return{id,kind:'corrupted graph transport',fixtureId:base.fixtureId,table,sourceReceipt:base.sourceReceipt,archiveFormats:base.archiveFormats};});
}
function consumerSource(observations,order){
 const blocks=observations.map((row,index)=>{
  const arguments_=order.map((_,slot)=>'vec!['+row.inputBits[slot].map(bits=>'f64::from_bits(0x'+bits+'u64)').join(',')+']');
  return`{\nlet observed:Vec<f64>=oracle_source::check_rust_text_certificate(${arguments_.join(',')});\nassert_eq!(observed.len(),5,"actual certificate result shape");\nwrite!(&mut line,"${index}").expect("format observation index");\nfor value in observed{write!(&mut line,":{:016x}",value.to_bits()).expect("format actual binary64 bits");}\nline.push('\\n');\n}`;
 });
 return`extern crate oracle_source;\nuse std::fmt::Write as _;\nfn main(){\nlet mut line=String::new();\n${blocks.join('\n')}\nprint!("{line}");\n}\n`;
}
/** Only JavaScript runs here: whole-kernel interpretation and genuine original graph archives. */
export function planRustTextCertificateNative(){
 const modelSources=verifyRustTextCertificateModel(),artifact=readProjectData(new URL('./data/translation.lino',import.meta.url)),replayed=artifactReplay(artifact);
 assert.equal(artifact.model,policy.model);assert.equal(textBytes(artifact.completeRust).length,273314,'the complete generated Rust artifact is unchanged');
 const kernel=kernelObservation(artifact),header=nativeHeader(artifact.completeRust,kernel.order),fixtures=readProjectData(new URL('../rust-text-value-projection/data/fixtures.lino',import.meta.url));
 const originals=originalTransports(fixtures,kernel.order),transports=[...originals,...corruptedTransports(originals)];assert.equal(transports.length,40);
 const observations=transports.map(row=>{
  const arrays=kernel.order.map(name=>row.table[name]),before=arrays.map(values=>values.map(numberBits));
  const expected=checkRustTextCertificate(...arrays),observed=kernel.evaluate(row.table);assert.equal(observed.length,policy.output.length);assert.equal(observed.length,5);
  assert.deepEqual(observed.map(numberBits),expected.map(numberBits),'the whole emitted JavaScript kernel must match the actual pure kernel cell for cell');
  assert.deepEqual(arrays.map(values=>values.map(numberBits)),before,'kernel calls must retain every actual input vector cell');
  if(row.independent)assert.deepEqual(observed,row.independent);else assert.notEqual(observed[0],1,'every declared corruption must be refused');
  return{id:row.id,kind:row.kind,fixtureId:row.fixtureId,sourceReceipt:row.sourceReceipt,archiveFormats:row.archiveFormats,inputBits:before,resultBits:observed.map(numberBits),decision:observed};
 });
 const source=artifact.completeRust,consumer=consumerSource(observations,kernel.order),sourceSha256=sha256(textBytes(source));
 const entry={id:'complete portable text certificate artifact',kind:'complete unmodified generated artifact',fixtureId:'data/translation.lino',source,sourceBytes:textBytes(source).length,sourceSha256,
  replaySha256:sha256(textBytes(replayed.completeRust)),originalReceipt:{sourceBytes:textBytes(source).length,sourceSha256},
  functionName:header.functionName,parameterModes:header.parameterModes,resultMode:header.resultMode,tokenCount:header.tokenCount,
  tableOrder:kernel.order,outputRoles:policy.output,declarations:artifact.declarations,invocationCount:40,originalGraphCount:13,corruptionCount:27,
  consumerSource:consumer,consumerSha256:sha256(consumer),javaScriptSha256:sha256(kernel.javascript),observations};
 const plan=freeze({model:policy.model,modelCommit:RUST_TEXT_CERTIFICATE_MODEL_COMMIT,modelSources,originalCount:1,entries:[entry]});checkedPlans.add(plan);return plan;
}
function parseObservations(output,expected){
 const lines=output.split('\n');assert.equal(lines.pop(),'','the native bits stream must end with a newline');assert.equal(lines.length,40,'all forty actual invocations must be reported once');assert.equal(lines.length,expected.length);
 return lines.map((line,index)=>{
  const parts=line.split(':');assert.equal(parts.length,6,'each native call reports exactly five binary64 cells');assert.equal(parts[0],String(index),'the native stream retains actual invocation order');
  const actual=parts.slice(1);actual.forEach(bits=>assert.match(bits,/^[0-9a-f]{16}$/u));assert.deepEqual(actual,expected[index].resultBits,'actual whole Rust artifact differs from the actual whole JavaScript kernel');
  return{id:expected[index].id,kind:expected[index].kind,fixtureId:expected[index].fixtureId,sourceReceipt:expected[index].sourceReceipt,archiveFormats:expected[index].archiveFormats,
   inputTables:expected[index].inputBits.map(bits=>({cells:bits.length,sha256:sha256(bits.join(''))})),resultBits:actual,decision:actual.map(bitsNumber)};
 });
}
export const runRustTextCertificateNative=createSourceNativeObservationRunner({
 nativeMode:rustTextCertificateNativeMode,checkedPlan:plan=>checkedPlans.has(plan),textBytes,parseObservations,
 entryReceipt:entry=>({functionName:entry.functionName,parameterModes:entry.parameterModes,resultMode:entry.resultMode,tokenCount:entry.tokenCount,
  tableOrder:entry.tableOrder,outputRoles:entry.outputRoles,declarations:entry.declarations,invocationCount:entry.invocationCount,originalGraphCount:entry.originalGraphCount,corruptionCount:entry.corruptionCount}),
 temporaryPrefix:'rust-text-certificate-original-',failureLabel:'Original complete Rust text certificate observations failed',
});
