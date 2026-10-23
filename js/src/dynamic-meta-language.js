// Runtime extension uses the same compiler, concept verifier and archived
// formal premises as prepared grammars. No host/child language pair table.
import { compileGrammar } from './grammar.js';
import { createDiscoveredConceptLinks, readDiscoveredConceptLinks } from './discovered-concept-links.js';
import { createMetaLanguageArchive, readMetaLanguageArchive,
  extendMetaLanguageArchive } from './meta-language-archive.js';
import { encodeStructuredDataLinks, decodeStructuredDataLinks } from './structured-data-links.js';
import { checkGrammarSelectionMappedTree } from './grammar-runtime.js';
import { LinkNetwork } from './network.js';

const discoveryIdentifier = 'language definitions';
const configurationIdentifier = 'language configuration';
const fail = message => { throw new TypeError(`dynamic meta-language: ${message}`); };

function graph(value, options) {
  const encoded = encodeStructuredDataLinks(value, options), network = new LinkNetwork();
  for (const link of encoded.network.links()) network.insertLinkWithOptionalId(link.id(), link.references(), link.metadata().clone());
  return { network, root: encoded.root };
}

function parserOptions(options) {
  if (!options || typeof options !== 'object' || Array.isArray(options)) fail('parser options must be a record');
  for (const key of ['grammarName', 'globalConceptSpace', 'resolveGrammar']) {
    if (Object.hasOwn(options, key)) fail(`the archived language definitions own ${key}`);
  }
  return options;
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
  const discovered = createDiscoveredConceptLinks(sources, options);
  const settings = graph(configuration(options), options);
  const archive = createMetaLanguageArchive([
    { identifier: discoveryIdentifier, kind: 'discovered concepts', ...discovered },
    { identifier: configurationIdentifier, kind: 'decorators', ...settings },
    ...(options.entries ?? []),
  ], options);
  return new DynamicMetaLanguage(archive, options);
}

/** Eager source-free reload. Derived parser caches can be discarded at any
 * point; all authoritative rules, model selections and premises are links.
 */
export class DynamicMetaLanguage {
  constructor(archive, options = {}) {
    const restored = readMetaLanguageArchive(archive.network, archive.root, options);
    const definition = restored.entries.find(entry => entry.identifier === discoveryIdentifier);
    const settings = restored.entries.find(entry => entry.identifier === configurationIdentifier);
    if (definition?.kind !== 'discovered concepts' || settings?.kind !== 'decorators') fail('missing language definitions or configuration');
    const config = decodeStructuredDataLinks(settings.network, settings.root, options);
    if (config?.schemaVersion !== 1 || !Array.isArray(config.records) || !Array.isArray(config.equivalences)) fail('invalid language configuration');
    const discovered = readDiscoveredConceptLinks(definition.network, definition.root);
    this.archive = archive;
    this.entries = restored.entries;
    this.sources = discovered.sources;
    this.space = discovered.space;
    this.formalDefinitions = discovered.formalDefinitions;
    this.configuration = config;
    const depthLimit = options.depthLimit ?? 256;
    if (!Number.isSafeInteger(depthLimit) || depthLimit < 1) fail('invalid retained tree depth limit');
    this.limits = { nodeLimit: options.nodeLimit ?? 100000, referenceLimit: options.referenceLimit ?? 1000000,
      byteLimit: options.byteLimit ?? 128 * 1024 * 1024, depthLimit };
    this.grammars = new Map(this.sources.map(source => [source.id, source.grammar]));
  }

  parser(sourceIdentifier, options = {}) {
    parserOptions(options);
    if (!this.grammars.has(sourceIdentifier)) fail('unknown source-qualified grammar identifier');
    const compiled = compileGrammar(this.grammars.get(sourceIdentifier), {
      ...options, grammarName: sourceIdentifier, globalConceptSpace: this.space,
      resolveGrammar: name => this.grammars.get(name),
    });
    return { parse: (input, overrides = {}) => compiled.parse(input, parserOptions(overrides)),
      parseTree: (input, overrides = {}) => compiled.parseTree(input, parserOptions(overrides)) };
  }

  /** Concrete syntax is always the default. AST callers supply an explicit
   * projection; the complete original CST and its archive remain available.
   */
  parse(sourceIdentifier, input, options = {}) {
    const { projection = 'concrete syntax', projectAbstractSyntax, identifier, ...settings } = options;
    if (!['concrete syntax', 'abstract syntax'].includes(projection)) fail('unknown syntax projection');
    if (projection === 'abstract syntax' && typeof projectAbstractSyntax !== 'function') fail('abstract syntax needs an explicit projection');
    const outcome = this.parser(sourceIdentifier, settings).parseTree(input);
    if (!outcome.tree) return { ...outcome, projection, archive: null };
    const retained = retainedData({ schemaVersion: 1, sourceIdentifier, projection: 'concrete syntax', outcome },
      0, new Set(), this.limits.depthLimit);
    if (outcome.sourcePlanes) checkGrammarSelectionMappedTree(outcome, settings);
    const concrete = graph(retained, this.limits);
    const entry = { identifier: identifier ?? `concrete syntax:${sourceIdentifier}`, kind: 'concrete syntax', ...concrete };
    const archive = extendMetaLanguageArchive(this.archive, [entry], { ...this.limits, replace: true });
    if (projection === 'concrete syntax') return { ...outcome, projection, archive, concrete };
    const abstract = projectAbstractSyntax(outcome.tree, { sourceIdentifier, space: this.space });
    return { ...outcome, tree: abstract, concreteTree: outcome.tree, projection, archive, concrete };
  }

  /** A new session retains prior source identities, complete old roots and
   * unresolved meanings. Replacing an existing grammar is explicit.
   */
  extend(additions, options = {}) {
    if (!Array.isArray(additions)) fail('language additions must be an array');
    const sources = new Map(this.sources.map(source => [source.id, source]));
    const added = new Set();
    for (const source of additions) {
      if (!source || added.has(source.id)) fail('duplicate added source');
      if (sources.has(source.id) && options.replace !== true) fail('grammar replacement requires an explicit selection');
      added.add(source.id);
      sources.set(source.id, source);
    }
    const records = new Map(this.configuration.records.map(record => [record.id, record]));
    for (const record of options.records ?? []) records.set(record.id, record);
    const previousEntries = this.entries.filter(entry => ![discoveryIdentifier, configurationIdentifier].includes(entry.identifier));
    return createDynamicMetaLanguage([...sources.values()], {
      ...this.limits, ...options, records: [...records.values()], previous: this.space,
      formalDefinitions: { ...this.formalDefinitions, ...options.formalDefinitions },
      equivalences: [...this.configuration.equivalences, ...(options.equivalences ?? [])],
      entries: [...previousEntries, ...(options.entries ?? [])],
    });
  }
}
