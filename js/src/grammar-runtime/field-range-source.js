// Physical CST field discovery and witness storage. The portable decisions
// select child roles, topmost fields and ranges; this adapter owns no mapper.
import { NestingTooDeep, readChainChildren } from './executor.js';
import { textOf, hexOf } from './text.js';
import { acceptGrammarSelectionPhysicalSpan } from '../translation/grammar-selection-mapping-rules.js';
import { acceptGrammarSelectionFieldRangeSelector, acceptGrammarSelectionFieldRangeMatch,
  readGrammarSelectionFieldRangeChildren, readGrammarSelectionFieldRangePlan } from '../translation/grammar-selection-field-range-rules.js';

const INVALID_FIELD_RANGE_VIEW = Object.freeze({});

/** Read topmost fields from the actual parent plane. The caller validates its
 * UTF-8 bytes and supplies the existing shared parse budget and remaining depth.
 * A witness preserves only the physical tree; it never retains a program or a
 * mapped language's decoded root as part of the parent source.
 */
export function readGrammarSelectionFieldRangeView(control, start, end, selector, children, remainingDepth) {
  control.retain(1);
  let offset = start;
  const refuse = position => {
    if (Number.isInteger(position) && position >= start && position <= end) offset = position;
    throw INVALID_FIELD_RANGE_VIEW;
  };
  const depth = level => {
    control.step();
    if (level > remainingDepth) throw new NestingTooDeep();
  };
  const string = value => {
    if (typeof value !== 'string') refuse(start);
    for (let index = 0; index < value.length; index += 1) control.step();
    control.retain(Buffer.byteLength(value, 'utf8') + 1);
    return value;
  };
  const own = (value, key) => {
    const descriptor = Object.getOwnPropertyDescriptor(value, key);
    if (!descriptor) return undefined;
    if (key === 'children' && descriptor.get === readChainChildren) {
      control.step();
      return readChainChildren.call(value);
    }
    if (!Object.hasOwn(descriptor, 'value')) refuse(start);
    return descriptor.value;
  };
  const array = value => {
    if (!Array.isArray(value) || ![Array.prototype, null].includes(Object.getPrototypeOf(value))) refuse(start);
    const length = own(value, 'length');
    control.retain(length + 1);
    const keys = Reflect.ownKeys(value);
    if (keys.length !== length + 1 || keys[keys.length - 1] !== 'length') refuse(start);
    for (let index = 0; index < length; index += 1) {
      control.step();
      const descriptor = Object.getOwnPropertyDescriptor(value, String(index));
      if (keys[index] !== String(index) || !descriptor || !Object.hasOwn(descriptor, 'value') || !descriptor.enumerable) refuse(start);
    }
    return value;
  };
  const activeMetadata = new Set();
  const metadata = (value, level) => {
    depth(level);
    if (value === null || typeof value === 'boolean' || typeof value === 'number' && Number.isFinite(value)) {
      control.retain(1);
      return value;
    }
    if (typeof value === 'string') return string(value);
    if (!value || typeof value !== 'object' || activeMetadata.has(value)) refuse(start);
    const isArray = Array.isArray(value), prototype = Object.getPrototypeOf(value);
    if (isArray ? prototype !== Array.prototype && prototype !== null
      : prototype !== Object.prototype && prototype !== null) refuse(start);
    control.retain(1);
    if (isArray) array(value);
    let count = 0;
    if (!isArray) for (const key in value) if (Object.hasOwn(value, key)) {
      control.step();
      control.retain(1);
      count += 1;
    }
    const keys = Reflect.ownKeys(value);
    if (!isArray && keys.length !== count) refuse(start);
    const copy = isArray ? [] : {};
    activeMetadata.add(value);
    for (const key of keys) {
      control.step();
      if (isArray && key === 'length') continue;
      if (typeof key !== 'string') refuse(start);
      const descriptor = Object.getOwnPropertyDescriptor(value, key);
      if (!Object.hasOwn(descriptor, 'value') || !descriptor.enumerable) refuse(start);
      string(key);
      control.retain(5);
      Object.defineProperty(copy, key, { value: metadata(descriptor.value, level + 1), enumerable: true,
        writable: true, configurable: true });
    }
    activeMetadata.delete(value);
    return Object.freeze(copy);
  };
  try {
    if (!selector || typeof selector !== 'object'
        || !acceptGrammarSelectionFieldRangeSelector(own(selector, 'kind'), own(selector, 'label'),
          own(selector, 'mode'), own(selector, 'first'), own(selector, 'count'))
        || !Number.isInteger(start) || !Number.isInteger(end) || start < 0 || start >= end
        || end > control.bytes.length) refuse(start);
    const label = own(selector, 'label'), mode = own(selector, 'mode');
    const first = own(selector, 'first'), following = first + own(selector, 'count');
    control.retain(3);
    const spans = [], selectedPaths = [], path = [];
    control.retain(1);
    const activeNodes = new Set();
    let matching = 0, cursor = start;
    const fieldHeader = (node, copy) => {
      const field = own(node, 'field'), trivia = own(node, 'trivia');
      if (field !== undefined) {
        if (field !== null) string(field);
        control.retain(1);
        copy.field = field;
      }
      if (trivia !== undefined && typeof trivia !== 'boolean') refuse(cursor);
      if (trivia) { control.retain(1); copy.trivia = true; }
    };
    const storedMetadata = (node, copy, key, level) => {
      const value = own(node, key);
      if (value === undefined || value === null) return;
      if (!value || typeof value !== 'object' || Array.isArray(value)) refuse(cursor);
      control.retain(1);
      copy[key] = metadata(value, level + 1);
    };
    const leafText = (node, copy, begin, finish) => {
      const width = finish - begin;
      control.retain(width + 1);
      for (let index = begin; index < finish; index += 1) control.step();
      const text = textOf(control.bytes, begin, finish);
      copy.text = text;
      if (text === null) {
        control.retain(width * 4 + 1);
        copy.hex = hexOf(control.bytes, begin, finish);
      }
      if (Object.hasOwn(node, 'text') && own(node, 'text') !== text) refuse(begin);
      if (Object.hasOwn(node, 'hex') && own(node, 'hex') !== copy.hex) refuse(begin);
    };
    const visitChildren = (values, role, level, select) => {
      array(values);
      control.retain(1);
      const copies = [];
      for (let index = 0; index < values.length; index += 1) {
        control.retain(3);
        path.push(role, index);
        copies.push(visit(own(values, String(index)), level + 1, select));
        path.length -= 2;
      }
      return Object.freeze(copies);
    };
    const visit = (node, level, select) => {
      depth(level);
      if (!node || typeof node !== 'object' || Array.isArray(node) || activeNodes.has(node)) refuse(cursor);
      const prototype = Object.getPrototypeOf(node);
      if (prototype !== Object.prototype && prototype !== null) refuse(cursor);
      const type = own(node, 'type'), begin = own(node, 'start'), finish = own(node, 'end');
      const sourceChildren = own(node, 'sourceChildren');
      const role = readGrammarSelectionFieldRangeChildren(type, sourceChildren !== undefined);
      if (role === '' || !acceptGrammarSelectionPhysicalSpan(begin, finish, cursor,
        finish - begin, type === 'missing') || finish > end) refuse(begin);
      if ((type === 'embed' || type === 'error' || type === 'missing')
          && (Object.hasOwn(node, 'field') || Object.hasOwn(node, 'trivia'))) refuse(begin);
      const matched = select && acceptGrammarSelectionFieldRangeMatch(own(node, 'field'), label, own(node, 'trivia') === true);
      if (matched) {
        control.retain(2);
        spans.push(begin, finish);
        if (matching >= first && matching < following) {
          control.retain(path.length + 2);
          for (const component of path) if (typeof component === 'string') string(component);
          selectedPaths.push(Object.freeze([...path]));
        }
        matching += 1;
      }
      activeNodes.add(node);
      control.retain(6);
      let copy;
      if (type === 'node' || type === 'token') {
        const kind = type === 'token' && own(node, 'projectionAnonymous') ? null
          : own(node, 'projectionName') ?? own(node, 'kind');
        if (kind !== null) string(kind);
        copy = { type, kind };
        fieldHeader(node, copy);
        copy.start = begin;
        copy.end = finish;
        if (role === 'children') copy.children = visitChildren(own(node, 'children'), role, level, select && !matched);
        else cursor = finish;
        storedMetadata(node, copy, 'attributes', level);
        if (type === 'token') {
          storedMetadata(node, copy, 'lexical', level);
          leafText(node, copy, begin, finish);
        }
      } else if (type === 'error') {
        copy = { type, start: begin, end: finish };
        leafText(node, copy, begin, finish);
        const reason = own(node, 'reason');
        if (reason) { control.retain(1); copy.reason = string(reason); }
        cursor = finish;
      } else if (type === 'missing') {
        const kind = string(own(node, 'kind'));
        copy = { type, kind, start: begin, end: finish };
        if (own(node, 'literal')) { control.retain(1); copy.literal = true; }
      } else {
        const language = string(own(node, 'language'));
        copy = { type, language, start: begin, end: finish };
        if (role === 'sourceChildren') copy.sourceChildren = visitChildren(sourceChildren, role, level, select && !matched);
        else copy.root = visit(own(node, 'root'), level + 1, false);
      }
      activeNodes.delete(node);
      if (!acceptGrammarSelectionPhysicalSpan(begin, finish, begin, cursor - begin, type === 'missing')) refuse(begin);
      return Object.freeze(copy);
    };
    const physicalChildren = visitChildren(children, 'children', 0, true);
    if (cursor !== end) refuse(cursor);
    control.retain(spans.length + 2);
    for (let index = 0; index < spans.length; index += 1) control.step();
    const plan = readGrammarSelectionFieldRangePlan(spans, start, end, first, own(selector, 'count'), mode);
    if (plan.length === 0) refuse(start);
    control.retain(plan.length / 2 + 3);
    const ranges = [], occurrences = [];
    for (let index = 0; index < plan.length; index += 2) {
      control.step();
      control.retain(3);
      ranges.push(Object.freeze([plan[index], plan[index + 1]]));
    }
    for (let index = 0; index < selectedPaths.length; index += 1) {
      control.step();
      control.retain(4);
      const at = (first + index) * 2;
      occurrences.push(Object.freeze({ path: selectedPaths[index], start: spans[at], end: spans[at + 1] }));
    }
    control.retain(4);
    return Object.freeze({ ranges: Object.freeze(ranges), occurrences: Object.freeze(occurrences),
      witness: Object.freeze({ start, end, children: physicalChildren }) });
  } catch (error) {
    if (error !== INVALID_FIELD_RANGE_VIEW) throw error;
    control.retain(3);
    return { reason: 'invalidEmbeddingMapping', offset };
  }
}
