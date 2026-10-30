import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/record-value-order-template.js',moduleName:'object_entry_pair_rules',model:'complete opaque own-entry pairing and ordering source 1',sourceDependencies:true,programAsset:'object-entry-pair-program'});
