// Compare native action inference with an independently instrumented parser.
import { createHash } from 'node:crypto';
import { performance } from 'node:perf_hooks';
import { compileGrammar } from '../../js/src/grammar.js';
import { inferGrammarAttributeAction } from '../../js/src/grammar-attribute-inference.js';
import { parseGrammarLinks, renderGrammarLinks } from '../../js/src/grammar-links.js';

const seed = 227224;
const oracleVersion = '24.14.0';
if (process.versions.node !== oracleVersion) {
  throw new Error('no attributed holdout oracle for Node ' + process.versions.node + '. Expected ' + oracleVersion);
}
const sourceLimit = 256;
const parserOptions = { ambiguity: 'reject', maxDepth: 64, stepLimit: 4096, memoLimit: 1024, memoryLimit: 2 * 1024 * 1024 };
const request = { rule: 'row', attribute: 'total', field: 'items', childAttribute: 'quantity' };
const listing = `(grammar (start row))
(extra (class plain (char %20) (char %09)))
(rule row normal (seq (literal %5B) (capture labeled items (ref quantity)) (repeat0 (seq (literal %2C) (capture labeled items (ref quantity)))) (literal %5D)))
(rule quantity normal (ref digits) (action (setAttribute quantity (number (matched)))))
(rule digits token (repeat1 (class plain (range 0 9))))
`;

function readInstrumentedRow(source) {
  if (Buffer.byteLength(source) > sourceLimit) return { diagnostic: 'input byte limit' };
  let position = 0;
  const fields = [];
  const skip = () => {
    while (source[position] === ' ' || source[position] === '\t') position += 1;
  };
  const reject = diagnostic => ({ diagnostic, position, fields });
  if (source[position++] !== '[') return reject('expected opening bracket');
  for (;;) {
    skip();
    const start = position;
    let quantity = 0;
    while (source[position] >= '0' && source[position] <= '9') {
      quantity = quantity * 10 + source.charCodeAt(position++) - 48;
    }
    if (start === position) return reject('expected unsigned quantity');
    fields.push({ start, end: position, quantity });
    skip();
    if (source[position] !== ',') break;
    position += 1;
  }
  if (source[position++] !== ']') return reject('expected closing bracket');
  if (position !== source.length) return reject('expected end of row');
  const total = fields.reduce((sum, { quantity }) => sum + quantity, 0);
  if (!Number.isSafeInteger(total)) return reject('quantity exceeds safe integer precision');
  return { fields, total };
}

const training = ['[602, 113]', '[497,\t71, 306]', '[229]'].map(source => {
  const trace = readInstrumentedRow(source);
  if (trace.diagnostic) throw new Error(trace.diagnostic);
  return { source, path: [], start: 0, end: Buffer.byteLength(source), value: trace.total, fields: trace.fields };
});

const started = performance.now();
const initial = parseGrammarLinks(listing);
const learned = inferGrammarAttributeAction(initial, request, training);
const repeated = inferGrammarAttributeAction(initial, request, training);
const grammar = learned.grammar ? renderGrammarLinks(learned.grammar) : null;
const deterministic = JSON.stringify(learned.candidates) === JSON.stringify(repeated.candidates)
  && grammar === (repeated.grammar ? renderGrammarLinks(repeated.grammar) : null);
const syntaxParser = compileGrammar(initial, parserOptions);
const attributedParser = grammar && compileGrammar(parseGrammarLinks(grammar), parserOptions);
function inspectLeaves(tree, input) {
  const pending = [tree];
  const pieces = [];
  let position = 0;
  let trivia = 0;
  let spans = tree.start === 0 && tree.end === Buffer.byteLength(input);
  while (pending.length) {
    const node = pending.pop();
    if (node.children) {
      for (let index = node.children.length - 1; index >= 0; index -= 1) pending.push(node.children[index]);
    } else if (node.type === 'token') {
      spans &&= node.start === position && Buffer.byteLength(node.text) === node.end - node.start;
      position = node.end;
      pieces.push(node.text);
      if (node.trivia) trivia += 1;
    }
  }
  return { reconstructs: pieces.join('') === input, spans, trivia };
}

const corpus = new Set(['[]', '[,1]', '[1,]', '[2 3]', '[+4]', '[-2]', '[1,π]', '[0]\n', 'x[2]', '[2,,3]']);
let state = seed;
const next = () => {
  state = (Math.imul(1664525, state) + 1013904223) >>> 0;
  return state;
};
for (let row = 0; row < 96; row += 1) {
  const values = Array.from({ length: row % 7 + 1 }, () => 1000 + next() % 8000);
  corpus.add('[' + values.join(row % 3 === 0 ? ',\t' : ', ') + ']');
  corpus.add('[' + values.join(', ') + ',]');
}
const rows = [];
for (const input of corpus) {
  if (training.some(row => row.source === input)) throw new Error('training input entered holdout');
  const independent = readInstrumentedRow(input);
  const regularOracle = /^\[[ \t]*[0-9]+(?:[ \t]*,[ \t]*[0-9]+)*[ \t]*\](?![\s\S])/u.test(input);
  const syntax = syntaxParser.parseTree(input);
  const attributed = attributedParser?.parseTree(input);
  const fields = attributed?.ok ? attributed.tree.children.filter(child => child.field === 'items').map(child => ({
    start: child.start, end: child.end, quantity: child.attributes?.quantity,
  })) : null;
  rows.push({ input, independent, regularOracle, syntax: syntax.ok, syntaxDiagnostic: syntax.rejection,
    syntaxAmbiguities: syntax.ambiguities, attributedDiagnostic: attributed?.rejection ?? null,
    attributedAmbiguities: attributed?.ambiguities ?? null,
    attributed: attributed?.ok ?? null, total: attributed?.tree?.attributes?.total ?? null, fields,
    concreteSyntax: attributed?.ok ? inspectLeaves(attributed.tree, input) : null,
    matchingFields: fields !== null && JSON.stringify(fields) === JSON.stringify(independent.fields) });
}
const accepted = rows.filter(row => row.independent.diagnostic === undefined);
const refused = rows.filter(row => row.independent.diagnostic !== undefined);
const truePositives = accepted.filter(row => row.attributed === true).length;
const falsePositives = refused.filter(row => row.attributed === true).length;
const falseNegatives = accepted.length - truePositives;
const totalMatches = accepted.filter(row => row.total === row.independent.total).length;
const fieldsMatch = accepted.filter(row => row.matchingFields).length;
const exactConcreteSyntax = accepted.filter(row => row.concreteSyntax?.reconstructs && row.concreteSyntax.spans).length;
const equivalent = deterministic && attributedParser && totalMatches === accepted.length
  && fieldsMatch === accepted.length && exactConcreteSyntax === accepted.length
  && rows.every(row => row.regularOracle === (row.independent.diagnostic === undefined)
    && row.syntax === row.regularOracle)
  && falseNegatives === 0 && falsePositives === 0;
const grade = (numerator, denominator) => denominator === 0 ? null : numerator / denominator;
const report = {
  method: 'finite native action candidate replay over successful instrumented parser traces',
  paperRevision: 'arXiv:2507.13117v1', oracleVersion, seed, sourceLimit, parserOptions,
  training, candidates: learned.candidates, diagnostic: learned.diagnostic, deterministic, grammar,
  grammarSha256: grammar && createHash('sha256').update(grammar).digest('hex'), grammarBytes: grammar && Buffer.byteLength(grammar),
  holdoutMetrics: attributedParser ? {
    truePositives, falsePositives, falseNegatives, totalMatches, fieldsMatch, exactConcreteSyntax,
    precision: grade(truePositives, truePositives + falsePositives), recall: grade(truePositives, accepted.length),
    f1: grade(2 * truePositives, 2 * truePositives + falsePositives + falseNegatives),
  } : null,
  elapsedMilliseconds: performance.now() - started, peakRssKib: process.resourceUsage().maxRSS,
  unsupported: ['exception semantics', 'actions outside the finite native vocabulary'], rows,
};
process.stdout.write(JSON.stringify(report, null, 2) + '\n');
if (!equivalent) process.exitCode = 1;
