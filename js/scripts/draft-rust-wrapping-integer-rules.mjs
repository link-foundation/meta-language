import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-wrapping-integer-rules.js',moduleName:'rust_wrapping_integer_rules',sourceDependencies:true,model:'Rust exact wrapping integer source arithmetic 1',programAsset:'rust-wrapping-integer-program'});
