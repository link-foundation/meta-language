import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-array-lookup-rules.js',moduleName:'rust_array_lookup_rules',sourceDependencies:true,model:'Rust immediate immutable scalar array lookup consumers 1',programAsset:'rust-array-lookup-program'});
