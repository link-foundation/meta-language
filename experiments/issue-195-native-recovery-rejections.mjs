// Recovers every rejection row of the native grammar fixtures and checks
// the recovered trees are lossless. Usage: node experiments/issue-195-native-recovery-rejections.mjs [id]
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks, renderSyntaxTree } from '../js/src/index.js';
import { NATIVE_GRAMMARS, fixturePath } from '../js/scripts/generate-native-grammar-fixtures.mjs';

const read = (relative) => readFileSync(new URL(`../${relative}`, import.meta.url), 'utf8');
const leaves = (tree) => (tree.type === 'node' ? tree.children.flatMap(leaves) : tree.type === 'missing' ? [] : [tree]);
for (const entry of NATIVE_GRAMMARS.filter(({ id }) => !process.argv[2] || id === process.argv[2])) {
  const fixture = JSON.parse(read(fixturePath(entry)));
  const parser = compileGrammar(parseGrammarLinks(read(entry.grammar)), { errorRecovery: true });
  let slowest = 0;
  for (const { source } of fixture.rejections) {
    const started = performance.now();
    const outcome = parser.parseTree(source);
    slowest = Math.max(slowest, performance.now() - started);
    const text = outcome.tree ? leaves(outcome.tree).map((leaf) => leaf.text).join('') : null;
    const flag = text === source ? '' : ' NOT LOSSLESS';
    if (process.argv[3] || flag) console.log(entry.id, JSON.stringify(source), outcome.rejection?.reason, flag, outcome.tree && renderSyntaxTree(outcome.tree));
  }
  console.log(entry.id, fixture.rejections.length, 'rejections, slowest', slowest.toFixed(1), 'ms');
}
