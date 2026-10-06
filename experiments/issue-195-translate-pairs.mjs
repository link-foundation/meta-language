// Translates the corpus through all 12 directed pairs with the public JS API, writes each target
// to OUT (default /tmp/pairs), checks that it reparses to a clean CST, and runs the native toolchain.
//   node experiments/issue-195-translate-pairs.mjs [OUT] [--no-native]
import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { LinkNetwork, translateProgram } from '../js/src/index.js';

const args = process.argv.slice(2);
const out = args.find((arg) => !arg.startsWith('--')) ?? '/tmp/pairs';
const native = !args.includes('--no-native');
const corpus = new URL('../parity/fixtures/translation-corpus/', import.meta.url);
const languages = {
  JavaScript: { file: 'project.mjs', ext: 'mjs', grammar: 'javascript' },
  Rust: { file: 'project.rs', ext: 'rs', grammar: 'rust' },
  Lean: { file: 'project.lean', ext: 'lean', grammar: 'lean' },
  Rocq: { file: 'project.v', ext: 'v', grammar: 'rocq' },
};
const expected = readFileSync(new URL('expected-output.txt', corpus), 'utf8');
mkdirSync(out, { recursive: true });

const run = (command, argv, cwd) => {
  try {
    const stdout = execFileSync(command, argv, { cwd, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
    return { ok: true, stdout, stderr: '' };
  } catch (error) {
    return { ok: false, stdout: error.stdout ?? '', stderr: error.stderr ?? error.message };
  }
};

function check(target, path, base) {
  switch (target) {
    case 'JavaScript': {
      const result = run('node', [path]);
      return { ...result, same: result.stdout === expected };
    }
    case 'Rust': {
      const binary = join(out, base);
      const build = run('rustc', ['--edition', '2024', '-O', '-o', binary, path]);
      if (!build.ok) return build;
      const result = run(binary, []);
      return { ...result, same: result.stdout === expected, stderr: build.stderr + result.stderr };
    }
    case 'Lean': {
      const result = run('lean', ['--run', path]);
      return { ...result, same: result.stdout === expected, warnings: /warning|error/.test(result.stdout + result.stderr) };
    }
    case 'Rocq': {
      const result = run('rocq', ['compile', '-q', `${base}.v`], out);
      return { ...result, same: null, warnings: /Warning|Error/.test(result.stderr) };
    }
    default:
      throw new Error(target);
  }
}

for (const [source, from] of Object.entries(languages)) {
  const text = readFileSync(new URL(from.file, corpus), 'utf8');
  for (const [target, to] of Object.entries(languages)) {
    if (target === source) continue;
    const base = `${source.toLowerCase()}_to_${target.toLowerCase()}`;
    const path = join(out, `${base}.${to.ext}`);
    const translation = translateProgram(text, source, target);
    writeFileSync(path, translation.code);
    const network = LinkNetwork.parse(translation.code, to.grammar);
    const clean = network.verifyFullMatch().isClean();
    const verdict = native ? check(target, path, base) : {};
    console.log([
      `${source} -> ${target}`,
      translation.contract.support,
      clean ? 'cst:clean' : 'cst:ERROR',
      native ? `native:${verdict.ok ? 'ok' : 'FAIL'}` : '',
      verdict.same === undefined || verdict.same === null ? '' : `output:${verdict.same ? 'same' : 'DIFF'}`,
      verdict.warnings ? 'WARNINGS' : '',
    ].filter(Boolean).join(' '));
    if (native && (!verdict.ok || verdict.warnings)) console.log(`  ${(verdict.stderr + verdict.stdout).slice(0, 1500).replace(/\n/g, '\n  ')}`);
  }
}
