// Deferred author recipe only. Do not invoke before the full source draft.
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-generic-type-alias-rules.js',moduleName:'rust_generic_type_alias_rules',sourceDependencies:true,model:'explicit ordinary Rust generic alias source templates 1',programAsset:'rust-generic-type-alias-program'});
