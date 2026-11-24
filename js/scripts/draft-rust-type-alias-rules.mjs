import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-type-alias-rules.js',moduleName:'rust_type_alias_rules',sourceDependencies:true,model:'ordinary transparent Rust source type alias definitions 1',programAsset:'rust-type-alias-program'});
