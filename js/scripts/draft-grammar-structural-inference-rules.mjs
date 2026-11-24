import { draftSourceLibrary } from './draft-source-library.mjs';

if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
draftSourceLibrary({
  sourcePath: 'js/src/grammar-structural-inference-rules.js',
  moduleName: 'grammar_structural_inference_rules',
  directory: '',
  model: 'recursive grammar inference from labelled delimiter productions 1',
  sourceDependencies: true,
  nativeEntryPoints: 'source module exports 1',
  programAsset: 'grammar-structural-inference-program',
  programEntryPoints: 'source module exports 1',
});
