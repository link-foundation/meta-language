import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/source-version-column-rules.js',moduleName:'source_version_column_rules',model:'canonical unsigned decimal source version columns 1',programAsset:'source-version-column-program'});
