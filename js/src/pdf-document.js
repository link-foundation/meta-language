import { inflateSync } from 'node:zlib';
import {
  documentStreamFitsBudget, pdfDrawingOperator, pdfLzwCodeWidth, pdfStreamFilterKind, pdfTextOperatorKind,
} from './document-container-policy.js';
import { parsePdfContentCst } from './pdf-grammar.js';
import { decodePdfFontText, pdfGlyphName, pdfPredefinedEncoding, pdfStringBytes, readPdfUnicodeMap } from './pdf-text-fonts.js';

const maximumStreamBytes = 16 * 1024 * 1024;
const maximumDecodedDocumentBytes = 32 * 1024 * 1024;
const maximumCmapBytes = 2 * 1024 * 1024;
const pdfWhitespace = /[\x00\t\n\f\r ]/gu;

const field = (node, name) => node?.children.find(child => child.field === name);
const operands = node => node?.children.filter(child => child.field === 'operand') ?? [];
const arrayElements = node => node?.children.filter(child => child.named && !child.extra) ?? [];
const textOf = (text, node) => node ? text.slice(node.start, node.end) : '';
const nameOf = (text, node) => node?.term === 'name' ? textOf(text, node).slice(1)
  .replace(/#([0-9a-fA-F]{2})/gu, (_, digits) => String.fromCharCode(Number.parseInt(digits, 16))) : '';

export function readPdfDocument(text, tree, decodeStreams) {
  const objects = new Map();
  for (const object of tree.children) {
    if (object.term === 'indirect_object') objects.set(objectReference(object), field(object, 'value'));
  }
  function objectReference(node) {
    return `${Number(textOf(text, field(node, 'object_number')))} ${Number(textOf(text, field(node, 'generation')))}`;
  }
  function resolve(node) {
    return node?.term === 'indirect_reference' ? objects.get(objectReference(node)) : node;
  }
  function entries(node) {
    const dictionary = resolve(node);
    return dictionary?.children.filter(child => child.term === 'dictionary_entry') ?? [];
  }
  function get(node, key) {
    return resolve(field(entries(node).find(entry => nameOf(text, field(entry, 'key')) === key), 'value'));
  }

  let decodedBytes = 0;
  const errors = [];
  const streams = [];
  const textRuns = [];
  const streamCache = new Map();
  const fontCache = new Map();
  const attached = new Set();

  function readStream(node) {
    if (node?.term !== 'stream') return null;
    if (streamCache.has(node)) return streamCache.get(node);
    const data = field(node, 'data');
    const dictionary = field(node, 'dictionary');
    const filter = get(dictionary, 'Filter');
    if (!data || (filter && !decodeStreams)) return null;
    if (!filter) return { source: textOf(text, data), data, filter: '' };
    const filters = filter.term === 'array' ? arrayElements(filter).map(node => nameOf(text, resolve(node))) : [nameOf(text, filter)];
    const filterKinds = filters.map(pdfStreamFilterKind);
    if (!filters.length || filterKinds.includes(0)) return null;
    const parameters = get(dictionary, 'DecodeParms');
    const options = parameters?.term === 'array' ? arrayElements(parameters).map(resolve) : [parameters];
    if (options.some(option => Number(textOf(text, get(option, 'Predictor'))) > 1)) return null;
    let result = null;
    try {
      if (decodedBytes >= maximumDecodedDocumentBytes || !documentStreamFitsBudget(data.end - data.start, 0, decodedBytes) || filters.length > 8) throw new Error('PDF stream exceeds the decoded size limit');
      let source = textOf(text, data);
      if (/[^\x00-\xFF]/u.test(source)) throw new Error('invalid binary PDF stream');
      for (const [index, kind] of filterKinds.entries()) {
        const limit = Math.min(maximumStreamBytes, maximumDecodedDocumentBytes - decodedBytes);
        if (limit === 0) throw new Error('PDF stream exceeds the decoded size limit');
        if (kind === 1) {
          try { source = inflateSync(Buffer.from(source, 'latin1'), { maxOutputLength: limit }).toString('latin1'); }
          catch { throw new Error('invalid or oversized FlateDecode PDF stream'); }
        } else if (kind === 2) {
          const encoded = source.replace(pdfWhitespace, '');
          if (!/^[0-9A-Fa-f]*>?$/u.test(encoded)) throw new Error('invalid ASCIIHexDecode PDF stream');
          source = Buffer.from(pdfStringBytes(`<${encoded.replace(/>$/u, '')}>`)).toString('latin1');
        } else if (kind === 3) {
          source = decodeAscii85(source, limit);
        } else {
          const parameter = get(options[index], 'EarlyChange');
          const earlyChange = parameter ? Number(textOf(text, parameter)) : 1;
          if (parameter && (parameter.term !== 'integer' || (earlyChange !== 0 && earlyChange !== 1))) {
            throw new Error('invalid LZWDecode PDF EarlyChange');
          }
          source = decodeLzw(source, earlyChange, limit);
        }
        if (!documentStreamFitsBudget(0, source.length, decodedBytes)) throw new Error('PDF stream exceeds the decoded size limit');
        decodedBytes += source.length;
      }
      result = { source, data, filter: filters.join(',') };
    } catch (error) {
      errors.push({ message: error.message, start: data.start, end: data.end });
    }
    streamCache.set(node, result);
    return result;
  }

  function readFont(node) {
    if (!node) return null;
    if (fontCache.has(node)) return fontCache.get(node);
    const encodingNode = get(node, 'Encoding');
    const encodingName = nameOf(text, encodingNode) || nameOf(text, get(encodingNode, 'BaseEncoding'));
    const fontName = nameOf(text, get(node, 'BaseFont'));
    const fallback = fontName === 'Symbol' ? 'SymbolSetEncoding' : fontName === 'ZapfDingbats' ? 'ZapfDingbatsEncoding' : 'StandardEncoding';
    const encoding = pdfPredefinedEncoding(encodingName || fallback);
    const composite = nameOf(text, get(node, 'Subtype')) === 'Type0' || encodingName.startsWith('Identity-');
    const differences = new Map();
    let code;
    for (const member of arrayElements(get(encodingNode, 'Differences'))) {
      if (member.term === 'integer') code = Number(textOf(text, member));
      else if (member.term === 'name' && code >= 0 && code <= 255) differences.set(code++, pdfGlyphName(nameOf(text, member)));
    }
    const toUnicode = readStream(get(node, 'ToUnicode'));
    const cmap = toUnicode && toUnicode.source.length <= maximumCmapBytes ? readPdfUnicodeMap(toUnicode.source) : null;
    const font = { encoding, differences, composite, cmap };
    fontCache.set(node, font);
    return font;
  }

  function visitContent(node, resources, state, depth = 0) {
    if (!node || depth > 16 || state.active.has(node)) return;
    const stream = readStream(node);
    if (!stream) return;
    const content = parsePdfContentCst(stream.source);
    if (!content || !content.children.some(operation => operation.term === 'operation' && pageOperator(stream.source, operation))) return;
    if (stream.filter && !attached.has(node)) {
      streams.push({ ...stream, tree: content });
      attached.add(node);
    }
    state.active.add(node);
    const { source, data, filter } = stream;
    const startOf = term => filter ? data.start : data.start + term.start;
    const endOf = term => filter ? data.end : data.start + term.end;
    function show(value) {
      if (!['literal_string', 'hex_string'].includes(value?.term)) return;
      const visible = decodePdfFontText(pdfStringBytes(textOf(source, value)), state.font);
      if (!visible) return;
      textRuns.push({
        text: visible, start: startOf(value), end: endOf(value),
        separated: state.separated, position: state.position,
      });
      state.separated = false;
    }
    for (const operation of content.children) {
      if (operation.term !== 'operation') continue;
      const operator = textOf(source, field(operation, 'operator'));
      const argumentsList = operands(operation);
      const kind = pdfTextOperatorKind(operator);
      if (kind === 1) {
        state.font = readFont(get(get(resources, 'Font'), nameOf(source, argumentsList[0])));
      } else if (kind === 2) {
        const form = get(get(resources, 'XObject'), nameOf(source, argumentsList[0]));
        const dictionary = field(form, 'dictionary');
        if (nameOf(text, get(dictionary, 'Subtype')) === 'Form') {
          const font = state.font;
          const ordinate = state.ordinate;
          const position = state.position;
          const leading = state.leading;
          state.orderable = false;
          visitContent(form, get(dictionary, 'Resources') ?? resources, state, depth + 1);
          state.font = font;
          state.ordinate = ordinate;
          state.position = position;
          state.leading = leading;
        }
      } else if (kind === 3) {
        show(argumentsList.at(-1));
      } else if (kind === 4) {
        for (const item of arrayElements(argumentsList.at(-1))) {
          if (item.term === 'integer' || item.term === 'real') {
            if (Number(textOf(source, item)) <= -250) showSpace(item);
          } else show(item);
        }
      } else if (kind === 5) {
        state.separated = true;
        advanceLine(state);
        show(argumentsList.at(-1));
      } else if (kind === 6) {
        state.separated = true;
        if (operator === 'BT') {
          state.ordinate = null;
          state.position = null;
        }
        if (operator === 'T*') advanceLine(state);
      } else if (kind === 7) {
        const ordinate = Number(textOf(source, argumentsList.at(-1)));
        if (operator === 'Tm' ? ordinate !== state.ordinate : ordinate !== 0) state.separated = true;
        state.ordinate = operator === 'Tm' ? ordinate : (state.ordinate ?? 0) + ordinate;
        const values = argumentsList.map(numericOperand);
        if (operator === 'Tm') {
          const [a, b, c, d, x, y] = values;
          if (values.length === 6 && values.every(Number.isFinite) && a > 0 && b === 0 && c === 0 && d > 0) {
            state.position = { x, y, horizontalScale: a, verticalScale: d };
          } else state.orderable = false;
        } else if (values.length === 2 && values.every(Number.isFinite)) {
          const [x, y] = values;
          const previous = state.position ?? { x: 0, y: 0, horizontalScale: 1, verticalScale: 1 };
          state.position = {
            ...previous, x: previous.x + x * previous.horizontalScale, y: previous.y + y * previous.verticalScale,
          };
          if (operator === 'TD') state.leading = -y;
        } else state.orderable = false;
      } else if (operator === 'TL') {
        state.leading = argumentsList.length === 1 ? numericOperand(argumentsList[0]) : NaN;
      } else if (operator === 'cm') {
        state.orderable = false;
      }
    }
    state.active.delete(node);

    function numericOperand(value) {
      return value.term === 'integer' || value.term === 'real' ? Number(textOf(source, value)) : NaN;
    }

    function showSpace(value) {
      const previous = textRuns.at(-1);
      if (previous && !state.separated && !previous.text.endsWith(' ')) {
        textRuns.push({ text: ' ', start: startOf(value), end: endOf(value), separated: false, position: state.position });
      }
    }
  }

  const catalog = [...objects.values()].find(node => nameOf(text, get(node, 'Type')) === 'Catalog');
  const root = get(catalog, 'Pages');
  const frames = root ? [{ node: root, resources: undefined }] : [];
  const visited = new Set();
  while (frames.length) {
    const { node, resources: inherited } = frames.pop();
    if (!node || visited.has(node)) continue;
    visited.add(node);
    const type = nameOf(text, get(node, 'Type'));
    const resources = get(node, 'Resources') ?? inherited;
    if (type === 'Pages') {
      for (const kid of arrayElements(get(node, 'Kids')).reverse()) frames.push({ node: resolve(kid), resources });
    } else if (type === 'Page') {
      const contents = get(node, 'Contents');
      const state = newTextState();
      const pageStart = textRuns.length;
      if (contents?.term === 'array') {
        for (const content of arrayElements(contents)) visitContent(resolve(content), resources, state);
      } else visitContent(contents, resources, state);
      if (state.orderable) orderPageText(textRuns, pageStart);
    }
  }
  if (!root) {
    // Minimal PDFs can contain standalone content streams with no page tree.
    for (const node of objects.values()) {
      if (node?.term === 'stream' && ['', 'XObject'].includes(nameOf(text, get(field(node, 'dictionary'), 'Type')))) {
        const dictionary = field(node, 'dictionary');
        if (['', 'Form'].includes(nameOf(text, get(dictionary, 'Subtype')))) visitContent(node, get(dictionary, 'Resources'), newTextState());
      }
    }
  }
  return { streams, textRuns, errors };
}

function decodeLzw(source, earlyChange, limit) {
  const invalid = 'invalid or oversized LZWDecode PDF stream';
  const dictionarySize = 4096;
  const prefix = new Uint16Array(dictionarySize);
  const suffix = new Uint8Array(dictionarySize);
  const sequence = new Uint8Array(dictionarySize);
  const chunks = [];
  const chunkSize = 8192;
  let chunk = Buffer.alloc(Math.min(chunkSize, limit));
  let chunkLength = 0;
  let outputLength = 0;
  let next = 258;
  let width = 9;
  let previous = -1;
  let bits = 0;
  let accumulator = 0;
  let offset = 0;
  for (;;) {
    while (bits < width) {
      const byte = source.charCodeAt(offset++);
      if (!(byte <= 255)) throw new Error(invalid);
      accumulator = (accumulator << 8) | byte;
      bits += 8;
    }
    bits -= width;
    const code = (accumulator >> bits) & ((1 << width) - 1);
    if (code === 256) {
      next = 258;
      width = 9;
      previous = -1;
      continue;
    }
    if (code === 257) {
      chunks.push(chunk.subarray(0, chunkLength));
      return Buffer.concat(chunks, outputLength).toString('latin1');
    }
    if (code > next || (code >= 258 && previous < 0)) throw new Error(invalid);
    // A code for the next entry denotes the previous entry followed by its first byte.
    const nextEntry = code === next;
    let entry = nextEntry ? previous : code;
    let length = nextEntry ? 1 : 0;
    while (entry >= 258) {
      sequence[length++] = suffix[entry];
      entry = prefix[entry];
    }
    sequence[length++] = entry;
    if (nextEntry) sequence[0] = entry;
    if (outputLength + length > limit) throw new Error(invalid);
    outputLength += length;
    while (length > 0) {
      if (chunkLength === chunk.length) {
        chunks.push(chunk);
        chunk = Buffer.alloc(Math.min(chunkSize, limit - (outputLength - length)));
        chunkLength = 0;
      }
      chunk[chunkLength++] = sequence[--length];
    }
    if (previous >= 0 && next < dictionarySize) {
      prefix[next] = previous;
      suffix[next++] = entry;
      width = pdfLzwCodeWidth(width, next, earlyChange);
    }
    previous = code;
  }
}

function newTextState() {
  return {
    active: new Set(), separated: true, font: null, ordinate: null,
    position: null, leading: 0, orderable: true,
  };
}

function advanceLine(state) {
  if (!state.position || !Number.isFinite(state.leading)) {
    state.position = null;
    return;
  }
  state.position = { ...state.position, y: state.position.y - state.leading * state.position.verticalScale };
}

function orderPageText(runs, start) {
  const lines = [];
  for (let index = start; index < runs.length; index += 1) {
    if (runs[index].separated) {
      const position = runs[index].position;
      if (!position || !Number.isFinite(position.x) || !Number.isFinite(position.y)) return;
      lines.push({ position, start: index, end: index });
    }
    lines.at(-1).end = index + 1;
  }
  lines.sort((left, right) => right.position.y - left.position.y || left.position.x - right.position.x);
  const ordered = lines.flatMap(line => runs.slice(line.start, line.end));
  for (const [index, run] of ordered.entries()) runs[start + index] = run;
}

function pageOperator(source, operation) {
  const operator = textOf(source, field(operation, 'operator'));
  return pdfTextOperatorKind(operator) !== 0 || pdfDrawingOperator(operator);
}

function decodeAscii85(source, limit) {
  const text = source.replace(pdfWhitespace, '').replace(/^<~/u, '').replace(/~>$/u, '');
  let zeroGroups = 0;
  for (const character of text) if (character === 'z') zeroGroups += 1;
  const bytes = Buffer.alloc(Math.min((Math.ceil((text.length - zeroGroups) / 5) + zeroGroups) * 4, limit + 1));
  let value = 0;
  let count = 0;
  let position = 0;
  function append(value, count) {
    if (value > 0xffffffff || position + count > bytes.length) throw new Error('invalid or oversized ASCII85Decode PDF stream');
    for (let index = 0; index < count; index += 1) bytes[position++] = value >>> (24 - 8 * index);
  }
  for (const character of text) {
    if (character === 'z' && count === 0) {
      append(0, 4);
    } else {
      const digit = character.charCodeAt(0) - 33;
      if (digit < 0 || digit > 84) throw new Error('invalid ASCII85Decode PDF stream');
      value = value * 85 + digit;
      count += 1;
      if (count === 5) { append(value, 4); value = 0; count = 0; }
    }
  }
  if (count === 1) throw new Error('incomplete ASCII85Decode PDF stream');
  if (count > 1) {
    for (let padding = count; padding < 5; padding += 1) value = value * 85 + 84;
    append(value, count - 1);
  }
  return bytes.toString('latin1', 0, position);
}
