// Close the one real Unicode decoder import without modifying its source or
// the frozen frontend library. Selection uses original tokenizer spans.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { tokenize } from '../src/translation/lexer.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';

const sourceFile = new URL('../src/translation/grammar-selection-mapping-rules.js', import.meta.url);
const dependencyFile = new URL('../src/translation/frontend-rules.js', import.meta.url);
const boundary = "import { decodeUnicodeEscape } from './frontend-rules.js';";
const hash = source => createHash('sha256').update(source).digest('hex');

/** Capture the complete actual declaration and original JSDoc/type input.
 * No function body or dependency algorithm is transcribed into this generator.
 */
export function readGrammarSelectionMappingSourceModule() {
  const source = readFileSync(sourceFile, 'utf8'), dependency = readFileSync(dependencyFile, 'utf8');
  assert.equal(source.split(boundary).length, 2, 'the mapping module owns exactly its recorded decoder import');
  const body = source.replace(boundary, '');
  assert.doesNotMatch(body, /^\s*import\b/mu, 'an unknown source dependency must refuse');
  const { tokens, comments } = tokenize(dependency, 'JavaScript');
  const matches = [];
  let depth = 0;
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];
    if (depth === 0 && token.kind === 'identifier' && token.value === 'export'
        && tokens[index + 1]?.value === 'function' && tokens[index + 2]?.value === 'decodeUnicodeEscape') matches.push(index);
    if (token.kind === 'punct' && token.value === '{') depth += 1;
    if (token.kind === 'punct' && token.value === '}') depth -= 1;
  }
  assert.equal(matches.length, 1, 'one actual exported source declaration owns the imported decoder');
  const first = matches[0], start = tokens[first].start;
  let opening = first + 3;
  while (opening < tokens.length && !(tokens[opening].kind === 'punct' && tokens[opening].value === '{')) opening += 1;
  assert.ok(opening < tokens.length, 'the original function body is present');
  let end = opening, braces = 1;
  while (braces > 0 && ++end < tokens.length) {
    if (tokens[end].kind === 'punct' && tokens[end].value === '{') braces += 1;
    if (tokens[end].kind === 'punct' && tokens[end].value === '}') braces -= 1;
  }
  assert.equal(braces, 0, 'the original complete function body is enclosed');
  const documentation = comments.filter(comment => comment.text.startsWith('/**')
    && comment.end <= start && dependency.slice(comment.end, start).trim() === '');
  const types = comments.filter(comment => /@typedef\b/u.test(comment.text) && /\bUnicodeEscape\s*\*\/$/u.test(comment.text));
  assert.equal(documentation.length, 1, 'the actual original function annotation is retained');
  assert.equal(types.length, 1, 'the actual decoder return type has one source declaration');
  const functionRange = { start, end: tokens[end].end };
  const closure = types[0].text + '\n' + documentation[0].text + '\n'
    + dependency.slice(functionRange.start, functionRange.end) + '\n' + body;
  return { source, closure, sourceClosure: { removedImport: boundary,
    inputs: [{ path: 'js/src/translation/grammar-selection-mapping-rules.js', sourceSha256: hash(source) },
      { path: 'js/src/translation/frontend-rules.js', source: dependency, sourceSha256: hash(dependency),
        function: { name: 'decodeUnicodeEscape', ...functionRange },
        documentation: { start: documentation[0].start, end: documentation[0].end },
        type: { name: 'UnicodeEscape', start: types[0].start, end: types[0].end } }] } };
}

export async function generateGrammarSelectionMappingRules({ check = false, native = false } = {}) {
  const closed = readGrammarSelectionMappingSourceModule();
  const policy = { schemaVersion: 1, model: 'Unicode scalar JSON string source mapping 1',
    source: closed.source, sourceSha256: hash(closed.source), closedSource: closed.closure,
    closedSourceSha256: hash(closed.closure), sourceClosure: closed.sourceClosure,
    sourcePlanes: 'once-owned ordered source definitions with explicit local positional references; these are not LinkNetwork identifiers or global concept equivalence proofs',
    semantics: 'one complete quoted JSON string; fixed Unicode escapes reuse the actual imported decoder, while JavaScript hexadecimal/braced escapes and non-scalar decoded text refuse',
    physicalOwnership: 'the complete matched syntactic source children remain the physical CST; each accepted child and witness is a complete CST on its separately declared decoded plane',
    boundary: 'Only source mapping correspondence is checked after generic Links decode. Language-CST validity requires parsing with the actual grammar; no priority grammar or arbitrary escape profile is thereby declared complete.' };
  for (const file of [new URL('../src/translation/data/grammar-selection-mapping-rules.lino', import.meta.url),
    new URL('../../rust/src/data/grammar-selection-mapping-rules.lino', import.meta.url)]) {
    if (check) checkProjectData(file, policy);
    else writeProjectData(file, policy);
  }
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const { encodeTypedProgramLinks, decodeTypedProgramLinks } = await import('../src/typed-program-links.js');
    const { recheckLinkedTypedProgram } = await import('../src/typed-program-checked-projection.js');
    const { LinkNetwork } = await import('../src/network.js');
    const { encodeLinkNetworkBinary, decodeLinkNetworkBinary } = await import('../src/link-network-binary.js');
    const checked = checkProgram(parseJavaScript(closed.closure, { referenceSemantics: false }));
    const packet = encodeTypedProgramLinks(checked);
    const text = recheckLinkedTypedProgram(
      decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root), { wholeProgram: true });
    const binary = recheckLinkedTypedProgram(
      decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)), packet.root), { wholeProgram: true });
    const emitted = emitRust(checked, { arrayConsumption: true });
    assert.equal(emitRust(text, { arrayConsumption: true }).text, emitted.text, 'complete text reload preserves every mapping decision');
    assert.equal(emitRust(binary, { arrayConsumption: true }).text, emitted.text, 'complete binary reload preserves every mapping decision');
    const artifact = { schemaVersion: 1, model: policy.model, sourceSha256: policy.sourceSha256,
      sourceClosure: closed.sourceClosure, closedSourceSha256: policy.closedSourceSha256,
      definitions: emitted.definitions, rustSource: emitted.text, rustSha256: hash(emitted.text),
      assumptions: emitted.assumptions, encodings: emitted.encodings };
    const output = new URL('../../parity/self-translation/grammar-selection-mapping-rules-native.lino', import.meta.url);
    if (check) checkProjectData(output, artifact);
    else writeProjectData(output, artifact);
    writeRecordRuleLibrary(emitted, 'grammar_selection_mapping_rules', { check });
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateGrammarSelectionMappingRules({ check: process.argv.includes('--check'), native: process.argv.includes('--native') });
  console.log('JSON string source mapping: original decoder closure and complete generated decisions');
}
