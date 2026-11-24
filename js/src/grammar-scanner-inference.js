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
import { readGrammarScannerModeStackLimit, readGrammarScannerModeStackOperation, renderGrammarScannerModeStackDefinition } from './grammar-scanner-mode-stack-rules.js';

const traceFields = ['mode', 'spelling', 'token', 'nextMode'];
export const grammarScannerInferenceArchiveIdentifier = 'regular-discovery-scanner-algorithm';
export const grammarScannerCharacterClassArchiveIdentifier = 'regular-discovery-scanner-character-class-algorithm';
export const grammarScannerModeStackArchiveIdentifier = 'regular-discovery-scanner-mode-stack-algorithm';

export function createGrammarScannerInferenceProgramLinks() {
  return createSourceDefinitionProgramLinks('meta-language/source-program/grammar-scanner-inference-program');
}

export function createGrammarScannerCharacterClassProgramLinks() {
  return createSourceDefinitionProgramLinks('meta-language/source-program/grammar-scanner-character-class-program');
}

export function createGrammarScannerModeStackProgramLinks() {
  return createSourceDefinitionProgramLinks('meta-language/source-program/grammar-scanner-mode-stack-program');
}

/** Infer native scanner declarations from a bounded, first-observed trace. */
export function inferGrammarScanner(observations, name = 'inferred_scanner') {
  return inferScanner(observations, name, false);
}

/** Infer scanner character classes from the observed scalars. */
export function inferGrammarScannerClasses(observations, name = 'inferred_scanner') {
  return inferScanner(observations, name, true);
}

/** Infer finite scanner transitions from complete pre-token and post-token mode stacks. */
export function inferGrammarScannerModeStack(observations, name = 'inferred_scanner', characterClasses = false) {
  const fail = reason => { throw new TypeError('grammar scanner inference: ' + reason); };
  const invalid = 'expected bounded nonempty scanner mode stack observations';
  if (!Array.isArray(observations) || observations.length === 0 || observations.length > readGrammarScannerObservationLimit()) fail(invalid);
  if (typeof name !== 'string' || !name || !name.isWellFormed() || Buffer.byteLength(name) > 4096) fail('invalid scanner name');
  if (typeof characterClasses !== 'boolean') fail('scanner character classes require a Boolean selection');
  const modes = [], spellings = [], tokens = [], effects = [], codePoints = [];
  const observedModes = new Set();
  let bytes = 0;
  const encode = value => {
    if (typeof value !== 'string' || !value || !value.isWellFormed()) fail(invalid);
    bytes += Buffer.byteLength(value);
    if (bytes > readGrammarScannerObservationByteLimit()) fail('observation byte limit exceeded');
    return percentEncodeLinksText(value);
  };
  const encodeStack = stack => {
    if (!Array.isArray(stack) || stack.length === 0 || stack.length > readGrammarScannerModeStackLimit()) fail(invalid);
    const encoded = [];
    for (const mode of stack) {
      const value = encode(mode);
      observedModes.add(value);
      encoded.push(value);
    }
    return encoded;
  };
  const fields = ['before', 'spelling', 'token', 'after'];
  for (const row of observations) {
    if (!row || typeof row !== 'object' || Array.isArray(row) || Object.keys(row).length !== fields.length
      || !fields.every(field => Object.hasOwn(row, field))) fail(invalid);
    const before = encodeStack(row.before), after = encodeStack(row.after);
    const spelling = encode(row.spelling), token = encode(row.token);
    const effect = readGrammarScannerModeStackOperation(before, after);
    if (!effect) fail('a token observation requires at most one native scanner mode operation');
    modes.push(before.at(-1));
    spellings.push(spelling);
    tokens.push(token);
    effects.push(effect);
    const scalar = row.spelling.codePointAt(0);
    codePoints.push(row.spelling.length === (scalar > 0xffff ? 2 : 1) ? scalar : -1);
  }
  const plan = planGrammarScannerTransitions(modes, spellings, tokens, effects);
  const reason = readGrammarScannerInferenceError(plan[0]);
  if (reason) fail(reason);
  const matchers = characterClasses ? planGrammarScannerCharacterClasses(modes, spellings, tokens, effects, codePoints, plan) : [];
  return readScannerListing(renderGrammarScannerModeStackDefinition(modes, spellings, tokens, effects,
    [...observedModes], codePoints, plan, matchers, percentEncodeLinksText(name)));
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
  return readScannerListing(listing);
}

function readScannerListing(listing) {
  const { declarations } = parseGrammarLinks('(grammar (start inferred_scanner_empty))\n'
    + listing + '(rule inferred_scanner_empty normal empty)\n');
  return { listing, modes: declarations.modes ?? [], scanner: declarations.scanners[0] };
}
