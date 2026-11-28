import { readFileSync } from 'node:fs';
import { DecoratorSet } from './decorators.js';
import { compileGrammar } from './grammar.js';
import { parseGrammarLinks } from './grammar-links.js';

const repairKinds = new Set([
  'closeOpenContainers', 'dropTrailingCommas', 'stripCodeFence',
  'extractFirstValue', 'stripTrailingGarbage', 'repairEscapes',
]);
const valueStart = /^[\t\n\r ]*(?:[\[{"-]|[0-9]|(?:true|false|null)(?![A-Za-z0-9_]))/u;
const fenceStart = /^[\t\n\r ]*(?:```|~~~)/u;
const proseFence = /\n[\t ]*(?:```|~~~)/u;
const fenceMarker = /^(?:`{3,}|~{3,})/u;
const isJsonSpace = (byte) => byte === 32 || byte === 9 || byte === 10 || byte === 13;
const isContinuation = (byte) => (byte & 0xc0) === 0x80;

let parser;
let decorators;

function parseRecoveryTree(bytes, startRule) {
  parser ??= compileGrammar(parseGrammarLinks(readFileSync(new URL('./data/json-recovery-grammar.lino', import.meta.url), 'utf8')));
  // Bound the shared recovery grammar below the JavaScript executor's call-stack limit.
  return parser.parseTree(bytes, { startRule, recovery: 'accept', maxDepth: 192 });
}

function parseValue(text) {
  let value;
  try {
    value = JSON.parse(text);
  } catch (error) {
    if (error instanceof SyntaxError) return undefined;
    throw error;
  }
  // Match the interoperable value domain of both package APIs.
  const pending = [[value, 0]];
  while (pending.length) {
    const [item, depth] = pending.pop();
    if (typeof item === 'string' && !item.isWellFormed()) return undefined;
    if (typeof item === 'number' && !Number.isFinite(item)) return undefined;
    if (item !== null && typeof item === 'object') {
      if (depth === 127) return undefined;
      for (const key of Object.keys(item)) {
        if (!key.isWellFormed()) return undefined;
        pending.push([item[key], depth + 1]);
      }
    }
  }
  return value;
}

function repetitionIssue(bytes) {
  const minimumLength = 256;
  let span;
  for (let period = 1; period <= 32 && bytes.length >= minimumLength; period += 1) {
    let start = 0;
    for (let end = period; end <= bytes.length; end += 1) {
      if (end < bytes.length && bytes[end] === bytes[end - period]) continue;
      if (end - start >= Math.max(minimumLength, period * 8)) {
        let from = start;
        let to = end;
        while (isContinuation(bytes[from])) from += 1;
        while (isContinuation(bytes[to])) to -= 1;
        if (to - from >= Math.max(minimumLength, period * 8)
            && (!span || to - from > span.end - span.start || (to - from === span.end - span.start && from < span.start))
            && bytes.subarray(from, from + period).some(byte => !isJsonSpace(byte))) {
          span = { start: from, end: to };
        }
      }
      start = end - period + 1;
    }
  }
  return span && { kind: 'degenerateRepetition', span };
}

function pointerPart(key) {
  return key.replaceAll('~', '~0').replaceAll('/', '~1');
}

function missingRequiredProperties(value, schema) {
  const issues = [];
  const pending = [[value, schema, '']];
  while (pending.length) {
    const [item, rule, pointer] = pending.pop();
    if (item === null || typeof item !== 'object' || !rule || typeof rule !== 'object') continue;
    if (Array.isArray(item)) {
      for (let index = item.length - 1; index >= 0; index -= 1) {
        const child = Array.isArray(rule.prefixItems) && index < rule.prefixItems.length ? rule.prefixItems[index] : rule.items;
        if (child && typeof child === 'object') pending.push([item[index], child, `${pointer}/${index}`]);
      }
      continue;
    }
    if (Array.isArray(rule.required)) {
      const reported = new Set();
      for (const key of rule.required) {
        if (typeof key !== 'string' || Object.hasOwn(item, key) || reported.has(key)) continue;
        reported.add(key);
        issues.push({ kind: 'missingRequiredProperty', pointer: `${pointer}/${pointerPart(key)}`, property: key });
      }
    }
    if (rule.properties && typeof rule.properties === 'object') {
      const keys = Object.keys(rule.properties).map(key => [Buffer.from(key), key]);
      keys.sort(([left], [right]) => Buffer.compare(right, left));
      for (const [, key] of keys) {
        if (Object.hasOwn(item, key)) pending.push([item[key], rule.properties[key], `${pointer}/${pointerPart(key)}`]);
      }
    }
  }
  return issues;
}

function collectEdits(tree, bytes, offset, allow, edits, fromProse = false) {
  const stack = [[tree, '']];
  while (stack.length) {
    const [node, region] = stack.pop();
    if (node.type === 'node') {
      let childRegion = node.kind;
      if (node.kind === 'trailing_region') {
        const followsValue = valueStart.test(bytes.toString('utf8', offset + node.start, offset + node.end));
        if (followsValue || fromProse || !allow.has('stripTrailingGarbage')) childRegion = 'following_value';
      }
      for (let index = node.children.length - 1; index >= 0; index -= 1) {
        stack.push([node.children[index], childRegion]);
      }
      continue;
    }
    if (node.type !== 'error' && node.type !== 'missing') continue;
    const span = { start: offset + node.start, end: offset + node.end };
    const { kind, action } = decorators.decorate('transformation', { type: node.type, region }) ?? {};
    if (!allow.has(kind)) return span;
    const removed = bytes.toString('utf8', span.start, span.end);
    let inserted = '';
    if (action === 'insert' && node.type === 'missing') inserted = node.kind;
    else if (action === 'escapeBackslash') inserted = '\\\\';
    else if (action === 'escapeControl') inserted = `\\u00${bytes[span.start].toString(16).padStart(2, '0')}`;
    else if (action !== 'remove') return span;
    const previous = edits.at(-1);
    if ((kind === 'stripCodeFence' || kind === 'extractFirstValue') && previous?.kind === kind && previous.span.end === span.start) {
      previous.span.end = span.end;
      previous.removed += removed;
    } else {
      edits.push({ kind, span, inserted, removed });
    }
  }
  return null;
}

function applyEdits(bytes, edits) {
  const chunks = [];
  let offset = 0;
  for (const { span, inserted } of edits) {
    chunks.push(bytes.toString('utf8', offset, span.start), inserted);
    offset = span.end;
  }
  chunks.push(bytes.toString('utf8', offset));
  return chunks.join('');
}

function rejectionSpan(rejection, offset, length) {
  const position = offset + (rejection?.offset ?? 0);
  return { start: position, end: rejection?.reason === 'syntax' ? position : offset + length };
}

function planRecovery(text, bytes, allowed, edits) {
  const startsWithFence = fenceStart.test(text);
  let frame = startsWithFence ? parseRecoveryTree(bytes, 'fenced_document') : null;
  if (frame && !frame.ok) return rejectionSpan(frame.rejection, 0, bytes.length);

  let parsed = frame ? null : parseRecoveryTree(bytes, 'document');
  let fromProse = false;
  if (parsed && !parsed.ok && allowed.has('extractFirstValue') && !valueStart.test(text)) {
    parsed = parseRecoveryTree(bytes, 'prose_document');
    fromProse = true;
    if (proseFence.test(text)) {
      const candidate = parseRecoveryTree(bytes, 'prose_fenced_document');
      if (candidate.ok) {
        const opening = candidate.tree.children.find(node => node.kind === 'fenced_region').start;
        const prefixEnd = parsed.tree?.children.find(node => node.kind === 'leading_region')?.end;
        if (prefixEnd === undefined || opening < prefixEnd) frame = candidate;
      }
    }
  }
  if (!frame) {
    if (!parsed.ok) return rejectionSpan(parsed.rejection, 0, bytes.length);
    return collectEdits(parsed.tree, bytes, 0, allowed, edits, fromProse);
  }

  const frameNodes = frame.tree.children.filter(node => node.type === 'node');
  const [opening, body, closing, tail] = frameNodes.pop().children.filter(node => node.type === 'node');
  if (closing) {
    const opener = bytes.toString('utf8', opening.start, opening.end).match(fenceMarker)[0];
    const closer = bytes.toString('utf8', closing.start, closing.end).trimStart().match(fenceMarker)[0];
    if (opener[0] !== closer[0] || closer.length < opener.length) return { start: closing.start, end: closing.end };
  }
  for (const prefix of [...frameNodes, opening]) {
    const rejected = collectEdits(prefix, bytes, 0, allowed, edits);
    if (rejected) return rejected;
  }
  parsed = parseRecoveryTree(bytes.subarray(body.start, body.end), 'document');
  if (!parsed.ok) return rejectionSpan(parsed.rejection, body.start, body.end - body.start);
  return collectEdits(parsed.tree, bytes, body.start, allowed, edits)
    ?? (closing && collectEdits(closing, bytes, 0, allowed, edits))
    ?? (tail && collectEdits(tail, bytes, 0, allowed, edits, fromProse))
    ?? null;
}

/** Recover one JSON value and report every edit in original UTF-8 byte coordinates. */
export function recoverJson(text, options = {}) {
  if (typeof text !== 'string' || !text.isWellFormed()) throw new TypeError('JSON recovery requires a Unicode scalar string');
  const { allow = [...repairKinds], schema } = options;
  if (!Array.isArray(allow) || allow.some(kind => !repairKinds.has(kind))) throw new TypeError('Invalid JSON recovery allow list');
  const allowed = new Set(allow);
  const bytes = Buffer.from(text);
  const repeated = repetitionIssue(bytes);
  const issues = repeated ? [repeated] : [];
  const edits = [];
  let recovered = text;
  let value = parseValue(text);
  const refuse = (span) => ({
    text, value: null, edits: [], complete: false, issues: [...issues, { kind: 'unrecoveredSyntax', span }],
  });
  if (value === undefined) {
    decorators ??= DecoratorSet.fromLino(readFileSync(new URL('./data/json-recovery-decorators.lino', import.meta.url), 'utf8'));
    const rejected = planRecovery(text, bytes, allowed, edits);
    if (rejected) return refuse(rejected);
    recovered = applyEdits(bytes, edits);
    value = parseValue(recovered);
    if (value === undefined) return refuse({ start: 0, end: bytes.length });
  }
  const missing = missingRequiredProperties(value, schema);
  issues.push(...missing);
  const complete = missing.length === 0 && !edits.some(({ kind }) => kind === 'closeOpenContainers');
  return { text: recovered, value, edits, complete, issues };
}
