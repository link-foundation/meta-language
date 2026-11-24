import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-primitive-associated-call-rules.js',moduleName:'rust_primitive_associated_call_rules',sourceDependencies:true,model:'Rust exact primitive associated source calls 1',programAsset:'rust-primitive-associated-call-program'});
