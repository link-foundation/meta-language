import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-panic-value-rules.js',moduleName:'rust_panic_value_rules',model:'Rust evaluated panic message values 1',programAsset:'rust-panic-value-program'});
