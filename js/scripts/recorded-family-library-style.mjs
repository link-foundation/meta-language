// Append reviewed library syntax styles without rewriting earlier Links records.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {DecoratorSet} from '../src/decorators.js';
import {flattenFamilyLibraryElseIf, mergeFamilyLibraryIdenticalBranches, qualifyFamilyLibraryBooleanFunctions,
  invertFamilyLibraryInequalityBranches, bindFamilyLibraryInequalityConditions, simplifyFamilyLibraryNegatedConjunctions, liftFamilyLibraryTruthinessConditions} from './portable-conditional-source-style.mjs';
export {flattenFamilyLibraryElseIf, mergeFamilyLibraryIdenticalBranches, qualifyFamilyLibraryBooleanFunctions,
  invertFamilyLibraryInequalityBranches, bindFamilyLibraryInequalityConditions, simplifyFamilyLibraryNegatedConjunctions, liftFamilyLibraryTruthinessConditions} from './portable-conditional-source-style.mjs';
import {readPortableSourceParameterStyles, readPortableSourceLintStyles} from './portable-source-library-style.mjs';

/** Replay only this family's post-base scope, then append an exact new record. */
export function applyRecordedFamilyLibraryStyles(document, source, {
  family, update = false, format = value => value, parameterLoans = false, baseRecordHashes = [],
} = {}) {
  assert.match(family, /^[a-z][a-z0-9-]*$/u, 'a recorded library has an explicit family');
  const lines = document.match(/[^\n]*\n|[^\n]+$/gu) ?? [];
  for (const [index, expected] of baseRecordHashes.entries()) {
    assert.equal(createHash('sha256').update(lines[index] ?? '').digest('hex'), expected,
      'the complete historical family style record retains every original byte');
  }
  const scope = 'reviewed-library-' + family;
  const definitions = DecoratorSet.fromLino(document).decorators;
  const scoped = new DecoratorSet(definitions.filter(({when}) => when.some(([field, value]) => field === 'scope' && value === scope)));
  const original = format(source);
  for (let pass = 0; pass < 10; pass += 1) {
    const replayed = format(scoped.decorate('emitter', {format: 'Rust', scope, source}).source);
    if (replayed === source) break;
    source = replayed;
    assert.ok(pass < 9, 'recorded family styles converge');
  }
  // Derive the target from actual emitted/base output, independently of any
  // later recorded replacement. A forged replacement cannot certify itself.
  let corrected = parameterLoans ? readPortableSourceParameterStyles(original,
    typeof parameterLoans === 'object' ? parameterLoans : undefined).source : original;
  if (parameterLoans && typeof parameterLoans === 'object' && parameterLoans.normalizeBorrowedArguments) {
    // Re-read the first pass's actual interfaces: a converted String becomes
    // an &str fact only after its source declaration has really changed.
    corrected = readPortableSourceParameterStyles(corrected, parameterLoans).source;
  }
  corrected = liftFamilyLibraryTruthinessConditions(corrected);
  corrected = flattenFamilyLibraryElseIf(corrected);
  corrected = mergeFamilyLibraryIdenticalBranches(corrected);
  corrected = simplifyFamilyLibraryNegatedConjunctions(corrected);
  corrected = invertFamilyLibraryInequalityBranches(corrected);
  corrected = bindFamilyLibraryInequalityConditions(corrected);
  // These families retain their original module attributes. Apply mechanical
  // function styles without adding the older common Number-model allowances.
  const header = /^#!\[allow\([\s\S]*?\)\]/mu.exec(corrected);
  if (header) {
    const end = header.index + header[0].length;
    corrected = corrected.slice(0, end) + readPortableSourceLintStyles(corrected.slice(end)).source;
  } else corrected = readPortableSourceLintStyles(corrected).source;
  corrected = qualifyFamilyLibraryBooleanFunctions(corrected);
  corrected = format(corrected);
  if (corrected === source) return {document, source};
  assert.ok(update, 'recorded family library styles are stale; run --update-library-lint-styles');
  const id = 'reviewed-b50-' + family + '-' + createHash('sha256').update(source).digest('hex').slice(0, 16);
  assert.ok(!definitions.some(definition => definition.id === id), 'a recorded family correction remains applicable');
  const addition = new DecoratorSet([{id, level: 'emitter', order: 5000,
    when: [['format', 'Rust'], ['scope', scope]],
    actions: [{op: 'replace', field: 'source', from: source, to: corrected}],
  }]).toLino();
  const updated = document + (document && !document.endsWith('\n') ? '\n' : '') + addition;
  const replayed = applyRecordedFamilyLibraryStyles(updated, original, {family, format, parameterLoans, baseRecordHashes});
  assert.equal(replayed.source, corrected, 'the appended family library style replays exactly');
  return {document: updated, source: corrected};
}
