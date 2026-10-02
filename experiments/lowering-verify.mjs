// Lowers every probed construct, at the top of a rule and nested in a
// sequence, into every notation and checks the package: the executable reads
// back as the lowered grammar with no emission note, and the reconstruction
// equals the original. Prints one status per construct and notation.
import { GRAMMAR_LOWERING_FORMATS, Grammar, checkGrammarLowering } from '../js/src/index.js';

const lit = (value) => ({ kind: 'literal', value });
const ref = (name) => ({ kind: 'ref', name });
const constructs = {
  literal: lit('ab'),
  literalBothQuotes: lit('a"b\'c'),
  literalInsensitive: { kind: 'literalInsensitive', value: 'a-b' },
  charRange: { kind: 'charRange', start: 'a', end: 'f' },
  charClass: { kind: 'charClass', negated: false, items: [{ kind: 'char', value: 'x' }, { kind: 'range', start: '0', end: '3' }] },
  negatedClass: { kind: 'charClass', negated: true, items: [{ kind: 'char', value: 'x' }] },
  any: { kind: 'any' },
  orderedChoice: { kind: 'choice', ordered: true, items: [lit('a'), lit('ab')] },
  orderedPrefixFree: { kind: 'choice', ordered: true, items: [lit('a'), lit('b')] },
  unorderedChoice: { kind: 'choice', ordered: false, items: [lit('a'), lit('b')] },
  choiceOfSeq: { kind: 'choice', ordered: false, items: [{ kind: 'seq', items: [lit('a'), ref('other')] }, { kind: 'empty' }] },
  seq: { kind: 'seq', items: [lit('a'), lit('b')] },
  optional: { kind: 'optional', item: lit('a') },
  repeat0: { kind: 'repeat0', item: lit('a') },
  repeat1: { kind: 'repeat1', item: { kind: 'charRange', start: '0', end: '9' } },
  repeat24: { kind: 'repeat', min: 2, max: 4, item: lit('a') },
  repeat2u: { kind: 'repeat', min: 2, max: null, item: lit('a') },
  and: { kind: 'and', item: lit('a') },
  not: { kind: 'not', item: lit('a') },
  captureLabeled: { kind: 'capture', label: 'x', item: lit('a') },
  captureUnlabeled: { kind: 'capture', label: null, item: { kind: 'optional', item: lit('a') } },
  empty: { kind: 'empty' },
};

const table = {};
let broken = 0;
for (const format of GRAMMAR_LOWERING_FORMATS) {
  for (const [name, construct] of Object.entries(constructs)) {
    for (const position of ['top', 'nested']) {
      const body = position === 'top' ? construct : { kind: 'seq', items: [lit('p'), construct, ref('other')] };
      // KIND sets the kind of the start rule, SOURCE the source format (`own`
      // for the target's own tag, `none` for no tag) and DOC adds a rule doc.
      const kind = process.env.KIND ?? 'normal';
      const rules = new Map([['start', { kind, expression: body }], ['other', { kind: 'normal', expression: lit('o') }]]);
      const tags = { pest: 'peg', 'tree-sitter-json': 'tree-sitter' };
      const source = { own: tags[format] ?? format, none: null }[process.env.SOURCE] ?? process.env.SOURCE ?? 'bnf';
      const grammar = new Grammar('start', rules, source);
      if (process.env.DOC) grammar.rules.set('start', Object.freeze({ ...grammar.rule('start'), doc: process.env.DOC }));
      let verdict;
      try {
        const result = checkGrammarLowering(grammar, format);
        verdict = result.status;
        if (result.status === 'broken') {
          broken += 1;
          if (process.env.VERBOSE) console.error(format, name, position, result.failures, result.lowering.executable, result.lowering.metadata);
        }
      } catch (error) {
        broken += 1;
        verdict = `ERR ${error.message.slice(0, 50)}`;
      }
      (table[`${name}/${position}`] ??= {})[format] = verdict;
    }
  }
}
console.table(table);
console.log(`broken: ${broken}`);
