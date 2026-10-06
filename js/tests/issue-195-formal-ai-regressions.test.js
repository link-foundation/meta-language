// The formal-ai inputs on which the JavaScript and Rust runtimes once disagreed
// (query parser, `to_lino`/`from_lino`, captures, root matches and projections),
// kept as local regression fixtures in parity/fixtures/formal-ai-regressions/.
// rust-golden.json holds the Rust probe's outputs on the same inputs: the small
// sections whole, documents and queries as one SHA-256 digest per entry.
// experiments/issue-195-formal-ai-golden.mjs regenerates it from a Rust run.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import * as api from '../src/index.js';
import {
  buildFormalAiInputs,
  canonical,
  formalAiHelpers,
} from '../scripts/issue-195-formal-ai-workload-probes.mjs';

const fixtures = new URL('../../parity/fixtures/formal-ai-regressions/', import.meta.url);
const golden = JSON.parse(readFileSync(new URL('rust-golden.json', fixtures), 'utf8'));
const javascript = formalAiHelpers(api).outputs(await buildFormalAiInputs(fixtures.pathname));
const digest = (value) => createHash('sha256').update(canonical(value)).digest('hex');

for (const section of ['ruleSet', 'seedNetwork', 'linkEdits', 'projections']) {
  test(`formal-ai regression: JavaScript ${section} equals the Rust golden`, () => {
    assert.equal(canonical(javascript[section]), canonical(golden.whole[section]));
  });
}

for (const section of ['documents', 'queries']) {
  test(`formal-ai regression: every JavaScript ${section} entry digests to the Rust golden`, () => {
    const expected = golden.digests[section];
    assert.deepEqual(Object.keys(javascript[section]).sort(), Object.keys(expected));
    const differing = Object.keys(expected).filter((key) => digest(javascript[section][key]) !== expected[key]);
    assert.deepEqual(differing, [], `entries differing from Rust: ${differing.slice(0, 5).join(', ')}`);
  });
}
