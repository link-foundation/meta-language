import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-unit-value-rules.js',moduleName:'rust_unit_value_rules',sourceDependencies:true,
 model:'Rust implicit Unit and ordered direct Unit call statements 1',programAsset:'rust-unit-value-program'});
