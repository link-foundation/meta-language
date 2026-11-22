import { draftSourceLibrary } from './draft-source-library.mjs';

if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
draftSourceLibrary({
  sourcePath: 'js/src/grammar-attribute-inference-rules.js',
  moduleName: 'grammar_attribute_inference_rules',
  directory: '',
  model: 'native grammar attribute inference candidate values 1',
  sourceDependencies: true,
  nativeEntryPoints: 'source module exports 1',
  programAsset: 'grammar-attribute-inference-program',
  programEntryPoints: 'source module exports 1',
});
