// Adds (or refreshes) the `translationCorpus` section of four-language-conformance.json:
// the pinned project corpus the 12 directed-translation evidence tests translate, and its oracle.
//   node experiments/issue-195-add-translation-corpus-fixture.mjs
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const fixturePath = new URL('../parity/fixtures/four-language-conformance.json', import.meta.url);
const corpus = new URL('../parity/fixtures/translation-corpus/', import.meta.url);
const digest = (file) => createHash('sha256').update(readFileSync(new URL(file, corpus))).digest('hex');
const theorems = ['fact_five', 'sumTo_formula', 'mirror_mirror', 'size_mirror'];
const camelMappings = [
  'Arith.fact', 'Arith.sumTo', 'Arith.fib', 'Arith.monus', 'Arith.halve', 'Arith.remainder',
  'Tree', 'Tree.leaf', 'Tree.node', 'Tree.size', 'Tree.total', 'Tree.mirror', 'Tree.insert',
  'classify', 'describe', 'sample',
];
const section = {
  description: 'Project programs in the portable core that every directed pair translates; the oracle is the source toolchain run on the source, the expected printed lines, the source theorems and assertions, and a fault-injected source whose translation the target toolchain must reject or observe differently.',
  directory: 'parity/fixtures/translation-corpus',
  expectedOutput: { file: 'expected-output.txt', sha256: digest('expected-output.txt') },
  sources: {
    JavaScript: {
      file: 'project.mjs', sha256: digest('project.mjs'), grammar: 'javascript', extension: '.mjs',
      theorems: [], assertions: 1, mappings: camelMappings,
      mutation: { find: 'if (n === 0n) return 1n;', replace: 'if (n === 0n) return 2n;' },
    },
    Rust: {
      file: 'project.rs', sha256: digest('project.rs'), grammar: 'rust', extension: '.rs',
      theorems: [], assertions: 1,
      mappings: [
        'arith.fact', 'arith.sum_to', 'arith.fib', 'arith.monus', 'arith.halve', 'arith.remainder',
        'Tree', 'Tree.Leaf', 'Tree.Node', 'tree.size', 'tree.total', 'tree.mirror', 'tree.insert',
        'classify', 'describe', 'sample',
      ],
      mutation: { find: 'if n == 0 {\n            1\n', replace: 'if n == 0 {\n            2\n' },
    },
    Lean: {
      file: 'project.lean', sha256: digest('project.lean'), grammar: 'lean', extension: '.lean',
      theorems, assertions: 0, mappings: [...camelMappings, ...theorems],
      mutation: { find: '  | 0 => 1\n  | n + 1 => (n + 1) * fact n', replace: '  | 0 => 2\n  | n + 1 => (n + 1) * fact n' },
    },
    Rocq: {
      file: 'project.v', sha256: digest('project.v'), grammar: 'rocq', extension: '.v',
      theorems, assertions: 0, mappings: [...camelMappings, ...theorems],
      mutation: { find: 'then 1%N else (n * fact', replace: 'then 2%N else (n * fact' },
    },
  },
  targets: {
    JavaScript: { runtime: 'ECMAScript 2026 host with BigInt', proof: false },
    Rust: { runtime: 'Rust 1.98.1 standard library', proof: false },
    Lean: { runtime: 'Lean 4.34.1 core library', proof: true, forbidden: ['sorry', 'admit', 'axiom', 'unsafe', 'implemented_by', 'extern'] },
    Rocq: { runtime: 'Rocq 9.2', proof: true, forbidden: ['Admitted', 'admit', 'Axiom', 'Parameter', 'Conjecture', 'Hypothesis', 'Variable', 'Abort'] },
  },
  assumptions: {
    'JavaScript -> Rust': [],
    'JavaScript -> Lean': [],
    'JavaScript -> Rocq': [],
    'Rust -> JavaScript': [],
    'Rust -> Lean': [],
    'Rust -> Rocq': [],
    'Lean -> JavaScript': [],
    'Lean -> Rust': [],
    'Lean -> Rocq': [],
    'Rocq -> JavaScript': [],
    'Rocq -> Rust': [],
    'Rocq -> Lean': [],
  },
};
const text = readFileSync(fixturePath, 'utf8');
const json = JSON.parse(text);
const rendered = JSON.stringify(section, null, 2).replace(/\n/g, '\n  ');
let next;
if (json.translationCorpus) {
  const start = text.indexOf('  "translationCorpus": ');
  next = `${text.slice(0, start)}  "translationCorpus": ${rendered}\n}\n`;
} else {
  next = `${text.replace(/\n\}\n?$/u, '')},\n  "translationCorpus": ${rendered}\n}\n`;
}
JSON.parse(next);
writeFileSync(fixturePath, next);
console.log('translationCorpus written');
