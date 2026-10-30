import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-byte-string-rules.js',moduleName:'rust_byte_string_rules',
 model:'complete Rust byte string lexical decisions 1',programAsset:'rust-byte-string-program',sourceDependencies:true,sourceAliases:true});
