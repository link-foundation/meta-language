// Explicit experimental validation owns this provenance; public APIs keep their defaults.
import { isDeepStrictEqual } from 'node:util';
import { readRuntimeProjectData } from '../../js/src/project-data-loader.js';
import { tokenize } from '../../js/src/translation/lexer.js';
import { parseRustTokenStream } from '../../js/src/translation/rust.js';
import { checkProgram } from '../../js/src/translation/check.js';
import { emitJavaScript } from '../../js/src/translation/emit-javascript.js';
import { checkProgramDeclarationOwners } from '../../js/src/typed-program-declaration-owners.js';
import {
  checkGeneratedOwnedRustData as data, checkGeneratedOwnedRustFields as fields,
  checkGeneratedOwnedRustInterval as interval, checkGeneratedOwnedRustCst as cst,
  mapGeneratedOwnedRustSurface as surfaceMap,
} from '../generated-owned-rust/model.mjs';
import { encodeGeneratedOwnedRust as encode, decodeGeneratedOwnedRust as decode } from '../generated-owned-rust/codec.mjs';
import { acceptRustRawBindingIdentifierModel, acceptRustRawBindingIdentifierType, acceptRustRawBindingIdentifierClosure } from './rules.mjs';
import { fail, validName, readRawBindingIdentifierSyntax, renameRawBindingIdentifierSurface } from './syntax.mjs';

export const RUST_RAW_BINDING_IDENTIFIER_POLICY = readRuntimeProjectData(new URL('./data/policy.lino', import.meta.url));
const policy = RUST_RAW_BINDING_IDENTIFIER_POLICY;
const sourceText = proof => proof.tokens.map((token, index) => proof.gaps[index].units.map(unit => String.fromCharCode(unit)).join('') + token.raw).join('');
const primitive = type => type && acceptRustRawBindingIdentifierType(type.kind, type.signed ?? false, type.bits ?? 0);
function parse(proof, program = null) {
  const plan = readRawBindingIdentifierSyntax(proof, policy, program);
  const surface = renameRawBindingIdentifierSurface(parseRustTokenStream(plan.tokens), plan);
  const expected = checkProgram(surface);
  if (expected.items.some(item => !acceptRustRawBindingIdentifierClosure(Boolean(item.recursive), Boolean(item.mutual)))) fail('recursive calls require an independent observation proof');
  return { plan, surface, expected };
}
function occurrences(surface, program, plan) {
  const mapped = surfaceMap(surface, program), sites = new Map();
  for (const [group, descriptor] of plan.assignments) sites.set(group.tokens[0].span.start, { group, descriptor });
  const result = [];
  const occurrence = (source, node) => {
    const site = sites.get(source.span?.start);
    if (!site || !node || !primitive(node.type)) fail('an occurrence lost its actual source or primitive expression');
    const { group, descriptor } = site;
    if (descriptor.kind === 'parameter' ? node.k !== 'var' || node.name !== descriptor.semanticName : node.k !== 'call' || node.fn !== descriptor.semanticName) fail('an occurrence differs from its actual inverse owner');
    result.push({ token: group.tail, alias: descriptor.record, checkedExpression: node });
  };
  function walk(source) {
    const node = mapped.get(source);
    if (source.k === 'name') occurrence(source, node);
    else if (source.k === 'app') { occurrence(source.fn, node); source.args.forEach(walk); }
    else if (!['num', 'bool'].includes(source.k) || !primitive(node?.type)) fail('a body lost its closed primitive expression interpretation');
  }
  for (const item of surface.items) walk(item.body);
  return result;
}
function check(program, proof) {
  data(program, proof); fields(proof, policy.fields.proof); interval(proof.span, true);
  if (!acceptRustRawBindingIdentifierModel(proof.model) || proof.language !== policy.language || proof.worker !== program.main) fail('the model or actual empty worker owner differs');
  if (!Array.isArray(proof.aliases) || !Array.isArray(proof.occurrences)) fail('the actual alias and occurrence inventories must be ordered arrays');
  checkProgramDeclarationOwners(program); cst(proof, {});
  const { plan, surface, expected } = parse(proof, program);
  if (!isDeepStrictEqual(program, expected) || !isDeepStrictEqual([...program.declarations.keys()], [...expected.declarations.keys()])) fail('the actual core differs from its entire alias-resolved source');
  const actual = occurrences(surface, program, plan);
  if (actual.length !== proof.occurrences.length) fail('an actual source occurrence was removed or invented');
  for (const [index, row] of proof.occurrences.entries()) {
    fields(row, policy.fields.occurrence);
    if (row.token !== actual[index].token || row.alias !== actual[index].alias || row.checkedExpression !== actual[index].checkedExpression) fail('an occurrence lost its actual token, inverse alias or checked expression identity');
  }
  return { plan, surface };
}
export { check as checkRustRawBindingIdentifiers };

export function createRustRawBindingIdentifiers(source, { model = policy.model } = {}) {
  if (typeof source !== 'string' || !acceptRustRawBindingIdentifierModel(model)) fail('select the explicit experimental model and primitive source string');
  const tokens = [], gaps = [];
  let end = 0;
  for (const token of tokenize(source, 'Rust').tokens) {
    const text = source.slice(end, token.start);
    gaps.push({ span: { start: end, end: token.start }, units: Array.from({ length: text.length }, (_, index) => text.charCodeAt(index)) });
    tokens.push({ kind: token.kind, value: token.value, raw: token.raw, span: { start: token.start, end: token.end },
      ...(token.kind === 'number' ? { suffix: token.suffix ?? '', floating: Boolean(token.floating) } : {}) });
    end = token.end;
  }
  const proof = { model, language: policy.language, span: { start: 0, end: source.length }, tokens, gaps, aliases: [], occurrences: [], worker: null };
  const { plan, expected: program } = parse(proof);
  const entries = new Map(plan.functions.map((row, index) => [row.target, program.items[index]]));
  for (const descriptor of plan.descriptors) {
    let owner;
    if (descriptor.kind === 'declaration') owner = entries.get(descriptor);
    else {
      const row = plan.functions.find(entry => entry.parameters.includes(descriptor));
      owner = entries.get(row.target).params[row.parameters.indexOf(descriptor)];
    }
    const record = { kind: descriptor.kind, value: descriptor.value, checkedOwner: owner, token: descriptor.group.tail };
    proof.aliases.push(record); descriptor.record = record; descriptor.checkedOwner = owner;
  }
  for (const [group, descriptor] of plan.assignments) {
    delete group.tail.value;
    group.tail[descriptor.kind === 'declaration' ? 'declaration' : 'binding'] = descriptor.checkedOwner;
  }
  proof.worker = program.main;
  const bound = parse(proof, program);
  proof.occurrences = occurrences(bound.surface, program, bound.plan);
  check(program, proof); return { program, proof };
}
export function createRustRawBindingIdentifierLinks(source, options) {
  const { program, proof } = createRustRawBindingIdentifiers(source, options);
  return encode(program, proof);
}
export function decodeRustRawBindingIdentifierLinks(network, root) {
  const result = decode(network, root); check(result.program, result.proof); return result;
}
export function encodeRustRawBindingIdentifierLinks(program, proof) { check(program, proof); return encode(program, proof); }

export function emitRustRawBindingIdentifiers(program, proof, target) {
  check(program, proof);
  if (target === 'JavaScript') return { language: target, text: emitJavaScript(program).text };
  if (target !== 'Rust') fail('this experiment has no interpretation for that target');
  const substitutions = new Map(), omitted = new Set();
  for (const group of readRawBindingIdentifierSyntax(proof, policy, program).logical) {
    const owner = group.tail.binding ?? group.tail.declaration;
    if (!owner) continue;
    const name = owner.name;
    if (!validName(name, true)) fail('the actual edited owner cannot be spelled as a raw identifier');
    const text = group.raw || !validName(name, false) ? 'r#' + name : name;
    if (text === group.tokens.map(token => token.raw).join('')) continue;
    substitutions.set(group.tokens[0], text);
    for (const token of group.tokens.slice(1)) omitted.add(token);
  }
  const text = proof.tokens.map((token, index) => proof.gaps[index].units.map(unit => String.fromCharCode(unit)).join('')
    + (omitted.has(token) ? '' : substitutions.get(token) ?? token.raw)).join('');
  // A changed declaration/binding graph earns fresh source extents by running
  // the same complete interpretation, rather than reusing historical spans.
  createRustRawBindingIdentifiers(text, { model: proof.model });
  return { language: target, text };
}
export function reconstructRustRawBindingIdentifierSource(program, proof) { check(program, proof); return sourceText(proof); }
