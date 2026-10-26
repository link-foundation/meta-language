import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-scalar-source-interface-rules.js',moduleName:'rust_scalar_source_interface_rules',model:'retained scalar Rust function interface 1',programAsset:'rust-scalar-source-interface-program'});
