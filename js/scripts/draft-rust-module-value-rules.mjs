import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-module-value-rules.js',moduleName:'rust_module_value_rules',sourceDependencies:true,model:'ordinary Rust source modules with actual public constant values 1',programAsset:'rust-module-value-program'});
