// Finite operational definitions for the bounded record-text draft. A proof
// concerns the complete recursive algorithm, closed input tape and selected
// model. It does not prove the ECMAScript or Node specifications in general.
import { readFileSync } from 'node:fs';
import { LinkNetwork } from '../js/src/network.js';
import {
  encodeFormalConceptDefinition, proveFormalConceptEquivalence,
  readFormalConceptDefinition, verifyFormalConceptEquivalence,
} from '../js/src/formal-concept-definition.js';
import { createTypedProgramLinks, decodeTypedProgramLinks } from '../js/src/typed-program-links.js';
import { recordTextModels, validateRecordTextDraft } from './issue-199-record-text-draft.mjs';

// This is the library's supported operational basis, analogous to the formal
// primitive registry. Reload never needs the original object or input source.
const rules = readFileSync(new URL('./issue-199-record-text-rules.mjs', import.meta.url), 'utf8');
const entry = 'readRecordConversionMeaning';
const requireModel = model => {
  if (!recordTextModels.includes(model)) throw new TypeError('a closed record definition needs a declared conversion model');
};
const archive = value => ({
  network: value.network ?? LinkNetwork.fromLino(value.links), root: value.root,
});

/** Keep the selected primitive interpretation as an executable semantic input. */
export function createClosedRecordConversionDefinition(tape, model) {
  validateRecordTextDraft(tape);
  requireModel(model);
  // JavaScript literal syntax is only the translator's input boundary. The
  // resulting archive contains typed ordered arrays and scalar links, with
  // neither source objects nor an opaque source-text envelope.
  const argumentsText = [tape.kinds, tape.keys, tape.values, tape.ends, model].map(value => JSON.stringify(value)).join(', ');
  const source = `${rules}\n/** @returns {string} */\nexport function ${entry}() { return renderRecordText(${argumentsText}); }\n`;
  return encodeFormalConceptDefinition(createTypedProgramLinks(source, 'JavaScript', {
    context: { referenceSemantics: false },
  }));
}

/** Validate the closed input and compare the actual algorithm with its basis. */
export function readClosedRecordConversionDefinition(value) {
  const actual = archive(value);
  if (readFormalConceptDefinition(actual).model !== 'checked typed program') throw new TypeError('a closed record definition needs the checked operational basis');
  const body = actual.network.link(actual.root).references()[0];
  const program = decodeTypedProgramLinks(actual.network, body);
  const declaration = program.declarations.get(entry);
  const invocation = declaration?.body;
  if (declaration?.params.length !== 0 || invocation?.k !== 'call'
    || invocation.fn !== 'renderRecordText' || invocation.args.length !== 5) {
    throw new TypeError('a closed record definition needs its complete conversion invocation');
  }
  const [kinds, keys, values, ends] = invocation.args.slice(0, 4).map((argument, index) => {
    const expectedKind = index === 3 ? 'float' : 'string';
    if (argument.k !== 'array' || argument.items.some(item => item.k !== 'lit' || item.type.kind !== expectedKind)) {
      throw new TypeError('a closed record definition needs literal ordered input arrays');
    }
    return argument.items.map(item => index === 3 ? Number(item.value) : item.value);
  });
  const selector = invocation.args[4];
  if (selector.k !== 'lit' || selector.type.kind !== 'string') throw new TypeError('a closed record definition needs its literal model premise');
  const model = selector.value;
  requireModel(model);
  const tape = { schemaVersion: 1, model: 'ordinary ASCII record tree', kinds, keys, values, ends };
  validateRecordTextDraft(tape);
  const expected = createClosedRecordConversionDefinition(tape, model);
  if (!proveFormalConceptEquivalence(actual, expected)) throw new TypeError('a closed record definition differs from its supported recursive algorithm');
  return { model, tape };
}

/** Unknown or counterfeit premises never become an equivalence certificate. */
export function proveClosedRecordConversionEquivalence(first, second) {
  try {
    readClosedRecordConversionDefinition(first);
    readClosedRecordConversionDefinition(second);
    return proveFormalConceptEquivalence(first, second);
  } catch { return null; }
}

export function verifyClosedRecordConversionEquivalence(first, second, certificate) {
  try {
    readClosedRecordConversionDefinition(first);
    readClosedRecordConversionDefinition(second);
    return verifyFormalConceptEquivalence(first, second, certificate);
  } catch { return false; }
}
