// Deferred rule controls. The complete existing executor trace/original-source
// suites independently exercise the resulting rewritten native definitions.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readProjectData} from '../scripts/project-data.mjs';
import {planBooleanConstructorMatchSharing,planClosedConstructorMatchSharing,planUnusedConstructorBindings,testUnusedConstructorAlias,planSingleConstructorProjections} from '../src/translation/boolean-constructor-match-rules.js';
import {assertVersions} from '../../experiments/javascript-properties/controls.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {shareSourceBooleanConstructorMatches,shareSourceClosedConstructorMatches} from '../scripts/source-boolean-match-sharing.mjs';
import {runInNewContext} from 'node:vm';
assertVersions();
const fixture=readProjectData(new URL('../../parity/fixtures/boolean-constructor-match.lino',import.meta.url));
for(const row of fixture.rows)test(row.name,()=>assert.deepEqual(planBooleanConstructorMatchSharing(row.owner,row.declared,row.owners,row.constructors,row.literals),row.expected));
for(const row of fixture.generalRows)test(row.name,()=>assert.deepEqual(planClosedConstructorMatchSharing(row.owner,row.declared,row.owners,row.constructors,row.classes),row.expected));
for(const row of fixture.bindingRows)test(row.name,()=>assert.deepEqual(planUnusedConstructorBindings(row.bindings,row.references),row.expected));
for(const row of fixture.aliasRows)test(row.name,()=>assert.equal(testUnusedConstructorAlias(row.nameOfBinding,row.value,row.available,row.references),row.expected));
for(const row of fixture.projectionRows)test(row.name,()=>assert.deepEqual(planSingleConstructorProjections(row.constructors,row.counts),row.expected));
for(const [profile,share]of [['Boolean',shareSourceBooleanConstructorMatches],['closed',shareSourceClosedConstructorMatches]])for(const row of [...fixture.programs,...(profile==='closed'?fixture.textPrograms:[])])test(profile+' '+row.name,()=>{
 assert.equal(runInNewContext(row.source+'\ncompute();'),row.expected);
 const original=checkProgram(parseJavaScript(row.source,{referenceSemantics:true})),shared=share(original);
 assert.ok(shared.proof.proofs.length>0,'the actual source must exercise a complete-owner match rewrite');
 const packet=encodeTypedProgramLinks(shared.program);
 for(const program of [original,shared.program,...[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network=>decodeTypedProgramLinks(network,packet.root))]){
  const javascript=emitJavaScript(program),entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(runInNewContext(javascript.text+'\n'+entry+'();'),row.expected,'all original scrutinee heap effects occur once through both codecs');
 }
});
for(const row of fixture.projectionPrograms)test(row.name,()=>{
 assert.equal(runInNewContext(row.source+'\ncompute();'),row.expected);
 const original=checkProgram(parseJavaScript(row.source,{referenceSemantics:row.referenceSemantics})),shared=shareSourceClosedConstructorMatches(original);
 assert.equal(shared.proof.proofs.some(proof=>proof.operation==='share immutable single-constructor parameter projections'),row.shared,'reference/deferred scopes retain their declared boundaries');
 const packet=encodeTypedProgramLinks(shared.program);
 for(const program of [original,shared.program,...[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network=>decodeTypedProgramLinks(network,packet.root))]){
  const javascript=emitJavaScript(program),entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(runInNewContext(javascript.text+'\n'+entry+'();'),row.expected);
 }
});

for(const row of fixture.lexicalAliasPrograms)test(row.name,()=>{
 assert.equal(runInNewContext(row.source+'\ncompute();'),row.expected);
 const original=checkProgram(parseJavaScript(row.source,{referenceSemantics:row.referenceSemantics})),shared=shareSourceClosedConstructorMatches(original);
 assert.equal(shared.proof.proofs.some(proof=>proof.operation==='substitute immutable lexical alias'),row.shared);
 const packet=encodeTypedProgramLinks(shared.program);
 for(const program of [original,shared.program,...[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network=>decodeTypedProgramLinks(network,packet.root))]){
  const javascript=emitJavaScript(program),entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(runInNewContext(javascript.text+'\n'+entry+'();'),row.expected);
 }
});

for(const row of fixture.scopedProjectionPrograms)test(row.name,()=>{
 assert.equal(runInNewContext(row.source+'\ncompute();'),row.expected);
 const original=checkProgram(parseJavaScript(row.source,{referenceSemantics:row.referenceSemantics})),shared=shareSourceClosedConstructorMatches(original);
 assert.equal(shared.proof.proofs.some(proof=>proof.operation==='share immutable single-constructor scoped projections'),row.shared);
 const packet=encodeTypedProgramLinks(shared.program);
 for(const program of [original,shared.program,...[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network=>decodeTypedProgramLinks(network,packet.root))]){
  const javascript=emitJavaScript(program),entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(runInNewContext(javascript.text+'\n'+entry+'();'),row.expected);
 }
});

for(const row of fixture.literalConditionPrograms)test(row.name,()=>{
 assert.equal(runInNewContext(row.source+'\ncompute();'),row.expected);
 const original=checkProgram(parseJavaScript(row.source,{referenceSemantics:row.referenceSemantics})),shared=shareSourceClosedConstructorMatches(original);
 assert.equal(shared.proof.proofs.some(proof=>proof.operation==='select checked literal condition'),row.selected);
 const packet=encodeTypedProgramLinks(shared.program);
 for(const program of [original,shared.program,...[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network=>decodeTypedProgramLinks(network,packet.root))]){
  const javascript=emitJavaScript(program),entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(runInNewContext(javascript.text+'\n'+entry+'();'),row.expected);
 }
});

for(const [rows,operation,expectedField]of [[fixture.booleanIdentityPrograms,'reduce checked Boolean identity','reduced'],[fixture.literalAliasPrograms,'substitute checked primitive literal','propagated']])for(const row of rows)test(row.name,()=>{
 assert.equal(runInNewContext(row.source+'\ncompute();'),row.expected);
 const original=checkProgram(parseJavaScript(row.source,{referenceSemantics:row.referenceSemantics})),shared=shareSourceClosedConstructorMatches(original);
 assert.equal(shared.proof.proofs.some(proof=>proof.operation===operation),row[expectedField]);
 const packet=encodeTypedProgramLinks(shared.program);
 for(const program of [original,shared.program,...[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network=>decodeTypedProgramLinks(network,packet.root))]){
  const javascript=emitJavaScript(program),entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(runInNewContext(javascript.text+'\n'+entry+'();'),row.expected);
 }
});
