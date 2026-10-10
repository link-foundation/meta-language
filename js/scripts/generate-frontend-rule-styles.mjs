#!/usr/bin/env node
// Derive compiler style decorators from machine-applicable Clippy feedback.
// Only temporary copies are edited; the generated Rust remains a translation
// of JavaScript plus reviewed, reproducible Links Notation decorator data.
import { spawnSync } from 'node:child_process';
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { DecoratorSet } from '../src/decorators.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const directory = mkdtempSync(join(tmpdir(), 'frontend-rule-styles-'));
const paths = ['frontend_rules.rs', 'frontend_rules/continuations.rs'];
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
    const diagnostics = result.stderr.trim().split('\n').map((line) => JSON.parse(line));
    if (process.argv.includes('--check')) {
      throw new Error(diagnostics.filter(({ level }) => level === 'error').map(({ rendered, message }) => rendered ?? message).join('\n'));
    }
    const edits = new Map();
    const visit = (diagnostic) => {
      const spans = (diagnostic.spans ?? []).filter(({ suggested_replacement: replacement }) => replacement !== null);
      if (spans.length > 0 && spans.every((span) => span.suggestion_applicability === 'MachineApplicable' && paths.some((path) => span.file_name === join(directory, 'translation', path)))) {
        for (const span of spans) {
          const list = edits.get(span.file_name) ?? [];
          list.push(span);
          edits.set(span.file_name, list);
        }
      }
      for (const child of diagnostic.children ?? []) visit(child);
    };
    diagnostics.forEach(visit);
    let changed = false;
    for (const [path, spans] of edits) {
      let source = readFileSync(path);
      let boundary = source.length;
      // Apply non-overlapping edits in descending order; alternatives are
      // re-evaluated by the next compiler round rather than corrupting offsets.
      for (const span of spans.sort((a, b) => b.byte_start - a.byte_start || b.byte_end - a.byte_end)) {
        if (span.byte_end > boundary) continue;
        const before = source;
        const replacement = Buffer.from(span.suggested_replacement);
        if (before.subarray(span.byte_start, span.byte_end).equals(replacement)) continue;
        source = Buffer.concat([before.subarray(0, span.byte_start), replacement, before.subarray(span.byte_end)]);
        // Literal replacements need line context: replacing an identifier
        // alone could also rewrite its declaration or an enum pattern.
        const start = before.lastIndexOf(10, Math.max(0, span.byte_start - 1)) + 1;
        const next = before.indexOf(10, span.byte_end);
        const end = next < 0 ? before.length : next + 1;
        const from = before.subarray(start, end).toString('utf8');
        const length = replacement.length - (span.byte_end - span.byte_start);
        const to = source.subarray(start, end + length).toString('utf8').replace(/[ \t]+$/gmu, '');
        actions.push({ op: 'replace', field: 'source', from, to });
        boundary = span.byte_start;
        changed = true;
      }
      writeFileSync(path, source);
    }
    if (!changed || round === 19) {
      throw new Error(`Clippy needs a reviewed style resolution:\n${diagnostics.filter(({ level }) => level === 'error').map(({ message, spans }) => `${message}: ${(spans ?? []).filter(({ is_primary }) => is_primary).map(({ text }) => text.map(({ text }) => text).join(' ')).join('; ')}`).join('\n')}`);
    }
  }
} finally {
  rmSync(directory, { recursive: true, force: true });
}
