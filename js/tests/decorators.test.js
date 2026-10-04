import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  analyzeProgram,
  compileGrammar,
  DECORATOR_LEVELS,
  decorateGrammar,
  decorator,
  DecoratorError,
  DecoratorSet,
  emitGbnf,
  importGbnf,
  LinkNetwork,
  LinkQuery,
  LinkType,
  mergeGrammars,
  ParseConfiguration,
  ReplacementRule,
  translateNativeConstruct,
  TranslationRule,
  TranslationRuleSet,
} from '../src/index.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const FIXTURE = 'parity/decorators/cases.json';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const read = (name) => readFileSync(path.join(root, 'parity/decorators', name), 'utf8');
const cases = JSON.parse(read('cases.json'));
const sets = { levels: DecoratorSet.fromLino(read('levels.lino')), composition: DecoratorSet.fromLino(read('composition.lino')) };
const levels = sets.levels;
const { hooks } = cases;

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-DECORATORS-EVERY-LEVEL',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-decorators-every-level',
    fixtureFile: FIXTURE,
    assertions,
    testName,
  });
}

// The node kinds of a public syntax tree in preorder.
const kinds = (node) => (node.type === 'node' ? [node.kind, ...node.children.flatMap(kinds)] : []);
const missing = (node) => (node.type === 'missing' ? [node.kind] : node.type === 'node' ? node.children.flatMap(missing) : []);

// Runs every hook with `decorators` and returns the runtime-neutral view the
// cases file records.
function runHooks(decorators) {
  const options = { decorators };
  const imported = importGbnf(hooks.grammar, options);
  const parser = compileGrammar(imported, options);
  const plain = compileGrammar(importGbnf(hooks.grammar), options);
  const merged = mergeGrammars([
    { id: 'one', language: 'demo', grammar: importGbnf(hooks.grammar) },
    { id: 'two', language: 'demo', grammar: importGbnf(hooks.grammar) },
  ], options);
  const { from, rule, to } = hooks.conceptMapping;
  const mapping = translateNativeConstruct(from, rule, to, options);
  const recovering = compileGrammar(imported, { ...options, errorRecovery: true, recovery: 'accept' });
  const program = analyzeProgram(hooks.cstToAst.source, hooks.cstToAst.language, {}, options);
  const network = LinkNetwork.parse(hooks.transformation.source, hooks.transformation.language, ParseConfiguration.default());
  const matches = network.find(LinkQuery.fromSexpression(hooks.transformation.query));
  network.replace(matches, ReplacementRule.capturedText(hooks.transformation.capture, hooks.transformation.replacement), options);
  const shell = new LinkNetwork();
  const command = shell.insertSyntaxNode('Shell', 'command', [shell.insertSourceToken('Shell', hooks.translationRule.token)]);
  const rules = new TranslationRuleSet('shell-to-js', [
    new TranslationRule('command', new LinkQuery({ linkType: LinkType.Syntax, language: 'Shell' }).withTerm('command'))
      .withReferenceCapture('body', 0)
      .withTemplate('JavaScript', hooks.translationRule.template),
  ]);
  return {
    importer: { ruleNames: imported.ruleNames() },
    grammarRule: { source: hooks.grammarRule.source, kinds: kinds(plain.parse(hooks.grammarRule.source)) },
    mergeDecision: { bases: merged.groups[0].decisions.map(({ basis }) => basis) },
    conceptMapping: { from, rule, to, relation: mapping.relation, rules: mapping.rules },
    executor: { source: hooks.executor.source, kinds: kinds(parser.parse(hooks.executor.source)) },
    recovery: { source: hooks.recovery.source, missing: missing(recovering.parseTree(hooks.recovery.source).tree) },
    cstToAst: { ...hooks.cstToAst, terms: program.sourceMappings.map(({ term }) => term) },
    transformation: { ...hooks.transformation, text: network.reconstructText() },
    emitter: { format: 'gbnf', source: emitGbnf(imported, options).source },
    translationRule: { ...hooks.translationRule, text: rules.render('JavaScript', shell, command, options) },
  };
}

test('one decorator API extends every pipeline level, each hook applying the shared levels set', () => {
  assert.deepEqual(DECORATOR_LEVELS, [
    'importer', 'grammar-rule', 'merge-decision', 'concept-mapping', 'executor',
    'recovery', 'cst-to-ast', 'transformation', 'emitter', 'translation-rule',
  ]);
  assert.deepEqual([...new Set(levels.decorators.map(({ level }) => level))].sort(), [...DECORATOR_LEVELS].sort());
  const { grammar, ...expected } = hooks;
  void grammar;
  assert.deepEqual(runHooks(levels), expected);
  observe(['decoratorAtEveryLevel'], 'one decorator API extends every pipeline level');
});

test('decorators compose by order and then by id, each decorating what the one before produced', () => {
  for (const { set, level, record, expected } of cases.records) {
    assert.deepEqual(sets[set].decorate(level, record), expected, `${set} ${level} ${JSON.stringify(record)}`);
  }
  // The file lists the composition decorators out of order; reading sorts them.
  assert.deepEqual(sets.composition.ids(), ['first', 'second', 'tie-a', 'tie-b', 'drop-comment']);
  const reversed = new DecoratorSet([...sets.composition.decorators].reverse());
  assert.deepEqual(reversed.ids(), sets.composition.ids());
  assert.throws(() => sets.composition.add({ id: 'first', level: 'emitter', actions: [{ op: 'drop' }] }), DecoratorError);
  observe(['compositionOrderDeterministic'], 'decorators compose in a defined order');
});

test('decorators are links data: the canonical Links Notation reads back to the same set', () => {
  assert.equal(levels.toLino(), read('levels.lino'));
  assert.equal(DecoratorSet.fromLino(sets.composition.toLino()).toLino(), sets.composition.toLino());
  const odd = new DecoratorSet([decorator({
    id: 'odd-text', level: 'emitter', when: [['line', 'a (b) "c" \'d\'']],
    actions: [{ op: 'replace', field: 'line', from: '%', to: ' ( ) \n' }, { op: 'set', field: 'note', value: '' }],
  })]);
  assert.deepEqual(DecoratorSet.fromLino(odd.toLino()), odd);
  assert.throws(() => DecoratorSet.fromLino('(decorator x (level nowhere) (drop))'), DecoratorError);
  assert.throws(() => DecoratorSet.fromLino('(decorator x (level emitter))'), DecoratorError);
  assert.throws(() => DecoratorSet.fromLino('(decorator x (level emitter) (set line %zz))'), DecoratorError);
  observe(['storedAsLinksData'], 'decorators are stored as links data');
});

test('removing a decorator gives exactly the output of the set without it', () => {
  const { set, remove, level, record, expected } = cases.removal;
  assert.deepEqual(sets[set].remove(remove).decorate(level, record), expected);
  assert.throws(() => sets[set].remove('absent'), DecoratorError);
  // Removing every decorator restores the undecorated pipeline.
  let empty = levels;
  for (const id of levels.ids()) empty = empty.remove(id);
  assert.deepEqual(runHooks(empty), runHooks(undefined));
  // Removing one level's decorator changes that hook alone.
  const withoutEmitter = runHooks(levels.remove('spaced-definition'));
  const all = runHooks(levels);
  for (const key of Object.keys(all)) {
    if (key === 'emitter') assert.notDeepEqual(withoutEmitter[key], all[key]);
    else assert.deepEqual(withoutEmitter[key], all[key], key);
  }
  observe(['removable'], 'decorators can be removed');
});

test('a hook rejects a decoration that would lose input', () => {
  const dropToken = new DecoratorSet([{ id: 'drop-token', level: 'executor', when: [['type', 'token']], actions: [{ op: 'drop' }] }]);
  assert.throws(() => compileGrammar(importGbnf(hooks.grammar), { decorators: dropToken }).parse('ab'), DecoratorError);
  const dropNode = new DecoratorSet([{ id: 'unwrap', level: 'executor', when: [['kind', 'item']], actions: [{ op: 'drop' }] }]);
  assert.deepEqual(kinds(compileGrammar(importGbnf(hooks.grammar), { decorators: dropNode }).parse('ab')), ['root']);
  const dropRule = new DecoratorSet([{ id: 'drop-rule', level: 'grammar-rule', when: [['name', 'root']], actions: [{ op: 'drop' }] }]);
  assert.deepEqual(decorateGrammar(importGbnf(hooks.grammar), dropRule).ruleNames(), ['item']);
  const concept = new DecoratorSet([{ id: 'concept', level: 'grammar-rule', when: [['name', 'item']], actions: [{ op: 'set', field: 'concept', value: 'grammar.letter' }] }]);
  assert.equal(decorateGrammar(importGbnf(hooks.grammar), concept).rules.get('item').concept, 'grammar.letter');
  assert.throws(() => decorator({ id: 'bad id', level: 'emitter', actions: [{ op: 'drop' }] }), DecoratorError);
  assert.throws(() => decorator({ id: 'empty', level: 'emitter', actions: [{ op: 'replace', field: 'line', from: '', to: 'x' }] }), DecoratorError);
});
