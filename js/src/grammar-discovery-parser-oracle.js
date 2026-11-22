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

/** Validate discovery labels and classify independent candidates with an existing
 * native grammar. The input symbols are Unicode scalars, so concatenation into
 * the oracle source preserves the exact discovery word. */
export function observeGrammarDiscoveryParserOracle(grammar, positives, negatives, candidates = []) {
  const groups = [positives, negatives, candidates];
  if (groups.some(group => !Array.isArray(group)) || groups.reduce((sum, group) => sum + group.length, 0) > limit('queries')) {
    throw new RangeError('grammar discovery oracle requires at most 256 input words');
  }
  for (const group of groups) for (const word of group) {
    if (!Array.isArray(word) || word.length > limit('symbols') || word.some(symbol =>
      typeof symbol !== 'string' || !symbol.isWellFormed() || [...symbol].length !== 1)) {
      throw new TypeError('grammar discovery oracle words require at most 128 Unicode scalar symbols');
    }
  }
  const parser = compileGrammar(grammar, parserLimits);
  const queried = new Map();
  const observations = [];
  for (const [words, expected] of [[positives, 1], [negatives, 0], [candidates, -1]]) {
    for (const word of words) {
      const source = word.join('');
      if (!queried.has(source)) {
        const outcome = parser.parseTree(source);
        const reason = outcome.rejection?.reason ?? 'unknown';
        if (!acceptGrammarDiscoveryParserOracleAnswer(outcome.ok, reason)) {
          throw new TypeError('grammar discovery oracle could not classify input: ' + reason);
        }
        queried.set(source, outcome.ok);
        observations.push({ word, accepted: outcome.ok });
      }
      if (!acceptGrammarDiscoveryParserOracleLabel(expected, queried.get(source))) {
        throw new TypeError('grammar discovery oracle disagrees with a supplied label');
      }
    }
  }
  const declared = new Set([...positives, ...negatives].map(word => word.join('')));
  const result = { positives: [...positives], negatives: [...negatives], observations };
  for (const { word, accepted } of observations) {
    const selection = readGrammarDiscoveryParserOracleTrainingClass(declared.has(word.join('')), accepted);
    if (selection > 0) result.positives.push(word);
    else if (selection < 0) result.negatives.push(word);
  }
  return result;
}

export function createGrammarDiscoveryParserOracleEntry(grammar, positives, negatives, candidates, observations) {
  const definition = createDiscoveredConceptLinks([{
    id: 'discovery-parser-oracle', language: 'Discovery parser oracle', edition: '1', grammar,
  }]);
  const premises = encodeStructuredDataLinks({ positives, negatives, candidates, observations });
  return { identifier: grammarDiscoveryParserOracleIdentifier, kind: 'grammar inference', ...createMetaLanguageArchive([
    { identifier: 'parser-oracle-definition', kind: 'discovered concepts', ...definition },
    { identifier: 'parser-oracle-policy', kind: 'typed program', ...createPolicyLinks() },
    { identifier: 'parser-oracle-premises', kind: 'grammar inference', ...premises },
  ]) };
}

export function readGrammarDiscoveryParserOracleEntry(entry, positives, negatives) {
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
  if (!data || typeof data !== 'object' || Object.keys(data).sort().join(',') !== 'candidates,negatives,observations,positives'
    || !Array.isArray(data.candidates)) throw invalid();
  const replayed = observeGrammarDiscoveryParserOracle(source.grammar, data.positives, data.negatives, data.candidates);
  if (!isDeepStrictEqual(replayed.observations, data.observations)
    || !isDeepStrictEqual(replayed.positives, positives.slice(0, replayed.positives.length))
    || !isDeepStrictEqual(replayed.negatives, negatives.slice(0, replayed.negatives.length))) throw invalid();
  return replayed.observations;
}
