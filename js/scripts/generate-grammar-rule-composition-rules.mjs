// Translate the complete closed authoring kernel. Grammar nodes and runtime
// selection algorithms are not replaced by this source/library producer.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { tokenize } from '../src/translation/lexer.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';
import { readRustLibraryItems } from './rust-library-packaging.mjs';
import { readPortableSourceParameterStyles } from './portable-source-library-style.mjs';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';
import { DecoratorSet } from '../src/decorators.js';

const root = new URL('../../', import.meta.url);
const sourcePath = 'js/src/translation/grammar-rule-composition-rules.js';
const libraryName = 'grammar_rule_composition_rules';
const digest = source => createHash('sha256').update(source).digest('hex');

export function readGrammarRuleCompositionSource() {
  const source = readFileSync(new URL(sourcePath, root), 'utf8');
  const tokens = tokenize(source, 'JavaScript').tokens;
  assert.ok(!tokens.some((token, index) => token.kind === 'identifier' && (token.value === 'import'
    || token.value === 'export' && ['*', '{'].includes(tokens[index + 1]?.value))),
  'retain one complete import-free kernel, without hidden dependencies or selected exports');
  const exports = tokens.flatMap((token, index) => token.value === 'export' && tokens[index + 1]?.value === 'function'
    ? [tokens[index + 2].value] : []);
  assert.deepEqual(exports, ['readGrammarCompositionFieldOrder', 'readGrammarCompositionContextIssue',
    'acceptGrammarCompositionCandidateProfile', 'readGrammarCompositionCandidateIssue', 'readGrammarCompositionDisposition',
    'readGrammarCompositionPublicationIssue'],
  'the entire original six-export authoring and publication contract remains present');
  const input = { path: sourcePath, source, bytes: Buffer.byteLength(source), sourceSha256: digest(source), removedImports: [] };
  return { source, bytes: input.bytes, sourceSha256: input.sourceSha256, exports,
    sourceClosure: { source, bytes: input.bytes, sourceSha256: input.sourceSha256, inputs: [input] } };
}

export function readGrammarRuleCompositionPolicy(closed = readGrammarRuleCompositionSource()) {
  return { schemaVersion: 1, model: 'explicit syntactic grammar rule composition 1',
    sourcePath, source: closed.source, sourceBytes: closed.bytes, sourceSha256: closed.sourceSha256,
    sourceClosure: closed.sourceClosure, decisions: closed.exports,
    default: 'An absent or empty composition keeps the original grammar expression and default complete CST behavior.',
    admission: 'A final normal syntactic rule accepts only one any-grammar or all-grammar expression with one exact empty authoring slot and a nonempty complete candidate list. The original actual expression replaces only that slot.',
    candidates: 'Exact existing candidate and selector validation precedes the primitive facts. All candidates share field-ranges with one exact common selector, or json-string without a selector. Source names are nonempty and unique; candidate order and every selector field are retained.',
    repetition: 'The complete validated ordered candidate key and outer mode determine whether the current wrapper is retained. A different existing wrapper remains the complete original item; no pointer or name-only marker establishes equality.',
    sourceOwnership: 'Source-qualified decorator sets are applied to their actual grammar before resolver construction. Candidate source identities are not local rule references. Existing source-plane, source-view, scanner and archive checks remain authoritative.',
    treeDecorators: 'Composition refuses executor or recovery decorators without a source-witness proof. The publication guard uses the actual mapped-source registry and effective tree-decorator presence to refuse mapped results before tree decoration. Existing identity-only tree decoration is unchanged.',
    resources: 'Materialization retains the actual original expression. Runtime selection, recursive parsing, mapping and checking use the existing memory, step, depth and source budgets.',
    boundary: 'Finite opt-in authoring decisions and complete-source translation proofs do not establish catalog coverage, language-CST validity or native execution. Those require their actual runtime and CI assertions.' };
}

function checkedOwners(program, inventory) {
  const owners = inventory.map(item => {
    const declaration = program.declarations.get(item.name);
    assert.ok(declaration && declaration.k === item.kind && declaration.name === item.name,
      'retain the actual complete checked owner ' + item.name);
    return { sourceOrdinal: item.ordinal, kind: declaration.k, name: declaration.name,
      fullName: declaration.fullName, generated: item.generated,
      ...(declaration.modulePath === undefined ? {} : { modulePath: declaration.modulePath }) };
  });
  assert.equal(owners.length, program.declarations.size, 'no original or compiler-added owner is omitted');
  assert.equal(new Set(owners.map(owner => owner.fullName)).size, owners.length, 'every actual owner has one identity');
  return owners;
}

function ownerMappings(emitted, owners) {
  return owners.map(owner => {
    const matches = emitted.mappings.filter(mapping => mapping.source === owner.fullName
      && mapping.kind === (owner.kind === 'fn' ? 'function' : owner.kind));
    assert.equal(matches.length, 1, 'one complete emitted mapping for ' + owner.fullName);
    return matches[0];
  });
}

function nativeSignature(source) {
  const header = /\bfn ([A-Za-z_][A-Za-z_0-9]*)\(([^)]*)\)\s*->\s*([^{}]+)\{/u.exec(source);
  assert.ok(header, 'the complete actual primitive function signature is present');
  return { name: header[1], parameters: header[2].split(',').map(part => part.trim()).filter(Boolean)
    .map(part => part.replace(/\s+/gu, '')), result: header[3].replace(/\s+/gu, '') };
}

/** Mechanical styles for the actual complete candidate owners. Authenticate
 * the standard Option<String> reader and every selected input/binding first. */
export function readGrammarRuleCompositionLibraryStyles(source) {
  const unchanged = { source, actions: [] };
  const whole = tokenize(source, 'Rust').tokens;
  const protectedNames = new Set(['String', 'str', 'Option', 'Vec']);
  for (let at = 0; at < whole.length; at += 1) {
    if (whole[at].kind === 'macro' && !['vec', 'format', 'panic', 'assert', 'matches'].includes(whole[at].value)) return unchanged;
    if (whole[at].kind !== 'identifier') continue;
    if (['use', 'extern', 'unsafe', 'impl'].includes(whole[at].value)) return unchanged;
    if (['type', 'struct', 'enum', 'mod', 'trait'].includes(whole[at].value)
      && protectedNames.has(whole[at + 1]?.value)) return unchanged;
  }
  const items = readRustLibraryItems(source).items;
  const arrays = items.filter(item => item.kind === 'mod' && item.name === 'ml_array');
  const spelling = text => tokenize(text, 'Rust').tokens.filter(token => token.kind !== 'eof')
    .map(token => token.kind === 'number' ? token.raw : token.value).join('');
  if (arrays.length !== 1 || spelling(readSourceTextRustBlock(arrays[0].source, 'fn', 'optional_at'))
    !== 'pubfnoptional_at<T:Clone>(values:&[T],index:Option<usize>)->Option<T>{index.and_then(|index|values.get(index)).cloned()}') return unchanged;
  let adapted = source;
  const actions = [];
  for (const item of items.filter(item => item.kind === 'fn'
    && /^(?:read_grammar_composition_candidate_issue|ml_read_grammar_composition_candidate_issue_loop[0-9]+)$/u.test(item.name))) {
    if (items.filter(other => other.kind === 'fn' && other.name === item.name).length !== 1) continue;
    const block = readSourceTextRustBlock(item.source, 'fn', item.name), { tokens, comments } = tokenize(block, 'Rust');
    const matching = new Map(), stack = [];
    for (let at = 0; at < tokens.length; at += 1) {
      if (tokens[at].kind !== 'punct') continue;
      if (['(', '[', '{'].includes(tokens[at].value)) stack.push(at);
      else if ([')', ']', '}'].includes(tokens[at].value)) {
        const open = stack.pop();
        assert.equal(tokens[open]?.value, { ')': '(', ']': '[', '}': '{' }[tokens[at].value]);
        matching.set(open, at); matching.set(at, open);
      }
    }
    assert.equal(stack.length, 0, 'the complete actual style owner has balanced delimiters');
    const words = (start, end) => tokens.slice(start, end).map(token => token.kind === 'number' ? token.raw : token.value).join('');
    const clean = (start, end) => !comments.some(comment => comment.start >= tokens[start].start && comment.end <= tokens[end - 1].end);
    const fn = tokens.findIndex(token => token.value === 'fn'), open = fn + 2, close = matching.get(open);
    if (tokens[open]?.value !== '(' || close === undefined) continue;
    const header = words(open + 1, close), columns = new Set([...header.matchAll(/(?:^|,)(\w+):&\[String\](?=,|$)/gu)].map(match => match[1]));
    const scalars = new Set([...header.matchAll(/(?:^|,)(?:mut)?(\w+):f64(?=,|$)/gu)].map(match => match[1]));
    if (columns.size === 0 || tokens.some((token, at) => token.kind === 'macro'
      || token.value === 'fn' && at !== fn || ['for', '|'].includes(token.value)
      || token.value === '=>' && tokens[at - 1]?.value === '}')) continue;
    const bindings = new Map(), assigned = new Set(), patterns = new Set();
    for (let at = close + 1; at < tokens.length; at += 1) {
      if (tokens[at].value === 'let') {
        let end = at + 1;
        while (end < tokens.length && !['=', ';', '}'].includes(tokens[end].value)) end += 1;
        for (const token of tokens.slice(at + 1, end)) if (token.kind === 'identifier' && !['mut', 'ref'].includes(token.value)) {
          bindings.set(token.value, (bindings.get(token.value) ?? 0) + 1);
        }
      }
      if (tokens[at].value === '=>') {
        let first = at - 1;
        // Rust permits a block-bodied match arm without a trailing comma.
        // Its closing brace ends the previous arm, never this pattern.
        while (first > close && ![',', '{', '}'].includes(tokens[first].value)) first = matching.get(first) < first ? matching.get(first) - 1 : first - 1;
        for (const token of tokens.slice(first + 1, at)) if (token.kind === 'identifier') patterns.add(token.value);
      }
      if (tokens[at].kind === 'punct' && ['=', '+=', '-=', '*=', '/=', '%='].includes(tokens[at].value)) {
        if (tokens[at - 1]?.kind === 'identifier' && tokens[at - 2]?.value !== 'let'
          && !(tokens[at - 2]?.value === 'mut' && tokens[at - 3]?.value === 'let')) assigned.add(tokens[at - 1].value);
        if (tokens[at - 1]?.value === ')') {
          const first = matching.get(at - 1);
          for (const token of tokens.slice(first + 1, at - 1)) if (token.kind === 'identifier') assigned.add(token.value);
        }
      }
    }
    for (const name of [...columns]) if (bindings.has(name) || patterns.has(name) || assigned.has(name)) columns.delete(name);
    for (const name of [...scalars]) if (bindings.has(name) || patterns.has(name)) scalars.delete(name);
    const edits = [], strings = new Set();
    for (let at = close + 1; at < tokens.length; at += 1) {
      if (words(at, at + 3) !== '.unwrap_or_else(') continue;
      const end = matching.get(at + 2);
      if (end === undefined || words(at + 3, end) !== '||String::new()' || tokens[at - 1]?.value !== ')') continue;
      const start = matching.get(at - 1);
      let first = start, last = at;
      while (tokens[first]?.value === '(' && matching.get(first) === last - 1) { first += 1; last -= 1; }
      if (words(first, first + 6) !== 'self::ml_array::optional_at(' || matching.get(first + 5) !== last - 1
        || !columns.has(tokens[first + 6]?.value) || tokens[first + 7]?.value !== ',') continue;
      const index = first + 8, indexEnd = matching.get(index + 5);
      if (words(index, index + 6) !== 'self::ml_array::number_index(' || indexEnd !== last - 2
        || indexEnd !== index + 7 || !(tokens[index + 6]?.kind === 'number' || scalars.has(tokens[index + 6]?.value))
        || !clean(start, end + 1)) continue;
      edits.push([tokens[at].start, tokens[end].end, '.unwrap_or_default()']);
      const name = tokens[start - 2]?.value;
      if (tokens[start - 3]?.value === 'let' && tokens[start - 2]?.kind === 'identifier' && tokens[start - 1]?.value === '='
        && tokens[end + 1]?.value === ';' && bindings.get(name) === 1 && !patterns.has(name) && !assigned.has(name)) strings.add(name);
    }
    for (let at = close + 1; at + 2 < tokens.length; at += 1) {
      if (tokens[at].kind !== 'identifier' || !strings.has(tokens[at].value)
        || !['==', '!='].includes(tokens[at + 1].value) || tokens[at + 2].kind !== 'string'
        || tokens[at + 2].value !== '' || !clean(at, at + 3)) continue;
      edits.push([tokens[at].start, tokens[at + 2].end, (tokens[at + 1].value === '!=' ? '!' : '') + tokens[at].value + '.is_empty()']);
    }
    let styled = block, boundary = block.length;
    for (const [start, end, text] of edits.sort((left, right) => right[0] - left[0])) {
      assert.ok(end <= boundary, 'the actual String style edits have disjoint original spans');
      styled = styled.slice(0, start) + text + styled.slice(end); boundary = start;
    }
    if (styled !== block) {
      assert.equal(adapted.split(block).length, 2, 'one complete actual function owner is recorded');
      actions.push({ op: 'replace', field: 'source', from: block, to: styled });
      adapted = adapted.replace(block, () => styled);
    }
  }
  return { source: adapted, actions };
}

export function readGrammarRuleCompositionLibraryInterface(files, emitted, owners) {
  const items = files.flatMap(file => readRustLibraryItems(file.source).items);
  const mappings = ownerMappings(emitted, owners), interfaces = [];
  const stylePath = 'parity/self-translation/grammar-rule-composition-rules-decorators.lino';
  const styleSource = readFileSync(new URL(stylePath, root), 'utf8');
  const styles = DecoratorSet.fromLino(styleSource).decorators;
  const recorded = styles.filter(style => style.id === libraryName + '-library-style');
  assert.equal(recorded.length, 1, 'one actual complete recorded library style');
  const dataOwners = owners.flatMap((owner, index) => owner.kind === 'data' ? [{ owner, target: mappings[index].target }] : []);
  for (const [index, owner] of owners.entries()) {
    const target = mappings[index].target;
    const declarations = items.filter(item => item.name === target
      && (owner.kind === 'fn' ? item.kind === 'fn' : ['struct', 'enum'].includes(item.kind)));
    assert.equal(declarations.length, 1, 'packaging retains the complete actual owner ' + owner.fullName);
    if (owner.kind !== 'fn') continue;
    const raw = readSourceTextRustBlock(emitted.text, 'fn', target), before = nativeSignature(raw);
    const loan = nativeSignature(readPortableSourceParameterStyles(raw).source);
    const actual = nativeSignature(declarations[0].source);
    assert.deepEqual(actual.parameters, loan.parameters, 'the actual ABI follows the shared immutable parameter decisions');
    let result = before.result;
    const relocations = [];
    for (const data of dataOwners) {
      const from = 'crate::' + data.target, to = 'self::' + data.target;
      if (!result.includes(from)) continue;
      assert.equal(recorded[0].actions.filter(action => action.op === 'replace' && action.field === 'source'
        && action.from === from && action.to === to).length, 1, 'the local data return owner has one recorded relocation');
      result = result.replaceAll(from, to); relocations.push({ source: data.owner.fullName, target: data.target, from, to });
    }
    assert.equal(actual.result, result, 'the actual return type changes only by recorded local owner relocation');
    interfaces.push({ source: owner.fullName, target, generated: owner.generated,
      parameters: actual.parameters, result: actual.result, rawParameters: before.parameters, rawResult: before.result,
      returnOwnerRelocations: relocations, rawDeclarationSha256: digest(raw), packagedDeclarationSha256: digest(declarations[0].source) });
  }
  assert.equal(interfaces.length, owners.filter(owner => owner.kind === 'fn').length, 'every actual function has a complete ABI proof');
  return { ownerMappings: mappings, interfaces,
    libraryStyle: { path: stylePath, bytes: Buffer.byteLength(styleSource), sourceSha256: digest(styleSource) },
    files: files.map(file => ({ path: file.path ?? libraryName + '.rs', bytes: Buffer.byteLength(file.source), sourceSha256: digest(file.source) })) };
}

export async function generateGrammarRuleCompositionRules({ check = false, native = false } = {}) {
  const closed = readGrammarRuleCompositionSource(), policy = readGrammarRuleCompositionPolicy(closed);
  for (const path of ['js/src/translation/data/grammar-rule-composition-rules.lino', 'rust/src/data/grammar-rule-composition-rules.lino']) {
    if (check) checkProjectData(new URL(path, root), policy);
    else writeProjectData(new URL(path, root), policy);
  }
  let artifact = null;
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const { encodeTypedProgramLinks, decodeTypedProgramLinks } = await import('../src/typed-program-links.js');
    const { recheckLinkedTypedProgram } = await import('../src/typed-program-checked-projection.js');
    const { LinkNetwork } = await import('../src/network.js');
    const { encodeLinkNetworkBinary, decodeLinkNetworkBinary } = await import('../src/link-network-binary.js');
    const surface = parseJavaScript(closed.source, { referenceSemantics: false });
    const inventory = surface.items.map((item, ordinal) => {
      assert.ok(['fn', 'data'].includes(item.k) && typeof item.name === 'string' && item.name !== '',
        'discover every actual original and compiler-added declaration');
      return { ordinal, kind: item.k, name: item.name, generated: item.generated === true,
        ...(item.span === undefined ? {} : { span: item.span }) };
    });
    assert.deepEqual(inventory.filter(item => !item.generated).map(item => item.name), closed.exports,
      'every original source export is checked, with no source subset or added host implementation');
    const checked = checkProgram(surface), owners = checkedOwners(checked, inventory);
    const output = emitRust(checked, { arrayConsumption: true });
    const emitted = { text: output.text, definitions: output.definitions, preludes: output.preludes,
      mappings: output.mappings, assumptions: output.assumptions, encodings: output.encodings };
    ownerMappings(emitted, owners);
    const packet = encodeTypedProgramLinks(checked), text = packet.network.toLino();
    const binary = encodeLinkNetworkBinary(packet.network), replays = [];
    for (const [format, network] of [['text', LinkNetwork.fromLino(text)], ['binary', decodeLinkNetworkBinary(binary)]]) {
      const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
      const restoredOwners = checkedOwners(restored, inventory);
      assert.deepEqual(restoredOwners, owners, 'every complete checked owner/order survives ' + format);
      const replay = emitRust(restored, { arrayConsumption: true });
      ownerMappings(replay, restoredOwners);
      for (const field of ['text', 'definitions', 'preludes', 'mappings', 'assumptions', 'encodings']) {
        assert.deepEqual(replay[field], emitted[field], 'complete emitted ' + field + ' survives whole-program ' + format);
      }
      replays.push({ format, checkedNamedSourceInventory: restoredOwners,
        emittedTextBytes: Buffer.byteLength(replay.text), emittedTextSha256: digest(replay.text) });
    }
    const files = writeRecordRuleLibrary(emitted, libraryName, { check, sourceStyles: readGrammarRuleCompositionLibraryStyles });
    artifact = { schemaVersion: 1, model: policy.model, source: closed.source, sourceSha256: closed.sourceSha256,
      sourceClosure: closed.sourceClosure, inventory, checkedNamedSourceInventory: owners, replays, emitted,
      emittedTextBytes: Buffer.byteLength(emitted.text), emittedTextSha256: digest(emitted.text),
      archive: { root: packet.root.asU64(), textBytes: Buffer.byteLength(text), textSha256: digest(text),
        binaryBytes: binary.byteLength, binarySha256: digest(binary) },
      library: readGrammarRuleCompositionLibraryInterface(files, emitted, owners),
      boundary: 'Complete original source, all discovered compiler owners, whole-program text/binary archive and emission/ABI proofs. Native compilation, adapter execution and language-CST validity require actual CI observations.' };
    const path = new URL('parity/self-translation/grammar-rule-composition-rules-native.lino', root);
    if (check) checkProjectData(path, artifact);
    else writeProjectData(path, artifact);
  }
  return { policy, artifact };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(new Set(args).size === args.length && args.every(argument => ['--check', '--native'].includes(argument)),
    'only complete source-generation/freshness options are admitted');
  const result = await generateGrammarRuleCompositionRules({ check: args.includes('--check'), native: args.includes('--native') });
  console.log(JSON.stringify({ phase: 'complete grammar rule composition source generation', sourceBytes: result.policy.sourceBytes,
    sourceSha256: result.policy.sourceSha256, nativeSource: result.artifact === null ? null : {
      bytes: result.artifact.emittedTextBytes, sha256: result.artifact.emittedTextSha256,
      owners: result.artifact.checkedNamedSourceInventory.length, interfaces: result.artifact.library.interfaces },
    nativeCompilation: false, nativeExecution: false }, null, 2));
}
