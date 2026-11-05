import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/owned-array-reader-domain-rules.js',moduleName:'owned_array_reader_domain_rules',model:'typed disjoint primitive array reader results 1',sourceDependencies:true,programAsset:'owned-array-reader-domain-program'});
