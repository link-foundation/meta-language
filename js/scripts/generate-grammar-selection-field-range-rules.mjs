// Capture and translate the complete closed physical-field selection kernel. The shared
// writer records library loans/styles; no native algorithm is authored here.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { tokenize } from '../src/translation/lexer.js';
import { DecoratorSet } from '../src/decorators.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';
import { readRustLibraryItems } from './rust-library-packaging.mjs';
import { readPortableSourceParameterStyles } from './portable-source-library-style.mjs';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';
import { readGrammarSelectionRangeLibraryStyles } from './generate-grammar-selection-included-range-rules.mjs';

const root = new URL('../../', import.meta.url);
const sourcePath = 'js/src/translation/grammar-selection-field-range-rules.js';
const libraryName = 'grammar_selection_field_range_rules';
const digest = source => createHash('sha256').update(source).digest('hex');

/** Retain the whole actual source; new dependencies must be closed explicitly. */
export function readGrammarSelectionFieldRangeSource() {
  const source = readFileSync(new URL(sourcePath, root), 'utf8');
  assert.doesNotMatch(source, /^\s*(?:import\b|export\s+(?:\*|\{))/mu,
    'the field-range kernel is one complete module without unrecorded imports or reexports');
  const tokens = tokenize(source, 'JavaScript').tokens;
  assert.ok(!tokens.some((token, index) => token.kind === 'identifier' && (token.value === 'import'
    || token.value === 'export' && ['*', '{'].includes(tokens[index + 1]?.value))),
  'actual dependency tokens cannot be hidden by line layout');
  assert.ok(Buffer.byteLength(source) > 0, 'the complete actual source is present');
  const input = { path: sourcePath, source, bytes: Buffer.byteLength(source),
    sourceSha256: digest(source), removedImports: [] };
  return { source, bytes: input.bytes, sourceSha256: input.sourceSha256,
    sourceClosure: { source, bytes: input.bytes, sourceSha256: input.sourceSha256, inputs: [input] } };
}

export function readGrammarSelectionFieldRangePolicy(closed = readGrammarSelectionFieldRangeSource()) {
  return { schemaVersion: 1, model: 'proper physical CST field range selection 1',
    sourcePath, source: closed.source, sourceBytes: closed.bytes, sourceSha256: closed.sourceSha256,
    sourceClosure: closed.sourceClosure,
    profile: 'The field-ranges mapping admits one source-field-selector with an exact nonempty field label, include or exclude mode, nonnegative integral first occurrence and strictly positive integral count. Counts and source positions are bounded by the original unsigned 32-bit source domain.',
    context: 'A field profile cannot be mixed with another source mapping or selected inside a lexical token. Zero count never means all; the exact label is a capture field, not a grammar rule name.',
    occurrences: 'The host supplies topmost non-trivia physical CST field occurrences in their original child order. Every supplied absolute span is validated, including unselected occurrences. The closed kernel neither sorts nor merges spans nor discovers trees through callbacks.',
    correspondence: 'The complete plan consumes one read-only flat numeric span column. Include mode retains selected relative pairs; exclude mode retains their ordered complement. Empty and full-width results refuse, so admitted output is a proper contraction consumed by the unchanged scalar-aware included-range mapper.',
    ownership: 'The existing physical tree traversal and source-plane checker retain actual child ownership, exact occurrence paths, complete source bytes and UTF8 scalar boundaries. Equal labels, byte strings or hashes do not establish those obligations.',
    resources: 'Host traversal, witness storage, source mapping and recursion retain their existing memory, step and depth budgets. The planner does not copy the complete immutable span column at each continuation.',
    archiveBoundary: 'Generic Links decoding does not authenticate source mappings. Source-free checking must replay the actual field occurrences and generated range plan against the complete physical CST and retained source definitions.',
    legacyBoundary: 'Identity, JSON-string and explicit included-range decision modules remain separate unchanged inputs. This producer retains every complete source and generated continuation owner rather than selecting a convenient function subset.',
    boundary: 'This finite physical-field profile does not establish complete Markdown or other priority-language support, scanner context discharge or language-CST validity. Native compilation and execution require actual CI observations.' };
}

function checkedOwners(program, inventory) {
  const owners = inventory.map(item => {
    const declaration = program.declarations.get(item.name);
    assert.ok(declaration && declaration.k === item.kind && declaration.name === item.name,
      'retain the actual complete checked owner ' + item.name);
    return { sourceOrdinal: item.ordinal, kind: declaration.k, name: declaration.name,
      fullName: declaration.fullName, modulePath: declaration.modulePath, generated: item.generated };
  });
  assert.equal(owners.length, program.declarations.size, 'no checked source or continuation owner is omitted');
  assert.equal(new Set(owners.map(owner => owner.fullName)).size, owners.length, 'every actual owner has one identity');
  return owners;
}

function ownerMappings(emitted, owners) {
  return owners.map(owner => {
    const mappings = emitted.mappings.filter(mapping => mapping.source === owner.fullName
      && mapping.kind === (owner.kind === 'fn' ? 'function' : owner.kind));
    assert.equal(mappings.length, 1, 'one complete actual emitted mapping for ' + owner.fullName);
    return mappings[0];
  });
}

function nativeParameters(value) {
  const parts = value.split(',').map(part => part.trim());
  if (parts.length === 1 && parts[0] === '') return [];
  if (parts.at(-1) === '') parts.pop();
  assert.ok(parts.every(part => part !== ''), 'only one optional trailing parameter comma is admitted');
  return parts.map(part => {
    const declaration = /^(mut\s+)?([A-Za-z_][A-Za-z_0-9]*)\s*:\s*([\s\S]+)$/u.exec(part);
    assert.ok(declaration, 'the actual primitive parameter retains its name and type');
    return (declaration[1] ? 'mut ' : '') + declaration[2] + ':' + declaration[3].replace(/\s+/gu, '');
  });
}

/** One actual continuation may discard an else only after both paths continue. */
function readGrammarSelectionFieldRangeContinuationStyles(source) {
  const unchanged = { source, actions: [] }, items = readRustLibraryItems(source).items;
  const name = 'ml_read_grammar_selection_field_range_plan_loop3';
  const enumeration = 'MlReadGrammarSelectionFieldRangePlanJoin5';
  const variant = enumeration + 'Next';
  const owners = items.filter(item => item.kind === 'fn' && item.name === name);
  const declarations = items.filter(item => item.name === enumeration);
  if (owners.length !== 1 || declarations.length !== 1 || declarations[0].kind !== 'enum') return unchanged;
  const words = tokens => tokens.filter(token => token.kind !== 'eof')
    .map(token => token.kind === 'number' ? token.raw : token.value).join('');
  const { tokens: enumTokens, comments: enumComments } = tokenize(declarations[0].source, 'Rust');
  const enumStart = enumTokens.findIndex(token => token.kind === 'identifier' && token.value === 'enum');
  if (!['pub', '#[derive(Clone,Debug,PartialEq)]pub'].includes(words(enumTokens.slice(0, enumStart)))
    || words(enumTokens.slice(enumStart)) !== `enum${enumeration}{${variant}(Vec<f64>,f64),}`
    || enumComments.some(comment => comment.start >= enumTokens[enumStart].start)) return unchanged;
  const complete = tokenize(source, 'Rust').tokens;
  if (complete.some((token, at) => token.kind === 'identifier'
    && ['type', 'struct', 'enum', 'union', 'mod', 'trait'].includes(token.value)
    && complete[at + 1]?.value === 'Vec')) return unchanged;
  const from = owners[0].source;
  const header = /\bfn \w+\(([^()]*)\)\s*->\s*self::MlReadGrammarSelectionFieldRangePlanLoop3Result\s*\{/u.exec(from);
  if (!header || JSON.stringify(nativeParameters(header[1])) !== JSON.stringify([
    'spans:&[f64]', 'mut start:f64', 'mut end:f64', 'mut first:f64', 'mode:&str',
    'mut following:f64', 'mut ranges:Vec<f64>', 'mut previous:f64', 'mut cursor:f64', 'mut width:f64', 'mut index:f64',
  ])) return unchanged;
  const { tokens, comments } = tokenize(from, 'Rust'), matching = new Map(), stack = [];
  for (let at = 0; at < tokens.length; at += 1) {
    if (tokens[at].kind !== 'punct') continue;
    if (['(', '[', '{'].includes(tokens[at].value)) stack.push(at);
    else if ([')', ']', '}'].includes(tokens[at].value)) {
      const open = stack.pop();
      assert.equal(tokens[open]?.value, { ')': '(', ']': '[', '}': '{' }[tokens[at].value]);
      matching.set(open, at); matching.set(at, open);
    }
  }
  assert.equal(stack.length, 0, 'the continuing-branch proof retains the complete actual owner');
  const body = tokens.findIndex(token => token.kind === 'punct' && token.value === '{');
  const end = matching.get(body);
  if (body < 0 || end === undefined || comments.some(comment => comment.start >= tokens[body].start
    && comment.end <= tokens[end].end)) return unchanged;
  if (tokens.slice(body + 1, end).some(token => token.kind === 'macro' && token.value !== 'vec'
    || token.kind === 'identifier' && ['fn', 'impl', 'unsafe', 'async', 'move', 'mod', 'enum', 'struct', 'type', 'trait', 'use', 'extern'].includes(token.value)
    || token.kind === 'punct' && token.value === '|')) return unchanged;
  const spelling = (first, last) => words(tokens.slice(first, last));
  const conditional = (start, finish) => {
    let open = start + 1;
    while (open < finish && tokens[open].value !== '{') {
      if (tokens[open].kind === 'macro' || ['let', 'match', ';', '}', '=>'].includes(tokens[open].value)) return undefined;
      if (['(', '['].includes(tokens[open].value)) open = matching.get(open) + 1;
      else open += 1;
    }
    const close = matching.get(open);
    if (close === undefined || close >= finish) return undefined;
    let alternate, end = close + 1;
    if (tokens[end]?.value === 'else') {
      const next = end + 1;
      if (tokens[next]?.value !== '{') return undefined;
      const last = matching.get(next);
      if (last === undefined || last >= finish) return undefined;
      alternate = { open: next, close: last }; end = last + 1;
    }
    return { kind: 'if', start, open, close, alternate, end };
  };
  const statements = (first, finish) => {
    const result = [];
    for (let at = first; at < finish;) {
      if (tokens[at].value === ';') { at += 1; continue; }
      let node;
      if (tokens[at].value === '{') {
        const close = matching.get(at);
        if (close === undefined || close >= finish) return undefined;
        node = { kind: 'block', open: at, close, end: close + 1 };
      } else if (tokens[at].value === 'if') node = conditional(at, finish);
      else if (tokens[at].value === 'match' && tokens[at + 1]?.kind === 'identifier' && tokens[at + 2]?.value === '{') {
        const close = matching.get(at + 2);
        if (close === undefined || close >= finish) return undefined;
        node = { kind: 'match', name: tokens[at + 1].value, open: at + 2, close, end: close + 1 };
      } else if (tokens[at].value === 'continue' && tokens[at + 1]?.value === ';') {
        node = { kind: 'continue', end: at + 2 };
      } else if (spelling(at, at + 6) === `self::${enumeration}::${variant}(`) {
        const close = matching.get(at + 5);
        if (close === undefined || close >= finish) return undefined;
        node = { kind: 'constructor', open: at + 5, close, end: close + 1 };
      } else {
        let last = at;
        for (; last < finish && tokens[last].value !== ';'; last += 1) {
          if (['(', '[', '{'].includes(tokens[last].value)) last = matching.get(last);
        }
        if (last >= finish) return undefined;
        node = tokens[at].value === 'let' && tokens[at + 1]?.kind === 'identifier' && tokens[at + 2]?.value === '='
          ? { kind: 'let', name: tokens[at + 1].value, first: at + 3, last, end: last + 1 }
          : { kind: 'other', end: last + 1 };
      }
      if (!node || node.end <= at || node.end > finish) return undefined;
      result.push(node); at = node.end;
    }
    return result;
  };
  const pair = (open, close) => tokens[open + 1]?.kind === 'identifier' && tokens[open + 2]?.value === ','
    && tokens[open + 3]?.kind === 'identifier' && tokens[open + 4]?.value === ')'
    && open + 4 === close && tokens[open + 1].value !== tokens[open + 3].value;
  const constructs = (first, finish) => {
    const last = statements(first, finish)?.at(-1);
    if (last?.kind === 'constructor') return pair(last.open, last.close);
    if (last?.kind === 'block') return constructs(last.open + 1, last.close);
    return last?.kind === 'if' && Boolean(last.alternate) && constructs(last.open + 1, last.close)
      && constructs(last.alternate.open + 1, last.alternate.close);
  };
  const continues = (open, close) => {
    const sequence = statements(open + 1, close), last = sequence?.at(-1);
    if (last?.kind === 'continue') return true;
    if (last?.kind === 'block') return continues(last.open, last.close);
    if (last?.kind === 'if') return Boolean(last.alternate) && continues(last.open, last.close)
      && continues(last.alternate.open, last.alternate.close);
    if (last?.kind !== 'match') return false;
    const first = last.open + 1, patternEnd = matching.get(first + 5);
    if (spelling(first, first + 6) !== `self::${enumeration}::${variant}(` || patternEnd === undefined
      || !pair(first + 5, patternEnd) || tokens[patternEnd + 1]?.value !== '=>'
      || tokens[patternEnd + 2]?.value !== '{') return false;
    const arm = patternEnd + 2, armEnd = matching.get(arm);
    if (armEnd === undefined || !(armEnd + 1 === last.close
      || tokens[armEnd + 1]?.value === ',' && armEnd + 2 === last.close)) return false;
    const bindings = sequence.slice(0, -1).filter(node => node.kind === 'let' && node.name === last.name);
    return bindings.length === 1 && sequence.at(-2) === bindings[0]
      && constructs(bindings[0].first, bindings[0].last) && continues(arm, armEnd);
  };
  const occurrenceBindings = tokens.filter((token, at) => token.value === 'let' && tokens[at + 1]?.value === 'occurrence');
  if (occurrenceBindings.length !== 1 || !tokens.some((token, at) => token.value === 'let'
    && spelling(at, at + 7) === 'letoccurrence=index/2f64;')) return unchanged;
  const edits = [];
  for (let at = body + 1; at < end; at += 1) {
    if (tokens[at].value !== 'if' || !['{', ';'].includes(tokens[at - 1]?.value)) continue;
    const branch = conditional(at, end);
    if (!branch?.alternate || spelling(at + 1, branch.open) !== '(occurrence>=first)&&(occurrence<following)'
      || !['}', ';'].includes(tokens[branch.end]?.value)) continue;
    // A bare block used as a let/return/call/match value is not a statement boundary.
    if ([...matching].some(([open, close]) => open < at && close > at && tokens[open].value === '{'
      && ['=', 'return', 'break', '(', '[', ',', '=>'].includes(tokens[open - 1]?.value))) continue;
    if (continues(branch.open, branch.close) && continues(branch.alternate.open, branch.alternate.close)) {
      edits.push(tokens[branch.close + 1]);
    }
  }
  if (edits.length !== 1) return unchanged;
  const token = edits[0], to = from.slice(0, token.start) + ' '.repeat(token.end - token.start) + from.slice(token.end);
  assert.equal(source.split(from).length, 2, 'one complete continuing branch owner is recorded');
  return { source: source.replace(from, () => to), actions: [{ op: 'replace', field: 'source', from, to }] };
}

/** Remove only pure literal type guards already fixed by the actual borrowed ABI. */
export function readGrammarSelectionFieldRangeLibraryStyles(source) {
  const numeric = readGrammarSelectionRangeLibraryStyles(source);
  const { tokens } = tokenize(numeric.source, 'Rust');
  const protectedNames = new Set(['str', 'String', 'bool', 'f64', 'std', 'core']);
  for (let at = 0; at < tokens.length; at += 1) {
    const token = tokens[at];
    if (token.kind === 'identifier' && ['use', 'extern', 'unsafe', 'impl'].includes(token.value)) return numeric;
    if (token.kind === 'identifier' && ['type', 'struct', 'enum', 'union', 'mod', 'trait', 'fn', 'const', 'static', 'let'].includes(token.value)) {
      const name = tokens[at + 1]?.value === 'mut' ? at + 2 : at + 1;
      if (protectedNames.has(tokens[name]?.value)) return numeric;
    }
    if (token.kind === 'macro' && !['vec', 'format', 'panic', 'assert', 'matches'].includes(token.value)) return numeric;
    if (token.kind === 'punct' && token.value === '#') {
      const open = tokens[at + 1]?.value === '!' ? at + 2 : at + 1;
      if (tokens[open]?.value !== '[' || !['allow', 'warn', 'deny', 'expect', 'must_use', 'derive'].includes(tokens[open + 1]?.value)) return numeric;
    }
  }
  const continuing = readGrammarSelectionFieldRangeContinuationStyles(numeric.source);
  numeric.source = continuing.source;
  numeric.actions.push(...continuing.actions);
  const items = readRustLibraryItems(numeric.source).items;
  const owners = [
    ['accept_grammar_selection_field_range_selector', ['kind:&str', 'label:&str', 'mode:&str', 'first:f64', 'count:f64'], 1],
    ['accept_grammar_selection_field_range_match', ['actual_field:&str', 'label:&str', 'trivia:bool'], 2],
  ];
  const replacements = [];
  for (const [name, parameters, count] of owners) {
    const candidates = items.filter(item => item.kind === 'fn' && item.name === name);
    if (candidates.length !== 1) return numeric;
    const from = candidates[0].source, header = /\bfn \w+\(([^()]*)\)\s*->\s*bool\s*\{/u.exec(from);
    if (!header || JSON.stringify(nativeParameters(header[1])) !== JSON.stringify(parameters)) return numeric;
    const { tokens, comments } = tokenize(from, 'Rust'), matching = new Map(), stack = [];
    for (let at = 0; at < tokens.length; at += 1) {
      if (tokens[at].kind === 'macro') return numeric;
      if (tokens[at].kind !== 'punct') continue;
      if (['(', '[', '{'].includes(tokens[at].value)) stack.push(at);
      else if ([')', ']', '}'].includes(tokens[at].value)) {
        const open = stack.pop();
        assert.equal(tokens[open]?.value, { ')': '(', ']': '[', '}': '{' }[tokens[at].value]);
        matching.set(open, at);
      }
    }
    assert.equal(stack.length, 0, 'literal type guards retain their complete actual owner');
    const edits = [];
    for (let at = 0; at + 6 < tokens.length; at += 1) {
      if (tokens[at].kind !== 'punct' || tokens[at].value !== '&&'
        || tokens[at + 1].kind !== 'punct' || tokens[at + 1].value !== '(' || matching.get(at + 1) !== at + 5
        || tokens[at + 2].kind !== 'string' || tokens[at + 2].raw !== '"string"'
        || tokens[at + 3].kind !== 'punct' || tokens[at + 3].value !== '=='
        || tokens[at + 4].kind !== 'string' || tokens[at + 4].raw !== '"string"'
        || tokens[at + 6].kind !== 'punct' || ![')', '&&', '||', '}'].includes(tokens[at + 6].value)) continue;
      if (comments.some(comment => comment.start >= tokens[at].start && comment.end <= tokens[at + 5].end)) return numeric;
      // Only this complete, side-effect-free right operand disappears. The
      // preceding operand and every later operand retain their order and bytes.
      edits.push([tokens[at].start, tokens[at + 5].end]);
    }
    if (edits.length !== count) return numeric;
    let to = from;
    for (const [start, end] of edits.reverse()) to = to.slice(0, start) + to.slice(end);
    replacements.push({ op: 'replace', field: 'source', from, to });
  }
  let adapted = numeric.source;
  for (const action of replacements) {
    assert.equal(adapted.split(action.from).length, 2, 'one complete borrowed-string type guard owner is recorded');
    adapted = adapted.replace(action.from, () => action.to);
  }
  return { source: adapted, actions: [...numeric.actions, ...replacements] };
}

function rejectInputCopies(block, names) {
  const tokens = tokenize(block, 'Rust').tokens, matching = new Map(), stack = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'punct') continue;
    const value = tokens[index].value;
    if (['(', '[', '{'].includes(value)) stack.push(index);
    else if ([')', ']', '}'].includes(value)) {
      const opening = stack.pop();
      assert.equal(tokens[opening]?.value, ({ ')': '(', ']': '[', '}': '{' })[value]);
      matching.set(opening, index);
    }
  }
  assert.equal(stack.length, 0, 'the complete actual function has balanced delimiters');
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'identifier' || !names.has(tokens[index].value)
      || ['.', '::'].includes(tokens[index - 1]?.value)) continue;
    let left = index, right = index;
    while (tokens[left - 1]?.value === '(' && matching.get(left - 1) === right + 1) {
      const previous = tokens[left - 2];
      if (previous?.kind === 'macro' || previous?.kind === 'identifier'
        && !['if', 'while', 'return', 'match', 'break'].includes(previous.value)
        || [')', ']', '?'].includes(previous?.value)) break;
      left -= 1; right += 1;
    }
    assert.ok(!(tokens[right + 1]?.value === '.' && ['clone', 'to_vec', 'to_owned'].includes(tokens[right + 2]?.value)
      && tokens[right + 3]?.value === '(' && tokens[right + 4]?.value === ')'),
    'an immutable whole input vector is not copied in a complete source or continuation owner');
  }
}

/** Inspect every actual packaged owner and the shared recorded parameter loans. */
export function readGrammarSelectionFieldRangeLibraryInterface(files, emitted, owners) {
  const items = files.flatMap(file => readRustLibraryItems(file.source).items);
  const mappings = ownerMappings(emitted, owners), interfaces = [];
  const rawItems = emitted.definitions.flatMap(definition => readRustLibraryItems(definition).items);
  const stylePath = 'parity/self-translation/grammar-selection-field-range-rules-decorators.lino';
  const styleSource = readFileSync(new URL(stylePath, root), 'utf8');
  const styles = DecoratorSet.fromLino(styleSource).decorators;
  const recordedStyles = styles.filter(style => style.id === libraryName + '-library-style');
  assert.equal(recordedStyles.length, 1, 'the actual writer records one complete family library style');
  const recordedStyle = recordedStyles[0];
  assert.equal(recordedStyle.level, 'emitter');
  assert.equal(recordedStyle.order, 0);
  assert.deepEqual(recordedStyle.when, [['format', 'Rust']]);
  const typeOwners = owners.flatMap((owner, index) => owner.kind === 'data'
    ? [{ owner, target: mappings[index].target }] : []);
  const relocateReturnType = value => {
    const tokens = tokenize(value, 'Rust').tokens, edits = [], relocations = [];
    for (let index = 0; index + 2 < tokens.length; index += 1) {
      if (tokens[index].kind !== 'identifier' || tokens[index].value !== 'crate'
        || tokens[index + 1].value !== '::' || tokens[index + 2].kind !== 'identifier'
        || tokens[index - 1]?.value === '::' || tokens[index + 3]?.value === '::') continue;
      const target = tokens[index + 2].value;
      const mapped = typeOwners.filter(item => item.target === target);
      if (mapped.length === 0) continue;
      assert.equal(mapped.length, 1, 'a relocated local return type has one actual checked data owner');
      const original = rawItems.filter(item => item.name === target && ['enum', 'struct'].includes(item.kind));
      const packaged = items.filter(item => item.name === target && ['enum', 'struct'].includes(item.kind));
      assert.equal(original.length, 1, 'the mapped data owner has one complete raw declaration');
      assert.equal(packaged.length, 1, 'the mapped data owner has one complete packaged declaration');
      assert.equal(packaged[0].kind, original[0].kind, 'library placement retains the actual data declaration kind');
      const from = 'crate::' + target, to = 'self::' + target;
      const actions = recordedStyle.actions.flatMap((action, actionIndex) => action.op === 'replace'
        && action.field === 'source' && action.from === from && action.to === to ? [{ action, actionIndex }] : []);
      assert.equal(actions.length, 1, 'the existing writer records this exact local type owner relocation once');
      edits.push({ start: tokens[index].start, end: tokens[index + 2].end, text: to });
      relocations.push({ source: mapped[0].owner.fullName, target, kind: original[0].kind, from, to,
        styleId: recordedStyle.id, actionIndex: actions[0].actionIndex,
        rawDeclarationSha256: digest(original[0].source), packagedDeclarationSha256: digest(packaged[0].source) });
    }
    let result = value;
    for (const edit of edits.sort((left, right) => right.start - left.start)) {
      result = result.slice(0, edit.start) + edit.text + result.slice(edit.end);
    }
    return { result, relocations };
  };
  for (const [index, owner] of owners.entries()) {
    const target = mappings[index].target;
    assert.match(target, /^[A-Za-z_][A-Za-z_0-9]*$/u, 'one actual flat library owner');
    const declarations = items.filter(item => item.name === target
      && (owner.kind === 'fn' ? item.kind === 'fn' : ['enum', 'struct'].includes(item.kind)));
    assert.equal(declarations.length, 1, 'packaging retains every actual source/continuation owner ' + owner.fullName);
    if (owner.kind !== 'fn') continue;
    const block = declarations[0].source;
    const raw = readSourceTextRustBlock(emitted.text, 'fn', target);
    const header = /\bfn \w+\(([^)]*)\)\s*->\s*([^{}]+)\{/u.exec(block);
    const original = /\bfn \w+\(([^)]*)\)\s*->\s*([^{}]+)\{/u.exec(raw);
    const parameterStyles = readPortableSourceParameterStyles(raw);
    const recorded = /\bfn \w+\(([^)]*)\)\s*->\s*([^{}]+)\{/u.exec(parameterStyles.source);
    assert.ok(header && original && recorded, 'complete raw, shared-loan and packaged signatures are present');
    const before = nativeParameters(original[1]), parameters = nativeParameters(header[1]);
    assert.deepEqual(parameters, nativeParameters(recorded[1]),
      'every source and continuation interface uses exactly the existing shared loan decision');
    if (!owner.generated) {
      const immutable = before.map(parameter => parameter.replace(/^(?:mut )?(\w+):Vec<(f64|String|bool)>$/u, '$1:&[$2]')
        .replace(/^(?:mut )?(\w+):String$/u, '$1:&str'));
      assert.deepEqual(parameters, immutable, 'original primitive columns and string inputs are borrowed without changing scalar parameters');
    }
    const resultProof = relocateReturnType(original[2]);
    assert.equal(header[2].replace(/\s+/gu, ''), resultProof.result.replace(/\s+/gu, ''),
      'the complete return type changes only through authenticated recorded local data owner relocation');
    const vectors = new Set(parameters.flatMap(parameter => /^(\w+):&\[(?:f64|String|bool)\]$/u.exec(parameter)?.[1] ?? []));
    rejectInputCopies(block, vectors);
    interfaces.push({ source: owner.fullName, target, generated: owner.generated, parameters,
      immutableVectorInputs: [...vectors], result: header[2].trim(), rawResult: original[2].trim(),
      resultTypeRelocations: resultProof.relocations, signature: header[0].slice(0, -1).trim() });
  }
  assert.equal(interfaces.length, owners.filter(owner => owner.kind === 'fn').length,
    'the ABI inventory contains every original and generated function');
  assert.ok(interfaces.some(item => !item.generated && item.parameters.some(parameter => /^\w+:&\[f64\]$/u.test(parameter))),
    'the actual original numeric input column remains borrowed');
  assert.ok(interfaces.some(item => !item.generated && item.parameters.some(parameter => /^\w+:&\[String\]$/u.test(parameter))),
    'the actual original mapping column remains borrowed');
  return { ownerMappings: mappings, interfaces,
    libraryStyle: { path: stylePath, bytes: Buffer.byteLength(styleSource), sourceSha256: digest(styleSource) },
    files: files.map(file => ({
      path: file.path ?? libraryName + '.rs', bytes: Buffer.byteLength(file.source), sourceSha256: digest(file.source) })) };
}

export async function generateGrammarSelectionFieldRangeRules({ check = false, native = false } = {}) {
  const closed = readGrammarSelectionFieldRangeSource();
  const policy = readGrammarSelectionFieldRangePolicy(closed);
  for (const relative of ['js/src/translation/data/grammar-selection-field-range-rules.lino',
    'rust/src/data/grammar-selection-field-range-rules.lino']) {
    const file = new URL(relative, root);
    if (check) checkProjectData(file, policy);
    else writeProjectData(file, policy);
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
        'classify every actual declaration, including all generated continuations');
      return { ordinal, kind: item.k, name: item.name, generated: item.generated === true, span: item.span };
    });
    const checked = checkProgram(surface), owners = checkedOwners(checked, inventory);
    const emitted = emitRust(checked, { arrayConsumption: true });
    ownerMappings(emitted, owners);
    const packet = encodeTypedProgramLinks(checked), text = packet.network.toLino();
    const binary = encodeLinkNetworkBinary(packet.network), replays = [];
    for (const [format, network] of [
      ['text', LinkNetwork.fromLino(text)], ['binary', decodeLinkNetworkBinary(binary)],
    ]) {
      const restored = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), { wholeProgram: true });
      const restoredOwners = checkedOwners(restored, inventory);
      assert.deepEqual(restoredOwners, owners, 'every complete checked owner/order survives ' + format);
      const replay = emitRust(restored, { arrayConsumption: true });
      ownerMappings(replay, restoredOwners);
      assert.equal(replay.text, emitted.text, 'complete native source survives whole-program ' + format + ' recheck');
      for (const field of ['definitions', 'preludes', 'mappings', 'assumptions', 'encodings']) {
        assert.deepEqual(replay[field], emitted[field], 'complete native ' + field + ' survives ' + format);
      }
      replays.push({ format, checkedNamedSourceInventory: restoredOwners,
        emittedTextBytes: Buffer.byteLength(replay.text), emittedTextSha256: digest(replay.text) });
    }
    const files = writeRecordRuleLibrary(emitted, libraryName, { check, sourceStyles: readGrammarSelectionFieldRangeLibraryStyles });
    const library = readGrammarSelectionFieldRangeLibraryInterface(files, emitted, owners);
    artifact = { schemaVersion: 1, model: policy.model, source: closed.source,
      sourceSha256: closed.sourceSha256, sourceClosure: closed.sourceClosure,
      inventory, checkedNamedSourceInventory: owners, replays,
      archive: { root: packet.root.asU64(), textBytes: Buffer.byteLength(text), textSha256: digest(text),
        binaryBytes: binary.byteLength, binarySha256: digest(binary) },
      emitted: { ...emitted, definitions: emitted.definitions, preludes: emitted.preludes },
      emittedTextBytes: Buffer.byteLength(emitted.text), emittedTextSha256: digest(emitted.text), library,
      boundary: 'Actual complete source capture, whole-program archive rechecks, raw Rust emission and recorded library packaging. Native compilation/execution, parser-host parity and language-CST validity require their actual CI/runtime assertions.' };
    const output = new URL('parity/self-translation/grammar-selection-field-range-rules-native.lino', root);
    if (check) checkProjectData(output, artifact);
    else writeProjectData(output, artifact);
  }
  return { policy, artifact };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(new Set(args).size === args.length && args.every(argument => ['--check', '--native'].includes(argument)),
    'only complete source-generation/freshness options are admitted');
  const result = await generateGrammarSelectionFieldRangeRules({ check: args.includes('--check'), native: args.includes('--native') });
  console.log(JSON.stringify({ phase: 'complete field-range source generation', sourceBytes: result.policy.sourceBytes,
    sourceSha256: result.policy.sourceSha256, nativeSource: result.artifact === null ? null : {
      bytes: result.artifact.emittedTextBytes, sha256: result.artifact.emittedTextSha256,
      owners: result.artifact.checkedNamedSourceInventory.length,
      interfaces: result.artifact.library.interfaces }, nativeCompilation: false, nativeExecution: false }, null, 2));
}
