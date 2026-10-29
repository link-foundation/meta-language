import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-byte-string-expression-rules.js',moduleName:'rust_byte_string_expression_rules',model:'ordinary Rust static byte array expressions 1',programAsset:'rust-byte-string-expression-program'});
