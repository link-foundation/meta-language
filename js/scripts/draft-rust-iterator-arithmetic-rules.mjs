import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-iterator-arithmetic-rules.js',moduleName:'rust_iterator_arithmetic_rules',model:'standard Rust numeric iterator arithmetic 1',programAsset:'rust-iterator-arithmetic-program'});
