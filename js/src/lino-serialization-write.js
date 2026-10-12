// Canonical link writing and scalar escaping without a parser dependency.
import { LinkFlags, LinkType } from './primitives.js';

const textEncoder = new TextEncoder();
const textDecoder = new TextDecoder('utf-8', { fatal: true, ignoreBOM: true });

const LINK_TYPE_TOKENS = Object.freeze({
  [LinkType.Concept]: 'concept',
  [LinkType.Document]: 'document',
  [LinkType.Dynamic]: 'link',
  [LinkType.Field]: 'field',
  [LinkType.Grammar]: 'grammar',
  [LinkType.Language]: 'language',
  [LinkType.Object]: 'object',
  [LinkType.Reference]: 'reference',
  [LinkType.Region]: 'region',
  [LinkType.Relation]: 'relation',
  [LinkType.Semantic]: 'semantic',
  [LinkType.SourceToken]: 'token',
  [LinkType.Syntax]: 'syntax',
  [LinkType.Trivia]: 'trivia',
  [LinkType.Type]: 'type',
});

const LINK_TYPES_BY_TOKEN = Object.freeze(Object.fromEntries(
  Object.entries(LINK_TYPE_TOKENS).map(([linkType, token]) => [token, linkType]),
));

export class LinoSerializationError extends Error {
  constructor(kind, message) {
    super(kind === 'parse'
      ? `links-notation parse error: ${message}`
      : `serialization structure error: ${message}`);
    this.name = 'LinoSerializationError';
    this.kind = kind;
  }
}

export function structureError(message) {
  return new LinoSerializationError('structure', message);
}

/** The canonical token of a link type, as Rust's `Display for LinkType` writes it. */
export function linkTypeToken(linkType) {
  return LINK_TYPE_TOKENS[linkType] ?? String(linkType).toLowerCase();
}

/** The link type of a canonical token; throws on an unknown token. */
export function linkTypeFromToken(token) {
  const linkType = LINK_TYPES_BY_TOKEN[token];
  if (linkType === undefined) {
    throw structureError(`unknown link type \`${token}\``);
  }
  return linkType;
}

/** Percent-encodes a string into a single escape-free Links Notation token. */
export function percentEncode(value) {
  const source = String(value);
  if (source.length === 0) {
    return '%';
  }
  if (/^[0-9A-Za-z_.-]+$/u.test(source)) return source;
  let encoded = '';
  for (const byte of textEncoder.encode(source)) {
    const unreserved = (byte >= 0x30 && byte <= 0x39)
      || (byte >= 0x41 && byte <= 0x5a)
      || (byte >= 0x61 && byte <= 0x7a)
      || byte === 0x2d || byte === 0x5f || byte === 0x2e;
    encoded += unreserved
      ? String.fromCharCode(byte)
      : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`;
  }
  return encoded;
}

/** Reverses `percentEncode`. */
export function percentDecode(value) {
  if (value === '%') {
    return '';
  }
  if (typeof value === 'string' && /^[0-9A-Za-z_.-]+$/u.test(value)) return value;
  const bytes = [];
  for (let index = 0; index < value.length;) {
    if (value[index] === '%') {
      if (index + 2 >= value.length) {
        throw structureError('truncated percent escape');
      }
      const hex = value.slice(index + 1, index + 3);
      if (!/^[0-9A-Fa-f]{2}$/.test(hex)) {
        throw structureError('invalid percent escape digit');
      }
      bytes.push(Number.parseInt(hex, 16));
      index += 3;
    } else {
      bytes.push(...textEncoder.encode(value[index]));
      index += 1;
    }
  }
  try {
    return textDecoder.decode(Uint8Array.from(bytes));
  } catch {
    throw structureError('percent escape is not valid UTF-8');
  }
}

function flagBits(flags) {
  return (flags.isError ? 0b0001 : 0)
    | (flags.hasError ? 0b0010 : 0)
    | (flags.isMissing ? 0b0100 : 0)
    | (flags.isExtra ? 0b1000 : 0);
}

/** Writes one `(<id>: <refs> (meta: ...))` statement. */
export function encodeLinkStatement(id, references, metadata, registered = false) {
  let output = `(${Number(id)}:`;
  for (const reference of references) {
    output += ` ${Number(reference)}`;
  }
  output += ' (meta:';
  if (metadata.linkType !== undefined) {
    output += ` (t: ${linkTypeToken(metadata.linkType)})`;
  }
  output += ` (n: ${metadata.named ? 1 : 0})`;
  if (metadata.term !== undefined) {
    output += ` (term: ${percentEncode(metadata.term)})`;
  }
  if (metadata.definition !== undefined) {
    output += ` (def: ${percentEncode(metadata.definition)})`;
  }
  if (metadata.language !== undefined) {
    output += ` (lang: ${percentEncode(metadata.language)})`;
  }
  if (metadata.span) {
    const { byteRange, start, end } = metadata.span;
    output += ` (span: ${byteRange.start} ${byteRange.end} ${start.row} ${start.column} ${end.row} ${end.column})`;
  }
  const bits = flagBits(metadata.flags ?? LinkFlags.clean());
  if (bits !== 0) {
    output += ` (flags: ${bits})`;
  }
  if (registered) {
    output += ' (reg: 1)';
  }
  return `${output}))`;
}

/** Serializes every link of a network, one statement per line, in id order. */
export function encodeNetworkLino(links, registeredIds) {
  let output = '';
  for (const link of links) {
    const id = link.id().asU64();
    output += `${encodeLinkStatement(id, link.references(), link.metadata(), registeredIds.has(id))}\n`;
  }
  return output;
}

