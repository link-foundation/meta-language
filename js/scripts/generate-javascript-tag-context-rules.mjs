// Translate the complete five-decision module; the existing writer owns all
// recorded library packaging. No native algorithm or compiler is invoked here.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';
import { tokenize } from '../src/translation/lexer.js';

const root = new URL('../../', import.meta.url);
const sourcePath = 'js/src/translation/javascript-tag-context-rules.js';
const digest = source => createHash('sha256').update(source).digest('hex');
export const JAVASCRIPT_TAG_CONTEXT_DECISIONS = Object.freeze([
  'readJavaScriptTagOwnerPosition', 'readJavaScriptTagReceiverForm',
  'readJavaScriptTagBindingStep', 'acceptJavaScriptTagHeaderFact',
  'readJavaScriptTagDeclaredOwner',
]);

export function readJavaScriptTagContextSelectorProvenance(source) {
  assert.equal(typeof source, 'string');
  const originalComment = '// PRIVATE JS-first selection decision.';
  const currentComment = '// Shared JS-first selection decision.';
  assert.ok(source.startsWith(currentComment), 'only the exact production selector comment transport is admitted');
  const currentBytes = 578 - Buffer.byteLength(originalComment) + Buffer.byteLength(currentComment);
  const currentPrefix = Buffer.from(source).subarray(0, currentBytes).toString('utf8');
  assert.equal(currentPrefix.split(currentComment).length, 2, 'the exact current comment has one prefix owner');
  const originalPrefix = originalComment + currentPrefix.slice(currentComment.length);
  assert.equal(Buffer.byteLength(originalPrefix), 578);
  assert.equal(digest(originalPrefix), '925ebeab62dc79a723dcb6312b422c9574009d09f0f68a9f51e9039d418eb065',
    'the full historical selector prefix is reconstructed without changing its body');
  assert.equal(currentComment + originalPrefix.slice(originalComment.length), currentPrefix,
    'the exact comment substitution has a complete prefix inverse');
  return { originalSelector: { source: originalPrefix, bytes: 578, sha256: digest(originalPrefix) },
    currentSelector: { source: currentPrefix, bytes: currentBytes, sha256: digest(currentPrefix) },
    selectorCommentTransport: { from: originalComment, to: currentComment,
      boundary: 'Only the first comment text changes. The actual current module is parsed, emitted and hashed unchanged.' } };
}

export function readJavaScriptTagContextSource(source = readFileSync(new URL(sourcePath, root), 'utf8')) {
  readJavaScriptTagContextSelectorProvenance(source);
  assert.doesNotMatch(source, /^\s*import\b/mu, 'this complete module has no unrecorded import dependency');
  const declarations = [...source.matchAll(/^(?:export )?function ([A-Za-z_$][A-Za-z_0-9$]*)\(/gmu)].map(match => match[1]);
  const exports = [...source.matchAll(/^export function ([A-Za-z_$][A-Za-z_0-9$]*)\(/gmu)].map(match => match[1]);
  assert.deepEqual(declarations, [...JAVASCRIPT_TAG_CONTEXT_DECISIONS], 'retain all five complete source declarations');
  assert.deepEqual(exports, declarations, 'all five source roots keep their original exports');
  return source;
}

export function readJavaScriptTagContextPolicy(source = readJavaScriptTagContextSource()) {
  readJavaScriptTagContextSource(source);
  return { schemaVersion: 1, model: 'actual JavaScript declared tag owner context 1',
    sourcePath, source, sourceSha256: digest(source), sourceClosureSha256: digest(source),
    decisions: [...JAVASCRIPT_TAG_CONTEXT_DECISIONS],
    ...readJavaScriptTagContextSelectorProvenance(source),
    inputs: 'Actual receiver kind/callee/path, lexical owner and boundary facts, header availability/asynchrony, declared type owner and candidate owners. Primitive String arrays, Number boundaries and Boolean facts retain their existing source domains.',
    ownership: 'A nearest actual binding stops lookup even without a type fact. Only exact declared Data or rooted Named owners disambiguate multiple candidates. Optional/union children and outer result context do not supply receiver owners.',
    diagnostics: 'Only speculative malformed source headers become an unavailable fact. Ordinary declaration checking, unknown/ambiguous tag refusals and exact source spans remain owned by their existing parser/checker paths.',
    boundary: 'Complete portable decisions and source-generation provenance, not evidence of native execution, arbitrary callable inference, namespace dispatch or whole JavaScript support.' };
}

export function readJavaScriptTagContextOwnerMappings(emission, inventory) {
  return inventory.map(owner => {
    const mappings = emission.mappings.filter(mapping => mapping.source === owner.fullName
      && mapping.kind === (owner.kind === 'fn' ? 'function' : owner.kind));
    assert.equal(mappings.length, 1, 'one actual emitted mapping for ' + owner.fullName);
    return mappings[0];
  });
}

function checkedInventory(program) {
  return [...program.declarations.values()].map((entry, ordinal) => ({ ordinal, kind: entry.k,
    name: entry.name, fullName: entry.fullName, modulePath: entry.modulePath }));
}

// A compiler-free portability boundary usable by the focused JS controls. The
// complete actual returned recheck feeds emission; no decoded predecessor is used.
export async function prepareJavaScriptTagContextTranslation(source = readJavaScriptTagContextSource()) {
  readJavaScriptTagContextSource(source);
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const { emitJavaScript } = await import('../src/translation/emit-javascript.js');
  const { encodeTypedProgramLinks, decodeTypedProgramLinks } = await import('../src/typed-program-links.js');
  const { recheckLinkedTypedProgram } = await import('../src/typed-program-checked-projection.js');
  const { LinkNetwork } = await import('../src/network.js');
  const { encodeLinkNetworkBinary, decodeLinkNetworkBinary } = await import('../src/link-network-binary.js');
  const surface = parseJavaScript(source, { referenceSemantics: false });
  const parsedInventory = surface.items.map((item, ordinal) => {
    assert.ok(['fn', 'data'].includes(item.k) && typeof item.name === 'string',
      'classify every parsed declaration, including actual lowering continuations');
    return { ordinal, kind: item.k, name: item.name, span: item.span };
  });
  const checked = checkProgram(surface), inventory = checkedInventory(checked);
  const sourceOwners = JAVASCRIPT_TAG_CONTEXT_DECISIONS.map(name => {
    const entry = checked.declarations.get(name);
    assert.ok(entry && entry.k === 'fn' && entry.name === name && entry.fullName === name && entry.modulePath.length === 0,
      'retain the actual complete flat source owner ' + name);
    return inventory.find(owner => owner.fullName === name);
  });
  const emitted = emitRust(checked, { arrayConsumption: true }), javascript = emitJavaScript(checked);
  readJavaScriptTagContextOwnerMappings(emitted, inventory);
  readJavaScriptTagContextOwnerMappings(javascript, inventory);
  const packet = encodeTypedProgramLinks(checked), text = packet.network.toLino(), binary = encodeLinkNetworkBinary(packet.network);
  const replays = [];
  const javascriptPrograms = [{ format: 'original', emission: javascript }];
  for (const [format, network] of [
    ['text', LinkNetwork.fromLino(text)], ['binary', decodeLinkNetworkBinary(binary)],
  ]) {
    const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
    const owners = checkedInventory(restored);
    assert.deepEqual(owners, inventory, 'every checked owner/order survives ' + format);
    const replay = emitRust(restored, { arrayConsumption: true }), replayJavaScript = emitJavaScript(restored);
    readJavaScriptTagContextOwnerMappings(replay, owners);
    readJavaScriptTagContextOwnerMappings(replayJavaScript, owners);
    assert.ok(replay.text === emitted.text, 'complete actual Rust text survives ' + format);
    assert.deepEqual(replay.definitions, emitted.definitions, 'all complete Rust definition values survive ' + format);
    assert.deepEqual(replay.preludes, emitted.preludes, 'all Rust preludes survive ' + format);
    assert.deepEqual(replay.mappings, emitted.mappings, 'actual Rust mappings survive ' + format);
    assert.ok(replayJavaScript.text === javascript.text, 'complete JavaScript text survives ' + format);
    assert.deepEqual(replayJavaScript.definitions, javascript.definitions, 'all JavaScript definitions survive ' + format);
    assert.deepEqual(replayJavaScript.preludes, javascript.preludes, 'all JavaScript preludes survive ' + format);
    assert.deepEqual(replayJavaScript.mappings, javascript.mappings, 'actual JavaScript mappings survive ' + format);
    javascriptPrograms.push({ format, emission: replayJavaScript });
    replays.push({ format, checkedInventory: owners,
      rustBytes: Buffer.byteLength(replay.text), rustSha256: digest(replay.text),
      javascriptBytes: Buffer.byteLength(replayJavaScript.text), javascriptSha256: digest(replayJavaScript.text) });
  }
  return { source, sourceSha256: digest(source), parsedInventory, checkedInventory: inventory,
    sourceOwners, emitted, javascript, javascriptPrograms, replays,
    archive: { root: packet.root.asU64(), textBytes: Buffer.byteLength(text), textSha256: digest(text),
      binaryBytes: binary.byteLength, binarySha256: digest(binary) } };
}

// The shared writer applies its actual recorded immutable loans. Inspect every
// real mapped root in the resulting whole-item library, without guessing names.
export function readJavaScriptTagContextNativeParameters(value) {
  const parts = value.split(',').map(part => part.trim());
  if (parts.length === 1 && parts[0] === '') return [];
  if (parts.at(-1) === '') parts.pop();
  assert.ok(parts.every(part => part !== ''), 'only one optional trailing parameter comma is admitted');
  return parts.map(part => part.replace(/\s+/gu, ''));
}

// These two complete generated owners admit only a keyword qualification and
// the same lazy standard constructor as a function item. No body is rebuilt.
export function readJavaScriptTagContextLibraryStyles(source) {
  const {tokens, comments} = tokenize(source, 'Rust'), matching = new Map(), stack = [];
  const unchanged = () => ({source, actions: []});
  for (let at = 0; at < tokens.length; at += 1) {
    const token = tokens[at];
    if (token.kind === 'identifier' && ['use', 'extern', 'unsafe', 'impl'].includes(token.value)) return unchanged();
    if (token.kind === 'identifier' && ['type', 'struct', 'enum', 'union', 'mod', 'trait', 'fn', 'const', 'static', 'let'].includes(token.value)) {
      const next = tokens[at + 1]?.value === 'mut' ? at + 2 : at + 1;
      if (['f64', 'bool', 'String', 'std', 'core'].includes(tokens[next]?.value)) return unchanged();
    }
    if (token.kind === 'macro' && !['vec', 'format', 'panic', 'assert', 'matches'].includes(token.value)) return unchanged();
    if (token.kind !== 'punct') continue;
    if (['(', '[', '{'].includes(token.value)) stack.push(at);
    else if ([')', ']', '}'].includes(token.value)) {
      const open = stack.pop();
      assert.equal(tokens[open]?.value, {')': '(', ']': '[', '}': '{'}[token.value], 'tag context styles require complete delimiters');
      matching.set(open, at); matching.set(at, open);
    }
  }
  assert.equal(stack.length, 0, 'tag context styles retain every delimiter');
  for (let at = 0; at < tokens.length; at += 1) if (tokens[at].value === '#' && tokens[at].kind === 'punct') {
    const open = tokens[at + 1]?.value === '!' ? at + 2 : at + 1;
    if (!['allow', 'warn', 'deny', 'expect', 'must_use', 'derive'].includes(tokens[open + 1]?.value)) return unchanged();
    at = matching.get(open);
  }
  const functions = [];
  for (let at = 0; at < tokens.length; at += 1) {
    if (tokens[at].value === 'pub' && tokens[at].kind === 'identifier') {
      const keyword = tokens[at + 1]?.value === 'const' ? at + 2 : at + 1;
      if (tokens[keyword]?.value === 'fn' && tokens[keyword + 1]?.kind === 'identifier'
        && tokens[keyword + 2]?.value === '(') {
        let body = matching.get(keyword + 2) + 1;
        while (body < tokens.length && !['{', ';'].includes(tokens[body].value)) body += 1;
        const end = matching.get(body);
        if (end !== undefined) {
          functions.push({at, keyword, end, name: tokens[keyword + 1].value});
          at = end; continue;
        }
      }
    }
    if (tokens[at].kind === 'punct' && ['(', '[', '{'].includes(tokens[at].value)) at = matching.get(at);
  }
  const expectedBinding = 'pub fn read_java_script_tag_binding_step(owns_binding:bool,boundary:f64)->f64{if owns_binding{1f64}else if boundary==0f64{0f64}else{2f64}}';
  const expectedOwner = 'pub fn read_java_script_tag_declared_owner(kind:&str,path:&[String],name:&str,root:&str)->String{if kind=="data"{name.to_owned()}else if ((kind=="named")&&((path.len() as f64)==2f64))&&match(self::ml_array::optional_at(path,self::ml_array::number_index(0f64)),root.to_owned()){(Some(ml_left),ml_right)=>ml_left==ml_right,_=>false}{(self::ml_array::optional_at(path,self::ml_array::number_index(1f64))).unwrap_or_else(||String::new())}else{String::new()}}';
  const tokenKey = text => JSON.stringify(tokenize(text, 'Rust').tokens.filter(token => token.kind !== 'eof').map(token => [token.kind, token.raw]));
  let adapted = source;
  const actions = [];
  for (const [name, expected] of [['read_java_script_tag_binding_step', expectedBinding], ['read_java_script_tag_declared_owner', expectedOwner]]) {
    const candidates = functions.filter(fn => fn.name === name);
    if (candidates.length !== 1) continue;
    const fn = candidates[0], start = tokens[fn.at].start, end = tokens[fn.end].end;
    const from = source.slice(start, end);
    if (source.split(from).length !== 2) continue;
    const ordinary = fn.keyword === fn.at + 2 ? from.replace(/^pub\s+const\s+fn/u, 'pub fn') : from;
    const expanded = name === 'read_java_script_tag_declared_owner'
      ? ordinary.replace(/\.unwrap_or_else\(\s*String\s*::\s*new\s*\)/u, '.unwrap_or_else(||String::new())') : ordinary;
    if (tokenKey(expanded) !== tokenKey(expected)) continue;
    let to = from;
    if (name === 'read_java_script_tag_binding_step' && fn.keyword === fn.at + 1) {
      to = from.slice(0, tokens[fn.keyword].start - start) + 'const ' + from.slice(tokens[fn.keyword].start - start);
    } else if (name === 'read_java_script_tag_declared_owner') {
      for (let at = fn.keyword; at < fn.end; at += 1) {
        if (tokens[at].value !== '||' || tokens.slice(at + 1, at + 6).map(token => token.value).join(' ') !== 'String :: new ( )') continue;
        if (comments.some(comment => comment.start >= tokens[at].start && comment.end <= tokens[at + 5].end)) continue;
        to = from.slice(0, tokens[at].start - start) + source.slice(tokens[at + 1].start, tokens[at + 3].end)
          + from.slice(tokens[at + 5].end - start);
      }
    }
    if (from !== to) {
      actions.push({op: 'replace', field: 'source', from, to});
      adapted = adapted.replace(from, to);
    }
  }
  return {source: adapted, actions};
}

export function readJavaScriptTagContextLibraryInterfaces(files, emitted, sourceOwners, inventory = sourceOwners) {
  const source = files.map(file => file.source).join('\n\n');
  const allMappings = readJavaScriptTagContextOwnerMappings(emitted, inventory);
  for (const [at, owner] of inventory.entries()) {
    const target = allMappings[at].target;
    assert.match(target, /^[A-Za-z_][A-Za-z_0-9]*$/u, 'one flat actual checked declaration');
    const form = owner.kind === 'fn' ? '(?:const )?fn' : '(?:enum|struct)';
    assert.equal([...source.matchAll(new RegExp('^pub ' + form + ' ' + target + '\\b', 'gmu'))].length, 1,
      'packaging retains every checked owner, including actual continuations: ' + owner.fullName);
  }
  const mappings = readJavaScriptTagContextOwnerMappings(emitted, sourceOwners);
  const interfaces = sourceOwners.map((owner, at) => {
    const target = mappings[at].target;
    assert.match(target, /^[A-Za-z_][A-Za-z_0-9]*$/u, 'one actual flat native source root');
    assert.equal([...source.matchAll(new RegExp('^pub (?:const )?fn ' + target + '\\b', 'gmu'))].length, 1,
      'the complete packaged library retains ' + owner.fullName);
    const block = readSourceTextRustBlock(source, 'fn', target), original = readSourceTextRustBlock(emitted.text, 'fn', target);
    const header = /\bfn \w+\(([^)]*)\)\s*->\s*([^\n{]+)\s*\{/u.exec(block);
    const before = /\bfn \w+\(([^)]*)\)\s*->\s*([^\n{]+)\s*\{/u.exec(original);
    assert.ok(header && before, 'the actual complete primitive header remains inspectable');
    const expected = readJavaScriptTagContextNativeParameters(before[1]).map(part => part.replace(/:Vec<String>$/u, ':&[String]').replace(/:String$/u, ':&str'));
    assert.deepEqual(readJavaScriptTagContextNativeParameters(header[1]), expected, 'only existing immutable parameter loans change this actual root ABI');
    assert.equal(header[2].replace(/\s+/gu, ''), before[2].replace(/\s+/gu, ''), 'the actual result type stays unchanged');
    return { source: owner.fullName, target, signature: header[0].slice(0, -1).trim(),
      parameters: readJavaScriptTagContextNativeParameters(header[1]), result: header[2].trim() };
  });
  return { interfaces, files: files.map(file => ({ path: file.path ?? 'javascript_tag_context_rules.rs',
    source: file.source, bytes: Buffer.byteLength(file.source), sourceSha256: digest(file.source) })) };
}

export async function generateJavaScriptTagContextRules({ check = false, native = false } = {}) {
  const policy = readJavaScriptTagContextPolicy();
  for (const relative of ['js/src/translation/data/javascript-tag-context-rules.lino', 'rust/src/data/javascript-tag-context-rules.lino']) {
    const file = new URL(relative, root);
    if (check) checkProjectData(file, policy);
    else writeProjectData(file, policy);
  }
  let artifact = null;
  if (native) {
    const translated = await prepareJavaScriptTagContextTranslation(policy.source);
    const files = writeRecordRuleLibrary(translated.emitted, 'javascript_tag_context_rules', { check,
      sourceStyles: source => {
        const result = readJavaScriptTagContextLibraryStyles(source);
        assert.equal(result.actions.length, 2, 'both exact tag context owner styles are recorded');
        return result;
      } });
    const library = readJavaScriptTagContextLibraryInterfaces(files, translated.emitted, translated.sourceOwners, translated.checkedInventory);
    artifact = { schemaVersion: 1, model: policy.model, source: policy.source, sourceSha256: policy.sourceSha256,
      parsedInventory: translated.parsedInventory, checkedInventory: translated.checkedInventory,
      sourceOwners: translated.sourceOwners, archive: translated.archive, replays: translated.replays,
      emitted: { ...translated.emitted, definitions: translated.emitted.definitions, preludes: translated.emitted.preludes },
      emittedBytes: Buffer.byteLength(translated.emitted.text), emittedSha256: digest(translated.emitted.text),
      sourceOwnerMappings: readJavaScriptTagContextOwnerMappings(translated.emitted, translated.sourceOwners), library,
      boundary: 'Actual complete source generation and source-free rechecks through both archives. Native compilation, execution and parser-host parity remain CI obligations.' };
    const output = new URL('parity/self-translation/javascript-tag-context-rules-native.lino', root);
    if (check) checkProjectData(output, artifact);
    else writeProjectData(output, artifact);
  }
  return { policy, artifact };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(new Set(args).size === args.length && args.every(argument => ['--check', '--native'].includes(argument)),
    'only source generation/freshness options are admitted');
  const result = await generateJavaScriptTagContextRules({ check: args.includes('--check'), native: args.includes('--native') });
  console.log(JSON.stringify({ phase: 'whole JavaScript tag context source generation', sourceBytes: Buffer.byteLength(result.policy.source),
    sourceSha256: result.policy.sourceSha256, decisions: result.policy.decisions.length,
    nativeSource: result.artifact === null ? null : { bytes: result.artifact.emittedBytes,
      sha256: result.artifact.emittedSha256, definitions: result.artifact.emitted.definitions.length,
      interfaces: result.artifact.library.interfaces }, nativeCompilation: false, nativeExecution: false }, null, 2));
}
