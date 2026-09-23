import { ByteRange, LinkFlags, Point, SourceSpan } from './primitives.js';

const encoder = new TextEncoder();

const LANGUAGE_ALIASES = new Map([
  ['javascript', 'JavaScript'],
  ['js', 'JavaScript'],
  ['ecmascript', 'JavaScript'],
  ['rust', 'Rust'],
  ['rs', 'Rust'],
  ['lean', 'Lean'],
  ['lean4', 'Lean'],
  ['rocq', 'Rocq'],
  ['coq', 'Rocq'],
]);

const ROOT_TERMS = Object.freeze({
  JavaScript: 'program',
  Rust: 'source_file',
  Lean: 'file',
  Rocq: 'source_file',
});

const KEYWORDS = Object.freeze({
  JavaScript: new Set([
    'async', 'await', 'break', 'case', 'catch', 'class', 'const', 'continue', 'debugger',
    'default', 'delete', 'do', 'else', 'export', 'extends', 'finally', 'for', 'function',
    'if', 'import', 'in', 'instanceof', 'let', 'new', 'of', 'return', 'static', 'super',
    'switch', 'this', 'throw', 'try', 'typeof', 'var', 'void', 'while', 'with', 'yield',
  ]),
  Rust: new Set([
    'as', 'async', 'await', 'break', 'const', 'continue', 'crate', 'dyn', 'else', 'enum',
    'extern', 'false', 'fn', 'for', 'if', 'impl', 'in', 'let', 'loop', 'match', 'mod',
    'move', 'mut', 'pub', 'ref', 'return', 'self', 'Self', 'static', 'struct', 'super',
    'trait', 'true', 'type', 'union', 'unsafe', 'use', 'where', 'while',
  ]),
  Lean: new Set([
    'abbrev', 'axiom', 'class', 'def', 'deriving', 'do', 'else', 'end', 'example',
    'export', 'if', 'import', 'in', 'inductive', 'instance', 'let', 'macro', 'match',
    'namespace', 'notation', 'opaque', 'open', 'partial', 'private', 'protected',
    'structure', 'syntax', 'theorem', 'universe', 'variable', 'where',
  ]),
  Rocq: new Set([
    'Axiom', 'Check', 'Class', 'CoFixpoint', 'CoInductive', 'Compute', 'Definition',
    'End', 'Eval', 'Export', 'Fixpoint', 'From', 'Goal', 'Hint', 'Import', 'Include',
    'Inductive', 'Instance', 'Lemma', 'Ltac', 'Module', 'Notation', 'Parameter',
    'Print', 'Proof', 'Qed', 'Record', 'Require', 'Section', 'Theorem', 'Universe',
    'Variable', 'Variables',
  ]),
});

const BUILTIN_TYPES = Object.freeze({
  JavaScript: new Set(),
  Rust: new Set([
    'bool', 'char', 'f32', 'f64', 'i8', 'i16', 'i32', 'i64', 'i128', 'isize', 'str',
    'u8', 'u16', 'u32', 'u64', 'u128', 'usize',
  ]),
  Lean: new Set(['Bool', 'Char', 'Float', 'Int', 'Nat', 'Prop', 'Sort', 'String', 'Type', 'UInt64']),
  Rocq: new Set(['bool', 'nat', 'Prop', 'Set', 'SProp', 'Type', 'Z']),
});

const IDENTIFIER_START = /[$_\p{ID_Start}]/u;
const IDENTIFIER_CONTINUE = /[$_\u200c\u200d\p{ID_Continue}]/u;
const OPEN_DELIMITERS = new Map([
  ['(', ')'],
  ['[', ']'],
  ['{', '}'],
]);
const CLOSE_DELIMITERS = new Set(OPEN_DELIMITERS.values());
const GROUP_TERMS = Object.freeze({
  '(': 'parenthesized_expression',
  '[': 'bracketed_expression',
  '{': 'block',
});

/** Returns the canonical name for one of the four registered language frontends. */
export function canonicalProgrammingLanguage(language) {
  return LANGUAGE_ALIASES.get(String(language).toLowerCase());
}

/**
 * Produces a lossless, recovery-aware lexical CST shared by the JavaScript runtime's
 * JavaScript, Rust, Lean 4, and Rocq/Coq frontends.
 */
export function parseProgrammingLanguage(text, language) {
  const canonical = canonicalProgrammingLanguage(language);
  if (!canonical) {
    return undefined;
  }

  const boundaries = sourceBoundaries(text);
  const tokens = scanTokens(text, canonical, boundaries);
  const tree = buildDelimiterTree(tokens, canonical);
  return { canonical, rootTerm: ROOT_TERMS[canonical], tokens, tree };
}

function scanTokens(text, canonical, boundaries) {
  const tokens = [];
  let offset = 0;
  while (offset < text.length) {
    const start = offset;
    const character = codePointAt(text, offset);
    let kind;
    let flags = LinkFlags.clean();

    if (/\s/u.test(character)) {
      offset = consumeWhile(text, offset, (value) => /\s/u.test(value));
      kind = 'whitespace';
      flags = flags.withExtra();
    } else if (startsLineComment(text, offset, canonical)) {
      offset = consumeLineComment(text, offset);
      kind = 'comment';
      flags = flags.withExtra();
    } else if (blockCommentDelimiter(text, offset, canonical)) {
      const delimiter = blockCommentDelimiter(text, offset, canonical);
      const result = consumeBlockComment(text, offset, delimiter.open, delimiter.close, delimiter.nested);
      offset = result.end;
      kind = 'comment';
      flags = flags.withExtra().withError(!result.closed);
    } else if (canonical === 'Rust' && rustRawStringPrefix(text, offset)) {
      const result = consumeRustRawString(text, offset);
      offset = result.end;
      kind = 'raw_string_literal';
      flags = flags.withError(!result.closed);
    } else if (
      character === '"' ||
      character === '`' ||
      (character === "'" && (canonical === 'JavaScript' || isCharacterLiteral(text, offset, canonical)))
    ) {
      const result = consumeQuoted(text, offset, character);
      offset = result.end;
      kind = character === '`'
        ? 'template_string'
        : character === "'" && canonical !== 'JavaScript'
          ? 'char_literal'
          : 'string';
      flags = flags.withError(!result.closed);
    } else if (IDENTIFIER_START.test(character)) {
      offset = consumeWhile(
        text,
        offset,
        (value) =>
          IDENTIFIER_CONTINUE.test(value) ||
          (value === "'" && (canonical === 'Lean' || canonical === 'Rocq')),
      );
      const value = text.slice(start, offset);
      kind = KEYWORDS[canonical].has(value)
        ? 'keyword'
        : BUILTIN_TYPES[canonical].has(value)
          ? 'primitive_type'
          : 'identifier';
    } else if (/[0-9]/u.test(character)) {
      offset = consumeNumber(text, offset);
      kind = 'number';
    } else {
      offset += character.length;
      kind = delimiterKind(character) ?? 'operator';
      if (/\p{Cc}/u.test(character)) {
        kind = 'unknown';
        flags = flags.withError();
      }
    }

    tokens.push({
      text: text.slice(start, offset),
      kind,
      named: !['whitespace', 'operator', 'open_delimiter', 'close_delimiter'].includes(kind),
      span: spanFor(boundaries, start, offset),
      flags,
    });
  }
  return tokens;
}

function buildDelimiterTree(tokens, canonical) {
  const root = { term: ROOT_TERMS[canonical], children: [], tokenIndex: undefined };
  const stack = [{ node: root, opener: undefined, tokenIndex: undefined }];

  for (const [tokenIndex, token] of tokens.entries()) {
    if (token.kind === 'open_delimiter') {
      const node = { term: GROUP_TERMS[token.text], children: [], tokenIndex: undefined };
      node.children.push({ term: token.kind, children: [], tokenIndex });
      stack.at(-1).node.children.push(node);
      stack.push({ node, opener: token.text, tokenIndex });
      continue;
    }

    if (token.kind === 'close_delimiter') {
      const current = stack.at(-1);
      if (current.opener && OPEN_DELIMITERS.get(current.opener) === token.text) {
        current.node.children.push({ term: token.kind, children: [], tokenIndex });
        stack.pop();
      } else {
        token.flags = token.flags.withError();
        stack.at(-1).node.children.push({ term: token.kind, children: [], tokenIndex });
      }
      continue;
    }

    stack.at(-1).node.children.push({ term: token.kind, children: [], tokenIndex });
  }

  for (const unclosed of stack.slice(1)) {
    const token = tokens[unclosed.tokenIndex];
    // The closer is missing, not the opener. Mark the retained opener as
    // containing an error so reconstruction never drops original source.
    token.flags = token.flags.withError();
  }
  return root;
}

function sourceBoundaries(text) {
  const result = new Map([[0, { byte: 0, row: 0, column: 0 }]]);
  let byte = 0;
  let row = 0;
  let column = 0;
  for (let offset = 0; offset < text.length;) {
    const character = codePointAt(text, offset);
    const byteLength = encoder.encode(character).length;
    byte += byteLength;
    if (character === '\n') {
      row += 1;
      column = 0;
    } else {
      column += byteLength;
    }
    offset += character.length;
    result.set(offset, { byte, row, column });
  }
  return result;
}

function spanFor(boundaries, start, end) {
  const from = boundaries.get(start);
  const to = boundaries.get(end);
  return new SourceSpan(
    new ByteRange(from.byte, to.byte),
    new Point(from.row, from.column),
    new Point(to.row, to.column),
  );
}

function startsLineComment(text, offset, canonical) {
  if (canonical === 'Lean') {
    return text.startsWith('--', offset);
  }
  if (canonical === 'Rocq') {
    return false;
  }
  return text.startsWith('//', offset);
}

function blockCommentDelimiter(text, offset, canonical) {
  if (canonical === 'Lean' && text.startsWith('/-', offset)) {
    return { open: '/-', close: '-/', nested: true };
  }
  if (canonical === 'Rocq' && text.startsWith('(*', offset)) {
    return { open: '(*', close: '*)', nested: true };
  }
  if ((canonical === 'JavaScript' || canonical === 'Rust') && text.startsWith('/*', offset)) {
    return { open: '/*', close: '*/', nested: canonical === 'Rust' };
  }
  return undefined;
}

function consumeLineComment(text, offset) {
  const newline = text.indexOf('\n', offset + 2);
  return newline === -1 ? text.length : newline;
}

function consumeBlockComment(text, offset, open, close, nested) {
  let cursor = offset + open.length;
  let depth = 1;
  while (cursor < text.length) {
    if (nested && text.startsWith(open, cursor)) {
      depth += 1;
      cursor += open.length;
    } else if (text.startsWith(close, cursor)) {
      depth -= 1;
      cursor += close.length;
      if (depth === 0) {
        return { end: cursor, closed: true };
      }
    } else {
      cursor += codePointAt(text, cursor).length;
    }
  }
  return { end: text.length, closed: false };
}

function consumeQuoted(text, offset, quote) {
  let cursor = offset + quote.length;
  let escaped = false;
  while (cursor < text.length) {
    const character = codePointAt(text, cursor);
    cursor += character.length;
    if (escaped) {
      escaped = false;
    } else if (character === '\\') {
      escaped = true;
    } else if (character === quote) {
      return { end: cursor, closed: true };
    } else if (character === '\n' && quote !== '`') {
      return { end: cursor, closed: false };
    }
  }
  return { end: text.length, closed: false };
}

function rustRawStringPrefix(text, offset) {
  return /^r#*"/u.test(text.slice(offset));
}

function consumeRustRawString(text, offset) {
  const prefix = /^r(#{0,255})"/u.exec(text.slice(offset));
  const hashes = prefix[1];
  const contentStart = offset + prefix[0].length;
  const close = `"${hashes}`;
  const closeOffset = text.indexOf(close, contentStart);
  return closeOffset === -1
    ? { end: text.length, closed: false }
    : { end: closeOffset + close.length, closed: true };
}

function isCharacterLiteral(text, offset, canonical) {
  if (text[offset] !== "'" || canonical === 'JavaScript' || canonical === 'Rocq') {
    return false;
  }
  if (canonical === 'Lean') {
    return /^'(?:\\.|[^'\\\n])'/u.test(text.slice(offset));
  }
  return /^'(?:\\.|[^'\\\n])'/u.test(text.slice(offset));
}

function consumeNumber(text, offset) {
  const match = /^(?:0[xX][0-9A-Fa-f_]+|0[bB][01_]+|0[oO][0-7_]+|[0-9][0-9_]*(?:\.[0-9_]*)?(?:[eE][+-]?[0-9_]+)?)(?:[A-Za-z][A-Za-z0-9_]*)?/u.exec(
    text.slice(offset),
  );
  return offset + match[0].length;
}

function consumeWhile(text, offset, predicate) {
  let cursor = offset;
  while (cursor < text.length) {
    const character = codePointAt(text, cursor);
    if (!predicate(character)) {
      break;
    }
    cursor += character.length;
  }
  return cursor;
}

function delimiterKind(character) {
  if (OPEN_DELIMITERS.has(character)) {
    return 'open_delimiter';
  }
  if (CLOSE_DELIMITERS.has(character)) {
    return 'close_delimiter';
  }
  return undefined;
}

function codePointAt(text, offset) {
  return String.fromCodePoint(text.codePointAt(offset));
}
