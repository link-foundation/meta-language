import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-source-version-rules.js',moduleName:'rust_source_version_rules',model:'explicit stable Rust source std versions 1',programAsset:'rust-source-version-program'});
