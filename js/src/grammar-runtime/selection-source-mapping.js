// Host storage for the shared mapping plans. All JSON/Unicode decisions live
// in portable generated kernels. The executor still owns every parse/budget.
import { readGrammarSelectionJsonStringPart,
  readGrammarSelectionMappingReservation, readGrammarSelectionMappedCover } from '../translation/grammar-selection-mapping-rules.js';
import { readSourceTextScalarByteWidth } from '../translation/source-text-byte-rules.js';
import { decodeAt, encodeText } from './text.js';

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
