import { readFileSync } from 'node:fs';
import { isDeepStrictEqual } from 'node:util';
import { compileGrammar } from './grammar.js';
import { createDiscoveredConceptLinks, readDiscoveredConceptLinks } from './discovered-concept-links.js';
import { createMetaLanguageArchive, readMetaLanguageArchive } from './meta-language-archive.js';
import { decodeStructuredDataLinks, encodeStructuredDataLinks } from './structured-data-links.js';
import { LinkNetwork } from './network.js';
import { decodeTypedProgramLinks } from './typed-program-links.js';
import { recheckLinkedTypedProgram } from './typed-program-checked-projection.js';
import { acceptGrammarDiscoveryParserOracleAnswer, acceptGrammarDiscoveryParserOracleLabel,
  readGrammarDiscoveryParserOracleLimit as limit, readGrammarDiscoveryParserOracleTrainingClass } from './grammar-discovery-parser-oracle-rules.js';

const parserLimits = Object.freeze({ maxDepth: limit('nesting'), stepLimit: limit('steps'), memoLimit: limit('memo'), memoryLimit: limit('memory') });
export const grammarDiscoveryParserOracleIdentifier = 'regular-discovery-parser-oracle';

function createPolicyLinks() {
  const network = LinkNetwork.fromLino(readFileSync(new URL('./data/grammar-discovery-parser-oracle-program.lino', import.meta.url), 'utf8'));
  const roots = network.links().filter(link => link.metadata().language === 'meta-language typed program'
    && link.metadata().term === 'typed program' && link.metadata().definition === '1');
  if (roots.length !== 1) throw new TypeError('grammar discovery oracle needs one typed policy root');
  const root = roots[0].id();
  recheckLinkedTypedProgram(decodeTypedProgramLinks(network, root), { wholeProgram: true });
  return { network, root };
}

function acceptedTokenSequence(tree) {
  const pending = [tree], sequence = [];
  while (pending.length) {
    const node = pending.pop();
    if (!node || node.trivia) continue;
    if (node.type === 'node') {
      for (let i = node.children.length - 1; i >= 0; i--) pending.push(node.children[i]);
    } else if (node.type === 'token' && typeof node.kind === 'string') sequence.push(node.kind);
    else throw new TypeError('scanner oracle requires a complete named token sequence');
  }
  return sequence;
}

/** Validate declared labels and classify independent candidates with a native
 * grammar. Scanner queries use explicit source spellings and named CST leaves. */
export function observeGrammarDiscoveryParserOracle(grammar, positives, negatives, candidates = [], sources) {
  const groups = [positives, negatives, candidates];
  const scanner = sources !== undefined;
  const keys = ['positives', 'negatives', 'candidates'];
  if (groups.some(group => !Array.isArray(group)) || groups.reduce((sum, group) => sum + group.length, 0) > limit('queries')) {
    throw new RangeError('grammar discovery oracle requires at most 256 input words');
  }
  if (scanner && (!sources || typeof sources !== 'object' || Object.keys(sources).sort().join(',') !== 'candidates,negatives,positives'
    || keys.some((key, index) => !Array.isArray(sources[key]) || sources[key].length !== groups[index].length
      || sources[key].some(source => typeof source !== 'string' || !source.isWellFormed() || Buffer.byteLength(source) > limit('source bytes'))))) {
    throw new TypeError('scanner oracle needs one bounded source spelling per word');
  }
  for (const group of groups) for (const word of group) {
    if (!Array.isArray(word) || word.length > limit('symbols') || word.some(symbol =>
      typeof symbol !== 'string' || !symbol.isWellFormed() || (scanner ? !symbol.length : [...symbol].length !== 1))
      || (scanner && word.reduce((bytes, symbol) => bytes + Buffer.byteLength(symbol), 0) > limit('source bytes'))) {
      throw new TypeError(scanner ? 'scanner oracle words require bounded nonempty token names'
        : 'grammar discovery oracle words require at most 128 Unicode scalar symbols');
    }
  }
  const parser = compileGrammar(grammar, parserLimits);
  const queried = new Map();
  const answers = new Map();
  const observations = [];
  const declared = new Set([...positives, ...negatives].map(word => JSON.stringify(word)));
  const result = { positives: [...positives], negatives: [...negatives], observations };
  for (const [group, expected] of [1, 0, -1].entries()) {
    for (const [index, word] of groups[group].entries()) {
      const source = scanner ? sources[keys[group]][index] : word.join('');
      const identity = JSON.stringify(word);
      if (!queried.has(source)) {
        const outcome = parser.parseTree(source, scanner ? { ambiguity: 'reject' } : {});
        const reason = outcome.rejection?.reason ?? 'unknown';
        if (!acceptGrammarDiscoveryParserOracleAnswer(outcome.ok, reason)) {
          throw new TypeError('grammar discovery oracle could not classify input: ' + reason);
        }
        if (scanner && outcome.ok && !isDeepStrictEqual(acceptedTokenSequence(outcome.tree), word)) {
          throw new TypeError('scanner oracle token sequence differs from the accepted source');
        }
        queried.set(source, { identity, accepted: outcome.ok });
        observations.push({ word, accepted: outcome.ok, ...(scanner ? { source } : {}) });
      }
      const answer = queried.get(source);
      if (answer.identity !== identity) throw new TypeError('scanner oracle source has conflicting token sequences');
      if (!acceptGrammarDiscoveryParserOracleLabel(expected, answer.accepted)) {
        throw new TypeError('grammar discovery oracle disagrees with a supplied label');
      }
      if (answers.has(identity) && answers.get(identity) !== answer.accepted) {
        throw new TypeError('scanner oracle spellings for one token sequence disagree');
      }
      if (!answers.has(identity)) {
        answers.set(identity, answer.accepted);
        const selection = readGrammarDiscoveryParserOracleTrainingClass(declared.has(identity), answer.accepted);
        if (selection > 0) result.positives.push(word);
        else if (selection < 0) result.negatives.push(word);
      }
    }
  }
  return result;
}

export function createGrammarDiscoveryParserOracleEntry(grammar, positives, negatives, candidates, observations, sources) {
  const definition = createDiscoveredConceptLinks([{
    id: 'discovery-parser-oracle', language: 'Discovery parser oracle', edition: '1', grammar,
  }]);
  const premises = encodeStructuredDataLinks({ positives, negatives, candidates, observations, ...(sources === undefined ? {} : { sources }) });
  return { identifier: grammarDiscoveryParserOracleIdentifier, kind: 'grammar inference', ...createMetaLanguageArchive([
    { identifier: 'parser-oracle-definition', kind: 'discovered concepts', ...definition },
    { identifier: 'parser-oracle-policy', kind: 'typed program', ...createPolicyLinks() },
    { identifier: 'parser-oracle-premises', kind: 'grammar inference', ...premises },
  ]) };
}

export function readGrammarDiscoveryParserOracleEntry(entry, positives, negatives, exactTraining = false, scanner = false) {
  const invalid = () => new TypeError('invalid grammar discovery oracle checkpoint');
  if (entry.kind !== 'grammar inference') throw invalid();
  const { entries } = readMetaLanguageArchive(entry.network, entry.root);
  if (entries.length !== 3) throw invalid();
  const [definition, policy, premises] = entries;
  if (definition.identifier !== 'parser-oracle-definition' || definition.kind !== 'discovered concepts'
    || policy.identifier !== 'parser-oracle-policy' || policy.kind !== 'typed program'
    || premises.identifier !== 'parser-oracle-premises' || premises.kind !== 'grammar inference') throw invalid();
  const bundled = createPolicyLinks();
  if (!policy.root.equals(bundled.root) || policy.network.toLino() !== bundled.network.toLino()) throw invalid();
  const { sources } = readDiscoveredConceptLinks(definition.network, definition.root);
  const [source] = sources;
  if (sources.length !== 1 || source.id !== 'discovery-parser-oracle'
    || source.language !== 'Discovery parser oracle' || source.edition !== '1') throw invalid();
  const data = decodeStructuredDataLinks(premises.network, premises.root);
  const fields = scanner ? 'candidates,negatives,observations,positives,sources' : 'candidates,negatives,observations,positives';
  if (!data || typeof data !== 'object' || Object.keys(data).sort().join(',') !== fields
    || !Array.isArray(data.candidates)) throw invalid();
  const replayed = observeGrammarDiscoveryParserOracle(source.grammar, data.positives, data.negatives, data.candidates, data.sources);
  if (!isDeepStrictEqual(replayed.observations, data.observations)
    || (exactTraining && (replayed.positives.length !== positives.length || replayed.negatives.length !== negatives.length))
    || !isDeepStrictEqual(replayed.positives, positives.slice(0, replayed.positives.length))
    || !isDeepStrictEqual(replayed.negatives, negatives.slice(0, replayed.negatives.length))) throw invalid();
  return replayed.observations;
}
