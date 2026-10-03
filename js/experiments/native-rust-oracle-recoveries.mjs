// Lists the cases of the pinned tree-sitter-rust corpus the oracle recovers
// from: they cannot be matches of the native Rust fixture.
import { corpusCases, grammarSourceOf } from '../scripts/import-native-grammars.mjs';
import { oracleRecovers } from '../scripts/native-grammar-rows.mjs';

const cases = corpusCases(grammarSourceOf('native-rust'));
const recovered = cases.filter(({ source }) => oracleRecovers(source, 'Rust'));
console.log(`${cases.length} cases, ${recovered.length} the oracle recovers from`);
for (const { file, title } of recovered) console.log(`${file}: ${title}`);
