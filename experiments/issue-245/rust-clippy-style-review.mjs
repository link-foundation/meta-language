import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { writeProjectData } from '../../js/scripts/project-data.mjs';

const root = new URL('../../', import.meta.url);
const commit = '83dff2e53f63bcfae83698c2531696e90417164f';
const job = 138157819576;
const digest = source => createHash('sha256').update(source).digest('hex');

function replaceReviewedText(source, from, to) {
  assert.equal(source.split(from).length, 2, from);
  return source.replace(from, () => to);
}

function readOtherStyles(addStyle) {
  const register = (module, part) => (...args) =>
    addStyle('rust/src/translation/' + module + '/part_' + part + '.rs', ...args);
  const ascii = register('ascii_text_predicate_rules', 2);
  ascii([3581], 151, 153, from =>
    replaceReviewedText(from, 'Vec::<f64>::new()\n    } else if', 'return Vec::<f64>::new();\n    }\n    if'));
  ascii([3601], 311, 319, from => {
    assert.match(from, /^ +\} else \{\n[\s\S]+\n +\}/u);
    return from.replace('} else {', '}\n{');
  });
  const numericDefault = from => from.replace(/\.clone\(\)\s+\.unwrap_or_else\(\|\| f64::NAN\)/u, '.unwrap_or(f64::NAN)');
  ascii([3640, 3657], 717, 718, numericDefault);
  const external = register('external_emission_rules', 1);
  external([3833, 3848], 457, 458, numericDefault);
  external([3857, 3872], 533, 534, numericDefault);

  const dependency = part => register('declaration_dependency_rules', part);
  dependency(2)([3707], 146, 149, from => {
    assert.match(from, /!\(\(\(members[^]*\|\| !\(\(\(declarations/u);
    return '    if (((!((members > 1f64) && ((members % 1f64) == 0f64)\n'
      + '        && (members <= 9_007_199_254_740_991f64)\n'
      + '        && (declarations > members) && ((declarations % 1f64) == 0f64)\n'
      + '        && (declarations <= 9_007_199_254_740_991f64)))';
  });
  const slash = from => replaceReviewedText(from, '.contains("\\\\")', ".contains('\\\\')");
  dependency(2)([3729], 462, 462, slash);
  dependency(4)([3752], 503, 503, slash);
  dependency(4)([3760], 608, 608, slash);
  dependency(5)([3789], 156, 156, slash);
  dependency(5)([3768], 70, 91, from => {
    const fallback = / +\{\n +self::MlReadDeclarationDeferredBindingLoop61Result::MlReadDeclarationDeferredBindingLoop61Return\(Vec::<f64>::new\(\)\)\n +\}/gu;
    const original = [...from.matchAll(fallback)];
    assert.equal(original.length, 2);
    return from.replace(fallback, '').replace('} else {\n                                if', '} else if')
      .replace(/\n +\}\s*\n +\}$/u, '\n}\n' + original[1][0]);
  });

  const rename = (from, to, count) => source => {
    const token = new RegExp('\\b' + from + '\\b', 'gu');
    assert.equal([...source.matchAll(token)].length, count);
    return source.replace(token, to);
  };
  dependency(3)([1653], 315, 317, rename('leaders_2', 'component_leaders', 2));
  dependency(3)([1668], 511, 515, rename('owners_2', 'source_owners', 2));
  register('javascript_record_parameter_rules', 3)([1681], 523, 529, rename('raw_', 'directive_text', 3));

  register('reference_identity_rules', 1)([4147], 22, 38, from => {
    const start = from.indexOf('pub fn ');
    assert.ok(start >= 0 && from.includes('clippy::too_many_arguments'));
    return replaceReviewedText(from.slice(start),
      '    left_reference: bool,\n    right_reference: bool,\n    same_domain: bool,\n'
        + '    domain_present: bool,\n    domain_reference: bool,\n    same_operation_domain: bool,\n) -> String {',
      '    operands: (bool, bool, bool),\n    domain: (bool, bool, bool),\n) -> String {\n'
        + '    let (left_reference, right_reference, same_domain) = operands;\n'
        + '    let (domain_present, domain_reference, same_operation_domain) = domain;');
  });
  register('rust_literal_match_rules', 1)([4163], 298, 304, from => {
    assert.match(from, /let _ = ml_present;\s+String::from\("string"\)/u);
    assert.match(from, /None => String::from\("undefined"\)/u);
    return '            }) || self::ml_array::optional_at(values, self::ml_array::number_index(at)).is_none()';
  });
}

// The saved job provides rendered spans only. Confirm each reported deletion
// against the exact source line, then retain the full module and log digests.
export function readRustClippyStyleReview(log, sourceAt) {
  const lines = log.replace(/\x1b\[[0-?]*[ -/]*[@-~]/gu, '').split('\n');
  const files = new Map();
  const readModule = file => {
    if (!files.has(file)) {
      const source = sourceAt(file);
      files.set(file, { source, lines: source.split('\n'), diagnostics: [], styles: [] });
    }
    return files.get(file);
  };
  const lineOffset = (module, row) =>
    module.lines.slice(0, row - 1).reduce((offset, sourceLine) => offset + sourceLine.length + 1, 0);
  for (const [index, line] of lines.entries()) {
    if (!line.endsWith('error: redundant clone')) continue;
    const location = /--> (src\/.+):(\d+):(\d+)$/u.exec(lines[index + 1]);
    assert.ok(location, 'every redundant-clone diagnostic has a primary source span');
    const [, path, lineNumber, columnNumber] = location;
    if (!/\/(?:part_\d+)\.rs$/u.test(path)) continue;
    const file = `rust/${path}`;
    const module = readModule(file);
    const row = Number(lineNumber);
    const column = Number(columnNumber);
    const text = module.lines[row - 1];
    const display = new RegExp(` ${row} \\| (.*)$`, 'u').exec(lines[index + 3]);
    assert.ok(display);
    const excerpt = display[1];
    const leadingEllipsis = excerpt.startsWith('...');
    const trailingEllipsis = excerpt.endsWith('...');
    const visible = excerpt.slice(leadingEllipsis ? 3 : 0, trailingEllipsis ? -3 : undefined);
    assert.ok((leadingEllipsis ? text.includes(visible) : text.startsWith(visible))
      && (trailingEllipsis || text.endsWith(visible)), file);
    assert.match(lines[index + 4], /\^{8} help: remove this$/u);
    const start = lineOffset(module, row) + column - 1;
    assert.equal(module.source.slice(start, start + 8), '.clone()', file);
    module.diagnostics.push({ logLine: index + 1, row, column, start });
  }
  readOtherStyles((file, logLines, first, last, transform) => {
    for (const line of logLines) {
      assert.match(lines[line - 1], / error: /u);
      assert.ok(lines[line].includes(file.slice(5) + ':'), 'reviewed diagnostic belongs to ' + file);
    }
    const module = readModule(file);
    const start = lineOffset(module, first);
    const from = module.lines.slice(first - 1, last).join('\n');
    const to = transform(from);
    assert.notEqual(to, from, file + ':' + first);
    module.styles.push({ logLines, start, from, to });
  });
  const modules = [...files].map(([path, { source, diagnostics, styles }]) => ({
    path, sourceSha256: digest(source), diagnostics: diagnostics.sort((left, right) => left.start - right.start), styles,
  }));
  assert.equal(modules.reduce((total, module) => total + module.diagnostics.length, 0), 150);
  assert.equal(modules.reduce((total, module) => total + module.styles.flatMap(style => style.logLines).length, 0), 19);
  return { schemaVersion: 2, run: 45283381518, job, commit, rustVersion: '1.100.0', logSha256: digest(log), modules };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const [, , logPath, outputPath] = process.argv;
  assert.ok(logPath && outputPath);
  const { execFileSync } = await import('node:child_process');
  const review = readRustClippyStyleReview(readFileSync(logPath, 'utf8'), (path) =>
    execFileSync('git', ['show', `${commit}:${path}`], { cwd: root, encoding: 'utf8' }));
  writeProjectData(outputPath, review);
}
