// Complete finite-source certificate and a data-selected value-state lowering.
// No production frontend, checker, emitter or default refusal is changed.
import { readFileSync } from 'node:fs';
import { isDeepStrictEqual } from 'node:util';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { DecoratorSet } from '../../js/src/decorators.js';
import { tokenize } from '../../js/src/translation/lexer.js';
import { parseJavaScript } from '../../js/src/translation/javascript.js';
import { checkProgram } from '../../js/src/translation/check.js';
import { encodeGeneratedOwnedRust, decodeGeneratedOwnedRust } from '../generated-owned-rust/codec.mjs';
import { acceptFreshNumberArrayCollectorPattern } from './rules.mjs';

export const FRESH_NUMBER_ARRAY_COLLECTOR_POLICY = readProjectData(new URL('./data/policy.lino', import.meta.url));
const policy = FRESH_NUMBER_ARRAY_COLLECTOR_POLICY;
const decoratorSource = readFileSync(new URL('./data/lowering.lino', import.meta.url), 'utf8');
const decorators = DecoratorSet.fromLino(decoratorSource);
const fail = message => { throw new TypeError('Fresh Number array collector: ' + message); };
const equal = (first, second, message) => { if (!isDeepStrictEqual(first, second)) fail(message); };
const fixed = new Map(policy.fixedTokens.map((value, index) => [value, index + 1]));
const codeOf = value => typeof value === 'number' ? value : fixed.get(value) ?? -99;
const patterns = policy.arguments.map(argument => [...policy.prefix, ...argument.syntax, ...policy.suffix].map(codeOf));

function checkContainers(root) {
  const seen = new WeakSet(), active = new WeakSet();
  let count = 0;
  function visit(value, depth = 0) {
    if (!value || typeof value !== 'object') {
      if (!['string', 'number', 'boolean', 'undefined'].includes(typeof value) && value !== null) fail('an unrepresented scalar or executable value was supplied');
      return;
    }
    if (active.has(value)) fail('a source certificate cannot contain a structural cycle');
    if (seen.has(value)) return;
    if (++count > 100000 || depth > 256) fail('the owned certificate exceeds its structural bound');
    active.add(value);
    const entries = Reflect.ownKeys(value);
    if (entries.some(key => typeof key !== 'string')) fail('symbol properties need a separate interpretation');
    if (value instanceof Map) {
      if (Object.getPrototypeOf(value) !== Map.prototype || entries.length) fail('a declaration map has a foreign prototype or extra fields');
      for (const [key, child] of Map.prototype.entries.call(value)) { visit(key, depth + 1); visit(child, depth + 1); }
    } else {
      if (Object.getPrototypeOf(value) !== (Array.isArray(value) ? Array.prototype : Object.prototype)) fail('a record or array has a foreign prototype');
      if (Array.isArray(value) && (entries.length !== value.length + 1 || entries.slice(0, -1).some((key, index) => key !== String(index)))) fail('an ordered array has holes or extra fields');
      for (const key of entries) {
        const descriptor = Object.getOwnPropertyDescriptor(value, key);
        if (!Object.hasOwn(descriptor, 'value') || !descriptor.enumerable && !(Array.isArray(value) && key === 'length')) fail('an owned field is an accessor or hidden property');
        if (key !== 'length' || !Array.isArray(value)) visit(descriptor.value, depth + 1);
      }
    }
    active.delete(value); seen.add(value);
  }
  visit(root);
}

function sourceFacts(source) {
  if (typeof source !== 'string' || !source.length || source.length > policy.maximumSourceUnits) fail('the complete original source exceeds its selected bound');
  const lexical = tokenize(source, 'JavaScript');
  if (lexical.tokens.length > policy.maximumTokens) fail('the source has too many tokens');
  const names = new Map();
  const tokens = lexical.tokens.map(token => ({ kind: token.kind, value: token.value,
    raw: source.slice(token.start, token.end), span: { start: token.start, end: token.end } }));
  const codes = tokens.map(token => {
    if (token.kind === 'eof') return fixed.get('eof');
    if (fixed.has(token.raw) && token.raw === token.value) return fixed.get(token.raw);
    if (token.kind !== 'identifier' || !/^[A-Za-z][A-Za-z0-9_]*$/u.test(token.raw) || token.raw !== token.value) return -99;
    if (!names.has(token.value)) names.set(token.value, policy.nameCodeMinimum + names.size);
    return names.get(token.value);
  });
  const selection = patterns.findIndex(pattern => acceptFreshNumberArrayCollectorPattern(codes, pattern));
  if (selection < 0) fail('the complete source is outside the closed fresh, unaliased, one-argument, ignored-result grammar');
  const pattern = patterns[selection];
  const bindings = policy.roles.map((role, index) => {
    const token = tokens[policy.declarations[role]];
    return { role, name: token.value, span: token.span };
  });
  tokens.forEach((token, index) => { if (pattern[index] < 0) token.binding = bindings[-pattern[index] - 1]; });
  let end = 0;
  const gaps = tokens.map(token => {
    const text = source.slice(end, token.span.start);
    const gap = { start: end, end: token.span.start, units: Array.from({ length: text.length }, (_, index) => text.charCodeAt(index)) };
    end = token.span.end;
    return gap;
  });
  const d = policy.declarations, argumentLength = policy.arguments[selection].syntax.length;
  const argumentEnd = d.argumentStart + argumentLength;
  const declarationSpan = { start: tokens[d.collectorKeyword].span.start, end: tokens[d.collectorEnd].span.end };
  const pushSpan = { start: tokens[d.pushStart].span.start, end: tokens[argumentEnd + 1].span.end };
  const argumentSpan = { start: tokens[d.argumentStart].span.start, end: tokens[argumentEnd - 1].span.end };
  const selected = decorators.decorate('transformation', { model: policy.model, role: policy.lowering.role });
  if (selected.bindingForm !== 'let' || selected.operation !== 'append value state' || selected.argumentEvaluation !== 'once before append') fail('the recorded lowering operation is not supported');
  const collector = bindings[2].name, declaration = source.slice(declarationSpan.start, declarationSpan.end);
  const argument = source.slice(argumentSpan.start, argumentSpan.end);
  const actions = [
    { role: 'collector declaration', span: declarationSpan, from: declaration, to: selected.bindingForm + declaration.slice('const'.length) },
    { role: 'ignored one-argument push', span: pushSpan, from: source.slice(pushSpan.start, pushSpan.end),
      to: policy.lowering.appendTemplate.replaceAll('$collector', collector).replace('$argument', argument) },
  ];
  for (const action of actions) if (source.split(action.from).length !== 2) fail('a selected source fragment is not unique at its actual owned interval');
  const lowered = new DecoratorSet([{ id: 'instantiate-certified-collector-state', level: 'transformation',
    when: [['model', policy.model]], actions: actions.map(action => ({ op: 'replace', field: 'source', from: action.from, to: action.to })) }])
    .decorate('transformation', { model: policy.model, source }).source;
  return { tokens, gaps, bindings, selection, declarationSpan, argumentSpan, pushSpan, actions, lowered };
}

function compile(source) {
  const facts = sourceFacts(source);
  // Both complete modules use the real frontend. The source's default push
  // still has its reference interpretation and remains refused by the checker.
  const original = parseJavaScript(source);
  const originalEntry = original.items.find(entry => entry.k === 'fn' && entry.name === facts.bindings[0].name);
  const numberArray = type => type?.kind === 'array' && type.element.kind === 'float';
  if (!originalEntry || originalEntry.params.length !== 1 || !numberArray(originalEntry.params[0].type) || !numberArray(originalEntry.ret)) fail('the original source interface is not a primitive Number array collector');
  const surface = parseJavaScript(facts.lowered);
  const program = checkProgram(surface);
  if (program.referenceSemantics || program.imports?.length || program.externals?.length || program.main?.effects?.length) fail('another reference, import or effect interpretation needs its own composition proof');
  const declaration = program.declarations.get(facts.bindings[0].name);
  if (!declaration || !numberArray(declaration.ret) || declaration.params.length !== 1 || !numberArray(declaration.params[0].type) || declaration.body.k !== 'let' || declaration.body.name !== facts.bindings[2].name) fail('the value-state lowering lost its checked collector or source interface');
  const loops = [...program.declarations.values()].filter(entry => entry !== declaration);
  if (loops.length !== 1 || loops[0].k !== 'fn' || !loops[0].generated) fail('the counted source loop lost its actual generated state declaration');
  return { facts, program, declaration, loop: loops[0] };
}

function certificate(facts, program) {
  const declaration = program.declarations.get(facts.bindings[0].name);
  const loop = [...program.declarations.values()].find(entry => entry !== declaration);
  return { model: policy.model, language: policy.language, interpretation: policy.interpretation,
    tokens: facts.tokens, gaps: facts.gaps, bindings: facts.bindings,
    argumentRule: policy.arguments[facts.selection].id, lowering: facts.actions,
    declaration, parameter: declaration.params[0], collector: declaration.body, loop,
    assumptions: policy.assumptions.map((assumption, index) => ({ ...assumption,
      owner: [declaration, declaration.params[0], declaration.body, loop][index] })) };
}

function originalSource(proof) {
  if (!Array.isArray(proof.tokens) || !Array.isArray(proof.gaps) || proof.tokens.length !== proof.gaps.length || proof.tokens.length > policy.maximumTokens) fail('the complete token/trivia inventory is malformed');
  let source = '', end = 0;
  for (const [index, token] of proof.tokens.entries()) {
    const gap = proof.gaps[index];
    if (!gap || !Array.isArray(gap.units) || gap.start !== end || gap.end !== token.span?.start || gap.units.length !== gap.end - gap.start || gap.units.some(unit => !Number.isSafeInteger(unit) || unit < 0 || unit > 65535)) fail('the complete original trivia has an invalid interval or code unit');
    if (typeof token.raw !== 'string' || token.raw.length !== token.span.end - token.span.start) fail('an original token lost its exact raw extent');
    source += String.fromCharCode(...gap.units) + token.raw;
    end = token.span.end;
    if (source.length > policy.maximumSourceUnits) fail('the source exceeds its bound');
  }
  if (source.length !== end || proof.tokens.at(-1)?.kind !== 'eof') fail('the source is not consumed completely');
  return source;
}

export function createFreshNumberArrayCollector(source, { model = '' } = {}) {
  if (model !== policy.model) fail('select the exact experimental source model explicitly');
  const { facts, program } = compile(source);
  const proof = certificate(facts, program);
  checkFreshNumberArrayCollector(program, proof);
  return { program, proof };
}

export function checkFreshNumberArrayCollector(program, proof) {
  checkContainers({ program, proof });
  if (proof?.model !== policy.model) fail('unknown selected source interpretation');
  const source = originalSource(proof);
  const rebuilt = compile(source);
  equal(program, rebuilt.program, 'the checked value program differs from its complete certified source lowering');
  const expected = certificate(rebuilt.facts, program);
  equal(proof, expected, 'the source certificate contains changed, missing or unrepresented fields');
  for (const field of ['declaration', 'parameter', 'collector', 'loop']) if (proof[field] !== expected[field]) fail('a source role points at an equal copy rather than its actual checked owner');
  for (const [index, token] of proof.tokens.entries()) {
    const expectedBinding = rebuilt.facts.tokens[index].binding;
    if (expectedBinding && token.binding !== proof.bindings[policy.roles.indexOf(expectedBinding.role)]) fail('a source use lost its actual lexical binder identity');
  }
  proof.assumptions.forEach((assumption, index) => { if (assumption.owner !== expected.assumptions[index].owner) fail('a premise lost its actual operation owner'); });
  return { source, loweredSource: rebuilt.facts.lowered };
}

export function encodeFreshNumberArrayCollector(program, proof) {
  checkFreshNumberArrayCollector(program, proof);
  return encodeGeneratedOwnedRust(program, proof);
}

export function decodeFreshNumberArrayCollector(network, root) {
  const result = decodeGeneratedOwnedRust(network, root);
  checkFreshNumberArrayCollector(result.program, result.proof);
  return result;
}
