// Future controls; never execute while the complete requirements draft is open.
import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { analyzeProgram } from '../src/program-representation.js';
import { createModuleBindingProgramLinks } from '../src/module-binding-program.js';
import { emitTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary,decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { observeJavaScriptModule } from '../../experiments/module-bindings/node-source.mjs';
import { planJavaScriptModuleBindingFields,readSourceModuleBindingRangeRole,readSourceBindingRenameAffixes } from '../src/translation/module-binding-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/module-bindings.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
test('module-local renaming preserves external names original exports execution and exact CST source',()=>{
  for(const row of fixture.cases){
    const program=analyzeProgram(row.source,'JavaScript');
    assert.equal(program.emit(),row.source);assert.ok(program.network.verifyFullMatch().isClean());
    const binding=program.bindings.find(binding=>binding.scope==='scope:0'&&binding.name===row.binding);assert.ok(binding,row.name);
    const renamed=program.renameBinding(binding.id,row.replacement);
    assert.equal(renamed.source,row.expectedSource,row.name);assert.equal(renamed.emit(),row.expectedSource);
    const expected=row.observations.map(row=>row.value);
    assert.deepEqual(observeJavaScriptModule(row.source,fixture.provider,row.observations).values,expected,row.name+' original Node');
    assert.deepEqual(observeJavaScriptModule(renamed.source,fixture.provider,row.observations).values,expected,row.name+' renamed Node');
  }
});
test('the complete module-role definition survives both codecs and all four emissions',()=>{
  const packet=createModuleBindingProgramLinks();
  for(const target of ['JavaScript','Rust','Lean','Rocq']){
    const expected=emitTypedProgramLinks(packet.network,packet.root,target);
    for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])
      assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);
  }
});
test('source-role errors and alias fragments keep exact independent boundaries',()=>{
  assert.deepEqual(readSourceBindingRenameAffixes(1,false,'add'),['add as ','']);
  assert.deepEqual(readSourceBindingRenameAffixes(2,false,'add'),['',' as add']);
  assert.deepEqual(readSourceBindingRenameAffixes(0,true,'field'),['field: ','']);
  assert.throws(()=>readSourceBindingRenameAffixes(3,false,'external'),/not a local binding/u);
  assert.deepEqual(planJavaScriptModuleBindingFields(['import_specifier'],[],[],[],[],[],[]),[-1,1,0]);
  assert.equal(readSourceModuleBindingRangeRole([2,5,1,9,12,3],2,5),1);
  assert.equal(readSourceModuleBindingRangeRole([2,5,1,9,12,3],9,12),3);
  assert.equal(readSourceModuleBindingRangeRole([2,5,1,9,12,3],6,8),0);
});
