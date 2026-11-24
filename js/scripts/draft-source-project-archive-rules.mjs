import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/source-project-archive-rules.js',moduleName:'source_project_archive_rules',directory:'',sourceDependencies:true,model:'complete source project concrete graphs 1',programAsset:'source-project-archive-program'});
