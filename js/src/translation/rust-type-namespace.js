import { readRuntimeProjectData } from '../project-data-loader.js';
import { unsupported, typeError } from './diagnostics.js';
import { RustSourceOrigins } from './rust-source-origins.js';
import { RUST_SOURCE_SIGNATURE_POLICY } from './rust-source-signatures.js';
import { acceptRustTypeNamespaceModel, acceptRustTypeNamespacePosition,
  acceptRustTypeNamespaceDeclaration, acceptRustTypeNamespaceFields, acceptRustTypeNamespaceAnchor } from './rust-type-namespace-rules.js';

export const RUST_TYPE_NAMESPACE_POLICY = readRuntimeProjectData(new URL('./data/rust-type-namespace.lino', import.meta.url));
const fail = detail => { throw typeError(`an owned Rust type namespace ${detail}`); };

/** Capture actual ordered source spellings alongside the existing signature. */
export function createRustTypeNamespaceProof(parameters, result) {
  return { model: RUST_TYPE_NAMESPACE_POLICY.model, language: RUST_TYPE_NAMESPACE_POLICY.language,
    parameters: parameters.map(position => [...(position.namespaceSpelling ?? [])]), result: [...(result.namespaceSpelling ?? [])] };
}

/** Compute the actual anchored parent traversal before the catalog lookup. */
export function requireRustTypeNamespaceAnchor(source, path) {
  let parents = 0;
  while (source[parents] === 'super') parents += 1;
  if (!acceptRustTypeNamespaceAnchor(parents > path.length)) fail('spelling traverses above its crate root');
}

/** Use the existing anchored, filtered lookup over actual checked declarations. */
export function validateRustTypeNamespace(program) {
  const selected = [...program.declarations.values()].some(entry => Object.hasOwn(entry, 'sourceTypeNamespace'));
  if (!selected) return;
  if (program.sourceLanguage !== RUST_TYPE_NAMESPACE_POLICY.language || program.imports?.length || program.externals?.length) fail('requires a Rust program without imported aliases or externals');
  const catalog = new Map();
  for (const [name, declaration] of program.declarations) {
    catalog.set(name, { k: declaration.k });
    if (declaration.k === 'data') for (const constructor of declaration.ctors) {
      const key = `${name}.${constructor.name}`;
      if (catalog.has(key) || program.declarations.has(key)) fail('cannot admit a constructor-overwritten declaration identity');
      catalog.set(key, { k: 'ctor' });
    }
  }
  const modules = [...program.items];
  for (let index = 0; index < modules.length; index += 1) {
    const item = modules[index];
    if (item.k === 'module') {
      if (catalog.has(item.fullName)) fail('cannot admit a shared module/declaration identity');
      catalog.set(item.fullName, { k: 'module' }); modules.push(...item.items);
    }
  }
  const origins = new RustSourceOrigins(catalog);
  const spelling = value => Array.isArray(value) && value.length > 0 && value.every(part => typeof part === 'string' && part.length > 0);
  const position = (source, signature, path, result) => {
    if (!spelling(source) || !acceptRustTypeNamespacePosition(signature.owner.kind,
      signature.wrappers.some(wrapper => !RUST_TYPE_NAMESPACE_POLICY.parameterWrappers.includes(wrapper)), result && signature.wrappers.length > 0)) fail('position needs an actual primitive or datatype spelling and a proved direct result');
    requireRustTypeNamespaceAnchor(source, path);
    const declared = origins.lookup(source, path, RUST_SOURCE_SIGNATURE_POLICY.typeOwnerKinds);
    if (declared) {
      if (!acceptRustTypeNamespaceDeclaration(declared.k) || signature.owner.kind !== 'declaration' || declared.fullName !== signature.owner.path.join('.')) fail('spelling does not target its actual scoped datatype declaration');
    } else if (signature.owner.kind !== 'primitive' || source.length !== 1 || signature.owner.path.length !== 1 || source[0] !== signature.owner.path[0]) fail('spelling has no actual primitive type binding');
  };
  for (const declaration of program.declarations.values()) {
    if (declaration.k !== 'fn') continue;
    if (!acceptRustTypeNamespaceFields(Object.keys(declaration).some(field => !RUST_TYPE_NAMESPACE_POLICY.declarationFields.includes(field)))) fail('function contains unrepresented interface fields');
    if (Object.hasOwn(declaration, 'generated') && declaration.generated !== false) fail('requires original declarations with absent or false generated facts');
    for (const parameter of declaration.params) if (!acceptRustTypeNamespaceFields(Object.keys(parameter).some(field => !RUST_TYPE_NAMESPACE_POLICY.parameterFields.includes(field))) || parameter.guard != null || parameter.default != null) fail('parameter contains unrepresented interface fields or defaults');
    const proof = declaration.sourceTypeNamespace;
    const signature = declaration.sourceSignature;
    if (!proof || typeof proof !== 'object' || Array.isArray(proof) || Object.keys(proof).length !== RUST_TYPE_NAMESPACE_POLICY.fields.length || RUST_TYPE_NAMESPACE_POLICY.fields.some(field => !Object.hasOwn(proof, field)) || !acceptRustTypeNamespaceModel(proof.model) || !proof.model || proof.language !== RUST_TYPE_NAMESPACE_POLICY.language) fail('has malformed or missing selected facts');
    if (!signature || !Array.isArray(proof.parameters) || proof.parameters.length !== signature.parameters.length) fail('must retain its owned source signature and ordered parameter positions');
    proof.parameters.forEach((source, index) => position(source, signature.parameters[index], declaration.modulePath, false));
    position(proof.result, signature.result, declaration.modulePath, true);
  }
}

export function requireRustTypeNamespaceWithoutImports(model) {
  if (model) throw unsupported('Rust type namespace imports', 'imported aliases require a separate source binding proof');
}
