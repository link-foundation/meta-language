import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-class-source-rules.js',moduleName:'javascript_class_source_rules',
  model:'closed JavaScript ordinary classes shared reference source lowering with public instance fields 4',programAsset:'javascript-class-source-program',valueSharing:'exact scalars, types and source spans 1'});
