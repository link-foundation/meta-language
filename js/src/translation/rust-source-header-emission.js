// Header facts come from the owned linked source facet. This selected model
// preserves primitive reference spelling while copying only checked values.
import { readRuntimeProjectData } from '../project-data-loader.js';
import { unsupported } from './diagnostics.js';
import { checkRustSourceSignature } from './rust-source-signatures.js';
import { acceptSharedNumericTypeShape, acceptSharedNumericParameter,
  acceptSharedNumericExpressionShape, acceptSharedNumericUnary,
  acceptSharedNumericBinary } from '../shared-numeric-function-rules.js';
import { acceptRustSourceHeaderEmissionModel, acceptRustSourceHeaderEmissionContext,
  acceptRustSourceHeaderEmissionDeclaration, acceptRustSourceHeaderEmissionPosition,
  acceptRustSourceHeaderEmissionExpression, readRustSourceHeaderPrimitive,
  acceptRustSourceHeaderLifetimeCharacter,
  renderRustSourceHeaderReference, renderRustSourceHeaderParameter,
  renderRustSourceHeaderLifetime, renderRustSourceHeaderDereference,
  renderRustSourceHeaderCopy, renderRustSourceHeaderFunction } from './rust-source-header-emission-rules.js';

export const RUST_SOURCE_HEADER_EMISSION_POLICY = readRuntimeProjectData(new URL('./data/rust-source-header-emission.lino', import.meta.url));
const fail = detail => { throw unsupported('Rust source header emission', detail); };

/** Admit the complete leaf program before changing any emitted interface. */
export function readRustSourceHeaderEmission(program, model) {
  const policy = RUST_SOURCE_HEADER_EMISSION_POLICY;
  if (!acceptRustSourceHeaderEmissionModel(model)) throw unsupported('Rust source header emission model', `select ${policy.model}`);
  const forbidden = Boolean(program.imports?.length || program.externals?.length || program.main?.effects?.length
    || policy.forbiddenContextFlags.some(name => Object.hasOwn(program, name) && program[name] !== false));
  if (!acceptRustSourceHeaderEmissionContext(program.sourceLanguage, forbidden)) fail('only Rust leaf value functions without imports, callers or effects are admitted');
  const headers = new Map();
  const pending = [];
  const type = value => {
    if (!value || !acceptSharedNumericTypeShape(value.kind, Object.keys(value).length === 1)) fail('only primitive binary64 and Boolean value types are admitted');
  };
  for (const declaration of program.declarations.values()) {
    if (!acceptRustSourceHeaderEmissionDeclaration(declaration.k, Boolean(declaration.modulePath?.length), Boolean(declaration.generated))) fail('only flat original leaf function declarations are admitted');
    if (!declaration.sourceSignature) fail('every original function needs its owned source signature facet');
    const signature = checkRustSourceSignature(declaration.sourceSignature, declaration.params.map(parameter => parameter.type), declaration.ret, program.sourceLanguage);
    for (const lifetime of signature.lifetimeBindings) {
      if (!lifetime.length) fail('the selected header lifetime profile requires an ASCII identifier');
      for (let index = 0; index < lifetime.length; index += 1) {
        if (!acceptRustSourceHeaderLifetimeCharacter(lifetime.charCodeAt(index), index === 0)) fail('the selected header lifetime profile requires an ASCII identifier');
      }
    }
    type(declaration.ret);
    for (const parameter of declaration.params) {
      type(parameter.type);
      if (!acceptSharedNumericParameter(parameter.guard != null, parameter.default != null)) fail('parameter defaults and guards need a separate interface proof');
    }
    for (const [position, result] of [...signature.parameters.map(value => [value, false]), [signature.result, true]]) {
      type(position.type);
      if (!acceptRustSourceHeaderEmissionPosition(position.type.kind, position.owner.kind,
        position.owner.path.length === 1 ? position.owner.path[0] : '', position.owner.arguments.length > 0,
        position.wrappers.some(wrapper => !policy.parameterWrappers.includes(wrapper)), result && position.wrappers.length > 0)) fail('headers require primitive f64 or bool owners, immutable-reference inputs and direct results');
    }
    headers.set(declaration.fullName, signature);
    pending.push(declaration.body);
  }
  const visited = new WeakSet();
  for (let at = 0; at < pending.length; at += 1) {
    const node = pending[at];
    if (!node || typeof node !== 'object') fail('a checked leaf body is required');
    if (visited.has(node)) continue;
    if (at >= 100000) fail('the leaf body exceeds its expression resource limit');
    visited.add(node); type(node.type);
    if (!acceptRustSourceHeaderEmissionExpression(node.k) || !acceptSharedNumericExpressionShape(node.k, node.rounding != null || node.byZero != null)) fail('calls and effects require a distinct interface transformation proof');
    if (node.k === 'unary') {
      if (!acceptSharedNumericUnary(node.op, node.type.kind, node.semantics ?? '')) fail('the unary operation has no admitted primitive value interpretation');
      pending.push(node.arg);
    } else if (node.k === 'binary') {
      type(node.domain);
      if (!acceptSharedNumericBinary(node.op, node.domain.kind, node.semantics ?? '')) fail('the binary operation has no admitted primitive value interpretation');
      pending.push(node.left, node.right);
    } else if (node.k === 'let') pending.push(node.value, node.body);
    else if (node.k === 'if') pending.push(node.cond, node.then, node.else);
  }
  return headers;
}

/** Traverse actual ordered facts; shared functions render every syntax choice. */
export function renderRustSourceHeaderEmission(signature, params, name, body, indent) {
  const copies = [];
  const parameters = params.map((parameter, index) => {
    const position = signature.parameters[index];
    let type = readRustSourceHeaderPrimitive(position.type.kind);
    let dereferenced = parameter.name;
    for (let layer = position.wrappers.length - 1; layer >= 0; layer -= 1) {
      type = renderRustSourceHeaderReference(position.lifetimes[layer] ?? '', type);
      dereferenced = renderRustSourceHeaderDereference(dereferenced);
    }
    if (position.wrappers.length) copies.push(renderRustSourceHeaderCopy(parameter.name, dereferenced));
    return renderRustSourceHeaderParameter(parameter.name, type);
  });
  return renderRustSourceHeaderFunction(name, signature.lifetimeBindings.map(renderRustSourceHeaderLifetime).join(', '),
    parameters.join(', '), readRustSourceHeaderPrimitive(signature.result.type.kind), indent([...copies, body].join('\n'), 1));
}
