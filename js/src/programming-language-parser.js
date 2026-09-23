import TreeSitterLanguagePack from '@kreuzberg/tree-sitter-language-pack';
import TreeSitter from 'tree-sitter';
import RocqGrammar from 'tree-sitter-rocq/bindings/node/index.js';

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
});

const LEAN_PUBLIC_ROOT = 'file';
const ROCQ_BUILTIN_TYPES = new Set(['bool', 'nat', 'Prop', 'Set', 'SProp', 'Type', 'Z']);

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

  return parseGrammarCst(text, canonical);
}

function parseGrammarCst(text, canonical) {
  const boundaries = sourceBoundaries(text);
  let root;
  let adapter;

  if (canonical === 'Rocq') {
    const parser = new TreeSitter();
    // Pass the complete generated binding, not only its language pointer. The
    // Node runtime also needs nodeTypeInfo when it unmarshals named children.
    parser.setLanguage(RocqGrammar);
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
