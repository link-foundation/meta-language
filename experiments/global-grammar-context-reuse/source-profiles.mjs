// Candidate current/frozen profiles for the complete context-reuse draft.
// This module does not change production sources or the frozen wire policy.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData,serializeProjectData,checkProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';
import {readRustSourceItems} from '../../js/scripts/generate-private-output-test-model.mjs';
import {readGlobalGrammarContextProposal} from './proposal.mjs';
import {diagnosticOrderDecorator} from '../grammar-outcome-network/native-transport/diagnostic-order/decorator.mjs';
import {applyDiagnosticOrderDecorator,readJavaScriptDiagnosticOrders,javascriptPath,nativePath} from '../grammar-outcome-network/native-transport/diagnostic-order/generate.mjs';
import {outcomeWirePolicy} from '../grammar-outcome-network/native-transport/wire/wire-policy.mjs';
const root=new URL('../../',import.meta.url);
const sourceFile='parity/self-translation/global-grammar-context-reuse.lino';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let profile;

function nativeContextDeclarationProof(original,current) {
  const selections=outcomeWirePolicy.sources.filter(row=>row.file===nativePath);assert.equal(selections.length,1);
  const names=selections[0].names,read=source=>{
    const bytes=Buffer.from(source),parsed=readRustSourceItems(bytes);
    const imports=parsed.items.filter(row=>row.kind==='use_declaration').map(row=>({source:parsed.text(row.node),start:row.node.start,end:row.node.end}));
    const declarations=names.map(name=>{
      const matches=parsed.items.filter(row=>row.name===name);assert.equal(matches.length,1,name);
      const row=matches[0],text=bytes.subarray(row.start,row.end).toString();
      return{name,kind:row.kind,start:row.start,end:row.end,source:text,...pin(text)};
    });
    return{imports,declarations};
  };
  const before=read(original),after=read(current);
  assert.deepEqual(after,before,'every selected actual native import, complete type and original byte interval stays exact');
  assert.ok(after.declarations.length>0);
  return after;
}

function javaScriptContextDiagnosticProof(original,current) {
  const before=readJavaScriptDiagnosticOrders(original),after=readJavaScriptDiagnosticOrders(current);
  assert.equal(before.length,9);assert.equal(after.length,before.length);
  return before.map((row,index)=>{
    const next=after[index],oldCall=Buffer.from(original).subarray(row.call.span.start,row.call.span.end).toString();
    const newCall=Buffer.from(current).subarray(next.call.span.start,next.call.span.end).toString();
    assert.equal(newCall,oldCall,'every actual complete diagnostic call and field-value spelling remains exact');
    assert.deepEqual(next.keys,row.keys);assert.equal(next.reason,row.reason);assert.equal(next.detail,row.detail);
    return{keys:row.keys,reason:row.reason,detail:row.detail,source:oldCall,...pin(oldCall),
      historicalCall:row.call,currentCall:next.call};
  });
}

/** Build an explicit candidate profile from certified original and proposed bytes. */
export function buildGlobalGrammarContextSourceProfiles() {
  if(profile)return structuredClone(profile);
  const packet=readProjectData(new URL(sourceFile,root));
  const javascript=readGlobalGrammarContextProposal(javascriptPath),native=readGlobalGrammarContextProposal(nativePath);
  const originalJavaScript=diagnosticOrderDecorator.inputs.find(row=>row.path===javascriptPath).source;
  const originalNative=diagnosticOrderDecorator.inputs.find(row=>row.path===nativePath).source;
  const corrected=applyDiagnosticOrderDecorator(originalNative,diagnosticOrderDecorator,{role:'frozen CI overlay'});
  const oldJavaScript=packet.sources.find(row=>row.file===javascriptPath).sourceText;
  const oldNative=packet.sources.find(row=>row.file===nativePath).sourceText;
  assert.equal(oldJavaScript,originalJavaScript);assert.equal(oldNative,corrected.source);
  assert.equal(pin(originalJavaScript).sha256,outcomeWirePolicy.pins[javascriptPath]);
  assert.equal(pin(originalNative).sha256,outcomeWirePolicy.pins[nativePath]);
  profile={schemaVersion:1,profile:'proposed global grammar context source views',
    proposal:{path:sourceFile,...pin(readFileSync(new URL(sourceFile,root)))},
    historicalPins:structuredClone(outcomeWirePolicy.pins),
    sources:[
      {file:javascriptPath,historical:pin(originalJavaScript),productionPredecessor:pin(oldJavaScript),proposed:javascript.proposed,
        inverseSteps:['inverse certified context reuse edits'],proof:javaScriptContextDiagnosticProof(originalJavaScript,javascript.source)},
      {file:nativePath,historical:pin(originalNative),productionPredecessor:pin(oldNative),proposed:native.proposed,
        inverseSteps:['inverse certified context reuse edits','inverse original diagnostic key permutation'],proof:nativeContextDeclarationProof(originalNative,native.source)},
    ],
    frozenOverlay:{original:pin(originalNative),output:pin(corrected.source),receipt:corrected.receipt},
    boundary:'An explicit candidate source profile, not installed production or observed native execution. Full original source actions authenticate the proposed bytes. Native imports and selected complete type declarations remain exact; JavaScript diagnostic calls retain complete field-value spellings. Frozen producer and overlay identities remain separate from proposed current sources. Predicate equivalence, compiler features and complete native execution remain required.'};
  return structuredClone(profile);
}

/** Caller edits cannot alter the private recomputed exact candidate profile. */
export function checkGlobalGrammarContextSourceProfiles(value) {
  const expected=buildGlobalGrammarContextSourceProfiles();
  assert.equal(serializeProjectData(value,'profile.lino'),serializeProjectData(expected,'profile.lino'),'complete candidate source-profile conservation');
  return structuredClone(expected);
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const value=buildGlobalGrammarContextSourceProfiles(),path=new URL('parity/self-translation/global-grammar-context-source-profiles.lino',root);
  if(process.argv.includes('--check'))checkProjectData(path,value);else writeProjectData(path,value);
  console.log('Drafted two current-source candidates with complete source inverses and distinct frozen producer/overlay identities.');
}
