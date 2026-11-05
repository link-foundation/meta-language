import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-saturating-integer-rules.js',moduleName:'rust_saturating_integer_rules',sourceDependencies:true,model:'Rust exact saturating integer source arithmetic 1',programAsset:'rust-saturating-integer-program'});
