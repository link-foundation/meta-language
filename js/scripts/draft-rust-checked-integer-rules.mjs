import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-checked-integer-rules.js',moduleName:'rust_checked_integer_rules',sourceDependencies:true,model:'Rust guarded checked integer immediate Option source elimination 1',programAsset:'rust-checked-integer-program'});
