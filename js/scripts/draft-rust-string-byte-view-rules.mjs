import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-string-byte-view-rules.js',moduleName:'rust_string_byte_view_rules',model:'ordinary Rust string byte views 1',programAsset:'rust-string-byte-view-program'});
