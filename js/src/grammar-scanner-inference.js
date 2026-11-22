import { parseGrammarLinks, percentEncodeLinksText } from './grammar-links.js';
import { createSourceDefinitionProgramLinks } from './source-definition-catalog.js';
import {
  planGrammarScannerTransitions,
  readGrammarScannerInferenceError,
  readGrammarScannerObservationByteLimit,
  readGrammarScannerObservationLimit,
  renderGrammarScannerDefinition,
} from './grammar-scanner-inference-rules.js';

const traceFields = ['mode', 'spelling', 'token', 'nextMode'];
export const grammarScannerInferenceArchiveIdentifier = 'regular-discovery-scanner-algorithm';

export function createGrammarScannerInferenceProgramLinks() {
  return createSourceDefinitionProgramLinks('meta-language/source-program/grammar-scanner-inference-program');
}

/** Infer native scanner declarations from a bounded, first-observed trace. */
export function inferGrammarScanner(observations, name = 'inferred_scanner') {
  const fail = reason => { throw new TypeError('grammar scanner inference: ' + reason); };
  if (!Array.isArray(observations) || observations.length > readGrammarScannerObservationLimit()) {
    fail(readGrammarScannerInferenceError(-1));
  }
  if (typeof name !== 'string' || !name || !name.isWellFormed() || Buffer.byteLength(name) > 4096) {
    fail('invalid scanner name');
  }
  const columns = [[], [], [], []];
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
  }
  const plan = planGrammarScannerTransitions(...columns);
  const reason = readGrammarScannerInferenceError(plan[0]);
  if (reason) fail(reason);
  const listing = renderGrammarScannerDefinition(...columns, plan, percentEncodeLinksText(name));
  const { declarations } = parseGrammarLinks('(grammar (start inferred_scanner_empty))\n'
    + listing + '(rule inferred_scanner_empty normal empty)\n');
  return { listing, modes: declarations.modes ?? [], scanner: declarations.scanners[0] };
}
