import { draftSourceLibrary } from './draft-source-library.mjs';

if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
draftSourceLibrary({
  sourcePath: 'js/src/grammar-scanner-inference-rules.js',
  moduleName: 'grammar_scanner_inference_rules',
  directory: '',
  model: 'finite grammar scanner inference from ordered observations 1',
  sourceDependencies: true,
  nativeEntryPoints: 'source module exports 1',
  programAsset: 'grammar-scanner-inference-program',
  programEntryPoints: 'source module exports 1',
});
