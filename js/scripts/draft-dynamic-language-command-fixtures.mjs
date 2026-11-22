// Author declarations only. Do not execute a grammar, command or fixture here.
import {readProjectData,writeProjectData} from './project-data.mjs';
import {structuredDataToLino} from '../src/structured-data-links.js';
import {grammarSelectionFixture} from '../tests/support/grammar-selection-cases.js';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const recursive=grammarSelectionFixture();
const scanner=readProjectData(new URL('../../parity/fixtures/regular-grammar-inference.lino',import.meta.url)).scanner;
const scannerSequences=[['short','begin','content','end','suffix'],['long','begin','content','end','suffix']];
const files=[{path:'configuration.lino',text:structuredDataToLino({records:[]})},
  {path:'first.lino',text:'(grammar (start document))\n(rule document normal (literal first))\n'},
  {path:'second.lino',text:'(grammar (start document))\n(rule document normal (literal second))\n'},
  {path:'first.txt',text:'first'}, {path:'second.txt',text:'second'},
  {path:'recursive.txt',text:recursive.chain.source},
  {path:'labelled.lino',text:structuredDataToLino({positives:[['a'],['b']],negatives:[['x']]})},
  {path:'counterexamples.lino',text:structuredDataToLino({positives:[['c']],negatives:[['y']]})},
  {path:'invalid-examples.lino',text:structuredDataToLino({positives:['a'],negatives:[]})},
  {path:'discovered.txt',text:'c'}, {path:'rejected.txt',text:'x'},
  {path:'scanner-examples.lino',text:structuredDataToLino({positives:[scannerSequences[0]],negatives:[[],['long']],scannerObservations:scanner.observations})},
  {path:'scanner-counterexamples.lino',text:structuredDataToLino({positives:[scannerSequences[1]],negatives:[['begin']]})},
  {path:'scanner-source.txt',text:scanner.accepted[1].input},
  {path:'scanner-rejection.txt',text:'==[x]🙂'},
  ...recursive.grammars.slice(0,4).map(value=>({path:value.name+'.lino',text:value.listing}))];
const source=['--source','first-source','Example language','authored-1','first.lino'];
const recursiveSources=recursive.grammars.slice(0,4).flatMap(value=>['--source',value.name,value.name,recursive.chain.edition,value.name+'.lino']);
const steps=[
 {name:'prepare from complete native grammar',args:['prepare',...source,'--configuration','configuration.lino'],save:'prepared.lino',exit:0,archive:true},
 {name:'inventory source identity',args:['list','prepared.lino'],exit:0,inventory:['first-source']},
 {name:'export archived grammar',args:['grammar','prepared.lino','--source','first-source'],exit:0,grammar:'first.lino'},
 {name:'default saved concrete syntax',args:['parse','prepared.lino','--source','first-source','--input','first.txt','--identifier','original document'],save:'parsed.lino',exit:0,archive:true,concrete:'original document',text:'first'},
 {name:'explicit abstract view retains concrete entry',args:['parse','parsed.lino','--source','first-source','--input','first.txt','--projection','ast','--identifier','abstract source'],save:'abstract.lino',exit:0,archive:true,concrete:'abstract source',abstract:'abstract syntax:abstract source',text:'first'},
 {name:'binary output reloads without original grammars',args:['parse','abstract.lino','--source','first-source','--input','first.txt','--binary'],save:'parsed.bin',exit:0,archive:true,binary:true},
 {name:'binary source inventory',args:['list','parsed.bin','--input-binary'],exit:0,inventory:['first-source']},
 {name:'prepare recursive grammar closure',args:['prepare',...recursiveSources,'--configuration','configuration.lino'],save:'recursive.lino',exit:0,archive:true},
 {name:'deep recursive source-free CST',args:['parse','recursive.lino','--source','Markdown','--input','recursive.txt','--identifier','deep document'],save:'deep.lino',exit:0,archive:true,concrete:'deep document',text:recursive.chain.source},
 {name:'extend complete source set',args:['extend','parsed.lino','--source','second-source','Another example language','authored-1','second.lino'],save:'extended.lino',exit:0,archive:true},
 {name:'parse newly added source',args:['parse','extended.lino','--source','second-source','--input','second.txt','--identifier','second document'],save:'second document.lino',exit:0,archive:true,concrete:'second document',text:'second'},
 {name:'implicit replacement refuses',args:['extend','parsed.lino','--source','first-source','Example language','authored-2','second.lino'],exit:2,error:'grammar replacement requires an explicit selection'},
 {name:'explicit replacement pins preceding contexts',args:['extend','parsed.lino','--source','first-source','Example language','authored-2','second.lino','--replace'],save:'replaced.lino',exit:0,archive:true,concrete:'original document',text:'first'},
 {name:'parse replaced source',args:['parse','replaced.lino','--source','first-source','--input','second.txt','--identifier','new document'],save:'new.lino',exit:0,archive:true,concrete:'new document',text:'second'},
 {name:'actual rejection is observable',args:['parse','prepared.lino','--source','first-source','--input','second.txt'],exit:1,rejection:true},
 {name:'duplicate command option refuses',args:['list','prepared.lino','--source','first-source','--source','first-source'],exit:2,error:'duplicate language argument --source'},
 {name:'invalid limit spelling refuses',args:['parse','prepared.lino','--source','first-source','--input','first.txt','--step-limit','01'],exit:2,error:'a parser limit needs a nonnegative safe integer'},
 {name:'unrepresented resolver override refuses',args:['parse','prepared.lino','--source','first-source','--input','first.txt','--resolve-grammar','foreign'],exit:2,error:'unexpected or incomplete language argument --resolve-grammar'},
 {name:'unknown projection refuses',args:['parse','prepared.lino','--source','first-source','--input','first.txt','--projection','invented'],exit:2,error:'unknown syntax projection'},
 {name:'invalid discovery corpus refuses',args:['discover','invalid-examples.lino'],exit:2,error:'labelled examples must contain positive and negative symbol sequences'},
 {name:'discover a grammar from labelled samples',args:['discover','labelled.lino','--identifier','regular-original','--binary'],exit:0,save:'discovered.bin',remove:['labelled.lino']},
 {name:'refine the stored binary checkpoint without training files',args:['refine','discovered.bin','--input-binary','--examples','counterexamples.lino','--identifier','regular-refined'],exit:0,save:'refined.lino',remove:['counterexamples.lino','discovered.bin']},
 {name:'install the rechecked text checkpoint without sample files',args:['install','refined.lino','--binary'],exit:0,save:'installed.bin',archive:true,binary:true},
 {name:'inventory the dynamically installed source',args:['list','installed.bin','--input-binary'],exit:0,inventory:['regular-refined']},
 {name:'parse a counterexample from the installed grammar',args:['parse','installed.bin','--input-binary','--source','regular-refined','--input','discovered.txt','--identifier','refined document'],exit:0,archive:true,concrete:'refined document',text:'c'},
 {name:'a negative observation still rejects after installation',args:['parse','installed.bin','--input-binary','--source','regular-refined','--input','rejected.txt'],exit:1,rejection:true},
 {name:'discover with retained scanner mode observations',args:['discover','scanner-examples.lino','--identifier','scanner-original','--binary'],exit:0,save:'scanner.bin',remove:['scanner-examples.lino']},
 {name:'keep the scanner when refining stored token observations',args:['refine','scanner.bin','--input-binary','--examples','scanner-counterexamples.lino','--identifier','scanner-refined'],exit:0,save:'scanner-refined.lino',remove:['scanner.bin','scanner-counterexamples.lino']},
 {name:'install the refined scanner and grammar',args:['install','scanner-refined.lino','--binary'],exit:0,save:'scanner-installed.bin',archive:true,binary:true,remove:['scanner-refined.lino']},
 {name:'the scanner installed from Links reconstructs Unicode source bytes',args:['parse','scanner-installed.bin','--input-binary','--source','scanner-refined','--input','scanner-source.txt','--identifier','scanner document'],exit:0,archive:true,concrete:'scanner document',text:scanner.accepted[1].input},
 {name:'installed scanner rejects an unobserved spelling',args:['parse','scanner-installed.bin','--input-binary','--source','scanner-refined','--input','scanner-rejection.txt'],exit:1,rejection:true},
];
writeProjectData(new URL('../../parity/fixtures/dynamic-language-command.lino',import.meta.url),{
 schemaVersion:1,model:'dynamic language command source-free snapshots 1',files,steps,
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript 2025',Rust:'>=1.90.0 <1.100.0; edition 2024',linksNotation:'>=0.22.0 <0.23.0'},
 assertions:['sameCompleteCommandSequenceBothRuntimes','actualExitCodeAndDiagnostics','sourceQualifiedInventory','archivedGrammarExport','concreteDefaultAndByteReconstruction','explicitAbstractRetainsConcrete','sourceFreeTextAndBinaryReload','recursiveGrammarResolution','extensionAndPinnedReplacementContexts','strictArgumentsAndLimits','regularDiscoveryCheckpointRefinementAndInstallation','scannerDiscoveryCheckpointRefinementAndInstallation'],
 boundary:'Authored future controls only. Miniature labelled grammars exercise generic dispatch, not complete catalog-language support. No command or grammar executes during fixture authoring.',
});
