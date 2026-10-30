// Future independent primitive and codec controls; intentionally unexecuted.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-primitive-bitwise.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const type of fixture.types)test(type.name+': actual bit operations and four-target checked codecs',()=>{
  const rows=fixture.functions.filter(row=>row.type===type.name), program=checkProgram(parseRust(rows.map(row=>row.source).join('\n')+'\nfn main(){}\n'));
  const js=emitJavaScript(program), packet=encodeTypedProgramLinks(program);
  for(const target of ['JavaScript','Rust','Lean','Rocq']){
    const expected=emitTypedProgramLinks(packet.network,packet.root,target);
    for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
      assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);
    }
  }
  for(const row of rows){
    const name=js.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.name).target;
    const fn=runInNewContext(js.text.replace(/^export /gmu,'')+'\n'+name+';');
    for(const left of type.values)for(const right of row.symbol==='!'?[type.values[0]]:type.values){
      const a=type.name==='bool'?left==='true':BigInt(left), b=type.name==='bool'?right==='true':BigInt(right);
      const expected=type.name==='bool'?(row.symbol==='!'?!a:row.symbol==='&'?a&&b:row.symbol==='|'?a||b:a!==b)
        :row.symbol==='!'?(type.signed?BigInt.asIntN(type.bits,~a):BigInt.asUintN(type.bits,~a)):row.symbol==='&'?a&b:row.symbol==='|'?a|b:a^b;
      assert.equal(fn(a,b),expected,row.name+'('+left+','+right+')');
    }
  }
});
test('Rust bit precedence, contextual literals and eager Boolean failure',()=>{
  const js=emitJavaScript(checkProgram(parseRust(fixture.precedenceSource+fixture.contextualSource+'\nfn main(){}\n')));
  const functions=runInNewContext(js.text.replace(/^export /gmu,'')+'\n[precedence,contextual];');
  assert.equal(functions[0](8n,3n),8n|(3n^(3n&5n)));
  assert.equal(functions[1](),0n);
  const eager=emitJavaScript(checkProgram(parseRust(fixture.eagerSource+'\nfn main(){}\n')));
  for(const name of ['eager_and','eager_or','eager_xor']){
    const target=eager.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===name).target;
    assert.throws(()=>runInNewContext(eager.text.replace(/^export /gmu,'')+'\n'+target+'();'),/eager right/u);
  }
});
