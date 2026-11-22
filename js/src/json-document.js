import { readFileSync } from 'node:fs';
import { compileGrammar } from './grammar.js';
import { parseGrammarLinks } from './grammar-links.js';
import { LinkNetwork } from './network.js';
import { ByteRange, LinkFlags, Point, SourceSpan } from './primitives.js';
import { sourceBoundaries } from './source-boundaries.js';

const grammarFile = new URL('./data/json-document/grammar.lino', import.meta.url);
let grammar;
let parser;
const valueKinds = new Set(['array', 'object', 'string', 'number', 'true', 'false', 'null']);

export function jsonDocumentGrammar() {
  return grammar ??= parseGrammarLinks(readFileSync(grammarFile, 'utf8'));
}

export function pointerChild(pointer, key) {
  return `${pointer}/${String(key).replaceAll('~', '~0').replaceAll('/', '~1')}`;
}

function documentParser() {
  return parser ??= compileGrammar(jsonDocumentGrammar());
}

export function containsOnlyJsonValues(source) {
  const values = documentParser().findAll(source, { rule: 'value', wordBoundaries: true });
  let end = 0;
  for (const value of values) {
    if (!/^[ \t\r\n]*$/u.test(source.slice(end, value.start))) return false;
    end = value.end;
  }
  return values.length > 0 && /^[ \t\r\n]*$/u.test(source.slice(end));
}

function isValue(node) {
  return !node.trivia && node.type !== 'missing' && valueKinds.has(node.kind);
}

export function readJsonDocument(source) {
  if (typeof source !== 'string') throw new TypeError('a JSON document must be text');
  const compiled = documentParser();
  const strict = compiled.parseTree(source);
  const { tree } = strict.ok ? strict : compiled.parseTree(source, { errorRecovery: true, recovery: 'accept' });
  const network = new LinkNetwork();
  const boundaries = sourceBoundaries(source);
  const nodes = [];
  function convert(raw) {
    const { start, end } = raw;
    const startOffset = boundaries.offsetOf(start);
    const endOffset = boundaries.offsetOf(end);
    const text = source.slice(startOffset, endOffset);
    const children = (raw.children ?? []).map(convert);
    const firstContent = children.find(child => !child.trivia)?.span.byteRange.start ?? start;
    const lastContent = children.findLast(child => !child.trivia)?.span.byteRange.end ?? end;
    const first = boundaries.get(boundaries.offsetOf(firstContent));
    const last = boundaries.get(boundaries.offsetOf(lastContent));
    const span = new SourceSpan(new ByteRange(firstContent, lastContent), new Point(first.row, first.column), new Point(last.row, last.column));
    let flags = LinkFlags.clean();
    if (raw.type === 'error') flags = LinkFlags.error();
    else if (raw.type === 'missing') flags = LinkFlags.missing();
    else if (children.some(child => child.flags.hasError)) flags = LinkFlags.containingError();
    else if (raw.trivia) flags = flags.withExtra();
    const references = children.map(child => child.linkId);
    if (children.length === 0 && start < end) references.push(network.insertSourceToken('JSON', text, span, flags));
    const term = raw.kind ?? (flags.isError ? 'ERROR' : text);
    const linkId = network.insertSyntaxNode('JSON', term, references, { span, flags, named: Boolean(raw.kind) && !raw.literal });
    for (const child of children) {
      if (child.field) network.insertFieldRelation(linkId, child.field, child.linkId);
    }
    const node = { ...raw, text, children, flags, linkId, span, pointer: '' };
    nodes.push(node);
    return node;
  }
  const root = convert(tree ?? { type: 'error', start: 0, end: Buffer.byteLength(source) });
  const value = root.children.find(isValue);
  if (value) assignPointers(value, '');
  return { source, root, value, nodes, network, boundaries, rejection: strict.rejection };
}

/** Decodes only values whose spelling is present, retaining known children of incomplete containers. */
export function readJsonValue(node) {
  if (!node || node.flags.isMissing) return undefined;
  if (!node.flags.hasError) return JSON.parse(node.text);
  if (node.kind === 'array') return node.children.filter(isValue).map(readJsonValue);
  if (node.kind === 'object') {
    const fields = [];
    for (const pair of node.children.filter(child => child.kind === 'pair')) {
      const key = pair.children.find(child => child.field === 'key');
      const value = pair.children.find(child => child.field === 'value');
      if (key && !key.flags.hasError) fields.push([JSON.parse(key.text), readJsonValue(value)]);
    }
    return Object.fromEntries(fields);
  }
  return undefined;
}

function assignPointers(node, pointer) {
  node.pointer = pointer;
  if (node.kind === 'array') {
    let index = 0;
    for (const child of node.children) assignPointers(child, isValue(child) ? pointerChild(pointer, index++) : pointer);
  } else if (node.kind === 'object') {
    for (const child of node.children) {
      const key = child.kind === 'pair' ? child.children.find(field => field.field === 'key') : undefined;
      let childPointer = pointer;
      if (key && !key.flags.hasError) childPointer = pointerChild(pointer, JSON.parse(key.text));
      assignPointers(child, childPointer);
    }
  } else for (const child of node.children) assignPointers(child, pointer);
}

export function jsonDocumentError(document) {
  const { rejection, nodes, root } = document;
  if (!rejection) return null;
  const offset = rejection.offset ?? root.start;
  const errors = nodes.filter(({ start, end, flags }) => start <= offset && offset <= end && (flags.isError || flags.isMissing));
  const location = errors.find(node => node.flags.isMissing && node.start === offset)
    ?? errors.find(node => node.start <= offset && offset < node.end)
    ?? errors[0] ?? root;
  return {
    keyword: 'syntax', pointer: location.pointer, span: location.span,
    linkId: location.linkId, expected: rejection.expected ?? [], reason: rejection.reason,
  };
}
