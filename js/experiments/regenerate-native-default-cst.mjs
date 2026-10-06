// Regenerates parity/fixtures/native-default-cst-expected.json only, without
// the per-grammar fixtures: node experiments/regenerate-native-default-cst.mjs
import { writeFileSync } from 'node:fs';
import { DEFAULT_CST_PATH, buildNativeDefaultCstExpected, renderFixture } from '../scripts/generate-native-grammar-fixtures.mjs';

const started = Date.now();
writeFileSync(new URL(`../../${DEFAULT_CST_PATH}`, import.meta.url), renderFixture(buildNativeDefaultCstExpected()));
console.log(`wrote ${DEFAULT_CST_PATH} in ${Date.now() - started} ms`);
