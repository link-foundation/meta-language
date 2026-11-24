import { Grammar, carryRuleDocs, compileGrammar } from './grammar.js';
import { parseGrammarLinks, percentEncodeLinksText } from './grammar-links.js';
import {
  readGrammarAttributeCandidateValues,
  readGrammarAttributeObservationByteLimit,
  readGrammarAttributeObservationLimit,
} from './grammar-attribute-inference-rules.js';

const fail = reason => { throw new TypeError('grammar attribute inference: ' + reason); };
const validName = value => typeof value === 'string' && value.length > 0
  && value.isWellFormed() && Buffer.byteLength(value) <= 4096;

function selectTraceNode(tree, { path, start, end }, rule) {
  let node = tree;
  for (const index of path) node = node?.children?.[index];
  if (node?.kind !== rule || node.start !== start || node.end !== end || node.trivia) return undefined;
  return node;
}

function actionGrammar(grammar, rule, attribute, candidate) {
  const definition = parseGrammarLinks('(grammar (start action))\n(rule action normal empty (action (setAttribute '
    + percentEncodeLinksText(attribute) + ' ' + candidate + ')))\n');
  const rules = new Map(grammar.rules);
  rules.set(rule, { ...rules.get(rule), action: definition.rule('action').action });
  const output = carryRuleDocs(new Grammar(grammar.start, rules, grammar.sourceFormat, grammar.declarations), grammar);
  output.kinds = [...grammar.kinds];
  return output;
}

/**
 * Search the native action vocabulary against instrumented parser observations.
 * Each path indexes actual children, including trivia, and must resolve to
 * the declared source rule and exact UTF-8 span. Ambiguous candidates stay
 * in the result so callers can select additional independent observations.
 */
export function inferGrammarAttributeAction(grammar, request, observations) {
  const { rule, attribute, field = null, childAttribute = null } = request;
  if (!validName(rule) || !validName(attribute) || field !== null && !validName(field)
    || childAttribute !== null && (field === null || !validName(childAttribute))) fail('invalid rule, attribute, or field selection');
  const owner = grammar.rule(rule);
  if (!owner || owner.action !== undefined) fail('expected an existing rule without an authored action');
  if (!Array.isArray(observations) || observations.length < 2
    || observations.length > readGrammarAttributeObservationLimit()) fail('expected bounded observations from at least two sources');
  const sources = new Set();
  let bytes = 0;
  const parser = compileGrammar(grammar);
  for (const row of observations) {
    if (!row || typeof row.source !== 'string' || !row.source.isWellFormed()
      || !Number.isSafeInteger(row.start) || !Number.isSafeInteger(row.end) || row.start < 0 || row.end < row.start
      || !Array.isArray(row.path) || row.path.length > 64
      || !row.path.every(index => Number.isSafeInteger(index) && index >= 0)
      || !(typeof row.value === 'string' && row.value.isWellFormed()
        || Number.isSafeInteger(row.value) && !Object.is(row.value, -0))) fail('invalid concrete trace observation');
    sources.add(row.source);
    bytes += Buffer.byteLength(row.source) + (typeof row.value === 'string' ? Buffer.byteLength(row.value) : 8);
    if (bytes > readGrammarAttributeObservationByteLimit()) fail('observation byte limit exceeded');
    const outcome = parser.parseTree(row.source);
    if (!outcome.ok || outcome.ambiguities.length > 0 || !selectTraceNode(outcome.tree, row, rule)) {
      fail('observation does not select an accepted unambiguous concrete rule occurrence');
    }
  }
  if (sources.size < 2) fail('expected bounded observations from at least two sources');
  const candidates = [];
  let selected = null;
  const encode = value => value === null ? '' : percentEncodeLinksText(value);
  for (const candidate of readGrammarAttributeCandidateValues(encode(field), encode(childAttribute))) {
    const graph = actionGrammar(grammar, rule, attribute, candidate);
    const candidateParser = compileGrammar(graph);
    const matches = observations.every(row => {
      const outcome = candidateParser.parseTree(row.source);
      if (!outcome.ok || outcome.ambiguities.length > 0) return false;
      const node = selectTraceNode(outcome.tree, row, rule);
      return node?.attributes && Object.hasOwn(node.attributes, attribute) && node.attributes[attribute] === row.value;
    });
    if (matches) {
      candidates.push(candidate);
      selected = graph;
    }
  }
  const diagnostic = candidates.length === 0 ? 'no native action in the selected vocabulary matches the observations'
    : candidates.length === 1 ? null : 'more observations are needed to distinguish the native actions';
  return { candidates, grammar: candidates.length === 1 ? selected : null, diagnostic };
}
