import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source-only authoring');
draftSourceLibrary({ sourcePath: 'js/src/translation/rust-primitive-bitwise-rules.js',
  moduleName: 'rust_primitive_bitwise_rules', model: 'total Rust primitive bitwise operations 1',
  programAsset: 'rust-primitive-bitwise-program', sourceFormatting: 'preserve' });
