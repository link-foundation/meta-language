import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateOriginalNativeNumberObservations} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-original-number-observations.mjs';
import {readSourceTextRustBlock} from '../scripts/source-text-byte-library-style.mjs';
import {DecoratorSet} from '../src/decorators.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';

const root=new URL('../../',import.meta.url);
const receiptPath='parity/self-translation/original-native-number-observations.lino';
const receipt=readProjectData(new URL(receiptPath,root));
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
const format=source=>execFileSync('rustfmt',['--edition','2024','--config','skip_children=true','--emit','stdout'],{input:source,encoding:'utf8',maxBuffer:2*1024*1024,timeout:15000});

function checkOriginalNumberSourceConservation(value) {
  for(const input of Object.values(value.inputs)){
    assert.equal(readFileSync(new URL(input.file,root),'utf8'),input.sourceText);
    assert.deepEqual(pin(input.sourceText),{bytes:input.bytes,sha256:input.sha256});
    for(const row of input.pieces??input.helpers)assert.equal(input.sourceText.slice(row.start,row.end),row.sourceText);
  }
  assert.deepEqual(pin(readFileSync(new URL(value.author.file,root))),{bytes:value.author.bytes,sha256:value.author.sha256});
  assert.equal(value.sourceInventory.declarations.length,9);assert.equal(value.sourceInventory.pins.length,13);
  for(const row of value.outputs)assert.deepEqual(pin(readFileSync(new URL(row.file,root))),{bytes:row.bytes,sha256:row.sha256});
  assert.deepEqual(pin(format(value.rawModel)),{bytes:value.outputs[0].bytes,sha256:value.outputs[0].sha256});
  assert.deepEqual(pin(format(value.relocation.source)),{bytes:value.outputs[1].bytes,sha256:value.outputs[1].sha256});
  const styles=DecoratorSet.fromLino(readFileSync(new URL('parity/self-translation/original-native-number-observation-decorators.lino',root),'utf8'));
  assert.equal(styles.decorate('emitter',{format:'Rust',component:'visitors',source:value.originalVisitors}).source,value.relocation.source);
  const counter=styles.decorate('emitter',{format:'Rust',component:'counters',source:value.inputs.sink.originalCounter}).source;
  assert.ok(value.rawModel.includes(counter));
  assert.equal(value.inputs.sink.pieces.map(row=>row.sourceText).join('\n\n'),value.inputs.sink.originalCounter);
}

test('original native number source generation retains every complete declaration counter and numeric validator',()=>{
  const generated=generateOriginalNativeNumberObservations({check:true});
  assert.deepEqual(generated.receipt,receipt);checkOriginalNumberSourceConservation(receipt);
  assert.deepEqual(receipt.inputs.parent.helpers.map(row=>row.name),['fail','write_number']);
  assert.ok(receipt.inputs.parent.exactNumberHelper.includes('if represented != i128::from(integer)'));
  assert.ok(receipt.inputs.parent.numberArm.includes('StructuredDataValue::Number(value)'));
  assert.equal(receipt.inputs.sink.pieces.length,5);
  assert.equal(receipt.sourceInventory.declarations.filter(row=>row.name==='SyntaxTree')[0].variants.length,5);
  assert.ok(generated.outputs.every(row=>row.source.split('\n').length<1000));
});

test('original JSON routing is distinct from host integer conversion and accessors precede unchanged validation',()=>{
  const original=receipt.originalVisitors,derived=receipt.relocation.source;
  assert.equal(original.split('serde_json::Value::Number(value) => sink.json_number(value, depth),').length,2);
  assert.equal(derived.split('serde_json::Value::Number(value) => sink.native_json_number(value, depth),').length,2);
  assert.equal(receipt.jsonVisitor.after.replace('sink.native_json_number(value, depth)','sink.json_number(value, depth)'),receipt.jsonVisitor.before);
  assert.equal(original.replace(receipt.trait.before,receipt.trait.after).replace(receipt.jsonVisitor.before,receipt.jsonVisitor.after),
    derived.replaceAll('meta_language::','crate::'));
  for(const name of ['usize','i64','json_number']){
    const originalMethod=readSourceTextRustBlock(receipt.inputs.sink.originalCounter,'fn',name);
    const observedMethod=readSourceTextRustBlock(receipt.implementation.after,'fn',name);
    assert.equal(observedMethod,originalMethod,'every complete original primitive and validation method remains byte exact');
  }
  const method=receipt.observationMethod,validation=method.indexOf('self.json_number(value, depth)?;');
  for(const accessor of ['value.is_i64()','value.is_u64()','value.is_f64()','value.as_i64()','value.as_u64()','value.as_f64()'])
    assert.ok(method.indexOf(accessor)>=0&&method.indexOf(accessor)<validation);
  assert.ok(method.indexOf('self.observations.push(')>validation);
  assert.ok(!method.includes(' as f64')&&!method.includes('serde_json::to_value'));
});

test('the complete allocation free original preflight and every source token survive observation extension',()=>{
  assert.ok(receipt.rawModel.includes(receipt.inputs.sink.preflight));
  assert.ok(receipt.rawModel.indexOf(receipt.inputs.sink.preflight)<receipt.rawModel.indexOf('let mut observed = ObservationSink'));
  assert.ok(receipt.inputs.sink.preflight.includes('count.occurrence(0, 1)?;'));
  for(const record of [receipt.storage,receipt.inherent,receipt.implementation,receipt.relocation]){
    let source=record.source;
    for(const edit of record.edits){assert.equal(source.slice(edit.start,edit.start+edit.to.length),edit.to);source=source.slice(0,edit.start)+edit.from+source.slice(edit.start+edit.to.length);}
    assert.ok([receipt.inputs.sink.originalCounter,receipt.originalVisitors.replace(receipt.trait.before,receipt.trait.after).replace(receipt.jsonVisitor.before,receipt.jsonVisitor.after)]
      .some(context=>context.includes(source)),'every complete original owner and context survives exact token inverse');
  }
  assert.deepEqual(receipt.limits,{nodeLimit:100000,byteLimit:16777216,depthLimit:256});
});

test('original number control fixtures retain independent variants exact payloads and expanded wire identities',()=>{
  const fixture=readProjectData(new URL('parity/fixtures/original-native-number-observations.lino',root));
  assert.deepEqual(fixture.observations.map(row=>row.wireOccurrence),[23,24,25,26,27,28]);
  assert.equal(fixture.expandedOccurrences,38);
  assert.deepEqual(fixture.observations.map(row=>row.floatingBits),[
    '0000000000000000','bff0000000000000','43b0000000000000','8000000000000000','3ff8000000000000','0000000000000001']);
  assert.deepEqual(fixture.observations.map(row=>row.signedDecimal),['0','-1','1152921504606846976',null,null,null]);
  assert.deepEqual(fixture.observations.map(row=>row.unsignedDecimal),['0',null,'1152921504606846976',null,null,null]);
  assert.deepEqual(fixture.observations.map(row=>[row.signed,row.unsigned,row.floating]),[
    [true,true,false],[true,false,false],[true,true,false],[false,false,true],[false,false,true],[false,false,true]]);
  assert.deepEqual(pin(readFileSync(new URL('rust/Cargo.lock',root))),fixture.numberVersions.lockSource);
  assert.match(fixture.boundary,/not actual native execution/u);
});

test('complete original number source contexts reload from both Links archives and forged source facts refuse',()=>{
  const text=readFileSync(new URL(receiptPath,root),'utf8'),network=LinkNetwork.fromLino(text);
  for(const current of [network,decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))]){
    const roots=current.links().filter(link=>link.metadata().term==='structured data');
    assert.equal(roots.length,1);
    const restored=decodeStructuredDataLinks(current,roots[0].id());
    assert.deepEqual(restored,receipt);checkOriginalNumberSourceConservation(restored);
  }
  for(const change of [value=>{value.inputs.sink.pieces[0].sourceText+='\n';},value=>{value.author.sha256='0'.repeat(64);},
    value=>{value.outputs[0].sha256='0'.repeat(64);},value=>{value.sourceInventory.declarations.pop();},
    value=>{value.rawModel=value.rawModel.replace('occurrences.checked_add(1)','occurrences.checked_add(2)');}]){
    const altered=structuredClone(receipt);change(altered);assert.throws(()=>checkOriginalNumberSourceConservation(altered));
  }
});
