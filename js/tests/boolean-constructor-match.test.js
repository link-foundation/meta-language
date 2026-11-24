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
import {shareSourceLexicalAliases} from '../scripts/source-lexical-aliases.mjs';
import {runInNewContext} from 'node:vm';
assertVersions();
const fixture=readProjectData(new URL('../../parity/fixtures/boolean-constructor-match.lino',import.meta.url));
for(const row of fixture.rows)test(row.name,()=>assert.deepEqual(planBooleanConstructorMatchSharing(row.owner,row.declared,row.owners,row.constructors,row.literals),row.expected));
for(const row of fixture.generalRows)test(row.name,()=>assert.deepEqual(planClosedConstructorMatchSharing(row.owner,row.declared,row.owners,row.constructors,row.classes),row.expected));
for(const row of fixture.bindingRows)test(row.name,()=>assert.deepEqual(planUnusedConstructorBindings(row.bindings,row.references),row.expected));
for(const row of fixture.aliasRows)test(row.name,()=>assert.equal(testUnusedConstructorAlias(row.nameOfBinding,row.value,row.available,row.references),row.expected));
for(const row of fixture.projectionRows)test(row.name,()=>assert.deepEqual(planSingleConstructorProjections(row.constructors,row.counts),row.expected));
for(const [profile,share]of [['Boolean',shareSourceBooleanConstructorMatches],['closed',shareSourceClosedConstructorMatches]])for(const row of [...fixture.programs,...(profile==='closed'?fixture.textPrograms:[])])test(profile+' '+row.name,()=>{
 const expected=row.expectedUnits?row.expectedUnits.map(unit=>String.fromCharCode(unit)).join(''):row.expected;
 assert.equal(runInNewContext(row.source+'\ncompute();'),expected);
 const original=checkProgram(parseJavaScript(row.source,{referenceSemantics:true})),shared=share(original);
 assert.ok(shared.proof.proofs.length>0,'the actual source must exercise a complete-owner match rewrite');
 const packet=encodeTypedProgramLinks(shared.program);
 for(const program of [original,shared.program,...[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network=>decodeTypedProgramLinks(network,packet.root))]){
  const javascript=emitJavaScript(program),entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(runInNewContext(javascript.text+'\n'+entry+'();'),expected,'all original scrutinee heap effects occur once through both codecs');
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

test('lexical alias substitution preserves unchanged checked branches',()=>{
 const source=`/** @param {boolean} flag @param {number} input @returns {number} */
 function compute(flag, input) {
  const alias = input;
  if (flag) return alias + 1;
  return input - 1;
 }`;
 const program=checkProgram(parseJavaScript(source));
 const entry=program.declarations.get('compute');
 const proofs=[];
 const rewritten=shareSourceLexicalAliases(program,entry,entry.body,proofs);
 assert.equal(proofs.length,1);
 assert.equal(rewritten.then.left.name,'input');
 assert.equal(rewritten.cond,entry.body.body.cond);
 assert.equal(rewritten.else,entry.body.body.else);
});

test('immutable nominal projections retain reference fields across mutations', () => {
 const source = `
 /** @typedef {{ current: number }} Counter */
 /** @typedef {{ $: 'wrapped', counter: Counter }} Wrapper */
 /** @param {Counter} counter @returns {number} */
 function increment(counter) { counter.current += 1; return counter.current; }
 /** @param {Wrapper} wrapper @returns {number} */
 function inspect(wrapper) {
  const before = wrapper.counter.current;
  const after = increment(wrapper.counter);
  return before * 100 + after * 10 + wrapper.counter.current;
 }
 /** @returns {number} */
 function compute() { return inspect({ $: 'wrapped', counter: { current: 3 } }); }
 `;
 const original = checkProgram(parseJavaScript(source, { referenceSemantics: true }));
 const shared = shareSourceClosedConstructorMatches(original, { shareImmutableNominalReferences: true });
 assert.ok(shared.proof.proofs.some(proof =>
  proof.owner === 'inspect' && proof.operation === 'share immutable single-constructor parameter projections'));
 const packet = encodeTypedProgramLinks(shared.program);
 for (const program of [original, shared.program, decodeTypedProgramLinks(
  decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)), packet.root)]) {
  const emitted = emitJavaScript(program);
  const entry = emitted.mappings.find(mapping => mapping.kind === 'function' && mapping.source === 'compute').target;
  assert.equal(runInNewContext(emitted.text + '\n' + entry + '();'), 344);
 }
 assert.equal(runInNewContext(source + '\ncompute();'), 344);
});

test('identical unbound nominal arms retain exactly one state change per selection', () => {
 const source = `
 /** @typedef {{ $: 'first' } | { $: 'second' } | { $: 'third' }} Choice */
 /** @typedef {{ value: number }} Counter */
 /** @param {Counter} counter @param {number} increment @returns {number} */
 function advance(counter, increment) { counter.value += increment; return counter.value; }
 /** @param {Counter} counter @param {Choice} choice @returns {number} */
 function inspect(counter, choice) {
  switch (choice.$) {
   case 'first': return advance(counter, 1);
   case 'second':
   case 'third': return advance(counter, 2);
  }
 }
 /** @returns {number} */
 function compute() {
  const counter = { value: 0 };
  const first = inspect(counter, { $: 'first' });
  const second = inspect(counter, { $: 'second' });
  const third = inspect(counter, { $: 'third' });
  return first * 100 + second * 10 + third;
 }
 `;
 const original = checkProgram(parseJavaScript(source, { referenceSemantics: true }));
 const shared = shareSourceClosedConstructorMatches(original, { shareImmutableNominalReferences: true });
 const proof = shared.proof.proofs.find(proof =>
  proof.owner === 'inspect' && proof.fallback?.kind === 'identical unbound body');
 assert.deepEqual(proof?.retainedIndices, [0]);
 const { network, root } = encodeTypedProgramLinks(shared.program);
 const text = LinkNetwork.fromLino(network.toLino());
 const binary = decodeLinkNetworkBinary(encodeLinkNetworkBinary(network));
 for (const program of [original, shared.program, decodeTypedProgramLinks(text, root), decodeTypedProgramLinks(binary, root)]) {
  const emitted = emitJavaScript(program);
  const entry = emitted.mappings.find(mapping => mapping.kind === 'function' && mapping.source === 'compute').target;
  assert.equal(runInNewContext(emitted.text + '\n' + entry + '();'), 135);
 }
 assert.equal(runInNewContext(source + '\ncompute();'), 135);
});

test('a repeated default call retains argument order and scrutinee effects',()=>{
 const source=`
 /** @typedef {{ $: 'first' } | { $: 'second' } | { $: 'third' }} Value */
 /** @param {{value:number}} holder @param {Value} value @returns {Value} */
 function observe(holder,value){holder.value+=1;return value;}
 /** @param {number} first @param {number} second @returns {number} */
 function combine(first,second){return first*10+second;}
 /** @param {{value:number}} holder @param {Value} value @param {number} input @returns {number} */
 function inspect(holder,value,input){
  const observed=observe(holder,value);
  switch(observed.$){
   case 'first': return combine(5,input);
   default: return combine(input,5);
  }
 }
 /** @returns {number} */ function compute(){
  const holder={value:0};
  const first=inspect(holder,{$:'first'},7);
  const second=inspect(holder,{$:'second'},7);
  const third=inspect(holder,{$:'third'},7);
  return first+second+third+holder.value;
 }`;
 const expected=210;
 assert.equal(runInNewContext(source+'\ncompute();'),expected);
 const original=checkProgram(parseJavaScript(source,{referenceSemantics:true}));
 const ordinary=shareSourceClosedConstructorMatches(original);
 const shared=shareSourceClosedConstructorMatches(original,{shareDirectCalls:true});
 assert.equal(ordinary.proof.proofs.some(proof=>proof.fallback?.kind==='direct call'),false);
 const proof=shared.proof.proofs.find(proof=>proof.owner==='inspect'&&proof.fallback?.kind==='direct call');
 assert.deepEqual(proof.retainedIndices,[0]);
 assert.equal(proof.fallback.target,'combine');
 assert.deepEqual(proof.fallback.args.map(argument=>argument.name??argument.value),['input','5']);
 const packet=encodeTypedProgramLinks(shared.program);
 for(const program of [original,shared.program,...[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network=>decodeTypedProgramLinks(network,packet.root))]){
  const javascript=emitJavaScript(program),entry=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='compute').target;
  assert.equal(runInNewContext(javascript.text+'\n'+entry+'();'),expected);
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
