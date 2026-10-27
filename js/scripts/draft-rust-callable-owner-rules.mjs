import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-callable-owner-rules.js',moduleName:'rust_callable_owner_rules',sourceDependencies:true,model:'Rust retained recursive callable source owner contracts 1',programAsset:'rust-callable-owner-program'});
