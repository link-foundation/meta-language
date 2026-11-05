import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source generation without behavioral execution');
draftSourceLibrary({ sourcePath: 'js/src/grammar-inference-rules.js', directory: '',
  moduleName: 'grammar_inference_rules', model: 'labelled regular grammar discovery 1',
  referenceSemantics: false,
  // rustfmt currently reports an internal trailing-whitespace failure on this
  // automatically emitted body. Preserve the draft; keep CI formatting gates.
  sourceFormatting: 'preserve',
  programAsset: 'grammar-inference-program' });
