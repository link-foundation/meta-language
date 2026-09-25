import { readFile } from 'node:fs/promises';
import { gunzipSync } from 'node:zlib';
import { Language as WebTreeSitterLanguage, Parser as WebTreeSitterParser } from 'web-tree-sitter';

import { canonicalLanguageName, languageEntry } from './language-catalog.js';
import { ByteRange, LinkFlags, Point, SourceSpan } from './primitives.js';

const encoder = new TextEncoder();
const GRAMMAR_DIRECTORY = new URL('./vendor/grammars/', import.meta.url);

/**
 * The grammar lock records, for every vendored grammar, the exact Rust crate
 * (or pinned upstream revision) its WebAssembly build was compiled from, so the
 * JavaScript and Rust runtimes parse with byte-identical generated parsers.
 */
export const GRAMMAR_LOCK = Object.freeze(
  JSON.parse(await readFile(new URL('grammar-lock.json', GRAMMAR_DIRECTORY), 'utf8')),
);

await WebTreeSitterParser.init();
const GRAMMARS = new Map(
  await Promise.all(
    Object.keys(GRAMMAR_LOCK.grammars).map(async (id) => [
      id,
      await WebTreeSitterLanguage.load(
        gunzipSync(await readFile(new URL(`${id}.wasm.gz`, GRAMMAR_DIRECTORY))),
      ),
    ]),
  ),
);

const LEAN_PUBLIC_ROOT = 'file';
const ROCQ_BUILTIN_TYPES = new Set(['bool', 'nat', 'Prop', 'Set', 'SProp', 'Type', 'Z']);

/** Returns the canonical name for a grammar-backed JavaScript frontend. */
export function canonicalProgrammingLanguage(language) {
  return canonicalLanguageName(language);
}

/**
 * Produces a lossless, recovery-aware grammar CST for a registered language.
 * Registered languages never silently fall back to lexical or character data.
 */
export function parseProgrammingLanguage(text, language) {
  const canonical = canonicalProgrammingLanguage(language);
  if (!canonical) {
    return undefined;
  }

  const parsed = parseGrammarCst(text, canonical);
  if (text.includes('\0')) {
    // NUL is prohibited input: the root reports that it contains an error
    // without relabeling the grammar's own nodes as ERROR.
    parsed.tree.flags = new LinkFlags({ ...parsed.tree.flags, hasError: true });
    const retained = parsed.tokens.map(({ text: token }) => token).join('');
    if (text.startsWith(retained) && retained.length < text.length) {
      const boundaries = sourceBoundaries(text);
      parsed.tokens.push({
        text: text.slice(retained.length),
        kind: 'invalid_source_character',
        named: false,
        span: spanFor(boundaries, retained.length, text.length),
        flags: LinkFlags.clean().withError(),
      });
    }
  }
  return parsed;
}

/** Parses an embedded region and clips any synthetic CSS terminator to its host span. */
export function parseEmbeddedProgrammingLanguage(text, language) {
  const canonical = canonicalProgrammingLanguage(language);
  const needsTerminator = canonical === 'CSS' && cssDeclarationNeedsTerminator(text);
  const parsed = parseProgrammingLanguage(needsTerminator ? `${text};` : text, language);
  if (!parsed || !needsTerminator) return parsed;
  return clipParsedSource(parsed, text);
}

function cssDeclarationNeedsTerminator(text) {
  const trimmed = text.trimEnd();
  return trimmed.length > 0 &&
    !trimmed.endsWith(';') &&
    !trimmed.endsWith('}') &&
    !trimmed.includes('{');
}

function clipParsedSource(parsed, text) {
  const byteEnd = encoder.encode(text).length;
  const endCoordinate = sourceBoundaries(text).get(text.length);
  const tokenIndexes = new Map();
  const tokens = [];
  for (const [index, token] of parsed.tokens.entries()) {
    if (token.span.byteRange.end > byteEnd) continue;
    tokenIndexes.set(index, tokens.length);
    tokens.push(token);
  }
  const clip = (node) => clipTreeNode(node, tokenIndexes, byteEnd, endCoordinate);
  const tree = clip(parsed.tree);
  const leading = (parsed.leading ?? []).map(clip).filter(Boolean);
  const trailing = (parsed.trailing ?? []).map(clip).filter(Boolean);
  return { ...parsed, rootTerm: tree.term, tokens, tree, leading, trailing };
}

function clipTreeNode(node, tokenIndexes, byteEnd, endCoordinate) {
  if (node.span.byteRange.start >= byteEnd && node.span.byteRange.end > byteEnd) return null;
  if (node.tokenIndex !== undefined) {
    const tokenIndex = tokenIndexes.get(node.tokenIndex);
    return tokenIndex === undefined ? null : { ...node, tokenIndex };
  }
  const children = node.children
    .map((child) => clipTreeNode(child, tokenIndexes, byteEnd, endCoordinate))
    .filter(Boolean);
  const span = node.span.byteRange.end <= byteEnd
    ? node.span
    : new SourceSpan(
        new ByteRange(node.span.byteRange.start, byteEnd),
        node.span.start,
        new Point(endCoordinate.row, endCoordinate.column),
      );
  return { ...node, children, span };
}

function parseGrammarCst(text, canonical) {
  const boundaries = sourceBoundaries(text);
  if (canonical === 'LiNo') {
    return parseTokenGrammar(text, canonical, boundaries, 'lino_document', linoLineTerm);
  }
  if (canonical === 'txt') {
    return parseTokenGrammar(
      text,
      canonical,
      boundaries,
      'text_document',
      () => 'line',
      validateBalancedParentheses,
    );
  }
  if (canonical === 'PDF') {
    return parseTokenGrammar(text, canonical, boundaries, 'pdf_file', pdfLineTerm, validatePdf);
  }
  if (languageEntry(canonical).family === 'natural') {
    return parseNaturalLanguageGrammar(text, canonical, boundaries);
  }
  const grammar = GRAMMARS.get(languageEntry(canonical).grammars[0]?.id);
  if (!grammar) {
    throw new Error(`no tree-sitter grammar is registered for ${canonical}`);
  }
  const parser = new WebTreeSitterParser();
  parser.setLanguage(grammar);
  const parsed = parser.parse(text);
  parser.delete();
  if (!parsed) {
    throw new Error(`tree-sitter parser returned no ${canonical} syntax tree`);
  }
  const root = parsed.rootNode;
  const adapter = TREE_SITTER_ADAPTER;

  // Tree-sitter starts the root after its leading padding, so the text
  // outside the root is retained as gap nodes beside it.
  const tokens = [];
  const leading = [];
  pushGapNodes(leading, 0, adapter.startOffset(root), text, boundaries, tokens);
  const tree = convertGrammarNode(root, adapter, canonical, text, boundaries, tokens);
  const trailing = [];
  pushGapNodes(trailing, adapter.endOffset(root), text.length, text, boundaries, tokens);
  parsed.delete();

  // Preserve the original public Lean root while retaining the grammar's
  // `module` root immediately below it. Consumers can query either layer.
  if (canonical === 'Lean') {
    const file = {
      term: LEAN_PUBLIC_ROOT,
      children: [...leading, tree, ...trailing],
      named: true,
      span: spanFor(boundaries, 0, text.length),
      flags: tree.flags,
    };
    return { canonical, rootTerm: file.term, tokens, tree: file, leading: [], trailing: [] };
  }
  return { canonical, rootTerm: tree.term, tokens, tree, leading, trailing };
}

function parseTokenGrammar(text, canonical, boundaries, rootTerm, lineTerm, validate = undefined) {
  const tokens = [];
  let hasLineError = false;
  const children = lineRanges(text).map(([start, end]) => {
    const term = lineTerm(text.slice(start, end));
    const isError = term.endsWith('_error');
    hasLineError ||= isError;
    return {
      term,
      children: lexicalNodes(text, start, end, boundaries, tokens),
      named: true,
      span: spanFor(boundaries, start, end),
      flags: isError ? LinkFlags.clean().withError() : LinkFlags.clean(),
    };
  });
  // A line error is located below the root; a document-level validation
  // failure without a located line error makes the root itself the error.
  let flags = LinkFlags.clean();
  if (hasLineError) {
    flags = new LinkFlags({ hasError: true });
  } else if (!(validate?.(text) ?? true)) {
    flags = LinkFlags.clean().withError();
  }
  const tree = {
    term: rootTerm,
    children,
    named: true,
    span: spanFor(boundaries, 0, text.length),
    flags,
  };
  return { canonical, rootTerm, tokens, tree };
}

function parseNaturalLanguageGrammar(text, canonical, boundaries) {
  const tokens = [];
  const children = [];
  // The built-in sentence grammar shared with the Rust runtime: a sentence ends
  // after a run of terminal punctuation, any closing punctuation or quotes, and
  // the whitespace that follows them.
  const sentence = /[^]*?[.!?\u0964\u3002\u061f\u06d4\uff01\uff1f]+[\p{Pe}\p{Pf}"']*\p{White_Space}*/uy;
  let sentenceStart = 0;
  for (let match = sentence.exec(text); match; match = sentence.exec(text)) {
    children.push(naturalSentence(text, sentenceStart, sentence.lastIndex, boundaries, tokens));
    sentenceStart = sentence.lastIndex;
  }
  if (sentenceStart < text.length) {
    children.push(naturalSentence(text, sentenceStart, text.length, boundaries, tokens));
  }
  const tree = {
    term: 'natural_language_document',
    children,
    named: true,
    span: spanFor(boundaries, 0, text.length),
    flags: LinkFlags.clean(),
  };
  return { canonical, rootTerm: tree.term, tokens, tree };
}

function naturalSentence(text, start, end, boundaries, tokens) {
  return {
    term: 'sentence',
    children: lexicalNodes(text, start, end, boundaries, tokens),
    named: true,
    span: spanFor(boundaries, start, end),
    flags: LinkFlags.clean(),
  };
}

function lexicalNodes(text, start, end, boundaries, tokens) {
  const nodes = [];
  // The built-in lexical grammar shared with the Rust runtime: maximal runs of
  // Unicode whitespace, of word characters, and of any other characters.
  const pattern = /\p{White_Space}+|[\p{L}\p{N}\p{M}_'-]+|[^\p{White_Space}\p{L}\p{N}\p{M}_'-]+/gu;
  pattern.lastIndex = start;
  while (pattern.lastIndex < end) {
    const match = pattern.exec(text);
    if (!match || match.index >= end) {
      break;
    }
    const tokenEnd = Math.min(pattern.lastIndex, end);
    const value = text.slice(match.index, tokenEnd);
    const whitespace = /^\p{White_Space}+$/u.test(value);
    const word = /^[\p{L}\p{N}\p{M}_'-]+$/u.test(value);
    nodes.push(grammarTokenNode(
      whitespace ? 'whitespace' : word ? 'word' : 'punctuation',
      match.index,
      tokenEnd,
      !whitespace,
      whitespace ? LinkFlags.clean().withExtra() : LinkFlags.clean(),
      text,
      boundaries,
      tokens,
    ));
    if (pattern.lastIndex >= end) {
      break;
    }
  }
  return nodes;
}

function lineRanges(text) {
  const ranges = [];
  let start = 0;
  for (let offset = 0; offset < text.length;) {
    const character = codePointAt(text, offset);
    offset += character.length;
    if (character === '\n') {
      ranges.push([start, offset]);
      start = offset;
    }
  }
  if (start < text.length || text.length === 0) {
    ranges.push([start, text.length]);
  }
  return ranges;
}

function linoLineTerm(line) {
  const trimmed = trimWhiteSpace(line);
  if (trimmed.startsWith('(') && trimmed.endsWith(')')) return 'link';
  if (trimmed.startsWith('(') || trimmed.endsWith(')')) return 'lino_error';
  if (trimmed.endsWith(':') && !trimmed.startsWith(':')) return 'definition';
  if (/^\p{White_Space}/u.test(line)) return 'definition_body';
  if (trimmed.split(/\p{White_Space}+/u).length > 1) return 'link';
  if (trimmed.length === 0) return 'blank_line';
  return 'atom';
}

function trimWhiteSpace(text) {
  return text.replace(/^\p{White_Space}+|\p{White_Space}+$/gu, '');
}

function pdfLineTerm(line) {
  const trimmed = trimWhiteSpace(line);
  if (trimmed.startsWith('%PDF-')) return 'header';
  if (trimmed === '%%EOF') return 'end_of_file';
  if (/^[0-9]+\p{White_Space}+[0-9]+\p{White_Space}+obj$/u.test(trimmed)) return 'object_header';
  if (trimmed === 'endobj') return 'object_end';
  if (trimmed === 'xref') return 'cross_reference_table';
  if (trimmed === 'trailer') return 'trailer';
  if (trimmed === 'stream' || trimmed === 'endstream') return 'stream_boundary';
  return 'pdf_line';
}

function validatePdf(text) {
  return /^%PDF-[0-9]+\.[0-9]+/u.test(text) && /%%EOF\p{White_Space}*$/u.test(text);
}

function validateBalancedParentheses(text) {
  let depth = 0;
  for (const character of text) {
    if (character === '(') depth += 1;
    if (character === ')' && depth-- === 0) return false;
  }
  return depth === 0;
}

function convertGrammarNode(node, adapter, canonical, text, boundaries, tokens, injected = []) {
  const start = adapter.startOffset(node);
  const end = adapter.endOffset(node);
  const children = [];
  let coveredUntil = start;
  const inlineTree = canonical === 'Markdown' && MARKDOWN_INLINE_CONTAINERS.has(adapter.term(node))
    ? parseMarkdownInline(node, text)
    : undefined;
  const ownChildren = inlineTree
    ? adapter.children(inlineTree.rootNode)
    : adapter.children(node);
  const extraChildren = inlineTree
    ? [...injected, ...markdownInlineExcludedChildren(node)]
    : injected;

  for (const { node: child, field, injected: nested } of distributeInjectedChildren(
    ownChildren,
    extraChildren,
    adapter,
  )) {
    const childStart = adapter.startOffset(child);
    pushGapNodes(children, coveredUntil, childStart, text, boundaries, tokens);
    const converted = convertGrammarNode(
      child,
      adapter,
      canonical,
      text,
      boundaries,
      tokens,
      nested,
    );
    converted.field = field;
    children.push(converted);
    coveredUntil = Math.max(coveredUntil, adapter.endOffset(child));
  }
  inlineTree?.delete();

  if (children.length === 0 && start < end) {
    const grammarTerm = adapter.term(node);
    const tokenText = text.slice(start, end);
    const semanticTerm = canonical === 'Rocq' && grammarTerm === 'ident'
      ? ROCQ_BUILTIN_TYPES.has(tokenText) ? 'primitive_type' : 'identifier'
      : grammarTerm;
    const tokenNode = grammarTokenNode(
      semanticTerm,
      start,
      end,
      adapter.isNamed(node),
      grammarFlags(node, adapter),
      text,
      boundaries,
      tokens,
    );
    // Rocq's grammar calls every identifier-like leaf `ident`. Retain that
    // concrete grammar node while exposing the cross-language semantic leaf
    // names used by existing identifier/type queries.
    return semanticTerm === grammarTerm
      ? tokenNode
      : {
          term: grammarTerm,
          children: [tokenNode],
          named: adapter.isNamed(node),
          span: tokenNode.span,
          flags: tokenNode.flags,
        };
  }

  pushGapNodes(children, coveredUntil, end, text, boundaries, tokens);

  // A Markdown inline tree is parsed separately from its block container, so
  // its errors reach the containing block nodes through their children.
  const flags = grammarFlags(node, adapter);
  return {
    term: adapter.term(node),
    children,
    named: adapter.isNamed(node),
    span: spanFor(boundaries, start, end),
    flags: !flags.hasError && children.some((child) => child.flags.hasError)
      ? new LinkFlags({ ...flags, hasError: true })
      : flags,
  };
}

// tree-sitter-markdown parses block structure and inline content with two
// grammars. Like upstream's `MarkdownParser` (bindings/rust/parser.rs in
// tree-sitter-md), every `inline` and `pipe_table_cell` block node is parsed
// again with the inline grammar over the node's range minus its named
// children after the first (block continuations such as a quote's `> `).
// Mirrors `parse_markdown_inline` in rust/src/tree_sitter_adapter.rs.
const MARKDOWN_INLINE_CONTAINERS = new Set(['inline', 'pipe_table_cell']);

function markdownInlineExcludedChildren(node) {
  return node.children.slice(1).filter((child) => child.isNamed);
}

function parseMarkdownInline(node, text) {
  const includedRanges = [];
  let start = { index: node.startIndex, position: node.startPosition };
  for (const child of markdownInlineExcludedChildren(node)) {
    includedRanges.push({
      startIndex: start.index,
      startPosition: start.position,
      endIndex: child.startIndex,
      endPosition: child.startPosition,
    });
    start = { index: child.endIndex, position: child.endPosition };
  }
  includedRanges.push({
    startIndex: start.index,
    startPosition: start.position,
    endIndex: node.endIndex,
    endPosition: node.endPosition,
  });
  const parser = new WebTreeSitterParser();
  parser.setLanguage(GRAMMARS.get('markdown_inline'));
  const tree = parser.parse(text, null, { includedRanges });
  parser.delete();
  if (!tree) throw new Error('tree-sitter parser returned no Markdown inline syntax tree');
  return tree;
}

// Places nodes from another tree (Markdown block continuations inside inline
// content) under the deepest child whose range contains them, and orders the
// rest among the children by start offset, block nodes first on ties.
function distributeInjectedChildren(children, injected, adapter) {
  const entries = children.map(({ node, field }) => ({ node, field, injected: [] }));
  const top = [];
  for (const node of injected) {
    const owner = entries.find((entry) => containsNode(entry.node, node, adapter));
    if (owner) owner.injected.push(node);
    else top.push({ node, field: null, injected: [] });
  }
  if (top.length === 0) return entries;
  return [...top, ...entries].sort(
    (left, right) => adapter.startOffset(left.node) - adapter.startOffset(right.node),
  );
}

function containsNode(outer, inner, adapter) {
  const outerStart = adapter.startOffset(outer);
  const outerEnd = adapter.endOffset(outer);
  const innerStart = adapter.startOffset(inner);
  const innerEnd = adapter.endOffset(inner);
  return outerStart <= innerStart && innerEnd <= outerEnd
    && (innerStart < innerEnd || (outerStart < innerStart && innerStart < outerEnd));
}

/** Term of source text consumed by hidden grammar rules, such as VB's `Module`. */
export const HIDDEN_TEXT_TERM = 'hidden_text';

// Text between visible tree-sitter children is either lexer extras or text
// matched by hidden grammar rules. Mirrors `insert_gap_token` in
// rust/src/tree_sitter_adapter.rs: leading and trailing whitespace is extra
// trivia, and the text between them is a non-extra hidden-text token.
function pushGapNodes(children, start, end, text, boundaries, tokens) {
  if (start >= end) return;
  const gap = text.slice(start, end);
  const leading = /^\p{White_Space}*/u.exec(gap)[0].length;
  const trailing = leading === gap.length ? 0 : /\p{White_Space}*$/u.exec(gap)[0].length;
  const pieces = [
    [start, start + leading, 'whitespace', LinkFlags.clean().withExtra()],
    [start + leading, end - trailing, HIDDEN_TEXT_TERM, LinkFlags.clean()],
    [end - trailing, end, 'whitespace', LinkFlags.clean().withExtra()],
  ];
  for (const [pieceStart, pieceEnd, term, flags] of pieces) {
    if (pieceStart < pieceEnd) {
      children.push(grammarTokenNode(term, pieceStart, pieceEnd, false, flags, text, boundaries, tokens));
    }
  }
}

function grammarTokenNode(term, start, end, named, flags, text, boundaries, tokens) {
  const tokenIndex = tokens.length;
  const token = {
    text: text.slice(start, end),
    kind: term,
    named,
    span: spanFor(boundaries, start, end),
    flags,
  };
  tokens.push(token);
  return { term, children: [], tokenIndex, named, span: token.span, flags };
}

function grammarFlags(node, adapter) {
  const isError = adapter.isError(node);
  const isMissing = adapter.isMissing(node);
  return new LinkFlags({
    isError,
    isMissing,
    isExtra: adapter.isExtra(node),
    hasError: isError || isMissing || adapter.hasError(node),
  });
}

function propertyOrCall(node, name) {
  const value = node[name];
  return typeof value === 'function' ? value.call(node) : value;
}

const TREE_SITTER_ADAPTER = Object.freeze({
  term: (node) => node.type,
  startOffset: (node) => node.startIndex,
  endOffset: (node) => node.endIndex,
  isNamed: (node) => propertyOrCall(node, 'isNamed'),
  isError: (node) => propertyOrCall(node, 'isError'),
  isMissing: (node) => propertyOrCall(node, 'isMissing'),
  isExtra: (node) => propertyOrCall(node, 'isExtra') ?? false,
  hasError: (node) => propertyOrCall(node, 'hasError'),
  children: (node) => node.children.map((child, index) => ({
    node: child,
    field: node.fieldNameForChild(index),
  })),
});

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

function codePointAt(text, offset) {
  return String.fromCodePoint(text.codePointAt(offset));
}
