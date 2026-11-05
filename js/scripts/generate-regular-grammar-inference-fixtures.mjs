// Declare future CI inputs only. This author imports no learner or parser.
import { writeProjectData } from './project-data.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft to author fixture declarations');
const repeated = (symbol, strategy) => ({ name: strategy + ' repeated ' + symbol,
  positives: [[symbol], [symbol, symbol]], negatives: [[]], options: { strategy },
  controls: [{ symbols: [symbol, symbol, symbol], accepted: true }, { symbols: [], accepted: false }, { symbols: ['unknown symbol'], accepted: false }] });
writeProjectData(new URL('../../parity/fixtures/regular-grammar-inference.lino', import.meta.url), {
  schemaVersion: 1, inputDomain: 'prefix-free nonempty Unicode symbols; declared examples are premises, not observations of implementation execution',
  versionRanges: { JavaScript: 'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0', Rust: '>=1.90.0 <1.100.0; edition 2024' },
  cases: ['rpni', 'edsm', 'alergia'].flatMap(strategy => [repeated('a', strategy), repeated('🙂', strategy)]),
  refusals: [
    { name: 'contradictory labels', positives: [['a']], negatives: [['a']], options: {}, reason: 'contradictory positive and negative grammar inference samples' },
    { name: 'state budget', positives: [['a']], negatives: [], options: { stateLimit: 1 }, reason: 'grammar inference state limit exceeded' },
    { name: 'pair budget', positives: [[], ['a'], ['a', 'a']], negatives: [], options: { pairLimit: 1 }, reason: 'grammar inference pair limit exceeded' },
    { name: 'attempt budget', positives: [['a'], ['a', 'a']], negatives: [[]], options: { attemptLimit: 1 }, reason: 'grammar inference merge attempt limit exceeded' },
    { name: 'symbol segmentation', positives: [['a'], ['ab']], negatives: [], options: {}, reason: 'overlapping symbol prefixes require a separate lexer definition' },
  ],
});
