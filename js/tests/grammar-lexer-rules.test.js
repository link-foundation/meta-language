import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import { importAntlr } from '../src/grammar-importers/antlr.js';
import { compileGrammar } from '../src/grammar.js';
import { readLexerDeclaration } from '../src/grammar-runtime/lexer-election.js';
import {
  acceptGrammarLexerDecimal, acceptGrammarLexerIdentifier, acceptGrammarLexerMode, acceptGrammarLexerTrivia, acceptGrammarLexerExpectedToken, readGrammarLexerTriviaIdentity, readGrammarLexerCommandPlan,
  readGrammarLexerDeclarationIssue, readGrammarLexerEntryForm,
  readGrammarLexerLazyNormalization, readGrammarLexerObligationPositions,
  readGrammarLexerSectionRulePositions, readGrammarLexerLiteralRulePosition, readGrammarLexerBindingNames,
  readGrammarLexerLiteralExtension,
  readGrammarLexerOwnerComparison,
} from '../src/translation/grammar-lexer-rules.js';
import { GRAMMAR_LEXER_FORMS } from '../src/grammar-lexer-forms.js';
import { grammarLexerRulesPolicy, readGrammarLexerSourceModule, renderGrammarLexerNativeForms } from '../scripts/generate-grammar-lexer-rules.mjs';

test('portable lexer command plans preserve ordering, aliases and unsupported source', () => {
  assert.deepEqual(readGrammarLexerCommandPlan('-> pushMode(STRING), type(TEXT), type(WORD), channel(0), popMode, more', 'RAW'),
    ['WORD', 'default', 'emit', 'pushMode', 'STRING', 'popMode', '', 'unsupported', 'more']);
  assert.deepEqual(readGrammarLexerCommandPlan('-> mode(DEFAULT_MODE), channel(HIDDEN)', 'RAW'), ['RAW', 'HIDDEN', 'emit', 'setMode', 'default']);
  assert.deepEqual(readGrammarLexerCommandPlan('-> channel(00), type(7), unknown(é)', 'RAW'), ['7', 'default', 'emit', 'unsupported', 'unknown(é)']);
  assert.deepEqual(readGrammarLexerCommandPlan('-> skip, , malformed(', 'RAW'), ['SKIP', 'default', 'skip', 'unsupported', 'malformed(']);
  assert.deepEqual(readGrammarLexerCommandPlan('-> skip, channel ( 000 )', 'RAW'), ['SKIP', 'default', 'skip']);
  assert.deepEqual(readGrammarLexerCommandPlan('-> type ( WORD ), skip, channel(HIDDEN)', 'RAW'), ['SKIP', 'HIDDEN', 'skip']);
  assert.deepEqual(readGrammarLexerCommandPlan('-> skip, type(WORD)', 'RAW'), ['WORD', 'default', 'emit']);
  for (const value of ['A', '_a9', 'Z_0']) assert.equal(acceptGrammarLexerIdentifier(value), true);
  for (const value of ['', '1A', 'é', 'A B', '@', '[', '`', '{', 'A/', 'A:', '𝒜']) assert.equal(acceptGrammarLexerIdentifier(value), false);
  for (const value of ['0', '004']) assert.equal(acceptGrammarLexerDecimal(value), true);
  for (const value of ['', '-1', '1a', '/', ':', '𝟙']) assert.equal(acceptGrammarLexerDecimal(value), false);
});

test('portable lexer mode, channel, disposition and expected type decisions are independent', () => {
  assert.equal(acceptGrammarLexerMode('quoted', 'quoted'), true);
  assert.equal(acceptGrammarLexerMode('default', 'quoted'), false);
  assert.equal(acceptGrammarLexerTrivia('default', 'emit'), false);
  assert.equal(acceptGrammarLexerTrivia('default', 'skip'), true);
  assert.equal(acceptGrammarLexerTrivia('CUSTOM', 'emit'), true);
  assert.equal(acceptGrammarLexerExpectedToken('WORD', 'WORD', false), true);
  assert.equal(acceptGrammarLexerExpectedToken('WORD', 'OTHER', false), false);
  assert.equal(acceptGrammarLexerExpectedToken('', 'OTHER', true), true);
  assert.equal(readGrammarLexerTriviaIdentity('RAW', 'SKIP', 'skip'), 'RAW');
  assert.equal(readGrammarLexerTriviaIdentity('RAW', 'WORD', 'emit'), 'WORD');
});

test('portable lexer source plans preserve original section ownership and token bindings', () => {
  assert.deepEqual(readGrammarLexerSectionRulePositions(1, ['parser', 'lexer', 'lexer'], ['Parser', 'Tokens', 'Other'],
    ['Tokens', '', ''], [0, 1, 1, 1, 2], ['normal', 'token', 'silent', 'normal', 'token']), [1, 2, -1, 0, 3]);
  assert.equal(readGrammarLexerLiteralRulePosition('a', 'combined', ['silent', 'token', 'token'], ['a', 'a', 'b'], [true, true, true]), 1);
  assert.equal(readGrammarLexerLiteralRulePosition('missing', 'combined', ['token'], ['a'], [true]), -1);
  assert.equal(readGrammarLexerLiteralRulePosition('missing', 'lexer', ['token'], ['a'], [true]), -2);
  assert.equal(readGrammarLexerLiteralRulePosition('a', 'combined', ['token', 'token'], ['a', 'a'], [true, true]), -2);
  assert.deepEqual(readGrammarLexerBindingNames(['WORD', 'UNUSED', 'TEXT'], ['WORD'], ['RAW', 'DROP'], ['TEXT', ''],
    ['WORD', 'SKIP'], ['emit', 'skip']), ['WORD', 'RAW', 'TEXT', 'DROP']);
  assert.throws(() => readGrammarLexerSectionRulePositions(0, [], [], [], [0], []), /invalid required text fact|missing required text fact/u);
});

test('portable lexer literal extension retains priority, original indices and later implicit identities', () => {
  const kinds = ['lexicalMode', 'lexicalOwner', 'lexicalLiteral', 'lexicalRule', 'lexicalRule'];
  const names = ['default', 'start', '@literal:0', '@literal:0', 'A'];
  const values = ['', '', 'later', '', ''];
  const targets = ['', 'start', '', '', ''];
  const items = ['', 'ref', '', 'literal', 'literal'];
  const bodies = ['', '', '', 'later', 'a'];
  const plan = (value, literals = [value, 'later']) => readGrammarLexerLiteralExtension('start', value, kinds, names, values, targets, items, bodies, literals);
  assert.deepEqual(plan('new'), [1, 2, 0, 2, 1, 3, 1]);
  assert.deepEqual(plan('new', ['later', 'new']), [1, 4, 1]);
  assert.deepEqual(plan('a'), [2, 2, 4]);
  assert.deepEqual(plan('later'), [0, -1, -1]);
  assert.deepEqual(plan(''), [0, -1, -1]);
  assert.deepEqual(readGrammarLexerLiteralExtension('other', 'new', kinds, names, values, targets, items, bodies, []), [0, -1, -1]);
  assert.deepEqual(plan('new', ['later']), [-1]);
  assert.deepEqual(readGrammarLexerLiteralExtension('start', 'new', kinds, [], values, targets, items, bodies, ['new']), [-1]);
  assert.deepEqual(readGrammarLexerLiteralExtension('start', 'a', [...kinds, 'lexicalRule'], [...names, 'B'], [...values, ''], [...targets, ''], [...items, 'literal'], [...bodies, 'a'], ['a', 'later']), [3, 2, -1]);
});

test('portable lexer owner comparison requires bijective actual references and preserves lexical rule identities', () => {
  assert.deepEqual(readGrammarLexerOwnerComparison(['lexicalOwner', 'lexicalOwner', 'lexicalObligation', 'lexicalObligation', 'lexicalRule'],
    ['start', 'word', 'word', 'A', 'A'], ['start', 'term', '', '', '']), [0, 0, 1, 1, 2, 1]);
  assert.deepEqual(readGrammarLexerOwnerComparison(['lexicalOwner', 'lexicalOwner'], ['word', 'word'], ['term', 'other']), [-1]);
  assert.deepEqual(readGrammarLexerOwnerComparison(['lexicalOwner', 'lexicalOwner'], ['word', 'other'], ['term', 'term']), [-1]);
  assert.deepEqual(readGrammarLexerOwnerComparison(['lexicalOwner'], ['word'], ['']), [-1]);
  assert.deepEqual(readGrammarLexerOwnerComparison(['lexicalOwner', 'lexicalRule'], ['word', 'word'], ['term', '']), [-1]);
  assert.deepEqual(readGrammarLexerOwnerComparison(['lexicalOwner'], [], []), [-1]);
});

const issue = (entries, actionRows = []) => readGrammarLexerDeclarationIssue(entries.map(({ kind }) => kind), entries.map(({ name = '' }) => name),
  entries.map(({ item = '' }) => item), entries.map(({ value = '' }) => value), entries.map(({ mode = '' }) => mode),
  entries.map(({ channel = '' }) => channel), entries.map(({ disposition = 'emit' }) => disposition), actionRows.map(({ owner }) => owner), actionRows.map(({ operation }) => operation), actionRows.map(({ mode = '' }) => mode));

test('portable lexer declaration validation owns duplicates, later declarations and mode actions', () => {
  const rules = [{ kind: 'lexicalRule', name: 'WORD', mode: 'STRING', channel: 'ERROR' },
    { kind: 'lexicalMode', name: 'STRING' }, { kind: 'lexicalChannel', name: 'ERROR' }];
  assert.deepEqual(issue(rules, [{ owner: 0, operation: 'pushMode', mode: 'STRING' }]), [0, -1, -1]);
  assert.deepEqual(issue(rules.slice(0, 1)), [7, 0, -1]);
  assert.deepEqual(issue([...rules, { ...rules[0] }]), [6, 3, -1]);
  assert.deepEqual(issue([{ kind: 'lexicalOwner', name: 'start', item: 'literal' }]), [3, 0, -1]);
  assert.deepEqual(issue([{ kind: 'lexicalOwner', name: 'start', item: 'ref' }, { kind: 'lexicalOwner', name: 'start', item: 'ref' }]), [3, 1, -1]);
  assert.deepEqual(issue([{ kind: 'lexicalBinding', name: 'A', item: 'ref' }, { kind: 'lexicalBinding', name: 'A', item: 'ref' }]), [4, 1, -1]);
  assert.deepEqual(issue([{ kind: 'lexicalLiteral', value: 'a' }, { kind: 'lexicalLiteral', value: 'a' }]), [5, 1, -1]);
  assert.deepEqual(issue(rules, [{ owner: 0, operation: 'setMode', mode: 'missing' }]), [10, 0, 0]);
  assert.deepEqual(issue(rules, [{ owner: 0, operation: 'unsupported' }]), [9, 0, 0]);
  assert.deepEqual(issue([{ kind: 'lexicalRule', name: 'A', mode: 'default', channel: 'default', disposition: 'more' }]), [11, 0, -1]);
  assert.deepEqual(readGrammarLexerDeclarationIssue([], ['orphan'], [], [], [], [], [], [], [], []), [1, -1, -1]);
});

test('lexer source declarations retain unused token types and filter channel comments', () => {
  const grammar = importAntlr(`grammar Retained; channels { ERROR, /* fake CHANNEL */ SPECIAL // ignored ALSO
    } tokens { UNUSED, /* not a token */ LATER } start: A EOF; A: 'a';`);
  const entries = grammar.declarations.lexers[0].expression.items;
  assert.deepEqual(entries.filter(({ kind }) => kind === 'lexicalChannel').map(({ name }) => name), ['ERROR', 'SPECIAL']);
  assert.deepEqual(entries.filter(({ kind }) => kind === 'lexicalType').map(({ name }) => name), ['UNUSED', 'LATER']);
  const plan = readLexerDeclaration(grammar.declarations.lexers[0], (message) => { throw new Error(message); });
  assert.deepEqual([...plan.types], ['UNUSED', 'LATER']);
  assert.equal(compileGrammar(grammar).parseTree('a').ok, true);
});

test('portable lexical obligation closure keeps cycles, sharing and root action ownership', () => {
  // 0 -> 1 -> 0 and 0 -> 2 -> 1; vertex 3 is an unreachable decoy.
  const sources = [0, 1, 0, 2], targets = [1, 0, 2, 1];
  const owners = [0, 1, 1, 2, 3, 2], categories = ['action', 'predicate', 'action', 'non-greedy', 'predicate', 'command'];
  assert.deepEqual(readGrammarLexerObligationPositions(4, 0, sources, targets, owners, categories), [0, 1, 3]);
  assert.deepEqual(readGrammarLexerObligationPositions(4, 1, sources, targets, owners, categories), [1, 2, 3]);
  assert.deepEqual(readGrammarLexerObligationPositions(4, 0, [0], [9], owners, categories), [-1]);
  assert.deepEqual(readGrammarLexerObligationPositions(4, -1, sources, targets, owners, categories), [-1]);
  assert.deepEqual(readGrammarLexerObligationPositions(0, 0, [], [], [], []), [-1]);
  assert.deepEqual(readGrammarLexerObligationPositions(4, 0, [0], [], owners, categories), [-1]);
});

test('portable lazy normalization admits exact final delimiters and retains other obligations', () => {
  const kinds = ['seq', 'literal', 'capture', 'repeat0', 'any', 'literal'];
  const labels = ['', '', 'non_greedy', '', '', ''], texts = ['', '/*', '', '', '', '*/'];
  const inner = [-1, -1, 3, 4, -1, -1];
  assert.deepEqual(readGrammarLexerLazyNormalization(kinds, labels, texts, inner, [0, 0, 0], [1, 2, 5]), [2, 5]);
  assert.deepEqual(readGrammarLexerLazyNormalization(kinds, labels, texts, inner, [0, 0, 0], [2, 1, 5]), [2, -1]);
  assert.deepEqual(readGrammarLexerLazyNormalization(kinds, labels, ['', '/*', '', '', '', ''], inner, [0, 0, 0], [1, 2, 5]), [2, -1]);
  assert.deepEqual(readGrammarLexerLazyNormalization(kinds, labels, texts, [], [], []), [-1]);
});

test('lexical schema source transports every field without hand-authored native form semantics', () => {
  const policy = grammarLexerRulesPolicy();
  assert.deepEqual(policy.forms, GRAMMAR_LEXER_FORMS);
  assert.equal(policy.source, readFileSync(new URL('../src/translation/grammar-lexer-rules.js', import.meta.url), 'utf8'));
  const native = renderGrammarLexerNativeForms(policy.forms);
  for (const [head, fields] of Object.entries(policy.forms)) {
    assert.equal(native.includes(JSON.stringify(head)), true);
    for (const [name] of fields) assert.equal(native.includes(JSON.stringify(name)), true);
  }
  assert.equal(native.includes('FieldType::Block("action", false)'), true);
  assert.throws(() => renderGrammarLexerNativeForms({ forged: [['payload', 'opaque']] }), /unsupported lexical schema field/u);
  assert.equal(readGrammarLexerEntryForm('nativeExtra'), 0);
});

test('portable lexical source closure records actual fact accessors and refuses unregistered imports', () => {
  const source = readFileSync(new URL('../src/translation/grammar-lexer-rules.js', import.meta.url), 'utf8');
  const facts = readFileSync(new URL('../src/translation/fact-column-rules.js', import.meta.url), 'utf8');
  const policy = grammarLexerRulesPolicy();
  assert.equal(policy.closedSource, facts + '\n' + source.replace(policy.sourceClosure.removedImport, ''));
  assert.equal(readGrammarLexerSourceModule(source), policy.closedSource);
  assert.equal(policy.sourceClosure.dependencies[0].path, 'js/src/translation/fact-column-rules.js');
  assert.throws(() => readGrammarLexerSourceModule(source.replace(policy.sourceClosure.removedImport, '')), /exactly its recorded/u);
  assert.throws(() => readGrammarLexerSourceModule(source + '\n' + policy.sourceClosure.removedImport), /exactly its recorded/u);
  assert.throws(() => readGrammarLexerSourceModule(source + '\nimport { foreign } from "unregistered";'), /unrecorded source import/u);
  const sparse = ['lexicalRule'];
  sparse.length = 2;
  assert.throws(() => readGrammarLexerDeclarationIssue(sparse, ['', ''], ['', ''], ['', ''], ['default', 'default'], ['default', 'default'], ['emit', 'emit'], [], [], []), /missing required text fact/u);
});

test('complete portable lexical rules translate from the actual shared JavaScript module', async () => {
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const { emitJavaScript } = await import('../src/translation/emit-javascript.js');
  const source = readGrammarLexerSourceModule();
  const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const output = emitRust(checked, { arrayConsumption: true });
  for (const name of ['read_grammar_lexer_entry_form', 'read_grammar_lexer_command_plan', 'read_grammar_lexer_declaration_issue',
    'read_grammar_lexer_obligation_positions', 'read_grammar_lexer_lazy_normalization', 'read_grammar_lexer_literal_extension', 'read_grammar_lexer_owner_comparison']) assert.equal(output.text.includes(name), true);
  assert.equal(output.text.includes('native lexer callback'), false);
  assert.match(output.text, /refuse_required_text_fact/u);
  assert.match(output.text, /panic!/u);
  assert.equal(output.assumptions?.some((assumption) => /array reads are in bounds/u.test(assumption)), false);
  const javascript = emitJavaScript(checked).text;
  const result = vm.runInNewContext(javascript + '\nreadGrammarLexerCommandPlan("-> type(WORD), channel(HIDDEN)", "RAW");', {}, { timeout: 1000 });
  assert.deepEqual(Array.from(result), ['WORD', 'HIDDEN', 'emit']);
  assert.throws(() => vm.runInNewContext(javascript + '\nreadRequiredTextFact(["x"], 2);', {}, { timeout: 1000 }), /invalid required text fact column position/u);
});
