import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-wrapping-shift-rules.js',moduleName:'rust_wrapping_shift_rules',sourceDependencies:true,model:'Rust exact wrapping shift and rotation source plans 1',programAsset:'rust-wrapping-shift-program'});
