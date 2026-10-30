import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-source-constant-evaluation-rules.js',moduleName:'rust_source_constant_evaluation_rules',sourceDependencies:true,model:'bounded actual Rust source constant scalar evaluation 1',programAsset:'rust-source-constant-evaluation-program'});
