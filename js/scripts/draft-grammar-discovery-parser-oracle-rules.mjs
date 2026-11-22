import { draftSourceLibrary } from './draft-source-library.mjs';

if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
draftSourceLibrary({
  sourcePath: 'js/src/grammar-discovery-parser-oracle-rules.js',
  moduleName: 'grammar_discovery_parser_oracle_rules',
  directory: '',
  model: 'bounded native grammar membership observations 1',
  programAsset: 'grammar-discovery-parser-oracle-program',
});
