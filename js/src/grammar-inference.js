// Native graph and grammar allocation around the shared discovery algorithm.
import { readFileSync } from 'node:fs';
import { isDeepStrictEqual } from 'node:util';
import { Grammar } from './grammar.js';
import { LinkNetwork } from './network.js';
import { LinkId } from './primitives.js';
import { encodeStructuredDataLinks, decodeStructuredDataLinks } from './structured-data-links.js';
import { createMetaLanguageArchive, readMetaLanguageArchive } from './meta-language-archive.js';
import { createDiscoveredConceptLinks, readDiscoveredConceptLinks } from './discovered-concept-links.js';
import { decodeTypedProgramLinks } from './typed-program-links.js';
import { recheckLinkedTypedProgram } from './typed-program-checked-projection.js';
import { inferRegularGrammarTable, findInferenceRepresentative,
  acceptInferredSymbolSequence, acceptInferenceTableShape, readInferenceFailureReason } from './grammar-inference-rules.js';

const algorithm = 'labelled regular grammar discovery 1';
const fail = message => { throw new TypeError('grammar inference: ' + message); };
const strategies = ['rpni', 'edsm', 'alergia'];

function configuration(options) {
  const result = { strategy: options.strategy ?? 'rpni', alpha: options.alpha ?? 0.05,
    stateLimit: options.stateLimit ?? 128, pairLimit: options.pairLimit ?? 8192,
    attemptLimit: options.attemptLimit ?? 4096, symbolLimit: options.symbolLimit ?? 2048,
    byteLimit: options.byteLimit ?? 4 * 1024 * 1024 };
  if (!strategies.includes(result.strategy) || !Number.isFinite(result.alpha) || result.alpha <= 0 || result.alpha >= 1) fail('invalid strategy or confidence');
  for (const [key, maximum] of [['stateLimit', 512], ['pairLimit', 32768], ['attemptLimit', 32768], ['symbolLimit', 16384], ['byteLimit', 16 * 1024 * 1024]]) {
    if (!Number.isSafeInteger(result[key]) || result[key] < 1 || result[key] > maximum) fail('invalid ' + key);
  }
  return result;
}

function samples(positives, negatives, options) {
  if (!Array.isArray(positives) || !Array.isArray(negatives) || positives.length + negatives.length > options.symbolLimit) fail('expected bounded labelled example lists');
  const alphabet = [], indexes = new Map(), encoded = [];
  let count = 0, bytes = 0;
  for (const [list, label] of [[positives, 1], [negatives, 0]]) for (const example of list) {
    if (!Array.isArray(example) || example.length > options.symbolLimit - count) fail('sample symbol limit exceeded');
    count += example.length; encoded.push(label, example.length);
    for (const symbol of example) {
      if (typeof symbol !== 'string' || symbol.length === 0 || !symbol.isWellFormed()) fail('symbols must be nonempty Unicode text');
      bytes += Buffer.byteLength(symbol);
      if (bytes > options.byteLimit) fail('sample byte limit exceeded');
      if (!indexes.has(symbol)) { indexes.set(symbol, alphabet.length); alphabet.push(symbol); }
      encoded.push(indexes.get(symbol));
    }
  }
  // Without a separately represented lexer, the grammar must retain a unique
  // symbol segmentation. Character samples naturally meet this condition.
  for (let left = 0; left < alphabet.length; left += 1) for (let right = 0; right < alphabet.length; right += 1) {
    if (left !== right && alphabet[left].startsWith(alphabet[right])) fail('overlapping symbol prefixes require a separate lexer definition');
  }
  return { alphabet, encoded };
}

function grammarFromTable(table, alphabet) {
  const states = [];
  for (let at = 0; at < table.length; at += 8) if (table[at] === 0 && findInferenceRepresentative(table, at) === at) states.push(at);
  const names = new Map(states.map((state, index) => [state, 'inferred_state_' + index]));
  const rules = new Map();
  for (const state of states) {
    const alternatives = [];
    if (table[state + 2] === 1 && table[state + 3] === 0) alternatives.push({ kind: 'empty' });
    for (let edge = 0; edge < table.length; edge += 8) if (table[edge] === 1 && table[edge + 1] === state) {
      alternatives.push({ kind: 'seq', items: [{ kind: 'literal', value: alphabet[table[edge + 2]] },
        { kind: 'ref', name: names.get(findInferenceRepresentative(table, table[edge + 3])) }] });
    }
    const name = names.get(state);
    rules.set(name, { name, kind: 'normal', expression: alternatives.length === 1 ? alternatives[0] : { kind: 'choice', ordered: false, items: alternatives } });
  }
  return new Grammar(names.get(findInferenceRepresentative(table, 0)), rules, 'inferred', { matching: 'generalized' });
}

/** Complete editable typed algorithm definition; this is not execution evidence. */
export function createRegularGrammarInferenceProgramLinks() {
  const network = LinkNetwork.fromLino(readFileSync(new URL('./data/grammar-inference-program.lino', import.meta.url), 'utf8'));
  const roots = network.links().filter(link => link.metadata().language === 'meta-language typed program'
    && link.metadata().term === 'typed program' && link.metadata().definition === '1');
  if (roots.length !== 1) fail('the algorithm needs one actual typed program root');
  const root = roots[0].id();
  recheckLinkedTypedProgram(decodeTypedProgramLinks(network, root), { wholeProgram: true });
  return { network, root };
}

/** Discover a right-linear grammar from declared token observations. No finite
 * sample is promoted to a proof of the unknown language's complete meaning.
 */
export function inferRegularGrammar(positives, negatives = [], options = {}) {
  const limits = configuration(options), input = samples(positives, negatives, limits);
  const identifier = options.identifier ?? 'inferred-language', language = options.language ?? 'Inferred language', edition = options.edition ?? 'regular discovery 1';
  for (const value of [identifier, language, edition]) if (typeof value !== 'string' || !value || !value.isWellFormed() || Buffer.byteLength(value) > 4096) fail('invalid source identity');
  const table = inferRegularGrammarTable(input.encoded, input.alphabet.length, strategies.indexOf(limits.strategy), limits.alpha,
    limits.stateLimit, limits.pairLimit, limits.attemptLimit);
  if (table[0] < 0) fail(readInferenceFailureReason(table[0]));
  if (!acceptInferenceTableShape(table, input.alphabet.length)) fail('invalid model table');
  const grammar = grammarFromTable(table, input.alphabet);
  const source = { id: identifier, language, edition, grammar };
  const model = { schemaVersion: 1, algorithm, identifier, language, edition, options: limits,
    positives, negatives, alphabet: input.alphabet, table };
  const data = encodeStructuredDataLinks(model), definitions = createRegularGrammarInferenceProgramLinks();
  const discovered = createDiscoveredConceptLinks([source]);
  const archive = createMetaLanguageArchive([
    { identifier: 'regular-discovery-model', kind: 'grammar inference', ...data },
    { identifier: 'regular-discovery-algorithm', kind: 'typed program', ...definitions },
    { identifier: 'regular-discovery-grammar', kind: 'discovered concepts', ...discovered },
  ]);
  return { grammar, source, alphabet: [...input.alphabet], table: [...table], ...archive };
}

/** Reload the complete algorithm, model, recursive grammar and concept graphs.
 * Recompute the selected version's decisions; stored declarations are never
 * trusted as proof or represented as independent observed execution evidence.
 */
export function readRegularGrammarInferenceLinks(network, root) {
  const entries = readMetaLanguageArchive(network, root).entries;
  const select = (identifier, kind) => {
    const entry = entries.find(value => value.identifier === identifier);
    if (!entry || entry.kind !== kind) fail('missing ' + identifier);
    return entry;
  };
  const data = select('regular-discovery-model', 'grammar inference');
  const model = decodeStructuredDataLinks(data.network, data.root);
  if (model?.schemaVersion !== 1 || model.algorithm !== algorithm) fail('unsupported model version');
  const definitions = select('regular-discovery-algorithm', 'typed program'), bundled = createRegularGrammarInferenceProgramLinks();
  if (!definitions.root.equals(bundled.root) || definitions.network.toLino() !== bundled.network.toLino()) fail('the selected algorithm definition differs from its versioned implementation');
  const options = configuration(model.options), input = samples(model.positives, model.negatives, options);
  const table = inferRegularGrammarTable(input.encoded, input.alphabet.length, strategies.indexOf(options.strategy), options.alpha,
    options.stateLimit, options.pairLimit, options.attemptLimit);
  if (table[0] < 0) fail(readInferenceFailureReason(table[0]));
  if (!acceptInferenceTableShape(table, input.alphabet.length)) fail('invalid model table');
  if (!isDeepStrictEqual(input.alphabet, model.alphabet) || !isDeepStrictEqual(table, model.table)) fail('model decisions do not follow the stored observations and algorithm');
  const candidate = select('regular-discovery-grammar', 'discovered concepts');
  const decoded = readDiscoveredConceptLinks(candidate.network, candidate.root);
  const grammar = grammarFromTable(table, input.alphabet), source = decoded.sources[0];
  if (decoded.sources.length !== 1 || source.id !== model.identifier || source.language !== model.language || source.edition !== model.edition
    || !isDeepStrictEqual(source.grammar.normalized(), grammar.normalized())) fail('candidate grammar differs from its actual model');
  return { grammar: source.grammar, source, alphabet: [...input.alphabet], table: [...table], network, root: LinkId.from(root) };
}

/** Execute the actual inferred automaton over an independently supplied token
 * sequence. Unknown alphabet members reject, including empty text symbols.
 */
export function acceptInferredGrammarSymbols(model, symbols) {
  if (!Array.isArray(symbols) || symbols.length > 16384) fail('input symbol limit exceeded');
  if (!Array.isArray(model?.alphabet) || !Array.isArray(model?.table) || model.alphabet.length > 16384
    || model.alphabet.some(symbol => typeof symbol !== 'string' || !symbol || !symbol.isWellFormed())
    || new Set(model.alphabet).size !== model.alphabet.length
    || !acceptInferenceTableShape(model.table, model.alphabet.length)) fail('invalid model table');
  const encoded = symbols.map(symbol => model.alphabet.indexOf(symbol));
  if (encoded.some(value => value < 0)) return false;
  return acceptInferredSymbolSequence(model.table, encoded);
}
