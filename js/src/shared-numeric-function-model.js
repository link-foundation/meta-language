// The profile guard is separate from formal graph comparison. Language is
// irrelevant only to this closed value interpretation, never by default.
import { decodeTypedProgramLinks, TYPED_PROGRAM_LINKS_SCHEMA } from './typed-program-links.js';
import { acceptSharedBooleanTypeShape, acceptSharedBooleanBinary, inferSharedBooleanBinaryDomain, acceptSharedNumericSourceSignature, acceptSharedNumericSourceType, acceptSharedNumericTypeShape, acceptSharedNumericContext, acceptSharedNumericContextFlag, acceptSharedNumericDeclaration, acceptSharedNumericParameter, acceptSharedNumericEntry, acceptSharedNumericExpressionShape, acceptSharedNumericLiteral, acceptSharedNumericUnary, acceptSharedNumericBinary } from './shared-numeric-function-rules.js';
import { LinkNetwork } from './network.js';
import { LinkId } from './primitives.js';
import { readRuntimeProjectData } from './project-data-loader.js';
import { checkRustConstFunctionProof } from './translation/rust-const-functions.js';
import { acceptBooleanDeclarationValueContext, acceptBooleanDeclarationValueEntry, acceptBooleanDeclarationValueSelector } from './boolean-declaration-value-rules.js';

export const SHARED_NUMERIC_FUNCTION_MODEL = 'closed binary floating point function';
export const SHARED_BOOLEAN_FUNCTION_MODEL = 'closed boolean function value';
export const BOOLEAN_FUNCTION_DECLARATION_VALUE_MODEL = 'closed boolean function declaration value';
const key = id => LinkId.from(id).asU64();
const fail = message => { throw new TypeError(`shared numeric function: ${message}`); };
const modelDeclaration = readRuntimeProjectData(new URL('./data/formal-concept-models.lino', import.meta.url))[SHARED_NUMERIC_FUNCTION_MODEL];
const numericText = new RegExp(modelDeclaration.numericLiteralPattern, 'u');
const canonicalNumber = value => Object.is(value, -0) ? '-0' : String(value);
function type(type, booleanOnly = false) {
  if (booleanOnly && (!type || !acceptSharedBooleanTypeShape(type.kind, Object.keys(type).length === 1))) fail('only Boolean values belong to the callable model');
  if (!type || !acceptSharedNumericTypeShape(type.kind, Object.keys(type).length === 1)) fail('only Boolean and binary64 value types are admitted');
}

/** Validate all observable operations before excluding a language premise. */
export function readSharedNumericFunctionModel(network, root, { canonical = false } = {}) {
  return readSharedFunctionValueModel(network, root, canonical, false);
}

/** Validate the distinct Boolean callable model without admitting numeric values. */
export function readSharedBooleanFunctionModel(network, root) {
  return readSharedFunctionValueModel(network, root, false, true);
}

/** Validate an actual owned entry, without constructing or qualifying a wrapper. */
export function readBooleanFunctionDeclarationValueModel(network, root, entryDeclaration) {
  return readSharedFunctionValueModel(network, root, false, true, LinkId.from(entryDeclaration));
}

function readSharedFunctionValueModel(network, root, canonical, booleanOnly, entryDeclaration) {
  const program = decodeTypedProgramLinks(network, root);
  if (!acceptSharedNumericContext(program.sourceLanguage, Boolean(program.externals?.length || program.imports?.length || program.main?.effects?.length
    || ['outputThreaded', 'abortsThreaded', 'referenceSemantics', 'utf16Strings'].some(name => !acceptSharedNumericContextFlag(Object.hasOwn(program, name), typeof program[name] === 'boolean', program[name] === false))))) fail('effects, imports and reference interpretations are outside the model');
  let entry;
  if (entryDeclaration !== undefined) {
    const constProof = checkRustConstFunctionProof(program);
    if (!acceptBooleanDeclarationValueContext(program.sourceLanguage, constProof)) fail('the direct declaration model requires JavaScript or the complete original Rust const proof');
    const fields = new Map();
    for (const relation of network.fieldRelations()) {
      if (!fields.has(key(relation.parent))) fields.set(key(relation.parent), []);
      fields.get(key(relation.parent)).push(relation);
    }
    const body = network.link(root).references()[0];
    const directory = fields.get(key(body)).find(field => field.label === TYPED_PROGRAM_LINKS_SCHEMA.fields.declarations).child;
    const selected = network.link(directory).references().find(id => network.link(id).references()[1].equals(entryDeclaration));
    if (!acceptBooleanDeclarationValueSelector(true, selected !== undefined)) fail('the selected identity is not an actual owned declaration');
    const name = network.link(network.link(selected).references()[0]).metadata().definition;
    entry = program.declarations.get(name);
    if (!entry || !acceptBooleanDeclarationValueEntry(entry.k, Boolean(entry.modulePath?.length), Boolean(entry.generated))) fail('the selected value needs an original flat function declaration');
  } else {
    entry = program.declarations.get('selected_function_definition');
    if (!entry || !acceptSharedNumericEntry(Boolean(entry.generated), entry.body.k, program.declarations.has(entry.body.fn))) fail('a checked selected-function wrapper is required');
    // The original wrapper profile is unchanged.
  }
  const expressions = []; const visited = new WeakSet();
  for (const declaration of program.declarations.values()) {
    if (!acceptSharedNumericDeclaration(declaration.k, Boolean(declaration.modulePath?.length))) fail('only flat closed function declarations are admitted');
    if (program.sourceLanguage === 'Rust') {
      const signature = declaration.sourceSignature;
      if (Object.hasOwn(declaration, 'sourceSignature') && !signature) fail('a present source signature must be an owned structured facet');
      if (!signature && (declaration !== entry || entryDeclaration !== undefined)) fail('a Rust function needs its owned source signature facet');
      if (signature) {
        if (!acceptSharedNumericSourceSignature(signature.model, signature.language, signature.parameters?.length === declaration.params.length, Boolean(signature.lifetimeBindings?.length))) fail('a Rust source signature has an unsupported model or layout');
        for (const [descriptor, expected] of [...signature.parameters.map((value, index) => [value, declaration.params[index].type]), [signature.result, declaration.ret]]) {
          type(descriptor.type, booleanOnly);
          if (descriptor.type.kind !== expected.kind || !acceptSharedNumericSourceType(descriptor.type.kind, descriptor.owner.kind, descriptor.owner.path.length === 1 ? descriptor.owner.path[0] : '', Boolean(descriptor.wrappers.length || descriptor.lifetimes.length || descriptor.owner.arguments.length))) fail('a Rust source signature requires direct primitive f64 or Boolean values');
        }
      }
    } else if (declaration.sourceSignature !== undefined) fail('a foreign source signature is outside the JavaScript value premise');
    type(declaration.ret, booleanOnly);
    for (const parameter of declaration.params) {
      type(parameter.type, booleanOnly);
      if (!acceptSharedNumericParameter(parameter.guard != null, parameter.default != null)) fail('guarded and default parameters need a separate interpretation');
    }
    expressions.push(declaration.body);
  }
  for (let at = 0; at < expressions.length; at += 1) {
    const node = expressions[at];
    if (!node || typeof node !== 'object') fail('missing checked expression');
    if (visited.has(node)) continue;
    if (at > 100000) fail('expression resource limit exceeded');
    visited.add(node); type(node.type, booleanOnly);
    if (!acceptSharedNumericExpressionShape(node.k, node.rounding != null || node.byZero != null)) fail('an operation has no shared numeric interpretation');
    if (node.k === 'lit') {
      const decimal = typeof node.value === 'string' && numericText.exec(node.value)?.[0] === node.value;
      if (!acceptSharedNumericLiteral(node.type.kind, typeof node.value === 'boolean', decimal, Number.isFinite(Number(node.value)), !canonical || canonicalNumber(Number(node.value)) === node.value, canonical)) fail('a literal is not a Boolean or canonical finite binary64');
    } else if (node.k === 'unary') {
      if (!acceptSharedNumericUnary(node.op, node.type.kind, node.semantics ?? '')) fail('unsupported unary primitive');
      expressions.push(node.arg);
    } else if (node.k === 'binary') {
      const domain = node.domain ?? (booleanOnly && inferSharedBooleanBinaryDomain(node.op) ? node.type : null);
      type(domain, booleanOnly);
      const accept = booleanOnly ? acceptSharedBooleanBinary : acceptSharedNumericBinary;
      if (!accept(node.op, domain.kind, node.semantics ?? '')) fail('unsupported binary primitive');
      expressions.push(node.left, node.right);
    } else if (node.k === 'let') expressions.push(node.value, node.body);
    else if (node.k === 'if') expressions.push(node.cond, node.then, node.else);
    else if (node.k === 'call') {
      if (!program.declarations.has(node.fn)) fail('a call captures an external interpretation');
      expressions.push(...node.args);
    }
  }
  const fields = new Map();
  for (const relation of network.fieldRelations()) {
    if (!fields.has(key(relation.parent))) fields.set(key(relation.parent), []);
    fields.get(key(relation.parent)).push(relation);
  }
  const body = network.link(root).references()[0];
  const sourceLanguage = fields.get(key(body)).find(relation => relation.label === TYPED_PROGRAM_LINKS_SCHEMA.fields.sourceLanguage);
  if (!sourceLanguage) fail('missing source-language premise');
  const directory = fields.get(key(body)).find(field => field.label === TYPED_PROGRAM_LINKS_SCHEMA.fields.declarations).child;
  const declarationIds = new Set(network.link(directory).references().map(id => key(network.link(id).references()[1])));
  let entryTarget;
  if (entryDeclaration !== undefined) {
    if (!acceptBooleanDeclarationValueSelector(true, declarationIds.has(key(entryDeclaration)))) fail('the selected identity is not an actual owned declaration');
    entryTarget = entryDeclaration;
  } else {
    const entryId = network.link(network.link(directory).references().find(id => network.link(network.link(id).references()[0]).metadata().definition === 'selected_function_definition')).references()[1];
    const child = (id, name) => fields.get(key(id))?.find(field => field.label === (TYPED_PROGRAM_LINKS_SCHEMA.fields[name] ?? name))?.child;
    const entryBody = child(entryId, 'body');
    const entryParams = network.link(child(entryId, 'params')).references();
    const entryArguments = network.link(child(entryBody, 'args')).references();
    entryTarget = child(entryBody, TYPED_PROGRAM_LINKS_SCHEMA.references.declaration);
    if (!entryTarget || entryTarget.equals(entryId) || entryArguments.length !== entryParams.length
      || entryArguments.some((argument, index) => !child(argument, TYPED_PROGRAM_LINKS_SCHEMA.references.binding)?.equals(entryParams[index]))) fail('the selected wrapper must pass each owned parameter to a distinct declared function');
  }
  const reached = new Set(); const pending = [root]; const literals = []; const signatureFields = []; const qualificationFields = [];
  for (let at = 0; at < pending.length; at += 1) {
    const id = key(pending[at]); if (reached.has(id)) continue;
    if (pending.length > 100000) fail('reference resource limit exceeded');
    reached.add(id); pending.push(...network.link(id).references());
    const relations = fields.get(id) ?? [];
    for (const relation of relations) {
      if (!['source span', 'provenance'].includes(relation.label)) pending.push(relation.child);
      if (['type', 'domain', 'return type'].includes(relation.label)) {
        const typeFields = fields.get(key(relation.child)) ?? [];
        const typeKind = typeFields.find(field => field.label === 'kind');
        const accept = booleanOnly ? acceptSharedBooleanTypeShape : acceptSharedNumericTypeShape;
        if (!accept(typeKind ? network.link(typeKind.child).metadata().definition : '', typeFields.length === 1)) fail('only Boolean and binary64 value types are admitted');
      }
    }
    const signature = relations.find(relation => relation.label === TYPED_PROGRAM_LINKS_SCHEMA.fields.sourceSignature);
    if (signature) {
      if (!declarationIds.has(id)) fail('a source signature belongs only to its owned declaration');
      signatureFields.push(signature);
    }
    if (entryDeclaration !== undefined) {
      const qualification = relations.find(relation => relation.label === TYPED_PROGRAM_LINKS_SCHEMA.fields.sourceConstFunction);
      if (qualification) {
        if (!declarationIds.has(id)) fail('a const qualification belongs only to its original owned declaration');
        qualificationFields.push(qualification);
      }
    }
    const kind = relations.find(relation => relation.label === TYPED_PROGRAM_LINKS_SCHEMA.fields.k);
    const literal = relations.find(relation => relation.label === (TYPED_PROGRAM_LINKS_SCHEMA.fields.value ?? 'value'));
    if (kind && literal && network.link(kind.child).metadata().definition === 'lit' && network.link(literal.child).metadata().term === TYPED_PROGRAM_LINKS_SCHEMA.scalars.string) literals.push(literal);
  }
  return { sourceLanguage: program.sourceLanguage, languageParent: body, languageLabel: sourceLanguage.label, literals, signatureFields, entryTarget, declarationIds, ...(entryDeclaration === undefined ? {} : { qualificationFields }) };
}

/** Read normalized numeric values while preserving all actual literal text. */
export function normalizeSharedNumericFunctionLiterals(network, root) {
  return readSharedNumericFunctionModel(network, root);
}

/** Build the model's validated IEEE value view without altering the archive. */
export function createSharedNumericFunctionMeaning(network, model) {
  const meaning = LinkNetwork.fromLino(network.toLino());
  const replacements = new Map();
  const bytes = new DataView(new ArrayBuffer(8));
  for (const relation of model.literals) {
    const original = network.link(relation.child);
    bytes.setFloat64(0, Number(original.metadata().definition), false);
    const bits = bytes.getBigUint64(0, false).toString(16).padStart(16, '0');
    if (!replacements.has(key(relation.child))) replacements.set(key(relation.child), meaning.insertLink([], original.metadata().withLanguage('meta-language formal definition').withTerm('binary floating point bit pattern').withDefinition(bits)));
    const link = meaning.link(relation.link.id());
    const references = link.references(); references[2] = replacements.get(key(relation.child)); link.setReferences(references);
  }
  return meaning;
}
