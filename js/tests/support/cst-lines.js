// Canonical CST lines: every node of a concrete syntax tree in preorder, one per line, as
//   <2 spaces per depth><field: ><•>LABEL SR:SC-ER:EC
// where LABEL is the node kind (named), its JSON-quoted kind (anonymous), `ERROR`, or
// `MISSING <kind>`, `•` marks nodes containing an error, fields are shown for named nodes, and
// points are zero-based rows and UTF-8 byte columns. The format is produced from the native
// `tree-sitter parse --cst` output (the conformance oracle) and from public LinkNetworks.
import { LinkType } from '../../src/index.js';

const LINE = /^(\s*)(?:([a-z_][a-z0-9_]*): )?(•)?(ERROR|MISSING (?:"(?:[^"\\]|\\.)*"|\S+)|"(?:[^"\\]|\\.)*"|\S+) (\d+):(\d+)-(\d+):(\d+)$/u;

/** Parses canonical CST lines into `{ depth, field, hasError, error, missing, named, kind, start, end }`. */
export function parseCstLines(text) {
  if (!text) return [];
  return text.split('\n').map((line) => {
    const match = LINE.exec(line);
    if (!match) throw new Error(`not a canonical CST line: ${JSON.stringify(line)}`);
    const [, indent, field, bullet, label, sr, sc, er, ec] = match;
    const missing = label.startsWith('MISSING ');
    const kindLabel = missing ? label.slice('MISSING '.length) : label;
    const named = !kindLabel.startsWith('"');
    return {
      depth: indent.length / 2,
      field: field ?? null,
      hasError: Boolean(bullet),
      error: label === 'ERROR',
      missing,
      named,
      kind: named ? kindLabel : JSON.parse(kindLabel),
      start: { row: Number(sr), column: Number(sc) },
      end: { row: Number(er), column: Number(ec) },
    };
  });
}

/** Formats one node record as a canonical CST line. */
export function formatCstLine(node) {
  const kind = node.named ? node.kind : JSON.stringify(node.kind);
  const label = node.error ? 'ERROR' : node.missing ? `MISSING ${kind}` : kind;
  const field = node.field && (node.named || node.error || node.missing) ? `${node.field}: ` : '';
  const bullet = node.hasError && !node.missing ? '•' : '';
  const { start, end } = node;
  return `${'  '.repeat(node.depth)}${field}${bullet}${label} ${start.row}:${start.column}-${end.row}:${end.column}`;
}

const CLI_RANGE = /^(\d+):(\d+) +- (\d+):(\d+) +/u;
const CLI_ESCAPES = { n: '\n', r: '\r', t: '\t', 0: '\0', '\\': '\\', v: '\v', f: '\f' };

// `(x as f64).log10() as usize` in the CLI: the digit count minus one, and 0 for 0.
const log10 = (value) => (value === 0 ? 0 : String(value).length - 1);

/** The CLI's range column width: the widest `log10(row) + log10(line length) + 1` of the source. */
function cliTotalWidth(source) {
  const lines = source.split('\n');
  if (lines.at(-1) === '') lines.pop();
  const widths = lines.map((line, row) => log10(row) + log10(Buffer.byteLength(line.replace(/\r$/u, ''), 'utf8')) + 1);
  return widths.length ? Math.max(...widths) : 1;
}

/**
 * Converts `tree-sitter parse --cst` output (ANSI colours allowed) of `source` into canonical
 * CST lines, following the renderer of the pinned CLI (0.25.10, `cli/src/parse.rs`): each line
 * is a padded range, `"  ".repeat(depth + 1)`, one more space for error-free nodes printed inside
 * an error subtree, then `field: `, `•` (named nodes containing an error), and the kind.
 * `offset` shifts the points of a region parsed on its own to its place in a host document.
 */
export function cliCstToLines(output, source, offset = { row: 0, column: 0 }) {
  // eslint-disable-next-line no-control-regex
  const lines = output.replace(/\u001b\[[0-9;]*m/gu, '').split('\n');
  const totalWidth = cliTotalWidth(source);
  const pad = (row, column) => ' '.repeat(Math.max(1, totalWidth - log10(row) - log10(column)));
  const shift = ({ row, column }) => ({ row: row + offset.row, column: row === 0 ? column + offset.column : column });
  const nodes = [];
  for (const line of lines) {
    const range = CLI_RANGE.exec(line);
    if (!range) continue;
    const [sr, sc, er, ec] = range.slice(1, 5).map(Number);
    const prefix = `${sr}:${sc}${pad(sr, sc)}- ${er}:${ec}${pad(er, ec)}`;
    if (!line.startsWith(prefix)) throw new Error(`unexpected CLI range padding: ${JSON.stringify(line)}`);
    const body = line.slice(prefix.length);
    const content = body.trimStart();
    // Multi-line leaf text continues on the following lines, each starting with a backtick.
    if (content.startsWith('`')) continue;
    const depth = Math.floor((body.length - content.length) / 2) - 1;
    const match = /^(?:([a-z_][a-z0-9_]*): )?(•)?(.*)$/u.exec(content.trimEnd());
    const [, field, bullet] = match;
    let rest = match[3];
    const node = { depth, field: field ?? null, hasError: Boolean(bullet), error: false, missing: false };
    if (rest.startsWith('MISSING: ')) {
      node.missing = true;
      rest = rest.slice('MISSING: '.length);
    }
    if (rest.startsWith('"') && !field && !bullet) {
      node.named = false;
      node.kind = rest.slice(1, -1).replace(/\\([nrt0\\vf])/gu, (_, code) => CLI_ESCAPES[code]);
    } else {
      node.named = true;
      node.kind = rest.split(' ')[0];
      node.error = node.kind === 'ERROR';
    }
    node.start = shift({ row: sr, column: sc });
    node.end = shift({ row: er, column: ec });
    nodes.push(node);
  }
  // A named MISSING node goes through the CLI's named branch: a childless node marked `•`
  // that is not an ERROR.
  nodes.forEach((node, index) => {
    const leaf = nodes[index + 1]?.depth !== node.depth + 1;
    if (node.named && node.hasError && !node.error && leaf) node.missing = true;
  });
  return nodes.map(formatCstLine).join('\n');
}

// The public parser adds two documented layers on top of the upstream grammar trees:
// Lean's `file` root wraps the grammar's `module` root, and every Rocq `ident` leaf gets a
// semantic `identifier`/`primitive_type` token child. The projection removes exactly those.
const SEMANTIC_ROCQ_LEAVES = new Set(['identifier', 'primitive_type']);

function networkIndex(network) {
  const links = [...network.links()];
  const byId = new Map(links.map((link) => [link.id().value, link]));
  const fields = new Map();
  const regionRoots = new Set();
  const referenced = new Set();
  for (const link of links) {
    const { linkType, term } = link.metadata();
    if (linkType === LinkType.Field) {
      const [parent, child] = link.references();
      fields.set(`${parent.value}:${child.value}`, term);
    } else if (linkType === LinkType.Region) {
      for (const reference of link.references().slice(2)) regionRoots.add(reference.value);
    } else if (linkType === LinkType.Syntax) {
      for (const reference of link.references()) referenced.add(reference.value);
    }
  }
  const children = (link) => link.references()
    .map((reference) => byId.get(reference.value))
    .filter((child) => child?.metadata().linkType === LinkType.Syntax);
  return { links, byId, fields, regionRoots, referenced, children };
}

/** The grammar roots of the document itself (not its embedded regions), after projection. */
export function documentGrammarRoots(network, language) {
  const index = networkIndex(network);
  const roots = index.links.filter((link) => link.metadata().linkType === LinkType.Syntax &&
    !index.referenced.has(link.id().value) && !index.regionRoots.has(link.id().value));
  return { index, roots: projectRoots(index, roots, language) };
}

/** The grammar roots of every region link, after projection, with the region language and span. */
export function regionGrammarRoots(network) {
  const index = networkIndex(network);
  return index.links.filter((link) => link.metadata().linkType === LinkType.Region).map((region) => {
    const language = index.byId.get(region.references()[1].value).metadata().term;
    const roots = region.references().slice(2).map((reference) => index.byId.get(reference.value))
      .filter((link) => link.metadata().linkType === LinkType.Syntax);
    return { language, span: region.metadata().span, index, roots: projectRoots(index, roots, language) };
  });
}

function projectRoots(index, roots, language) {
  return language === 'Lean'
    ? roots.flatMap((root) => (root.metadata().term === 'file' ? index.children(root) : [root]))
    : roots;
}

/** Renders `roots` of a network index as canonical CST lines plus the rendered link ids. */
export function renderCstLines({ index, roots }, language) {
  const lines = [];
  const rendered = new Set();
  const visit = (link, depth, field) => {
    const metadata = link.metadata();
    let children = index.children(link);
    if (language === 'Rocq' && metadata.term === 'ident' && children.length === 1 &&
      SEMANTIC_ROCQ_LEAVES.has(children[0].metadata().term) && index.children(children[0]).length === 0) {
      children = [];
    }
    rendered.add(link.id().value);
    const { flags, span } = metadata;
    lines.push(formatCstLine({
      depth,
      field: field ?? null,
      hasError: Boolean(flags.hasError),
      error: Boolean(flags.isError),
      missing: Boolean(flags.isMissing),
      named: metadata.named,
      kind: metadata.term,
      start: span.start,
      end: span.end,
    }));
    for (const child of children) visit(child, depth + 1, index.fields.get(`${link.id().value}:${child.id().value}`));
  };
  for (const root of roots) visit(root, 0);
  return { text: lines.join('\n'), rendered };
}

/** Byte offset of every row start in `source`. */
export function rowOffsets(source) {
  const bytes = Buffer.from(source, 'utf8');
  const offsets = [0];
  for (let index = 0; index < bytes.length; index += 1) if (bytes[index] === 0x0a) offsets.push(index + 1);
  return offsets;
}

const toByte = (offsets, point) => offsets[point.row] + point.column;

/**
 * Trivia problems: every byte outside the oracle's leaf tokens must be covered by a trivia
 * link, and every trivia link must cover only such bytes or exactly one oracle node (a comment).
 * The one exception is text a hidden grammar rule matched (Lean's `#eval`), which the CLI
 * prints no leaf for: a non-extra source token outside every leaf, trimmed of whitespace.
 */
export function triviaProblems(network, source, oracleText, window = null) {
  const offsets = rowOffsets(source);
  const nodes = parseCstLines(oracleText);
  const [from, to] = window ?? [0, Buffer.byteLength(source, 'utf8')];
  const inLeaf = new Uint8Array(to - from);
  const nodeRanges = new Set();
  nodes.forEach((node, position) => {
    const start = toByte(offsets, node.start);
    const end = toByte(offsets, node.end);
    nodeRanges.add(`${start}:${end}`);
    const leaf = nodes[position + 1]?.depth !== node.depth + 1;
    if (leaf) inLeaf.fill(1, start - from, end - from);
  });
  const covered = new Uint8Array(to - from);
  const problems = [];
  const bytes = Buffer.from(source, 'utf8');
  for (const link of network.links()) {
    const metadata = link.metadata();
    const hiddenText = metadata.linkType === LinkType.SourceToken && !metadata.flags?.isExtra;
    if (!hiddenText && (metadata.linkType !== LinkType.Trivia || metadata.term !== 'token trivia')) continue;
    const { start, end } = metadata.span.byteRange;
    if (start < from || end > to) continue;
    const gap = inLeaf.subarray(start - from, end - from).every((flag) => flag === 0);
    if (hiddenText) {
      if (!gap || start === end) continue;
      if (/^[\p{White_Space}\u200B\u2060\uFEFF]|[\p{White_Space}\u200B\u2060\uFEFF]$/u.test(bytes.subarray(start, end).toString('utf8'))) {
        problems.push(`hidden-rule text ${start}..${end} keeps surrounding whitespace`);
      }
      covered.fill(1, start - from, end - from);
      continue;
    }
    covered.fill(1, start - from, end - from);
    if (!gap && !nodeRanges.has(`${start}:${end}`)) problems.push(`trivia ${start}..${end} overlaps a token`);
  }
  for (let byte = 0; byte < to - from; byte += 1) {
    if (!inLeaf[byte] && !covered[byte]) {
      problems.push(`byte ${byte + from} is outside every token and every trivia link`);
      break;
    }
  }
  return problems;
}

/**
 * Diagnostic problems: the verification report must list exactly the oracle's error, missing
 * and error-containing nodes among the rendered links, and nothing else; with `checkClean`
 * (for a whole document), `isClean()` must also agree with the oracle.
 */
export function diagnosticProblems(network, rendered, oracleText, { checkClean = true } = {}) {
  const expected = parseCstLines(oracleText).filter((node) => node.hasError || node.missing || node.error)
    .map((node) => `${node.error ? 'error' : node.missing ? 'missing' : 'has-error'} ${node.start.row}:${node.start.column}-${node.end.row}:${node.end.column}`)
    .sort();
  const byId = new Map([...network.links()].map((link) => [link.id().value, link]));
  const actual = [];
  const report = network.verifyFullMatch();
  for (const issue of report.issues) {
    if (!rendered.has(issue.linkId.value)) continue;
    const { span } = byId.get(issue.linkId.value).metadata();
    const kind = issue.flags.isError ? 'error' : issue.flags.isMissing ? 'missing' : 'has-error';
    actual.push(`${kind} ${span.start.row}:${span.start.column}-${span.end.row}:${span.end.column}`);
  }
  actual.sort();
  const problems = [];
  if (JSON.stringify(actual) !== JSON.stringify(expected)) {
    problems.push(`diagnostics ${JSON.stringify(actual)} differ from the oracle ${JSON.stringify(expected)}`);
  }
  if (checkClean && report.isClean() !== (expected.length === 0)) problems.push(`isClean() is ${report.isClean()}`);
  return problems;
}

/** The first differing line of two CST texts, for failure messages. */
export function firstDifference(actual, expected) {
  const left = actual.split('\n');
  const right = expected.split('\n');
  const index = left.findIndex((line, position) => line !== right[position]);
  const at = index < 0 ? Math.min(left.length, right.length) : index;
  return `line ${at + 1}\n  actual   ${left[at]}\n  expected ${right[at]}`;
}

/**
 * Problems of the public tree of a whole `source` document against its oracle CST lines:
 * structure, kinds, fields, spans and flags, exact reconstruction, trivia and diagnostics.
 * Returns the rendered public tree too.
 */
export function documentOracleProblems(network, language, source, oracleText) {
  const { text, rendered } = renderCstLines(documentGrammarRoots(network, language), language);
  const problems = [];
  if (text !== oracleText) problems.push(`CST differs at ${firstDifference(text, oracleText)}`);
  if (network.reconstructText() !== source) problems.push('reconstruction differs from the source');
  problems.push(...triviaProblems(network, source, oracleText), ...diagnosticProblems(network, rendered, oracleText));
  return { problems, text };
}

/** The S-expression of canonical CST lines: named nodes with fields, ERROR and MISSING. */
export function cstLinesToSexp(text) {
  const nodes = parseCstLines(text);
  let output = '';
  const stack = [];
  const close = (depth) => {
    while (stack.length && stack.at(-1) >= depth) {
      stack.pop();
      output += ')';
    }
  };
  for (const node of nodes) {
    close(node.depth);
    if (node.missing) {
      output += ` ${node.field ? `${node.field}: ` : ''}(MISSING ${node.named ? node.kind : JSON.stringify(node.kind)}`;
    } else if (node.error || node.named) {
      output += ` ${node.field ? `${node.field}: ` : ''}(${node.error ? 'ERROR' : node.kind}`;
    } else {
      continue;
    }
    stack.push(node.depth);
  }
  close(0);
  return output.trim();
}
