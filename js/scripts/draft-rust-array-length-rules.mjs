import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-array-length-rules.js',moduleName:'rust_array_length_rules',model:'ordinary exact 64-bit Rust array length through existing integer operations 1',programAsset:'rust-array-length-program'});
