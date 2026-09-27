// Runs tree-sitter upstream test corpora (test/corpus/*.txt) through LinkNetwork.parse and
// compares the rendered S-expression of the network with the upstream expected tree.
//   node experiments/issue-195-upstream-corpus.mjs DIR LANGUAGE [--verbose]
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { LinkNetwork, LinkType } from '../js/src/index.js';

const [directory, language] = process.argv.slice(2);
const verbose = process.argv.includes('--verbose');

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

export function renderNetwork(network, language) {
  const links = [...network.links()];
  const byId = new Map(links.map((link) => [link.id().value, link]));
  const syntax = links.filter((link) => link.metadata().linkType === LinkType.Syntax);
  const referenced = new Set();
  for (const link of syntax) for (const reference of link.references()) referenced.add(reference.value);
  const fields = new Map();
  for (const link of links.filter((item) => item.metadata().linkType === LinkType.Field)) {
    const [parent, child] = link.references();
    fields.set(`${parent.value}:${child.value}`, link.metadata().term);
  }
  const roots = syntax.filter((link) => !referenced.has(link.id().value));
  const render = (link, field) => {
    const metadata = link.metadata();
    const children = link.references()
      .map((reference) => byId.get(reference.value))
      .filter((child) => child?.metadata().linkType === LinkType.Syntax);
    const parts = children
      .map((child) => render(child, fields.get(`${link.id().value}:${child.id().value}`)))
      .filter(Boolean);
    const prefix = field ? `${field}: ` : '';
    if (language === 'Rocq' && metadata.term === 'ident' && children.length === 1 &&
      SEMANTIC_ROCQ_LEAVES.has(children[0].metadata().term) &&
      children[0].references().every((reference) => byId.get(reference.value)?.metadata().linkType !== LinkType.Syntax)) {
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
  if (language === 'Lean') {
    return roots.flatMap((root) => root.metadata().term === 'file'
      ? root.references().map((reference) => byId.get(reference.value))
        .filter((child) => child?.metadata().linkType === LinkType.Syntax)
      : [root]).map((root) => render(root)).join(' ');
  }
  return roots.map((root) => render(root)).join(' ');
}

export const normalize = (sexp) => sexp.replace(/\s+/gu, ' ').replace(/ \)/gu, ')').replace(/\( /gu, '(').trim();
export const stripFields = (sexp) => sexp.replace(/\b[\w-]+: /gu, '');

if (directory && language) {
  let passed = 0;
  let failed = 0;
  let errors = 0;
  for (const file of readdirSync(directory).filter((name) => name.endsWith('.txt')).sort()) {
    for (const test of parseCorpus(readFileSync(join(directory, file), 'utf8'))) {
      if (test.attributes.some((attribute) => attribute.startsWith(':language') || attribute === ':skip')) continue;
      const network = LinkNetwork.parse(test.source, language);
      const clean = network.verifyFullMatch().isClean();
      if (test.attributes.includes(':error')) {
        if (!clean) errors += 1;
        else { failed += 1; console.log(`FAIL ${file} ${test.name}: expected an error`); }
        continue;
      }
      const expected = normalize(test.expected);
      let actual = normalize(renderNetwork(network, language));
      if (!/\w: /u.test(expected)) actual = stripFields(actual);
      if (actual === expected && network.reconstructText() === test.source) passed += 1;
      else {
        failed += 1;
        console.log(`FAIL ${file} ${test.name}`);
        if (verbose) console.log(`  expected ${expected}\n  actual   ${actual}`);
      }
    }
  }
  console.log(`${language}: ${passed} passed, ${errors} expected errors, ${failed} failed`);
}
