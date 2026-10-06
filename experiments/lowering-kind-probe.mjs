// Probes which rule kinds each notation writes and reads back unchanged, and
// whether the rule keeps its name, for a rule holding a plain literal.
import { GRAMMAR_LOWERING_FORMATS, Grammar, grammarEmitter, grammarImporter } from '../js/src/index.js';

for (const source of ['bnf', 'native']) {
  const table = {};
  for (const format of GRAMMAR_LOWERING_FORMATS) {
    const tag = source === 'native' ? { pest: 'peg', 'tree-sitter-json': 'tree-sitter' }[format] ?? format : source;
    for (const kind of ['normal', 'atomic', 'silent', 'token']) {
      const rules = new Map([
        ['start', { kind: 'normal', expression: { kind: 'seq', items: [{ kind: 'literal', value: 'a' }, { kind: 'ref', name: 'item' }] } }],
        ['item', { kind, expression: { kind: 'literal', value: 'b' } }],
      ]);
      let verdict;
      try {
        const { source: text, report } = grammarEmitter(format)(new Grammar('start', rules, tag));
        const back = grammarImporter(format)(text);
        const [, second] = back.ruleNames();
        verdict = `${back.rule(second)?.kind}${second === 'item' ? '' : ` as ${second}`}${report.lossy.length ? ' lossy' : ''}`;
      } catch (error) {
        verdict = `ERR ${error.message.slice(0, 30)}`;
      }
      (table[kind] ??= {})[format] = verdict;
    }
  }
  console.log(source);
  console.table(table);
}
