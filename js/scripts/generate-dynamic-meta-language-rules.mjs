import { fileURLToPath } from 'node:url';
import { draftSourceLibrary } from './draft-source-library.mjs';
export function draftDynamicMetaLanguageRules() {
  return draftSourceLibrary({ sourcePath: 'js/src/dynamic-meta-language-rules.js',
    moduleName: 'dynamic_meta_language_rules', directory: '', model: 'immutable linked language contexts 1' });
}
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.length !== 3 || process.argv[2] !== '--draft') throw new TypeError('usage: generate-dynamic-meta-language-rules.mjs --draft');
  draftDynamicMetaLanguageRules();
}
