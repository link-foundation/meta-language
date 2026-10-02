// Projects concrete syntax trees to the rows of
// parity/fixtures/default-cst-expected.json, `[depth, field, kind, named,
// startByte, endByte, flags]`, so a native grammar's trees can be compared
// with the tree-sitter oracle that still backs the default parse of its
// language. rust/tests/unit/issue_195_native_grammar_rows.rs projects the
// Rust trees the same way.
//
// The native tree is projected as tree-sitter places nodes: whitespace trivia
// is not a row, named trivia (comments) is an extra row (flag X), leading
// trivia belongs before the node it precedes, a node spans its first to last
// non-trivia leaf, and the root starts at its first leaf that is not
// whitespace and ends at the end of the input. Kinds in `hidden` are leaves
// the native tree keeps and the oracle drops, such as a byte order mark; they
// are projected like whitespace. Kinds in `anonymous` are leaves the oracle
// keeps inside a node without a row of their own, as tree-sitter keeps a
// regular expression token such as a line break; they are not rows but count
// in the spans. Node kinds in `extras` are rows with flag X, as the oracle
// marks a comment node it parses as an extra.
import { LinkNetwork, LinkType } from '../src/index.js';

/** The rows of the default parse of `source` as `language`. */
export function oracleRows(source, language) {
  const network = LinkNetwork.parse(source, language);
  const fields = new Map();
  for (const link of network.links()) {
    if (link.metadata().linkType === LinkType.Field && link.references().length === 3) {
      const [parent, label, child] = link.references();
      fields.set(`${parent.asU64()}:${child.asU64()}`, network.link(label).metadata().term);
    }
  }
  const syntax = network.links().filter((link) => link.metadata().linkType === LinkType.Syntax);
  const children = new Set(syntax.flatMap((link) => link.references().map((id) => id.asU64())));
  const root = syntax.find((link) => !children.has(link.id().asU64()));
  const rows = [];
  const visit = (link, depth, field) => {
    const { term, named, span, flags } = link.metadata();
    rows.push([depth, field ?? null, term, named ? 1 : 0, span.byteRange.start, span.byteRange.end,
      `${flags.isError ? 'E' : ''}${flags.isMissing ? 'M' : ''}${flags.isExtra ? 'X' : ''}`]);
    for (const reference of link.references()) {
      const child = network.link(reference);
      if (child.metadata().linkType === LinkType.Syntax) {
        visit(child, depth + 1, fields.get(`${link.id().asU64()}:${reference.asU64()}`));
      }
    }
  };
  visit(root, 0);
  return rows;
}

/** The rows of a native `SyntaxTree` of `source`. */
export function nativeRows(tree, source, { hidden = [], anonymous = [], extras = [] } = {}) {
  const hiddenKinds = new Set(hidden);
  const anonymousKinds = new Set(anonymous);
  const extraKinds = new Set(extras);
  const invisible = (node) => node.type === 'token' && ((node.trivia && node.kind === null) || hiddenKinds.has(node.kind));
  const trivia = (node) => invisible(node) || (node.type === 'token' && node.trivia);
  const hoist = (node) => {
    if (node.type !== 'node') return [node];
    const children = node.children.flatMap(hoist);
    let first = 0;
    while (first < children.length && trivia(children[first])) first += 1;
    return [...children.slice(0, first), { ...node, children: children.slice(first) }];
  };
  const span = (node) => {
    if (node.type !== 'node') return [node.start, node.end];
    const inner = node.children.filter((child) => !trivia(child)).map(span);
    return inner.length === 0 ? [node.start, node.start] : [inner[0][0], inner.at(-1)[1]];
  };
  const rows = [];
  const visit = (node, depth) => {
    if (invisible(node) || (node.type === 'token' && anonymousKinds.has(node.kind))) return;
    const [start, end] = span(node);
    if (node.type === 'node') {
      rows.push([depth, node.field ?? null, node.kind, 1, start, end, extraKinds.has(node.kind) ? 'X' : '']);
      for (const child of node.children.flatMap(hoist)) visit(child, depth + 1);
    } else {
      rows.push([depth, node.field ?? null, node.kind ?? node.text, node.kind ? 1 : 0, start, end, node.trivia ? 'X' : '']);
    }
  };
  const length = Buffer.byteLength(source);
  const leaves = [];
  const collect = (node) => (node.type === 'node' ? node.children.forEach(collect) : leaves.push(node));
  collect(tree);
  const first = leaves.find((leaf) => !invisible(leaf));
  rows.push([0, null, tree.kind, 1, first ? first.start : length, length, '']);
  for (const child of tree.children.flatMap(hoist)) visit(child, 1);
  return rows;
}

/** Whether any row carries an error or missing flag. */
export const hasRecovery = (rows) => rows.some((row) => /[EM]/u.test(row[6]));

/**
 * Whether the default parse of `source` as `language` recovers from an
 * error: a row with an error or missing flag, or a missing token the rows do
 * not show, such as a line break tree-sitter inserts at the end of the input,
 * which only the root's has-error flag records.
 */
export function oracleRecovers(source, language) {
  if (hasRecovery(oracleRows(source, language))) return true;
  const network = LinkNetwork.parse(source, language);
  return network.links().some((link) => link.metadata().linkType === LinkType.Syntax && link.metadata().flags.hasError);
}
