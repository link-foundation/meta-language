import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-primitive-selection-rules.js',moduleName:'rust_primitive_selection_rules',sourceDependencies:true,model:'Rust exact primitive minimum and maximum source selection 1',programAsset:'rust-primitive-selection-program'});
