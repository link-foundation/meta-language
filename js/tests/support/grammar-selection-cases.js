// Small authored definitions only. These labels exercise generic dispatch;
// they are deliberately not substitutes for catalog language grammars.
import { percentEncodeLinksText } from '../../src/grammar-links.js';

export const grammarSelectionLiteral = text => `(literal ${percentEncodeLinksText(text)})`;
export const grammarSelectionCandidate = (name, mapping = 'identity') => `(grammar-candidate ${percentEncodeLinksText(name)} ${mapping})`;
export const grammarSelectionExpression = (mode, names, region = '(ref body)') => `(${mode} ${region} ${names.map(name => grammarSelectionCandidate(name)).join(' ')})`;

export function authoredGrammarSelectionListing(expression, root = 'document') {
  return `(grammar (start ${root}))
(extra (ref spaces))
(rule ${root} normal ${expression} (source-names (authored ${root})))
(rule spaces token (repeat1 (choice unordered ${grammarSelectionLiteral(' ')} ${grammarSelectionLiteral('\n')})))
(rule body silent (repeat0 (choice unordered (seq ${grammarSelectionLiteral('{')} (ref body) ${grammarSelectionLiteral('}')}) (seq (not (choice unordered ${grammarSelectionLiteral('{')} ${grammarSelectionLiteral('}')})) any))))
`;
}

export function grammarSelectionFixture() {
  const literal = grammarSelectionLiteral, select = grammarSelectionExpression, grammar = authoredGrammarSelectionListing;
  return { schemaVersion: 1, id: 'grammar-recursive-selection',
    boundary: 'Authored miniature grammar definitions, not catalog-language support or escape-decoding evidence.',
    grammars: [
      { name: 'Markdown', listing: grammar(`(seq ${literal('m{')} ${select('any-grammar', ['JavaScript', 'Python'])} ${literal('}')})`) },
      { name: 'JavaScript', listing: grammar(`(seq ${literal('j{')} ${select('any-grammar', ['JSON'])} ${literal('}')})`) },
      { name: 'JSON', listing: grammar(`(seq ${literal('d{')} ${select('any-grammar', ['Markdown'])} ${literal('}')})`) },
      { name: 'Python', listing: grammar(literal('π')) },
      { name: 'first', listing: grammar(literal('π'), 'first_root') },
      { name: 'second', listing: grammar(literal('π'), 'second_root') },
      { name: 'other', listing: grammar(literal('x')) },
    ],
    chain: { root: 'Markdown', source: ' m{ j{d{m{π}}}} \n', languages: ['Markdown', 'JavaScript', 'JSON', 'Markdown', 'Python'], edition: 'authored-1' },
    any: { listing: grammar(select('any-grammar', ['first', 'second'])), source: 'π', languages: ['first', 'second'] },
    all: { listing: grammar(select('all-grammar', ['first', 'second'])), source: 'π' },
    allRejected: { listing: grammar(select('all-grammar', ['first', 'other'])), source: 'π' },
    unknown: { listing: grammar(select('any-grammar', ['first', 'missing'])), source: 'π', reason: 'unknownGrammar' },
    escaped: { listing: grammar(`(any-grammar (ref body) ${grammarSelectionCandidate('first', 'quoted-escape')})`), source: 'π', reason: 'unsupportedEmbeddingMapping' },
    cyclic: { name: 'cycle', listing: grammar(select('any-grammar', ['cycle'], literal('π'))), source: 'π', reason: 'nonConsumingEmbeddingCycle' },
    consuming: { name: 'recursive', listing: grammar(`(choice unordered ${literal('x')} (seq ${literal('{')} ${select('any-grammar', ['recursive'])} ${literal('}')}))`), sources: ['x', '{x}', '{{{x}}}'] },
  };
}
