import {checkRustRuntimeShell} from './translation/rust-runtime-shell.js';
// Rebuild a linked product program through the existing Surface checker.
// The adapter carries declarations and shared projection instructions; it
// does not assign meaning to types, operators, patterns or recursion.
import {checkRustConstFunctionContainers} from './translation/rust-const-functions.js';
import { checkProgram } from './translation/check.js';
import {checkRustOwnedLocalStateRecordShapes} from './translation/rust-owned-local-state.js';
import { validateRustTypeNamespace, checkRustTypeNamespaceRecordShapes } from './translation/rust-type-namespace.js';
import { projectLinkedProgramExpression, projectLinkedProgramDeclarationPath } from './typed-program-expression-projection.js';
import { readProgramExpressionProjection, readProgramNeutralFieldDefault } from './typed-program-link-rules.js';

export function recheckLinkedTypedProgram(program, {wholeProgram=false}={}) {
  if(checkRustConstFunctionContainers(program))wholeProgram=true;
  if(checkRustRuntimeShell(program))return program;
