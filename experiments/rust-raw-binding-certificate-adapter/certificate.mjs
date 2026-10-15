// Thin first-class certificate bridge around the unchanged complete source model.
import { readRuntimeProjectData } from '../../js/src/project-data-loader.js';
import { checkOwnedProgramContainers } from '../../js/src/translation/owned-program-containers.js';
import { acceptRustOwnedLocalContainer } from '../../js/src/translation/rust-owned-local-state-rules.js';
import { withParts } from '../../js/src/translation/emit-common.js';
import {
  checkGeneratedOwnedRustData as data, checkGeneratedOwnedRustFields as fields,
} from '../generated-owned-rust/model.mjs';
import {
  RUST_RAW_BINDING_IDENTIFIER_POLICY,
  createRustRawBindingIdentifiers, checkRustRawBindingIdentifiers,
  checkRustRawBindingIdentifierInvocation, emitRustRawBindingIdentifiers,
} from '../rust-raw-binding-identifiers/model.mjs';
import { readRawBindingIdentifierSyntax } from '../rust-raw-binding-identifiers/syntax.mjs';
import {
  acceptRustRawBindingCertificateSource, acceptRustRawBindingCertificateContext,
  acceptRustRawBindingCertificateRoots, acceptRustRawBindingCertificateOwners,
} from './rules.mjs';

export const RAW_BINDING_CERTIFICATE_POLICY = readRuntimeProjectData(new URL('./data/policy.lino', import.meta.url));
const policy = RAW_BINDING_CERTIFICATE_POLICY;
const fail = reason => { throw new TypeError('Experimental program-owned Rust raw binding certificate: ' + reason); };
const plainRecord = value => value !== null && typeof value === 'object'
  && [Object.prototype, null].includes(Object.getPrototypeOf(value));

export function readRustRawBindingCertificate(program) {
  // The shared guard rejects non-data roots before its own-descriptor selector.
  const selected = checkOwnedProgramContainers(program, {
    selectedFields: [policy.facetField, policy.contextField], acceptContainer: acceptRustOwnedLocalContainer, fail,
  });
  const mainDescriptor = Object.getOwnPropertyDescriptor(program, 'main');
  if (mainDescriptor && (!Object.hasOwn(mainDescriptor, 'value') || !plainRecord(mainDescriptor.value))) fail('the actual main owner must be a plain data record');
  if (!selected) return null;
  // Descriptor validation precedes every selected graph read, including when
  // a fact was misplaced, hidden or replaced by an accessor.
  data(program, {});
  const facet = program?.[policy.facetField], context = program?.main?.[policy.contextField];
  if (!acceptRustRawBindingCertificateRoots(Object.hasOwn(program, policy.facetField), Boolean(program.main && Object.hasOwn(program.main, policy.contextField)))) fail('the program facet and actual main source context are both required');
  fields(context, policy.contextFields);
  if (!acceptRustRawBindingCertificateContext(context.model)) fail('the actual main context model is unknown');
  if (!facet || !acceptRustRawBindingCertificateOwners(facet.worker === program.main, context.sourceExtent === facet.span)) fail('the certificate lost its actual main or exact source extent identity');
  const core = { ...program, main: { ...program.main } };
  delete core[policy.facetField]; delete core.main[policy.contextField];
  // Only the main adapter record is projected. Every actual declaration,
  // parameter, expression and ordered parameter array keeps its identity.
  const proof = { ...facet, worker: core.main };
  checkRustRawBindingIdentifiers(core, proof);
  return { program, facet, core, proof };
}

export function captureRustRawBindingCertificate(source, language, options) {
  const independent = policy.sourceOptions.every(name => options[name] === undefined)
    && !options.utf16Strings && options.context === undefined;
  if (!acceptRustRawBindingCertificateSource(language, independent)) fail('the candidate source model requires independent Rust primitive source');
  const { program, proof } = createRustRawBindingIdentifiers(source, { model: options[policy.sourceOption] });
  program[policy.facetField] = proof;
  program.main[policy.contextField] = { model: policy.contextModel, sourceExtent: proof.span };
  readRustRawBindingCertificate(program);
  return { program, provenance: {
    sourceProfile: { language, [policy.sourceOption]: options[policy.sourceOption] },
    userProvenance: options.provenance ?? null,
  } };
}

export function checkRustRawBindingCertificateInvocation(program, declaration, args) {
  const selected = readRustRawBindingCertificate(program);
  if (!selected) fail('an invocation requires the actual program-owned certificate');
  return checkRustRawBindingIdentifierInvocation(selected.core, selected.proof, declaration, args);
}

function assumptions(selected) {
  return selected.facet.inputAssumptions.map(premise => ({
    id: policy.inputAssumption.id,
    statement: policy.inputAssumption.statement,
    details: ['actual declaration ' + premise.declaration.fullName + ' at ' + premise.declaration.span.start + '..' + premise.declaration.span.end,
      'actual ordered parameter owners: ' + premise.parameterOwners.map((parameter, index) => index + ':' + parameter.name).join(', ')],
  }));
}
export function decorateRustRawBindingCertificateEmission(selected, emitted, options = {}) {
  if (policy.targetOptions.some(name => options[name] !== undefined)) fail('target profile composition needs an independent certificate');
  return withParts({ ...emitted, assumptions: [...emitted.assumptions, ...assumptions(selected)], encodings: [...emitted.encodings, policy.encoding] }, emitted.preludes, emitted.definitions);
}
export function emitRustRawBindingCertificate(selected, options = {}) {
  if (policy.targetOptions.some(name => options[name] !== undefined)) fail('target profile composition needs an independent certificate');
  const text = emitRustRawBindingIdentifiers(selected.core, selected.proof, 'Rust').text;
  // Reparse the whole emitted module before deriving nonserialized definition
  // parts. No selected function text is supplied to any source parser.
  const rebound = createRustRawBindingIdentifiers(text);
  const syntax = readRawBindingIdentifierSyntax(rebound.proof, RUST_RAW_BINDING_IDENTIFIER_POLICY, rebound.program);
  const definitions = rebound.program.items.map(declaration => {
    const index = rebound.proof.tokens.findIndex(token => token.span.start === declaration.span.start && token.raw === 'fn');
    if (index < 0) fail('an emitted declaration lost its actual whole-source header');
    const previous = rebound.proof.tokens[index - 1];
    const start = previous?.raw === 'pub' ? previous.span.start : declaration.span.start;
    return text.slice(start, declaration.span.end);
  });
  return withParts({ language: 'Rust', text, entry: 'main',
    mappings: selected.core.items.map((declaration, index) => ({ kind: 'function', source: declaration.fullName,
      target: syntax.functions[index].target.group.tokens.map(token => token.raw).join(''), sourceSpan: { ...declaration.span } })),
    assumptions: assumptions(selected), encodings: [policy.encoding], theorems: [],
  }, [], definitions);
}
