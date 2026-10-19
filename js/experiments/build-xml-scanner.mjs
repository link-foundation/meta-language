// Generate the XML/DTD scanner common to the pinned oracle sources.
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
export const XML_NAME_PATTERN = '[\\p{L}_:][\\p{L}\\p{N}_:.\\u00b7-]*';
export const DTD_SCANNERS = [
  { family: 'pattern-token', name: 'processing_instruction_target', token: 'processing_instruction_target', pattern: XML_NAME_PATTERN,
    excludedTexts: ['xml','xmL','xMl','xML','Xml','XmL','XMl','XML'] },
  { family: 'content', name: 'processing_instruction_content', token: 'processing_instruction_content', closing: '?>', stops: ['\n','?'], allowEnd: false, allowEmpty: true, closingContext: '\\?> *\\n' },
  { family: 'delimited', name: 'comment', token: 'comment', opening: '<!--', closing: '--', closingSuffix: '>' },
];
export const XML_SCANNERS = [...DTD_SCANNERS,
  { family: 'paired-name', name: 'element_tag', startToken: 'start_tag_name', endToken: 'end_tag_name', emptyToken: 'self_closing_tag_end', namePattern: XML_NAME_PATTERN, emptyClosing: '/>' },
  { family: 'content', name: 'character_data', token: 'character_data', closing: '<', stops: ['&'], allowEnd: true, rejected: [']]>' ], allowedOpening: [']]>' ] },
  { family: 'content', name: 'character_data_section', token: 'character_data_section', closing: ']]>', allowEnd: false },
  { family: 'pattern-token', name: 'erroneous_end_name', token: 'erroneous_end_name', pattern: '[^\\s\\S]' },
];

if (process.argv[1] && fileURLToPath(import.meta.url) === resolve(process.argv[1])) process.stdout.write(scannerFamilies(process.argv.includes('--xml') ? XML_SCANNERS : DTD_SCANNERS));
