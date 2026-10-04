import { LinkNetwork, LinkType, analyzeProgram, translateProgram } from '../src/index.js';
const corpus = [
  ['JavaScript', 'const α = 1;\n'],
  ['Rust', 'fn id(x: u64) -> u64 { x }\n'],
  ['Lean', 'def id (x : Nat) : Nat := x\n'],
  ['Rocq', 'Definition id (x : nat) : nat := x.\n'],
  ['Coq', 'Definition id (x : nat) : nat := x.\n'],
  ['Markdown', '# Title\n\n```js\nlet a = 1;\n```\n'],
  ['LiNo', '(papa (lovesMama: loves mama))\n'],
  ['Python', 'def f(x):\n    return x\n'],
];
for (const [language, source] of corpus) {
  const n = LinkNetwork.parse(source, language);
  const terms = [...new Set(n.links().filter((l) => l.metadata().linkType === LinkType.Syntax).map((l) => l.metadata().term))];
  console.log(language, n.verifyFullMatch().isClean(), n.reconstructText() === source, terms.slice(0, 12).join(','));
}
for (const [language, source] of corpus.slice(0, 4)) console.log(language, analyzeProgram(source, language).bindings.map((b) => `${b.name}:${b.kind}`).join(' '));
console.log(translateProgram('console.log(42);', 'JavaScript', 'Rust').code);
