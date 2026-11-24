import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/project-translation-archive-rules.js',moduleName:'project_translation_archive_rules',directory:'',model:'complete project translation source archives 1',programAsset:'project-translation-archive-program'});
