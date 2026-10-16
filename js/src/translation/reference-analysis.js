// Reference representation is selected from operations and inferred values.
// Scalar equality alone never selects a heap representation.
import { tokenize } from './lexer.js';
import { referenceMethodOperation, referenceOperations } from './reference-model.js';
import { createSurfaceReferenceColumns, createSurfaceReferenceFacts } from './surface-reference-facts.js';
import {
  acceptSurfaceReferenceClosureToken,
  acceptSurfaceReferenceMutatingMethod,
  hasSurfaceStructuralIdentity,
  requireSurfaceReferenceToken,
} from './surface-reference-analysis-rules.js';

/** Actual source-method aliases of the existing linked mutating operations. */
export function readSurfaceReferenceMutatingMethods() {
  const methods = [...new Set(referenceOperations().map(operation => operation.method).filter(method => typeof method === 'string'))];
  return methods.filter(method => ['array', 'map', 'set', 'record'].some(kind => referenceMethodOperation(kind, method)?.mutates));
}

const mutatingMethods = readSurfaceReferenceMutatingMethods();

export function requiresReferenceSemantics(source) {
  const { tokens } = tokenize(source, 'JavaScript');
  // Templates carry structured parts instead of value text. These columns
  // project only nonempty keyword/operator/method comparisons, so missing
  // token text and empty text have the same original selection decisions.
  const closure = tokens.some((token, index) => acceptSurfaceReferenceClosureToken(token.kind, token.value ?? '', tokens[index + 1]?.value ?? ''));
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];
    const value = token.value ?? '';
    if (requireSurfaceReferenceToken([
      token.kind, value,
      tokens[index - 1]?.kind ?? '', tokens[index - 1]?.value ?? '', tokens[index - 2]?.value ?? '',
      tokens[index + 1]?.kind ?? '', tokens[index + 1]?.value ?? '', tokens[index + 2]?.value ?? '',
    ], closure, acceptSurfaceReferenceMutatingMethod(value, mutatingMethods),
    )) return true;
  }
  return false;
}

export function hasStructuralIdentity(program) {
  const facts = createSurfaceReferenceFacts(program);
  const packed = createSurfaceReferenceColumns(facts);
  return hasSurfaceStructuralIdentity(packed.columns, packed.rows, packed.size, facts.root);
}
