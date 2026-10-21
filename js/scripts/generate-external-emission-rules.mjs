// Normal source author. Native decisions are generated only through the core.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';

export async function generateExternalEmissionRules({ check = false, native = false } = {}) {
  const root = new URL('../../', import.meta.url);
  const sourcePath = 'js/src/translation/external-emission-rules.js';
  const source = readFileSync(new URL(sourcePath, root), 'utf8');
  const policy = { schemaVersion: 1, model: 'actual JavaScript external binding emission 1', sourcePath,
    source, sourceSha256: createHash('sha256').update(source).digest('hex'),
    assertionVersions: { Node: '>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0', Rust: '>=1.90.0 <1.100.0' },
    boundary: 'Actual supplied function and constant bindings retain their target names; local/declaration allocation reserves those names. The initial profile admits simple ASCII strict ECMAScript module binding identifiers, including ordinary builtin spellings. Unsupported identifiers need an explicit target-binding alias, never a silent rename. Native generation is not native execution.' };
  for (const path of ['js/src/translation/data/external-emission-rules.lino', 'rust/src/data/external-emission-rules.lino']) {
    if (check) checkProjectData(new URL(path, root), policy); else writeProjectData(new URL(path, root), policy);
  }
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const { writeRecordRuleLibrary } = await import('./record-rule-library.mjs');
    const emitted = emitRust(checkProgram(parseJavaScript(source, { referenceSemantics: false })), { arrayConsumption: true });
    const files = writeRecordRuleLibrary(emitted, 'external_emission_rules', { check });
    const artifact = { schemaVersion: 1, model: policy.model, sourcePath, source,
      sourceSha256: policy.sourceSha256, rustSource: emitted.text, definitions: emitted.definitions,
      mappings: emitted.mappings, assumptions: emitted.assumptions, encodings: emitted.encodings, files,
      boundary: 'Complete automatic source generation and normal formatting only; native compilation and runtime parity require CI.' };
    const output = new URL('parity/self-translation/external-emission-rules-native.lino', root);
    if (check) checkProjectData(output, artifact); else writeProjectData(output, artifact);
    assert.equal(emitted.mappings.filter(row => row.source === 'renderJavaScriptExternalCall').length, 1);
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(args.every(argument => argument === '--check' || argument === '--native'));
  await generateExternalEmissionRules({ check: args.includes('--check'), native: args.includes('--native') });
  console.log('External emission retains checked provider binding names and distinguishes reads from calls');
}
