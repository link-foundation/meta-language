// Run on a CI runner once the full source draft is ready for verification.
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { performance } from 'node:perf_hooks';
import { createDynamicMetaLanguage } from '../../js/src/dynamic-meta-language.js';
import { inferRegularGrammar } from '../../js/src/grammar-inference.js';
import { renderGrammarLinks } from '../../js/src/grammar-links.js';

const corpusRevision = '1ef36fa01286573e846ac449e8683f8833c5b26a';
const inputLimit = 8192;
const learnerLimits = { stateLimit: 128, pairLimit: 8192, attemptLimit: 4096, symbolLimit: 2048, byteLimit: 16384 };
const parserLimits = { maxDepth: 192, stepLimit: 100_000, memoLimit: 4096, memoryLimit: 8 * 1024 * 1024 };
const limitedReasons = new Set(['stepLimit', 'memoryBudget', 'nestingDepth']);
const decoder = new TextDecoder('utf-8', { fatal: true, ignoreBOM: true });
const digest = value => createHash('sha256').update(value).digest('hex');
const positives = ['219037', 'false ', ' null\n', '"learned text 219037"', '[219037, false]', '{"training":219037}', '\t{"training":[219037]}'];
const negatives = ['219037x', 'False ', ' null\nx', '"learned text 219037', '[219037, false,]', '{"training" 219037}', '\t{"training":[219037}'];
const oracleSource = `import json, sys
def reject_constant(value):
    raise ValueError(value)
try:
    json.loads(sys.stdin.buffer.read().decode('utf-8'), parse_constant=reject_constant)
except (UnicodeError, ValueError) as error:
    print(error, file=sys.stderr)
    sys.exit(1)
except RecursionError as error:
    print(error, file=sys.stderr)
    sys.exit(2)
`;

function git(checkout, ...arguments_) {
  const result = spawnSync('git', ['-C', checkout, ...arguments_], { maxBuffer: 256 * 1024 });
  if (result.status !== 0) throw new Error(result.error?.message ?? result.stderr.toString());
  return result.stdout;
}

function readHoldout(checkout) {
  const directory = git(checkout, 'ls-tree', '-rlz', corpusRevision, '--', 'test_parsing').toString('utf8');
  return directory.split('\0').filter(Boolean).map(record => {
    const [, oid, size, path] = /^100644 blob ([0-9a-f]{40})\s+(\d+)\t(test_parsing\/[iny]_[^\t\/]+\.json)$/u.exec(record) ?? [];
    if (!path) throw new Error('uncovered corpus entry ' + JSON.stringify(record));
    const label = { y: 'accept', n: 'reject', i: 'implementation-defined' }[path.split('/')[1][0]];
    if (Number(size) > inputLimit) return { path, oid, label, size: Number(size), limit: 'input bytes' };
    const input = git(checkout, 'cat-file', 'blob', oid);
    if (input.length !== Number(size)) throw new Error('corpus length mismatch ' + path);
    return { path, oid, label, size: input.length, sha256: digest(input), input };
  });
}

function compareOracles(input) {
  let node = 'accept';
  let nodeDiagnostic;
  try {
    JSON.parse(decoder.decode(input));
  } catch (error) {
    if (!(error instanceof SyntaxError || error instanceof TypeError)) throw error;
    node = 'reject';
    nodeDiagnostic = error.message;
  }
  const python = spawnSync('python3', ['-I', '-c', oracleSource], { input, maxBuffer: 16384, timeout: 3000 });
  const status = { 0: 'accept', 1: 'reject', 2: 'nesting limit' }[python.status];
  return { node, nodeDiagnostic, python: status ?? 'oracle failure',
    pythonDiagnostic: python.error?.message ?? (python.stderr.toString() || undefined) };
}

function inspectTree(tree, input) {
  let end = 0, spans = tree.start === 0 && tree.end === input.length;
  let nodes = 0, fields = 0, trivia = 0;
  const pieces = [];
  const pending = [{ node: tree, parent: tree }];
  while (pending.length) {
    const { node, parent } = pending.pop();
    nodes += 1;
    if (node.field !== undefined) fields += 1;
    if (node.trivia) trivia += 1;
    spans &&= node.start >= parent.start && node.end <= parent.end && node.start <= node.end;
    if (node.type === 'token' || node.type === 'error') {
      const piece = node.text === null ? Buffer.from(node.hex, 'hex') : Buffer.from(node.text);
      spans &&= node.start === end && node.end - node.start === piece.length;
      end = node.end;
      pieces.push(piece);
    }
    for (const child of (node.children ?? node.sourceChildren ?? (node.root ? [node.root] : [])).toReversed()) pending.push({ node: child, parent: node });
  }
  return { nodes, fields, trivia, exactSpans: spans && end === input.length, exactBytes: Buffer.concat(pieces).equals(input) };
}

function measure(rows, learner, oracle) {
  const scored = rows.filter(row => row[learner] === 'accept' || row[learner] === 'reject')
    .filter(row => row[oracle] === 'accept' || row[oracle] === 'reject');
  const count = (prediction, expected) => scored.filter(row => row[learner] === prediction && row[oracle] === expected).length;
  const truePositive = count('accept', 'accept'), trueNegative = count('reject', 'reject');
  const falsePositive = count('accept', 'reject'), falseNegative = count('reject', 'accept');
  const ratio = (top, bottom) => bottom ? top / bottom : null;
  return { truePositive, trueNegative, falsePositive, falseNegative, scored: scored.length, unscored: rows.length - scored.length,
    precision: ratio(truePositive, truePositive + falsePositive), recall: ratio(truePositive, truePositive + falseNegative),
    f1: ratio(2 * truePositive, 2 * truePositive + falsePositive + falseNegative) };
}

const [checkout, strategy] = process.argv.slice(2);
if (process.argv.length !== 4 || !checkout || !['rpni', 'edsm', 'alergia'].includes(strategy)) throw new Error('usage: node json-holdout.mjs <JSONTestSuite checkout> <rpni|edsm|alergia>');
if (process.versions.node !== '24.14.0') throw new Error('uncovered Node oracle ' + process.versions.node);
const python = spawnSync('python3', ['--version'], { encoding: 'utf8' });
const pythonVersion = python.stdout?.trim();
if (python.status !== 0 || !/^Python 3\.12\.\d+$/u.test(pythonVersion ?? '')) throw new Error('uncovered Python oracle ' + pythonVersion);
const corpus = readHoldout(checkout);
if (!corpus.length) throw new Error('pinned holdout is empty');
const training = new Set([...positives, ...negatives].map(digest));
const infer = () => inferRegularGrammar(positives.map(value => [...value]), negatives.map(value => [...value]), { ...learnerLimits, strategy });
const started = performance.now();
const model = infer();
const trained = performance.now();
const grammar = renderGrammarLinks(model.grammar), archive = digest(model.network.toLino());
const deterministic = digest(infer().network.toLino()) === archive;
const parser = createDynamicMetaLanguage([model.source]).parser(model.source.id, parserLimits);
let learnerQueries = 0, oracleQueries = 0, parseMilliseconds = 0;
const rows = corpus.map(({ input, ...record }) => {
  if (!input) return record;
  if (training.has(record.sha256)) return { ...record, limit: 'identical training input' };
  oracleQueries += 2;
  const oracle = compareOracles(input);
  const before = performance.now();
  const result = parser.parseTree(input);
  parseMilliseconds += performance.now() - before;
  learnerQueries += 1;
  const reason = result.rejection?.reason;
  const learned = result.ok ? 'accept' : limitedReasons.has(reason) ? 'resource limit' : 'reject';
  const expected = [record.label, oracle.node, oracle.python];
  const disagreement = expected.some(value => (value === 'accept' || value === 'reject') && value !== learned);
  const concrete = result.tree ? inspectTree(result.tree, input) : undefined;
  return { ...record, ...oracle, learned, disagreement, rejection: result.rejection, ambiguities: result.ambiguities,
    concrete, treeSha256: result.tree ? digest(JSON.stringify(result.tree)) : undefined,
    tree: disagreement || (concrete && (!concrete.exactBytes || !concrete.exactSpans)) ? result.tree : undefined };
});
process.stdout.write(JSON.stringify({ schemaVersion: 1, corpusRevision, strategy, inputLimit, learnerLimits, parserLimits,
  oracles: { node: process.versions.node, v8: process.versions.v8, python: pythonVersion },
  training: { positives, negatives }, archive, deterministic, grammar, grammarRules: model.grammar.rules.size, grammarBytes: Buffer.byteLength(grammar),
  learnerQueries, oracleQueries, trainMilliseconds: trained - started, parseMilliseconds, peakLearnerProcessRssKiB: process.resourceUsage().maxRSS,
  acceptance: Object.fromEntries(['label', 'node', 'python'].map(oracle => [oracle, measure(rows, 'learned', oracle)])), rows }, null, 2) + '\n');
