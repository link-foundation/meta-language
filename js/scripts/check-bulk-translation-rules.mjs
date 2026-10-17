#!/usr/bin/env node
// Check the new generated families serially. Existing JavaScript workflow
// freshness gates continue to own the previously registered generators.
import { spawn } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repository = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

export const BULK_TRANSLATION_RULE_CHECKS = Object.freeze([
  ['generate-array-value-rules.mjs'],
  ['generate-callable-source-rules.mjs'],
  ['generate-optional-rules.mjs'],
  ['generate-javascript-surface-rules.mjs'],
  ['generate-primitive-method-rules.mjs'],
  ['generate-surface-raw-metadata-rules.mjs'],
  ['generate-surface-reference-analysis-rules.mjs'],
  ['generate-record-value-rules.mjs', '--native'],
  ['generate-record-emission-rules.mjs', '--native'],
  ['generate-record-type-syntax-rules.mjs', '--native'],
  ['generate-type-syntax-rules.mjs', '--native'],
  ['generate-grammar-lexer-rules.mjs', '--native'],
  ['generate-grammar-selection-rules.mjs', '--native'],
  ['generate-grammar-selection-fixtures.mjs'],
  ['generate-private-output-test-model.mjs'],
  ['extend-typed-program-schema-second-stage.mjs'],
  ['generate-owned-array-append-rules.mjs', '--native'],
  ['generate-nullable-type-syntax-rules.mjs', '--native'],
  ['generate-callable-emission-rules.mjs', '--native'],
  ['generate-string-code-unit-template.mjs'],
  // The live fourth gate authenticates every immutable third/predecessor byte.
  // The frozen third CLI's strict boundary remains tested against its archive.
  ['extend-typed-program-schema-fourth-stage.mjs'],
  ['generate-grammar-selection-mapping-rules.mjs', '--native'],
  ['generate-grammar-selection-mapping-fixtures.mjs'],
  ['generate-rust-character-rules.mjs', '--native'],
  ['generate-javascript-tag-context-rules.mjs', '--native'],
  ['generate-rust-character-literal-fixtures.mjs'],
  ['generate-rust-diagnostic-attribute-rules.mjs', '--native'],
].map(([name, ...arguments_]) => Object.freeze({ script: `js/scripts/${name}`, arguments: Object.freeze([...arguments_, '--check']) })));

function invokeTranslationRuleCheck(command, arguments_, options) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, arguments_, { ...options, stdio: 'inherit' });
    child.once('error', reject);
    child.once('close', (exitCode, signal) => resolve({ exitCode, signal }));
  });
}

/** No compiler, update mode, subprocess parallelism or early failure exit. */
export async function runBulkTranslationRuleChecks({ root = repository, checks = BULK_TRANSLATION_RULE_CHECKS,
  invoke = invokeTranslationRuleCheck, report = message => console.error(message), environment = process.env } = {}) {
  const failures = [];
  for (const check of checks) {
    const command = `node ${check.script} ${check.arguments.join(' ')}`;
    report(`Translation rule freshness: ${command}`);
    try {
      if (!/^js\/scripts\/(?:generate|extend)-[a-z0-9-]+\.mjs$/u.test(check.script)
        || check.arguments.filter(value => value === '--check').length !== 1
        || check.arguments.some(value => !['--check', '--native'].includes(value))) {
        throw new Error('freshness checks require an explicit check-only source generator');
      }
      const actual = await invoke(process.execPath, [path.join(root, check.script), ...check.arguments], { cwd: root, env: environment });
      if (actual.exitCode !== 0 || actual.signal !== null) throw new Error(`exit ${actual.exitCode}, signal ${actual.signal ?? 'none'}`);
    } catch (error) {
      const failure = { script: check.script, command, error: error.message };
      failures.push(failure);
      report(`Translation rule freshness FAILED: ${command}: ${failure.error}`);
    }
  }
  report(`Translation rule freshness: ${checks.length} checks, ${failures.length} failures`);
  return { checks: checks.length, failures };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  try {
    if (process.argv.length !== 2) throw new Error('the complete translation rule freshness gate accepts no selection or update arguments');
    const result = await runBulkTranslationRuleChecks();
    process.exitCode = result.failures.length ? 1 : 0;
  } catch (error) { console.error(error.message); process.exitCode = 1; }
}
