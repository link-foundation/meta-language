// Compares the native tree-sitter CLI tree and the JS runtime tree of one input and of
// its metamorphic variants, to tell a runtime defect from a relation the grammar breaks.
// Usage: TREE_SITTER_CLI=tree-sitter node experiments/issue-195-generative-oracle.mjs LANGUAGE JSON_SOURCE
import { mkdtemp, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

import { LinkNetwork } from '../js/src/index.js';
import { cargoLockVersions } from '../js/scripts/build-vendored-grammars.mjs';
import { cliGrammarDirectory, cliOutput } from '../js/scripts/generate-default-cst-expectations.mjs';
import { cliCstToLines, documentGrammarRoots, renderCstLines } from '../js/tests/support/cst-lines.js';
import { RELATIONS, relationHolds } from '../js/tests/support/generative.js';

const [language, json] = process.argv.slice(2);
const source = JSON.parse(json);
const scratch = await mkdtemp(join(tmpdir(), 'generative-oracle-'));
process.env.TREE_SITTER_LIBDIR = join(scratch, 'lib');
const directory = await cliGrammarDirectory(language.toLowerCase(), await cargoLockVersions(), scratch);
const cli = async (text, name) => cliCstToLines(await cliOutput(process.env.TREE_SITTER_CLI ?? 'tree-sitter', directory, text, scratch, name), text);
const runtime = (text) => {
  const network = LinkNetwork.parse(text, language);
  return renderCstLines(documentGrammarRoots(network, language), language).text;
};
const base = { cli: await cli(source, 'base'), runtime: runtime(source) };
console.log('base: runtime equals CLI', base.cli === base.runtime);
for (const [relation, { transform }] of Object.entries(RELATIONS)) {
  const variant = transform(source);
  const cliVariant = await cli(variant, relation);
  const runtimeVariant = runtime(variant);
  console.log(relation, 'runtime equals CLI', cliVariant === runtimeVariant,
    'holds for CLI', relationHolds(relation, base.cli, cliVariant), 'holds for runtime', relationHolds(relation, base.runtime, runtimeVariant));
  if (process.env.VERBOSE) console.log(`--- base CLI\n${base.cli}\n--- variant CLI\n${cliVariant}`);
}
await rm(scratch, { recursive: true, force: true });
