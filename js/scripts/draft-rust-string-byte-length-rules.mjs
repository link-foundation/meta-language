import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-string-byte-length-rules.js',moduleName:'rust_string_byte_length_rules',sourceDependencies:true,model:'ordinary Rust Unicode scalar String UTF8 byte length 1',programAsset:'rust-string-byte-length-program'});
