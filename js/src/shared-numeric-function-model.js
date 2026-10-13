// The profile guard is separate from formal graph comparison. Language is
// irrelevant only to this closed value interpretation, never by default.
import { decodeTypedProgramLinks, TYPED_PROGRAM_LINKS_SCHEMA } from './typed-program-links.js';
import { acceptSharedNumericTypeShape, acceptSharedNumericContext, acceptSharedNumericContextFlag, acceptSharedNumericDeclaration, acceptSharedNumericParameter, acceptSharedNumericEntry, acceptSharedNumericExpressionShape, acceptSharedNumericLiteral, acceptSharedNumericUnary, acceptSharedNumericBinary } from './shared-numeric-function-rules.js';
import { LinkId } from './primitives.js';
import { readRuntimeProjectData } from './project-data-loader.js';

export const SHARED_NUMERIC_FUNCTION_MODEL = 'closed binary floating point function';
const key = id => LinkId.from(id).asU64();
const fail = message => { throw new TypeError(`shared numeric function: ${message}`); };
const modelDeclaration = readRuntimeProjectData(new URL('./data/formal-concept-models.lino', import.meta.url))[SHARED_NUMERIC_FUNCTION_MODEL];
const numericText = new RegExp(modelDeclaration.numericLiteralPattern, 'u');
const canonicalNumber = value => Object.is(value, -0) ? '-0' : String(value);
function type(type) {
  if (!type || !acceptSharedNumericTypeShape(type.kind, Object.keys(type).length === 1)) fail('only Boolean and binary64 value types are admitted');
}

/** Validate all observable operations before excluding a language premise. */
export function readSharedNumericFunctionModel(network, root, { canonical = true } = {}) {
  const program = decodeTypedProgramLinks(network, root);
  if (!acceptSharedNumericContext(program.sourceLanguage, Boolean(program.externals?.length || program.imports?.length || program.main?.effects?.length
    || ['outputThreaded', 'abortsThreaded', 'referenceSemantics', 'utf16Strings'].some(name => !acceptSharedNumericContextFlag(Object.hasOwn(program, name), typeof program[name] === 'boolean', program[name] === false))))) fail('effects, imports and reference interpretations are outside the model');
  const entry = program.declarations.get('selected_function_definition');
  if (!entry || !acceptSharedNumericEntry(Boolean(entry.generated), entry.body.k, program.declarations.has(entry.body.fn))) fail('a checked selected-function wrapper is required');
  const expressions = []; const visited = new WeakSet();
  for (const declaration of program.declarations.values()) {
    if (!acceptSharedNumericDeclaration(declaration.k, Boolean(declaration.modulePath?.length))) fail('only flat closed function declarations are admitted');
    type(declaration.ret);
    for (const parameter of declaration.params) {
      type(parameter.type);
      if (!acceptSharedNumericParameter(parameter.guard != null, parameter.default != null)) fail('guarded and default parameters need a separate interpretation');
    }
    expressions.push(declaration.body);
  }
  for (let at = 0; at < expressions.length; at += 1) {
    const node = expressions[at];
    if (!node || typeof node !== 'object') fail('missing checked expression');
    if (visited.has(node)) continue;
    if (at > 100000) fail('expression resource limit exceeded');
    visited.add(node); type(node.type);
    if (!acceptSharedNumericExpressionShape(node.k, node.rounding != null || node.byZero != null)) fail('an operation has no shared numeric interpretation');
    if (node.k === 'lit') {
      const decimal = typeof node.value === 'string' && numericText.exec(node.value)?.[0] === node.value;
      if (!acceptSharedNumericLiteral(node.type.kind, typeof node.value === 'boolean', decimal, Number.isFinite(Number(node.value)), canonicalNumber(Number(node.value)) === node.value, canonical)) fail('a literal is not a Boolean or canonical finite binary64');
    } else if (node.k === 'unary') {
      if (!acceptSharedNumericUnary(node.op, node.type.kind, node.semantics ?? '')) fail('unsupported unary primitive');
      expressions.push(node.arg);
    } else if (node.k === 'binary') {
      type(node.domain);
      if (!acceptSharedNumericBinary(node.op, node.domain.kind, node.semantics ?? '')) fail('unsupported binary primitive');
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
  const reached = new Set(); const pending = [root]; const literals = [];
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
        if (!acceptSharedNumericTypeShape(typeKind ? network.link(typeKind.child).metadata().definition : '', typeFields.length === 1)) fail('only Boolean and binary64 value types are admitted');
      }
    }
    const kind = relations.find(relation => relation.label === TYPED_PROGRAM_LINKS_SCHEMA.fields.k);
    const literal = relations.find(relation => relation.label === (TYPED_PROGRAM_LINKS_SCHEMA.fields.value ?? 'value'));
    if (kind && literal && network.link(kind.child).metadata().definition === 'lit' && network.link(literal.child).metadata().term === TYPED_PROGRAM_LINKS_SCHEMA.scalars.string) literals.push(literal);
  }
  return { sourceLanguage: program.sourceLanguage, languageParent: body, languageLabel: sourceLanguage.label, literals };
}

/** Canonicalize spelling only at an already checked floating literal relation. */
export function normalizeSharedNumericFunctionLiterals(network, root) {
  const model = readSharedNumericFunctionModel(network, root, { canonical: false });
  const replacements = new Map();
  for (const relation of model.literals) {
    const original = network.link(relation.child);
    const value = canonicalNumber(Number(original.metadata().definition));
    if (value === original.metadata().definition) continue;
    if (!replacements.has(key(relation.child))) replacements.set(key(relation.child), network.insertLink([], original.metadata().withDefinition(value)));
    const references = relation.link.references(); references[2] = replacements.get(key(relation.child)); relation.link.setReferences(references);
  }
  return readSharedNumericFunctionModel(network, root);
}
