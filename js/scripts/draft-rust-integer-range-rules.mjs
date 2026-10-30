import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-integer-range-rules.js',moduleName:'rust_integer_range_rules',sourceDependencies:true,model:'exact integer range pattern domain partition 1',programAsset:'rust-integer-range-program'});
