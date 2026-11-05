// Author declarations only. Do not execute a grammar, command or fixture here.
import {writeProjectData} from './project-data.mjs';
import {structuredDataToLino} from '../src/structured-data-links.js';
import {grammarSelectionFixture} from '../tests/support/grammar-selection-cases.js';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const recursive=grammarSelectionFixture();
const files=[{path:'configuration.lino',text:structuredDataToLino({records:[]})},
  {path:'first.lino',text:'(grammar (start document))\n(rule document normal (literal first))\n'},
  {path:'second.lino',text:'(grammar (start document))\n(rule document normal (literal second))\n'},
  {path:'first.txt',text:'first'}, {path:'second.txt',text:'second'},
  {path:'recursive.txt',text:recursive.chain.source},
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
];
writeProjectData(new URL('../../parity/fixtures/dynamic-language-command.lino',import.meta.url),{
 schemaVersion:1,model:'dynamic language command source-free snapshots 1',files,steps,
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript 2025',Rust:'>=1.90.0 <1.100.0; edition 2024',linksNotation:'>=0.22.0 <0.23.0'},
 assertions:['sameCompleteCommandSequenceBothRuntimes','actualExitCodeAndDiagnostics','sourceQualifiedInventory','archivedGrammarExport','concreteDefaultAndByteReconstruction','explicitAbstractRetainsConcrete','sourceFreeTextAndBinaryReload','recursiveGrammarResolution','extensionAndPinnedReplacementContexts','strictArgumentsAndLimits'],
 boundary:'Authored future controls only. Miniature labelled grammars exercise generic dispatch, not complete catalog-language support. No command or grammar executes during fixture authoring.',
});
