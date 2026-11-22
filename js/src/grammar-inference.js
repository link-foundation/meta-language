// Native graph and grammar allocation around the shared discovery algorithm.
import { readFileSync } from 'node:fs';
import { isDeepStrictEqual } from 'node:util';
import { Grammar } from './grammar.js';
import { LinkNetwork } from './network.js';
import { LinkId } from './primitives.js';
import { encodeStructuredDataLinks, decodeStructuredDataLinks } from './structured-data-links.js';
import { createMetaLanguageArchive, extendMetaLanguageArchive, readMetaLanguageArchive } from './meta-language-archive.js';
import { observeGrammarDiscoveryParserOracle, createGrammarDiscoveryParserOracleEntry,
  readGrammarDiscoveryParserOracleEntry, grammarDiscoveryParserOracleIdentifier } from './grammar-discovery-parser-oracle.js';
import { createDiscoveredConceptLinks, readDiscoveredConceptLinks } from './discovered-concept-links.js';
import { decodeTypedProgramLinks } from './typed-program-links.js';
import { recheckLinkedTypedProgram } from './typed-program-checked-projection.js';
import { createGrammarScannerInferenceProgramLinks, grammarScannerInferenceArchiveIdentifier, inferGrammarScanner,
  createGrammarScannerCharacterClassProgramLinks, grammarScannerCharacterClassArchiveIdentifier, inferGrammarScannerClasses,
  createGrammarScannerModeStackProgramLinks, grammarScannerModeStackArchiveIdentifier, inferGrammarScannerModeStack } from './grammar-scanner-inference.js';
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

function samples(positives, negatives, options, scanner) {
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
  if (scanner) {
    const tokens = new Set(scanner.scanner.tokens);
    if ([...tokens].some(token => token.startsWith('inferred_state_'))) fail('scanner token name collides with the inferred rule namespace');
    if (alphabet.some(symbol => !tokens.has(symbol))) fail('sample contains a token missing from the scanner observations');
  } else {
    // Character samples keep their unique literal segmentation.
    for (let left = 0; left < alphabet.length; left += 1) for (let right = 0; right < alphabet.length; right += 1) {
      if (left !== right && alphabet[left].startsWith(alphabet[right])) fail('overlapping symbol prefixes require a separate lexer definition');
    }
  }
  return { alphabet, encoded };
}

function grammarFromTable(table, alphabet, scanner) {
  const states = [];
  for (let at = 0; at < table.length; at += 8) if (table[at] === 0 && findInferenceRepresentative(table, at) === at) states.push(at);
  const names = new Map(states.map((state, index) => [state, 'inferred_state_' + index]));
  const rules = new Map();
  for (const state of states) {
    const alternatives = [];
    if (table[state + 2] === 1 && table[state + 3] === 0) alternatives.push({ kind: 'empty' });
    for (let edge = 0; edge < table.length; edge += 8) if (table[edge] === 1 && table[edge + 1] === state) {
      const symbol = alphabet[table[edge + 2]];
      alternatives.push({ kind: 'seq', items: [scanner ? { kind: 'ref', name: symbol } : { kind: 'literal', value: symbol },
        { kind: 'ref', name: names.get(findInferenceRepresentative(table, table[edge + 3])) }] });
    }
    const name = names.get(state);
    rules.set(name, { name, kind: 'normal', expression: alternatives.length === 1 ? alternatives[0] : { kind: 'choice', ordered: false, items: alternatives } });
  }
  return new Grammar(names.get(findInferenceRepresentative(table, 0)), rules, 'inferred', {
    matching: 'generalized', ...(scanner ? { modes: scanner.modes, scanners: [scanner.scanner] } : {}),
  });
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
  const characterClasses = options.scannerCharacterClasses === undefined ? false : options.scannerCharacterClasses;
  const modeStacks = options.scannerModeStackObservations !== undefined;
  if (modeStacks && options.scannerObservations !== undefined) fail('select one scanner observation format');
  if (typeof characterClasses !== 'boolean' || (characterClasses && !modeStacks && options.scannerObservations === undefined)) fail('scanner character classes require scanner observations');
  const scanner = modeStacks ? inferGrammarScannerModeStack(options.scannerModeStackObservations, undefined, characterClasses)
    : options.scannerObservations === undefined ? undefined
    : (characterClasses ? inferGrammarScannerClasses : inferGrammarScanner)(options.scannerObservations);
  const limits = configuration(options), input = samples(positives, negatives, limits, scanner);
  const identifier = options.identifier ?? 'inferred-language', language = options.language ?? 'Inferred language', edition = options.edition ?? 'regular discovery 1';
  for (const value of [identifier, language, edition]) if (typeof value !== 'string' || !value || !value.isWellFormed() || Buffer.byteLength(value) > 4096) fail('invalid source identity');
  const table = inferRegularGrammarTable(input.encoded, input.alphabet.length, strategies.indexOf(limits.strategy), limits.alpha,
    limits.stateLimit, limits.pairLimit, limits.attemptLimit);
  if (table[0] < 0) fail(readInferenceFailureReason(table[0]));
  if (!acceptInferenceTableShape(table, input.alphabet.length)) fail('invalid model table');
  const grammar = grammarFromTable(table, input.alphabet, scanner);
  const source = { id: identifier, language, edition, grammar };
  const model = { schemaVersion: modeStacks ? 4 : characterClasses ? 3 : scanner ? 2 : 1, algorithm, identifier, language, edition, options: limits,
    positives, negatives, alphabet: input.alphabet, table,
    ...(modeStacks ? { scannerModeStackObservations: options.scannerModeStackObservations }
      : scanner ? { scannerObservations: options.scannerObservations } : {}),
    ...(characterClasses ? { scannerCharacterClasses: true } : {}) };
  const data = encodeStructuredDataLinks(model), definitions = createRegularGrammarInferenceProgramLinks();
  const discovered = createDiscoveredConceptLinks([source]);
  const archive = createMetaLanguageArchive([
    { identifier: 'regular-discovery-model', kind: 'grammar inference', ...data },
    { identifier: 'regular-discovery-algorithm', kind: 'typed program', ...definitions },
    { identifier: 'regular-discovery-grammar', kind: 'discovered concepts', ...discovered },
    ...(scanner ? [{ identifier: grammarScannerInferenceArchiveIdentifier, kind: 'typed program',
      ...createGrammarScannerInferenceProgramLinks() }] : []),
    ...(characterClasses ? [{ identifier: grammarScannerCharacterClassArchiveIdentifier, kind: 'typed program',
      ...createGrammarScannerCharacterClassProgramLinks() }] : []),
    ...(modeStacks ? [{ identifier: grammarScannerModeStackArchiveIdentifier, kind: 'typed program',
      ...createGrammarScannerModeStackProgramLinks() }] : []),
  ]);
  return { grammar, source, alphabet: [...input.alphabet], table: [...table], ...archive };
}

/** Discover a regular grammar with a native parser as a finite oracle. */
export function inferRegularGrammarWithParserOracle(grammar, positives, negatives, candidates = [], options = {}) {
  const { oracleSources } = options;
  const scanner = options.scannerObservations !== undefined || options.scannerModeStackObservations !== undefined;
  if (scanner !== (oracleSources !== undefined)) fail('native parser oracle discovery requires source spellings exactly for scanner input');
  const observed = observeGrammarDiscoveryParserOracle(grammar, positives, negatives, candidates, oracleSources);
  const result = inferRegularGrammar(observed.positives, observed.negatives, options);
  const entry = createGrammarDiscoveryParserOracleEntry(grammar, positives, negatives, candidates, observed.observations, oracleSources);
  return { ...result, ...extendMetaLanguageArchive(result, [entry]), oracleObservations: observed.observations };
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
  if (model?.algorithm !== algorithm || ![1, 2, 3, 4].includes(model.schemaVersion)) fail('unsupported model version');
  const modeStacks = model.schemaVersion === 4;
  const classes = model.scannerCharacterClasses === true;
  if (Object.hasOwn(model, 'scannerObservations') !== [2, 3].includes(model.schemaVersion)
    || Object.hasOwn(model, 'scannerModeStackObservations') !== modeStacks
    || Object.hasOwn(model, 'scannerCharacterClasses') !== classes
    || (model.schemaVersion === 3 && !classes) || (model.schemaVersion < 3 && classes)) fail('unsupported model version');
  const storedScanner = entries.find(entry => entry.identifier === grammarScannerInferenceArchiveIdentifier);
  const storedClasses = entries.find(entry => entry.identifier === grammarScannerCharacterClassArchiveIdentifier);
  if (classes) {
    const bundledClasses = createGrammarScannerCharacterClassProgramLinks();
    if (!storedClasses || storedClasses.kind !== 'typed program' || !storedClasses.root.equals(bundledClasses.root)
      || storedClasses.network.toLino() !== bundledClasses.network.toLino()) fail('the selected scanner character class definition differs from its versioned implementation');
  } else if (storedClasses) fail('the scanner character class definition needs its versioned model');
  const storedModeStacks = entries.find(entry => entry.identifier === grammarScannerModeStackArchiveIdentifier);
  if (modeStacks) {
    const bundledModeStacks = createGrammarScannerModeStackProgramLinks();
    if (!storedModeStacks || storedModeStacks.kind !== 'typed program' || !storedModeStacks.root.equals(bundledModeStacks.root)
      || storedModeStacks.network.toLino() !== bundledModeStacks.network.toLino()) fail('the selected scanner mode stack definition differs from its versioned implementation');
  } else if (storedModeStacks) fail('the scanner mode stack definition needs its versioned model');
  let scanner;
  if (model.schemaVersion >= 2) {
    const bundledScanner = createGrammarScannerInferenceProgramLinks();
    if (!storedScanner || storedScanner.kind !== 'typed program' || !storedScanner.root.equals(bundledScanner.root)
      || storedScanner.network.toLino() !== bundledScanner.network.toLino()) fail('the selected scanner definition differs from its versioned implementation');
    scanner = modeStacks ? inferGrammarScannerModeStack(model.scannerModeStackObservations, undefined, classes)
      : (classes ? inferGrammarScannerClasses : inferGrammarScanner)(model.scannerObservations);
  } else if (storedScanner) fail('the selected scanner definition needs its actual observations');
  const definitions = select('regular-discovery-algorithm', 'typed program'), bundled = createRegularGrammarInferenceProgramLinks();
  if (!definitions.root.equals(bundled.root) || definitions.network.toLino() !== bundled.network.toLino()) fail('the selected algorithm definition differs from its versioned implementation');
  const options = configuration(model.options), input = samples(model.positives, model.negatives, options, scanner);
  const table = inferRegularGrammarTable(input.encoded, input.alphabet.length, strategies.indexOf(options.strategy), options.alpha,
    options.stateLimit, options.pairLimit, options.attemptLimit);
  if (table[0] < 0) fail(readInferenceFailureReason(table[0]));
  if (!acceptInferenceTableShape(table, input.alphabet.length)) fail('invalid model table');
  if (!isDeepStrictEqual(input.alphabet, model.alphabet) || !isDeepStrictEqual(table, model.table)) fail('model decisions do not follow the stored observations and algorithm');
  const candidate = select('regular-discovery-grammar', 'discovered concepts');
  const decoded = readDiscoveredConceptLinks(candidate.network, candidate.root);
  const grammar = grammarFromTable(table, input.alphabet, scanner), source = decoded.sources[0];
  if (decoded.sources.length !== 1 || source.id !== model.identifier || source.language !== model.language || source.edition !== model.edition
    || !isDeepStrictEqual(source.grammar.normalized(), grammar.normalized())) fail('candidate grammar differs from its actual model');
  const oracle = entries.find(entry => entry.identifier === grammarDiscoveryParserOracleIdentifier);
  const oracleObservations = oracle ? readGrammarDiscoveryParserOracleEntry(oracle, model.positives, model.negatives, false, model.schemaVersion > 1) : undefined;
  return { grammar: source.grammar, source, alphabet: [...input.alphabet], table: [...table], network, root: LinkId.from(root),
    ...(oracleObservations ? { oracleObservations } : {}) };
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
