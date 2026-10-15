import {checkRustRuntimeShell} from './translation/rust-runtime-shell.js';
import {checkRustPureUnitCalls} from './translation/rust-pure-unit-calls.js';
import {checkRustConstFunctionProof,checkRustConstFunctionContainers} from './translation/rust-const-functions.js';
import {checkRustEmptyOptionProof} from './translation/rust-empty-options.js';
import {checkRustSourceSignature} from './translation/rust-source-signatures.js';
import {validateRustTypeNamespace, checkRustTypeNamespaceRecordShapes} from './translation/rust-type-namespace.js';
import {checkRustUnitCloneDerivations} from './translation/rust-unit-clone.js';
import {validateSourceSpans} from './source-spans.js';
import {acceptProgramIntegralMetadata} from './program-numeric-metadata-rules.js';
import {checkRustScalarLoopProof} from './translation/rust-scalar-loops.js';
import {checkRustPrimitiveOwnerProof} from './translation/rust-primitive-owners.js';
import {checkRustOwnedLocalStateProof,checkRustOwnedLocalStateRecordShapes} from './translation/rust-owned-local-state.js';
// Validate the source-free checked IR boundary before any target emission.
import { fixedBounds, sameType } from './translation/types.js';
import { checkLinkedMatchCoverage } from './translation/check.js';
import { acceptLinkedTextConversionKind, readNumberDisplayRefusal, readTuplePosition } from './translation/primitive-value-rules.js';
import { readLinkedPatternBindingScope } from './typed-program-pattern-bindings.js';
import { acceptProgramDivisionRounding, acceptProgramExpressionField, acceptProgramExpressionOperation, acceptProgramOptionalAbsence, acceptProgramTupleProductModel, readProgramReferenceRole } from './typed-program-link-rules.js';

const primitives = new Set(['nat', 'int', 'bool', 'string', 'unit', 'float', 'output']);
const operationKinds = new Set(['math', 'stringMap', 'stringTest', 'method', 'collectionRead', 'collectionItems', 'optionalPredicate']);

export function validateLinkedTypedProgram(program, references = new WeakMap(), { graphPatterns = false, patternScopes = new WeakMap() } = {}) {
  checkRustConstFunctionContainers(program);
  checkRustRuntimeShell(program);
