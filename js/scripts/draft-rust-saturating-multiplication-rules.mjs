import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-saturating-multiplication-rules.js',moduleName:'rust_saturating_multiplication_rules',sourceDependencies:true,model:'Rust exact saturating multiplication source plan 1',programAsset:'rust-saturating-multiplication-program'});
