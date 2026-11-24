import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-integer-bit-rules.js',moduleName:'rust_integer_bit_rules',sourceDependencies:true,model:'Rust exact fixed integer bit recurrence source plans 1',programAsset:'rust-integer-bit-program'});
