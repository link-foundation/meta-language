// PRIVATE source/inverse controls only. No Rust compiler or native runtime.
import assert from 'node:assert/strict';
import test from 'node:test';
import {buildIntegration,proposeParent} from './generate.mjs';

let result;
const current=()=>result??=buildIntegration();

test('the native parent proposal is exactly invertible and moves the actual Number arm without changing its validation',()=>{
  const packet=current();
  assert.equal(packet.parent.inverse(packet.parent.source),packet.original);
  assert.equal(packet.receipt.parent.originalSha256,packet.receipt.predecessor.originalParentPin);
  assert.deepEqual(packet.receipt.sourcePolicyChanges,[]);
  assert.ok(packet.parent.source.includes('mod grammar_outcome;\npub use grammar_outcome::encode_grammar_parse_outcome_links;'));
  assert.ok(packet.parent.helper.includes('fn exact_json_number(number: &Number) -> Result<f64, String>'));
  const statements=packet.parent.originalArm.slice('        Value::Number(number) => {\n'.length,-'\n        }'.length)
    .split('\n').map(line=>line.slice(12)).join('\n');
  const expected='fn exact_json_number(number: &Number) -> Result<f64, String> {\n'
    +statements.replace('StructuredDataValue::Number(value)','Ok(value)').split('\n').map(line=>'    '+line).join('\n')+'\n}\n\n';
  assert.equal(packet.parent.helper,expected);
  assert.ok(packet.parent.source.includes(packet.parent.callArm));
  // Whole-source conservation catches every unrelated mutation; an old pin
  // cannot be replaced by a caller's merely similar source.
  assert.throws(()=>proposeParent(packet.original+'\n'),/authenticated/);
  assert.throws(()=>packet.parent.inverse(packet.parent.source.replace('mod grammar_outcome;','mod foreign;')));
  const changed=packet.parent.source.replace('const LANGUAGE:', 'const OTHER_LANGUAGE:');
  assert.notEqual(packet.parent.inverse(changed),packet.original);
});

test('the concrete sink proposal counts before graph allocation and shares the exact borrowed exhaustive visitor',()=>{
  const {child}=current();
  for(const declaration of ['impl OutcomeWireSink for CountSink','type Node = ();','impl OutcomeWireSink for DirectSink','type Node = LinkId;',
    'fn wire_parse_outcome<S: OutcomeWireSink>','fn wire_syntax_tree<S: OutcomeWireSink>',
    'fn wire_grammar_selection<S: OutcomeWireSink>','fn wire_leaf_text<S: OutcomeWireSink>','fn wire_operation_value<S: OutcomeWireSink>'])
    assert.ok(child.includes(declaration),declaration);
  const entry=child.slice(child.indexOf('pub fn encode_grammar_parse_outcome_links('),child.indexOf('// Generated exact borrowed declaration transport.'));
  const ordered=['limits.node_limit == 0','limits.depth_limit > 256','let mut count = CountSink',
    'wire_parse_outcome(outcome, &mut count, 1)?;','count.occurrence(0, 1)?;',
    'let mut sink = DirectSink::new(limits);','wire_parse_outcome(outcome, &mut sink, 1)?;',
    'sink.encoder.node("structured data", Some("1"), &[body], 0)?;'];
  let previous=-1;
  for(const marker of ordered) {const position=entry.indexOf(marker);assert.ok(position>previous,marker);previous=position;}
  assert.ok(child.includes('checked_add(1)')&&child.includes('checked_add(bytes)'));
  assert.ok(child.includes('references.try_reserve(count)'));
  assert.ok(child.includes('self.encoder.scalar("string value", Some(value), depth)'));
  assert.ok(child.includes('write_number(exact_json_number(value)?)?'));
  assert.ok(child.includes('self.json_number(&serde_json::Number::from(integer), depth)'));
  for(const forbidden of ['.clone(', 'from_json(', 'to_value', 'StructuredDataValue', ' as f64', 'encode_utf16', '_ =>'])
    assert.ok(!child.includes(forbidden),forbidden);
  // A syntax/type/CI runtime check remains required. These assertions verify
  // source contracts, not successful native CountSink or DirectSink execution.
  assert.ok(current().receipt.pending.includes('native compilation and execution'));
});
