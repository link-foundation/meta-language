// Probes which grammar constructs each text emitter writes so that the
// importer of the same format reads them back canonically unchanged and
// without a lossy note. Used to design the faithful lowering tables.
import {
  GRAMMAR_LOSSLESS_FORMATS, Grammar, canonicalRuleDefinition, grammarEmitter, grammarImporter, renderLinksExpression,
} from '../js/src/index.js';

const lit = (value) => ({ kind: 'literal', value });
const ref = (name) => ({ kind: 'ref', name });
const constructs = {
  literal: lit('ab'),
  literalBothQuotes: lit('a"b\'c'),
  literalInsensitive: { kind: 'literalInsensitive', value: 'ab' },
  charRange: { kind: 'charRange', start: 'a', end: 'f' },
  charClass: { kind: 'charClass', negated: false, items: [{ kind: 'char', value: 'x' }, { kind: 'range', start: '0', end: '3' }] },
  negatedClass: { kind: 'charClass', negated: true, items: [{ kind: 'char', value: 'x' }] },
  any: { kind: 'any' },
  orderedChoice: { kind: 'choice', ordered: true, items: [lit('a'), lit('b')] },
  unorderedChoice: { kind: 'choice', ordered: false, items: [lit('a'), lit('b')] },
  seq: { kind: 'seq', items: [lit('a'), lit('b')] },
  optional: { kind: 'optional', item: lit('a') },
  repeat0: { kind: 'repeat0', item: lit('a') },
  repeat1: { kind: 'repeat1', item: lit('a') },
  repeat24: { kind: 'repeat', min: 2, max: 4, item: lit('a') },
  repeat2u: { kind: 'repeat', min: 2, max: null, item: lit('a') },
  and: { kind: 'and', item: lit('a') },
  not: { kind: 'not', item: lit('a') },
  captureLabeled: { kind: 'capture', label: 'x', item: lit('a') },
  captureUnlabeled: { kind: 'capture', label: null, item: lit('a') },
  empty: { kind: 'empty' },
};

const formats = GRAMMAR_LOSSLESS_FORMATS;
const table = {};
for (const format of formats) {
  const source = { abnf: 'abnf', antlr: 'antlr', bnf: 'bnf', ebnf: 'ebnf', gbnf: 'gbnf', lark: 'lark', pest: 'peg', 'tree-sitter-json': 'tree-sitter' }[format];
  for (const [name, construct] of Object.entries(constructs)) {
    for (const position of ['top', 'nested']) {
      for (const kind of ['normal']) {
        const body = position === 'top' ? construct : { kind: 'seq', items: [lit('p'), construct, ref('other')] };
        const rules = new Map([['start', { kind, expression: body }], ['other', { kind: 'normal', expression: lit('o') }]]);
        const grammar = new Grammar('start', rules, source);
        let verdict;
        try {
          const { source: text, report } = grammarEmitter(format)(grammar);
          const back = grammarImporter(format)(text);
          // GBNF names its start rule root; compare the start rule by role.
          const backRule = (rule) => (rule === 'start' ? back.startRule() : back.rule(rule));
          const same = ['start', 'other'].every((rule) => backRule(rule)
            && canonicalRuleDefinition(backRule(rule)) === canonicalRuleDefinition(grammar.rule(rule)));
          const extra = back.ruleNames().filter((rule) => !['start', 'other', back.startRule()?.name].includes(rule));
          verdict = `${same ? 'same' : 'DIFF'}${report.lossy.length ? ' lossy' : ''}${extra.length ? ` +${extra.length}` : ''}`;
          if (!same && process.env.VERBOSE) {
            console.error(format, name, position, renderLinksExpression(back.startRule()?.expression ?? { kind: 'empty' }), JSON.stringify(text));
          }
        } catch (error) {
          verdict = `ERR ${error.message.slice(0, 40)}`;
        }
        (table[`${name}/${position}`] ??= {})[format] = verdict;
      }
    }
  }
}
if (process.env.FORMAT) { for (const [k, v] of Object.entries(table)) console.log(k.padEnd(28), v[process.env.FORMAT]); } else console.table(table);
