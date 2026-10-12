#!/usr/bin/env node
// Bounded lexical fixtures. These exercise reusable scanner semantics; they
// are not declarations of complete native language grammar support.
import { readFileSync, writeFileSync } from 'node:fs';
import { scannerFamilies } from './scanner-families.mjs';
import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { renderSyntaxTree } from '../src/grammar-runtime/syntax-tree.js';

export function structuredScannerFixtures() {
  const fixtures = [];
  const add = (name, scanners, rules, accept, reject) => {
    const generated = scannerFamilies(scanners);
    const listing = `(grammar (start source))\n${generated}${rules}\n`;
    const parser = compileGrammar(parseGrammarLinks(listing));
    const trees = {};
    for (const input of accept) {
      const outcome = parser.parseTree(input);
      if (!outcome.ok) throw new Error(`${name} rejects ${JSON.stringify(input)}: ${JSON.stringify(outcome.rejection)}`);
      trees[input] = renderSyntaxTree(outcome.tree);
    }
    for (const input of reject) if (parser.parseTree(input).ok) throw new Error(`${name} accepts invalid ${JSON.stringify(input)}`);
    fixtures.push({ name, scanners, generated, listing, accept, reject, trees });
  };
  const hereRules = '(rule source normal (repeat1 (seq (ref start) (literal %0A) (ref content) (ref end) (optional (literal %0A)))))';
  for (const [name, options, accept, reject] of [
    ['Ruby squiggly here-document labels', { opening: '<<~', indentation: ' \t' }, ['<<~EOF\na😀\n  EOF\n', '<<~"END"\nEND', "<<~'RAW'\n#{value}\nRAW\n", '<<~E\nEXTRA\nE\n<<~F\nF'], ['<<~E\nE trailing', '<<~E\nx E\n', '<<~E\nE_MORE\n']],
    ['Bash tab-stripped here-document labels', { opening: '<<-', indentation: '\t' }, ['<<-EOF\n\ttext\n\tEOF\n', "<<-'EOF'\n$HOME\nEOF"], ['<<-EOF\n EOF\n', '<<-EOF\nEOFX\n']],
    ['PHP flexible here-document labels', { opening: '<<<', indentation: ' \t', closingSuffix: ';' }, ['<<<EOF\né\n  EOF;\n', "<<<'RAW'\n$variable\nRAW;"], ['<<<EOF\nEOF_suffix;', '<<<EOF\nEOF trailing']],
    ['Perl quoted here-document labels', { opening: '<<', queue: true }, ['<<"EOF"\n$name\nEOF\n', "<<'RAW'\nRAW", '<<A\na\nA\n<<B\nb\nB'], ['<<A\nBA\n', '<<A\nA_other\n']],
  ]) add(name, [{ family: 'here-document', name: 'documents', startToken: 'start', contentToken: 'content', endToken: 'end', ...options }], hereRules, accept, reject);
  add('FIFO here-document bodies', [{ family: 'here-document', name: 'documents', startToken: 'start', contentToken: 'content', endToken: 'end', queue: true }], '(rule source normal (seq (ref start) (literal %20) (ref start) (literal %0A) (ref content) (ref end) (literal %0A) (ref content) (ref end)))', ['<<A <<B\na\nA\nb\nB'], ['<<A <<B\na\nB\nb\nA']);
  for (const [language, marker, accept] of [
    ['Ruby', '#{', ['"hello #{name}"', '"#{ {name: "hi"} }"', '"\\#{plain}"']],
    ['Elixir', '#{', ['"#{name}"', '"""multi\n#{name}\nline"""']],
    ['Kotlin', '${', ['"${ {name: "hi"} }"', '"""multi\n${name}\nline"""']],
    ['Scala', '${', ['"hello ${name}"', '"${ {name: "hi"} }"']],
    ['PHP', '{', ['"hello {$variable}"', '"{$array{key}}"']],
  ]) {
    const options = { family: 'interpolated-quoted', name: 'strings', startToken: 'start', contentToken: 'content', endToken: 'end', interpolationStartToken: 'interpolation_start', interpolationEndToken: 'interpolation_end', braceOpenToken: 'brace_open', braceCloseToken: 'brace_close', quotes: [...(['Elixir', 'Kotlin', 'Scala'].includes(language) ? [{ opening: '"""', closing: '"""', interpolation: marker, multiline: true, ...(['Kotlin', 'Scala'].includes(language) ? { escape: null } : {}) }] : []), { opening: '"', closing: '"', interpolation: marker, multiline: false }] };
    const rules = '(rule source normal (repeat1 (ref quoted)))\n(rule quoted normal (seq (ref start) (repeat0 (choice unordered (ref content) (ref interpolation))) (ref end)))\n(rule interpolation normal (seq (ref interpolation_start) (repeat0 (choice unordered (ref expression) (ref quoted) (ref braces))) (ref interpolation_end)))\n(rule braces normal (seq (ref brace_open) (repeat0 (choice unordered (ref expression) (ref quoted) (ref braces))) (ref brace_close)))\n(rule expression token (repeat1 (class negated (char %22) (char %7B) (char %7D))))';
    add(`${language} nested quoted interpolation`, [options], rules, [...accept, '""', '"a""b"'], ['"unfinished', `"${marker}name"`, `"${marker}{name}"`, '"line\nline"']);
  }
  add('Swift parenthesized interpolation', [{ family: 'interpolated-quoted', name: 'strings', startToken: 'start', contentToken: 'content', endToken: 'end', interpolationStartToken: 'interpolation_start', interpolationEndToken: 'interpolation_end', braceOpenToken: 'brace_open', braceCloseToken: 'brace_close', interpolationClosing: ')', braceOpening: '(', braceClosing: ')', quotes: [{ opening: '"', closing: '"', interpolation: '\\(', multiline: false }] }], '(rule source normal (repeat1 (ref quoted)))\n(rule quoted normal (seq (ref start) (repeat0 (choice unordered (ref content) (ref interpolation))) (ref end)))\n(rule interpolation normal (seq (ref interpolation_start) (repeat0 (choice unordered (ref expression) (ref quoted) (ref braces))) (ref interpolation_end)))\n(rule braces normal (seq (ref brace_open) (repeat0 (choice unordered (ref expression) (ref quoted) (ref braces))) (ref brace_close)))\n(rule expression token (repeat1 (class negated (char %22) (char %28) (char %29))))', ['"hello \\(name)"', '"\\(f("hi", (x)))"', '"\\\\(plain)"'], ['"\\(name"', '"\\((name)"']);
  add('Haskell and Elm offside block boundaries', [{ family: 'offside-layout', name: 'layout', openToken: 'open', separatorToken: 'separator', closeToken: 'close', closing: ['in'] }], '(rule source normal (seq (ref block) (optional (literal in))))\n(rule block normal (seq (literal where) (ref open) (ref statement) (repeat0 (seq (ref separator) (ref statement))) (ref close)))\n(rule statement normal (choice ordered (ref block) (ref name)))\n(rule name token (class plain (range a h)))', ['where\n  a\n  b', 'where\n  where\n    a\n  b', 'where a in', 'where\n a\n in'], ['where', 'where\n a\n  b', 'where\n a b']);
  add('YAML indentation block boundaries', [{ family: 'offside-layout', name: 'layout', openToken: 'open', separatorToken: 'separator', closeToken: 'close' }], '(rule source normal (ref block))\n(rule block normal (seq (literal %3A) (ref open) (ref entry) (repeat0 (seq (ref separator) (ref entry))) (ref close)))\n(rule entry normal (choice ordered (ref block) (ref name)))\n(rule name token (class plain (range a h)))', [':\n  a\n  b', ':\n  :\n    a\n  b'], [':', ':\n a\n  b']);
  add('Markdown fenced code block boundaries', [{ family: 'fenced-block', name: 'fences', token: 'fence' }], '(rule source normal (repeat1 (ref fence)))', ['```js\nconst x = 1;\n```\n', '  ~~~~lang\né😀\n ~~~~~\n', '```\n~~~/\n``\n```', '```\nunclosed', '```\n```` bad\n```\n', '```\n```\n~~~\n~~~\n', '```', '```\n   ', '~~~\n~~~x', '```\r\ntext\r\n```\r\n'], ['``\ntext\n``', '    ```\ntext\n```', '```x`x\n```', '```\n\0\n```']);
  return fixtures;
}

if (process.argv[1]?.endsWith('generate-structured-scanner-fixtures.mjs')) {
  const target = new URL('../../parity/fixtures/scanner-structured-contexts.json', import.meta.url);
  const content = `${JSON.stringify(structuredScannerFixtures(), null, 2)}\n`;
  if (process.argv.includes('--check')) {
    if (readFileSync(target, 'utf8') !== content) throw new Error('structured scanner fixtures have drifted');
  } else writeFileSync(target, content);
}
