import { draftSourceLibrary } from './draft-source-library.mjs';

if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
draftSourceLibrary({
  sourcePath: 'js/src/translation/rust-concrete-source-rules.js',
  moduleName: 'rust_concrete_source_rules',
  directory: '',
  model: 'actual Rust concrete source owner coordinates and kinds 1',
  programAsset: 'rust-concrete-source-program',
});
