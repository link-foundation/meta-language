import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source generation without behavioral execution');
draftSourceLibrary({ sourcePath: 'js/src/syntax-projection-rules.js', moduleName: 'syntax_projection_rules',
  directory: '', model: 'concrete syntax projection with linked origins 1' });
