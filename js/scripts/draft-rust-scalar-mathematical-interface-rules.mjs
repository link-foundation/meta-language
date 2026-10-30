import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-scalar-mathematical-interface-rules.js',moduleName:'rust_scalar_mathematical_interface_rules',model:'retained mathematical Rust function interface 1',programAsset:'rust-scalar-mathematical-interface-program',sourceDependencies:true});
