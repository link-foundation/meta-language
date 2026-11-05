import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-primitive-owner-path-rules.js',moduleName:'rust_primitive_owner_path_rules',sourceDependencies:true,model:'Rust exact shared primitive owner path source domain 1',programAsset:'rust-primitive-owner-path-program'});
