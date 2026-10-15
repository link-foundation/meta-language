import {acceptRustRuntimeShellModel} from './rust-runtime-shell-rules.js';
import {readRustRuntimeShellSource,refuseRustRuntimeShellComposition} from './rust-runtime-shell.js';
import {acceptRustPureUnitCallModel} from './rust-pure-unit-call-rules.js';
import {readRustPureUnitFunction,readRustPureUnitMain,finishRustPureUnitSource,readRustPureUnitItemContext} from './rust-pure-unit-calls.js';
import {acceptRustEmptyOptionSourceModel} from './rust-empty-option-rules.js';
import {readRustEmptyOptionResult,readRustEmptyOptionBody,createRustEmptyOptionProof,finishRustEmptyOptionSource} from './rust-empty-options.js';
import { acceptRustFloatPredicateMethod, acceptRustFloatOperationMethod, acceptRustFloatConstantName, acceptRustFloatConstantOwner, acceptRustFloatPredicateReceiver } from './rust-float-predicate-rules.js';
import { createRustFloatPredicateExpression, createRustFloatOperationExpression, createRustFloatConstantExpression } from './rust-float-predicates.js';
import { acceptRustCompletionModel } from './rust-completion-rules.js';
import { readRustFunctionCompletion } from './rust-completions.js';
import { readRustCompletionPanicMessage } from './rust-completion-panic.js';
import { acceptRustSourceSignatureModel } from './rust-source-signature-rules.js';
import { readRustSourceTypeHead, readRustSourceTypeOwner, createRustSourceSignature, RUST_SOURCE_SIGNATURE_POLICY } from './rust-source-signatures.js';
import { acceptRustTypeNamespaceModel, acceptRustTypeNamespaceDeclaration, acceptRustTypeNamespaceBodyConstructor, acceptRustTypeNamespaceBodyMethod, acceptRustTypeNamespaceMethodDiscovery, acceptRustTypeNamespaceDiscoveryType } from './rust-type-namespace-rules.js';
import { captureRustUnitCloneDerivation, acceptRustUnitCloneReceiver } from './rust-unit-clone.js';
import { createRustTypeNamespaceProof, requireRustTypeNamespaceWithoutImports, requireRustTypeNamespaceAnchor } from './rust-type-namespace.js';
import { readRustInclusiveRangeQuery, readRustStringEmptyQuery, readRustEmptyStringConstructor } from './rust-read-only-queries.js';
import { acceptRustReadOnlyQueryOwner } from './rust-read-only-query-rules.js';
import { readStringMapOperation, readStringTestOperation } from './frontend-rules.js';
// Rust frontend for the portable core. It reads the subset of Rust the core
// can represent faithfully: modules with `use` of crate items, enums with
// unit and tuple variants (`Box<T>` fields are the recursive occurrences of
// a value type), free functions over machine integers, `bool` and `String`,
// expression bodies with `let`, `if`, `match` and `format!`, and a `main`
// made of `println!`, `let` and `assert!`/`assert_eq!` statements.
// Everything else is rejected with a precise obligation.

import { rustMacroMessage } from './aborts.js';
import { TranslationError, unsupported } from './diagnostics.js';
import { TokenCursor, describe, tokenize } from './lexer.js';
import { BOOL, FLOAT, STRING, UNIT, array, rustFixedType, tuple } from './types.js';
import { RustSourceOrigins } from './rust-source-origins.js';
import { acceptRustSourceBoxOwner, acceptRustSourceLifetimeBinding, acceptRustSourceLifetimeReference, acceptRustSourceLifetimeResult } from './rust-indirection-rules.js';
import { acceptRustTupleIndexSpelling } from './rust-product-rules.js';
import { acceptRustSourceArrayOwner, acceptRustSourceArrayElement, acceptRustSourceArrayResultModel, acceptRustSourceArrayResult } from './rust-array-rules.js';
import { acceptRustUnwrittenScalarParameter } from './rust-scalar-parameter-rules.js';
import { readRustScalarParameterBody } from './rust-scalar-parameters.js';
import { acceptRustScalarLoopModel, readRustScalarLoop } from './rust-scalar-loops.js';
import { checkProgram } from './check.js';
import {acceptRustPrimitiveOwnerModel} from './rust-primitive-owner-rules.js';
import {readRustStandardPrimitivePosition,createRustPrimitiveOwnerProof,acceptRustPrimitiveOwnerImportName} from './rust-primitive-owners.js';
import {acceptRustOwnedLocalStateModel,acceptRustOwnedLocalSourceOperation} from './rust-owned-local-state-rules.js';
import {readRustOwnedLocalStateBody} from './rust-owned-local-state-reader.js';
import {RUST_OWNED_LOCAL_STATE_POLICY} from './rust-owned-local-state.js';
import {acceptRustConstFunctionModel} from './rust-const-function-rules.js';
import {createRustConstFunctionProof} from './rust-const-functions.js';

const ACCEPTED_DERIVES = new Set(['Clone', 'Copy', 'Debug', 'PartialEq', 'Eq', 'Hash', 'PartialOrd', 'Ord']);
const ACCEPTED_LINTS = new Set(['allow', 'warn', 'deny', 'expect', 'must_use', 'inline']);
const COMPARISON = { '==': 'eq', '!=': 'ne', '<': 'lt', '<=': 'le', '>': 'gt', '>=': 'ge' };
const ADDITIVE = { '+': 'add', '-': 'sub' };
const MULTIPLICATIVE = { '*': 'mul', '/': 'div', '%': 'rem' };
const RESERVED_ITEMS = new Set(['impl', 'trait', 'struct', 'union', 'static', 'type', 'extern', 'macro_rules', 'async', 'unsafe']);

export function parseRust(source, { arrayResultModel = '', completionModel = '', sourceSignatureModel = '', scalarLoopModel = '', primitiveOwnerModel = '', emptyOptionSourceModel = '', typeNamespaceModel = '',pureUnitCallModel='',localValueStateModel='',constFunctionModel='',runtimeShellModel='' } = {}) {
  if(!acceptRustConstFunctionModel(constFunctionModel))throw unsupported('Rust const function model','select the legacy model or closed pure Rust const predicates 1');
  if(constFunctionModel&&[arrayResultModel,completionModel,scalarLoopModel,primitiveOwnerModel,emptyOptionSourceModel,typeNamespaceModel,localValueStateModel,pureUnitCallModel,runtimeShellModel].some(Boolean))throw unsupported('Rust const function model','other source interpretations require independent composition proofs');
  if(!acceptRustRuntimeShellModel(runtimeShellModel))throw unsupported('Rust runtime shell model','select the legacy model or closed pure Rust unit runtime shell 1');
  if(runtimeShellModel){if([arrayResultModel,completionModel,sourceSignatureModel,scalarLoopModel,primitiveOwnerModel,emptyOptionSourceModel,typeNamespaceModel,pureUnitCallModel,localValueStateModel,constFunctionModel].some(Boolean))refuseRustRuntimeShellComposition('the whole runtime interpretation requires its independent source profile');return readRustRuntimeShellSource(source);}
