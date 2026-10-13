// Generate bounded CST controls from the authenticated Rust grammar oracle.
// The fixture records syntax, fields and aliases; it asserts no compiler meaning.
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { LANGUAGE_CATALOG } from '../src/language-catalog.js';
import { grammarSourceOf, sourceText } from './import-native-grammars.mjs';
import { oracleSnapshot } from './native-grammar-rows.mjs';
import { checkProjectData, writeProjectData } from './project-data.mjs';

const entry = grammarSourceOf('native-rust');
const original = JSON.parse(sourceText(entry));
const controls = [
  ['range-end-dereference', 'prompt[..*opening]'],
  ['parenthesized-range-end-dereference', 'prompt[..(*opening)]'],
  ['parenthesized-range-start-dereference', 'prompt[(*opening)..]'],
  ['parenthesized-open-range-multiplication', '(..) * opening'],
  ['ordinary-unary-and-binary', '-x + y * z'],
  ['range-end-identifier', 'prompt[..opening]'],
  ['explicit-range-start-and-dereference', 'prompt[1..*opening]'],
  ['range-start-dereference', 'prompt[*opening..]'],
  ['inclusive-range-end', 'prompt[..=opening]'],
  ['open-index-range', 'prompt[..]'],
  ['ordinary-dereference', '*opening'],
  ['indexed-addition-and-dereference', 'prompt[1 + *opening]'],
  ['range-end-negation', 'prompt[..-opening]'],
];
const cases = controls.map(([id, expression]) => {
  const source = `fn f(){let x = ${expression};}`;
  const oracle = oracleSnapshot(source, 'Rust');
  if (oracle.recovers) throw new Error(`the pinned oracle rejects ${id}`);
  return { id, source, rows: oracle.rows };
});
const value = {
  description: 'A source prefix cannot borrow another production alternative\'s operand precedence. Oracle CST fields and aliases are preserved; CST operator shapes do not establish Rust compiler semantics.',
  sourceGrammar: {
    revision: entry.revision,
    sha256: entry.sha256,
    productions: ['range_expression', 'unary_expression', 'binary_expression'].map(name => ({
      name,
      sha256: createHash('sha256').update(JSON.stringify(original.rules[name])).digest('hex'),
    })),
  },
  formalAiOrigin: {
    revision: 'd209aac6461b355f1a527831202af3423135f7e6',
    file: 'rust/src/agentic_coding/local_search.rs',
    blob: '82b09d73dda676151ab539ea73ad024c7aa1a935',
    byteLength: 23780,
    affectedByteSpans: [[10784, 10794], [11439, 11449]],
    expectedBinaryExpressions: 69,
    expectedUnaryExpressions: 32,
    expectedNamedNodes: 4070,
  },
  projection: LANGUAGE_CATALOG.nativeGrammars['native-rust'],
  operandHeadCases: [
    { sourceRuleMatches: false, sourceReductionMatches: false, accepted: false },
    { sourceRuleMatches: true, sourceReductionMatches: false, accepted: true },
    { sourceRuleMatches: false, sourceReductionMatches: true, accepted: true },
    { sourceRuleMatches: true, sourceReductionMatches: true, accepted: true },
  ],
  operandGrammar: `
    (grammar (start assignment) (matching longest))
    (rule assignment normal (precedence 1 right (seq (ref operand) (literal %3D) (ref operand))))
    (rule operand silent (choice unordered (ref application) (ref identifier) (literal 0)))
    (rule application normal (precedence 4 left (seq (ref operand) (literal %28) (ref operand) (literal %29))))
    (rule identifier token (repeat1 (class plain (range a z))))
  `,
  operandSources: ['x(x)=y', '0(x)=y'],
  cases,
};
const path = fileURLToPath(new URL('../../parity/fixtures/grammar-prefix-operand-precedence.lino', import.meta.url));
if (process.argv.includes('--check')) checkProjectData(path, value);
else writeProjectData(path, value);
