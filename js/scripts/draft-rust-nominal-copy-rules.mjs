import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-nominal-copy-rules.js',moduleName:'rust_nominal_copy_rules',sourceDependencies:true,model:'actual nominal Copy declaration field graph 1',programAsset:'rust-nominal-copy-program'});
