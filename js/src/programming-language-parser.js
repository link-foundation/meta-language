import TreeSitterLanguagePack from '@kreuzberg/tree-sitter-language-pack';
import { readFile } from 'node:fs/promises';
import { Language as WebTreeSitterLanguage, Parser as WebTreeSitterParser } from 'web-tree-sitter';

import { ByteRange, LinkFlags, Point, SourceSpan } from './primitives.js';

const encoder = new TextEncoder();
await WebTreeSitterParser.init();
const ROCQ_GRAMMAR = await WebTreeSitterLanguage.load(
  await readFile(new URL('./vendor/tree-sitter-rocq.wasm', import.meta.url)),
);

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
  ['python', 'Python'],
  ['py', 'Python'],
  ['c', 'C'],
  ['c++', 'C++'],
  ['cpp', 'C++'],
  ['c#', 'C#'],
  ['csharp', 'C#'],
  ['java', 'Java'],
  ['typescript', 'TypeScript'],
  ['ts', 'TypeScript'],
  ['tsx', 'TSX'],
  ['visual basic', 'Visual Basic'],
  ['vb', 'Visual Basic'],
  ['vb.net', 'Visual Basic'],
  ['vbnet', 'Visual Basic'],
  ['delphi/object pascal', 'Delphi/Object Pascal'],
  ['delphi', 'Delphi/Object Pascal'],
  ['object pascal', 'Delphi/Object Pascal'],
  ['pascal', 'Delphi/Object Pascal'],
  ['go', 'Go'],
  ['golang', 'Go'],
  ['r', 'R'],
  ['ruby', 'Ruby'],
  ['rb', 'Ruby'],
  ['php', 'PHP'],
  ['swift', 'Swift'],
  ['kotlin', 'Kotlin'],
  ['kt', 'Kotlin'],
  ['scala', 'Scala'],
  ['lua', 'Lua'],
  ['perl', 'Perl'],
  ['pl', 'Perl'],
  ['sql-ansi', 'SQL ANSI'],
  ['sql ansi', 'SQL ANSI'],
  ['sql-postgres', 'SQL PostgreSQL'],
  ['sql postgresql', 'SQL PostgreSQL'],
  ['sql-mysql', 'SQL MySQL'],
  ['sql mysql', 'SQL MySQL'],
  ['sql-sqlite', 'SQL SQLite'],
  ['sql sqlite', 'SQL SQLite'],
  ['sql-server', 'SQL Server'],
  ['sql server', 'SQL Server'],
  ['sql-oracle', 'SQL Oracle'],
  ['sql oracle', 'SQL Oracle'],
  ['sql-bigquery', 'SQL BigQuery'],
  ['sql bigquery', 'SQL BigQuery'],
  ['sql-snowflake', 'SQL Snowflake'],
  ['sql snowflake', 'SQL Snowflake'],
  ['html', 'HTML'],
  ['css', 'CSS'],
  ['json', 'JSON'],
  ['yaml', 'YAML'],
  ['yml', 'YAML'],
  ['toml', 'TOML'],
  ['xml', 'XML'],
  ['dtd', 'DTD'],
  ['ini', 'INI'],
  ['protobuf', 'Protocol Buffers'],
  ['proto', 'Protocol Buffers'],
  ['protocol buffers', 'Protocol Buffers'],
  ['graphql', 'GraphQL'],
  ['gql', 'GraphQL'],
  ['csv', 'CSV'],
  ['json5', 'JSON5'],
  ['markdown', 'Markdown'],
  ['md', 'Markdown'],
  ['lino', 'LiNo'],
  ['txt', 'txt'],
  ['text', 'txt'],
  ['plain text', 'txt'],
  ['pdf', 'PDF'],
  ['docx', 'DOCX'],
  ['english', 'English'],
  ['en', 'English'],
  ['mandarin chinese', 'Mandarin Chinese'],
  ['chinese', 'Mandarin Chinese'],
  ['zh', 'Mandarin Chinese'],
  ['hindi', 'Hindi'],
  ['hi', 'Hindi'],
  ['spanish', 'Spanish'],
  ['es', 'Spanish'],
  ['modern standard arabic', 'Modern Standard Arabic'],
  ['arabic', 'Modern Standard Arabic'],
  ['ar', 'Modern Standard Arabic'],
  ['french', 'French'],
  ['fr', 'French'],
  ['bengali', 'Bengali'],
  ['bn', 'Bengali'],
  ['portuguese', 'Portuguese'],
  ['pt', 'Portuguese'],
  ['russian', 'Russian'],
  ['ru', 'Russian'],
  ['urdu', 'Urdu'],
  ['ur', 'Urdu'],
]);

const PACK_GRAMMARS = Object.freeze({
  JavaScript: 'javascript',
  Rust: 'rust',
  Lean: 'lean',
  Python: 'python',
  C: 'c',
  'C++': 'cpp',
  'C#': 'csharp',
  Java: 'java',
  TypeScript: 'typescript',
  TSX: 'tsx',
  'Visual Basic': 'vb',
  'Delphi/Object Pascal': 'pascal',
  Go: 'go',
  R: 'r',
  Ruby: 'ruby',
  PHP: 'php',
  Swift: 'swift',
  Kotlin: 'kotlin',
  Scala: 'scala',
  Lua: 'lua',
  Perl: 'perl',
  'SQL ANSI': 'sql',
  'SQL PostgreSQL': 'sql',
  'SQL MySQL': 'sql',
  'SQL SQLite': 'sql',
  'SQL Server': 'sql',
  'SQL Oracle': 'sql',
  'SQL BigQuery': 'sql_bigquery',
  'SQL Snowflake': 'sql',
  HTML: 'html',
  CSS: 'css',
  JSON: 'json',
  YAML: 'yaml',
  TOML: 'toml',
  XML: 'xml',
  DTD: 'dtd',
  INI: 'ini',
  'Protocol Buffers': 'proto',
  GraphQL: 'graphql',
  CSV: 'csv',
  JSON5: 'json5',
  Markdown: 'markdown',
  DOCX: 'xml',
});

const LEAN_PUBLIC_ROOT = 'file';
const ROCQ_BUILTIN_TYPES = new Set(['bool', 'nat', 'Prop', 'Set', 'SProp', 'Type', 'Z']);
const NATURAL_LANGUAGES = new Set([
  'English',
  'Mandarin Chinese',
  'Hindi',
  'Spanish',
  'Modern Standard Arabic',
  'French',
  'Bengali',
  'Portuguese',
  'Russian',
  'Urdu',
]);

/** Returns the canonical name for a grammar-backed JavaScript frontend. */
export function canonicalProgrammingLanguage(language) {
  return LANGUAGE_ALIASES.get(String(language).toLowerCase());
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
    parsed.tree.flags = parsed.tree.flags.withError();
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
  const tree = clipTreeNode(parsed.tree, tokenIndexes, byteEnd, endCoordinate);
  return { ...parsed, rootTerm: tree.term, tokens, tree };
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
  if (NATURAL_LANGUAGES.has(canonical)) {
    return parseNaturalLanguageGrammar(text, canonical, boundaries);
  }
  let root;
  let adapter;

  if (canonical === 'Rocq') {
    const parser = new WebTreeSitterParser();
    parser.setLanguage(ROCQ_GRAMMAR);
    root = parser.parse(text).rootNode;
    adapter = NODE_TREE_SITTER_ADAPTER;
  } else {
    const grammar = PACK_GRAMMARS[canonical];
    if (!grammar) {
      throw new Error(`no tree-sitter grammar is registered for ${canonical}`);
    }
    const parsed = TreeSitterLanguagePack.getParser(grammar).parse(text);
    if (!parsed) {
      throw new Error(`tree-sitter parser returned no ${canonical} syntax tree`);
    }
    root = parsed.rootNode();
    adapter = LANGUAGE_PACK_ADAPTER;
  }

  const byteOffsets = new Map(
    [...boundaries.entries()].map(([offset, coordinate]) => [coordinate.byte, offset]),
  );
  const tokens = [];
  const tree = convertGrammarNode(
    root,
    adapter,
    canonical,
    text,
    boundaries,
    byteOffsets,
    tokens,
  );

  // Preserve the original public Lean root while retaining the grammar's
  // `module` root immediately below it. Consumers can query either layer.
  const publicTree = canonical === 'Lean'
    ? {
        term: LEAN_PUBLIC_ROOT,
        children: [tree],
        named: true,
        span: spanFor(boundaries, 0, text.length),
        flags: tree.flags,
      }
    : tree;

  return { canonical, rootTerm: publicTree.term, tokens, tree: publicTree };
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
  const valid = !hasLineError && (validate?.(text) ?? true);
  const tree = {
    term: rootTerm,
    children,
    named: true,
    span: spanFor(boundaries, 0, text.length),
    flags: valid ? LinkFlags.clean() : LinkFlags.clean().withError(),
  };
  return { canonical, rootTerm, tokens, tree };
}

function parseNaturalLanguageGrammar(text, canonical, boundaries) {
  const tokens = [];
  const children = [];
  let sentenceStart = 0;
  const terminal = /[.!?\u0964\u3002\u061f\u06d4\uff01\uff1f]/u;
  for (let offset = 0; offset < text.length;) {
    const character = codePointAt(text, offset);
    offset += character.length;
    if (!terminal.test(character)) {
      continue;
    }
    while (offset < text.length && /\s/u.test(codePointAt(text, offset))) {
      offset += codePointAt(text, offset).length;
    }
    children.push(naturalSentence(text, sentenceStart, offset, boundaries, tokens));
    sentenceStart = offset;
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
  const pattern = /\s+|[\p{L}\p{N}\p{M}_'-]+|[^\s\p{L}\p{N}\p{M}_'-]+/gu;
  pattern.lastIndex = start;
  while (pattern.lastIndex < end) {
    const match = pattern.exec(text);
    if (!match || match.index >= end) {
      break;
    }
    const tokenEnd = Math.min(pattern.lastIndex, end);
    const value = text.slice(match.index, tokenEnd);
    const whitespace = /^\s+$/u.test(value);
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
  const trimmed = line.trim();
  if (/^\(\d+(?::\s*[\d\s]+)?\)$/u.test(trimmed) || /^\d+(?:\s+\d+)+$/u.test(trimmed)) {
    return 'link';
  }
  if (/^[^:\n]+:\s*$/u.test(trimmed)) {
    return 'definition';
  }
  return 'lino_error';
}

function pdfLineTerm(line) {
  const trimmed = line.trim();
  if (trimmed.startsWith('%PDF-')) return 'header';
  if (trimmed === '%%EOF') return 'end_of_file';
  if (/^\d+\s+\d+\s+obj$/u.test(trimmed)) return 'object_header';
  if (trimmed === 'endobj') return 'object_end';
  if (trimmed === 'xref') return 'cross_reference_table';
  if (trimmed === 'trailer') return 'trailer';
  if (trimmed === 'stream' || trimmed === 'endstream') return 'stream_boundary';
  return 'pdf_line';
}

function validatePdf(text) {
  return /^%PDF-\d+\.\d+/u.test(text) && /%%EOF\s*$/u.test(text);
}

function validateBalancedParentheses(text) {
  let depth = 0;
  for (const character of text) {
    if (character === '(') depth += 1;
    if (character === ')' && depth-- === 0) return false;
  }
  return depth === 0;
}

function convertGrammarNode(node, adapter, canonical, text, boundaries, byteOffsets, tokens) {
  const start = adapter.startOffset(node, byteOffsets);
  const end = adapter.endOffset(node, byteOffsets);
  const children = [];
  let coveredUntil = start;

  for (const { node: child, field } of adapter.children(node)) {
    const childStart = adapter.startOffset(child, byteOffsets);
    if (coveredUntil < childStart) {
      children.push(grammarTokenNode(
        'whitespace',
        coveredUntil,
        childStart,
        false,
        LinkFlags.clean().withExtra(),
        text,
        boundaries,
        tokens,
      ));
    }
    const converted = convertGrammarNode(
      child,
      adapter,
      canonical,
      text,
      boundaries,
      byteOffsets,
      tokens,
    );
    converted.field = field;
    children.push(converted);
    coveredUntil = Math.max(coveredUntil, adapter.endOffset(child, byteOffsets));
  }

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

  if (coveredUntil < end) {
    children.push(grammarTokenNode(
      'whitespace',
      coveredUntil,
      end,
      false,
      LinkFlags.clean().withExtra(),
      text,
      boundaries,
      tokens,
    ));
  }

  return {
    term: adapter.term(node),
    children,
    named: adapter.isNamed(node),
    span: spanFor(boundaries, start, end),
    flags: grammarFlags(node, adapter),
  };
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

const NODE_TREE_SITTER_ADAPTER = Object.freeze({
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

const LANGUAGE_PACK_ADAPTER = Object.freeze({
  term: (node) => node.kind(),
  startOffset: (node, byteOffsets) => byteOffsets.get(node.startByte()),
  endOffset: (node, byteOffsets) => byteOffsets.get(node.endByte()),
  isNamed: (node) => node.isNamed(),
  isError: (node) => node.isError(),
  isMissing: (node) => node.isMissing(),
  isExtra: (node) => node.isExtra(),
  hasError: (node) => node.hasError(),
  children: (node) => {
    const cursor = node.walk();
    if (!cursor.gotoFirstChild()) {
      return [];
    }
    const children = [];
    do {
      children.push({ node: cursor.node(), field: cursor.fieldName() });
    } while (cursor.gotoNextSibling());
    return children;
  },
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
