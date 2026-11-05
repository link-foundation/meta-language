// Future observations/counterexamples are declarations, never learner execution.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
writeProjectData(new URL('../../parity/fixtures/regular-grammar-refinement.lino',import.meta.url),{
 schemaVersion:1,model:'labelled regular grammar refinement 1',
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 cases:['rpni','edsm','alergia'].flatMap(strategy=>['a','🙂'].map(symbol=>({name:strategy+' counterexample refinement '+symbol,options:{strategy,identifier:'counterexample-'+strategy+'-'+(symbol==='a'?'ascii':'unicode')},positives:[[symbol],[symbol,symbol]],negatives:[[]],steps:[{positives:[[symbol,symbol]],negatives:[[symbol,symbol,symbol]]},{positives:[[symbol,symbol,symbol,symbol]],negatives:[['b']]}]}))),
 archiveLimits:{nodeLimit:1000000,referenceLimit:4000000,byteLimit:134217728},
 assertions:['allOriginalModelsDecisionsAndGrammarGraphsRetained','singleActualLearnerAndRefinementDefinitions','exactOrderedObservationPrefixesWithStatisticalMultiplicity','deterministicResumptionThroughTextAndBinary','newSourceQualifiedIdentitiesWithoutUnprovedConceptMerges','allDeclaredPositiveAndNegativeExamplesRetained','contradictionAndIdentityFailuresPreserveOriginalGraph','missingChangedOrUnreferencedHistoryRefusal','linkedCstDefaultAfterDynamicInstallation','independentJavaScriptNativeTablesGrammarsAndArchiveBytes'],
 boundary:'Unexecuted future controls. This version relearns deterministically from the complete retained corpus under the existing RPNI/EDSM/ALERGIA-style implementation. It claims neither unknown-language completeness, a new modern discovery method, asymptotically incremental learning nor formal semantic equivalence. Eight journal steps and finite graph/learner limits remain explicit.',
});
