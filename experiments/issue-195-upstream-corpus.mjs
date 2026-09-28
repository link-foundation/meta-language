// Runs tree-sitter upstream test corpora (test/corpus/*.txt) through LinkNetwork.parse and
// compares the rendered S-expression of the network with the upstream expected tree.
//   node experiments/issue-195-upstream-corpus.mjs DIR LANGUAGE [--verbose]
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { LinkNetwork } from '../js/src/index.js';
import { normalize, parseCorpus, renderNetwork, stripFields } from '../js/tests/support/cst-sexpression.js';

export { normalize, parseCorpus, renderNetwork, stripFields };

const [directory, language] = process.argv.slice(2);
const verbose = process.argv.includes('--verbose');

if (directory && language && process.argv[1] === fileURLToPath(import.meta.url)) {
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
