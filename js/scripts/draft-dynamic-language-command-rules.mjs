import {draftSourceLibrary} from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/dynamic-language-command-rules.js', moduleName:'dynamic_language_command_rules', directory:'',
  model:'complete dynamic language command vocabulary and source option admission 1', programAsset:'dynamic-language-command-program'});
