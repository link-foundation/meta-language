// Evidence of the issue #195 runtime-parity and shared-concept requirements.
//
// The runtime parity check compares the JavaScript runtime's observations with
// the Rust probe's. The comparison alone does not show what it covered, so each
// assertion of a parity requirement is checked separately on each runtime's
// observation: the fixture revision both runtimes read, the completeness of the
// compared CSTs, the flags, spans, trivia, and diagnostics in them, the
// compared bindings, transformations, and translations, the language
// distinctions left after normalization, and a mutation of the requirement's
// own observations that changes no text yet is reported as a mismatch.
//
// The observations are about 100 MB per runtime, most of it the PDF grammar
// cases' trees. The Rust probe streams them as one NDJSON record per section
// entry, sections are compared entry by entry, and the kept evidence is one
// digest per entry with the full entries only where the runtimes differ.
import { createHash } from 'node:crypto';
import { mkdir, open } from 'node:fs/promises';
import path from 'node:path';

export const PARITY_SECTIONS = Object.freeze([
  'schemaVersion',
  'fixtureDigest',
  'positive',
  'negative',
  'inventory',
  'builtins',
  'linoGrammar',
  'pdfGrammar',
  'semantics',
  'diagnostics',
  'bindingRenames',
  'transforms',
  'translations',
]);

const NETWORK_SECTIONS = ['positive', 'negative', 'inventory', 'builtins', 'linoGrammar', 'pdfGrammar'];
const LANGUAGES = ['JavaScript', 'Rust', 'Lean', 'Rocq'];
const PROOF_LANGUAGES = ['Lean', 'Rocq'];
const TEXT_KEYS = new Set([
  'source', 'reconstruction', 'emit', 'replace', 'insert', 'delete', 'clone', 'move',
  'code', 'decodedSource', 'renamed',
]);

const comparisons = new WeakMap();

/**
 * The compared sections whose observations differ between the runtimes. A
 * section is compared entry by entry, which equals comparing its canonical
 * JSON without building that string for the whole section.
 */
export function mismatchedSections(left, right, sections = PARITY_SECTIONS) {
  return sections.filter((section) => !sameCanonical(left[section], right[section]));
}

function sameCanonical(left, right) {
  if (Array.isArray(left) && Array.isArray(right)) {
    return left.length === right.length &&
      left.every((entry, index) => canonicalJson(entry) === canonicalJson(right[index]));
  }
  return canonicalJson(left) === canonicalJson(right);
}

/** The probe's NDJSON records of `observation`: one per entry of a list section, one per other section. */
export function* observationRecords(observation) {
  for (const section of PARITY_SECTIONS) {
    const value = observation[section];
    if (Array.isArray(value)) {
      for (const [index, entry] of value.entries()) yield { section, index, value: entry };
    } else {
      yield { section, index: null, value };
    }
  }
}

/** Rebuilds an observation from its NDJSON records, rejecting unknown sections, repeats and gaps. */
export function observationFromRecords(records) {
  const observation = {};
  for (const { section, index, value } of records) {
    if (!PARITY_SECTIONS.includes(section)) throw new Error(`unknown runtime-parity section ${section}`);
    if (index === null) {
      if (section in observation) throw new Error(`runtime-parity section ${section} is repeated`);
      observation[section] = value;
      continue;
    }
    observation[section] ??= [];
    if (!Array.isArray(observation[section]) || index !== observation[section].length) {
      throw new Error(`runtime-parity section ${section} entry ${index} is out of order`);
    }
    observation[section].push(value);
  }
  // A list section without entries has no records.
  for (const section of PARITY_SECTIONS) observation[section] ??= [];
  return observation;
}

function entryDigest(value) {
  return createHash('sha256').update(canonicalJson(value)).digest('hex');
}

/**
 * The kept runtime-parity evidence: one digest per section entry and runtime,
 * and the full entries of both runtimes only where their digests differ.
 */
export function parityDigests(observations) {
  const digests = [];
  const differences = [];
  for (const { digest, difference } of parityDigestRecords(observations)) {
    digests.push(digest);
    if (difference) differences.push(difference);
  }
  return { digests, differences };
}

/** Yields one digest and optional difference at a time, without accumulating artifact contents. */
export function* parityDigestRecords(observations) {
  for (const section of PARITY_SECTIONS) {
    const values = Object.fromEntries(Object.entries(observations).map(([runtime, observation]) =>
      [runtime, observation[section]]));
    const list = Object.values(values).some(Array.isArray);
    const count = list ? Math.max(...Object.values(values).map((value) => (Array.isArray(value) ? value.length : 0))) : 1;
    for (let index = 0; index < count; index += 1) {
      const entries = Object.fromEntries(Object.entries(values).map(([runtime, value]) =>
        [runtime, list ? (Array.isArray(value) ? value[index] : undefined) : value]));
      const record = {
        section,
        index: list ? index : null,
        ...Object.fromEntries(Object.entries(entries).map(([runtime, entry]) =>
          [runtime, entry === undefined ? null : entryDigest(entry)])),
      };
      let difference;
      if (new Set(Object.keys(entries).map((runtime) => record[runtime])).size > 1) {
        difference = { section, index: record.index, ...Object.fromEntries(
          Object.entries(entries).map(([runtime, entry]) => [runtime, entry ?? null])) };
      }
      yield { digest: record, difference };
    }
  }
}

/** Writes NDJSON evidence with backpressure; even an all-different corpus keeps only one encoded entry. */
export async function writeParityDigestArtifacts(directory, observations) {
  await mkdir(directory, { recursive: true });
  const digests = await open(path.join(directory, PARITY_ARTIFACT_FILES.digests), 'w');
  try {
    const differences = await open(path.join(directory, PARITY_ARTIFACT_FILES.differences), 'w');
    try {
      for (const { digest, difference } of parityDigestRecords(observations)) {
        await digests.writeFile(`${JSON.stringify(digest)}\n`);
        if (difference) await differences.writeFile(`${JSON.stringify(difference)}\n`);
      }
    } finally {
      await differences.close();
    }
  } finally {
    await digests.close();
  }
}

/** The runtime-parity artifacts: entry digests, differing entries, and each runtime's translations for the native stages. */
export const PARITY_ARTIFACT_FILES = Object.freeze({
  digests: 'digests.ndjson',
  differences: 'differences.ndjson',
  translations: (runtime) => `${runtime}-translations.json`,
});

export function ndjson(records) {
  return records.map((record) => `${JSON.stringify(record)}\n`).join('');
}

// The full comparison of a pair of observations, computed once per pair.
function comparison(left, right) {
  if (!comparisons.has(left)) comparisons.set(left, new WeakMap());
  const byRight = comparisons.get(left);
  if (!byRight.has(right)) byRight.set(right, mismatchedSections(left, right));
  return byRight.get(right);
}

/**
 * Per requirement, the observations it is about, the property they must show,
 * and a text-preserving mutation of them that the comparison must detect.
 */
export const PARITY_REQUIREMENTS = Object.freeze({
  'I195-PARITY-CST': {
    sections: NETWORK_SECTIONS,
    holds: (observation) =>
      networks(observation).every(({ source, reconstruction }) => reconstruction === source) &&
      observation.positive.every(({ clean }) => clean) &&
      observation.negative.every(({ nodes }) => nodes.some((node) => hasErrorFlag(JSON.parse(node)))),
    mutate: (observation) => {
      const node = JSON.parse(observation.positive[0].nodes[0]);
      observation.positive[0].nodes[0] = canonicalJson({ ...node, named: !node.named });
    },
  },
  'I195-PARITY-DIAGNOSTICS': {
    sections: ['negative', 'diagnostics'],
    holds: (observation) => {
      const negative = observation.diagnostics.slice(0, observation.negative.length);
      const contextless = observation.diagnostics.slice(observation.negative.length);
      return negative.length > 0 && contextless.length === LANGUAGES.length &&
        negative.every(({ diagnostics }) => diagnostics.some(({ kind }) => kind === 'parse-error')) &&
        contextless.every(({ diagnostics }) =>
          diagnostics.some(({ kind }) => kind === 'missing-project-context')) &&
        // Recovery: the erroneous programs still reconstruct losslessly.
        observation.negative.every(({ source, reconstruction }) => reconstruction === source);
    },
    mutate: (observation) => {
      const [diagnostic] = observation.diagnostics[0].diagnostics;
      diagnostic.range = { ...diagnostic.range, end: diagnostic.range.end + 1 };
    },
  },
  'I195-PARITY-SEMANTICS': {
    sections: ['semantics', 'bindingRenames'],
    holds: (observation) =>
      LANGUAGES.every((language) => {
        const program = semanticProgram(observation, language);
        return program && program.bindings.length > 0 && program.modules.length > 0 &&
          program.types.length > 0 &&
          (program.proofs.length > 0) === PROOF_LANGUAGES.includes(language);
      }) &&
      observation.bindingRenames.some(({ renamed }) => renamed === null) &&
      observation.bindingRenames.some(({ renamed }) => renamed !== null),
    mutate: (observation) => {
      observation.semantics[0].bindings[0].kind = `${observation.semantics[0].bindings[0].kind}-mutated`;
    },
  },
  'I195-PARITY-TRANSFORMS': {
    sections: ['transforms', 'bindingRenames'],
    holds: (observation) =>
      sameMembers(observation.transforms.map(({ language }) => language), LANGUAGES) &&
      observation.transforms.every((transform) =>
        ['replace', 'insert', 'delete', 'clone', 'move'].every((edit) => transform[edit] !== transform.emit)),
    mutate: (observation) => {
      observation.transforms[0].queryCount += 1;
    },
  },
  'I195-PARITY-TRANSLATIONS': {
    sections: ['translations'],
    holds: (observation) => translationsCoverEveryDirection(observation) &&
      observation.translations.every(({ sourceLanguage, decodedSource }) =>
        decodedSource === semanticProgram(observation, sourceLanguage)?.source),
    mutate: (observation) => {
      observation.translations[0].contract.support = `${observation.translations[0].contract.support}-mutated`;
    },
  },
  'I195-SHARED-SCHEMA': {
    sections: ['schemaVersion', 'semantics', 'translations'],
    holds: (observation) =>
      observation.schemaVersion === 1 &&
      new Set(observation.translations.map(({ contract }) => contract.schemaVersion)).size === 1 &&
      Number.isInteger(observation.translations[0]?.contract.schemaVersion) &&
      // Every language registers its own extension forms (directives,
      // attributes, macros, notations).
      LANGUAGES.every((language) => semanticProgram(observation, language)?.extensions.length > 0),
    mutate: (observation) => {
      observation.schemaVersion += 1;
    },
  },
  'I195-SHARED-PROVENANCE': {
    sections: ['semantics', 'translations'],
    holds: (observation) =>
      observation.semantics.every(({ sourceMappingCoverage }) =>
        Object.values(sourceMappingCoverage).every(Boolean)) &&
      observation.semantics.every(({ source, bindings }) => bindings.every(({ declaration }) =>
        declaration.start < declaration.end && declaration.end <= Buffer.byteLength(source))) &&
      observation.translations.every(({ sourceLanguage, targetLanguage, contract }) =>
        contract.source === sourceLanguage && contract.target === targetLanguage &&
        typeof contract.encoding === 'string' && contract.encoding.length > 0),
    mutate: (observation) => {
      observation.semantics[0].sourceMappingCoverage.fullSpan = false;
    },
  },
  'I195-SHARED-CONCEPTS': {
    sections: ['semantics'],
    holds: (observation) => {
      const [lean, rocq] = PROOF_LANGUAGES.map((language) =>
        new Set(semanticProgram(observation, language).proofs.map(({ kind }) => kind)));
      // Lean's and Rocq's proof vocabularies stay apart, and no two languages
      // share one binding-kind profile.
      return [...lean].every((kind) => !rocq.has(kind)) &&
        new Set(LANGUAGES.map((language) => bindingKinds(observation, language))).size === LANGUAGES.length;
    },
    mutate: (observation) => {
      const lean = semanticProgram(observation, 'Lean');
      const theorem = lean.proofs.find(({ kind }) => kind === 'theorem') ?? lean.proofs[0];
      theorem.kind = 'Theorem';
    },
  },
});

/**
 * The assertions of `requirementId` that hold for `runtime`'s observation, and
 * why each other assertion does not hold. Nothing holds while any section
 * differs between the runtimes or the requirement's own property fails.
 */
export function parityAssertions({ requirementId, runtime, observations, pinnedDigest }) {
  const requirement = PARITY_REQUIREMENTS[requirementId];
  if (!requirement) throw new Error(`no parity evidence for ${requirementId}`);
  const own = observations[runtime];
  const other = observations[runtime === 'javascript' ? 'rust' : 'javascript'];
  const mismatches = comparison(own, other);
  if (mismatches.length > 0) return { passed: [], failed: { parity: `mismatch in ${mismatches.join(', ')}` } };
  if (!requirement.sections.every((section) => present(own[section]))) {
    return { passed: [], failed: { sections: `empty ${requirement.sections.join(', ')}` } };
  }
  if (!requirement.holds(own)) return { passed: [], failed: { property: `${requirementId} property` } };

  const checks = {
    sameFixtureRevision: () => own.fixtureDigest === pinnedDigest && other.fixtureDigest === pinnedDigest,
    completeStructureCompared: () => networks(own).every(completeTree),
    fieldsFlagsAndSpansCompared: () =>
      networks(own).every(({ nodes }) => nodes.map((node) => JSON.parse(node)).every(fullNodeSignature)) &&
      own.positive.every(({ fields }) => fields.length > 0) &&
      networks(own).some(({ nodes }) => nodes.some((node) => hasErrorFlag(JSON.parse(node)))),
    triviaAndDiagnosticsCompared: () =>
      own.positive.every(({ trivia }) => trivia.length > 0) &&
      own.diagnostics.every(({ diagnostics }) => diagnostics.length > 0),
    bindingsCompared: () =>
      LANGUAGES.every((language) => semanticProgram(own, language)?.bindings.length > 0) &&
      own.bindingRenames.length > 0 &&
      own.bindingRenames.every(({ bindings }) => bindings.length > 0),
    transformationsCompared: () =>
      sameMembers(own.transforms.map(({ language }) => language), LANGUAGES) &&
      own.bindingRenames.some(({ renamed }) => renamed !== null),
    translationsCompared: () => translationsCoverEveryDirection(own),
    normalizationPreservesDistinctions: () =>
      new Set(own.positive.map(({ language }) => language)).size === own.positive.length &&
      own.positive.every(({ language, nodes }) =>
        nodes.map((node) => JSON.parse(node)).some((node) => node.language === language)) &&
      new Set(own.semantics.map((program) => stableJson(program))).size === own.semantics.length,
    noTextLevelEqualityShortcut: () => {
      const mutated = structuredClone(own);
      requirement.mutate(mutated);
      return canonicalJson(texts(mutated)) === canonicalJson(texts(own)) &&
        mismatchedSections(mutated, other, requirement.sections).length > 0;
    },
  };
  const passed = [];
  const failed = {};
  for (const [assertion, check] of Object.entries(checks)) {
    if (check()) passed.push(assertion);
    else failed[assertion] = `${assertion} does not hold`;
  }
  return { passed, failed };
}

function networks(observation) {
  return NETWORK_SECTIONS.flatMap((section) => observation[section]);
}

// Every node hangs under a syntax root, and one root spans the whole source,
// so the compared node and edge lists are the whole tree.
function completeTree({ source, nodes, edges }) {
  const children = new Set(edges.map((edge) => edge.split(' -> ')[1]));
  const roots = [...new Set(nodes)].filter((node) => !children.has(node)).map((node) => JSON.parse(node));
  const length = Buffer.byteLength(source);
  return nodes.length > 0 && roots.every(({ type }) => type === 'syntax') &&
    roots.some(({ span }) => span?.byteStart === 0 && span.byteEnd === length);
}

function fullNodeSignature({ span, flags }) {
  return span !== null &&
    ['byteStart', 'byteEnd', 'startRow', 'startColumn', 'endRow', 'endColumn']
      .every((key) => Number.isInteger(span[key])) &&
    ['isError', 'hasError', 'isMissing', 'isExtra'].every((key) => typeof flags[key] === 'boolean');
}

function hasErrorFlag({ flags }) {
  return flags.isError || flags.isMissing;
}

function semanticProgram(observation, language) {
  return observation.semantics.find((program) => program.language === language);
}

function bindingKinds(observation, language) {
  return [...new Set(semanticProgram(observation, language).bindings.map(({ kind }) => kind))].sort().join(',');
}

function translationsCoverEveryDirection({ translations }) {
  const directions = translations.map(({ sourceLanguage, targetLanguage }) => `${sourceLanguage}->${targetLanguage}`);
  const expected = LANGUAGES.flatMap((source) =>
    LANGUAGES.filter((target) => target !== source).map((target) => `${source}->${target}`));
  return sameMembers(directions, expected) && translations.every(({ code }) => code.length > 0);
}

function sameMembers(values, expected) {
  return values.length === expected.length && stableJson([...values].sort()) === stableJson([...expected].sort());
}

function present(value) {
  return Array.isArray(value) ? value.length > 0 : value !== undefined && value !== null;
}

// Every text an observation carries, in document order.
function texts(value, found = []) {
  if (Array.isArray(value)) value.forEach((item) => texts(item, found));
  else if (value && typeof value === 'object') {
    for (const key of Object.keys(value).sort()) {
      if (TEXT_KEYS.has(key)) found.push(value[key]);
      else texts(value[key], found);
    }
  }
  return found;
}

export function canonicalJson(value) {
  if (Array.isArray(value)) return `[${value.map(canonicalJson).join(',')}]`;
  if (value && typeof value === 'object') {
    return `{${Object.keys(value).sort().map((key) =>
      `${JSON.stringify(key)}:${canonicalJson(value[key])}`).join(',')}}`;
  }
  return JSON.stringify(value);
}

export function stableJson(value) {
  return `${JSON.stringify(sortKeys(value), null, 2)}\n`;
}

function sortKeys(value) {
  if (Array.isArray(value)) return value.map(sortKeys);
  if (!value || typeof value !== 'object') return value;
  return Object.fromEntries(Object.keys(value).sort().map((key) => [key, sortKeys(value[key])]));
}
