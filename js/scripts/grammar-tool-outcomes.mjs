// Authoring-tool outcomes distinguish syntax rejection from failed operations.
import { parseWithGrammar } from '../src/grammar.js';
import { GrammarParseError } from '../src/grammar-runtime.js';

function operationDiagnostic(error, seen = new Set()) {
  if (!(error instanceof Error)) return { thrownType: typeof error, description: String(error) };
  if (seen.has(error)) return { name: error.name, message: error.message, repeatedCause: true };
  seen.add(error);
  const diagnostic = { name: error.name, message: error.message };
  for (const key of ['reason', 'stack', 'code']) if (error[key] !== undefined) diagnostic[key] = error[key];
  if ('cause' in error) diagnostic.cause = operationDiagnostic(error.cause, seen);
  return diagnostic;
}

export function grammarToolOperationFailure(error) {
  return { accepted: null, error: operationDiagnostic(error) };
}

export function nativeGrammarToolOutcome(grammar, source, projectTree) {
  try { return { accepted: true, tree: projectTree(parseWithGrammar(grammar, source), source) }; }
  catch (error) {
    if (!(error instanceof GrammarParseError)) return grammarToolOperationFailure(error);
    return { accepted: false, offset: error.offset, reason: error.reason, expected: error.expected };
  }
}

export function targetGrammarToolOutcome(parser, source) {
  try { return { accepted: true, tree: parser.parse(source) }; }
  catch (error) {
    if (typeof parser.SyntaxError !== 'function' || !(error instanceof parser.SyntaxError)) return grammarToolOperationFailure(error);
    const offset = error.location?.start?.offset;
    if (!Number.isInteger(offset) || offset < 0 || offset > source.length) return grammarToolOperationFailure(error);
    return { accepted: false, offset: Buffer.byteLength(source.slice(0, offset)), reason: 'syntax' };
  }
}

export function isGrammarToolSyntaxRejection(outcome) {
  return outcome.accepted === false && outcome.reason === 'syntax';
}
