import{draftSourceLibrary}from'./draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft for source-only authoring');
draftSourceLibrary({sourcePath:'js/src/translation/rust-record-rules.js',moduleName:'rust_record_rules',
 model:'owned nominal Rust record field order 1',programAsset:'rust-record-program'});
