import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source generation without behavioral execution');
draftSourceLibrary({ sourcePath: 'js/src/resource-description-address-rules.js', directory: '',
  moduleName: 'resource_description_address_rules', model: 'RFC 3987 IP literal authority 1',
  // Preserve generated source after rustfmt's internal whitespace failure.
  // Existing CI formatting gates remain unchanged.
  sourceFormatting: 'preserve',
  programAsset: 'resource-description-address-program' });
