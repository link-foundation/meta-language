import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-array-element-owner-rules.js',moduleName:'rust_array_element_owner_rules',model:'actual Rust array literal element source owners 1',programAsset:'rust-array-element-owner-program'});
