import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-byte-pattern-rules.js',moduleName:'rust_byte_pattern_rules',model:'ordinary Rust unsigned byte literal patterns 1',programAsset:'rust-byte-pattern-program'});
