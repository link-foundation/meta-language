import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import { emitDecimalDraftRust } from '../../experiments/issue-199-check-decimal-draft.mjs';
import { qualifyGeneratedRustRootPaths, wrapGeneratedRustNamespace } from '../../experiments/issue-199-generated-rust-namespace.mjs';
import { retainCompilerFeedback } from '../scripts/compiler-style-feedback.mjs';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';

test('generated namespace adaptation preserves opaque literals and refuses external dependencies', () => {
  const source = 'pub mod ml { pub const VALUE:i32=3; }\n// crate::ml::VALUE\npub fn value()->i32 { let text="crate::ml::VALUE"; crate::ml::VALUE }\n';
  const adapted = qualifyGeneratedRustRootPaths(source, 'ml_example');
  assert.equal(adapted, source.replace('"; crate::ml::VALUE', '"; crate::ml_example::ml::VALUE'));
  assert.throws(() => qualifyGeneratedRustRootPaths('pub fn value(){crate::external::call();}', 'ml_example'), /external crate-root dependency/u);
  for (const namespace of ['self', 'crate', 'ml_x;evil', 'ml_']) assert.throws(() => wrapGeneratedRustNamespace(source, namespace), /reserved ml_ identifier/u);
  assert.throws(() => qualifyGeneratedRustRootPaths(null, 'ml_example'), /source text/u);
  assert.throws(() => qualifyGeneratedRustRootPaths('pub fn value(){', 'ml_example'), /unclosed module scope/u);
  assert.throws(() => qualifyGeneratedRustRootPaths('}', 'ml_example'), /unbalanced module scope/u);
  for (const literal of ['r"crate::ml::VALUE"', 'r#"crate::ml::VALUE"#', 'br#"crate::ml::VALUE"#']) {
    assert.throws(() => qualifyGeneratedRustRootPaths(`pub mod ml {} pub const TEXT: &str = ${literal};`, 'ml_example'), /raw string literals/u);
  }
  const declarations = '/* crate::A /* crate::C */ */\npub type A = i32; pub const C: A = 1; pub static mut S: A = 2; pub const fn value()->A { crate::C }\npub fn sample()->A { let text="😀 crate::C"; crate::value() + crate::C }';
  const qualified = qualifyGeneratedRustRootPaths(declarations, 'ml_example');
  assert.equal(qualified, declarations.replace('{ crate::C }', '{ crate::ml_example::C }')
    .replace('crate::value() + crate::C', 'crate::ml_example::value() + crate::ml_example::C'));
});

test('translation CI preserves compiler feedback after failures and before cleanup', () => {
  const jobs = parseWorkflow(readFileSync(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8'));
  const steps = jobs.find(job => job.id === 'tests').steps;
  const upload = steps.findIndex(step => step.name === 'Retain translation compiler feedback and native observations');
  assert.ok(upload > steps.findIndex(step => step.name === 'Run the test group'));
  assert.ok(upload < steps.findIndex(step => step.name === 'Clean regenerable caches'));
  assert.ok(steps[upload].afterFailure, 'a failing native compilation must still retain its structured diagnostics');
  assert.match(steps[upload].text, /matrix\.group == 'translation'/u);
  assert.match(steps[upload].text, /translation-test-feedback-\$\{\{ github\.sha \}\}/u);
  assert.match(steps[upload].text, /issue-195-artifacts\/compiler-feedback/u);
  assert.match(steps[upload].text, /issue-195-results\/artifacts\/decimal-format-draft-native\.lino/u);
  assert.match(steps[upload].text, /if-no-files-found: error/u);
});

test('the closed decimal generator can be embedded without crate-root re-exports', () => {
  const emitted = emitDecimalDraftRust();
  const adapted = wrapGeneratedRustNamespace(emitted.text);
  assert.ok(adapted.startsWith('pub mod ml_decimal_draft {\n'));
  assert.ok(adapted.includes('crate::ml_decimal_draft::ml::Big'));
  assert.ok(adapted.includes('crate::ml_decimal_draft::render_shortest_decimal_value'));
  assert.ok(adapted.includes('fn main() {'), 'the generated entry point remains private to its module');
  const body = adapted.slice('pub mod ml_decimal_draft {\n'.length, -3);
  assert.equal(body.replaceAll('crate::ml_decimal_draft::', 'crate::'), emitted.text,
    'removing the declared namespace qualification reproduces every generated algorithm byte');
});

test('CI compiles and executes generated decimal helpers inside an isolated namespace', {
  skip: process.env.TRANSLATION_CHECK_RUST !== '1' ? 'native namespace verification runs only in CI' : false,
}, () => {
  assert.equal(process.env.CI, 'true', 'never compile Rust locally');
  const directory = mkdtempSync(join(tmpdir(), 'meta-decimal-namespace-'));
  const source = join(directory, 'namespace.rs');
  const executable = join(directory, 'namespace');
  const module = wrapGeneratedRustNamespace(emitDecimalDraftRust().text);
  const host = `fn main() {
    use ml_decimal_draft::{ml::Big, render_finite_binary_double_precision, render_binary_double_precision_special};
    println!("{}", render_finite_binary_double_precision(Big::from_u128(4503599627370496), Big::from_i128(-52), false, "ecmascript-number-text".to_owned()));
    println!("{}", render_finite_binary_double_precision(Big::from_u128(0), Big::from_i128(-1074), true, "rust-default-binary-double-precision-text".to_owned()));
    println!("{}", render_binary_double_precision_special("rust-default-binary-double-precision-text".to_owned(), "infinity".to_owned(), false));
    println!("{}", render_binary_double_precision_special("node-console-number-text".to_owned(), "infinity".to_owned(), true));
}\n`;
  try {
    writeFileSync(source, module + host);
    const compiled = spawnSync('rustc', ['--edition', '2024', '-O', '-D', 'warnings', '--error-format=json', source, '-o', executable],
      { encoding: 'utf8', timeout: 120_000, maxBuffer: 8 * 1024 * 1024 });
    retainCompilerFeedback({ root: fileURLToPath(new URL('../../', import.meta.url)), group: 'decimal-namespace-draft',
      compiler: 'rustc', round: 0, stderr: compiled.stderr ?? '', inputs: [{ path: source, relativePath: 'generated/decimal-namespace-draft.rs' }] });
    assert.equal(compiled.status, 0, compiled.error?.message ?? compiled.stderr);
    const executed = spawnSync(executable, [], { encoding: 'utf8', timeout: 120_000 });
    assert.equal(executed.status, 0, executed.error?.message ?? executed.stderr);
    assert.deepEqual(executed.stdout.trimEnd().split('\n'), ['1', '-0', 'inf', '-Infinity']);
  } finally { rmSync(directory, { recursive: true, force: true }); }
});
