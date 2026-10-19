// Host storage for the shared mapping plans. All JSON/Unicode decisions live
// in portable generated kernels. The executor still owns every parse/budget.
import { readGrammarSelectionJsonStringPart,
  readGrammarSelectionMappingReservation, readGrammarSelectionMappedCover } from '../translation/grammar-selection-mapping-rules.js';
import { readSourceTextScalarByteWidth } from '../translation/source-text-byte-rules.js';
import { decodeAt, encodeText } from './text.js';
import { DEFAULT_MAX_DEPTH, NestingTooDeep } from './executor.js';
import { readGrammarSelectionFieldRangeView } from './field-range-source.js';
import {
  acceptGrammarSelectionIncludedRangeSourceRange,
  readGrammarSelectionIncludedRangeReservation,
  readGrammarSelectionIncludedRangePlan,
  readGrammarSelectionIncludedRangePart,
  acceptGrammarSelectionIncludedRangeByteBoundary,
  acceptGrammarSelectionIncludedRangeDerivation,
} from '../translation/grammar-selection-included-range-rules.js';

function sourceRegistry(executor) {
  return executor.budget.memory.selectionSources ??= { byBytes: new WeakMap(), records: [] };
}

function sourceRecord(executor) {
  const registry = sourceRegistry(executor);
  let record = registry.byBytes.get(executor.bytes);
  if (!record) {
    executor.retain(1);
    // The declaration describes the entire concrete source plane, including
    // bytes outside the selected string. Identity-only byte parsing does not
    // enter this bounded UTF-8 mapping profile.
    for (let position = 0; position < executor.bytes.length;) {
      executor.step();
      const scalar = decodeAt(executor.bytes, position);
      if (scalar.codePoint < 0) {
        record = { reason: 'unsupportedEmbeddingMapping', offset: position };
        registry.byBytes.set(executor.bytes, record);
        return record;
      }
      position += scalar.length;
    }
    record = { bytes: executor.bytes, parent: null, mapping: null, mappings: new Map(),
      source: registry.records.length };
    record.definition = Object.freeze({ kind: 'grammar-source', identity: record.source,
      encoding: 'UTF-8', byteLength: executor.bytes.length, derivation: null });
    registry.records.push(record);
    registry.byBytes.set(executor.bytes, record);
  }
  return record;
}

/** Derive one source once per actual parent buffer, mapping and byte interval. */
export function mapGrammarSelectionSource(executor, start, end) {
  const input = sourceRecord(executor);
  if (input.reason) return input;
  const key = `json-string|${start}|${end}`;
  if (input.mappings.has(key)) return input.mappings.get(key);
  const length = end - start;
  executor.retain(readGrammarSelectionMappingReservation(length, false));
  const steps = readGrammarSelectionMappingReservation(length, true);
  for (let index = 0; index < steps; index += 1) executor.step();
  const codes = [], positions = [];
  for (let position = start; position < end;) {
    const next = decodeAt(executor.bytes, position, end);
    if (next.codePoint < 0) {
      const failed = { reason: 'invalidEmbeddingMapping', offset: position };
      input.mappings.set(key, failed);
      return failed;
    }
    positions.push(position);
    codes.push(next.codePoint);
    position += next.length;
  }
  positions.push(end);
  const pieces = [], intervals = [[start, start + 1, 0, 0]];
  let index = 1, outputEnd = 0;
  for (;;) {
    const part = readGrammarSelectionJsonStringPart(codes, index);
    if (part[0] === 3) break;
    if (part[0] !== 0) {
      const failed = { reason: part[0] === 2 ? 'unsupportedEmbeddingMapping' : 'invalidEmbeddingMapping',
        offset: positions[part[1]] ?? start };
      input.mappings.set(key, failed);
      return failed;
    }
    const next = part[2], width = readSourceTextScalarByteWidth(part[1]);
    pieces.push(String.fromCodePoint(part[1]));
    intervals.push([positions[index], positions[next], outputEnd, outputEnd + width]);
    index = next;
    outputEnd += width;
  }
  intervals.push([end - 1, end, outputEnd, outputEnd]);
  const registry = sourceRegistry(executor), bytes = encodeText(pieces.join(''));
  const source = registry.records.length;
  const mapping = Object.freeze({ kind: 'source-mapping', operation: 'json-string', input: input.source,
    output: source, intervals: Object.freeze(intervals.map(row => Object.freeze(row))) });
  const record = { bytes, source, parent: input, mapping, mappings: new Map(),
    definition: Object.freeze({ kind: 'grammar-source', identity: source, encoding: 'UTF-8', byteLength: bytes.length,
      derivation: Object.freeze({ operation: 'json-string', input: input.source, start, end }) }) };
  registry.records.push(record);
  registry.byBytes.set(bytes, record);
  const result = { bytes, mapping };
  input.mappings.set(key, result);
  return result;
}

/** Execute the explicit relative-range view on this actual parent source.
 * The shared plans own interval, scalar-copy and contraction decisions.
 */
export function mapGrammarSelectionIncludedRangeSource(executor, start, end, ranges) {
  const input = sourceRecord(executor);
  if (input.reason) return input;
  return mapRangeSource(executor, input, start, end, ranges);
}

/** A physical witness owns its own view, even when another witness selects
 * equal bytes. Rediscover before consulting the cache: callers can edit the
 * actual children between uses, while retained definitions stay immutable.
 */
export function mapGrammarSelectionFieldRangeSource(executor, start, end, selector, children) {
  const input = sourceRecord(executor);
  if (input.reason) return input;
  const view = readGrammarSelectionFieldRangeView(executor, start, end, selector, children,
    (executor.maxDepth ?? DEFAULT_MAX_DEPTH) - (executor.depth ?? 0));
  if (view.reason) return view;
  executor.retain(6);
  const parameters = Object.freeze({
    kind: Object.getOwnPropertyDescriptor(selector, 'kind').value,
    label: Object.getOwnPropertyDescriptor(selector, 'label').value,
    mode: Object.getOwnPropertyDescriptor(selector, 'mode').value,
    first: Object.getOwnPropertyDescriptor(selector, 'first').value,
    count: Object.getOwnPropertyDescriptor(selector, 'count').value,
  });
  const key = fieldRangeCacheKey(executor, ['field-ranges', start, end, parameters,
    view.occurrences, view.ranges, view.witness]);
  if (!input.fieldMappings) { executor.retain(1); input.fieldMappings = new WeakMap(); }
  const witnesses = input.fieldMappings;
  let mappings = witnesses.get(children);
  if (!mappings) {
    executor.retain(1);
    mappings = new Map();
    witnesses.set(children, mappings);
  }
  if (!mappings.has(key)) executor.retain(Buffer.byteLength(key, 'utf8') + 1);
  return mapRangeSource(executor, input, start, end, view.ranges,
    { key, mappings, selector: parameters, occurrences: view.occurrences, witness: view.witness });
}

// These are already descriptor-checked, frozen plain facts. Type tags keep
// container shapes distinct, and signed zero remains a real metadata value.
function fieldRangeCacheKey(executor, value) {
  const parts = [], maximumDepth = (executor.maxDepth ?? DEFAULT_MAX_DEPTH) - (executor.depth ?? 0);
  const append = text => {
    executor.retain(Buffer.byteLength(text, 'utf8') + 1);
    for (let index = 0; index < text.length; index += 1) executor.step();
    parts.push(text);
  };
  const visit = (item, level) => {
    executor.step();
    if (level > maximumDepth) throw new NestingTooDeep();
    if (item === null) append('n');
    else if (typeof item === 'boolean') append(item ? 'b1' : 'b0');
    else if (typeof item === 'number') append(`d${Object.is(item, -0) ? '-0' : JSON.stringify(item)};`);
    else if (typeof item === 'string') append(`s${JSON.stringify(item)}`);
    else if (Array.isArray(item)) {
      append('a[');
      for (const child of item) visit(child, level + 1);
      append(']');
    } else {
      append('o{');
      for (const key of Object.keys(item)) {
        append(JSON.stringify(key));
        visit(item[key], level + 1);
      }
      append('}');
    }
  };
  visit(value, 0);
  const key = parts.join('');
  executor.retain(Buffer.byteLength(key, 'utf8') + 1);
  return key;
}

// Both explicit ranges and discovered fields execute the unchanged B68 byte
// contraction protocol. Their cache ownership and recorded operations remain
// separate; a field plane is never an included-range record with a new label.
function mapRangeSource(executor, input, start, end, ranges, field = null) {
  const registry = sourceRegistry(executor);
  const failure = offset => ({ reason: 'invalidEmbeddingMapping', offset });
  if (!acceptGrammarSelectionIncludedRangeDerivation('included-ranges', input.source,
    registry.records.length, start, end, executor.bytes.length) || !Array.isArray(ranges)) return failure(start);
  const reservation = readGrammarSelectionIncludedRangeReservation(end - start, ranges.length * 2, false);
  if (reservation < 0) return failure(start);
  for (const range of ranges) {
    executor.step();
    if (!Array.isArray(range) || range.length !== 2
        || !acceptGrammarSelectionIncludedRangeSourceRange('source-range', range[0], range[1])) return failure(start);
  }
  const key = field ? field.key : JSON.stringify(['included-ranges', start, end, ranges]);
  const mappings = field ? field.mappings : input.mappings;
  if (mappings.has(key)) return mappings.get(key);
  executor.retain(reservation);
  const steps = readGrammarSelectionIncludedRangeReservation(end - start, ranges.length * 2, true);
  for (let index = 0; index < steps; index += 1) executor.step();
  const columns = ranges.flat();
  const plan = readGrammarSelectionIncludedRangePlan(columns, end - start);
  const refuse = offset => {
    const result = failure(offset);
    mappings.set(key, result);
    return result;
  };
  if (plan.length === 0) return refuse(start);
  for (const relative of [0, end - start, ...columns]) {
    const position = start + relative;
    if (!acceptGrammarSelectionIncludedRangeByteBoundary(executor.bytes[position] ?? -1,
      position === executor.bytes.length)) return refuse(position);
  }
  const bytes = new Uint8Array(plan[plan.length - 1]);
  const intervals = [];
  let row = 0, position = 0, output = 0;
  for (;;) {
    const scalar = decodeAt(executor.bytes, start + position, end);
    const part = readGrammarSelectionIncludedRangePart(plan, row, position, output, scalar.length);
    if (part[0] === 1) break;
    if (part.length !== 8 || part[0] !== 0) return refuse(start + position);
    const [inputStart, inputEnd, outputStart, outputEnd] = part.slice(4);
    intervals.push(Object.freeze([start + inputStart, start + inputEnd, outputStart, outputEnd]));
    if (outputEnd > outputStart) bytes.set(executor.bytes.subarray(start + inputStart, start + inputEnd), outputStart);
    [row, position, output] = part.slice(1, 4);
  }
  const source = registry.records.length;
  const parameters = Object.freeze(ranges.map(range => Object.freeze([...range])));
  const mapping = Object.freeze(field
    ? { kind: 'source-mapping', operation: 'field-ranges', input: input.source, output: source,
      selector: field.selector, occurrences: field.occurrences, ranges: parameters, intervals: Object.freeze(intervals) }
    : { kind: 'source-mapping', operation: 'included-ranges', input: input.source,
      output: source, ranges: parameters, intervals: Object.freeze(intervals) });
  const record = { bytes, source, parent: input, mapping, mappings: new Map(),
    definition: Object.freeze({ kind: 'grammar-source', identity: source, encoding: 'UTF-8', byteLength: bytes.length,
      derivation: Object.freeze(field
        ? { operation: 'field-ranges', input: input.source, start, end, selector: field.selector,
          occurrences: field.occurrences, ranges: parameters, witness: field.witness }
        : { operation: 'included-ranges', input: input.source, start, end, ranges: parameters }) }) };
  registry.records.push(record);
  registry.byBytes.set(bytes, record);
  const result = { bytes, mapping };
  mappings.set(key, result);
  return result;
}

/** Sources are once-owned definitions, not public JS object-identity promises.
 * A row records an actual completed transform, including attempts whose child
 * grammar subsequently rejected. Such a row does not claim a successful CST.
 */
export function withGrammarSelectionSources(outcome, memory) {
  if (!memory.selectionSources?.records.length) return outcome;
  return { ...outcome, sourcePlanes: memory.selectionSources.records.map(record => record.definition) };
}

/** Speculative region failures must not poison a later accepted region. */
export function rememberGrammarSelectionMappingFailure(executor, diagnostic) {
  const origin = sourceRecord(executor);
  const qualified = origin.reason ? diagnostic : projectGrammarSelectionDiagnostic(executor, { source: origin.source,
    sourceStart: diagnostic.start, sourceEnd: diagnostic.end, ...diagnostic });
  executor.retain(1);
  (executor.budget.memory.selectionMappingFailures ??= []).push(qualified);
}

/** Preserve a covering interval instead of collapsing a real decoded range. */
export function projectGrammarSelectionSpan(mapping, start, end) {
  const cover = readGrammarSelectionMappedCover(mapping.intervals, start, end);
  if (cover.length !== 3) throw new TypeError('a grammar mapping has no source cover for this decoded range');
  return { displayAnchor: cover[0], start: cover[1], end: cover[2] };
}

/** Keep the original diagnostic plane and project its display coordinate along
 * actual mapping edges to the root source. No hash or equal text joins sources.
 */
export function projectGrammarSelectionDiagnostic(executor, diagnostic) {
  let record = executor.budget.memory.selectionSources?.byBytes.get(executor.bytes);
  if (!record?.parent) return diagnostic;
  const source = diagnostic.source ?? record.source;
  const sourceStart = diagnostic.sourceStart ?? diagnostic.start, sourceEnd = diagnostic.sourceEnd ?? diagnostic.end;
  let start = sourceStart, end = sourceEnd;
  // A supplied diagnostic may already be anchored in the current plane after
  // one child transform, while preserving its deeper original coordinates.
  start = diagnostic.start; end = diagnostic.end;
  const sourceRoute = [...(diagnostic.sourceRoute ?? [])];
  while (record.parent) {
    const cover = projectGrammarSelectionSpan(record.mapping, start, end);
    sourceRoute.push({ output: record.source, outputStart: start, outputEnd: end,
      input: record.parent.source, inputStart: cover.start, inputEnd: cover.end, displayAnchor: cover.displayAnchor });
    start = cover.start; end = cover.end;
    record = record.parent;
  }
  return { ...diagnostic, start, end, source, sourceStart, sourceEnd, sourceRoute,
    offsetSemantics: 'covering-source-spelling' };
}
