// Requirement I195-GRAMMAR-FEATURE-UNION: the native grammar representation
// and its executor cover every feature of docs/vision.md#grammar-feature-union.
// parity/fixtures/grammar-feature-union.json holds, per feature, a native
// grammar listing, positive inputs with their expected concrete syntax trees,
// negative inputs with their expected rejections and a grammar mutation that
// changes the outcome. Everything here runs through the public package API;
// docs/grammar/feature-union.md is the specification.
import assert from 'node:assert/strict';
import { readdirSync, readFileSync } from 'node:fs';
import test from 'node:test';

import {
  checkGrammarLowering,
  compileGrammar,
  deserializeGrammar,
  GRAMMAR_LOWERING_FORMATS,
  GrammarParseError,
  GrammarRuntimeError,
  lowerGrammar,
  mergeGrammars,
  parseGrammarLinks,
  parseNativeGrammar,
  renderGrammarLinks,
  renderNativeGrammar,
  renderSyntaxTree,
  serializeGrammar,
} from '../src/index.js';
import { OPERATION_FORMS } from '../src/grammar-feature-forms.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const repositoryRoot = new URL('../../', import.meta.url);
const fixture = JSON.parse(readFileSync(new URL('parity/fixtures/grammar-feature-union.json', repositoryRoot), 'utf8'));
const vision = readFileSync(new URL('docs/vision.md', repositoryRoot), 'utf8');

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-FEATURE-UNION',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-feature-union',
    fixtureFile: 'parity/fixtures/grammar-feature-union.json',
    assertions,
    testName,
  });
}

const languages = new Map(Object.entries(fixture.languages).map(([name, listing]) => [name, parseNativeGrammar(listing)]));
const resolveGrammar = (name) => languages.get(name);

// The bullet list of docs/vision.md#grammar-feature-union, without its punctuation.
function visionFeatureTitles() {
  const section = vision.slice(vision.indexOf('## Grammar feature union'));
  const list = section.slice(section.indexOf('\n- '), section.indexOf('\n\n', section.indexOf('\n- ')));
  return list.split('\n').filter((line) => line.startsWith('- ')).map((line) => line.slice(2).replace(/[;.]$/u, ''));
}

function inputOf(item) {
  if (item.input !== undefined) return item.input;
  return Uint8Array.from(item.inputHex.match(/../gu).map((pair) => Number.parseInt(pair, 16)));
}

// The outcome of one parse in the fixture's shape: the rendered tree, the
// reported ambiguities and the rejection, each only when present.
function outcome(parser, item) {
  const result = parser.parseTree(inputOf(item), item.options ?? {});
  const summary = {};
  if (result.tree) summary.tree = renderSyntaxTree(result.tree);
  if (result.ambiguities.length > 0) summary.ambiguities = result.ambiguities;
  if (result.rejection) summary.rejection = result.rejection;
  assert.equal(result.ok, result.rejection === null);
  return summary;
}

function expected(item) {
  const { tree, ambiguities, rejection } = item;
  return Object.fromEntries(Object.entries({ tree, ambiguities, rejection }).filter(([, value]) => value !== undefined));
}

function compile(grammar, feature) {
  return compileGrammar(grammar, { resolveGrammar, ...(feature.options ?? {}) });
}

function loadFailure(listing, feature) {
  try {
    compile(parseNativeGrammar(listing), feature);
  } catch (error) {
    if (error instanceof GrammarRuntimeError) return { reason: error.reason };
    throw error;
  }
  return null;
}

// What a grammar uses, as names: expression kinds (with the choice order and
// class item kinds), rule kinds and fields, declarations, operations, and
// left recursion.
function formsOf(grammar) {
  const forms = new Set();
  const visitOperation = (operation) => {
    forms.add(`operation:${operation.operation}`);
    for (const value of Object.values(operation)) {
      for (const item of Array.isArray(value) ? value : [value]) {
        if (item && typeof item === 'object' && typeof item.operation === 'string') visitOperation(item);
        else if (item && typeof item === 'object' && typeof item.kind === 'string') visitExpression(item);
      }
    }
  };
  const visitExpression = (expression) => {
    forms.add(expression.kind === 'choice' ? `choice:${expression.ordered ? 'ordered' : 'unordered'}` : expression.kind);
    if (expression.kind === 'ref' && expression.arguments) forms.add('ref:arguments');
    for (const item of expression.items ?? []) {
      if (expression.kind === 'charClass' || expression.kind === 'byteClass') forms.add(`${expression.kind}:${item.kind}`);
      else visitExpression(item);
    }
    for (const key of ['item', 'synchronize']) if (expression[key]) visitExpression(expression[key]);
    for (const argument of expression.arguments ?? []) visitExpression(argument);
    if (expression.condition) visitOperation(expression.condition);
  };
  const normalized = grammar.normalized();
  for (const rule of normalized.rules) {
    forms.add(`rule:${rule.kind}`);
    for (const field of ['parameters', 'channel', 'modes', 'action']) if (rule[field] !== undefined) forms.add(`rule:${field}`);
    visitExpression(rule.expression);
    for (const operation of rule.action ?? []) visitOperation(operation);
  }
  const declarations = normalized.declarations ?? {};
  for (const [key, value] of Object.entries(declarations)) forms.add(`declaration:${key}`);
  for (const extra of declarations.extras ?? []) visitExpression(extra);
  for (const macro of declarations.macros ?? []) visitExpression(macro.expression);
  for (const scanner of declarations.scanners ?? []) for (const operation of scanner.operations) visitOperation(operation);
  if (leftRecursive(normalized.rules)) forms.add('left-recursion');
  return forms;
}

// Whether some rule can reach itself before consuming input, through the
// first item of sequences and every alternative of choices.
function leftRecursive(rules) {
  const byName = new Map(rules.map((rule) => [rule.name, rule]));
  const leftmost = (expression, found) => {
    if (expression.kind === 'ref') found.add(expression.name);
    else if (expression.kind === 'seq' && expression.items.length > 0) leftmost(expression.items[0], found);
    else if (expression.kind === 'choice' || expression.kind === 'longest') for (const item of expression.items) leftmost(item, found);
    else if (expression.item) leftmost(expression.item, found);
    return found;
  };
  return rules.some((rule) => {
    const seen = new Set();
    const pending = [...leftmost(rule.expression, new Set())];
    while (pending.length > 0) {
      const name = pending.pop();
      if (name === rule.name) return true;
      if (seen.has(name) || !byName.has(name)) continue;
      seen.add(name);
      pending.push(...leftmost(byName.get(name).expression, new Set()));
    }
    return false;
  });
}

// The forms each feature's grammar must use, so a fixture grammar cannot
// pass a feature without representing it.
const FEATURE_FORMS = {
  alternatives: ['choice:ordered', 'choice:unordered'],
  recursion: ['left-recursion'],
  precedence: ['precedence'],
  ambiguity: ['declaration:conflicts', 'dynamicPrecedence'],
  lexical: ['longest', 'lexicalPrecedence'],
  unicode: ['charClass:category', 'charClass:script', 'byteClass', 'byteClass:byteRange'],
  trivia: ['declaration:extras', 'token', 'immediateToken'],
  modes: ['declaration:modes', 'rule:modes', 'rule:channel', 'operation:pushMode', 'operation:popMode'],
  layout: ['declaration:scanners', 'operation:emit', 'operation:push', 'operation:pop', 'predicate'],
  predicates: ['predicate', 'rule:action', 'operation:fieldText'],
  actions: ['rule:action', 'operation:setAttribute', 'operation:buildNode', 'operation:sumOf'],
  fields: ['capture', 'alias'],
  parameterization: ['rule:parameters', 'parameter', 'ref:arguments'],
  imports: ['declaration:imports'],
  macros: ['declaration:macros', 'expand'],
  embedded: ['embed'],
  recovery: ['recover', 'missing'],
};

// Every value of a scanner or action is plain data: no function, no class
// instance, nothing JSON cannot carry.
function assertPlainData(value, where) {
  if (value === null || ['string', 'number', 'boolean'].includes(typeof value)) return;
  assert.ok(typeof value === 'object', `${where} holds a ${typeof value}`);
  if (Array.isArray(value)) {
    value.forEach((item, index) => assertPlainData(item, `${where}[${index}]`));
    return;
  }
  assert.equal(Object.getPrototypeOf(value), Object.prototype, `${where} is not a plain object`);
  for (const [key, item] of Object.entries(value)) assertPlainData(item, `${where}.${key}`);
}

test('every feature of the grammar feature union has a native representation that both serializations carry', (context) => {
  assert.deepEqual(fixture.features.map(({ title }) => title), visionFeatureTitles());
  assert.deepEqual(fixture.features.map(({ id }) => id), Object.keys(FEATURE_FORMS));
  for (const feature of fixture.features) {
    const grammar = parseNativeGrammar(feature.listing);
    const forms = formsOf(grammar);
    for (const form of FEATURE_FORMS[feature.id]) assert.ok(forms.has(form), `${feature.id} uses ${form}`);
    assert.equal(renderNativeGrammar(grammar), feature.listing, `${feature.id}: the listing is its own rendering`);
    const links = renderGrammarLinks(grammar);
    const fromLinks = parseGrammarLinks(links);
    assert.deepEqual(fromLinks.normalized(), grammar.normalized(), `${feature.id}: the links form carries the grammar`);
    assert.equal(renderGrammarLinks(fromLinks), links);
    assert.equal(renderNativeGrammar(fromLinks), feature.listing);
    assert.deepEqual(deserializeGrammar(serializeGrammar(grammar)).normalized(), grammar.normalized());
  }
  observe(['everyUnionFeatureRepresented'], context.name);
});

test('every feature executes its positive cases into the expected concrete syntax trees, and its mutation changes the result', (context) => {
  for (const feature of fixture.features) {
    assert.ok(feature.positive.length > 0, `${feature.id} has positive cases`);
    const grammar = parseNativeGrammar(feature.listing);
    const parsers = [compile(grammar, feature), compile(parseGrammarLinks(renderGrammarLinks(grammar)), feature)];
    for (const item of feature.positive) {
      for (const parser of parsers) {
        const result = parser.parseTree(inputOf(item), item.options ?? {});
        assert.ok(result.ok, `${feature.id} accepts ${item.input ?? item.inputHex}`);
        assert.deepEqual(outcome(parser, item), expected(item), `${feature.id}: ${item.input ?? item.inputHex}`);
        assert.equal(renderSyntaxTree(parser.parse(inputOf(item), item.options ?? {})), item.tree);
      }
      // The tree is lossless: its leaves are the input, byte for byte.
      const tree = parsers[0].parse(inputOf(item), item.options ?? {});
      assert.equal(tree.start, 0);
      assert.equal(tree.end, typeof inputOf(item) === 'string' ? Buffer.byteLength(inputOf(item)) : inputOf(item).length);
    }
    const { mutation } = feature;
    assert.ok(feature.listing.includes(mutation.from), `${feature.id}: the mutation applies`);
    const mutated = feature.listing.replace(mutation.from, mutation.to);
    assert.notDeepEqual(mutation.after, mutation.before, `${feature.id}: the mutation changes the result`);
    assert.deepEqual(outcome(compile(grammar, feature), mutation), mutation.before);
    const after = mutation.after.loadError
      ? { loadError: loadFailure(mutated, feature) }
      : outcome(compile(parseNativeGrammar(mutated), feature), mutation);
    assert.deepEqual(after, mutation.after, `${feature.id}: the mutated grammar`);
  }
  observe(['everyUnionFeatureExecuted'], context.name);
});

test('every feature rejects its negative cases with the expected reason and position', (context) => {
  for (const feature of fixture.features) {
    assert.ok(feature.negative.length > 0, `${feature.id} has negative cases`);
    const parser = compile(parseNativeGrammar(feature.listing), feature);
    for (const item of feature.negative) {
      if (item.listing !== undefined) {
        assert.deepEqual(loadFailure(item.listing, feature), item.loadError, `${feature.id}: ${item.listing}`);
        continue;
      }
      const result = parser.parseTree(inputOf(item), item.options ?? {});
      assert.equal(result.ok, false, `${feature.id} rejects ${item.input ?? item.inputHex}`);
      assert.deepEqual(outcome(parser, item), expected(item), `${feature.id}: ${item.input ?? item.inputHex}`);
      assert.throws(() => parser.parse(inputOf(item), item.options ?? {}), (error) => {
        assert.ok(error instanceof GrammarParseError);
        assert.equal(error.reason, item.rejection.reason);
        return true;
      });
    }
  }
  // The step budget bounds a parse as the depth limit does.
  const recursion = fixture.features.find(({ id }) => id === 'recursion');
  const limited = compile(parseNativeGrammar(recursion.listing), { options: { stepLimit: 5 } }).parseTree('e:1-2-3');
  assert.deepEqual(limited.rejection, { reason: 'stepLimit', limit: 5 });
  observe(['negativeCasesRejected'], context.name);
});

test('external scanners and semantic actions are executable link definitions', (context) => {
  const byId = new Map(fixture.features.map((feature) => [feature.id, feature]));
  for (const id of ['layout', 'modes', 'predicates', 'actions']) {
    const grammar = parseNativeGrammar(byId.get(id).listing);
    const normalized = grammar.normalized();
    const operations = [
      ...(normalized.declarations?.scanners ?? []).flatMap((scanner) => scanner.operations),
      ...normalized.rules.flatMap((rule) => rule.action ?? []),
    ];
    assert.ok(operations.length > 0, `${id} has scanner or action operations`);
    assertPlainData(normalized, id);
    for (const operation of operations) assert.ok(Object.hasOwn(OPERATION_FORMS, operation.operation));
  }

  // The layout scanner and the sum action are links of the links form, and
  // the grammar read back from those links runs them.
  const layout = byId.get('layout');
  const layoutLinks = renderGrammarLinks(parseNativeGrammar(layout.listing));
  assert.match(layoutLinks, /^\(scanner layout \(tokens newline indent dedent\) \(operations \(if \(valid newline\)/mu);
  const actions = byId.get('actions');
  const actionLinks = renderGrammarLinks(parseNativeGrammar(actions.listing));
  assert.match(actionLinks, /\(action \(setAttribute value \(sumOf terms value\)\)/u);
  const run = (links, feature, input) => outcome(compile(parseGrammarLinks(links), feature), { input });
  assert.deepEqual(run(layoutLinks, layout, layout.positive[0].input), expected(layout.positive[0]));
  assert.deepEqual(run(actionLinks, actions, actions.positive[0].input), expected(actions.positive[0]));

  // Changing one operation link changes what the parser does: a scanner that
  // never emits indent cannot open a block, and a lower action limit fails
  // the sum.
  const [nested] = layout.positive;
  assert.match(nested.tree, /\(indent ""\)/u);
  const withoutIndent = layoutLinks.replace('(push indents (variable pending)) (emit indent)', '(push indents (variable pending)) fail');
  assert.notEqual(withoutIndent, layoutLinks);
  assert.equal(run(withoutIndent, layout, nested.input).rejection?.reason, 'syntax');
  const [summed] = actions.positive;
  assert.match(summed.tree, /^\(sum \{value=6\}/u);
  const lowerLimit = actionLinks.replace('(integer 100)', '(integer 2)');
  assert.notEqual(lowerLimit, actionLinks);
  assert.equal(run(lowerLimit, actions, summed.input).rejection?.reason, 'syntax');

  // An operation outside its context, or one naming a token the scanner does
  // not declare, is rejected when the grammar loads.
  const outOfContext = 'start s\nrule s = normal literal("a") action(emit(s))\n';
  assert.deepEqual(loadFailure(outOfContext, {}), { reason: 'operation' });
  const undeclared = layout.listing.replace('emit(dedent)', 'emit(outdent)');
  assert.deepEqual(loadFailure(undeclared, layout), { reason: 'operation' });

  // The executor interprets those links; it compiles no host code.
  const runtime = new URL('js/src/grammar-runtime/', repositoryRoot);
  for (const file of readdirSync(runtime)) {
    const source = readFileSync(new URL(file, runtime), 'utf8');
    assert.doesNotMatch(source, /\beval\s*\(|new Function\s*\(|import\s*\(/u, file);
  }
  observe(['scannersAndActionsExecutableAsLinks'], context.name);
});

test('merging and lowering keep the declarations and rule fields of the feature union', (context) => {
  const { merge, lowering } = fixture.interchange;
  for (const item of merge) {
    const result = mergeGrammars(item.sources.map(({ id, precedence, listing }) => ({
      id, language: 'quoted', precedence, grammar: parseNativeGrammar(listing),
    })));
    assert.equal(result.groups.length, 1, item.id);
    const [group] = result.groups;
    assert.equal(renderNativeGrammar(group.grammar), item.merged, `${item.id}: merged grammar`);
    assert.deepEqual(group.decisions, item.decisions, `${item.id}: decisions`);
    assert.deepEqual(group.alternatives, item.alternatives, `${item.id}: alternatives`);
  }

  assert.deepEqual(lowering.formats, GRAMMAR_LOWERING_FORMATS);
  const grammar = parseNativeGrammar(lowering.listing);
  const unhonored = (line) => /^\((declarations|attributes) /u.test(line);
  const withoutSteps = (metadata) => metadata.split('\n').filter((line) => !unhonored(line)).join('\n');
  for (const format of lowering.formats) {
    const lowered = lowerGrammar(grammar, format);
    assert.equal(lowered.status, lowering.status, `${format}: status`);
    assert.deepEqual(lowered.metadata.split('\n').filter(unhonored), lowering.steps, `${format}: steps`);
    assert.deepEqual(checkGrammarLowering(grammar, format).failures, [], `${format}: reconstructs`);
    // Negative control: without the steps the declarations and fields are lost.
    const dropped = checkGrammarLowering(grammar, format, { editMetadata: withoutSteps }).failures;
    assert.deepEqual(dropped.map(({ kind, detail }) => `${kind}: ${detail}`), lowering.dropped, `${format}: dropped`);
  }
  observe(['everyUnionFeatureRepresented', 'negativeCasesRejected'], context.name);
});

test('no JavaScript source module imports peggy, which is only a development dependency', () => {
  const sources = new URL('js/src/', repositoryRoot);
  const pending = [sources];
  while (pending.length > 0) {
    const directory = pending.pop();
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const location = new URL(entry.name + (entry.isDirectory() ? '/' : ''), directory);
      if (entry.isDirectory()) pending.push(location);
      else if (/\.[cm]?js$/u.test(entry.name)) {
        assert.doesNotMatch(readFileSync(location, 'utf8'), /from\s+['"]peggy['"]|require\(\s*['"]peggy['"]\s*\)/u, location.pathname);
      }
    }
  }
  const manifest = JSON.parse(readFileSync(new URL('js/package.json', repositoryRoot), 'utf8'));
  assert.equal(manifest.dependencies.peggy, undefined);
  assert.equal(manifest.devDependencies.peggy, '5.1.0');
});
