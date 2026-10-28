// Deferred rule controls. The complete existing executor trace/original-source
// suites independently exercise the resulting rewritten native definitions.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readProjectData} from '../scripts/project-data.mjs';
import {planBooleanConstructorMatchSharing} from '../src/translation/boolean-constructor-match-rules.js';
import {assertVersions} from '../../experiments/javascript-properties/controls.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {shareSourceBooleanConstructorMatches} from '../scripts/source-boolean-match-sharing.mjs';
import {runInNewContext} from 'node:vm';
assertVersions();
const fixture=readProjectData(new URL('../../parity/fixtures/boolean-constructor-match.lino',import.meta.url));
for(const row of fixture.rows)test(row.name,()=>assert.deepEqual(planBooleanConstructorMatchSharing(row.owner,row.declared,row.owners,row.constructors,row.literals),row.expected));
for(const row of fixture.programs)test(row.name,()=>{
 assert.equal(runInNewContext(row.source+'\ncompute();'),row.expected);
 const original=checkProgram(parseJavaScript(row.source,{referenceSemantics:true})),shared=shareSourceBooleanConstructorMatches(original);
 assert.ok(shared.proof.proofs.length>0,'the actual source must exercise a complete-owner match rewrite');
 const packet=encodeTypedProgramLinks(shared.program);
 for(const program of [original,shared.program,...[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network=>decodeTypedProgramLinks(network,packet.root))]){
  const javascript=emitJavaScript(program),entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(runInNewContext(javascript.text+'\n'+entry+'();'),row.expected,'all original scrutinee heap effects occur once through both codecs');
 }
});
