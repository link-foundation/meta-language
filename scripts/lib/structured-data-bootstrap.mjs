// Read only the canonical typed data statements our dependency-free writer
// emits. General Links Notation remains the installed library's responsibility.
import { Link, LinkMetadata, LinkType } from '../../js/src/primitives.js';
import { StructuredDataNetwork } from '../../js/src/structured-data-encode.js';
import { decodeStructuredDataLinks, structuredDataGraphCost } from '../../js/src/structured-data-graph.js';
import { percentDecode, percentEncode } from '../../js/src/lino-serialization-write.js';
import { limitsOf, encoder, terms } from '../../js/src/structured-data-model.js';
import { namespacePattern } from '../../js/src/structured-data-record-model.js';
import { acceptStructuredDataRecordField } from '../../js/src/structured-data-link-rules.js';
export { structuredDataToLino } from '../../js/src/structured-data-encode.js';
export { structuredDataRecordToLino } from '../../js/src/structured-data-records-write.js';

const fail = message => { throw new TypeError(`canonical bootstrap data: ${message}`); };
const token = '([A-Za-z0-9_.%\\-]+)';
const fields = ` \\(meta: \\(t: semantic\\) \\(n: 1\\) \\(term: ${token}\\)(?: \\(def: ${token}\\))? \\(lang: ${token}\\)\\)\\)`;
const frameFields = ' (meta: (t: semantic) (n: 1) (term: structured%20data%20record) (def: 1) (lang: meta-language%20structured%20data)))';

function textLimit(source, options) {
  const limits = limitsOf(options);
  if (typeof source !== 'string' || encoder.encode(source).length > limits.byteLimit) fail('input byte resource limit exceeded');
  return limits;
}

function canonicalString(source) {
  const value = percentDecode(source);
  if (percentEncode(value) !== source) fail('noncanonical string escape');
  return value;
}

function identity(source) {
  const value = Number(source);
  if (!/^[1-9][0-9]*$/u.test(source) || !Number.isSafeInteger(value)) fail('invalid link identity');
  return value;
}

function graph(source, namespace, options, expectedRoot = null) {
  const limits = textLimit(source, options);
  const prefix = namespace ? `${namespace}/` : '';
  const id = `${prefix}[1-9][0-9]*`;
  const statement = new RegExp(`\\((${id}):((?: ${id})*)${fields}`, 'y');
  const network = new StructuredDataNetwork();
  // This owned graph never escapes: only decoded values and costs are returned.
  // Reuse equal immutable metadata, retaining every link and validation charge.
  const containerMetadata = new Map();
  const roots = [];
  let cursor = 0;
  let previous = 0;
  while (cursor < source.length) {
    statement.lastIndex = cursor;
    const match = statement.exec(source);
    if (!match) fail('not a canonical owned data statement');
    const number = identity(match[1].slice(prefix.length));
    if (number <= previous) fail('duplicate or unordered link identity');
    previous = number;
    if (network._links.size >= limits.nodeLimit) fail('node resource limit exceeded');
    const references = match[2].trim() ? match[2].trim().split(' ').map(reference => identity(reference.slice(prefix.length))) : [];
    const term = canonicalString(match[3]);
    const definition = match[4] === undefined ? undefined : canonicalString(match[4]);
    const language = canonicalString(match[5]);
    let metadata = definition === undefined ? containerMetadata.get(term)?.get(language) : undefined;
    if (!metadata) {
      metadata = LinkMetadata.new().withLinkType(LinkType.Semantic).withNamed(true).withTerm(term).withLanguage(language);
      if (definition !== undefined) metadata = metadata.withDefinition(definition);
      else {
        Object.freeze(metadata.flags); Object.freeze(metadata);
        if (!containerMetadata.has(term)) containerMetadata.set(term, new Map());
        containerMetadata.get(term).set(language, metadata);
      }
    }
    network._links.set(number, new Link(number, references, metadata));
    if (term === terms.root) roots.push(number);
    cursor = statement.lastIndex;
    if (cursor < source.length) {
      if (source[cursor] !== (namespace ? ' ' : '\n')) fail('invalid statement separator');
      cursor++;
    }
  }
  if (roots.length !== 1 || (expectedRoot !== null && roots[0] !== expectedRoot)) fail('invalid schema root');
  const value = decodeStructuredDataLinks(network, roots[0], limits);
  return { value, cost: structuredDataGraphCost(network, roots[0]) };
}

/** The canonical numeric graph format emitted by structuredDataToLino. */
export function readBootstrapStructuredDataLino(source, options = {}) {
  textLimit(source, options);
  if (!source.endsWith('\n')) fail('incomplete canonical document');
  return graph(source, null, options).value;
}

/** One physical line per complete UUID-scoped owned data frame. */
export function readBootstrapStructuredDataRecords(source, options = {}) {
  const limits = textLimit(source, options);
  if (source && !source.endsWith('\n')) fail('incomplete canonical record');
  for (const [label, value] of [['t', 'semantic'], ['n', '1'], ['term', 'structured%20data%20record'], ['def', '1'], ['lang', 'meta-language%20structured%20data']]) {
    if (!acceptStructuredDataRecordField(label, value)) fail('unsupported shared record model');
  }
  const namespaces = new Set();
  const values = [];
  let serializedBytes = 0;
  let expandedNodes = 0;
  let expandedBytes = 0;
  for (const line of source.split('\n').slice(0, -1)) {
    const prefix = /^\(([0-9a-f-]+)\/record: \1\/([1-9][0-9]*) /u.exec(line);
    if (!prefix || !namespacePattern.test(prefix[1]) || namespaces.has(prefix[1]) || !line.endsWith(frameFields)) fail('invalid or duplicate record namespace');
    if (namespaces.size >= limits.nodeLimit) fail('record resource limit exceeded');
    namespaces.add(prefix[1]);
    serializedBytes += encoder.encode(line).length;
    if (serializedBytes > limits.byteLimit) fail('record byte resource limit exceeded');
    const sourceGraph = line.slice(prefix[0].length, -frameFields.length);
    const { value, cost } = graph(sourceGraph, prefix[1], limits, identity(prefix[2]));
    expandedNodes += cost.nodes; expandedBytes += cost.bytes;
    if (expandedNodes > limits.nodeLimit || expandedBytes > limits.byteLimit) fail('expanded record resource limit exceeded');
    values.push(value);
  }
  return values;
}

/** Opt-in inspection of the same completely validated canonical graph. Existing
 * readers, their exact bodies, and default resource limits remain unchanged. */
export function inspectBootstrapStructuredDataLino(source, options = {}) {
  textLimit(source, options);
  if (!source.endsWith('\n')) fail('incomplete canonical document');
  return graph(source, null, options);
}
