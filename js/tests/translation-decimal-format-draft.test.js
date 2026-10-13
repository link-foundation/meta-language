import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { inspect } from 'node:util';
import { fileURLToPath } from 'node:url';
import vm from 'node:vm';
import test from 'node:test';
import { decimalDraftValues, emitDecimalDraftRust } from '../../experiments/issue-199-check-decimal-draft.mjs';
import { formatBinaryDoublePrecisionValue } from '../../experiments/issue-199-binary-double-precision-format.mjs';
import { writeProjectData } from '../scripts/project-data.mjs';
import { retainCompilerFeedback } from '../scripts/compiler-style-feedback.mjs';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';

const root = fileURLToPath(new URL('../../', import.meta.url));
const hash = value => createHash('sha256').update(value).digest('hex');
const models = ['ecmascript-number-text', 'node-console-number-text', 'rust-default-binary-double-precision-text'];
const inputs = decimalDraftValues();
const view = new DataView(new ArrayBuffer(8));
const bitPattern = value => { view.setFloat64(0, value, false); return view.getBigUint64(0, false).toString(16).padStart(16, '0'); };

function reloadDecimalDraft() {
  const source = readFileSync(new URL('../../experiments/issue-199-decimal-number-rules.mjs', import.meta.url), 'utf8');
  const archive = createTypedProgramLinks(source, 'JavaScript', { context: { referenceSemantics: false } });
  const text = archive.network.toLino();
  assert.ok(Buffer.byteLength(text) < 1024 * 1024, 'this is a bounded algorithm archive');
  return { network: LinkNetwork.fromLino(text), root: archive.root };
}

test('decimal draft agrees with independent JS text oracles and emits its complete algorithm automatically', () => {
  assert.equal(inputs.length, 92);
  for (const value of inputs) {
    assert.equal(formatBinaryDoublePrecisionValue(value, models[0]), String(value), bitPattern(value));
    assert.equal(formatBinaryDoublePrecisionValue(value, models[1]), inspect(value), bitPattern(value));
  }
  const emitted = emitDecimalDraftRust();
  assert.equal(emitted.definitions.length, 22);
  assert.ok(emitted.definitions.some(definition => definition.includes('render_finite_binary_double_precision')));
});

test('decimal draft survives source-free LiNo reload and emits executable JS and Rust', () => {
  const archive = reloadDecimalDraft();
  const program = decodeTypedProgramLinks(archive.network, archive.root);
  assert.equal(program.declarations.size, 22);
  const javascript = emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code;
  const rust = emitTypedProgramLinks(archive.network, archive.root, 'Rust').code;
  assert.equal(rust, emitDecimalDraftRust().text);
  const context = vm.createContext({});
  vm.runInContext(javascript, context, { timeout: 1000 });
  assert.equal(vm.runInContext("renderFiniteBinaryDoublePrecision(4503599627370496n,-52n,false,'ecmascript-number-text')", context, { timeout: 1000 }), '1');
  assert.equal(vm.runInContext("renderFiniteBinaryDoublePrecision(0n,-1074n,true,'rust-default-binary-double-precision-text')", context, { timeout: 1000 }), '-0');
  // Exercise the lowered, reloaded program across the complete existing
  // pattern set, rather than checking only its original source implementation.
  for (const value of inputs) {
    const bits = BigInt('0x' + bitPattern(value));
    const negative = bits >> 63n !== 0n;
    const biasedExponent = Number(bits >> 52n & 0x7ffn);
    const fraction = bits & ((1n << 52n) - 1n);
    for (const model of models) {
      const actual = biasedExponent === 0x7ff
        ? context.renderBinaryDoublePrecisionSpecial(model, fraction === 0n ? 'infinity' : 'not-a-number', negative)
        : context.renderFiniteBinaryDoublePrecision(biasedExponent === 0 ? fraction : fraction + (1n << 52n),
          biasedExponent === 0 ? -1074n : BigInt(biasedExponent - 1075), negative, model);
      const expected = model === models[0] ? String(value) : model === models[1] ? inspect(value) : formatBinaryDoublePrecisionValue(value, model);
      assert.equal(actual, expected, `${bitPattern(value)} ${model}`);
    }
  }
});

test('CI compares generated decimal rules with actual native Rust display and both JS oracles', {
  skip: process.env.TRANSLATION_CHECK_RUST !== '1' ? 'native compilation and formatting run only in CI' : false,
}, () => {
  assert.equal(process.env.CI, 'true', 'never compile Rust locally for this experiment');
  const directory = mkdtempSync(join(tmpdir(), 'meta-decimal-native-'));
  const source = join(directory, 'decimal.rs');
  const executable = join(directory, 'decimal');
  const archive = reloadDecimalDraft();
  const generated = emitTypedProgramLinks(archive.network, archive.root, 'Rust').code;
  const bridge = `
fn format_declared_bits(bits: u64, model: &str) -> String {
    let negative = bits >> 63 != 0;
    let biased_exponent = (bits >> 52) & 0x7ff;
    let fraction = bits & ((1u64 << 52) - 1);
    if biased_exponent == 0x7ff {
        return render_binary_double_precision_special(model.to_owned(), if fraction == 0 { "infinity" } else { "not-a-number" }.to_owned(), negative);
    }
    let mantissa = if biased_exponent == 0 { fraction } else { fraction + (1u64 << 52) };
    let exponent = if biased_exponent == 0 { -1074 } else { i128::from(biased_exponent) - 1075 };
    render_finite_binary_double_precision(ml::Big::from_u128(u128::from(mantissa)), ml::Big::from_i128(exponent), negative, model.to_owned())
}
pub fn run_decimal_oracle() {
    for bits in [${inputs.map(value => '0x' + bitPattern(value) + 'u64').join(', ')}] {
        println!("{:016x}\\t{}\\t{}\\t{}\\t{}", bits, f64::from_bits(bits), format_declared_bits(bits,"${models[0]}"), format_declared_bits(bits,"${models[1]}"), format_declared_bits(bits,"${models[2]}"));
    }
}
`;
  try {
    // Keep the source-free generated program intact in its own Rust module.
    // Its generated entry point and this oracle's host entry point have
    // separate namespaces, while the bit bridge can call private definitions.
    // Generated signatures use absolute crate paths for their shared types
    // and continuations. Re-export their public items at that declared root;
    // the generated private main remains inside decimal_rules.
    const nativeSource = `mod decimal_rules {\n${generated}${bridge}\n}\npub use decimal_rules::*;\nfn main() { decimal_rules::run_decimal_oracle(); }\n`;
    writeFileSync(source, nativeSource);
    const compiler = spawnSync('rustc', ['--version'], { encoding: 'utf8', timeout: 30_000 });
    assert.equal(compiler.status, 0, compiler.stderr);
    const compiled = spawnSync('rustc', ['--edition', '2024', '-O', '-D', 'warnings', '--error-format=json', source, '-o', executable], {
      encoding: 'utf8', timeout: 120_000, maxBuffer: 8 * 1024 * 1024,
    });
    retainCompilerFeedback({ root, group: 'decimal-format-draft', round: 0, stderr: compiled.stderr ?? '',
      inputs: [{ path: source, relativePath: 'generated/decimal-format-draft.rs' }] });
    assert.equal(compiled.status, 0, compiled.error?.message ?? compiled.stderr);
    const executed = spawnSync(executable, [], { encoding: 'utf8', timeout: 120_000, maxBuffer: 1024 * 1024 });
    assert.equal(executed.status, 0, executed.error?.message ?? executed.stderr);
    const rows = executed.stdout.trimEnd().split('\n').map(line => line.split('\t'));
    assert.equal(rows.length, inputs.length);
    rows.forEach(([bits, rustDisplay, jsText, consoleText, rustText], index) => {
      const value = inputs[index];
      assert.equal(bits, bitPattern(value));
      assert.equal(jsText, String(value), bits);
      assert.equal(consoleText, inspect(value), bits);
      assert.equal(rustText, rustDisplay, bits);
      assert.equal(rustText, formatBinaryDoublePrecisionValue(value, models[2]), bits);
    });
    writeProjectData(join(root, 'issue-195-results/artifacts/decimal-format-draft-native.lino'), {
      schemaVersion: 1, compiler: compiler.stdout.trim(), algorithmSha256: hash(readFileSync(new URL('../../experiments/issue-199-decimal-number-rules.mjs', import.meta.url))),
      generatedSourceSha256: hash(nativeSource), models, rows,
      scope: 'explicit 92-pattern draft; no project-wide formatting or completion observation',
    });
  } finally { rmSync(directory, { recursive: true, force: true }); }
});
