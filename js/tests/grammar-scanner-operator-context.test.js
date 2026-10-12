import assert from 'node:assert/strict';
import test from 'node:test';
import { operatorLineScanner, unaryOperatorBoundaryScanner } from '../scripts/operator-context-scanner.mjs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { readProjectData } from '../scripts/project-data.mjs';

const profiles = readProjectData(new URL('../../parity/naming/grammar-name-expansions.json', import.meta.url)).grammars.elixir.scannerFamilies;
const line = profiles.find(({ family }) => family === 'operator-line');
const unary = profiles.find(({ family }) => family === 'unary-operator-boundary');
const leaves = node => node.type === 'token' ? [node] : node.children.flatMap(leaves);
const scannerParser = (listing, token) => compileGrammar(parseGrammarLinks(`(grammar (start document))
${listing}(rule document normal (seq (immediateToken (ref ${token})) (token (repeat0 any))))
`));

test('operator line decisions retain every pinned operator and stop before its source text', () => {
  const parser = scannerParser(operatorLineScanner(line), line.operatorToken);
  for (const operator of [...line.operators, ...line.wordOperators, line.separatedOperator.join(' ')]) {
    const prefix = ' \t\r\n  \n\t';
    const source = `${prefix}${operator} x`;
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, source);
    assert.deepEqual(outcome.ambiguities, [], source);
    const tokens = leaves(outcome.tree);
    assert.equal(tokens.map(({ text }) => text).join(''), source);
    assert.equal(tokens.at(-1).text, `${operator} x`, source);
    assert.equal(tokens.at(-1).start, Buffer.byteLength(prefix), source);
  }
});

test('operator lookahead rejects incomplete longer operators, keywords and numeric arities', () => {
  const parser = scannerParser(operatorLineScanner(line), line.operatorToken);
  for (const source of ['\n+ x', '\n- x', '\n<< x', '\n>> x', '\n<| x', '\n^ x', '\n^^ x', '\n! x', '\n~ x', '\n::: x', '\n... x', '\nwhenx', '\nnot input', '\n++: x', '\n++ / 2', '\n==/\n2']) {
    assert.equal(parser.parseTree(source).ok, false, source);
  }
  for (const source of ['\n++:x', '\n++ / x', '\nnot\tin[x]', '\nwhen[x]']) {
    assert.equal(parser.parseTree(source).ok, true, source);
  }
});

test('block and comment line markers preserve their independent validity and marked spans', () => {
  const listing = operatorLineScanner(line);
  const block = scannerParser(listing, line.keywordToken);
  const comment = scannerParser(listing, line.commentToken);
  for (const source of ['\ndo\n', ' \r\n \tdo(x)']) assert.equal(block.parseTree(source).ok, true, source);
  for (const source of ['\ndoodle', '\ndo: x', '\n# comment']) assert.equal(block.parseTree(source).ok, false, source);
  for (const source of ['\n # comment', '\r\n\t#']) {
    const outcome = comment.parseTree(source);
    assert.equal(outcome.ok, true, source);
    assert.equal(leaves(outcome.tree).at(-1).text, source.slice(source.indexOf('#')));
    assert.equal(leaves(outcome.tree).map(({ text }) => text).join(''), source);
  }
  for (const source of ['\ndo\n', '\n+ x']) assert.equal(comment.parseTree(source).ok, false, source);
});

test('unary boundaries require preceding horizontal space and an adjacent operand', () => {
  const parser = scannerParser(unaryOperatorBoundaryScanner(unary), unary.token);
  for (const source of [' +x', '\t-x', ' \t+\u00e9']) {
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, source);
    assert.equal(leaves(outcome.tree).map(({ text }) => text).join(''), source);
    assert.equal(leaves(outcome.tree).at(-1).text, source.trimStart());
  }
  for (const source of ['+x', '-x', '\n+x', ' + x', ' -\nx', ' ++x', ' --x', ' ->x', ' +:x', ' -/2']) {
    assert.equal(parser.parseTree(source).ok, false, source);
  }
});

test('private external extras keep concrete text without becoming named syntax rows', () => {
  const source = {
    name: 'private_extra',
    externals: [{ type: 'SYMBOL', name: '_line' }],
    extras: [{ type: 'SYMBOL', name: '_line' }],
    rules: { document: { type: 'REPEAT1', content: { type: 'STRING', value: 'x' } } },
  };
  const scanner = '(scanner line (tokens _line) (operations (consume (literal %0A)) (emit _line)))\n';
  const imported = importTreeSitterNative(source, { scanners: scanner });
  assert.deepEqual(imported.report.unsupported, []);
  const parser = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported)));
  const input = 'x\nx';
  const outcome = parser.parseTree(input);
  assert.equal(outcome.ok, true);
  const tokens = leaves(outcome.tree);
  assert.equal(tokens.map(({ text }) => text).join(''), input);
  assert.equal(tokens.find(({ text }) => text === '\n').kind, null);
  assert.equal(tokens.some(({ kind }) => kind === '_line'), false);
});

test('contextual source-pattern replacements preserve anonymous extra identity independently of private tokens', async () => {
  const { transformNativeSource } = await import('../scripts/native-grammar-transforms.mjs');
  const source = { name: 'contextual_extra', extras: [{ type: 'PATTERN', value: '\\s' }],
    rules: { document: { type: 'REPEAT1', content: { type: 'STRING', value: 'x' } } } };
  const transformed = transformNativeSource(source, [{ family: 'contextual-newline-extras', pattern: '\\s', token: '_line', anonymous: true }]);
  const scanner = '(scanner line (tokens _line) (operations (consume (literal %0A)) (emit _line)))\n';
  const imported = importTreeSitterNative(transformed, { scanners: scanner });
  assert.deepEqual(imported.report.unsupported, []);
  const outcome = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported))).parseTree('x\nx');
  assert.equal(outcome.ok, true);
  const tokens = leaves(outcome.tree);
  assert.equal(tokens.map(({ text }) => text).join(''), 'x\nx');
  assert.equal(tokens.find(({ text }) => text === '\n').kind, 'unnamed_token');
  assert.equal(tokens.find(({ text }) => text === '\n').trivia, true);
  assert.equal(tokens.some(({ kind }) => kind === '_line'), false);
  assert.throws(() => transformNativeSource(source, [{ family: 'contextual-newline-extras', pattern: '\\s', token: '_line', anonymous: 'true' }]), TypeError);
  const forged = structuredClone(transformed);
  forged.extras.at(-1).content = { type: 'STRING', value: 'x' };
  assert.throws(() => importTreeSitterNative(forged, { scanners: scanner }), /declared private scanner token/u);
});

test('public external extras retain their named token identity and source span', () => {
  const source = {
    name: 'public_extra',
    externals: [{ type: 'SYMBOL', name: 'annotation' }],
    extras: [{ type: 'SYMBOL', name: 'annotation' }],
    rules: { document: { type: 'REPEAT1', content: { type: 'STRING', value: 'x' } } },
  };
  const scanner = '(scanner annotations (tokens annotation) (operations (consume (literal %23)) (emit annotation)))\n';
  const imported = importTreeSitterNative(source, { scanners: scanner });
  const outcome = compileGrammar(parseGrammarLinks(renderTreeSitterNative(imported))).parseTree('x#x');
  assert.equal(outcome.ok, true);
  assert.deepEqual(leaves(outcome.tree).filter(({ kind }) => kind === 'annotation').map(({ text, start, end, trivia }) => ({ text, start, end, trivia })), [{ text: '#', start: 1, end: 2, trivia: true }]);
});
