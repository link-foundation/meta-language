// Provenance-only successor. Current emission and native parity remain independent CI gates.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {serializeProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const root=new URL('../../',import.meta.url);
const policy={
  "schemaVersion": 1,
  "model": "own codebase raw source descriptor successor 1",
  "revision": "52132727d7ce158762a4499b7032a42a9f16f3c3",
  "property": "ownCodebaseRawSourceDescriptorSuccessors",
  "artifactFile": "experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino",
  "registerFile": "parity/self-translation/complete-declaration-receipts.lino",
  "loanFile": "experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino",
  "sourceFile": "js/src/translation/javascript.js",
  "artifactDescriptorPrevious": {
    "file": "js/src/translation/javascript.js",
    "bytes": 130416,
    "sha256": "38d155ce765157039d2555c070e537e97a91f2c64587c70b6c6c578c5371ebe8"
  },
  "anchors": [
    {
      "file": "experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino",
      "bytes": 1816560,
      "sha256": "5525a0b7613e99622f7abfb4da7c293358483c80d29561f19497ce06486f1b0d"
    },
    {
      "file": "parity/self-translation/complete-declaration-receipts.lino",
      "bytes": 110791,
      "sha256": "4c4156d0fa5aca370f2902dd302dfa1c549646c28310870739f7c09dff4e8aa8"
    },
    {
      "file": "experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino",
      "bytes": 688930,
      "sha256": "d32f67b137799af50b9ca0a0f9ac8db727627a9eca967bc6bed3066e14da8f6f"
    }
  ],
  "sourceOwner": {
    "file": "js/src/translation/javascript.js",
    "predecessor": {
      "bytes": 133676,
      "sha256": "ec80211901f3a66ffb230746d20b67adbce0a2716f6d8de1099a7c9742dbc62b"
    },
    "current": {
      "bytes": 133770,
      "sha256": "1ee38749dcf8723b27489dbd33f6e3f177a9e52c898a85cf215aecdcbdc31211"
    },
    "edits": [
      {
        "start": 108297,
        "end": 108336,
        "previousText": "items.push({ spread, value: this.expr()",
        "currentStart": 108297,
        "currentEnd": 108430,
        "currentText": "const value = this.expr();\n      items.push({ spread, value: spread ? { k: 'forOfIterable', object: value, span: value.span } : value"
      }
    ]
  },
  "adapters": [
    {
      "file": "js/scripts/program-string-source-descriptors.mjs",
      "predecessor": {
        "bytes": 3857,
        "sha256": "afc1a2b0a80f31301c264c0fd1dbe00e3cbff9784067bffefbbf22191af900ec"
      },
      "current": {
        "bytes": 4041,
        "sha256": "151dd3654618a4f6760b2252ffe3e93d42b6ddd71bc8cc4da99dce25e7377258"
      },
      "edits": [
        {
          "start": 344,
          "end": 1439,
          "previousText": "const root=new URL('../../',import.meta.url);\nconst policyFile='parity/self-translation/program-string-source-descriptors.lino';\nconst policySha256='d404eef323e602b5f31dc88b0588d4a8a4ff5b23af98f82e0ee62540b6b39741';\nconst revision='90636f9fb52b1b9095defbac041c8089608b8576';\nconst property='programStringRepresentationSourceSuccessors';\nconst pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});\nlet policy;\n\nfunction readProgramStringSourceDescriptorPolicy() {\n  if(!policy){const bytes=readFileSync(new URL(policyFile,root));assert.equal(pin(bytes).sha256,policySha256);policy=readProjectData(new URL(policyFile,root));assert.equal(policy.schemaVersion,1);assert.equal(policy.revision,revision);}\n  return policy;\n}\n\n/** Remove precisely the appended edge and, for its actual new artifact, invert\n * only the two current source descriptor fields. Unknown artifacts stay visible\n * to the original strict gate rather than acquiring a historical identity. */\nexport function restoreProgramStringSourceDescriptorEdge(artifact,identity,register) {",
          "currentStart": 344,
          "currentEnd": 1623,
          "currentText": "import {restoreOwnCodebaseRawDescriptorEdge} from './own-codebase-raw-source-descriptors.mjs';\nconst root=new URL('../../',import.meta.url);\nconst policyFile='parity/self-translation/program-string-source-descriptors.lino';\nconst policySha256='d404eef323e602b5f31dc88b0588d4a8a4ff5b23af98f82e0ee62540b6b39741';\nconst revision='90636f9fb52b1b9095defbac041c8089608b8576';\nconst property='programStringRepresentationSourceSuccessors';\nconst pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});\nlet policy;\n\nfunction readProgramStringSourceDescriptorPolicy() {\n  if(!policy){const bytes=readFileSync(new URL(policyFile,root));assert.equal(pin(bytes).sha256,policySha256);policy=readProjectData(new URL(policyFile,root));assert.equal(policy.schemaVersion,1);assert.equal(policy.revision,revision);}\n  return policy;\n}\n\n/** Remove precisely the appended edge and, for its actual new artifact, invert\n * only the two current source descriptor fields. Unknown artifacts stay visible\n * to the original strict gate rather than acquiring a historical identity. */\nexport function restoreProgramStringSourceDescriptorEdge(artifact,identity,register) {\n  ({artifact,register}=restoreOwnCodebaseRawDescriptorEdge(artifact,identity,register));"
        }
      ]
    },
    {
      "file": "js/scripts/generate-program-string-source-descriptors.mjs",
      "predecessor": {
        "bytes": 5546,
        "sha256": "7c0d608596934a1b9f096ddb0bc89d7c2a4c92db6f7acf93984d1e965c4dc105"
      },
      "current": {
        "bytes": 5716,
        "sha256": "a18e3f09e7446d5a6bed50782de3f1b37276ab82f519b41f32d5273713f5b4d4"
      },
      "edits": [
        {
          "start": 498,
          "end": 4467,
          "previousText": "const root=new URL('../../',import.meta.url),revision='90636f9fb52b1b9095defbac041c8089608b8576';\nconst artifactFile='experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino';\nconst registerFile='parity/self-translation/complete-declaration-receipts.lino';\nconst loanFile='experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino';\nconst sourceFile='js/src/typed-program-checked-projection.js';\nconst output='parity/self-translation/program-string-source-descriptors.lino';\nconst helper='js/scripts/program-string-source-descriptors.mjs';\nconst pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});\n\n/** Append one exact current-source edge; all older fields and bytes remain. */\nexport function generateProgramStringSourceDescriptors({check=false}={}) {\n  const original=file=>{const{blob,certificate}=captureGitArtifactCertificate(revision,file);return{file,commit:revision,...pin(blob),sourceText:blob.toString('utf8'),certificate};};\n  const artifact=original(artifactFile),register=original(registerFile),loan=original(loanFile),sourceOwner={...original(sourceFile),current:pin(readFileSync(new URL(sourceFile,root)))};\n  const value=readStructuredDataLino(artifact.sourceText),changes=[];\n  const owner=value.decisionSources[11];assert.equal(owner.file,sourceFile);\n  assert.deepEqual(owner,{file:sourceFile,bytes:sourceOwner.bytes,sha256:sourceOwner.sha256});\n  for(const field of ['bytes','sha256']){assert.notEqual(owner[field],sourceOwner.current[field]);changes.push({path:['decisionSources','11',field],before:owner[field],after:sourceOwner.current[field]});owner[field]=sourceOwner.current[field];}\n  const currentText=serializeProjectData(value,artifactFile),edge={identity:'raw consuming',sourceRevision:revision,\n    previous:{bytes:artifact.bytes,sha256:artifact.sha256},current:pin(currentText),source:{file:sourceFile,previous:{bytes:sourceOwner.bytes,sha256:sourceOwner.sha256},current:sourceOwner.current},changes};\n  const inverse=structuredClone(value);for(const change of changes)inverse.decisionSources[11][change.path.at(-1)]=change.before;\n  assert.equal(serializeProjectData(inverse,artifactFile),artifact.sourceText,'every emitted owner archive mapping fixture and original artifact byte stays exact');\n  const records=readStructuredDataLino(register.sourceText);assert.equal(Object.hasOwn(records,'programStringRepresentationSourceSuccessors'),false);\n  records.programStringRepresentationSourceSuccessors=[edge];const registerText=serializeProjectData(records,registerFile);\n  delete records.programStringRepresentationSourceSuccessors;assert.equal(serializeProjectData(records,registerFile),register.sourceText);\n  const receipt=readStructuredDataLino(loan.sourceText),previous=structuredClone(receipt.predecessor);\n  assert.deepEqual({bytes:receipt.predecessor.bytes,sha256:receipt.predecessor.sha256},edge.previous);\n  Object.assign(receipt.predecessor,edge.current);const loanText=serializeProjectData(receipt,loanFile);receipt.predecessor=previous;assert.equal(serializeProjectData(receipt,loanFile),loan.sourceText);\n  const packet={schemaVersion:1,revision,artifact,register:{...register,current:pin(registerText)},loan:{...loan,current:pin(loanText)},sourceOwner,edge,\n    boundary:'Only two current checked-program projection descriptor fields change. Every complete native definition, owner, archive, fixture, input/result and older receipt remains exact. The new register entry and borrowed predecessor descriptor have complete original byte inverses. Actual new-source generation and native execution remain mandatory independent checks.'};\n  const writes=[[artifactFile,artifact.sourceText,currentText],[registerFile,register.sourceText,registerText],[loanFile,loan.sourceText,loanText]];\n  for(const[file,old,text]of writes){\n    const path=new URL(file,root),actual=readFileSync(path,'utf8')",
          "currentStart": 498,
          "currentEnd": 4637,
          "currentText": "import {restoreOwnCodebaseRawDescriptorText} from './own-codebase-raw-source-descriptors.mjs';\nconst root=new URL('../../',import.meta.url),revision='90636f9fb52b1b9095defbac041c8089608b8576';\nconst artifactFile='experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino';\nconst registerFile='parity/self-translation/complete-declaration-receipts.lino';\nconst loanFile='experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino';\nconst sourceFile='js/src/typed-program-checked-projection.js';\nconst output='parity/self-translation/program-string-source-descriptors.lino';\nconst helper='js/scripts/program-string-source-descriptors.mjs';\nconst pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});\n\n/** Append one exact current-source edge; all older fields and bytes remain. */\nexport function generateProgramStringSourceDescriptors({check=false}={}) {\n  const original=file=>{const{blob,certificate}=captureGitArtifactCertificate(revision,file);return{file,commit:revision,...pin(blob),sourceText:blob.toString('utf8'),certificate};};\n  const artifact=original(artifactFile),register=original(registerFile),loan=original(loanFile),sourceOwner={...original(sourceFile),current:pin(readFileSync(new URL(sourceFile,root)))};\n  const value=readStructuredDataLino(artifact.sourceText),changes=[];\n  const owner=value.decisionSources[11];assert.equal(owner.file,sourceFile);\n  assert.deepEqual(owner,{file:sourceFile,bytes:sourceOwner.bytes,sha256:sourceOwner.sha256});\n  for(const field of ['bytes','sha256']){assert.notEqual(owner[field],sourceOwner.current[field]);changes.push({path:['decisionSources','11',field],before:owner[field],after:sourceOwner.current[field]});owner[field]=sourceOwner.current[field];}\n  const currentText=serializeProjectData(value,artifactFile),edge={identity:'raw consuming',sourceRevision:revision,\n    previous:{bytes:artifact.bytes,sha256:artifact.sha256},current:pin(currentText),source:{file:sourceFile,previous:{bytes:sourceOwner.bytes,sha256:sourceOwner.sha256},current:sourceOwner.current},changes};\n  const inverse=structuredClone(value);for(const change of changes)inverse.decisionSources[11][change.path.at(-1)]=change.before;\n  assert.equal(serializeProjectData(inverse,artifactFile),artifact.sourceText,'every emitted owner archive mapping fixture and original artifact byte stays exact');\n  const records=readStructuredDataLino(register.sourceText);assert.equal(Object.hasOwn(records,'programStringRepresentationSourceSuccessors'),false);\n  records.programStringRepresentationSourceSuccessors=[edge];const registerText=serializeProjectData(records,registerFile);\n  delete records.programStringRepresentationSourceSuccessors;assert.equal(serializeProjectData(records,registerFile),register.sourceText);\n  const receipt=readStructuredDataLino(loan.sourceText),previous=structuredClone(receipt.predecessor);\n  assert.deepEqual({bytes:receipt.predecessor.bytes,sha256:receipt.predecessor.sha256},edge.previous);\n  Object.assign(receipt.predecessor,edge.current);const loanText=serializeProjectData(receipt,loanFile);receipt.predecessor=previous;assert.equal(serializeProjectData(receipt,loanFile),loan.sourceText);\n  const packet={schemaVersion:1,revision,artifact,register:{...register,current:pin(registerText)},loan:{...loan,current:pin(loanText)},sourceOwner,edge,\n    boundary:'Only two current checked-program projection descriptor fields change. Every complete native definition, owner, archive, fixture, input/result and older receipt remains exact. The new register entry and borrowed predecessor descriptor have complete original byte inverses. Actual new-source generation and native execution remain mandatory independent checks.'};\n  const writes=[[artifactFile,artifact.sourceText,currentText],[registerFile,register.sourceText,registerText],[loanFile,loan.sourceText,loanText]];\n  for(const[file,old,text]of writes){\n    const path=new URL(file,root),observed=readFileSync(path,'utf8'),actual=check?restoreOwnCodebaseRawDescriptorText(observed,file):observed"
        }
      ]
    },
    {
      "file": "js/tests/translation-program-string-source-descriptors.test.js",
      "predecessor": {
        "bytes": 3889,
        "sha256": "747c2f2dc260444c394ac4e9b0f21c5438b14071c66c19ee50441c7b38546455"
      },
      "current": {
        "bytes": 4296,
        "sha256": "5743ead96a184a787c62736a190674a51e5eb4ea35bd1bae7e98647a35a3a6fb"
      },
      "edits": [
        {
          "start": 496,
          "end": 2624,
          "previousText": "const root=new URL('../../',import.meta.url);\n\ntest('the current program-string source descriptor edge retains every original artifact register and borrowed receipt byte',()=>{\n  const packet=generateProgramStringSourceDescriptors({check:true}),artifact=readProjectData(new URL(packet.artifact.file,root)),register=readProjectData(new URL(packet.register.file,root));\n  const restored=restoreProgramStringSourceDescriptorEdge(artifact,'raw consuming',register);\n  assert.equal(serializeProjectData(restored.artifact,packet.artifact.file),packet.artifact.sourceText);\n  assert.equal(serializeProjectData(restored.register,packet.register.file),packet.register.sourceText);\n  for(const row of [packet.artifact,packet.register,packet.loan,packet.sourceOwner])verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:packet.revision,path:row.file});\n  assert.equal(packet.edge.changes.length,2);assert.equal(artifact.definitionCount,106);\n  for(const field of ['completeRust','owners','declarationNames','totals','assumptions','encodings','mappings','preludeHashes','replays','archive'])assert.deepEqual(artifact[field],restored.artifact[field],field);\n  const loan=readProjectData(new URL(packet.loan.file,root)),original=readProjectData(new URL('parity/self-translation/program-string-source-descriptors.lino',root));\n  assert.deepEqual({bytes:loan.predecessor.bytes,sha256:loan.predecessor.sha256},packet.edge.current);\n  const old=JSON.parse(JSON.stringify(loan));Object.assign(old.predecessor,packet.edge.previous);\n  assert.equal(serializeProjectData(old,packet.loan.file),original.loan.sourceText);\n  assert.equal(serializeProjectData(artifact,packet.artifact.file),readFileSync(new URL(packet.artifact.file,root),'utf8'));\n});\n\ntest('changed descriptor ownership register fields edge order and caller mutations cannot enter the historical source chain',()=>{\n  const packet=readProjectData(new URL('parity/self-translation/program-string-source-descriptors.lino',root));\n  const artifact=readProjectData(new URL(packet.artifact.file,root)),register=readProjectData(new URL(packet.register.file,root",
          "currentStart": 496,
          "currentEnd": 3031,
          "currentText": "import {restoreOwnCodebaseRawDescriptorEdge,restoreOwnCodebaseRawDescriptorText,restoreOwnCodebaseRawDescriptorData} from '../scripts/own-codebase-raw-source-descriptors.mjs';\nconst root=new URL('../../',import.meta.url);\n\ntest('the current program-string source descriptor edge retains every original artifact register and borrowed receipt byte',()=>{\n  const packet=generateProgramStringSourceDescriptors({check:true});\n  const {artifact,register}=restoreOwnCodebaseRawDescriptorEdge(readProjectData(new URL(packet.artifact.file,root)),'raw consuming',readProjectData(new URL(packet.register.file,root)));\n  const restored=restoreProgramStringSourceDescriptorEdge(artifact,'raw consuming',register);\n  assert.equal(serializeProjectData(restored.artifact,packet.artifact.file),packet.artifact.sourceText);\n  assert.equal(serializeProjectData(restored.register,packet.register.file),packet.register.sourceText);\n  for(const row of [packet.artifact,packet.register,packet.loan,packet.sourceOwner])verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:packet.revision,path:row.file});\n  assert.equal(packet.edge.changes.length,2);assert.equal(artifact.definitionCount,106);\n  for(const field of ['completeRust','owners','declarationNames','totals','assumptions','encodings','mappings','preludeHashes','replays','archive'])assert.deepEqual(artifact[field],restored.artifact[field],field);\n  const loan=restoreOwnCodebaseRawDescriptorData(readProjectData(new URL(packet.loan.file,root)),packet.loan.file),original=readProjectData(new URL('parity/self-translation/program-string-source-descriptors.lino',root));\n  assert.deepEqual({bytes:loan.predecessor.bytes,sha256:loan.predecessor.sha256},packet.edge.current);\n  const old=JSON.parse(JSON.stringify(loan));Object.assign(old.predecessor,packet.edge.previous);\n  assert.equal(serializeProjectData(old,packet.loan.file),original.loan.sourceText);\n  assert.equal(serializeProjectData(artifact,packet.artifact.file),restoreOwnCodebaseRawDescriptorText(readFileSync(new URL(packet.artifact.file,root),'utf8'),packet.artifact.file));\n});\n\ntest('changed descriptor ownership register fields edge order and caller mutations cannot enter the historical source chain',()=>{\n  const packet=readProjectData(new URL('parity/self-translation/program-string-source-descriptors.lino',root));\n  const {artifact,register}=restoreOwnCodebaseRawDescriptorEdge(readProjectData(new URL(packet.artifact.file,root)),'raw consuming',readProjectData(new URL(packet.register.file,root)"
        }
      ]
    }
  ]
};
const policyFile='parity/self-translation/own-codebase-raw-source-descriptors.lino';
const policySha256='92e693e3b6de06082d2cffec1f59bfb168cc0c1bc8960494e619820bdc7343ff';
const boundary='Only the current JavaScript frontend descriptor and borrowed predecessor digest change. Every complete stored native definition, owner, archive, fixture, input/result and previous receipt remains exact. The independently certified 52132727 frontend is distinct from the older descriptor inside the certified artifact. Its actual 94-byte source successor has a complete byte inverse. Descriptor preservation supplies no semantic equality or native execution evidence; actual current generation, archive replay and native execution remain independent CI gates.';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const exactKeys=(value,keys)=>{assert.ok(value&&typeof value==='object'&&!Array.isArray(value));assert.deepEqual(Object.keys(value).sort(),keys.slice().sort());};
let cached;

/** Detached exact source anchors; no identity is inferred from equal names or samples. */
export function readOwnCodebaseRawDescriptorPolicy(){return{...structuredClone(policy),boundary};}

/** Apply and invert the one finite whole-source interval, preserving every byte. */
export function deriveOwnCodebaseRawDescriptorSource(row,source){
  assert.equal(typeof source,'string');assert.deepEqual(pin(source),row.predecessor);
  let current=Buffer.from(source);
  for(const edit of row.edits.toReversed()){
    assert.equal(current.subarray(edit.start,edit.end).toString('utf8'),edit.previousText);
    current=Buffer.concat([current.subarray(0,edit.start),Buffer.from(edit.currentText),current.subarray(edit.end)]);
  }
  assert.deepEqual(pin(current),row.current);
  let inverse=current;
  for(const edit of row.edits.toReversed()){
    assert.equal(inverse.subarray(edit.currentStart,edit.currentEnd).toString('utf8'),edit.currentText);
    inverse=Buffer.concat([inverse.subarray(0,edit.currentStart),Buffer.from(edit.previousText),inverse.subarray(edit.currentEnd)]);
  }
  assert.equal(inverse.toString('utf8'),source,'every original source byte and historical assertion is retained');
  return current.toString('utf8');
}

function capture(row,expected,extra=[]){
  exactKeys(row,['file','commit','bytes','sha256','sourceText','certificate','current',...extra]);
  for(const key of ['file','bytes','sha256'])assert.equal(row[key],expected[key]);
  assert.equal(row.commit,policy.revision);assert.equal(typeof row.sourceText,'string');
  const bytes=Buffer.from(row.sourceText);assert.equal(bytes.toString('utf8'),row.sourceText);
  assert.deepEqual(pin(bytes),{bytes:expected.bytes,sha256:expected.sha256});
  verifyGitArtifactCertificate(row.certificate,bytes,{revision:policy.revision,path:row.file});
}

/** Reproduce only metadata from independently certified whole artifacts. No translator runs. */
export function deriveOwnCodebaseRawDescriptorOutputs(packet){
  const artifact=readStructuredDataLino(packet.artifact.sourceText);
  assert.deepEqual(artifact.decisionSources[1],policy.artifactDescriptorPrevious);
  const changes=['bytes','sha256'].map(field=>({path:['decisionSources','1',field],before:artifact.decisionSources[1][field],after:policy.sourceOwner.current[field]}));
  for(const change of changes)artifact.decisionSources[1][change.path.at(-1)]=change.after;
  const artifactText=serializeProjectData(artifact,policy.artifactFile);
  const edge={identity:'raw consuming',sourceRevision:policy.revision,
    previous:{bytes:packet.artifact.bytes,sha256:packet.artifact.sha256},current:pin(artifactText),
    source:{file:policy.sourceFile,artifactPrevious:structuredClone(policy.artifactDescriptorPrevious),
      checkedPrevious:{revision:policy.revision,...policy.sourceOwner.predecessor},current:structuredClone(policy.sourceOwner.current)},changes};
  const inverse=structuredClone(artifact);
  for(const change of changes)inverse.decisionSources[1][change.path.at(-1)]=change.before;
  assert.equal(serializeProjectData(inverse,policy.artifactFile),packet.artifact.sourceText,'every original definition, archive and stored semantic field remains exact');
  const register=readStructuredDataLino(packet.register.sourceText);assert.equal(Object.hasOwn(register,policy.property),false);
  register[policy.property]=[edge];const registerText=serializeProjectData(register,policy.registerFile);
  delete register[policy.property];assert.equal(serializeProjectData(register,policy.registerFile),packet.register.sourceText);
  const loan=readStructuredDataLino(packet.loan.sourceText),previous=structuredClone(loan.predecessor);
  assert.deepEqual({bytes:previous.bytes,sha256:previous.sha256},edge.previous);
  Object.assign(loan.predecessor,edge.current);const loanText=serializeProjectData(loan,policy.loanFile);
  loan.predecessor=previous;assert.equal(serializeProjectData(loan,policy.loanFile),packet.loan.sourceText);
  return{edge,outputs:[{file:policy.artifactFile,source:artifactText},{file:policy.registerFile,source:registerText},{file:policy.loanFile,source:loanText}]};
}

/** Authenticate every complete predecessor, current descriptor and adapter source inverse. */
export function verifyOwnCodebaseRawDescriptorPacket(packet){
  exactKeys(packet,['schemaVersion','model','revision','artifact','register','loan','sourceOwner','adapters','edge','boundary']);
  for(const key of ['schemaVersion','model','revision'])assert.equal(packet[key],policy[key]);assert.equal(packet.boundary,boundary);
  for(const [index,key]of ['artifact','register','loan'].entries())capture(packet[key],policy.anchors[index]);
  capture(packet.sourceOwner,{file:policy.sourceFile,...policy.sourceOwner.predecessor},['edits']);
  assert.deepEqual(packet.sourceOwner.current,policy.sourceOwner.current);assert.deepEqual(packet.sourceOwner.edits,policy.sourceOwner.edits);
  deriveOwnCodebaseRawDescriptorSource(policy.sourceOwner,packet.sourceOwner.sourceText);
  assert.equal(packet.adapters.length,policy.adapters.length);
  for(const [index,row]of packet.adapters.entries()){
    const expected=policy.adapters[index];capture(row,{file:expected.file,...expected.predecessor},['edits']);
    assert.deepEqual(row.current,expected.current);assert.deepEqual(row.edits,expected.edits);
    deriveOwnCodebaseRawDescriptorSource(expected,row.sourceText);
  }
  const result=deriveOwnCodebaseRawDescriptorOutputs(packet);assert.deepEqual(packet.edge,result.edge);
  for(const output of result.outputs){const row=[packet.artifact,packet.register,packet.loan].find(row=>row.file===output.file);assert.deepEqual(row.current,pin(output.source));}
  return packet;
}

function loadPacket(){
  if(!cached){const bytes=readFileSync(new URL(policyFile,root));assert.equal(pin(bytes).sha256,policySha256,'the complete reviewed finite descriptor packet remains exact');cached=verifyOwnCodebaseRawDescriptorPacket(readBootstrapStructuredDataLino(bytes.toString('utf8')));}
  return cached;
}

/** Detached policy data cannot alter the internal source authority. */
export function readOwnCodebaseRawDescriptorPacket(){return structuredClone(loadPacket());}

function restoreText(source,file,packet){
  assert.equal(typeof source,'string');const row=[packet.artifact,packet.register,packet.loan].find(row=>row.file===file);assert.ok(row,'one exact registered artifact path');
  if(source===row.sourceText)return source;
  assert.deepEqual(pin(source),row.current,'unknown complete artifacts cannot acquire predecessor authority');
  const value=readStructuredDataLino(source);
  if(file===policy.artifactFile){
    for(const change of packet.edge.changes){const owner=value.decisionSources[1],field=change.path.at(-1);assert.equal(owner.file,policy.sourceFile);assert.equal(owner[field],change.after);owner[field]=change.before;}
  }else if(file===policy.registerFile){assert.equal(Object.keys(value).at(-1),policy.property);assert.deepEqual(value[policy.property],[packet.edge]);delete value[policy.property];}
  else{assert.deepEqual({bytes:value.predecessor.bytes,sha256:value.predecessor.sha256},packet.edge.current);Object.assign(value.predecessor,packet.edge.previous);}
  const previous=serializeProjectData(value,file);assert.equal(previous,row.sourceText,'the complete certified predecessor is recovered without changing a semantic field');
  return previous;
}

/** Historical checks get the whole certified predecessor; old writers cannot overwrite successors. */
export function restoreOwnCodebaseRawDescriptorText(source,file){return restoreText(source,file,loadPacket());}
export function restoreOwnCodebaseRawDescriptorData(value,file){return readStructuredDataLino(restoreText(serializeProjectData(value,file),file,loadPacket()));}

/** Remove only the finite new register edge before the unchanged strict historical chain. */
export function restoreOwnCodebaseRawDescriptorEdge(artifact,identity,register){
  if(!Object.hasOwn(register,policy.property))return{artifact,register};
  const packet=loadPacket();
  const previousRegister=readStructuredDataLino(restoreText(serializeProjectData(register,policy.registerFile),policy.registerFile,packet));
  if(identity!=='raw consuming'||artifact===null)return{artifact,register:previousRegister};
  const source=serializeProjectData(artifact,policy.artifactFile);
  if(pin(source).sha256!==packet.edge.current.sha256)return{artifact,register:previousRegister};
  assert.deepEqual(pin(readFileSync(new URL(policy.sourceFile,root))),policy.sourceOwner.current,'actual current frontend source remains independently pinned');
  return{artifact:readStructuredDataLino(restoreText(source,policy.artifactFile,packet)),register:previousRegister};
}
