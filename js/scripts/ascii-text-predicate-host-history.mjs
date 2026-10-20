// Complete finite native source replay; normal verification uses no Git process.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { verifyGitArtifactCertificate } from './git-object-certificate.mjs';

export const ASCII_HOST_REVISION = '52132727d7ce158762a4499b7032a42a9f16f3c3';
export const ASCII_HOST_ADDITIONS = [
  'rust/src/translation/rust/expressions/ascii_text_predicates.rs',
  'rust/tests/unit/translation_ascii_text_predicates.rs',
];
export const ASCII_HOST_REGISTRATIONS = [
  { path: 'rust/src/translation/mod.rs', declaration: 'pub(crate) mod ascii_text_predicate_rules;' },
  { path: 'rust/tests/unit/mod.rs', declaration: 'mod translation_ascii_text_predicates;' },
];
export const ASCII_ASSERTION_DRAFT_SHA256 = '6ba1da5e18033f88972760c5a2d1e4915b4f22123dca513b980ab67fe95414e0';
export const ASCII_ASSERTION_EDITS = [
  { before: "        const reparsed = checkProgram(parseRust(native.definitions.join('\\n\\n') + '\\nfn main(){}'));\n"
      + '        assert.deepEqual(observe(emitJavaScript(reparsed).text, owner.name, inputs), expected);\n',
    after: '        assert.deepEqual(observe(emitJavaScript(reversed).text, owner.name, inputs), expected);\n'
      + '        // Definitions alone omit the generated runtime and are not a complete\n'
      + '        // standalone source interpretation. Preserve this exact input and its\n'
      + '        // explicit boundary; CI executes the complete native.text on all inputs.\n'
      + "        const definitionsOnly = native.definitions.join('\\n\\n') + '\\nfn main(){}';\n"
      + "        if (owner.name === 'is_identifier') {\n"
      + "          const closure = tokenize(definitionsOnly, 'Rust').tokens.find(token => token.value === 'move');\n"
      + '          assert.ok(closure);\n'
      + "          assert.throws(() => checkProgram(parseRust(definitionsOnly)), error => error.kind === 'unsupported'\n"
      + "            && error.details.construct === 'closure'\n"
      + "            && error.reason === 'closure: higher-order values are outside the portable core'\n"
      + '            && error.span.start === closure.start && error.span.end === closure.end);\n'
      + '        } else {\n          const reparsed = checkProgram(parseRust(definitionsOnly));\n'
      + '          assert.deepEqual(observe(emitJavaScript(reparsed).text, owner.name, inputs), expected);\n        }\n'
      + "        const completeNative = LinkNetwork.parse(native.text, 'Rust');\n"
      + '        assert.ok(completeNative.links().every(link => !link.metadata().flags.hasError && !link.metadata().flags.isMissing));\n'
      + '        assert.deepEqual(Buffer.from(completeNative.reconstructText()), Buffer.from(native.text));\n'
      + '        for (const restored of reloads(completeNative)) assert.deepEqual(Buffer.from(restored.reconstructText()), Buffer.from(native.text));\n' },
  { before: "  assert.equal(result.artifact.checkedNamedSourceInventory.length, result.artifact.inventory.length);\n});\n",
    after: "  assert.equal(result.artifact.checkedNamedSourceInventory.length, result.artifact.inventory.length);\n});\n"
      + '\n\n'
      + "test('actual JavaScript array literal spreads reuse Unicode iteration without changing internal array construction', () => {\n"
      + "  const source = '/** @param {string} value @returns {string[]} */function points(value){return [...value];}';\n"
      + "  const values = ['', 'A', 'Aπ', '😀', 'A😀Z', '\\ud800', '\\udc00', '\\ud800x\\udc00'];\n"
      + '  const expected = values.map(value => [...value]);\n'
      + "  const packet = createTypedProgramLinks(source, 'JavaScript');\n"
      + '  for (const network of reloads(packet.network)) {\n'
      + '    const program = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });\n'
      + '    const code = emitJavaScript(program).text;\n'
      + "    assert.deepEqual(observe(code, 'points', values).map(value => [...value]), expected);\n"
      + '    assert.ok(emitRust(program, { arrayConsumption: true }).definitions.length > 0);\n  }\n'
      + "  const wrong = '/** @param {number} value */function points(value){return [...value];}';\n"
      + "  assert.throws(() => createTypedProgramLinks(wrong, 'JavaScript'), /iterable|iterator|array|spread/u);\n});\n" },
];
export const ASCII_HOST_EDITS = [
  { path: 'rust/src/translation/rust/expressions.rs', changes: [
    { before: 'mod control;\nmod read_only_queries;\n', after: 'mod ascii_text_predicates;\nmod control;\nmod read_only_queries;\n' },
    { before: '                let method = self.cursor.identifier(Some("method"))?;\n',
      after: '                let method = self.cursor.identifier(Some("method"))?;\n'
        + '                if let Some(ascii) = self.read_ascii_text_predicate(&value, &method, path)? {\n'
        + '                    value = ascii;\n                    continue;\n                }\n' },
  ] },
  { path: 'rust/src/translation/javascript/expressions/values.rs', changes: [
    { before: '    /// `[a, ...xs, b]`: the elements in order, with the elements of each spread array in place.\n',
      after: '    /// `[a, ...xs, b]`: ordered values and the elements of each supported iterable.\n' },
    { before: '            items.push(SArrayItem::new(self.expr()?, spread));\n',
      after: '            let value = self.expr()?;\n            let place = value.span;\n'
        + '            let value = if spread {\n                SExpr::new(\n'
        + '                    SNode::ForOfIterable {\n                        object: Box::new(value),\n                        ty: None,\n                    },\n'
        + '                    place,\n                )\n            } else {\n                value\n            };\n'
        + '            items.push(SArrayItem::new(value, spread));\n' },
  ] },
];

export function asciiHostPin(source) {
  return { bytes: Buffer.byteLength(source), sha256: createHash('sha256').update(source).digest('hex') };
}

export function replayAsciiHostEdits(source, changes) {
  const edits = [];
  let current = source;
  for (const change of changes) {
    assert.equal(current.split(change.before).length, 2, 'one exact existing transport hook');
    const start = current.indexOf(change.before);
    edits.push({ start, removed: change.before, inserted: change.after });
    current = current.slice(0, start) + change.after + current.slice(start + change.before.length);
  }
  return { source: current, edits };
}

export function verifyAsciiTextPredicateHosts(archive) {
  assert.equal(archive.schemaVersion, 1);
  assert.equal(archive.model, 'Rust ASCII character query host replay 1');
  assert.equal(archive.revision, ASCII_HOST_REVISION);
  assert.equal(archive.coordinateUnit, 'UTF-16 code units');
  assert.deepEqual(archive.hosts.map(row => row.path), ASCII_HOST_EDITS.map(row => row.path));
  assert.deepEqual(archive.additions.map(row => row.path), ASCII_HOST_ADDITIONS);
  assert.deepEqual(archive.registrations, ASCII_HOST_REGISTRATIONS);
  verifyAsciiAssertionCorrection(archive.assertionCorrection);
  const files = [];
  for (const [index, row] of archive.hosts.entries()) {
    assert.equal(typeof row.source, 'string');
    const bytes = Buffer.from(row.source);
    assert.equal(bytes.toString('utf8'), row.source);
    verifyGitArtifactCertificate(row.certificate, bytes, { revision: ASCII_HOST_REVISION, path: row.path });
    const replay = replayAsciiHostEdits(row.source, ASCII_HOST_EDITS[index].changes);
    assert.deepEqual(row.edits, replay.edits, 'only the exact declared transport edits are admitted');
    assert.deepEqual(asciiHostPin(replay.source), row.current);
    let inverse = replay.source;
    for (const edit of row.edits.toReversed()) {
      assert.equal(inverse.slice(edit.start, edit.start + edit.inserted.length), edit.inserted);
      inverse = inverse.slice(0, edit.start) + edit.removed + inverse.slice(edit.start + edit.inserted.length);
    }
    assert.equal(inverse, row.source, 'every original token, branch, assertion and byte is independently recoverable');
    files.push({ path: row.path, source: replay.source, predecessor: row.source });
  }
  for (const row of archive.additions) {
    assert.equal(typeof row.source, 'string');
    assert.equal(Buffer.from(row.source).toString('utf8'), row.source);
    assert.deepEqual(asciiHostPin(row.source), row.pin);
    files.push({ path: row.path, source: row.source });
  }
  assert.equal(archive.sourceArtifact.path, 'parity/self-translation/ascii-text-predicate-rules-native.lino');
  assert.ok(Array.isArray(archive.sourceArtifact.libraryInterface));
  for (const source of ['acceptAsciiCharacterMethod', 'acceptRustAsciiCharacterOwner',
    'planRustAsciiTextQuery', 'planRustAsciiCharacterCallback']) {
    assert.equal(archive.sourceArtifact.libraryInterface.filter(row => row.source === source).length, 1,
      'each actual shared source decision retains its generated native signature');
  }
  return files;
}

export function verifyAsciiAssertionCorrection(correction) {
  assert.equal(correction.path, 'js/tests/translation-ascii-text-predicates.test.js');
  assert.equal(correction.draft.kind, 'unexecuted private draft');
  assert.equal(correction.draft.commit, null, 'a private draft must not acquire fabricated Git membership');
  assert.deepEqual(asciiHostPin(correction.draft.source), correction.draft.pin);
  assert.equal(correction.draft.pin.sha256, ASCII_ASSERTION_DRAFT_SHA256);
  const replay = replayAsciiHostEdits(correction.draft.source, ASCII_ASSERTION_EDITS);
  assert.deepEqual(correction.edits, replay.edits);
  assert.equal(replay.source, correction.current.source);
  assert.deepEqual(asciiHostPin(replay.source), correction.current.pin);
  let inverse = replay.source;
  for (const edit of correction.edits.toReversed()) {
    assert.equal(inverse.slice(edit.start, edit.start + edit.inserted.length), edit.inserted);
    inverse = inverse.slice(0, edit.start) + edit.removed + inverse.slice(edit.start + edit.inserted.length);
  }
  assert.equal(inverse, correction.draft.source, 'every original input, independent oracle and surrounding assertion is recoverable');
  assert.equal(correction.nativeTest, 'rust/tests/unit/translation_ascii_text_predicates.rs');
  assert.equal(correction.nativeAssertion, 'both_actual_whole_helpers_preserve_sources_cst_native_execution_and_bidirectional_semantic_archives');
  return correction;
}
