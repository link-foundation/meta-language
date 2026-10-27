import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-integer-magnitude-rules.js',moduleName:'rust_integer_magnitude_rules',sourceDependencies:true,model:'Rust exact unsigned integer magnitude source plans 1',programAsset:'rust-integer-magnitude-program'});
