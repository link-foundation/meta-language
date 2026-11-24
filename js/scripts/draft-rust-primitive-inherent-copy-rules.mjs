import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-primitive-inherent-copy-rules.js',moduleName:'rust_primitive_inherent_copy_rules',sourceDependencies:true,model:'Rust immutable primitive inherent receiver source adjustment 1',programAsset:'rust-primitive-inherent-copy-program'});
