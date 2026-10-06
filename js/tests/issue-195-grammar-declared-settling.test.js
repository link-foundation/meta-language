// Declared settling: how two parses of one text that end alike are settled
// is grammar data, `(settling STEP...)`, which the executor reads for every
// grammar, whatever format it was imported from.
import assert from 'node:assert/strict';
import { readdirSync, readFileSync } from 'node:fs';
import test from 'node:test';

import {
  compileGrammar,
  Grammar,
  importAntlr,
  importPest,
  parseGrammarLinks,
  parseNativeGrammar,
  renderGrammarLinks,
  renderNativeGrammar,
  renderSyntaxTree,
} from '../src/index.js';
import { DEFAULT_SETTLING, SETTLING_STEPS } from '../src/grammar-feature-forms.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const repositoryRoot = new URL('../../', import.meta.url);

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-GRAMMAR-DECLARED-SETTLING',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-grammar-declared-settling',
    fixtureFile: 'docs/grammar/feature-union.md',
    assertions,
    testName,
  });
}

// The tree and the ambiguities of `input` under `grammar`.
function settle(grammar, input) {
  const result = compileGrammar(grammar).parseTree(input);
  return [result.tree ? renderSyntaxTree(result.tree) : null, result.ambiguities.length];
}

const withSettling = (grammar, settling) => new Grammar(grammar.start, grammar.rules, grammar.sourceFormat, { ...grammar.declarations, settling });

// Two rules match `x`; `b` has the higher dynamic precedence.
const choice = `rule s = normal choice(ref(a), ref(b))
rule a = normal literal("x")
rule b = normal dynamicPrecedence(1, literal("x"))
`;
// Under `(matching longest)` the literal and the token rule's pattern both match `if`.
const lexed = `rule s = normal choice(ref(keyword), ref(name))
rule keyword = normal literal("if")
rule name = token repeat1(range("a", "z"))
`;
const listing = (header, body) => parseNativeGrammar(`start s\n${header}${body}`);

test('a grammar declares how its parses are settled, step by step', (context) => {
  assert.deepEqual(SETTLING_STEPS, ['tokens', 'precedence', 'dynamic', 'first', 'ambiguity']);
  // Without a declaration a grammar settles by its matching's default.
  assert.deepEqual(settle(listing('', choice), 'x'), ['(s (b "x"))', 0]);
  assert.deepEqual(settle(listing('settling dynamic ambiguity\n', choice), 'x'), ['(s (b "x"))', 0]);
  // Without `dynamic` the tie is reported, or settled by the first parse.
  assert.deepEqual(settle(listing('settling ambiguity\n', choice), 'x'), ['(s (a "x"))', 1]);
  assert.deepEqual(settle(listing('settling first\n', choice), 'x'), ['(s (a "x"))', 0]);
  assert.deepEqual(settle(listing('settling dynamic first\n', choice), 'x'), ['(s (b "x"))', 0]);

  // `tokens` prefers the tokens a lexer lexes, so the keyword is no ambiguity.
  assert.deepEqual(settle(listing('matching longest\n', lexed), 'if'), ['(s (keyword "if"))', 0]);
  assert.deepEqual(settle(listing('matching longest\nsettling tokens precedence dynamic ambiguity\n', lexed), 'if'), ['(s (keyword "if"))', 0]);
  assert.deepEqual(settle(listing('matching longest\nsettling precedence dynamic ambiguity\n', lexed), 'if'), ['(s (keyword "if"))', 1]);

  // The declaration travels through both native forms.
  const declared = listing('matching longest\nsettling tokens dynamic first\n', lexed);
  assert.deepEqual(declared.declarations.settling, ['tokens', 'dynamic', 'first']);
  assert.match(renderNativeGrammar(declared), /^matching longest\nsettling tokens dynamic first\n/mu);
  const links = renderGrammarLinks(declared);
  assert.match(links, /\(matching longest\) \(settling tokens dynamic first\)\)/u);
  assert.deepEqual(parseGrammarLinks(links).declarations.settling, ['tokens', 'dynamic', 'first']);

  // A settling is known steps, each once, with exactly one tie step, last.
  for (const steps of ['', 'dynamic', 'louder ambiguity', 'dynamic dynamic first', 'first ambiguity', 'ambiguity dynamic']) {
    assert.throws(() => compileGrammar(listing(`settling ${steps}\n`, choice)), steps);
  }
  observe(['settlingDeclaredInGrammar'], context.name);
});

test('every importer settles through the same declared steps', (context) => {
  // The tree-sitter grammars declare the steps of a tree-sitter parser.
  const directory = new URL('js/src/data/native-grammars/', repositoryRoot);
  const declaredBy = new Map();
  for (const file of readdirSync(directory).filter((name) => name.endsWith('.lino'))) {
    const header = readFileSync(new URL(file, directory), 'utf8').split('\n', 1)[0];
    if (header.includes('(matching longest)')) declaredBy.set(file, /\(settling ([^)]*)\)/u.exec(header)?.[1] ?? null);
  }
  assert.ok(declaredBy.size >= 6);
  for (const [file, steps] of declaredBy) assert.equal(steps, DEFAULT_SETTLING.longest.join(' '), file);

  // An ANTLR import settles by the generalized default, a pest import by
  // the PEG one, and either follows a settling declared on it.
  const antlr = importAntlr("grammar g;\ns : a | b ;\na : 'x' ;\nb : 'x' ;\n");
  assert.equal(antlr.declarations.settling, undefined);
  assert.deepEqual(settle(antlr, 'x'), ['(s (a "x"))', 1]);
  assert.deepEqual(settle(withSettling(antlr, [...DEFAULT_SETTLING.generalized]), 'x'), ['(s (a "x"))', 1]);
  assert.deepEqual(settle(withSettling(antlr, ['first']), 'x'), ['(s (a "x"))', 0]);
  const pest = importPest('s = { a | b }\na = { "x" }\nb = { "x" }\n');
  assert.deepEqual(settle(pest, 'x'), settle(withSettling(pest, [...DEFAULT_SETTLING.peg]), 'x'));
  observe(['sharedByEveryImporter'], context.name);
});

test('the executor settles by the declared steps, not by source format', (context) => {
  const runtime = new URL('js/src/grammar-runtime/', repositoryRoot);
  for (const file of readdirSync(runtime)) {
    const source = readFileSync(new URL(file, runtime), 'utf8');
    // No settling decided by the format a grammar was imported from.
    assert.doesNotMatch(source, /sourceFormat\s*===\s*'(?!peg')/u, file);
    assert.doesNotMatch(source, /'tree-sitter'|'antlr'|'lark'/u, file);
  }
  const executor = readFileSync(new URL('executor.js', runtime), 'utf8');
  for (const step of SETTLING_STEPS.filter((name) => name !== 'first')) assert.match(executor, new RegExp(`'${step}'|settling\\.${step}`, 'u'), step);
  observe(['noHostCodeSettling'], context.name);
});
