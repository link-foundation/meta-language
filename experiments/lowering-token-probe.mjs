// Probes whether a token rule with an upper-case name is written by ANTLR and
// Lark without a lossy note, and what a parser rule referring to it reads back as.
import { Grammar, grammarEmitter, grammarImporter, renderLinksExpression } from '../js/src/index.js';

const body = JSON.parse(process.argv[2] ?? '{"kind":"charRange","start":"a","end":"f"}');
for (const format of ['antlr', 'lark', 'tree-sitter-json']) {
  for (const tag of ['bnf', format]) {
    const rules = new Map([
      ['start', { kind: 'normal', expression: { kind: 'seq', items: [{ kind: 'literal', value: 'a' }, { kind: 'ref', name: 'LOWERED1' }] } }],
      ['LOWERED1', { kind: 'token', expression: body }],
    ]);
    const { source, report } = grammarEmitter(format)(new Grammar('start', rules, tag));
    const back = grammarImporter(format)(source);
    console.log(format, tag, JSON.stringify(report.lossy), back.ruleNames().map((n) => `${n}:${back.rule(n).kind}=${renderLinksExpression(back.rule(n).expression)}`).join(' | '));
  }
}
