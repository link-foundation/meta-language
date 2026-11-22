import { ByteRange } from './primitives.js';
import { containsOnlyJsonValues, jsonDocumentError, readJsonDocument } from './json-document.js';
import { validateJsonDocument } from './json-schema-validation.js';

const allowedActions = new Set([
  'closeOpenContainers', 'dropTrailingCommas', 'stripCodeFence', 'extractFirstValue',
  'stripTrailingGarbage', 'repairEscapes',
]);
const closingToken = new Map([['array', ']'], ['object', '}'], ['string', '"']]);
const openingToken = new Map([['array', '['], ['object', '{'], ['string', '"']]);

function coversErrors(nodes, start, end) {
  let covered = start;
  for (const node of nodes) {
    if (node.start > covered) return false;
    if (node.end > covered) covered = node.end;
    if (covered >= end) return true;
  }
  return false;
}

function detectRepetitions(document) {
  const { source, boundaries, nodes } = document;
  const issues = [];
  for (const node of nodes) {
    if (node.kind !== 'string' && node.type !== 'error') continue;
    const first = boundaries.offsetOf(node.start);
    const last = boundaries.offsetOf(node.end);
    let longest;
    // Search periods up to 32 UTF-16 units in each leaf or string. Each period
    // requires one pass and retains a single covering run.
    for (let period = 1; period <= Math.min(32, Math.floor((last - first) / 8)); period += 1) {
      let run = first;
      for (let end = first + period; end <= last; end += 1) {
        if (end < last && source[end] === source[end - period]) continue;
        if (end - run >= period * 8 && end - run >= 32 && (!longest || end - run > longest.length)) {
          const startByte = boundaries.get(run)?.byte;
          const endByte = boundaries.get(end)?.byte;
          if (startByte !== undefined && endByte !== undefined) {
            longest = { startByte, endByte, pattern: source.slice(run, run + period), length: end - run };
          }
        }
        run = end - period + 1;
      }
    }
    if (longest) issues.push({
      keyword: 'repetition', pointer: node.pointer, span: new ByteRange(longest.startByte, longest.endByte),
      expected: [], linkId: node.linkId, pattern: longest.pattern,
    });
  }
  return issues;
}

/** Propose only enabled, CST-supported JSON text edits in original UTF-8 coordinates. */
export function recoverJson(source, { allow = [], schema } = {}) {
  if (!Array.isArray(allow) || allow.some(action => !allowedActions.has(action))) {
    throw new RangeError('unknown JSON recovery action');
  }
  const actions = new Set(allow);
  const document = readJsonDocument(source);
  const { root, nodes, value: firstValue, boundaries } = document;
  const errors = nodes.filter(node => node.flags.isError).sort((left, right) => left.start - right.start);
  const edits = [];
  function add(kind, start, end, inserted, linkId) {
    edits.push({ kind, span: new ByteRange(start, end), inserted,
      removed: source.slice(boundaries.offsetOf(start), boundaries.offsetOf(end)), linkId });
  }

  if (actions.has('stripCodeFence')) {
    const opening = /^[ \t\r\n]*(```(?:json)?)[ \t]*(?:\r?\n)/iu.exec(source);
    const closing = /(?:\r?\n)[ \t]*(```)[ \t\r\n]*$/u.exec(source);
    if (opening && closing && closing.index >= opening[0].length) {
      const start = boundaries.get(opening[0].indexOf(opening[1])).byte;
      const end = start + opening[1].length;
      const closingStart = boundaries.get(closing.index + closing[0].indexOf(closing[1])).byte;
      const closingEnd = closingStart + closing[1].length;
      if (coversErrors(errors.filter(node => node.end > start), start, end)
        && coversErrors(errors.filter(node => node.end > closingStart), closingStart, closingEnd)) {
        add('stripCodeFence', start, end, '', errors.find(node => node.end > start).linkId);
        add('stripCodeFence', closingStart, closingEnd, '', errors.find(node => node.end > closingStart).linkId);
      }
    }
  }

  if (actions.has('dropTrailingCommas')) {
    for (const node of nodes) {
      if (node.kind !== 'array' && node.kind !== 'object') continue;
      const significant = node.children.filter(child => child.start < child.end && !child.trivia
        && !(child.flags.hasError && /^[ \t\r\n]*$/u.test(child.text)));
      const last = node.children.at(-1);
      const [comma, closing] = actions.has('closeOpenContainers') && last?.flags.isMissing && last.end === root.end
        ? [significant.at(-1), last] : significant.slice(-2);
      if (comma?.text !== ',' || (closing?.kind ?? closing?.text) !== closingToken.get(node.kind)) continue;
      const missing = node.children.find(child => child.start >= comma.end && child.end <= closing.start && child.flags.hasError);
      if (missing || closing.flags.hasError) add('dropTrailingCommas', comma.start, comma.end, '', (missing ?? closing).linkId);
    }
  }

  let truncated = false;
  if (actions.has('closeOpenContainers')) {
    for (const node of nodes) {
      const closing = node.children.at(-1);
      if (!closing?.flags.isMissing || closing.kind !== closingToken.get(node.kind) || closing.end !== root.end) continue;
      if (!node.children.some(child => child.type === 'token' && child.text === openingToken.get(node.kind))) continue;
      add('closeOpenContainers', closing.start, closing.end, closing.kind, closing.linkId);
      truncated = true;
    }
  }

  if (actions.has('repairEscapes')) {
    for (const node of nodes) {
      if (node.kind !== 'string') continue;
      const closing = node.children.at(-1);
      for (const error of errors) {
        const insideString = error.start > node.start && error.end <= node.end;
        const controlAtBoundary = closing?.flags.isMissing && error.start === closing.start
          && /^[\u0000-\u001f]/u.test(error.text);
        if (!insideString && !controlAtBoundary) continue;
        const marked = insideString ? error.text : error.text[0];
        const offset = boundaries.offsetOf(error.start);
        for (const match of marked.matchAll(/\\(?:["\\/bfnrt]|u[0-9a-fA-F]{4})|\\|[\u0000-\u001f]/gu)) {
          const [character] = match;
          if (character.length !== 1) continue;
          const start = boundaries.get(offset + match.index).byte;
          add('repairEscapes', start, start + 1, JSON.stringify(character).slice(1, -1), error.linkId);
        }
      }
    }
  }

  if (firstValue && !firstValue.flags.hasError && edits.length === 0) {
    const suffix = root.children.filter(child => child.flags.isError && child.start >= firstValue.end);
    for (const child of suffix) {
      if (actions.has('extractFirstValue') && containsOnlyJsonValues(child.text)) {
        add('extractFirstValue', child.start, child.end, '', child.linkId);
      } else if (actions.has('stripTrailingGarbage')) {
        add('stripTrailingGarbage', child.start, child.end, '', child.linkId);
      }
    }
  }

  let text = '';
  let cursor = 0;
  edits.sort((left, right) => left.span.start - right.span.start);
  const applied = [];
  for (const edit of edits) {
    const start = boundaries.offsetOf(edit.span.start);
    if (start < cursor) continue;
    text += source.slice(cursor, start) + edit.inserted;
    cursor = boundaries.offsetOf(edit.span.end);
    applied.push(edit);
  }
  text += source.slice(cursor);
  const result = applied.length ? readJsonDocument(text) : document;
  const issues = detectRepetitions(document);
  const validation = schema === undefined ? undefined : validateJsonDocument(result, schema);
  const syntax = jsonDocumentError(result);
  issues.push(...(validation?.errors ?? (syntax ? [syntax] : [])));
  return { text, value: syntax ? undefined : validation?.value ?? JSON.parse(text), edits: applied, complete: !truncated && !syntax,
    issues, schemaIssues: validation?.schemaIssues ?? [], network: result.network, sourceNetwork: document.network };
}
