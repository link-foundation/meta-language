#!/usr/bin/env node
// Derive compiler style decorators from machine-applicable Clippy feedback.
// Only temporary copies are edited; the generated Rust remains a translation
// of JavaScript plus reviewed, reproducible Links Notation decorator data.
import { spawnSync } from 'node:child_process';
import { mkdtempSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { DecoratorSet } from '../src/decorators.js';
import { applyRecordedCompilerFeedback, readCompilerFeedbackCorrections, retainCompilerFeedback } from './compiler-style-feedback.mjs';
import { readProjectData } from './project-data.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const paths = ['frontend_rules.rs', ...readdirSync(join(root, 'rust/src/translation/frontend_rules'))
  .filter((name) => name.endsWith('.rs')).sort().map((name) => `frontend_rules/${name}`)];
const feedbackIndex = process.argv.indexOf('--feedback');
if (feedbackIndex !== -1) {
  if (!process.argv[feedbackIndex + 1]) throw new Error('--feedback needs a raw compiler feedback file');
  applyRecordedCompilerFeedback({ root, feedbackFile: process.argv[feedbackIndex + 1],
    decoratorFile: 'parity/self-translation/frontend-rules-decorators.lino', allowedPaths: paths.map((path) => `rust/src/translation/${path}`) });
  process.exit(0);
}
if (process.env.CI !== 'true' || process.env.GITHUB_ACTIONS !== 'true') throw new Error('native compiler style execution runs only in GitHub CI; use --feedback for offline replay');
const directory = mkdtempSync(join(tmpdir(), 'frontend-rule-styles-'));
const actions = [];
try {
  mkdirSync(join(directory, 'translation/frontend_rules'), { recursive: true });
  for (const path of paths) writeFileSync(join(directory, 'translation', path), readFileSync(join(root, 'rust/src/translation', path)));
  writeFileSync(join(directory, 'translation/mod.rs'), 'pub(crate) mod frontend_rules;\n');
  writeFileSync(join(directory, 'check.rs'), 'pub mod translation;\n');
  for (let round = 0; round < 20; round += 1) {
    const result = spawnSync('clippy-driver', [
      '--edition', '2024', '--crate-type', 'lib', '--emit', 'metadata', '--error-format=json',
      '-D', 'warnings', '-W', 'clippy::all', '-W', 'clippy::pedantic', '-W', 'clippy::nursery',
      '-A', 'clippy::module_name_repetitions', '-A', 'clippy::too_many_lines',
      '-A', 'clippy::missing_errors_doc', '-A', 'clippy::missing_panics_doc',
      join(directory, 'check.rs'), '-o', join(directory, 'check.rmeta'),
    ], { encoding: 'utf8', maxBuffer: 16 * 1024 * 1024, timeout: 60000 });
    if (result.error) throw result.error;
    const diagnostics = retainCompilerFeedback({ root, group: 'frontend-rules', round, stderr: result.stderr,
      inputs: paths.map((path) => ({ path: join(directory, 'translation', path), relativePath: `rust/src/translation/${path}` })) });
    if (result.status === 0) {
      if (actions.length === 0) {
        console.log('generated frontend rules pass Clippy without new style replacements');
        break;
      }
      const data = new DecoratorSet([{
        id: 'compiler-feedback-style', level: 'emitter', order: 1000,
        when: [['format', 'Rust'], ['scope', 'formatted-module']], actions,
      }]).toLino();
      const path = join(root, 'parity/self-translation/frontend-rules-decorators.lino');
      const existing = readFileSync(path, 'utf8');
      // Keep earlier feedback: its literal replacements precede these ones.
      const previous = DecoratorSet.fromLino(existing);
      const feedback = previous.decorators.find(({ id }) => id === 'compiler-feedback-style');
      const combined = new DecoratorSet([{
        ...DecoratorSet.fromLino(data).decorators[0],
        actions: [...(feedback?.actions ?? []), ...actions],
      }]).toLino();
      const lines = existing.split('\n').filter((line) => !line.startsWith('(decorator compiler-feedback-style '));
      writeFileSync(path, `${lines.join('\n').trimEnd()}\n\n${combined}`);
      console.log(`recorded ${actions.length} compiler style replacements`);
      break;
    }
    if (process.argv.includes('--check')) {
      throw new Error(diagnostics.filter(({ level }) => level === 'error').map(({ rendered, message }) => rendered ?? message).join('\n'));
    }
    const feedback = readProjectData(join(root,
      'issue-195-artifacts/compiler-feedback/frontend-rules', `round-${round}.lino`));
    const corrections = readCompilerFeedbackCorrections({ feedback,
      allowedPaths: paths.map(path => `rust/src/translation/${path}`) });
    for (const [path, source] of corrections.inputs) {
      if (!readFileSync(path).equals(source)) throw new Error('the online compiler feedback source changed before replay');
    }
    for (const [path, source] of corrections.outputs) writeFileSync(path, source);
    actions.push(...corrections.actions);
    if (!corrections.actions.length || round === 19) {
      throw new Error(`Clippy needs a reviewed style resolution:\n${diagnostics.filter(({ level }) => level === 'error').map(({ message, spans }) => `${message}: ${(spans ?? []).filter(({ is_primary }) => is_primary).map(({ text }) => text.map(({ text }) => text).join(' ')).join('; ')}`).join('\n')}`);
    }
  }
} finally {
  rmSync(directory, { recursive: true, force: true });
}
