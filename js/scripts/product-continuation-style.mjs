import { tokenize } from '../src/translation/lexer.js';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';
import { simplifyRustBranches } from './reviewed-rust-branch-styles.mjs';
import { readPortableSourceLintStyles } from './portable-source-lint-style.mjs';

/** Keep the generated continuation names used by the existing library styles. */
export function restoreProductContinuationNames(source, names) {
  const { tokens } = tokenize(source, 'Rust');
  const replacements = new Map(names);
  const collisions = new Set(replacements.values());
  const seen = new Set();
  for (const token of [...tokens].reverse()) {
    if (token.kind !== 'identifier') continue;
    if (collisions.has(token.value)) throw new Error(`product continuation name already exists: ${token.value}`);
    const restored = replacements.get(token.value);
    if (restored === undefined) continue;
    seen.add(token.value);
    source = source.slice(0, token.start) + restored + source.slice(token.end);
  }
  if (seen.size !== replacements.size) throw new Error('product continuation names changed');
  return source;
}

/** Inline the two ordered, single-use operands of an optional comparison. */
export function readProductComparisonStyles(source) {
  const from = readSourceTextRustBlock(source, 'fn', 'ml_closure1_loop2');
  const comparison = /if \{\s*let (source_ml_source1_comparison_operand\d+) =\s*(self::ml_array::optional_at\([\s\S]+?\));\s*\{\s*let (source_ml_source1_comparison_operand\d+) = (index|true);\s*match \(\s*\1\.clone\(\),\s*\3,\s*\) \{\s*\(Some\(ml_left\), ml_right\) => ml_left == ml_right,\s*_ => false,\s*\}\s*\}\s*\} \{/gu;
  const matches = [...from.matchAll(comparison)];
  const expected = [
    ['self::ml_array::optional_at(targets,self::ml_array::number_index(edge))', 'index'],
    ['self::ml_array::optional_at(previous,self::ml_array::number_index(self::ml_array::optional_at(sources,self::ml_array::number_index(edge)).unwrap_or(f64::NAN)))', 'true'],
  ];
  const operands = matches.map(match => [match[2].replace(/\s+/gu, '').replaceAll(',)', ')'), match[4]]);
  if (JSON.stringify(operands) !== JSON.stringify(expected)) {
    throw new Error('product continuation optional comparisons changed');
  }
  const inlined = from.replace(comparison, (_, _leftName, left, _rightName, right) =>
    `if match (${left}, ${right}) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false, } {`);
  const to = readPortableSourceLintStyles(simplifyRustBranches(inlined)).source;
  return [{ op: 'replace', field: 'source', from, to }];
}
