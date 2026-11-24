// Run on a CI runner once the full source draft is ready for verification.
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { closeSync, openSync, readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { performance } from 'node:perf_hooks';
import { createDynamicMetaLanguage } from '../../js/src/dynamic-meta-language.js';
import { inferRegularGrammar } from '../../js/src/grammar-inference.js';
import { renderGrammarLinks } from '../../js/src/grammar-links.js';

const corpusRevision = '1ef36fa01286573e846ac449e8683f8833c5b26a';
const supplementalCorpora = [
  { name: 'Crucio JSON benchmark', directory: 'gi-benchmarks', revision: 'aab9b09e1af68eed73fb07231b8e0bbd4870cdf2',
    license: null, paths: ['json/json-test'], pathPattern: /^json\/json-test\/test-\d+\.ex$/u, sampleLimit: 64 },
  { name: 'npm CLI manifests', directory: 'npm-cli', revision: '417daa72b09c5129e7390cd12743ef31bf3ddb83',
    license: 'Artistic-2.0', paths: ['package.json', 'package-lock.json', 'workspaces'],
    pathPattern: /^(package(-lock)?\.json|workspaces\/[^/]+\/package\.json)$/u, sampleLimit: 32 },
  { name: 'TypeScript JSON sources', directory: 'typescript', revision: 'c63de15a992d37f0d6cec03ac7631872838602cb',
    license: 'Apache-2.0', paths: ['package.json', 'lib/typesMap.json', 'src/compiler/diagnosticMessages.json'],
    pathPattern: /^(package\.json|lib\/typesMap\.json|src\/compiler\/diagnosticMessages\.json)$/u, sampleLimit: 3 },
];
const externalLearners = new Map([
  ['xvada', { revision: '5221b183bc6e1ac583505299173fdbae0706da99', paper: 'arXiv:2607.08959v4',
    mode: 'model-free TreeVada with HDD', lark: '1.2.2', license: 'MIT' }],
  ['natgi', { revision: '561f35a9bc9fa02c62de6d2ebfbc6266357fe414', paper: 'arXiv:2509.26616v2',
    mode: 'model-free bracket-guided trees with HDD', lark: '1.2.2', license: 'MIT' }],
  ['crucio', { revision: 'ce913637763f0b25017d38e0e01944f54cf86d38', paper: 'arXiv:2601.12385v1',
    mode: 'decomposition forest with distributional inference', lark: '1.1.9', license: null }],
]);
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
  const result = spawnSync('git', ['-C', checkout, ...arguments_], { maxBuffer: 2 * 1024 * 1024 });
  if (result.status !== 0) throw new Error(result.error?.message ?? result.stderr.toString());
  return result.stdout;
}

function readHoldout(checkout) {
  const directory = git(checkout, 'ls-tree', '-rlz', corpusRevision, '--', 'test_parsing').toString('utf8');
  return directory.split('\0').filter(Boolean).map(record => {
    const [, oid, size, path] = /^100(?:644|755) blob ([0-9a-f]{40})\s+(\d+)\t(test_parsing\/[iny]_[^\t\/]+\.json)$/u.exec(record) ?? [];
    if (!path) throw new Error('uncovered corpus entry ' + JSON.stringify(record));
    const label = { y: 'accept', n: 'reject', i: 'implementation-defined' }[path.split('/')[1][0]];
    const row = { corpus: 'JSONTestSuite', path, oid, label, size: Number(size) };
    return row.size > inputLimit ? { ...row, limit: 'input bytes' } : readCorpusInput(checkout, row);
  });
}

function readCorpusInput(checkout, row) {
  const input = git(checkout, 'cat-file', 'blob', row.oid);
  if (input.length !== row.size) throw new Error('corpus length mismatch ' + row.path);
  return { ...row, sha256: digest(input), input };
}

function readSupplementalCorpus(root, source) {
  const checkout = root + '/' + source.directory;
  const tree = git(checkout, 'ls-tree', '-rlz', source.revision, '--', ...source.paths).toString('utf8');
  const rows = tree.split('\0').filter(Boolean).flatMap(record => {
    const [, oid, size, path] = /^100(?:644|755) blob ([0-9a-f]{40})\s+(\d+)\t([^\t]+)$/u.exec(record) ?? [];
    if (!path || !source.pathPattern.test(path)) return [];
    return [{ corpus: source.name, path: source.directory + '/' + path, oid, size: Number(size), label: 'unlabelled' }];
  });
  if (rows.length === 0) throw new Error('empty pinned corpus ' + source.name);
  rows.sort((a, b) => b.size - a.size || (a.path < b.path ? -1 : 1));
  let selected = 0;
  return rows.map(row => {
    if (row.size > inputLimit) return { ...row, limit: 'input bytes' };
    if (selected === source.sampleLimit) return { ...row, limit: 'sample count' };
    selected += 1;
    return readCorpusInput(checkout, row);
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

function measureExternalLearner(strategy, checkout, tracePath, candidates) {
  const { revision, mode, lark } = externalLearners.get(strategy);
  if (git(checkout, 'rev-parse', 'HEAD').toString().trim() !== revision) throw new Error('uncovered ' + strategy + ' revision');
  if (git(checkout, 'status', '--porcelain', '--untracked-files=all').length !== 0) {
    throw new Error(strategy + ' inputs must match the pinned checkout');
  }
  const budget = { seed: 101, inputLimit, memoryBytes: 512 * 1024 * 1024, queryLimit: 4096, trainingSeconds: 90, parseSeconds: 2,
    evaluationSeconds: 90 };
  const holdout = candidates.map(({ path, sha256, input }) => ({ path, sha256, bytes: input.toString('base64') }));
  const pythonScript = fileURLToPath(new URL('./json-research-learner.py', import.meta.url));
  const invoke = trace => {
    const logPath = trace + '.log';
    const log = openSync(logPath, 'wx');
    let result;
    try {
      result = spawnSync('python3', ['-s', '-B', pythonScript, checkout, trace], {
        input: JSON.stringify({ strategy, ...budget, positives, holdout }), encoding: 'utf8', maxBuffer: 8 * 1024 * 1024,
        stdio: ['pipe', 'pipe', log],
        env: { ...process.env, PYTHONHASHSEED: String(budget.seed), OMP_NUM_THREADS: '1', OPENBLAS_NUM_THREADS: '1' },
      });
    } finally {
      closeSync(log);
    }
    if (result.status !== 0 && result.status !== 75) {
      const reason = result.error?.message ?? strategy + ' process exited with ' + (result.signal ?? result.status);
      throw new Error(reason + '. See ' + logPath);
    }
    const measurement = JSON.parse(result.stdout);
    if (measurement.larkVersion !== lark) throw new Error('uncovered ' + strategy + ' Lark version ' + measurement.larkVersion);
    if ((result.status === 75) !== Boolean(measurement.trainingLimit)) throw new Error('inconsistent ' + strategy + ' training outcome');
    return measurement;
  };
  const { holdout: learned, ...first } = invoke(tracePath);
  const repeated = invoke(tracePath + '.repeat');
  const trace = readFileSync(tracePath), repeatedQueries = readFileSync(tracePath + '.repeat');
  const firstTrace = digest(trace), repeatedTrace = digest(repeatedQueries);
  const trainingQueries = new Set([trace, repeatedQueries].flatMap(bytes =>
    bytes.toString('utf8').split('\n').filter(Boolean).map(line => JSON.parse(line).sha256)));
  const provenance = { budget, archive: null, learnerMode: mode, firstTrace, repeatedTrace };
  if (first.trainingLimit) {
    return { ...first, ...provenance, trainingQueries, grammar: null, deterministic: null, repeatedTrainingLimit: repeated.trainingLimit,
      outcomes: Object.fromEntries(candidates.map(({ path }) =>
        [path, { learned: 'unavailable', learnerDiagnostic: first.trainingLimit }])) };
  }
  const learnedPaths = new Set(Object.keys(learned));
  if (learnedPaths.size !== candidates.length || candidates.some(row => !learnedPaths.has(row.path))) {
    throw new Error(strategy + ' omitted an eligible holdout input');
  }
  return { ...first, ...provenance, trainingQueries, outcomes: learned,
    deterministic: firstTrace === repeatedTrace && first.grammar === repeated.grammar
      && JSON.stringify(learned) === JSON.stringify(repeated.holdout) };
}

const [checkout, corpusRoot, strategy, externalCheckout, tracePath] = process.argv.slice(2);
const external = externalLearners.get(strategy);
if (!checkout || !corpusRoot || (!external && !['rpni', 'edsm', 'alergia'].includes(strategy))
  || process.argv.length !== (external ? 7 : 5) || (external && (!externalCheckout || !tracePath))) {
  throw new Error('usage: node json-holdout.mjs <JSONTestSuite checkout> <supplemental corpus root> <rpni|edsm|alergia|xvada|natgi|crucio> [<learner checkout> <training trace.jsonl>]');
}
if (process.versions.node !== '24.14.0') throw new Error('uncovered Node oracle ' + process.versions.node);
const python = spawnSync('python3', ['--version'], { encoding: 'utf8' });
const pythonVersion = python.stdout?.trim();
if (python.status !== 0 || !/^Python 3\.12\.\d+$/u.test(pythonVersion ?? '')) throw new Error('uncovered Python oracle ' + pythonVersion);
const corpus = [...readHoldout(checkout), ...supplementalCorpora.flatMap(source => readSupplementalCorpus(corpusRoot, source))];
if (!corpus.length) throw new Error('pinned holdout is empty');
const training = new Set([...positives, ...negatives].map(digest));
const eligible = record => record.input && !training.has(record.sha256);
function measureLocalLearner() {
  const infer = () => inferRegularGrammar(positives.map(value => [...value]), negatives.map(value => [...value]), { ...learnerLimits, strategy });
  const started = performance.now();
  const model = infer();
  const trainMilliseconds = performance.now() - started;
  const grammar = renderGrammarLinks(model.grammar), archive = digest(model.network.toLino());
  return { grammar, archive, trainMilliseconds, grammarRules: model.grammar.rules.size, deterministic: digest(infer().network.toLino()) === archive,
    parser: createDynamicMetaLanguage([model.source]).parser(model.source.id, parserLimits) };
}
const { parser, outcomes, trainingQueries, ...learner } = external
  ? measureExternalLearner(strategy, externalCheckout, tracePath, corpus.filter(eligible)) : measureLocalLearner();
let learnerQueries = 0, oracleQueries = 0, parseMilliseconds = 0;
const rows = corpus.map(({ input, ...record }) => {
  if (!input) return record;
  if (training.has(record.sha256)) return { ...record, limit: 'identical training input' };
  oracleQueries += 2;
  const oracle = compareOracles(input);
  const expected = [record.label, oracle.node, oracle.python];
  const disagrees = learned => expected.some(value => (value === 'accept' || value === 'reject') && value !== learned);
  if (outcomes) {
    const outcome = outcomes[record.path];
    if (trainingQueries.has(record.sha256)) return { ...record, ...oracle, limit: 'identical membership query', outcome };
    if (outcome.learned !== 'unavailable' && outcome.attempted !== false) learnerQueries += 1;
    return { ...record, ...oracle, ...outcome, disagreement: disagrees(outcome.learned) };
  }
  learnerQueries += 1;
  const before = performance.now();
  const result = parser.parseTree(input);
  parseMilliseconds += performance.now() - before;
  const reason = result.rejection?.reason;
  const learned = result.ok ? 'accept' : limitedReasons.has(reason) ? 'resource limit' : 'reject';
  const disagreement = disagrees(learned);
  const concrete = result.tree ? inspectTree(result.tree, input) : undefined;
  return { ...record, ...oracle, learned, disagreement, rejection: result.rejection, ambiguities: result.ambiguities,
    concrete, treeSha256: result.tree ? digest(JSON.stringify(result.tree)) : undefined,
    tree: disagreement || (concrete && (!concrete.exactBytes || !concrete.exactSpans)) ? result.tree : undefined };
});
const score = sourceRows => Object.fromEntries(['label', 'node', 'python'].map(oracle =>
  [oracle, measure(sourceRows, 'learned', oracle)]));
process.stdout.write(JSON.stringify({ schemaVersion: 2, corpusRevision, strategy, inputLimit,
  supplementalCorpora: supplementalCorpora.map(({ pathPattern, ...source }) => ({ ...source, pathPattern: pathPattern.source })),
  oracles: { node: process.versions.node, v8: process.versions.v8, python: pythonVersion },
  training: { positives, negatives: external ? [] : negatives }, ...learner,
  grammarBytes: learner.grammar === null ? null : Buffer.byteLength(learner.grammar),
  ...(external ? { learnerRevision: external.revision, learnerPaper: external.paper, learnerLicense: external.license }
    : { learnerLimits, parserLimits, parseMilliseconds, peakLearnerProcessRssKiB: process.resourceUsage().maxRSS }),
  learnerQueries, oracleQueries,
  acceptance: score(rows),
  acceptanceByCorpus: Object.fromEntries(['JSONTestSuite', ...supplementalCorpora.map(source => source.name)]
    .map(source => [source, score(rows.filter(row => row.corpus === source))])),
  rows }, null, 2) + '\n');
