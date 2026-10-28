import { projectTypeScriptTyping as projectTypeScriptTypingDetails } from './typescript-portable-typing.js';

export * from './access.js';
export * from './api-style-fixtures.js';
export * from './grammar.js';
export * from './grammar-importers.js';
export * from './grammar-emitters.js';
export * from './grammar-merge.js';
export * from './grammar-round-trip.js';
export * from './grammar-validate.js';
export * from './grammar-interchange.js';
export * from './grammar-links.js';
export * from './grammar-lossless.js';
export * from './grammar-lowering.js';
export * from './grammar-reverse.js';
export * from './grammar-runtime.js';
export * from './graphql-adapter.js';
export * from './language-catalog.js';
export * from './concept-records.js';
export * from './concept-distinctions.js';
export * from './global-concept-space.js';
export { createDiscoveredConceptLinks, readDiscoveredConceptLinks } from './discovered-concept-links.js';
export { inferRegularGrammar, readRegularGrammarInferenceLinks, acceptInferredGrammarSymbols,
  createRegularGrammarInferenceProgramLinks } from './grammar-inference.js';
export {refineRegularGrammarInference,readRefinedRegularGrammarInferenceLinks} from './grammar-inference-refinement.js';
export { createConceptViews, readConceptOntology, readConceptTaxonomy } from './concept-ontology.js';
export { BinaryLinksError, encodeLinkNetworkBinary, decodeLinkNetworkBinary } from './link-network-binary.js';
export { UnsignedLinkId, UnsignedLink, UnsignedLinkNetwork, createDenseUnsignedLinkNetwork,
  restoreDenseUnsignedLinkNetwork, encodeUnsignedLinkNetworkBinary,
  decodeUnsignedLinkNetworkBinary } from './unsigned-link-network.js';
export { createMetaLanguageArchive, readMetaLanguageArchive, metaLanguageArchiveToText,
  readMetaLanguageArchiveNetworkText,
  readMetaLanguageArchiveText, metaLanguageArchiveToBinary, readMetaLanguageArchiveBinary,
  extendMetaLanguageArchive } from './meta-language-archive.js';
export { DynamicMetaLanguage, createDynamicMetaLanguage } from './dynamic-meta-language.js';
export { runDynamicLanguageCommand } from './dynamic-language-cli.js';
export { createDynamicLanguageCommandProgramLinks } from './dynamic-language-command-program.js';
export { createRustByteStringProgramLinks } from './rust-byte-string-program.js';
export { readIntrinsicModuleInitializerBinding } from './translation/intrinsic-initializers.js';
export { acceptIntrinsicModuleInitializerScope } from './translation/intrinsic-initializer-rules.js';
export { createJavaScriptRecordClosureParameterProgramLinks } from './record-closure-parameter-program.js';
export { ResourceDescriptionNTriplesError, importResourceDescriptionNTriples, emitResourceDescriptionNTriples, readResourceDescriptionNTriplesSource } from './resource-description-ntriples.js';
export { importResourceDescriptionGraph, decodeResourceDescriptionGraph,
  mergeResourceDescriptionGraphs, verifyResourceDescriptionGraphWitnesses } from './resource-description-graphs.js';
export { verifyGlobalConceptCorrespondence } from './global-concept-rules.js';
export { encodeFormalConceptDefinition, readFormalConceptDefinition, proveFormalConceptEquivalence, verifyFormalConceptEquivalence } from './formal-concept-definition.js';
export * from './decorators.js';
export * from './grammar-decorators.js';
export * from './grammar-concepts.js';
export * from './language-profile.js';
export * from './language-support.js';
export * from './network.js';
export * from './parser-registry.js';
export { grammarNames } from './programming-language-parser.js';
export * from './program-representation.js';
export * from './program-snapshot.js';
export * from './program-translation.js';
export * from './primitives.js';
export * from './query.js';
export * from './query-algebra.js';
export * from './query-plan.js';
export * from './regions.js';
export * from './grammar-composition-batch.js';
export * from './self-translation.js';
export { draftProjectTranslation, projectTranslationDraftToLinks } from './project-translation.js';
export { composeRustSourceProject, parseRustSourceProject, createRustSourceProjectProgramLinks, readRustSourceProjectConcreteGraphs } from './rust-source-project.js';
export {runRustSourceProjectCommand} from './rust-source-project-cli.js';
export {createRustSourceOwnerProgramLinks} from './rust-source-owner-program.js';
export {createSourceProjectArchive,readSourceProjectArchive,extendSourceProjectArchive} from './source-project-archive.js';
export { eraseTypeScript } from './typescript-erasure.js';
/** Project supported TypeScript annotations into the shared frontend's JSDoc form. */
export function projectTypeScriptTyping(source) {
  return projectTypeScriptTypingDetails(source).code;
}
export { createTypedProgramLinks, encodeTypedProgramLinks, encodeTypedProgramLinksWithValueSharing, encodeTypedProgramLinksWithRecordShapes, decodeTypedProgramLinks, emitTypedProgramLinks, readTypedProgramBindingIdentities } from './typed-program-links.js';
export {createNumberBitwiseProgramLinks} from './number-bitwise-program.js';
export { createFixedIntegerCastProgramLinks } from './fixed-integer-cast-program.js';
export { createReferenceMethodProgramLinks } from './reference-method-program.js';
export { createRecordFieldWriteProgramLinks } from './record-write-program.js';
export { createBigIntegerBitwiseProgramLinks } from './big-integer-bitwise-program.js';
export { createRustPrimitiveBitwiseProgramLinks } from './rust-primitive-bitwise-program.js';
export { createArrayStateMethodProgramLinks } from './array-state-method-program.js';
export { createRustShiftProgramLinks } from './rust-shift-program.js';
export { createRustRecordProgramLinks } from './rust-record-program.js';
export { createJavaScriptOptionalCallProgramLinks } from './javascript-optional-call-program.js';
export { createJavaScriptOptionalChainProgramLinks } from './javascript-optional-chain-program.js';
export { createJavaScriptNullishFieldProgramLinks } from './javascript-nullish-field-program.js';
export { createJavaScriptRecordOrderProgramLinks } from './javascript-record-order-program.js';
export { acceptJavaScriptRecordOrderNames, acceptJavaScriptRecordOrderPermutation, acceptJavaScriptRecordPropertyOrder, createJavaScriptRecordPropertyOrder, mergeJavaScriptRecordPropertyOrder, rebaseJavaScriptRecordPropertyOrder, readJavaScriptRecordPropertyKeys } from './translation/javascript-record-order-rules.js';
export { createJavaScriptOwnPropertyProgramLinks } from './javascript-own-property-program.js';
export { createJavaScriptLiteralPropertyProgramLinks } from './javascript-literal-property-program.js';
export { createUnsignedLinkIdentityProgramLinks } from './unsigned-link-identity-program.js';
export { createJavaScriptPropertyCallProgramLinks } from './javascript-property-call-program.js';
export { createLinkAllocationProgramLinks } from './link-allocation-program.js';
export { acceptLinkAllocationCapacity, acceptLinkAllocationWord } from './link-allocation-rules.js';
export { createRustInherentProgramLinks } from './rust-inherent-program.js';
export { createUnicodeScalarProgramLinks } from './unicode-scalar-program.js';
export { createJavaScriptRecordParameterProgramLinks } from './javascript-record-parameter-program.js';
export { createModuleBindingProgramLinks } from './module-binding-program.js';
export { createCallableSourceProgramLinks } from './callable-source-program.js';
export { createScalarAssignmentProgramLinks } from './scalar-assignment-program.js';
export { createSourceIdentifierProgramLinks } from './source-identifier-program.js';
export { createReferenceArrayProjectionProgramLinks } from './reference-array-projection-program.js';
export { createJavaScriptIdentifierProgramLinks } from './javascript-identifier-program.js';
export { SOURCE_IDENTIFIER_UNICODE_VERSION, normalizeUnicodeText } from './translation/unicode-identifiers.js';
export { encodeFormalFunctionDefinition, encodeFormalFunctionDeclarationValue } from './formal-function-links.js';
export { encodeSharedNumericFunctionDefinition } from './shared-numeric-function-links.js';
export * from './semantics.js';
export * from './syntax-projection.js';
export * from './sql-adapter.js';
export * from './substitution.js';
export { encodeStructuredDataLinks, decodeStructuredDataLinks, structuredDataToLino, readStructuredDataLino } from './structured-data-links.js';
export { structuredDataRecordToLino, readStructuredDataRecords } from './structured-data-records.js';
export * from './transform.js';
export * from './translation.js';
export { readSourceTextUnits, readSourceTextByteLength, readSourceTextByte, sliceSourceTextBytes } from './translation/source-text-bytes.js';

export {createStringNumberProgramLinks} from './string-number-program.js';
export {convertJavaScriptStringNumber} from './string-number.js';

export {createBigIntegerConversionProgramLinks} from './big-integer-conversion-program.js';
export {convertJavaScriptStringBigInteger,convertJavaScriptStringBigIntegerUnits,convertJavaScriptNumberBigInteger,convertJavaScriptBooleanBigInteger,divideJavaScriptBigInteger,powerJavaScriptBigInteger,powerJavaScriptNumber} from './big-integer-conversion.js';

export {createSourceExceptionProgramLinks} from './source-exception-program.js';
export {createJavaScriptStringUnitProgramLinks} from './javascript-string-unit-program.js';
export {createStringConstructionProgramLinks} from './string-construction-program.js';
export {createUtf16TextProgramLinks} from './utf16-text-program.js';
export {createFormalStringUnitProgramLinks} from './formal-string-unit-program.js';

export {createJavaScriptComparisonProgramLinks} from './javascript-comparison-program.js';

export {createArrayBindingProgramLinks} from './array-binding-program.js';

export {createJavaScriptStringRangeProgramLinks} from './javascript-string-range-program.js';

export {createOptionalValueProgramLinks} from './optional-value-program.js';

export {mapUnicodeCaseUtf16Text} from './translation/unicode-case-rules.js';
export {createUnicodeCaseProgramLinks} from './unicode-case-program.js';

export {createRustUnitValueProgramLinks} from './rust-unit-value-program.js';

export {createRustFormattingProgramLinks} from './rust-formatting-program.js';
export {sourceDefinitionCatalog,sourceDefinitionEntry,createSourceDefinitionProgramLinks,createSourceDefinitionFormalFunctionLinks} from './source-definition-catalog.js';
export {runSourceDefinitionCommand} from './source-definition-cli.js';
export {createJavaScriptRecordCallableParameterProgramLinks} from './record-callable-parameter-program.js';
export {createJavaScriptArrayFormalParameterProgramLinks} from './array-formal-parameter-program.js';
