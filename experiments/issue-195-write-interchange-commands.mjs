// Writes the `commands` section of parity/fixtures/grammar-importers.json: the
// shared `meta-language grammar` invocations, their input files and the
// standard output, standard error and exit status both runtimes must report.
// The expectations are taken from the JavaScript runner and kept only when
// the Rust binary reports the same; where only the parser's own detail line
// after `error: cannot import ...` differs, just the first standard error
// line is recorded as `stderrHead`. Each recorded expectation is meant to be
// reviewed by hand. Usage:
// node experiments/issue-195-write-interchange-commands.mjs [rust binary]
import { spawnSync } from 'node:child_process';
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import { runGrammarCommand } from '../js/src/grammar-interchange.js';

const fixtureUrl = new URL('../parity/fixtures/grammar-importers.json', import.meta.url);
const binary = process.argv[2] ?? 'rust/target/debug/meta-language';
const corpus = JSON.parse(readFileSync(fixtureUrl, 'utf8'));

const extension = { abnf: 'abnf', bnf: 'bnf', ebnf: 'ebnf', pest: 'pest', 'tree-sitter-json': 'json' };
const caseFile = (id) => {
  const testCase = corpus.cases.find((entry) => entry.id === id);
  return `${id.split(':')[1]}.${extension[testCase.format]}`;
};
const caseFiles = (...ids) => Object.fromEntries(ids.map((id) => [caseFile(id), { case: id }]));

const PROBLEMS = `start expr
rule expr = normal choice(seq(ref(expr), literal("+"), ref(term)), ref(term))
rule term = normal seq(ref(missing), repeat0(optional(literal("x"))), capture("lbl", literal("y")))
rule orphan = normal literal("z")
rule a = normal seq(ref(b), literal("1"))
rule b = normal choice(ref(a), empty)
`;
const LISTING = `# a native listing
format abnf
start sum
rule sum = normal seq(ref(digit), repeat(seq(literal("+"), ref(digit)), 0, unbounded))
rule digit = token class(range("0", "9"))
`;

const specs = [
  { id: 'help', args: ['help'] },
  { id: 'usage-without-command', args: [] },
  { id: 'formats', args: ['formats'] },
  ...corpus.cases.map(({ id, format }) => ({ id: `import:${id}`, args: ['import', '--from', format, caseFile(id)], files: caseFiles(id) })),
  { id: 'validate:abnf:message', args: ['validate', '--from', 'abnf', caseFile('abnf:message')], files: caseFiles('abnf:message') },
  { id: 'validate:problems', args: ['validate', '--from', 'native', 'problems.grammar'], files: { 'problems.grammar': PROBLEMS } },
  { id: 'convert:abnf-to-pest', args: ['convert', '--from', 'abnf', '--to', 'pest', caseFile('abnf:message')], files: caseFiles('abnf:message') },
  { id: 'convert:bnf-to-tree-sitter-json', args: ['convert', '--from', 'bnf', '--to', 'tree-sitter-json', caseFile('bnf:message')], files: caseFiles('bnf:message') },
  { id: 'convert:pest-to-ebnf', args: ['convert', '--from', 'pest', '--to', 'ebnf', caseFile('pest:message')], files: caseFiles('pest:message') },
  { id: 'convert:abnf-constructs-to-gbnf', args: ['convert', '--from', 'abnf', '--to', 'gbnf', 'constructs.abnf'], files: caseFiles('abnf:constructs') },
  { id: 'convert:tree-sitter-json-constructs-to-bnf-unsupported', args: ['convert', '--from', 'tree-sitter-json', '--to', 'bnf', 'constructs.json'], files: caseFiles('tree-sitter-json:constructs') },
  { id: 'convert:pest-constructs-to-abnf-unsupported', args: ['convert', '--from', 'pest', '--to', 'abnf', 'constructs.pest'], files: caseFiles('pest:constructs') },
  { id: 'convert:ebnf-to-antlr', args: ['convert', '--from', 'ebnf', '--to', 'antlr', caseFile('ebnf:message')], files: caseFiles('ebnf:message') },
  { id: 'convert:ebnf-constructs-to-lark', args: ['convert', '--from', 'ebnf', '--to', 'lark', 'constructs.ebnf'], files: caseFiles('ebnf:constructs') },
  { id: 'export:native-to-abnf', args: ['export', '--to', 'abnf', 'sum.grammar'], files: { 'sum.grammar': LISTING } },
  { id: 'export:native-to-native', args: ['export', '--to', 'native', 'sum.grammar'], files: { 'sum.grammar': LISTING } },
  {
    id: 'merge:bnf-and-ebnf',
    args: ['merge', '--language', 'message', '--source', `bnf:${caseFile('bnf:message')}`, '--source', `ebnf:${caseFile('ebnf:message')}`, '--to', 'ebnf'],
    files: caseFiles('bnf:message', 'ebnf:message'),
  },
  {
    id: 'merge:required-equivalence-unresolved',
    args: ['merge', '--source', 'abnf:message.abnf', '--source', 'pest:message.pest', '--require', 'message.abnf:message=message.pest:word'],
    files: caseFiles('abnf:message', 'pest:message'),
  },
  { id: 'rename:referenced-rule', args: ['rename', '--from', 'ebnf', '--rule', 'word', '--name', 'name', '--to', 'ebnf', caseFile('ebnf:message')], files: caseFiles('ebnf:message') },
  { id: 'rename:unknown-rule', args: ['rename', '--from', 'ebnf', '--rule', 'nothing', '--name', 'name', caseFile('ebnf:message')], files: caseFiles('ebnf:message') },
  ...corpus.cases.map(({ id, format, accepts, rejects }) => ({
    id: `round-trip:${id}`,
    args: ['round-trip', '--from', format, ...accepts.flatMap((text) => ['--accept', text]), ...rejects.flatMap((text) => ['--reject', text]), caseFile(id)],
    files: caseFiles(id),
  })),
  { id: 'round-trip:native-wrong-samples', args: ['round-trip', '--from', 'native', '--accept', '1+x', '--reject', '1+2', 'sum.grammar'], files: { 'sum.grammar': LISTING } },
  { id: 'error:unknown-command', args: ['compile'] },
  { id: 'error:unknown-option', args: ['import', '--form', 'bnf', 'a.bnf'] },
  { id: 'error:option-needs-value', args: ['import', 'a.bnf', '--from'] },
  { id: 'error:option-given-twice', args: ['convert', '--from', 'bnf', '--from', 'ebnf', '--to', 'pest', 'a.bnf'] },
  { id: 'error:missing-option', args: ['convert', '--from', 'bnf', 'a.bnf'] },
  { id: 'error:missing-file', args: ['validate', '--from', 'bnf'] },
  { id: 'error:unexpected-argument', args: ['import', '--from', 'bnf', 'a.bnf', 'b.bnf'] },
  { id: 'error:unknown-import-format', args: ['import', '--from', 'yacc', 'a.y'] },
  { id: 'error:unknown-export-format', args: ['convert', '--from', 'bnf', '--to', 'yacc', 'a.bnf'] },
  { id: 'error:cannot-read', args: ['import', '--from', 'bnf', 'absent.bnf'] },
  { id: 'error:merge-source-without-format', args: ['merge', '--source', 'message.bnf'] },
  { id: 'error:merge-duplicate-file-names', args: ['merge', '--source', 'bnf:a/message.bnf', '--source', 'bnf:b/message.bnf'], files: { 'a/message.bnf': { case: 'bnf:message' }, 'b/message.bnf': { case: 'bnf:message' } } },
  { id: 'error:merge-required-equivalence-without-equals', args: ['merge', '--source', 'bnf:message.bnf', '--require', 'message.bnf:word'], files: caseFiles('bnf:message') },
  ...corpus.malformed.map(({ format, reason }, index) => ({
    id: `error:malformed:${format}:${reason.replaceAll(' ', '-')}`,
    args: ['import', '--from', format, `malformed.${extension[format]}`],
    files: { [`malformed.${extension[format]}`]: { malformed: index } },
  })),
  ...[
    ['no-rules', '# nothing\n\n'],
    ['unknown-directive', 'rules a = normal empty\n'],
    ['unknown-expression', 'rule a = normal lit("x")\n'],
    ['unterminated-string', 'rule a = normal literal("x)\n'],
    ['reversed-bounds', 'rule a = normal repeat(any, 3, 2)\n'],
    ['bound-too-large', 'rule a = normal repeat(any, 1234567890, unbounded)\n'],
    ['wide-character', 'rule a = normal class(char("ab"))\n'],
    ['unknown-kind', 'rule a = loud empty\n'],
    ['duplicate-rule', 'rule a = normal empty\nrule a = normal any\n'],
    ['undefined-start', 'start b\nrule a = normal empty\n'],
    ['trailing-text', 'rule a = normal empty empty\n'],
  ].map(([name, text]) => ({ id: `error:native:${name}`, args: ['import', '--from', 'native', 'bad.grammar'], files: { 'bad.grammar': text } })),
];

function contents(files) {
  return Object.fromEntries(
    Object.entries(files ?? {}).map(([name, value]) => [
      name,
      typeof value === 'string' ? value : 'case' in value ? corpus.cases.find(({ id }) => id === value.case).source : corpus.malformed[value.malformed].source,
    ]),
  );
}

const commands = specs.map((spec) => {
  const files = contents(spec.files);
  const readFile = (name) => {
    if (!Object.hasOwn(files, name)) throw new Error(`no file ${name}`);
    return files[name];
  };
  const js = runGrammarCommand(spec.args, { readFile });
  const directory = mkdtempSync(join(tmpdir(), 'interchange-commands-'));
  for (const [name, text] of Object.entries(files)) {
    spawnSync('mkdir', ['-p', join(directory, name, '..')]);
    writeFileSync(join(directory, name), text);
  }
  const result = spawnSync(join(process.cwd(), binary), ['grammar', ...spec.args], { cwd: directory, encoding: 'utf8' });
  const rust = { exitCode: result.status, stdout: result.stdout, stderr: result.stderr };
  if (js.exitCode !== rust.exitCode || js.stdout !== rust.stdout) {
    throw new Error(`${spec.id}: runtimes disagree\n${JSON.stringify({ js, rust }, null, 2)}`);
  }
  const entry = { id: spec.id, args: spec.args, files: spec.files ?? {}, exitCode: js.exitCode, stdout: js.stdout };
  if (js.stderr === rust.stderr) return { ...entry, stderr: js.stderr };
  const [head] = js.stderr.split('\n');
  if (!head.startsWith('error: cannot import ') || rust.stderr.split('\n')[0] !== head) {
    throw new Error(`${spec.id}: standard error differs\n${JSON.stringify({ js, rust }, null, 2)}`);
  }
  return { ...entry, stderrHead: head };
});

// Two-space JSON with short string arrays kept on one line, as in
// experiments/issue-195-write-importer-fixture.mjs.
function format(value, indent = '') {
  const next = `${indent}  `;
  if (Array.isArray(value)) {
    if (value.every((item) => typeof item === 'string')) {
      return `[${value.map((item) => JSON.stringify(item)).join(', ')}]`;
    }
    return `[\n${value.map((item) => next + format(item, next)).join(',\n')}\n${indent}]`;
  }
  if (value && typeof value === 'object') {
    const entries = Object.entries(value);
    if (entries.length === 0) return '{}';
    return `{\n${entries.map(([key, item]) => `${next}${JSON.stringify(key)}: ${format(item, next)}`).join(',\n')}\n${indent}}`;
  }
  return JSON.stringify(value);
}

writeFileSync(fixtureUrl, `${format({ ...corpus, commands })}\n`);
console.log(`${commands.length} commands, ${commands.filter((command) => 'stderrHead' in command).length} with a runtime-specific detail line`);
