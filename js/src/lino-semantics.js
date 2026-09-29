import { ByteRange, LinkMetadata, LinkType, Point, SourceSpan } from './primitives.js';

const encoder = new TextEncoder();
const decoder = new TextDecoder();

const OPEN = 0x28;
const CLOSE = 0x29;
const COLON = 0x3a;
const NEWLINE = 0x0a;
const RETURN = 0x0d;

// Rust's `u8::is_ascii_whitespace`: space, tab, line feed, form feed and
// carriage return.
const isAsciiWhitespace = (byte) =>
  byte === 0x20 || byte === 0x09 || byte === NEWLINE || byte === 0x0c || byte === RETURN;
const startsWithWhitespace = (text) => /^\p{White_Space}/u.test(text);
const trim = (text) => text.replace(/^\p{White_Space}+|\p{White_Space}+$/gu, '');

/**
 * Inserts the links-notation semantics of `text` into `network`: every
 * parenthesized form, line of several references, or indented definition
 * becomes a Relation link spanning its source, named for `(name: ...)` and
 * `name:` definitions; every atom becomes a Concept point unless an earlier
 * link already carries its name. Mirrors `Parser` in rust/src/lino_parser.rs.
 */
export function insertLinoSemantics(network, text, language) {
  new LinoSemanticsParser(network, text, language).parseDocument();
}

class LinoSemanticsParser {
  constructor(network, text, language) {
    this.network = network;
    this.bytes = encoder.encode(text);
    this.language = language;
    this.cursor = 0;
    this.lineStarts = [0];
    for (const [index, byte] of this.bytes.entries()) {
      if (byte === NEWLINE) this.lineStarts.push(index + 1);
    }
  }

  parseDocument() {
    while (this.cursor < this.bytes.length) {
      this.skipWhitespace(true);
      if (this.cursor >= this.bytes.length) break;
      if (this.peek() === OPEN) {
        this.parseExpression();
      } else {
        this.parseLineForm();
      }
    }
  }

  parseLineForm() {
    const lineStart = this.cursor;
    const lineEnd = this.lineEnd(lineStart);
    const line = this.text(lineStart, lineEnd);
    const trimmed = trim(line);
    if (trimmed === '') {
      this.cursor = this.nextLineStart(lineEnd);
      return;
    }
    if (!startsWithWhitespace(line) && trimmed.endsWith(':')) {
      this.parseIndentedDefinition(lineStart, lineEnd, trimmed);
      return;
    }
    const references = this.parseLineReferences(lineStart, lineEnd);
    if (references.length > 1) {
      this.insertRelation(references, undefined, lineStart, lineEnd);
    }
    this.cursor = this.nextLineStart(lineEnd);
  }

  parseIndentedDefinition(lineStart, lineEnd, trimmed) {
    const name = trim(trimmed.replace(/:+$/u, ''));
    let childStart = this.nextLineStart(lineEnd);
    let definitionEnd = lineEnd;
    const references = [];
    while (childStart < this.bytes.length) {
      const childEnd = this.lineEnd(childStart);
      if (!startsWithWhitespace(this.text(childStart, childEnd))) break;
      references.push(...this.parseLineReferences(childStart, childEnd));
      definitionEnd = childEnd;
      childStart = this.nextLineStart(childEnd);
    }
    this.insertRelation(references, name, lineStart, definitionEnd);
    this.cursor = childStart;
  }

  parseLineReferences(start, end) {
    this.cursor = start;
    const references = [];
    while (this.cursor < end) {
      this.skipWhitespace(false);
      if (this.cursor >= end) break;
      const reference = this.peek() === OPEN
        ? this.parseParenthesizedRelation()
        : this.parseAtomReference();
      if (reference === undefined) break;
      references.push(reference);
    }
    return references;
  }

  parseExpression() {
    this.skipWhitespace(false);
    const next = this.peek();
    if (next === undefined || next === CLOSE) return undefined;
    return next === OPEN ? this.parseParenthesizedRelation() : this.parseAtomReference();
  }

  // Nested parentheses are read with an explicit stack of open relations, so
  // a document of any nesting depth is read without deep recursion.
  parseParenthesizedRelation() {
    const open = [this.openRelation()];
    for (;;) {
      const relation = open.at(-1);
      this.skipWhitespace(false);
      const next = this.peek();
      if (next === OPEN) {
        open.push(this.openRelation());
        continue;
      }
      let reference;
      if (next === CLOSE) {
        this.cursor += 1;
      } else if (next !== undefined) {
        reference = this.parseAtomReference();
      }
      if (reference !== undefined) {
        relation.references.push(reference);
        continue;
      }
      open.pop();
      const id = this.closeRelation(relation);
      if (open.length === 0) return id;
      open.at(-1).references.push(id);
    }
  }

  // Reads `(` and a leading `name:` of the relation it opens.
  openRelation() {
    const start = this.cursor;
    this.cursor += 1;
    this.skipWhitespace(false);
    const references = [];
    let named;
    if (this.peek() !== CLOSE) {
      const atom = this.parseAtomText();
      if (atom) {
        this.skipWhitespace(false);
        if (this.peek() === COLON) {
          this.cursor += 1;
          named = this.insertRelation([], atom.text, start, atom.end);
        } else {
          references.push(this.referenceForAtom(atom.text));
        }
      }
    }
    return { start, references, named };
  }

  closeRelation({ start, references, named }) {
    if (named === undefined) {
      return this.insertRelation(references, undefined, start, this.cursor);
    }
    const link = this.network.link(named);
    link.setReferences(references);
    link.setMetadata(link.metadata().withSpan(this.span(start, this.cursor)));
    return named;
  }

  parseAtomReference() {
    const atom = this.parseAtomText();
    return atom ? this.referenceForAtom(atom.text) : undefined;
  }

  parseAtomText() {
    this.skipWhitespace(false);
    const start = this.cursor;
    while (this.cursor < this.bytes.length) {
      const byte = this.bytes[this.cursor];
      if (isAsciiWhitespace(byte) || byte === OPEN || byte === CLOSE || byte === COLON) break;
      this.cursor += 1;
    }
    return start === this.cursor
      ? undefined
      : { text: this.text(start, this.cursor), end: this.cursor };
  }

  referenceForAtom(atom) {
    return this.network.findTerm(atom) ??
      this.network.insertTypedPoint(LinkType.Concept, atom);
  }

  insertRelation(references, name, start, end) {
    let metadata = LinkMetadata.new()
      .withLinkType(LinkType.Relation)
      .withNamed(name !== undefined)
      .withLanguage(this.language)
      .withSpan(this.span(start, end));
    if (name !== undefined) metadata = metadata.withTerm(name);
    const id = this.network.insertLink(references, metadata);
    // A named relation is referenced by its name later in the document, and
    // by itself.
    if (name !== undefined) this.network._terms.set(name, id);
    return id;
  }

  skipWhitespace(acrossLines) {
    while (this.cursor < this.bytes.length) {
      const byte = this.bytes[this.cursor];
      if (!isAsciiWhitespace(byte) || (!acrossLines && (byte === NEWLINE || byte === RETURN))) break;
      this.cursor += 1;
    }
  }

  peek() {
    return this.bytes[this.cursor];
  }

  text(start, end) {
    return decoder.decode(this.bytes.subarray(start, end));
  }

  lineEnd(start) {
    const index = this.bytes.indexOf(NEWLINE, start);
    return index === -1 ? this.bytes.length : index;
  }

  nextLineStart(lineEnd) {
    return this.bytes[lineEnd] === NEWLINE ? lineEnd + 1 : lineEnd;
  }

  span(start, end) {
    return new SourceSpan(new ByteRange(start, end), this.point(start), this.point(end));
  }

  point(byte) {
    // The last line starting at or before `byte`, found by binary search.
    let row = 0;
    let high = this.lineStarts.length - 1;
    while (row < high) {
      const middle = (row + high + 1) >> 1;
      if (this.lineStarts[middle] <= byte) row = middle;
      else high = middle - 1;
    }
    return new Point(row, byte - this.lineStarts[row]);
  }
}
