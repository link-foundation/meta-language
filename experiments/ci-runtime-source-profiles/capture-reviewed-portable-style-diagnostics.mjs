import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { checkProjectData, readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { formatProductRuleSource } from '../../js/scripts/product-rule-style-corrections.mjs';
import { tokenize } from '../../js/src/translation/lexer.js';

const root = fileURLToPath(new URL('../../', import.meta.url));
const [directory, mode] = process.argv.slice(2);
assert.ok(directory && [undefined, '--check'].includes(mode));
const review = readProjectData(new URL('../../parity/self-translation/portable-ci-clone-review-138183525811.lino', import.meta.url));
const { capture } = review;
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const log = readFileSync(join(directory, `ci-${capture.run}-job-${capture.job}-rust-lint.log`));
assert.equal(digest(log), capture.logSha256);
const lines = log.toString('utf8').split('\n');
const plain = line => line.replace(/^\S+Z /u, '').replace(/\x1b\[[0-9;]*m/gu, '');
const paths = new Map();
const functions = new Map();
const referenceArgumentExpectation = `// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
`;

function readDiagnosticSource(logLine, lint) {
  assert.match(plain(lines[logLine - 1]), /^error: /u);
  const heading = /^\s*--> (src\/[^:]+\.rs):(\d+):(\d+)$/u.exec(plain(lines[logLine]));
  assert.ok(heading);
  let end = logLine + 1;
  while (end < lines.length && plain(lines[end]) !== '') end += 1;
  // Rust can render a blank line before the help and expected lint name.
  while (end + 1 < lines.length && /^\s*(?:=|help:)/u.test(plain(lines[end + 1]))) {
    end += 2;
    while (end < lines.length && plain(lines[end]) !== '') end += 1;
  }
  const diagnostic = lines.slice(logLine - 1, end).join('\n') + '\n';
  assert.ok(diagnostic.includes(`#${lint}`), `${logLine} is ${lint}`);
  const [, relative, line, column] = heading;
  const path = 'rust/' + relative;
  if (!paths.has(path)) {
    const source = execFileSync('git', ['--no-replace-objects', 'show', `${capture.commit}:${path}`], { cwd: root });
    paths.set(path, { path, bytes: source.length, sha256: digest(source), source: source.toString('utf8') });
  }
  return { path, line: Number(line), column: Number(column), logLine, lint, diagnosticSha256: digest(diagnostic) };
}

function readReviewedDeclaration(diagnostics, prefix = '') {
  const { path, line } = diagnostics[0];
  const original = paths.get(path).source;
  const sourceLines = original.split('\n');
  let first = line - 1;
  while (!/^pub fn /u.test(sourceLines[first])) first -= 1;
  let last = first;
  while (sourceLines[last] !== '}') last += 1;
  assert.ok(diagnostics.every(row => row.path === path && row.line > first && row.line <= last + 1
    && row.column > 0 && row.column <= sourceLines[row.line - 1].length));
  const body = sourceLines.slice(first, last + 1).join('\n');
  const name = /^pub fn (\w+)\(/u.exec(body)[1];
  let source = body;
  const ownershipReview = review.functions.find(row => row.path === path && row.name === name);
  if (ownershipReview) {
    assert.equal(ownershipReview.source, body);
    for (const { start } of ownershipReview.edits.toSorted((left, right) => right.start - left.start)) {
      assert.equal(source.slice(start, start + '.clone()'.length), '.clone()');
      source = source.slice(0, start) + source.slice(start + '.clone()'.length);
    }
    source = formatProductRuleSource(source).trimEnd();
  }
  assert.ok(original.includes(prefix + body));
  source = prefix + source;
  const key = path + ':' + name;
  if (!functions.has(key)) functions.set(key, { path, name, source, bytes: Buffer.byteLength(source), sha256: digest(source), edits: [] });
  return functions.get(key);
}

function recordEdit(reviews, before, after, prefix) {
  const diagnostics = reviews.map(([line, lint]) => readDiagnosticSource(line, lint));
  const declaration = readReviewedDeclaration(diagnostics, prefix);
  const from = typeof before === 'string' ? before : before.exec(declaration.source)?.[0];
  assert.ok(from && from !== after && declaration.source.split(from).length === 2, declaration.name);
  declaration.edits.push({ start: declaration.source.indexOf(from), from, to: after, diagnostics });
}

function recordBindingRename(logLine, from, to, count) {
  const diagnostic = readDiagnosticSource(logLine, 'similar_names');
  const declaration = readReviewedDeclaration([diagnostic]);
  const identifiers = tokenize(declaration.source, 'Rust').tokens.filter(token => token.kind === 'identifier');
  assert.ok(!identifiers.some(token => token.value === to));
  const occurrences = identifiers.filter(token => token.value === from);
  assert.equal(occurrences.length, count);
  for (const token of occurrences) declaration.edits.push({
    start: token.start, from, to, diagnostics: token === occurrences[0] ? [diagnostic] : [],
  });
}

recordBindingRename(1647, 'leaders_2', 'component_representatives', 2);
recordBindingRename(1662, 'owners_2', 'declaration_owner_column', 2);
recordBindingRename(1675, 'raw_', 'directive_spelling', 3);
// Combine identical empty-result branches while retaining the original lazy condition order.
recordEdit([[3575, 'if_same_then_else']], '|| (start >= (kinds.len() as f64))\n    {\n        Vec::<f64>::new()\n    } else if !(match (',
  '|| (start >= (kinds.len() as f64))\n        || !(match (');
recordEdit([[3595, 'redundant_else']], `                } else {
                    {
                        let index_3 = index + 1f64;
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }`, `                }
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }`);
for (const [unnecessaryClosure, copyClone] of [[3634, 3651], [3827, 3842], [3851, 3866]]) {
  recordEdit([[unnecessaryClosure, 'unnecessary_lazy_evaluations'], [copyClone, 'clone_on_copy']],
    /\.clone\(\)\n\s*\.unwrap_or_else\(\|\| f64::NAN\)/u, '.unwrap_or(f64::NAN)');
}
recordEdit([[3701, 'nonminimal_bool']], `(((!(((members > 1f64) && ((members % 1f64) == 0f64))
        && (members <= 9_007_199_254_740_991f64))
        || !(((declarations > members) && ((declarations % 1f64) == 0f64))
            && (declarations <= 9_007_199_254_740_991f64)))`,
`((!((members > 1f64) && ((members % 1f64) == 0f64)
        && (members <= 9_007_199_254_740_991f64)
        && (declarations > members) && ((declarations % 1f64) == 0f64)
        && (declarations <= 9_007_199_254_740_991f64))`);
for (const [line, receiver] of [[3723, 'actual_name'], [3746, 'text'], [3754, 'text'], [3783, 'text']]) {
  recordEdit([[line, 'single_char_pattern']], `${receiver}.contains("\\\\")`, `${receiver}.contains('\\\\')`);
}
// Both paths that reach the joined expression return the same empty decision.
recordEdit([[3762, 'branches_sharing_code']], `
                                {
                                    self::MlReadDeclarationDeferredBindingLoop61Result::MlReadDeclarationDeferredBindingLoop61Return(Vec::<f64>::new())
                                }
                            } else {
                                if (part == "statement_block") && (field == "body") {`, `
                            } else if (part == "statement_block") && (field == "body") {`);
const sharedBranch = [...functions.values()].find(row => row.name === 'ml_read_declaration_deferred_binding_loop61');
const sharedTail = `                                }
                                {
                                    self::MlReadDeclarationDeferredBindingLoop61Result::MlReadDeclarationDeferredBindingLoop61Return(Vec::<f64>::new())
                                }
                            }`;
const secondTail = sharedBranch.source.lastIndexOf(sharedTail);
assert.ok(secondTail > sharedBranch.edits[0].start);
sharedBranch.edits.push({ start: secondTail, from: sharedTail, to: `                            }
                            self::MlReadDeclarationDeferredBindingLoop61Result::MlReadDeclarationDeferredBindingLoop61Return(Vec::<f64>::new())`, diagnostics: [] });
recordEdit([[4141, 'fn_params_excessive_bools']], `${referenceArgumentExpectation}pub fn read_reference_identity_comparison_error(
    operation: &str,
    left_kind: &str,
    right_kind: &str,
    left_reference: bool,
    right_reference: bool,
    same_domain: bool,
    domain_present: bool,
    domain_reference: bool,
    same_operation_domain: bool,
) -> String {`, `pub fn read_reference_identity_comparison_error(
    operation: &str,
    left_kind: &str,
    right_kind: &str,
    left_reference: bool,
    right_reference: bool,
    same_domain: bool,
    domain_conditions: (bool, bool, bool),
) -> String {
    let (domain_present, domain_reference, same_operation_domain) = domain_conditions;`, referenceArgumentExpectation);
recordEdit([[4157, 'option_if_let_else']], `(match self::ml_array::optional_at(values, self::ml_array::number_index(at)) {
                Some(ml_present) => {
                    let _ = ml_present;
                    String::from("string")
                }
                None => String::from("undefined"),
            })`, `self::ml_array::optional_at(values, self::ml_array::number_index(at))
                .map_or_else(|| String::from("undefined"), |_| String::from("string"))`);

const output = new URL('../../parity/self-translation/portable-ci-source-style-review-138183525811.lino', import.meta.url);
const packet = {
  schemaVersion: 1, reviewKind: review.reviewKind, capture,
  sources: [...paths.values()].map(({ path, bytes, sha256 }) => ({ path, bytes, sha256 })),
  functions: [...functions.values()].map(row => ({ ...row, edits: row.edits.toSorted((left, right) => left.start - right.start) })),
};
if (mode === '--check') checkProjectData(output, packet);
else writeProjectData(output, packet);
console.log(`Bound ${packet.functions.flatMap(row => row.edits).flatMap(edit => edit.diagnostics).length} hosted diagnostics to ${packet.functions.length} complete generated functions.`);
