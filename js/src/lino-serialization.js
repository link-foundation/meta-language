// Lossless serialization of a link network to and from Links Notation text.
//
// Mirrors `rust/src/lino_serialization.rs`: every link becomes one statement
//
//   (<id>: <ref> ... (meta: (t: <type>) (n: <0|1>) (term: <pct>) ...))
//
// keyed by its numeric id. String payloads (`term`, `def`, `lang`) are
// percent-encoded so they always form a single escape-free reference token.
// Both runtimes write byte-identical text for the same network and read each
// other's output back into the same links, terms and metadata.

import { Parser } from 'links-notation';

import {
  ByteRange,
  LinkFlags,
  LinkMetadata,
  LinkType,
  Point,
  SourceSpan,
} from './primitives.js';

const textEncoder = new TextEncoder();
const textDecoder = new TextDecoder('utf-8', { fatal: true });

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

function structureError(message) {
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

/** Whether parsed Links Notation statements use the canonical `meta` sublink form. */
export function hasMetaSublinks(statements) {
  return statements.some((statement) => (
    (statement.values ?? []).some((value) => value.id === 'meta' && (value.values ?? []).length > 0)
  ));
}

/** Parses Links Notation text the way the canonical reader does (comments off). */
export function parseLinoStatements(source) {
  try {
    return new Parser({ comments: false }).parse(source);
  } catch (error) {
    throw new LinoSerializationError('parse', error.message);
  }
}

function parseNumber(value, message) {
  if (value === undefined || value === null || !/^\d+$/.test(String(value))) {
    throw structureError(message(value));
  }
  return Number(value);
}

/**
 * Decodes canonical statements into `{ id, references, metadata, registered }`
 * records. Throws `LinoSerializationError` on the same inputs Rust's
 * `LinkNetwork::from_lino` rejects.
 */
export function decodeCanonicalStatements(statements) {
  const records = [];
  for (const statement of statements) {
    if (statement.id === null || statement.id === undefined || statement.values.length === 0) {
      throw structureError('top-level statement must be an identified link');
    }
    const id = parseNumber(statement.id, (value) => `invalid link id \`${value}\``);
    const references = [];
    let metaFields;
    for (const value of statement.values) {
      if (value.values.length === 0 && value.id !== null) {
        references.push(parseNumber(value.id, (text) => `invalid link id \`${text}\``));
      } else if (value.id === 'meta') {
        metaFields = value.values;
      } else {
        throw structureError('statement values must be references or a meta sublink');
      }
    }
    if (!metaFields) {
      throw structureError('statement is missing its meta sublink');
    }
    const { metadata, registered } = decodeMeta(metaFields);
    records.push({ id, references, metadata, registered });
  }
  return records;
}

function decodeMeta(fields) {
  let metadata = LinkMetadata.new();
  let registered = false;
  let bits = 0;
  for (const field of fields) {
    if (field.id === null || field.id === undefined || field.values.length === 0) {
      throw structureError('meta field must be an identified link');
    }
    switch (field.id) {
      case 't':
        metadata = metadata.withLinkType(linkTypeFromToken(singleReference(field)));
        break;
      case 'n':
        metadata = metadata.withNamed(singleReference(field) === '1');
        break;
      case 'term':
        metadata = metadata.withTerm(percentDecode(singleReference(field)));
        break;
      case 'def':
        metadata = metadata.withDefinition(percentDecode(singleReference(field)));
        break;
      case 'lang':
        metadata = metadata.withLanguage(percentDecode(singleReference(field)));
        break;
      case 'span':
        metadata = metadata.withSpan(parseSpan(field.values));
        break;
      case 'flags': {
        const text = singleReference(field);
        bits = /^\d+$/.test(text) ? Number(text) : Number.NaN;
        if (!Number.isInteger(bits) || bits > 255) {
          throw structureError(`invalid flags value \`${text}\``);
        }
        break;
      }
      case 'reg':
        registered = true;
        break;
      default:
        throw structureError(`unknown meta field \`${field.id}\``);
    }
  }
  if (bits !== 0) {
    metadata = metadata.withFlags(new LinkFlags({
      isError: (bits & 0b0001) !== 0,
      hasError: (bits & 0b0111) !== 0,
      isMissing: (bits & 0b0100) !== 0,
      isExtra: (bits & 0b1000) !== 0,
    }));
  }
  return { metadata, registered };
}

function singleReference(field) {
  if (field.values.length !== 1 || field.values[0].values.length !== 0) {
    throw structureError('meta field must hold exactly one reference');
  }
  return field.values[0].id;
}

function parseSpan(values) {
  if (values.length !== 6) {
    throw structureError('span field requires six numbers');
  }
  const numbers = values.map((value) => {
    if (value.values.length !== 0) {
      throw structureError('span field values must be numbers');
    }
    return parseNumber(value.id, (text) => `invalid span number \`${text}\``);
  });
  return new SourceSpan(
    new ByteRange(numbers[0], numbers[1]),
    new Point(numbers[2], numbers[3]),
    new Point(numbers[4], numbers[5]),
  );
}
