import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-byte-ascii-rules.js',moduleName:'rust_byte_ascii_rules',model:'Rust standard byte ASCII predicate methods 1',programAsset:'rust-byte-ascii-program'});
