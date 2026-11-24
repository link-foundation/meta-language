import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/rust-project-source-rules.js',moduleName:'rust_project_source_rules',sourceDependencies:true,directory:'',model:'complete Rust source module composition 1',programAsset:'rust-project-source-program'});
