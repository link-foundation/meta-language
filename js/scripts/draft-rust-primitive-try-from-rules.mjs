import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-primitive-try-from-rules.js',moduleName:'rust_primitive_try_from_rules',sourceDependencies:true,model:'Rust exact primitive fallible result source elimination 1',programAsset:'rust-primitive-try-from-program'});
