import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source-only authoring');
draftSourceLibrary({ sourcePath:'js/src/translation/javascript-record-parameter-rules.js', moduleName:'javascript_record_parameter_rules',
  sourceDependencies:true, model:'ordinary source and closed historical record parameter bound names 1', programAsset:'javascript-record-parameter-program' });
