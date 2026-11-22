import { draftSourceLibrary } from './draft-source-library.mjs';

const mode = process.argv.slice(2).join(' ');
if (!['--draft', '--draft-character-classes', '--draft-mode-stack'].includes(mode)) throw new TypeError('use --draft, --draft-character-classes or --draft-mode-stack');
if (mode === '--draft') draftSourceLibrary({
  sourcePath: 'js/src/grammar-scanner-inference-rules.js',
  moduleName: 'grammar_scanner_inference_rules',
  directory: '',
  model: 'finite grammar scanner inference from ordered observations 1',
  sourceDependencies: true,
  nativeEntryPoints: 'source module exports 1',
  programAsset: 'grammar-scanner-inference-program',
  programEntryPoints: 'source module exports 1',
});
if (mode !== '--draft-mode-stack') draftSourceLibrary({
  sourcePath: 'js/src/grammar-scanner-character-class-rules.js',
  moduleName: 'grammar_scanner_character_class_rules',
  directory: '',
  model: 'observed grammar scanner character classes 1',
  sourceDependencies: true,
  nativeEntryPoints: 'source module exports 1',
  programAsset: 'grammar-scanner-character-class-program',
  programEntryPoints: 'source module exports 1',
});
if (mode !== '--draft-character-classes') draftSourceLibrary({
  sourcePath: 'js/src/grammar-scanner-mode-stack-rules.js',
  moduleName: 'grammar_scanner_mode_stack_rules',
  directory: '',
  model: 'finite grammar scanner inference from mode stack observations 1',
  sourceDependencies: true,
  nativeEntryPoints: 'source module exports 1',
  programAsset: 'grammar-scanner-mode-stack-program',
  programEntryPoints: 'source module exports 1',
});
