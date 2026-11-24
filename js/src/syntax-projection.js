// Generic abstraction retains actual concrete graph identities and every
// nontrivia token, including operators. Decorators remain inspectable data.
import { LinkNetwork } from './network.js';
import { LinkId, LinkMetadata, LinkFlags, LinkType } from './primitives.js';
import { encodeStructuredDataLinks, decodeStructuredDataLinks } from './structured-data-links.js';
import { decoratorSet } from './decorators.js';
import { readSyntaxProjectionTerm, acceptConcreteSyntaxNodeType, acceptAbstractSyntaxNode,
  acceptConcreteSyntaxChildRole } from './syntax-projection-rules.js';
import { acceptMetaLanguageArchiveNextIdentity } from './meta-language-archive-rules.js';

const fail = message => { throw new TypeError('syntax projection: ' + message); };
const identity = id => LinkId.from(id).asU64();
const edgeIdentity = (container, index, role, target) => `${identity(container)}\0${index}\0${role}\0${identity(target)}`;
const language = readSyntaxProjectionTerm('language');
const limitsOf = options => {
  const limits = { nodeLimit: options.nodeLimit ?? 100000, depthLimit: options.depthLimit ?? 256,
    byteLimit: options.byteLimit ?? 128 * 1024 * 1024, referenceLimit: options.referenceLimit ?? 1000000 };
  if (Object.values(limits).some(value => !Number.isSafeInteger(value) || value < 1)) fail('invalid resource limit');
  return limits;
};

function selectIsland(network, root, limits) {
  const selected = new LinkNetwork(), pending = [root], seen = new Set();
  for (let at = 0; at < pending.length; at += 1) {
    const id = identity(pending[at]);
    if (seen.has(id)) continue;
    if (seen.size >= limits.nodeLimit) fail('node resource limit exceeded');
    const link = network.link(id);
    if (!link) fail('dangling concrete reference');
    seen.add(id);
    selected.insertLinkWithOptionalId(id, link.references(), link.metadata().clone());
    for (const reference of link.references()) pending.push(reference);
  }
  return selected;
}
function objectFields(network, root) {
  const link = network.link(root);
  if (link?.metadata().term !== 'object value') fail('expected a concrete object');
  return new Map(link.references().map(pair => {
    const fields = network.link(pair)?.references();
    if (fields?.length !== 2) fail('invalid concrete property');
    const name = network.link(fields[0])?.metadata();
    if (name?.term !== 'string value') fail('a concrete property name must be Unicode scalar text');
    return [name.definition, { pair, value: fields[1] }];
  }));
}
function scalar(network, id) {
  if (id === undefined) return undefined;
  const link = network.link(id), metadata = link?.metadata();
  if (metadata?.term === 'string value') return metadata.definition ?? '';
  if (metadata?.term === 'Unicode code unit string') return link.references().map(id =>
    String.fromCharCode(Number(network.link(id).metadata().definition))).join('');
  if (metadata?.term === 'number value') return Number(metadata.definition);
  if (metadata?.term === 'boolean value') return metadata.definition === 'true';
  if (metadata?.term === 'null value') return null;
  fail('expected a scalar concrete property');
}
function childEdges(network, root, type) {
  const fields = objectFields(network, root), edges = [];
  const array = (selected, role) => {
    if (!selected) return;
    const values = network.link(selected.value);
    if (values?.metadata().term !== 'array value') fail('expected concrete child array');
    values.references().forEach((target, index) => edges.push({ container: selected.value, index, target, role }));
  };
  if (acceptConcreteSyntaxChildRole(type, 'children')) array(fields.get('children'), 'children');
  if (acceptConcreteSyntaxChildRole(type, 'root')) {
    const selected = fields.get('root');
    if (!selected) fail('an embedding needs its concrete root');
    edges.push({ container: selected.pair, index: 1, target: selected.value, role: 'root' });
  }
  if (acceptConcreteSyntaxChildRole(type, 'sourceChildren')) array(fields.get('sourceChildren'), 'sourceChildren');
  if (acceptConcreteSyntaxChildRole(type, 'alternative root') && fields.has('selection')) {
    const selection = objectFields(network, fields.get('selection').value), alternatives = selection.get('alternatives');
    if (alternatives) {
      const list = network.link(alternatives.value);
      if (list?.metadata().term !== 'array value') fail('expected concrete alternatives');
      for (const alternative of list.references()) {
        const selected = objectFields(network, alternative).get('root');
        if (!selected) fail('an alternative needs its concrete root');
        edges.push({ container: selected.pair, index: 1, target: selected.value, role: 'alternative root' });
      }
    }
  }
  return edges;
}
function sourceRoot(network, concreteRoot, path) {
  const schema = network.link(concreteRoot);
  if (schema?.metadata().term !== 'structured data' || schema.references().length !== 1) fail('expected a complete structured concrete graph');
  let selected = schema.references()[0];
  for (const name of path) {
    if (typeof name !== 'string') fail('invalid concrete tree path');
    const next = objectFields(network, selected).get(name);
    if (!next) fail('missing concrete tree path');
    selected = next.value;
  }
  return selected;
}
function nodeRecord(network, root) {
  const fields = objectFields(network, root), record = {};
  for (const name of ['type', 'kind', 'field', 'language', 'text', 'hex', 'start', 'end']) {
    if (fields.has(name)) record[name] = String(scalar(network, fields.get(name).value));
  }
  if (!acceptConcreteSyntaxNodeType(record.type)) fail('unknown concrete syntax node type');
  return { record, trivia: fields.has('trivia') && scalar(network, fields.get('trivia').value) === true };
}
function budget(network, limits) {
  let references = 0, bytes = 0, maximum = 0;
  const encoder = new TextEncoder();
  for (const link of network.links()) {
    maximum = Math.max(maximum, identity(link.id()));
    references += link.references().length;
    if (link.references().some(id => !network.link(id))) fail('dangling graph reference');
    for (const name of ['term', 'definition', 'language']) bytes += encoder.encode(link.metadata()[name] ?? '').length;
  }
  if (!acceptMetaLanguageArchiveNextIdentity(network._nextId, maximum)) fail('invalid allocator state');
  if (network._links.size > limits.nodeLimit || references > limits.referenceLimit || bytes > limits.byteLimit) fail('graph resource limit exceeded');
}

/** Project any native grammar's structured CST into an editable linked AST.
 * Every node points directly to its actual concrete object, every edge records
 * its original container/position, and the entire concrete graph remains present.
 * No callback, source execution or language-pair table participates.
 */
export function createAbstractSyntaxLinks(concreteNetwork, concreteRoot, options = {}) {
  const limits = limitsOf(options);
  budget(concreteNetwork, limits);
  decodeStructuredDataLinks(concreteNetwork, concreteRoot, limits);
  const path = options.treePath ?? [];
  if (!Array.isArray(path) || path.length > limits.depthLimit) fail('invalid concrete tree path');
  const treeRoot = sourceRoot(concreteNetwork, concreteRoot, path), set = decoratorSet(options.decorators);
  const network = new LinkNetwork();
  for (const link of concreteNetwork.links()) network.insertLinkWithOptionalId(new LinkId(identity(link.id())),
    link.references().map(id => new LinkId(identity(id))), new LinkMetadata({ ...link.metadata(), flags: LinkFlags.clean() }));
  network._nextId = concreteNetwork._nextId;
  network._terms = new Map([...(concreteNetwork._terms ?? [])].map(([term, id]) => [term, new LinkId(identity(id))]));
  network._conceptSyntax = new Map(concreteNetwork._conceptSyntax ?? []);
  let references = [...network.links()].reduce((count, link) => count + link.references().length, 0);
  const append = (refs, metadata) => {
    if (network._links.size >= limits.nodeLimit || references + refs.length > limits.referenceLimit) fail('graph resource limit exceeded');
    references += refs.length;
    return network.insertLink(refs.map(id => new LinkId(identity(id))), metadata);
  };
  const insert = (role, refs = [], definition) => append(refs, LinkMetadata.new().withLinkType(LinkType.Syntax)
    .withLanguage(language).withNamed(true).withTerm(readSyntaxProjectionTerm(role)).withDefinition(definition));
  const data = value => {
    const encoded = encodeStructuredDataLinks(value, limits), mapping = new Map();
    for (const link of encoded.network.links()) mapping.set(identity(link.id()), append([], link.metadata().clone()));
    for (const link of encoded.network.links()) {
      const refs = link.references().map(id => mapping.get(identity(id)));
      if (references + refs.length > limits.referenceLimit) fail('graph resource limit exceeded');
      references += refs.length; network.link(mapping.get(identity(link.id()))).setReferences(refs);
    }
    return mapping.get(identity(encoded.root));
  };
  const active = new Set(), memo = new Map();
  const project = (origin, depth) => {
    if (depth > limits.depthLimit) fail('tree depth resource limit exceeded');
    const id = identity(origin);
    if (active.has(id)) fail('cyclic concrete tree');
    if (memo.has(id)) return memo.get(id);
    const { record, trivia } = nodeRecord(network, origin), decorated = set.decorate('cst-to-ast', record);
    if (!acceptAbstractSyntaxNode(record.type, trivia, decorated === null)) { memo.set(id, null); return null; }
    active.add(id);
    const edges = [];
    for (const edge of childEdges(network, origin, record.type)) {
      const target = project(edge.target, depth + 1);
      if (target !== null) edges.push(insert('edge', [edge.container, insert('index', [], String(edge.index)), target], edge.role));
    }
    const result = insert('node', [origin, data(decorated), insert('children', edges)]);
    active.delete(id); memo.set(id, result); return result;
  };
  const projected = project(treeRoot, 0);
  const settings = data({ schemaVersion: 1, treePath: path, decorators: set.decorators });
  const root = insert('root', [concreteRoot, insert('children', projected === null ? [] : [projected]), settings, treeRoot], '1');
  budget(network, limits);
  return { network, root, concreteRoot: LinkId.from(concreteRoot) };
}

/** Reload projected nodes and their actual concrete origins. Edits retain
 * explicit provenance; this structural reader asserts no semantic equivalence.
 */
export function readAbstractSyntaxLinks(network, root, options = {}) {
  const limits = limitsOf(options); budget(network, limits);
  const required = (id, role, count) => {
    const link = network.link(id), metadata = link?.metadata();
    if (metadata?.language !== language || metadata.term !== readSyntaxProjectionTerm(role)
      || metadata.linkType !== LinkType.Syntax || !metadata.named || metadata.span
      || Object.values(metadata.flags ?? {}).some(Boolean)
      || count !== null && link.references().length !== count) fail('invalid projection link');
    return link;
  };
  const header = required(root, 'root', 4);
  if (header.metadata().definition !== '1') fail('unknown projection schema');
  const [concreteRoot, roots, settingsRoot, selected] = header.references();
  decodeStructuredDataLinks(selectIsland(network, concreteRoot, limits), concreteRoot, limits);
  const settings = decodeStructuredDataLinks(selectIsland(network, settingsRoot, limits), settingsRoot, limits);
  if (!settings || typeof settings !== 'object' || Array.isArray(settings)
    || settings.schemaVersion !== 1 || !Array.isArray(settings.treePath) || settings.treePath.length > limits.depthLimit
    || !Array.isArray(settings.decorators)) fail('invalid projection settings');
  decoratorSet(settings.decorators);
  if (identity(sourceRoot(network, concreteRoot, settings.treePath)) !== identity(selected)) fail('concrete root selection changed');
  const active = new Set(), memo = new Map(), heights = new Map(), properties = new Map();
  let propertyBytes = 0;
  const readProperties = root => {
    const key = identity(root);
    if (properties.has(key)) return properties.get(key);
    const record = decodeStructuredDataLinks(selectIsland(network, root, limits), root, limits);
    if (!record || typeof record !== 'object' || Array.isArray(record) || Object.values(record).some(value => typeof value !== 'string')) fail('invalid abstract properties');
    for (const [name, value] of Object.entries(record)) propertyBytes += (name.length + value.length) * 2;
    if (propertyBytes > limits.byteLimit) fail('materialized property byte limit exceeded');
    properties.set(key, record); return record;
  };
  const read = (id, depth) => {
    if (depth > limits.depthLimit) fail('tree depth resource limit exceeded');
    const key = identity(id);
    if (active.has(key)) fail('cyclic abstract tree');
    if (memo.has(key)) {
      if (depth + heights.get(key) > limits.depthLimit) fail('tree depth resource limit exceeded');
      return memo.get(key);
    }
    const [origin, properties, children] = required(id, 'node', 3).references();
    const { record } = nodeRecord(network, origin), expected = new Set(childEdges(network, origin, record.type)
      .map(edge => edgeIdentity(edge.container, edge.index, edge.role, edge.target)));
    active.add(key);
    const edges = required(children, 'children', null).references().map(reference => {
      const edge = required(reference, 'edge', 3), [container, position, target] = edge.references();
      const spelling = required(position, 'index', 0).metadata().definition;
      if (!/^(0|[1-9][0-9]*)$/u.test(spelling ?? '') || !Number.isSafeInteger(Number(spelling))) fail('invalid concrete child position');
      const child = read(target, depth + 1), index = Number(spelling);
      if (!expected.has(edgeIdentity(container, index, edge.metadata().definition, child.origin))) fail('abstract edge lost its concrete origin');
      return { role: edge.metadata().definition, container, index, node: child };
    });
    const height = edges.reduce((height, edge) => Math.max(height, 1 + heights.get(identity(edge.node.id))), 0);
    if (depth + height > limits.depthLimit) fail('tree depth resource limit exceeded');
    const value = { id: LinkId.from(id), origin, properties: readProperties(properties), children: edges };
    active.delete(key); heights.set(key, height); memo.set(key, value); return value;
  };
  const projected = required(roots, 'children', null).references();
  if (projected.length > 1) fail('invalid abstract root selection');
  const tree = projected.length ? read(projected[0], 0) : null;
  if (tree && identity(tree.origin) !== identity(selected)) fail('abstract root lost its concrete origin');
  return { concreteRoot, tree, settings };
}
