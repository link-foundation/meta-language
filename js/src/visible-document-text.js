import { documentCharacterReference } from './document-character-references.js';
import { documentProjectionRules } from './document-projection-rules.js';
import { canonicalLanguageName } from './language-catalog.js';
import { LinkType } from './primitives.js';
import { documentScalarIsWhitespace } from './document-text-policy.js';
import { binaryStringCoordinates } from './binary-string-source.js';
import { readBinaryString } from './open-packaging-conventions.js';

const utf8 = new TextEncoder();
const readUtf8 = new TextDecoder('utf-8', { ignoreBOM: true });

const rangeOrder = (left, right) => left.start - right.start || right.end - left.end;
const contains = (outer, inner) => outer.start <= inner.start && inner.end <= outer.end;

function syntaxLinks(network) {
  const links = new Map();
  for (const link of network.links()) {
    const metadata = link.metadata();
    const range = metadata.span?.byteRange;
    if (metadata.linkType !== LinkType.Syntax || !range || range.start === range.end) continue;
    links.set(link.id().value, {
      start: range.start, end: range.end, term: metadata.term,
      language: canonicalLanguageName(metadata.language ?? ''),
      references: link.references(), children: [],
    });
  }
  const pointsToParent = [...links.values()].some(record => record.references.some(id =>
    network.link(id)?.metadata().linkType === LinkType.Document));
  for (const record of links.values()) {
    for (const reference of record.references) {
      const target = links.get(reference.value);
      if (!target || target === record || target.language !== record.language) continue;
      if (contains(record, target) && !(pointsToParent && target.start === record.start && target.end === record.end)) {
        record.children.push(target);
      } else if (contains(target, record)) target.children.push(record);
    }
  }
  for (const record of links.values()) record.children.sort(rangeOrder);
  return [...links.values()].sort(rangeOrder);
}

function markupName(tag, read, kinds) {
  if (!tag) return '';
  const name = tag.children.find(child => kinds.includes(child.term));
  return name ? read(name.start, name.end) : '';
}

function htmlName(tag, read) {
  return markupName(tag, read, ['tag_name', 'erroneous_end_tag_name']).toLowerCase();
}

function htmlEdits(records, read) {
  const hiddenHtml = documentProjectionRules('hiddenHtml');
  const hiddenSyntax = documentProjectionRules('hiddenHtmlSyntax');
  const htmlTags = documentProjectionRules('htmlTag');
  const htmlBlocks = documentProjectionRules('htmlBlock');
  const edits = [];
  for (const record of records) {
    const { start, end, term, children } = record;
    if (hiddenSyntax.includes(term)) {
      edits.push({ start, end, kind: 'skip' });
      continue;
    }
    if (term === 'element' || term === 'markup_element') {
      const tag = children.find(child => htmlTags.includes(child.term));
      if (hiddenHtml.includes(htmlName(tag, read))) edits.push({ start, end, kind: 'skip' });
      continue;
    }
    if (!htmlTags.includes(term)) continue;
    const name = htmlName(record, read);
    const kind = name === 'br' ? 'break'
      : name === 'li' && term === 'start_tag' ? 'bullet'
        : htmlBlocks.includes(name) ? 'separator'
          : term === 'end_tag' && (name === 'td' || name === 'th') ? 'space' : 'skip';
    edits.push({ start, end, kind });
  }
  return edits;
}

function markdownEdits(records, read) {
  const links = documentProjectionRules('markdownLink');
  const hiddenSyntax = documentProjectionRules('markdownHiddenSyntax');
  const edits = [];
  for (const record of records) {
    const { start, end, term, children } = record;
    if (links.includes(term)) {
      const label = children.find(child => child.term === 'link_text' || child.term === 'image_description');
      edits.push(...(label ? [{ start, end: label.start, kind: 'skip' }, { start: label.end, end, kind: 'skip' }]
        : [{ start, end, kind: 'skip' }]));
    } else if (term === 'code_span') {
      const delimiters = children.filter(child => child.term === 'code_span_delimiter');
      if (delimiters.length !== 2) continue;
      let textStart = delimiters[0].end;
      let textEnd = delimiters[1].start;
      const code = read(textStart, textEnd);
      if (/^[ \r\n]/u.test(code) && /[ \r\n]$/u.test(code) && /[^ \r\n]/u.test(code)) {
        textStart += code.startsWith('\r\n') ? 2 : 1;
        textEnd -= code.endsWith('\r\n') ? 2 : 1;
      }
      edits.push({ start, end, kind: 'code', textStart, textEnd });
    } else if (hiddenSyntax.includes(term)) {
      edits.push({ start, end, kind: 'skip' });
    } else if (term === 'backslash_escape') {
      edits.push({ start, end, kind: 'replace', text: read(start + 1, end) });
    } else if (term === 'hard_line_break') {
      edits.push({ start, end, kind: 'break' });
    } else if (term === 'uri_autolink' || term === 'email_autolink') {
      edits.push({ start, end: start + 1, kind: 'skip' }, { start: end - 1, end, kind: 'skip' });
    }
  }
  return edits;
}

function markdownSelections(records, embeddedHtml, read) {
  const containers = documentProjectionRules('markdownTextContainer');
  const selections = records.filter(record => containers.includes(record.term));
  const edits = markdownEdits(records, read);
  const tags = htmlEdits(embeddedHtml, read);
  const markers = records.filter(record => record.term.startsWith('list_marker_'));
  let lastEnd = 0;
  let nextEdit = 0;
  let nextTag = 0;
  let nextMarker = 0;
  const chunks = [];
  // Markdown block and inline records are nested, with source-ordered leaves.
  edits.sort(rangeOrder);
  tags.sort(rangeOrder);
  for (const selection of selections) {
    const { start, end, term } = selection;
    if (start < lastEnd) continue;
    while (nextEdit < edits.length && edits[nextEdit].start < start) nextEdit += 1;
    while (nextTag < tags.length && tags[nextTag].start < start) nextTag += 1;
    let marker = null;
    while (nextMarker < markers.length && markers[nextMarker].end <= start) {
      const candidate = markers[nextMarker++];
      if (candidate.start >= lastEnd) marker = candidate;
    }
    const kind = term === 'pipe_table_cell' && chunks.length > 0 && !/[\r\n]/u.test(read(lastEnd, start)) ? 'space' : 'separator';
    chunks.push({ kind, start: lastEnd, end: marker?.start ?? start });
    if (marker) chunks.push({ kind: 'bullet', start: marker.start, end: marker.end });
    if (term === 'html_block') {
      projectEdits(selection, tags, nextTag, chunks, 'html');
    } else if (term === 'code_fence_content' || term === 'indented_code_block') {
      chunks.push({ kind: 'text', start, end, syntax: null, preserve: true });
    } else {
      projectEdits(selection, edits, nextEdit, chunks, 'markdown');
    }
    lastEnd = end;
  }
  return chunks;
}

function docxSelections(records, read) {
  const chunks = [];
  const namespaceBindings = new Map([['w', 'http://schemas.openxmlformats.org/wordprocessingml/2006/main']]);
  const elementScopes = [];
  for (const record of records) {
    if (record.term !== 'element') continue;
    while (elementScopes.length && elementScopes.at(-1).end <= record.start) {
      for (const [prefix, previous] of elementScopes.pop().bindings) {
        if (previous === undefined) namespaceBindings.delete(prefix);
        else namespaceBindings.set(prefix, previous);
      }
    }
    const open = record.children.find(child => child.term === 'STag' || child.term === 'EmptyElemTag');
    if (!open) continue;
    const bindings = new Map();
    for (const attribute of open.children.filter(child => child.term === 'Attribute')) {
      const externalName = markupName(attribute, read, ['Name']);
      if (externalName !== 'xmlns' && !externalName.startsWith('xmlns:')) continue;
      const value = attribute.children.find(child => child.term === 'AttValue');
      if (!value) continue;
      const prefix = externalName.slice(6);
      if (!bindings.has(prefix)) bindings.set(prefix, namespaceBindings.get(prefix));
      namespaceBindings.set(prefix, read(value.start + 1, value.end - 1));
    }
    if (bindings.size) elementScopes.push({ end: record.end, bindings });
    const qualified = markupName(open, read, ['Name']);
    const separator = qualified.indexOf(':');
    const namespace = namespaceBindings.get(separator === -1 ? '' : qualified.slice(0, separator));
    if (namespace !== 'http://schemas.openxmlformats.org/wordprocessingml/2006/main' &&
        namespace !== 'http://purl.oclc.org/ooxml/wordprocessingml/main') continue;
    const name = qualified.slice(separator + 1);
    if (name === 'p') chunks.push({ kind: 'separator', start: open.start, end: open.end });
    if (name === 'tab') chunks.push({ kind: 'replace', text: '\t', start: open.start, end: open.end });
    if (name === 'br' || name === 'cr') chunks.push({ kind: 'break', start: open.start, end: open.end });
    if (name !== 't') continue;
    const close = record.children.find(child => child.term === 'ETag');
    if (!close || open.end >= close.start) continue;
    const content = record.children.find(child => child.term === 'content');
    let start = open.end;
    for (const child of content?.children ?? []) {
      if (!['CDSect', 'Comment', 'PI', 'element'].includes(child.term)) continue;
      if (start < child.start) chunks.push({ kind: 'text', syntax: 'xml', start, end: child.start, preserve: true });
      const literal = child.children.find(part => part.term === 'CData');
      if (literal) chunks.push({ kind: 'text', start: literal.start, end: literal.end, preserve: true });
      start = child.end;
    }
    if (start < close.start) chunks.push({ kind: 'text', syntax: 'xml', start, end: close.start, preserve: true });
  }
  return chunks.sort(rangeOrder);
}

function projectEdits(selection, edits, first, chunks, syntax) {
  let cursor = selection.start;
  for (let index = first; index < edits.length && edits[index].start < selection.end; index += 1) {
    const edit = edits[index];
    if (edit.start < cursor || !contains(selection, edit)) continue;
    if (edit.start > cursor) chunks.push({ kind: 'text', start: cursor, end: edit.start, syntax });
    chunks.push(edit);
    cursor = edit.end;
  }
  if (cursor < selection.end) chunks.push({ kind: 'text', start: cursor, end: selection.end, syntax });
}

function projectChunks(chunks, read, options, sourceOrdered = true) {
  const parts = [];
  const map = [];
  let textEnd = 0;
  let pendingSpace = null;
  const separators = [];

  function append(text, start, end, kind) {
    if (text.length === 0) return;
    const previous = map.at(-1);
    const next = textEnd + text.length;
    if (kind === 'source' && previous?.kind === 'source' && previous.sourceEnd === start &&
        previous.sourceEnd - previous.sourceStart === previous.textEnd - previous.textStart && text.length === end - start) {
      previous.textEnd = next;
      previous.sourceEnd = end;
    } else {
      map.push({ textStart: textEnd, textEnd: next, sourceStart: start, sourceEnd: end, kind });
    }
    parts.push(text);
    textEnd = next;
  }

  function boundary(chunk, force) {
    pendingSpace = null;
    if (textEnd === 0) return;
    if (!force && separators.length > 0) return;
    if (force && separators.length === 1 && separators[0].kind !== 'break') separators.pop();
    separators.push(chunk);
  }

  function emit(text, start, end, kind, preserve = false) {
    if (options.nbsp === 'space' && text.includes('\u00a0')) {
      text = text.replaceAll('\u00a0', ' ');
      if (kind === 'source') kind = 'replacement';
    }
    if (options.collapseWhitespace && !preserve && [...text].every(character => documentScalarIsWhitespace(character.codePointAt(0)))) {
      if (textEnd === 0 || separators.length > 0) return;
      pendingSpace = { start: pendingSpace?.start ?? start, end };
      return;
    }
    for (const separator of separators) append(options.blockSeparator, separator.start, separator.end, 'separator');
    separators.length = 0;
    if (pendingSpace) append(' ', pendingSpace.start, pendingSpace.end, 'whitespace');
    pendingSpace = null;
    append(text, start, end, kind);
  }

  function readText(chunk) {
    const source = read(chunk.start, chunk.end);
    let byte = chunk.start;
    for (let index = 0; index < source.length;) {
      const reference = options.decodeEntities && chunk.syntax && source[index] === '&'
        ? documentCharacterReference(source, index, chunk.syntax) : null;
      const character = String.fromCodePoint(source.codePointAt(index));
      const next = reference?.end ?? index + character.length;
      let value = reference?.text ?? character;
      if (chunk.code && (value === '\r' || value === '\n')) {
        value = ' ';
        if (character === '\r' && source[next] === '\n') {
          emit(value, byte, byte + 2, 'replacement', true);
          byte += 2;
          index = next + 1;
          continue;
        }
      }
      const length = reference ? next - index : documentScalarByteWidth(character.codePointAt(0));
      emit(value, byte, byte + length, reference ? 'reference' : value === character ? 'source' : 'replacement', chunk.preserve);
      byte += length;
      index = next;
    }
  }

  for (const chunk of chunks) {
    switch (chunk.kind) {
      case 'skip': break;
      case 'separator': case 'break': boundary(chunk, chunk.kind === 'break'); break;
      case 'bullet':
        boundary(options.listBullet ? { ...chunk, end: chunk.start } : chunk, false);
        if (options.listBullet) emit(options.listBullet, chunk.start, chunk.end, 'bullet', true);
        break;
      case 'space': emit(' ', chunk.start, chunk.end, 'whitespace'); break;
      case 'replace': emit(chunk.text, chunk.start, chunk.end, 'replacement', true); break;
      case 'code': readText({ start: chunk.textStart, end: chunk.textEnd, code: true, preserve: true }); break;
      default: readText(chunk);
    }
  }
  return new VisibleDocumentText(parts.join(''), map, sourceOrdered);
}

class VisibleDocumentText {
  constructor(text, map, sourceOrdered = true) {
    this.text = text;
    this.map = map;
    Object.defineProperty(this, 'sourceOrdered', { value: sourceOrdered });
  }

  sourceRange(start, end) {
    if (!Number.isSafeInteger(start) || !Number.isSafeInteger(end) || start < 0 || end <= start || end > this.text.length) return null;
    const first = spanAt(this.map, start);
    const last = spanAt(this.map, end - 1);
    if (!this.sourceOrdered && first !== last) {
      let sourceStart = Infinity;
      let sourceEnd = 0;
      for (const span of this.map) {
        if (span.textEnd <= start) continue;
        if (span.textStart >= end) break;
        sourceStart = Math.min(sourceStart, sourceOffset(span, Math.max(start, span.textStart), false));
        sourceEnd = Math.max(sourceEnd, sourceOffset(span, Math.min(end, span.textEnd), true));
      }
      return { start: sourceStart, end: sourceEnd };
    }
    return { start: sourceOffset(first, start, false), end: sourceOffset(last, end, true) };
  }
}

function documentScalarByteWidth(value) {
  if (value < 0x80) return 1;
  if (value < 0x800) return 2;
  if (value < 0x10000) return 3;
  return 4;
}

function spanAt(map, offset) {
  let first = 0;
  let last = map.length;
  while (first < last) {
    const middle = first + Math.floor((last - first) / 2);
    if (map[middle].textEnd <= offset) first = middle + 1;
    else last = middle;
  }
  return map[first];
}

function sourceOffset(span, offset, end) {
  if (span.kind === 'source' && span.sourceEnd - span.sourceStart === span.textEnd - span.textStart) {
    return span.sourceStart + offset - span.textStart;
  }
  return end ? span.sourceEnd : span.sourceStart;
}

function projectPackagedText(documents, read, options) {
  const parts = [];
  const map = [];
  let textEnd = 0;
  for (const document of documents) {
    const projection = document.network.visibleText(options);
    if (!projection.text) continue;
    const { start, end } = document.sourceRange;
    if (textEnd && options.blockSeparator) {
      parts.push(options.blockSeparator);
      map.push({ textStart: textEnd, textEnd: textEnd + options.blockSeparator.length,
        sourceStart: start, sourceEnd: start, kind: 'separator' });
      textEnd += options.blockSeparator.length;
    }
    if (document.compression === 'stored') {
      const coordinates = document.encoding === 'binary-string'
        ? binaryStringCoordinates(readBinaryString(read(start, end))) : null;
      for (const span of projection.map) {
        const range = coordinates?.(span.sourceStart, span.sourceEnd).byteRange;
        const sourceStart = start + (range?.start ?? span.sourceStart);
        const sourceEnd = start + (range?.end ?? span.sourceEnd);
        map.push({ ...span, textStart: span.textStart + textEnd, textEnd: span.textEnd + textEnd, sourceStart, sourceEnd });
      }
    } else {
      map.push({ textStart: textEnd, textEnd: textEnd + projection.text.length,
        sourceStart: start, sourceEnd: end, kind: 'replacement' });
    }
    parts.push(projection.text);
    textEnd += projection.text.length;
  }
  return new VisibleDocumentText(parts.join(''), map, false);
}

/** Projects container syntax into text, keeping the original UTF-8 source ranges. */
export function projectVisibleDocumentText(network, {
  blockSeparator = '\n', decodeEntities = true, collapseWhitespace = true, nbsp = 'space', listBullet = '',
} = {}) {
  if (nbsp !== 'space' && nbsp !== 'keep') throw new TypeError('nbsp must be space or keep');
  const sourceLanguage = network.links().find(link => link.metadata().linkType === LinkType.Language);
  const language = canonicalLanguageName(sourceLanguage?.metadata().term ?? '');
  const options = { blockSeparator, decodeEntities, collapseWhitespace, nbsp, listBullet };
  if (language === 'PDF') {
    const chunks = [];
    for (const link of network.links()) {
      const { term, span, language, definition, linkType } = link.metadata();
      if (language !== 'PDF' || linkType !== LinkType.Semantic || !span || !definition ||
          (term !== 'PDF text line' && term !== 'PDF text run')) continue;
      const { start, end } = span.byteRange;
      if (term === 'PDF text line') chunks.push({ kind: 'separator', start, end: start });
      chunks.push({ kind: 'replace', start, end, text: definition });
    }
    return projectChunks(chunks, null, options, false);
  }
  const packaged = language === 'DOCX' && network.links().some(link => link.metadata().term === 'docx_package');
  const documents = packaged ? network.derivedDocuments() : [];
  const source = !packaged || documents.some(document => document.encoding === 'binary-string')
    ? utf8.encode(network.reconstructText()) : new Uint8Array();
  const read = (start, end) => readUtf8.decode(source.subarray(start, end));
  if (packaged) return projectPackagedText(documents, read, options);
  const records = ['HTML', 'Markdown', 'DOCX'].includes(language) ? syntaxLinks(network) : [];
  const ownRecords = records.filter(record => record.language === language);
  let chunks = [];
  if (language === 'HTML') {
    const edits = htmlEdits(ownRecords, read).sort(rangeOrder);
    projectEdits({ start: 0, end: source.length }, edits, 0, chunks, 'html');
  } else if (language === 'Markdown') {
    chunks = markdownSelections(ownRecords, records.filter(record => record.language === 'HTML'), read);
  } else if (language === 'DOCX') {
    chunks = docxSelections(ownRecords, read);
  } else {
    chunks.push({ kind: 'text', start: 0, end: source.length, preserve: true });
  }
  return projectChunks(chunks, read, options);
}
