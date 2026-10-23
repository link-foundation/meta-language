// Runtime extension uses the same compiler, concept verifier and archived
// formal premises as prepared grammars. No host/child language pair table.
import { compileGrammar } from './grammar.js';
import { createDiscoveredConceptLinks, readDiscoveredConceptLinks } from './discovered-concept-links.js';
import { createMetaLanguageArchive, readMetaLanguageArchive,
  extendMetaLanguageArchive } from './meta-language-archive.js';
import { encodeStructuredDataLinks, decodeStructuredDataLinks } from './structured-data-links.js';
import { checkGrammarSelectionMappedTree } from './grammar-runtime.js';
import { LinkNetwork } from './network.js';
import { LinkId, LinkMetadata, LinkFlags, SourceSpan, ByteRange, Point } from './primitives.js';
import { createHash } from 'node:crypto';
import { readDynamicMetaLanguageIdentifier, acceptDynamicSyntaxProjection,
  readDynamicConcreteSyntaxVersion, readDynamicParserOptionOrder,
  acceptDynamicParserOption, readDynamicSnapshotIdentifier,
  readDynamicConcreteIdentifier } from './dynamic-meta-language-rules.js';
import { decoratorSet } from './decorators.js';

const discoveryIdentifier = readDynamicMetaLanguageIdentifier('definitions');
const configurationIdentifier = readDynamicMetaLanguageIdentifier('configuration');
const fail = message => { throw new TypeError(`dynamic meta-language: ${message}`); };

function dynamicLimits(options) {
  const limits = { nodeLimit: options.nodeLimit ?? 100000, referenceLimit: options.referenceLimit ?? 1000000,
    byteLimit: options.byteLimit ?? 128 * 1024 * 1024, depthLimit: options.depthLimit ?? 256 };
  if (Object.values(limits).some(value => !Number.isSafeInteger(value) || value < 1)) fail('invalid resource limit');
  return limits;
}

// Exported views own their IDs and metadata objects. Parser caches derive only
// from the private linked snapshot; editing an exported graph is explicit via
// a new session or extend(), never an unvalidated host-table mutation.
function copyNetwork(source, limits = {}) {
  if (!(source instanceof LinkNetwork) || source._links.size > (limits.nodeLimit ?? 100000)) fail('source network node limit exceeded');
  const network = new LinkNetwork();
  let references = 0;
  for (const link of source.links()) {
    const targets = link.references();
    references += targets.length;
    if (references > (limits.referenceLimit ?? 1000000)) fail('source network reference limit exceeded');
    const value = link.metadata(), span = value.span;
    const metadata = new LinkMetadata({ ...value, flags: new LinkFlags({ ...value.flags }),
      span: span ? new SourceSpan(new ByteRange(span.byteRange.start, span.byteRange.end),
        new Point(span.start.row, span.start.column), new Point(span.end.row, span.end.column)) : undefined });
    network.insertLinkWithOptionalId(new LinkId(link.id().asU64()),
      targets.map(id => new LinkId(id.asU64())), metadata);
  }
  network._nextId = source._nextId;
  network._terms = new Map([...source._terms].map(([term, id]) => [term, new LinkId(LinkId.from(id).asU64())]));
  network._conceptSyntax = new Map(source._conceptSyntax);
  return network;
}

function graph(value, options) {
  const encoded = encodeStructuredDataLinks(value, options), network = new LinkNetwork();
  for (const link of encoded.network.links()) network.insertLinkWithOptionalId(link.id(), link.references(), link.metadata().clone());
  return { network, root: encoded.root };
}

function island(network, root, options) {
  const selected = new LinkNetwork(), pending = [root], seen = new Set();
  for (let index = 0; index < pending.length; index += 1) {
    const id = LinkId.from(pending[index]).asU64();
    if (seen.has(id)) continue;
    if (seen.size >= (options.nodeLimit ?? 100000)) fail('context node limit exceeded');
    const link = network.link(id);
    if (!link) fail('dangling context reference');
    seen.add(id);
    selected.insertLinkWithOptionalId(id, link.references(), link.metadata().clone());
    for (const target of link.references()) pending.push(target);
  }
  return selected;
}

function contextFingerprint(entry, options) {
  const selected = island(entry.network, entry.root, options);
  return createHash('sha256').update(String(LinkId.from(entry.root).asU64()) + '\0')
    .update(selected.toLino()).digest('hex');
}

// Snapshot aliases share an actual source root. Ordinals are directory names,
// never evidence for merging independent graphs or equal concept meanings.
function pinContext(entries, entry, role) {
  const prefix = readDynamicMetaLanguageIdentifier(role);
  const existing = entries.find(value => value.identifier.startsWith(prefix)
    && value.network === entry.network && LinkId.from(value.root).equals(entry.root));
  if (existing) return existing.identifier;
  const identifier = readDynamicSnapshotIdentifier(entries.map(entry => entry.identifier), prefix);
  entries.push({ ...entry, identifier });
  return identifier;
}

function checkSnapshotContexts(entries, options) {
  const directory = new Map(entries.map(entry => [entry.identifier, entry])), checked = new Map();
  for (const entry of entries) {
    if (entry.kind !== 'concrete syntax' || entry.network.link(entry.root)?.metadata().language !== 'meta-language structured data') continue;
    const data = decodeStructuredDataLinks(island(entry.network, entry.root, options), entry.root, options);
    if (!data || typeof data !== 'object' || data.schemaVersion !== readDynamicConcreteSyntaxVersion() || data.projection !== 'concrete syntax') continue;
    for (const [field, role, kind] of [['definitionContext', 'definition snapshot', 'discovered concepts'],
      ['configurationContext', 'configuration snapshot', 'language configuration']]) {
      const identifier = data[field], selected = directory.get(identifier);
      if (typeof identifier !== 'string' || !identifier.startsWith(readDynamicMetaLanguageIdentifier(role))
        || !selected || selected.kind !== kind && !(kind === 'language configuration' && selected.kind === 'decorators')) fail('missing concrete syntax context');
      if (!checked.has(identifier)) {
        const network = island(selected.network, selected.root, options);
        const sources = kind === 'discovered concepts'
          ? readDiscoveredConceptLinks(network, selected.root).sources.map(source => source.id) : null;
        if (sources === null) decodeStructuredDataLinks(network, selected.root, options);
        checked.set(identifier, { fingerprint: contextFingerprint(selected, options), sources });
      }
      const context = checked.get(identifier);
      if (data[field + 'Fingerprint'] !== context.fingerprint) fail('concrete syntax context changed');
      if (context.sources !== null && !context.sources.includes(data.sourceIdentifier)) fail('unknown concrete syntax source');
    }
  }
}

function parserOptions(options) {
  if (!options || typeof options !== 'object' || Array.isArray(options)) fail('parser options must be a record');
  for (const key of ['grammarName', 'globalConceptSpace', 'resolveGrammar']) {
    if (Object.hasOwn(options, key)) fail(`the archived language definitions own ${key}`);
  }
  for (const key of Reflect.ownKeys(options)) {
    if (typeof key !== 'string' || !acceptDynamicParserOption(key)) fail('unknown parser option');
    if (!Object.hasOwn(Object.getOwnPropertyDescriptor(options, key), 'value')) fail('parser options must be data');
    const value = options[key];
    if (value === undefined || value === null) continue;
    if (['maxDepth', 'stepLimit', 'memoLimit', 'memoryLimit', 'maxRepairs'].includes(key)
      && (!Number.isSafeInteger(value) || value < 0)) fail('invalid parser resource limit');
    if (key === 'startRule' && typeof value !== 'string') fail('start rule must be text');
    if (key === 'errorRecovery' && typeof value !== 'boolean') fail('error recovery must be Boolean');
    if (key === 'ambiguity' && !['accept', 'report', 'reject'].includes(value)) fail('unknown ambiguity policy');
    if (key === 'recovery' && !['accept', 'reject'].includes(value)) fail('unknown recovery policy');
  }
  return options;
}

function parserSettingsData(options) {
  const result = {};
  for (const key of readDynamicParserOptionOrder().split('|')) {
    if (key === 'decorators') result[key] = retainedData(decoratorSet(options.decorators).decorators);
    else if (options[key] !== undefined && options[key] !== null) result[key] = key === 'ambiguity'
      ? options[key] === 'reject' ? 'reject' : 'report' : retainedData(options[key]);
  }
  return result;
}

// Optional absent public fields stay absent. Accessors and foreign objects are
// refused rather than executed while preparing a source-independent archive.
function retainedData(value, depth = 0, active = new Set(), depthLimit = 256) {
  if (depth > depthLimit) fail('retained tree depth limit exceeded');
  if (value === null || ['string', 'boolean', 'number'].includes(typeof value)) return value;
  if (!value || typeof value !== 'object' || active.has(value)) fail('invalid retained tree value');
  if (!Array.isArray(value) && ![Object.prototype, null].includes(Object.getPrototypeOf(value))) fail('foreign retained tree object');
  active.add(value);
  const result = Array.isArray(value) ? [] : {};
  for (const key of Reflect.ownKeys(value)) {
    if (Array.isArray(value) && key === 'length') continue;
    const descriptor = Object.getOwnPropertyDescriptor(value, key);
    if (typeof key !== 'string' || !Object.hasOwn(descriptor, 'value') || !descriptor.enumerable) fail('non-data retained tree property');
    if (descriptor.value !== undefined) Object.defineProperty(result, key, {
      value: retainedData(descriptor.value, depth + 1, active, depthLimit), enumerable: true, writable: true, configurable: true,
    });
  }
  active.delete(value);
  return result;
}

function configuration(options) {
  return { schemaVersion: 1, records: (options.records ?? []).map(({ formalDefinition, ...record }) => record),
    equivalences: options.equivalences ?? [] };
}

/** Prepare a complete editable archive from native grammars and actual formal
 * definition graphs. Independent origins stay separate until proof succeeds.
 */
export function createDynamicMetaLanguage(sources, options = {}) {
  const limits = dynamicLimits(options);
  const discovered = createDiscoveredConceptLinks(sources, options);
  const settings = graph(configuration(options), limits);
  const entries = [
    { identifier: discoveryIdentifier, kind: 'discovered concepts', ...discovered },
    { identifier: configurationIdentifier, kind: 'language configuration', ...settings },
    ...(options.entries ?? []),
  ];
  pinContext(entries, entries[0], 'definition snapshot');
  pinContext(entries, entries[1], 'configuration snapshot');
  const archive = createMetaLanguageArchive(entries, limits);
  return new DynamicMetaLanguage(archive, limits);
}

/** Eager source-free reload. Derived parser caches can be discarded at any
 * point; all authoritative rules, model selections and premises are links.
 */
export class DynamicMetaLanguage {
  #archive;
  #sources;
  #space;
  #formalDefinitions;
  #configuration;
  #limits;
  #grammars;
  #definitionContext;
  #configurationContext;
  #definitionFingerprint;
  #configurationFingerprint;
  #defaultParsers = new Map();

  constructor(archive, options = {}) {
    const limits = dynamicLimits(options);
    let snapshot = { network: copyNetwork(archive.network, limits), root: new LinkId(LinkId.from(archive.root).asU64()) };
    const restored = readMetaLanguageArchive(snapshot.network, snapshot.root, limits);
    const definition = restored.entries.find(entry => entry.identifier === discoveryIdentifier);
    const settings = restored.entries.find(entry => entry.identifier === configurationIdentifier);
    if (definition?.kind !== 'discovered concepts' || !['language configuration', 'decorators'].includes(settings?.kind)) fail('missing language definitions or configuration');
    const config = decodeStructuredDataLinks(island(settings.network, settings.root, limits), settings.root, limits);
    if (config?.schemaVersion !== 1 || !Array.isArray(config.records) || !Array.isArray(config.equivalences)) fail('invalid language configuration');
    const discovered = readDiscoveredConceptLinks(island(definition.network, definition.root, limits), definition.root);
    const count = restored.entries.length;
    this.#definitionContext = pinContext(restored.entries, definition, 'definition snapshot');
    this.#configurationContext = pinContext(restored.entries, settings, 'configuration snapshot');
    if (restored.entries.length !== count) snapshot = createMetaLanguageArchive(restored.entries, limits);
    this.#definitionFingerprint = contextFingerprint(definition, limits);
    this.#configurationFingerprint = contextFingerprint(settings, limits);
    checkSnapshotContexts(restored.entries, limits);
    this.#archive = snapshot;
    this.#sources = discovered.sources;
    this.#space = discovered.space;
    this.#formalDefinitions = discovered.formalDefinitions;
    this.#configuration = config;
    this.#limits = limits;
    this.#grammars = new Map(this.#sources.map(source => [source.id, source.grammar]));
  }

  get archive() { return { network: copyNetwork(this.#archive.network, this.#limits), root: new LinkId(this.#archive.root.asU64()) }; }
  get entries() { const archive = this.archive; return readMetaLanguageArchive(archive.network, archive.root, this.#limits).entries; }
  get sources() { return this.#readDiscoveryView().sources; }
  get space() { return structuredClone(this.#space); }
  get formalDefinitions() { return this.#readDiscoveryView().formalDefinitions; }
  get configuration() { return structuredClone(this.#configuration); }
  get limits() { return { ...this.#limits }; }

  #readDiscoveryView() {
    const entry = this.entries.find(entry => entry.identifier === discoveryIdentifier);
    return readDiscoveredConceptLinks(island(entry.network, entry.root, this.#limits), entry.root);
  }

  parser(sourceIdentifier, options = {}) {
    parserOptions(options);
    if (!this.#grammars.has(sourceIdentifier)) fail('unknown source-qualified grammar identifier');
    const defaults = Object.keys(options).length === 0;
    if (defaults && this.#defaultParsers.has(sourceIdentifier)) return this.#defaultParsers.get(sourceIdentifier);
    const compiled = compileGrammar(this.#grammars.get(sourceIdentifier), {
      ...options, grammarName: sourceIdentifier, globalConceptSpace: this.#space,
      resolveGrammar: name => this.#grammars.get(name),
    });
    const parser = Object.freeze({ parse: Object.freeze((input, overrides = {}) => compiled.parse(input, parserOptions(overrides))),
      parseTree: Object.freeze((input, overrides = {}) => compiled.parseTree(input, parserOptions(overrides))) });
    if (defaults) this.#defaultParsers.set(sourceIdentifier, parser);
    return parser;
  }

  /** Concrete syntax is always the default. AST callers supply an explicit
   * projection; the complete original CST and its archive remain available.
   */
  parse(sourceIdentifier, input, options = {}) {
    const { projection = 'concrete syntax', projectAbstractSyntax, identifier, replaceSnapshot = false, ...settings } = options;
    if (!acceptDynamicSyntaxProjection(projection)) fail('unknown syntax projection');
    if (projection === 'abstract syntax' && typeof projectAbstractSyntax !== 'function') fail('abstract syntax needs an explicit projection');
    const outcome = this.parser(sourceIdentifier, settings).parseTree(input);
    if (!outcome.tree) return { ...outcome, projection, archive: null };
    const retained = retainedData({ schemaVersion: readDynamicConcreteSyntaxVersion(), sourceIdentifier, projection: 'concrete syntax',
      definitionContext: this.#definitionContext, configurationContext: this.#configurationContext,
      definitionContextFingerprint: this.#definitionFingerprint, configurationContextFingerprint: this.#configurationFingerprint,
      parserOptions: parserSettingsData(settings), outcome },
      0, new Set(), this.#limits.depthLimit);
    if (outcome.sourcePlanes) checkGrammarSelectionMappedTree(outcome, settings);
    const concrete = graph(retained, this.#limits);
    const entries = this.entries;
    const selected = identifier ?? readDynamicConcreteIdentifier(entries.map(entry => entry.identifier), sourceIdentifier);
    if (entries.some(entry => entry.identifier === selected && entry.kind !== 'concrete syntax')) fail('concrete syntax identifier collides with another entry point');
    const entry = { identifier: selected, kind: 'concrete syntax', ...concrete };
    const archive = extendMetaLanguageArchive(this.#archive, [entry], { ...this.#limits, replace: replaceSnapshot });
    if (projection === 'concrete syntax') return { ...outcome, projection, archive, concrete };
    const abstract = projectAbstractSyntax(structuredClone(outcome.tree), { sourceIdentifier, space: this.space });
    return { ...outcome, tree: abstract, concreteTree: outcome.tree, projection, archive, concrete };
  }

  /** A new session retains prior source identities, complete old roots and
   * unresolved meanings. Replacing an existing grammar is explicit.
   */
  extend(additions, options = {}) {
    if (!Array.isArray(additions)) fail('language additions must be an array');
    const sources = new Map(this.#sources.map(source => [source.id, source]));
    const added = new Set();
    for (const source of additions) {
      if (!source || added.has(source.id)) fail('duplicate added source');
      if (sources.has(source.id) && options.replace !== true) fail('grammar replacement requires an explicit selection');
      added.add(source.id);
      sources.set(source.id, source);
    }
    const records = new Map(this.#configuration.records.map(record => [record.id, record]));
    const definitions = { ...this.#formalDefinitions, ...options.formalDefinitions };
    for (const record of options.records ?? []) {
      records.set(record.id, record);
      if (record.formalDefinition && !Object.hasOwn(options.formalDefinitions ?? {}, record.id)) definitions[record.id] = record.formalDefinition;
    }
    const previousEntries = this.entries.filter(entry => ![discoveryIdentifier, configurationIdentifier].includes(entry.identifier));
    return createDynamicMetaLanguage([...sources.values()], {
      ...this.#limits, ...Object.fromEntries(Object.entries(options).filter(([, value]) => value !== undefined)), records: [...records.values()], previous: this.#space,
      formalDefinitions: definitions,
      equivalences: [...this.#configuration.equivalences, ...(options.equivalences ?? [])],
      entries: [...previousEntries, ...(options.entries ?? [])],
    });
  }
}
