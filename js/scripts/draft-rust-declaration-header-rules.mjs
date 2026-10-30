import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-declaration-header-rules.js',moduleName:'rust_declaration_header_rules',sourceDependencies:true,model:'ordinary Rust complete declaration header discovery 1',programAsset:'rust-declaration-header-program'});
