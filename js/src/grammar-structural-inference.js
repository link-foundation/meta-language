import { LinkId } from './primitives.js';
import { Grammar } from './grammar.js';
import { createDiscoveredConceptLinks } from './discovered-concept-links.js';
import { encodeStructuredDataLinks, decodeStructuredDataLinks } from './structured-data-links.js';
import { createMetaLanguageArchive, readMetaLanguageArchive } from './meta-language-archive.js';
import { createSourceDefinitionProgramLinks } from './source-definition-catalog.js';
import { inferGrammarStructuralProductions } from './grammar-structural-inference-rules.js';

const algorithm = 'recursive grammar inference from labelled delimiter productions 1';
const modelIdentifier = 'structural-discovery-model';
const grammarIdentifier = 'structural-discovery-grammar';
const algorithmIdentifier = 'structural-discovery-algorithm';
const fail = message => { throw new TypeError('grammar structural inference: ' + message); };

function limit(value, maximum) {
  if (!Number.isSafeInteger(value) || value < 1 || value > maximum) fail('invalid resource limit');
  return value;
}

function sourceText(value) {
  return typeof value === 'string' && value.length > 0 && value.isWellFormed() && Buffer.byteLength(value) <= 4096;
}

function indexObservations(positives, negatives, delimiters, symbolLimit, byteLimit) {
  if (!Array.isArray(positives) || positives.length === 0 || !Array.isArray(negatives)
    || positives.length + negatives.length > symbolLimit) fail('expected bounded labelled examples with at least one positive');
  if (!Array.isArray(delimiters) || delimiters.length === 0 || delimiters.length > 32) fail('expected bounded delimiter pairs');
  const alphabet = [], indexes = new Map();
  let count = 0, bytes = 0;
  function encode(examples) {
    return Array.from(examples, example => {
      if (!Array.isArray(example) || example.length > symbolLimit - count) fail('sample symbol limit exceeded');
      count += example.length;
      return Array.from(example, token => {
        if (!sourceText(token)) fail('tokens must be nonempty Unicode text');
        bytes += Buffer.byteLength(token);
        if (bytes > byteLimit) fail('sample byte limit exceeded');
        if (!indexes.has(token)) {
          indexes.set(token, alphabet.length);
          alphabet.push(token);
        }
        return indexes.get(token);
      });
    });
  }
  const accepted = encode(positives), rejected = encode(negatives);
  const ordered = alphabet.toSorted();
  if (ordered.some((token, index) => ordered[index + 1]?.startsWith(token))) fail('overlapping token prefixes require a separately represented lexer');
  const openingGroups = alphabet.map(() => 0), closingGroups = alphabet.map(() => 0);
  for (const [index, pair] of delimiters.entries()) {
    if (!Array.isArray(pair) || pair.length !== 2) fail('a delimiter pair needs one opening and one closing token');
    const opening = indexes.get(pair[0]), closing = indexes.get(pair[1]);
    if (opening === undefined || closing === undefined || opening === closing
      || openingGroups[opening] || closingGroups[opening] || openingGroups[closing] || closingGroups[closing]) fail('delimiter pairs need distinct observed tokens');
    openingGroups[opening] = index + 1;
    closingGroups[closing] = index + 1;
  }
  return { alphabet, accepted, rejected, openingGroups, closingGroups };
}

function grammarFromProductions(productions, alphabet, delimiters) {
  const start = 'inferred_structure';
  const groupName = group => 'inferred_delimiter_group_' + group;
  const literal = value => ({ kind: 'literal', value });
  const alternatives = new Map();
  for (const [group, ...symbols] of productions) {
    const entries = alternatives.get(group) ?? [];
    const items = symbols.map(symbol => symbol < 0 ? { kind: 'ref', name: groupName(-symbol) } : literal(alphabet[symbol]));
    entries.push(items.length === 0 ? { kind: 'empty' } : items.length === 1 ? items[0] : { kind: 'seq', items });
    alternatives.set(group, entries);
  }
  const rules = new Map();
  for (const [group, entries] of [...alternatives].sort(([left], [right]) => left - right)) {
    const interior = entries.length === 1 ? entries[0] : { kind: 'choice', ordered: false, items: entries };
    const name = group === 0 ? start : groupName(group);
    const expression = group === 0 ? interior : { kind: 'seq', items: [
      literal(delimiters[group - 1][0]), interior, literal(delimiters[group - 1][1]),
    ] };
    rules.set(name, { name, kind: 'normal', expression });
  }
  return new Grammar(start, rules, 'inferred', { matching: 'generalized' });
}

const definition = () => createSourceDefinitionProgramLinks('meta-language/source-program/grammar-structural-inference-program');

/**
 * Infer recursive productions from labelled token examples and disjoint
 * delimiter pairs. All premises and the executable grammar travel together.
 */
export function inferStructuralGrammar(positives, negatives = [], options = {}) {
  const { delimiters, identifier = 'inferred-structure', language = 'Inferred language', edition = 'structural discovery 1' } = options;
  if (![identifier, language, edition].every(sourceText)) fail('invalid source identity');
  const limits = {
    depthLimit: limit(options.depthLimit ?? 32, 64),
    productionLimit: limit(options.productionLimit ?? 256, 512),
    symbolLimit: limit(options.symbolLimit ?? 2048, 4096),
    byteLimit: limit(options.byteLimit ?? 262144, 1048576),
  };
  const input = indexObservations(positives, negatives, delimiters, limits.symbolLimit, limits.byteLimit);
  const productions = inferGrammarStructuralProductions(input.accepted, input.rejected, input.openingGroups, input.closingGroups, limits.depthLimit, limits.productionLimit);
  const reason = productions[0][0];
  if (reason < 0) fail({
    [-1]: 'unbalanced delimiters in a positive example',
    [-2]: 'delimiter nesting limit exceeded',
    [-3]: 'production limit exceeded',
    [-4]: 'recursive production merge accepts a negative example',
  }[reason]);
  const grammar = grammarFromProductions(productions, input.alphabet, delimiters);
  const source = { id: identifier, language, edition, grammar };
  const model = { schemaVersion: 1, algorithm, identifier, language, edition, options: limits,
    delimiters: delimiters.map(pair => [...pair]), positives: positives.map(example => [...example]),
    negatives: negatives.map(example => [...example]), alphabet: input.alphabet, productions };
  const archive = createMetaLanguageArchive([
    { identifier: modelIdentifier, kind: 'grammar inference', ...encodeStructuredDataLinks(model) },
    { identifier: algorithmIdentifier, kind: 'typed program', ...definition() },
    { identifier: grammarIdentifier, kind: 'discovered concepts', ...createDiscoveredConceptLinks([source]) },
  ]);
  return { grammar, source, alphabet: input.alphabet, productions, ...archive };
}

/** Reload and check the grammar against the complete archived observations. */
export function readStructuralGrammarInferenceLinks(network, root) {
  const entries = readMetaLanguageArchive(network, root).entries;
  const premise = entries.find(entry => entry.identifier === modelIdentifier && entry.kind === 'grammar inference');
  if (!premise || entries.length !== 3) fail('invalid structural discovery checkpoint');
  const model = decodeStructuredDataLinks(premise.network, premise.root);
  if (model?.schemaVersion !== 1 || model.algorithm !== algorithm || !model.options) fail('unsupported structural discovery model');
  const result = inferStructuralGrammar(model.positives, model.negatives, { ...model.options, delimiters: model.delimiters,
    identifier: model.identifier, language: model.language, edition: model.edition });
  const expected = readMetaLanguageArchive(result.network, result.root).entries;
  for (const entry of expected) {
    const actual = entries.find(candidate => candidate.identifier === entry.identifier);
    if (actual?.kind !== entry.kind || !actual.root.equals(entry.root) || actual.network.toLino() !== entry.network.toLino())
      fail('checkpoint differs from its actual structural discovery model');
  }
  return { ...result, network, root: LinkId.from(root) };
}
