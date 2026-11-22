import { parseGrammarLinks, percentEncodeLinksText } from './grammar-links.js';
import { createSourceDefinitionProgramLinks } from './source-definition-catalog.js';
import {
  planGrammarScannerTransitions,
  readGrammarScannerInferenceError,
  readGrammarScannerObservationByteLimit,
  readGrammarScannerObservationLimit,
  renderGrammarScannerDefinition,
} from './grammar-scanner-inference-rules.js';
import { planGrammarScannerCharacterClasses, renderGrammarScannerCharacterClassDefinition } from './grammar-scanner-character-class-rules.js';

const traceFields = ['mode', 'spelling', 'token', 'nextMode'];
export const grammarScannerInferenceArchiveIdentifier = 'regular-discovery-scanner-algorithm';
export const grammarScannerCharacterClassArchiveIdentifier = 'regular-discovery-scanner-character-class-algorithm';

export function createGrammarScannerInferenceProgramLinks() {
  return createSourceDefinitionProgramLinks('meta-language/source-program/grammar-scanner-inference-program');
}

export function createGrammarScannerCharacterClassProgramLinks() {
  return createSourceDefinitionProgramLinks('meta-language/source-program/grammar-scanner-character-class-program');
}

/** Infer native scanner declarations from a bounded, first-observed trace. */
export function inferGrammarScanner(observations, name = 'inferred_scanner') {
  return inferScanner(observations, name, false);
}

/** Infer scanner character classes from the observed scalars. */
export function inferGrammarScannerClasses(observations, name = 'inferred_scanner') {
  return inferScanner(observations, name, true);
}

function inferScanner(observations, name, characterClasses) {
  const fail = reason => { throw new TypeError('grammar scanner inference: ' + reason); };
  if (!Array.isArray(observations) || observations.length > readGrammarScannerObservationLimit()) {
    fail(readGrammarScannerInferenceError(-1));
  }
  if (typeof name !== 'string' || !name || !name.isWellFormed() || Buffer.byteLength(name) > 4096) {
    fail('invalid scanner name');
  }
  const columns = [[], [], [], []];
  const codePoints = [];
  let bytes = 0;
  for (const row of observations) {
    if (!row || typeof row !== 'object' || Array.isArray(row) || Object.keys(row).length !== traceFields.length) {
      fail(readGrammarScannerInferenceError(-1));
    }
    for (const [index, key] of traceFields.entries()) {
      const value = row[key];
      if (!Object.hasOwn(row, key) || typeof value !== 'string' || !value || !value.isWellFormed()) {
        fail(readGrammarScannerInferenceError(-1));
      }
      bytes += Buffer.byteLength(value);
      if (bytes > readGrammarScannerObservationByteLimit()) fail('observation byte limit exceeded');
      columns[index].push(percentEncodeLinksText(value));
    }
    if (characterClasses) {
      const scalar = row.spelling.codePointAt(0);
      codePoints.push(row.spelling.length === (scalar > 0xffff ? 2 : 1) ? scalar : -1);
    }
  }
  const plan = planGrammarScannerTransitions(...columns);
  const reason = readGrammarScannerInferenceError(plan[0]);
  if (reason) fail(reason);
  const scannerName = percentEncodeLinksText(name);
  const [modes, , tokens, nextModes] = columns;
  const listing = characterClasses
    ? renderGrammarScannerCharacterClassDefinition(modes, tokens, nextModes, codePoints, plan,
      planGrammarScannerCharacterClasses(...columns, codePoints, plan), scannerName)
    : renderGrammarScannerDefinition(...columns, plan, scannerName);
  const { declarations } = parseGrammarLinks('(grammar (start inferred_scanner_empty))\n'
    + listing + '(rule inferred_scanner_empty normal empty)\n');
  return { listing, modes: declarations.modes ?? [], scanner: declarations.scanners[0] };
}
