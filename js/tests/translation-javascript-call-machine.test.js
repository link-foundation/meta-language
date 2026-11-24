// Deferred complete executor, independent source and source graph controls.
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {readBoundedCanonicalNetwork} from '../src/canonical-network-reader.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {ordinarySourceArchiveByteLimit,ordinarySourceArchiveNodeLimit} from '../src/typed-program-source-owners.js';
import {readSuppliedSourceLibraryClosure} from '../src/translation/source-library-closure.js';
import {assertVersions,machineFixture,observeCallMachine,observeInvalidCallProgram,observeInvalidCallMachine,observeSourceErrorResumptionRefusal} from '../../experiments/javascript-properties/call-machine-controls.mjs';
assertVersions();
for(const row of machineFixture.programs)test(row.name,()=>observeCallMachine(row));
for(const row of machineFixture.invalid)test(row.name,()=>observeInvalidCallProgram(row));
for(const row of machineFixture.invalidStates)test(row.name,()=>observeInvalidCallMachine(row));
for(const row of machineFixture.errorResumptionRefusals)test(row.name,()=>observeSourceErrorResumptionRefusal(row));
test('a supplied source imports the registered portable heap validator from its actual provider', () => {
 const path = 'js/src/translation/javascript-ordinary-heap.js';
 const provider = 'js/src/translation/javascript-heap-validation-portable.js';
 const source = "import {testJavaScriptValidatedHeapSnapshot} from '#ordinary-heap-validation-authority';\n"
  + 'export function validate(heap) { return testJavaScriptValidatedHeapSnapshot(heap); }';
 const modules = [{path, source}, {path: provider, source: readFileSync(new URL('../src/translation/javascript-heap-validation-portable.js', import.meta.url), 'utf8')}];
 const options = {aliases: true, reExports: 'resolved named module providers 1'};
 const closure = readSuppliedSourceLibraryClosure(path, modules, options);
 assert.equal(closure.inputs.find(input => input.path === path).removedImports[0].dependency, provider);
 assert.throws(() => readSuppliedSourceLibraryClosure(path, modules.slice(0, 1), options), /actual source module is missing/u);
 assert.throws(() => readSuppliedSourceLibraryClosure(path,
  [{path, source: source.replace('Snapshot}', 'Snapshot as renamed}')}, modules[1]], options), /registered unchanged binding/u);
 const unrelated = 'js/src/translation/unrelated.js';
 assert.throws(() => readSuppliedSourceLibraryClosure(unrelated, [{path: unrelated, source}, modules[1]], options),
  /registered package import/u);
});

test('the actual machine source graph keeps both codecs and all four targets',()=>{
 const targets = ['JavaScript', 'Rust', 'Lean', 'Rocq'];
 const binaryLimits = {recordLimit: ordinarySourceArchiveNodeLimit};
 let {network, root} = createSourceDefinitionProgramLinks('meta-language/source-program/javascript-call-machine-program');
 const expected = targets.map(target => emitTypedProgramLinks(network, root, target));
 let text = network.toLino();
 let binary = encodeLinkNetworkBinary(network, binaryLimits);
 network = null;
 function compareEmissions(reloaded, transport) {
  targets.forEach((target, index) =>
   assert.deepEqual(emitTypedProgramLinks(reloaded, root, target), expected[index], transport + ' ' + target));
 }
 compareEmissions(readBoundedCanonicalNetwork(text, {
  byteLimit: ordinarySourceArchiveByteLimit,
  nodeLimit: ordinarySourceArchiveNodeLimit,
 }), 'canonical Links');
 text = null;
 compareEmissions(decodeLinkNetworkBinary(binary, binaryLimits), 'binary Links');
 binary = null;
});
