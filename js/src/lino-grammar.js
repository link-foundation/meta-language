import { anonymous, errorNode, extra, fillExtras, node, propagateErrors, spanning, withField } from './builtin-grammar.js';

/**
 * The built-in LiNo grammar CST, shared with `rust/src/lino_grammar.rs`.
 *
 * The recursive descent below mirrors, rule for rule and with the same
 * ordered choices, backtracking and indentation side effects, the official
 * links-notation 0.13 PEG grammar (`links-notation/src/grammar.pegjs`), so
 * every document the official parser accepts yields a clean CST whose `link`
 * nodes carry the official `id`, `values` and `children` as `id`, `value` and
 * `child` fields.
 *
 * - `lino_document` is the root; every link form is a named `link` node.
 * - References are named `reference` or `quoted_reference` leaves; `(`, `)`
 *   and `:` are anonymous leaves.
 * - LiNo whitespace (space, tab, CR, LF) is an anonymous `whitespace` extra
 *   owned by the smallest node that spans it, as tree-sitter places extras.
 * - Where the official grammar rejects the source, an `ERROR` node spans the
 *   rest of that line (with its `(`, `)`, `:` and `reference` tokens) and
 *   parsing restarts on the next line as a new document.
 *
 * Offsets are string indices; every delimiter is ASCII, so the Rust port
 * computes the same tree over UTF-8 byte offsets.
 */
export function parseLinoCst(text) {
  const children = [];
  let position = 0;
  while (position < text.length) {
    const parser = new LinoGrammarParser(text, position);
    const document = parser.document();
    children.push(...document.links);
    if (document.complete) break;
    const errorEnd = lineEnd(text, document.errorStart);
    children.push(lineErrorNode(text, document.errorStart, errorEnd));
    position = errorEnd;
  }
  const root = node('lino_document', 0, text.length, children);
  propagateErrors(root);
  return fillExtras(root, (start, end) => {
    if (![...text.slice(start, end)].every(isLinoWhitespace)) {
      throw new Error(`LiNo grammar left ${JSON.stringify(text.slice(start, end))} outside its CST`);
    }
    return [extra('whitespace', false, start, end)];
  });
}

class LinoGrammarParser {
  constructor(text, position) {
    this.text = text;
    this.position = position;
    this.indentationStack = [0];
    this.baseIndentation = null;
  }

  // document = skipEmptyLines links _ eof / _ eof
  // Where neither alternative reaches the end, the links parsed so far are
  // kept and the error starts after the whitespace that follows them.
  document() {
    this.skipEmptyLines();
    const links = this.links() ?? [];
    this.whitespace();
    return this.atEnd()
      ? { complete: true, links }
      : { complete: false, links, errorStart: this.position };
  }

  // skipEmptyLines = ([ \t]* [\r\n])*
  skipEmptyLines() {
    for (;;) {
      const start = this.position;
      this.inlineWhitespace();
      if (!this.newlineCharacter()) {
        this.position = start;
        return;
      }
    }
  }

  // links = firstLine line*   (pops the indentation pushed for it)
  links() {
    const first = this.firstLine();
    if (!first) return null;
    const links = [first];
    for (let line = this.line(); line; line = this.line()) links.push(line);
    if (this.indentationStack.length > 1) this.indentationStack.pop();
    return links;
  }

  // firstLine = SET_BASE_INDENTATION element
  firstLine() {
    const start = this.position;
    const spaces = this.spaces();
    if (this.baseIndentation === null) this.baseIndentation = spaces;
    const element = this.element();
    if (!element) this.position = start;
    return element;
  }

  // line = CHECK_INDENTATION element
  line() {
    const start = this.position;
    const spaces = this.spaces();
    if (this.normalizeIndentation(spaces) < this.currentIndentation()) {
      this.position = start;
      return null;
    }
    const element = this.element();
    if (!element) this.position = start;
    return element;
  }

  // element = anyLink PUSH_INDENTATION links / anyLink
  element() {
    const start = this.position;
    const link = this.anyLink();
    if (link) {
      const spaces = this.spaces();
      if (this.normalizeIndentation(spaces) > this.currentIndentation()) {
        this.indentationStack.push(this.normalizeIndentation(spaces));
        const links = this.links();
        if (links) {
          link.children.push(...links.map((child) => withField(child, 'child')));
          link.end = links.at(-1).end;
          return link;
        }
      }
    }
    this.position = start;
    return this.anyLink();
  }

  // anyLink = multiLineAnyLink eol / indentedIdLink / singleLineAnyLink
  anyLink() {
    const start = this.position;
    const multiLine = this.multiLineAnyLink();
    if (multiLine && this.eol()) return multiLine;
    this.position = start;
    return this.indentedIdLink() ?? this.singleLineAnyLink();
  }

  // multiLineAnyLink = multiLineValueLink / multiLineLink
  multiLineAnyLink() {
    return this.multiLineValueLink() ?? this.multiLineLink();
  }

  // multiLineValueLink = "(" multiLineValues _ ")"
  multiLineValueLink() {
    const start = this.position;
    const open = this.literal('(');
    if (!open) return null;
    const values = this.multiLineValues();
    this.whitespace();
    const close = this.literal(')');
    if (!close) return this.fail(start);
    return spanning('link', [open, ...values, close]);
  }

  // multiLineLink = "(" _ reference _ ":" multiLineValues _ ")"
  multiLineLink() {
    const start = this.position;
    const open = this.literal('(');
    if (!open) return null;
    this.whitespace();
    const id = this.reference();
    if (!id) return this.fail(start);
    this.whitespace();
    const colon = this.literal(':');
    if (!colon) return this.fail(start);
    const values = this.multiLineValues();
    this.whitespace();
    const close = this.literal(')');
    if (!close) return this.fail(start);
    return spanning('link', [open, withField(id, 'id'), colon, ...values, close]);
  }

  // multiLineValues = _ (referenceOrLink _)*
  multiLineValues() {
    this.whitespace();
    const values = [];
    for (let value = this.referenceOrLink(); value; value = this.referenceOrLink()) {
      values.push(withField(value, 'value'));
      this.whitespace();
    }
    return values;
  }

  // singleLineValues = (__ referenceOrLink)+
  singleLineValues() {
    const values = [];
    for (;;) {
      const start = this.position;
      this.inlineWhitespace();
      const value = this.referenceOrLink();
      if (!value) {
        this.position = start;
        break;
      }
      values.push(withField(value, 'value'));
    }
    return values.length > 0 ? values : null;
  }

  // referenceOrLink = multiLineAnyLink / reference
  referenceOrLink() {
    return this.multiLineAnyLink() ?? this.reference();
  }

  // singleLineAnyLink = singleLineLink eol / singleLineValueLink eol
  singleLineAnyLink() {
    const start = this.position;
    const named = this.singleLineLink();
    if (named && this.eol()) return named;
    this.position = start;
    const values = this.singleLineValues();
    if (values && this.eol()) return spanning('link', values);
    return this.fail(start);
  }

  // singleLineLink = __ reference __ ":" singleLineValues
  singleLineLink() {
    const start = this.position;
    this.inlineWhitespace();
    const id = this.reference();
    if (!id) return this.fail(start);
    this.inlineWhitespace();
    const colon = this.literal(':');
    if (!colon) return this.fail(start);
    const values = this.singleLineValues();
    if (!values) return this.fail(start);
    return spanning('link', [withField(id, 'id'), colon, ...values]);
  }

  // indentedIdLink = reference __ ":" eol
  indentedIdLink() {
    const start = this.position;
    const id = this.reference();
    if (!id) return null;
    this.inlineWhitespace();
    const colon = this.literal(':');
    if (!colon || !this.eol()) return this.fail(start);
    return spanning('link', [withField(id, 'id'), colon]);
  }

  // reference = quotedReference / simpleReference
  reference() {
    return this.quotedReference() ?? this.simpleReference();
  }

  // N opening quotes (", ' or `), content in which 2N quotes escape N, and
  // exactly N closing quotes not followed by another quote.
  quotedReference() {
    const start = this.position;
    const quote = this.text[start];
    if (quote !== '"' && quote !== "'" && quote !== '`') return null;
    let position = start;
    while (this.text[position] === quote) position += 1;
    const count = position - start;
    const close = quote.repeat(count);
    const escape = quote.repeat(count * 2);
    while (position < this.text.length) {
      if (this.text.startsWith(escape, position)) {
        position += escape.length;
      } else if (this.text.startsWith(close, position) && this.text[position + count] !== quote) {
        this.position = position + count;
        return node('quoted_reference', start, this.position);
      } else {
        position += 1;
      }
    }
    return null;
  }

  // simpleReference = [^ \t\n\r(:)]+
  simpleReference() {
    const start = this.position;
    while (this.position < this.text.length && isReferenceCharacter(this.text[this.position])) {
      this.position += 1;
    }
    return this.position > start ? node('reference', start, this.position) : null;
  }

  // eol = __ ([\r\n]+ / eof)
  eol() {
    const start = this.position;
    this.inlineWhitespace();
    if (this.atEnd()) return true;
    if (!this.newlineCharacter()) {
      this.position = start;
      return false;
    }
    while (this.newlineCharacter());
    return true;
  }

  literal(character) {
    if (this.text[this.position] !== character) return null;
    this.position += 1;
    return anonymous(character, this.position - 1, this.position);
  }

  newlineCharacter() {
    const character = this.text[this.position];
    if (character !== '\n' && character !== '\r') return false;
    this.position += 1;
    return true;
  }

  // " "*
  spaces() {
    const start = this.position;
    while (this.text[this.position] === ' ') this.position += 1;
    return this.position - start;
  }

  // __ = [ \t]*
  inlineWhitespace() {
    while (this.text[this.position] === ' ' || this.text[this.position] === '\t') this.position += 1;
  }

  // _ = [ \t\n\r]*
  whitespace() {
    while (isLinoWhitespace(this.text[this.position])) this.position += 1;
  }

  atEnd() {
    return this.position >= this.text.length;
  }

  fail(start) {
    this.position = start;
    return null;
  }

  normalizeIndentation(spaces) {
    return this.baseIndentation === null ? spaces : Math.max(0, spaces - this.baseIndentation);
  }

  currentIndentation() {
    return this.indentationStack.at(-1);
  }
}

function lineErrorNode(text, start, end) {
  const children = [];
  let position = start;
  while (position < end) {
    const character = text[position];
    if (isLinoWhitespace(character)) {
      position += 1;
    } else if (character === '(' || character === ')' || character === ':') {
      children.push(anonymous(character, position, position + 1));
      position += 1;
    } else {
      const referenceStart = position;
      while (position < end && isReferenceCharacter(text[position])) position += 1;
      children.push(node('reference', referenceStart, position));
    }
  }
  return errorNode(start, end, children);
}

// The rest of the line after `start`, without its line terminator.
function lineEnd(text, start) {
  let end = start;
  while (end < text.length && text[end] !== '\n' && text[end] !== '\r') end += 1;
  return end;
}

function isLinoWhitespace(character) {
  return character === ' ' || character === '\t' || character === '\n' || character === '\r';
}

function isReferenceCharacter(character) {
  return !isLinoWhitespace(character) && character !== '(' && character !== ':' && character !== ')';
}
