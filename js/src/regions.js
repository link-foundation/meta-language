import { ByteRange, Point, SourceSpan } from './primitives.js';

const TXT_LANGUAGE = 'txt';

const textEncoder = new TextEncoder();

/// Region detection strategy for mixed-language parsing.
export const RegionDetectionPolicy = Object.freeze({
  NameDriven: 'NameDriven',
  ContentDriven: 'ContentDriven',
  Both: 'Both',
});

/// Embedded region discovered inside a mixed-language document.
export class EmbeddedRegion {
  constructor(language, span) {
    this._language = language;
    this._span = span;
  }

  language() {
    return this._language;
  }

  span() {
    return this._span;
  }
}

export function detectEmbeddedRegions(text, language, policy) {
  const regions = [];
  switch (String(language).toLowerCase()) {
    case TXT_LANGUAGE:
      regions.push(regionFor(text, TXT_LANGUAGE, 0, byteLength(text)));
      break;
    case 'markdown':
      regions.push(...detectMarkdownFencedRegions(text, policy));
      regions.push(...detectMarkdownHtmlRegions(text));
      break;
    case 'html':
      regions.push(...detectHtmlElementRegions(text, 'script', 'JavaScript'));
      regions.push(...detectHtmlElementRegions(text, 'style', 'CSS'));
      regions.push(...detectHtmlStyleAttributes(text));
      break;
    default:
      break;
  }
  return regions;
}

function detectMarkdownFencedRegions(text, policy) {
  const regions = [];
  let offset = 0;
  let openFence = null;

  for (const line of splitInclusive(text, '\n')) {
    const trimmed = trimStart(trimEndMatches(line, ['\r', '\n']));
    const lineBytes = byteLength(line);
    if (openFence !== null) {
      const { languageTag, contentStart } = openFence;
      if (trimmed.startsWith('```')) {
        openFence = null;
        const language = regionLanguageFromTagOrContent(
          languageTag,
          sliceBytes(text, contentStart, offset),
          policy,
        );
        if (language !== null) {
          regions.push(regionFor(text, language, contentStart, offset));
        }
      }
      // else: keep current openFence as-is
    } else if (trimmed.startsWith('```')) {
      const rest = trimmed.slice('```'.length);
      const languageTag = firstWhitespaceToken(rest);
      openFence = { languageTag, contentStart: offset + lineBytes };
    }
    offset += lineBytes;
  }

  if (openFence !== null) {
    const { languageTag, contentStart } = openFence;
    const language = regionLanguageFromTagOrContent(
      languageTag,
      sliceBytes(text, contentStart, byteLength(text)),
      policy,
    );
    if (language !== null) {
      regions.push(regionFor(text, language, contentStart, byteLength(text)));
    }
  }

  return regions;
}

function regionLanguageFromTagOrContent(languageTag, content, policy) {
  switch (policy) {
    case RegionDetectionPolicy.NameDriven:
      return languageTag.length > 0 ? languageTag : null;
    case RegionDetectionPolicy.ContentDriven:
      return sniffLanguage(content) ?? TXT_LANGUAGE;
    case RegionDetectionPolicy.Both:
      if (languageTag.length === 0) {
        return sniffLanguage(content) ?? TXT_LANGUAGE;
      }
      return languageTag;
    default:
      return null;
  }
}

function detectMarkdownHtmlRegions(text) {
  const regions = [];
  let searchStart = 0;

  for (;;) {
    const start = byteIndexOf(text, '<', searchStart);
    if (start < 0) {
      break;
    }
    const nextChar = charAtByte(text, start + 1);
    if (nextChar === undefined) {
      break;
    }
    if (!isAsciiAlphabetic(nextChar)) {
      searchStart = start + 1;
      continue;
    }

    const close = byteIndexOf(text, '>', start);
    if (close < 0) {
      break;
    }
    const firstTagEnd = close + 1;
    const tagName = trimMatches(
      firstWhitespaceToken(sliceBytes(text, start + 1, firstTagEnd - 1)),
      '/',
    ).toLowerCase();
    if (tagName.length === 0) {
      searchStart = firstTagEnd;
      continue;
    }

    const closingTag = `</${tagName}>`;
    const tail = sliceBytes(text, firstTagEnd).toLowerCase();
    const relativeEnd = tail.indexOf(closingTag);
    const regionEnd = relativeEnd < 0
      ? firstTagEnd
      : firstTagEnd + byteLength(tail.slice(0, relativeEnd)) + byteLength(closingTag);
    regions.push(regionFor(text, 'HTML', start, regionEnd));
    searchStart = regionEnd;
  }

  return regions;
}

function detectHtmlElementRegions(text, element, language) {
  const regions = [];
  const lower = text.toLowerCase();
  const open = `<${element}`;
  const close = `</${element}>`;
  let searchStart = 0;

  for (;;) {
    const start = byteIndexOfLower(text, lower, open, searchStart);
    if (start < 0) {
      break;
    }
    const openEnd = byteIndexOfLower(text, lower, '>', start);
    if (openEnd < 0) {
      break;
    }
    const contentStart = openEnd + 1;
    const closeStart = byteIndexOfLower(text, lower, close, contentStart);
    if (closeStart < 0) {
      break;
    }
    const contentEnd = closeStart;
    regions.push(regionFor(text, language, contentStart, contentEnd));
    searchStart = contentEnd + byteLength(close);
  }

  return regions;
}

function detectHtmlStyleAttributes(text) {
  const regions = [];
  const lower = text.toLowerCase();
  const marker = 'style="';
  let searchStart = 0;

  for (;;) {
    const markerStart = byteIndexOfLower(text, lower, marker, searchStart);
    if (markerStart < 0) {
      break;
    }
    const valueStart = markerStart + byteLength(marker);
    const valueEnd = byteIndexOf(text, '"', valueStart);
    if (valueEnd < 0) {
      break;
    }
    regions.push(regionFor(text, 'CSS', valueStart, valueEnd));
    searchStart = valueEnd + 1;
  }

  return regions;
}

export function sniffLanguage(content) {
  const trimmed = trimStart(content);
  const upper = trimmed.toUpperCase();

  if (trimmed.includes('fn main')) {
    return 'rust';
  }
  if (trimmed.startsWith('def ')) {
    return 'Python';
  }
  if (trimmed.startsWith('<')) {
    return 'HTML';
  }
  if (
    trimmed.includes('function ') ||
    trimmed.includes('const ') ||
    trimmed.includes('let ')
  ) {
    return 'JavaScript';
  }
  if (upper.startsWith('SELECT ')) {
    return 'sql-ansi';
  }
  return null;
}

function regionFor(text, language, start, end) {
  return new EmbeddedRegion(
    language,
    new SourceSpan(
      new ByteRange(start, end),
      pointAtByte(text, start),
      pointAtByte(text, end),
    ),
  );
}

function pointAtByte(text, byte) {
  let row = 0;
  let column = 0;
  let index = 0;

  for (const character of text) {
    if (index >= byte) {
      break;
    }
    if (character === '\n') {
      row += 1;
      column = 0;
    } else {
      column += 1;
    }
    index += byteLength(character);
  }

  return new Point(row, column);
}

// --- UTF-8 byte helpers ---

function byteLength(value) {
  return textEncoder.encode(value).length;
}

// Slice `text` by UTF-8 byte offsets, returning a JS string.
function sliceBytes(text, startByte, endByte) {
  const bytes = textEncoder.encode(text);
  const end = endByte === undefined ? bytes.length : endByte;
  return new TextDecoder().decode(bytes.slice(startByte, end));
}

// Byte offset of the first occurrence of `needle` (a string) in `text` at or
// after byte offset `from`, or -1.
function byteIndexOf(text, needle, from) {
  const prefix = sliceBytes(text, 0, from);
  const tail = sliceBytes(text, from);
  const relative = tail.indexOf(needle);
  if (relative < 0) {
    return -1;
  }
  return byteLength(prefix) + byteLength(tail.slice(0, relative));
}

// Same as byteIndexOf but searching in the lowercased text `lower` for `needle`
// (already lowercased), returning a byte offset into the original `text`.
function byteIndexOfLower(text, lower, needle, from) {
  const lowerTail = sliceBytes(lower, from);
  const relative = lowerTail.indexOf(needle);
  if (relative < 0) {
    return -1;
  }
  return from + byteLength(lowerTail.slice(0, relative));
}

// Character starting at byte offset `byte`, or undefined if out of range.
function charAtByte(text, byte) {
  const tail = sliceBytes(text, byte);
  return tail.length > 0 ? tail[Symbol.iterator]().next().value : undefined;
}

// --- string helpers mirroring Rust ---

function splitInclusive(text, separator) {
  if (text.length === 0) {
    return [];
  }
  const parts = [];
  let current = '';
  for (const character of text) {
    current += character;
    if (character === separator) {
      parts.push(current);
      current = '';
    }
  }
  if (current.length > 0) {
    parts.push(current);
  }
  return parts;
}

function trimEndMatches(value, characters) {
  const set = new Set(characters);
  let end = value.length;
  while (end > 0 && set.has(value[end - 1])) {
    end -= 1;
  }
  return value.slice(0, end);
}

function trimStart(value) {
  return value.replace(/^\s+/, '');
}

function trimMatches(value, character) {
  let start = 0;
  let end = value.length;
  while (start < end && value[start] === character) {
    start += 1;
  }
  while (end > start && value[end - 1] === character) {
    end -= 1;
  }
  return value.slice(start, end);
}

function firstWhitespaceToken(value) {
  const match = value.match(/\S+/);
  return match ? match[0] : '';
}

function isAsciiAlphabetic(character) {
  return /^[A-Za-z]$/.test(character);
}
