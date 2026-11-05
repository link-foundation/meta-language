import { fileURLToPath } from 'node:url';
import { draftSourceLibrary } from './draft-source-library.mjs';

export function draftMetaLanguageArchiveRules() {
  return draftSourceLibrary({ sourcePath: 'js/src/meta-language-archive-rules.js',
    moduleName: 'meta_language_archive_rules', directory: '', model: 'complete meta-language graph archive 1' });
}
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.length !== 3 || process.argv[2] !== '--draft') throw new TypeError('usage: generate-meta-language-archive-rules.mjs --draft');
  draftMetaLanguageArchiveRules();
}
