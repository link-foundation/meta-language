import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync, writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {checkProjectData, writeProjectData} from '../../js/scripts/project-data.mjs';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {originalLinkSymbolPolicy, originalLinkSymbolRulesSource} from './policy.mjs';

export function generateOriginalLinkSymbolProjection(check = false) {
 const source = originalLinkSymbolRulesSource();
 const program = checkProgram(parseJavaScript(source, originalLinkSymbolPolicy.translation));
 const emitted = emitRust(program);
 const artifact = {version: 1, model: originalLinkSymbolPolicy.model,
  sourceSha256: createHash('sha256').update(source).digest('hex'),
  declarationNames: program.items.map(item => item.fullName),
  declarations: emitted.definitions, completeRust: emitted.text};
 for (const [name, value] of [['policy', originalLinkSymbolPolicy], ['native-definitions', artifact]]) {
  const url = new URL('./data/' + name + '.lino', import.meta.url);
  if (check) checkProjectData(url, value); else writeProjectData(url, value);
 }
 const url = new URL('./rules.mjs', import.meta.url);
 if (check) assert.equal(readFileSync(url, 'utf8'), source); else writeFileSync(url, source);
 return artifact;
}
if (process.argv[1] === fileURLToPath(import.meta.url)) {
 const artifact = generateOriginalLinkSymbolProjection(process.argv.includes('--check'));
 console.log('Original String symbol projection: ' + artifact.declarationNames.length + ' complete automatically emitted definitions');
}
