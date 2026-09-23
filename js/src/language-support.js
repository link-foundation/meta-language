/** Schema revision for four-language parser, emitter, and translation contracts. */
export const LANGUAGE_REPRESENTATION_SCHEMA_VERSION = 1;

export const RepresentationLevel = Object.freeze({
  Preserved: 'preserved',
  ConcreteSyntax: 'concrete-syntax',
  Opaque: 'opaque',
  Unavailable: 'unavailable',
});

export const TranslationSupport = Object.freeze({
  UnsupportedObligation: 'unsupported-obligation',
});

const SUPPORT = Object.freeze([
  language({
    name: 'JavaScript',
    aliases: ['javascript', 'js', 'ecmascript'],
    version: 'ECMAScript 2026',
    edition: 'ECMA-262, 17th edition',
    extensions: ['.js', '.mjs', '.cjs'],
    proofSyntax: RepresentationLevel.Unavailable,
  }),
  language({
    name: 'Rust',
    aliases: ['rust', 'rs'],
    version: 'Rust 1.98.1',
    edition: '2024',
    extensions: ['.rs'],
    proofSyntax: RepresentationLevel.Unavailable,
  }),
  language({
    name: 'Lean',
    aliases: ['lean', 'lean4'],
    version: 'Lean 4.34.0',
    edition: 'Lean 4',
    extensions: ['.lean'],
    proofSyntax: RepresentationLevel.Opaque,
  }),
  language({
    name: 'Rocq',
    aliases: ['rocq', 'coq'],
    version: 'Rocq 9.3.0',
    edition: 'Vernacular',
    extensions: ['.v'],
    proofSyntax: RepresentationLevel.Opaque,
  }),
]);

const BY_ALIAS = new Map(
  SUPPORT.flatMap((entry) => entry.aliases.map((alias) => [alias.toLowerCase(), entry])),
);

/** Returns the immutable capability/fidelity declaration for a built-in frontend. */
export function languageSupport(languageName) {
  return BY_ALIAS.get(String(languageName).toLowerCase());
}

/** Returns the four built-in capability declarations in canonical order. */
export function fourLanguageSupport() {
  return SUPPORT;
}

/**
 * Returns all 12 directed translation hooks.
 *
 * These hooks fail closed: concrete syntax is not evidence that two languages
 * have equivalent types, effects, modules, or proof universes.
 */
export function translationContracts() {
  return SUPPORT.flatMap((source) =>
    SUPPORT.filter((target) => target !== source).map((target) => translation(source, target)),
  );
}

export function translationContract(sourceLanguage, targetLanguage) {
  const source = languageSupport(sourceLanguage);
  const target = languageSupport(targetLanguage);
  if (!source || !target || source === target) {
    return undefined;
  }
  return translation(source, target);
}

function language({ name, aliases, version, edition, extensions, proofSyntax }) {
  return Object.freeze({
    schemaVersion: LANGUAGE_REPRESENTATION_SCHEMA_VERSION,
    name,
    aliases: Object.freeze([...aliases]),
    version,
    edition,
    extensions: Object.freeze([...extensions]),
    sourceBytes: RepresentationLevel.Preserved,
    concreteSyntax: RepresentationLevel.ConcreteSyntax,
    bindingResolution: RepresentationLevel.Unavailable,
    typeElaboration: RepresentationLevel.Unavailable,
    dynamicExtensions: RepresentationLevel.Opaque,
    proofSyntax,
    emitter: 'ordered source-token emitter',
  });
}

function translation(source, target) {
  return Object.freeze({
    schemaVersion: LANGUAGE_REPRESENTATION_SCHEMA_VERSION,
    source: source.name,
    target: target.name,
    support: TranslationSupport.UnsupportedObligation,
    observation: 'source bytes and source-runtime concrete syntax; no resolved semantics',
    requiredRuntime: runtimeFor(target.name),
    encoding: 'none registered',
    assumptions: Object.freeze([]),
    obligation:
      `No semantic-preservation proof or explicit encoding is registered for ${source.name} → ` +
      `${target.name}; translation must stop instead of relabelling source text.`,
  });
}

function runtimeFor(languageName) {
  if (languageName === 'Lean') {
    return 'Lean 4.34.0 kernel and project environment';
  }
  if (languageName === 'Rocq') {
    return 'Rocq 9.3.0 kernel and project environment';
  }
  if (languageName === 'Rust') {
    return 'Rust 1.98.1, edition 2024';
  }
  return 'ECMAScript 2026 host';
}
