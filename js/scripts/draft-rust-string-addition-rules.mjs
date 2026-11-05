import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-string-addition-rules.js',moduleName:'rust_string_addition_rules',model:'standard Rust owned String addition 1',programAsset:'rust-string-addition-program'});
