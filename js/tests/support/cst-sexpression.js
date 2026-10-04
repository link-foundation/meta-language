// Renders the Syntax links of a public LinkNetwork as a tree-sitter S-expression
// (`tree-sitter parse` style: named nodes, field labels, ERROR and MISSING), and
// reads tree-sitter test corpora (test/corpus/*.txt), so public parses can be
// compared with trees printed by upstream grammars and the native CLI.
import { LinkType } from '../../src/index.js';

/** Splits a tree-sitter corpus file into `{ name, attributes, source, expected }` cases. */
export function parseCorpus(text) {
  const cases = [];
  const header = /^(={3,})([^=\r\n]*)\r?\n([\s\S]*?)\r?\n\1\2\r?\n/gmu;
  const headers = [...text.matchAll(header)];
  for (const [index, match] of headers.entries()) {
    const [nameLine, ...attributeLines] = match[3].split(/\r?\n/u);
    const bodyStart = match.index + match[0].length;
    const bodyEnd = index + 1 < headers.length ? headers[index + 1].index : text.length;
    const body = text.slice(bodyStart, bodyEnd);
    const suffix = match[2].trim();
    const dividers = [...body.matchAll(new RegExp(`^-{3,}${suffix.replace(/[^\w]/gu, '\\$&')}\\r?\\n`, 'gmu'))];
    const divider = dividers.at(-1);
    let source = body.slice(0, divider ? divider.index : body.length);
    if (source.endsWith('\n')) source = source.slice(0, -1);
    if (source.endsWith('\r')) source = source.slice(0, -1);
    const expected = divider ? body.slice(divider.index + divider[0].length).trim() : '';
    cases.push({
      name: nameLine.trim(),
      attributes: attributeLines.map((line) => line.trim()).filter(Boolean),
      source,
      expected,
    });
  }
  return cases;
}

// The public parser adds two documented layers on top of the upstream grammar trees:
// Lean's `file` root wraps the grammar's `module` root, and every Rocq `ident` leaf gets a
// semantic `identifier`/`primitive_type` token child. The projection removes exactly those.
const SEMANTIC_ROCQ_LEAVES = new Set(['identifier', 'primitive_type']);

/** The grammar tree of `network` as an S-expression, with the public layers projected away. */
export function renderNetwork(network, language) {
  const links = [...network.links()];
  const byId = new Map(links.map((link) => [link.id().value, link]));
  const syntax = links.filter((link) => link.metadata().linkType === LinkType.Syntax);
  const referenced = new Set();
  for (const link of syntax) for (const reference of link.references()) referenced.add(reference.value);
  const fields = new Map();
  for (const { parent, label, child } of network.fieldRelations()) {
    fields.set(`${parent.value}:${child.value}`, label);
  }
  const roots = syntax.filter((link) => !referenced.has(link.id().value));
  const syntaxChildren = (link) => link.references()
    .map((reference) => byId.get(reference.value))
    .filter((child) => child?.metadata().linkType === LinkType.Syntax);
  const render = (link, field) => {
    const metadata = link.metadata();
    const children = syntaxChildren(link);
    const parts = children
      .map((child) => render(child, fields.get(`${link.id().value}:${child.id().value}`)))
      .filter(Boolean);
    const prefix = field ? `${field}: ` : '';
    if (language === 'Rocq' && metadata.term === 'ident' && children.length === 1 &&
      SEMANTIC_ROCQ_LEAVES.has(children[0].metadata().term) && syntaxChildren(children[0]).length === 0) {
      return `${prefix}(ident)`;
    }
    if (metadata.flags.isMissing) {
      const term = metadata.named ? metadata.term : JSON.stringify(metadata.term);
      return `${prefix}(MISSING ${term})`;
    }
    if (metadata.flags.isError) return `${prefix}(ERROR${parts.length ? ` ${parts.join(' ')}` : ''})`;
    if (!metadata.named) return parts.join(' ');
    return `${prefix}(${metadata.term}${parts.length ? ` ${parts.join(' ')}` : ''})`;
  };
  const grammarRoots = language === 'Lean'
    ? roots.flatMap((root) => (root.metadata().term === 'file' ? syntaxChildren(root) : [root]))
    : roots;
  return grammarRoots.map((root) => render(root)).join(' ');
}

/** Collapses whitespace so corpus trees and rendered trees compare as strings. */
export const normalize = (sexp) => sexp.replace(/\s+/gu, ' ').replace(/ \)/gu, ')').replace(/\( /gu, '(').trim();

/** Drops field labels, for corpus cases whose expected tree omits them. */
export const stripFields = (sexp) => sexp.replace(/\b[\w-]+: /gu, '');
