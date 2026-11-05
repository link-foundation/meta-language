import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-pointer-integer-owner-rules.js',moduleName:'rust_pointer_integer_owner_rules',model:'distinct Rust pointer integer source owners 1',programAsset:'rust-pointer-integer-owner-program'});
