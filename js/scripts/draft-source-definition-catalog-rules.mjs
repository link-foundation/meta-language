import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/source-definition-catalog-rules.js',moduleName:'source_definition_catalog_rules',directory:'',model:'complete qualified typed source programs 1',programAsset:'source-definition-catalog-program'});
