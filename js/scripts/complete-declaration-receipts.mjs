// Finite current receipts; historical artifacts and semantic output stay exact.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {structuredDataToLino} from '../src/structured-data-links.js';
import {restoreTranslationSourceHistory} from './translation-source-history.mjs';
import {restoreCurrentTranslationSourceDescriptors} from './current-translation-source-descriptors.mjs';

export const COMPLETE_DECLARATION_RECEIPTS = new URL('../../parity/self-translation/complete-declaration-receipts.lino',import.meta.url);
const digest=value=>createHash('sha256').update(value).digest('hex');

// A source revision authenticates source descriptors; it does not claim that
// the derived successor artifact was already committed at that source revision.
const descriptorSuccessor = {
 identity:'raw consuming',
 previous:{file:'experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino',
  revision:'191f214903cc123cfb1566d82a90e18819be6b4d',bytes:1814953,
  sha256:'99c8d2e697ee99d9b8b8c34c7c1bfb9c7a121206aa455618756f37435c7624a9'},
 current:{file:'experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino',
  sourceRevision:'b4b92d47ee0a186dbe3920c1844eb907afe56aeb',bytes:1814953,
  sha256:'0698669cdac848bfd661a1f87be629cdc81ddc3b4a2213bfb145d4c9de441644'},
 sources:[
  {file:'js/src/translation/check.js',
   previous:{bytes:126330,sha256:'e4a95a59a9f9878d2842928db794a4973eaa56daae8e11a970dfafd4f15f7551'},
   current:{bytes:127369,sha256:'abf66bd551b94e9870a8d2c0d709af7c4a72e5fdc3fbd1e828a89de140f92880'}},
  {file:'js/src/translation/emit-rust.js',
   previous:{bytes:79266,sha256:'69315a498f97e54b16f2fb8f3d7454d84e818ddce7bb89904b4ae85279af1f79'},
   current:{bytes:77647,sha256:'843b6c072917fa8f86d36eabf64df022c92824062a921e5d8df71ac3d2df0f31'}},
 ],
};

/** Apply the one reviewed descriptor edge before the original receipt inverse. */
function readHistoricalCompleteDeclarationDescriptorSuccessor(artifact,identity,register){
 const {successors,...originalRegister}=register;
 const originalBytes=Buffer.from(structuredDataToLino(originalRegister));
 assert.equal(originalBytes.length,68598,'all four original receipt rows retain their complete byte count');
 assert.equal(digest(originalBytes),'932974138d4bcd76ca097ed5d1e726abae850a85bea3f7d96ff8143ad84602ba',
  'removing only successors restores the complete immutable four-row register');
 const base=register.receipts.find(row=>row.identity===identity);
 assert.ok(base,'the successor cannot introduce an undeclared receipt identity');
 const bytes=Buffer.from(structuredDataToLino(artifact)),actual=digest(bytes);
 if(successors===undefined){
  assert.ok([base.previous.sha256,base.current.sha256].includes(actual),'unregistered descriptor successors refuse');
  return {artifact:structuredClone(artifact),bytes};
 }
 assert.ok(Array.isArray(successors));
 assert.equal(successors.length,1,'the reviewed finite chain has exactly one descriptor successor');
 const edge=successors[0],{changes,...metadata}=edge;
 assert.deepEqual(metadata,descriptorSuccessor,'the whole descriptor edge and both source revisions are independently pinned');
 const expected=[];
 for(const [index,source]of descriptorSuccessor.sources.entries())for(const field of ['bytes','sha256']){
  expected.push({path:['decisionSources',String(index+3),field],before:source.previous[field],after:source.current[field]});
 }
 assert.deepEqual(changes,expected,'only the two owned descriptor byte counts and hashes may change');
 if(identity!==edge.identity||actual!==edge.current.sha256){
  assert.ok([base.previous.sha256,base.current.sha256].includes(actual),'unknown artifacts cannot enter the descriptor chain');
  return {artifact:structuredClone(artifact),bytes};
 }
 assert.equal(bytes.length,edge.current.bytes,'the entire descriptor successor byte count is exact');
 const inverse=structuredClone(artifact);
 for(const [index,source]of edge.sources.entries()){
  const descriptor=inverse.decisionSources[index+3];
  assert.deepEqual(descriptor,{file:source.file,...source.current},'each actual descriptor retains its exact file owner and fields');
 }
 for(const change of changes){
  const descriptor=inverse.decisionSources[Number(change.path[1])],field=change.path[2];
  assert.deepEqual(descriptor[field],change.after);
  descriptor[field]=change.before;
 }
 const restored=Buffer.from(structuredDataToLino(inverse));
 assert.equal(restored.length,edge.previous.bytes,'the whole preceding current artifact byte count is recovered');
 assert.equal(digest(restored),edge.previous.sha256,'only the reviewed descriptor fields changed in the complete artifact');
 assert.equal(edge.previous.sha256,base.current.sha256,'the successor rejoins the unchanged original receipt current endpoint');
 assert.equal(edge.previous.bytes,base.current.bytes);
 assert.equal(edge.previous.file,base.current.file);
 return {artifact:inverse,bytes:restored};
}

// This second finite edge authenticates source descriptors only. It never
// changes an emitted definition, archive premise or original receipt row.
const descriptorRefreshSuccessor = {
  "identity": "raw consuming",
  "previous": {
    "file": "experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino",
    "revision": "1a69d1b33be71ca7421eb8a7a2e4dba9b599ef26",
    "bytes": 1814953,
    "sha256": "0698669cdac848bfd661a1f87be629cdc81ddc3b4a2213bfb145d4c9de441644"
  },
  "current": {
    "file": "experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino",
    "sourceRevision": "113a7747f799cbce5167167a783eb88427bd2825",
    "bytes": 1814953,
    "sha256": "95032ee3af3c0de10c6486aa1d76e4b4d315e34379cb5d081a47ae9cb35793e2"
  },
  "sources": [
    {
      "file": "js/src/translation/emit-rust.js",
      "previous": {
        "bytes": 77647,
        "sha256": "843b6c072917fa8f86d36eabf64df022c92824062a921e5d8df71ac3d2df0f31"
      },
      "current": {
        "bytes": 77908,
        "sha256": "143e41659d4f0915c2f4e90865c96fce1fba8d3e1cfaf99e8cbdb56108a62c64"
      }
    }
  ],
  "changes": [
    {
      "path": [
        "decisionSources",
        "4",
        "bytes"
      ],
      "before": 77647,
      "after": 77908
    },
    {
      "path": [
        "decisionSources",
        "4",
        "sha256"
      ],
      "before": "843b6c072917fa8f86d36eabf64df022c92824062a921e5d8df71ac3d2df0f31",
      "after": "143e41659d4f0915c2f4e90865c96fce1fba8d3e1cfaf99e8cbdb56108a62c64"
    }
  ]
};

function readCompleteDeclarationDescriptorSuccessor(artifact,identity,register){
 if(!Object.hasOwn(register,'descriptorSuccessors'))return readHistoricalCompleteDeclarationDescriptorSuccessor(artifact,identity,register);
 const {descriptorSuccessors,...previousRegister}=register;
 const previousBytes=Buffer.from(structuredDataToLino(previousRegister));
 assert.equal(previousBytes.length,76897,'the complete preceding register byte count stays exact');
 assert.equal(digest(previousBytes),'2ca95df9f892d1093548021d37ac8a2e909d15eaffe337cbd888b3dd5f0cbad6',
  'removing only descriptorSuccessors restores every original receipt and the first successor');
 assert.ok(Array.isArray(descriptorSuccessors));
 assert.equal(descriptorSuccessors.length,1,'exactly one separately reviewed descriptor refresh is registered');
 const edge=descriptorSuccessors[0];
 assert.deepEqual(edge,descriptorRefreshSuccessor,'the whole second descriptor edge is independently pinned');
 const bytes=Buffer.from(structuredDataToLino(artifact));
 if(identity!==edge.identity||digest(bytes)!==edge.current.sha256)return readHistoricalCompleteDeclarationDescriptorSuccessor(artifact,identity,previousRegister);
 assert.equal(bytes.length,edge.current.bytes);
 const inverse=structuredClone(artifact),source=edge.sources[0];
 assert.deepEqual(inverse.decisionSources[4],{file:source.file,...source.current},
  'the changed descriptor retains its exact owner and complete field set');
 inverse.decisionSources[4].bytes=source.previous.bytes;
 inverse.decisionSources[4].sha256=source.previous.sha256;
 const restored=Buffer.from(structuredDataToLino(inverse));
 assert.equal(restored.length,edge.previous.bytes);
 assert.equal(digest(restored),edge.previous.sha256,
  'only the two reviewed fields change in the entire raw consuming artifact');
 return readHistoricalCompleteDeclarationDescriptorSuccessor(inverse,identity,previousRegister);
}

// This third finite edge records only the generated neutral type declaration.
const typeNeutralDescriptorSuccessor = {
  "identity": "raw consuming",
  "previous": {
    "file": "experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino",
    "revision": "18a8e0a04bab43f6572be311ceb75c6596fdc147",
    "bytes": 1814953,
    "sha256": "95032ee3af3c0de10c6486aa1d76e4b4d315e34379cb5d081a47ae9cb35793e2"
  },
  "current": {
    "file": "experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino",
    "sourceRevision": "18a8e0a04bab43f6572be311ceb75c6596fdc147",
    "bytes": 1814953,
    "sha256": "103d9b46e25b75114d03ae87ecd3204dc3e0ae2eb047aec75402937cca976a63"
  },
  "sources": [
    {
      "file": "js/src/typed-program-link-rules.js",
      "previous": {
        "bytes": 18400,
        "sha256": "9a109701a11fb6150e5302d79406f1601fc1fd20ac425b3b6c245bde40ae701f"
      },
      "current": {
        "bytes": 18731,
        "sha256": "2958d7e259db8c4ddce69b1d4e736da69f3ab343a53461de9ad13cfcce106b91"
      }
    }
  ],
  "changes": [
    {
      "path": [
        "decisionSources",
        "9",
        "bytes"
      ],
      "before": 18400,
      "after": 18731
    },
    {
      "path": [
        "decisionSources",
        "9",
        "sha256"
      ],
      "before": "9a109701a11fb6150e5302d79406f1601fc1fd20ac425b3b6c245bde40ae701f",
      "after": "2958d7e259db8c4ddce69b1d4e736da69f3ab343a53461de9ad13cfcce106b91"
    }
  ],
  "sourceExtension": {
    "start": 8832,
    "end": 9163,
    "source": "/** @param {string} kind @param {string} field @returns {string} */\nexport function readProgramTypeNeutralFieldDefault(kind, field) {\n  if (((field === \"reference\") && ((kind === \"array\") || (kind === \"record\") || (kind === \"map\") || (kind === \"set\"))) || ((kind === \"record\" && field === \"open\"))) return \"false\";\n  return \"\";\n}\n\n"
  }
};

function readTypeNeutralCompleteDeclarationDescriptorSuccessor(artifact,identity,register){
 if(!Object.hasOwn(register,'typeNeutralDescriptorSuccessors'))return readCompleteDeclarationDescriptorSuccessor(artifact,identity,register);
 const {typeNeutralDescriptorSuccessors,...previousRegister}=register;
 const previousBytes=Buffer.from(structuredDataToLino(previousRegister));
 assert.equal(previousBytes.length,82262,'the complete prior register byte count is unchanged');
 assert.equal(digest(previousBytes),'35449016aca9b95e58fa0b27e172fe4b279ea5175b17cff88ce4137aa4c067bf',
  'removing only the third edge restores every complete original register byte');
 assert.deepEqual(typeNeutralDescriptorSuccessors,[typeNeutralDescriptorSuccessor],
  'the third edge and its whole source insertion are independently pinned');
 const edge=typeNeutralDescriptorSuccessors[0],bytes=Buffer.from(structuredDataToLino(artifact));
 if(identity!==edge.identity||digest(bytes)!==edge.current.sha256)return readCompleteDeclarationDescriptorSuccessor(artifact,identity,previousRegister);
 assert.equal(bytes.length,edge.current.bytes);
 const source=edge.sources[0],current=Buffer.from(restoreTranslationSourceHistory(
  readFileSync(new URL('../../'+source.file,import.meta.url),'utf8'),source.file,'2b9fd0df45757bf70a69deea6ab97d969cf6a25d'));
 assert.equal(current.length,source.current.bytes);assert.equal(digest(current),source.current.sha256);
 const insertion=edge.sourceExtension;
 assert.equal(current.subarray(insertion.start,insertion.end).toString('utf8'),insertion.source);
 const previous=Buffer.concat([current.subarray(0,insertion.start),current.subarray(insertion.end)]);
 assert.equal(previous.length,source.previous.bytes);assert.equal(digest(previous),source.previous.sha256,
  'removing only the complete generated declaration restores the whole previous source');
 const inverse=structuredClone(artifact);
 assert.deepEqual(inverse.decisionSources[9],{file:source.file,...source.current});
 inverse.decisionSources[9]={file:source.file,...source.previous};
 const restored=Buffer.from(structuredDataToLino(inverse));
 assert.equal(restored.length,edge.previous.bytes);assert.equal(digest(restored),edge.previous.sha256,
  'only the two source descriptor fields change in the entire raw artifact');
 return readCompleteDeclarationDescriptorSuccessor(inverse,identity,previousRegister);
}

/** The public standalone gate retains its cloned-data result and all checks. */
export function restoreCompleteDeclarationDescriptorSuccessor(artifact,identity,register){
 ({artifact,register}=restoreCurrentTranslationSourceDescriptors(artifact,identity,register));
 return readTypeNeutralCompleteDeclarationDescriptorSuccessor(artifact,identity,register).artifact;
}

/** Restore only a recorded source endpoint, archive receipt or source descriptor. */
export function restoreCompleteDeclarationReceipt(artifact,identity,{register=readProjectData(COMPLETE_DECLARATION_RECEIPTS)}={}){
 ({artifact,register}=restoreCurrentTranslationSourceDescriptors(artifact,identity,register));
 const successor=readTypeNeutralCompleteDeclarationDescriptorSuccessor(artifact,identity,register);
 artifact=successor.artifact;
 assert.equal(register.schemaVersion,1);
 const matches=register.receipts.filter(row=>row.identity===identity);
 assert.equal(matches.length,1,'one complete declaration-range receipt has this identity');
 const receipt=matches[0],bytes=successor.bytes,actual=digest(bytes);
 if(actual===receipt.previous.sha256){assert.equal(bytes.length,receipt.previous.bytes);return structuredClone(artifact);}
 assert.equal(actual,receipt.current.sha256,'the entire current artifact must match its reviewed receipt');
 assert.equal(bytes.length,receipt.current.bytes);
 assert.equal(receipt.previous.file,receipt.current.file);
 assert.match(receipt.previous.revision,/^[a-f0-9]{40}$/u);
 const inverse=structuredClone(artifact),paths=new Set();
 for(const change of receipt.changes){
  const path=change.path,key=JSON.stringify(path),last=path.at(-1);
  assert.ok(path.length>1&&!paths.has(key)&&path.every(part=>typeof part==='string'&&!['__proto__','prototype','constructor'].includes(part)),'each recorded path is distinct and owned');
  paths.add(key);
  const endpoint=['span','sourceSpan'].includes(path.at(-2))&&last==='end';
  const archive=path.includes('archive')&&['textBytes','textSha256','binaryBytes','binarySha256'].includes(last);
  const descriptor=identity==='raw consuming'&&['bytes','sha256'].includes(last)
   &&((path.length===3&&path[0]==='decisionSources'&&/^\d+$/u.test(path[1]))
    ||(path.length===6&&path.slice(0,4).join('.')==='predecessor.proof.producerApplicability.sources'&&/^\d+$/u.test(path[4])));
  assert.ok(endpoint||archive||descriptor,'semantic fields cannot be refreshed as source provenance');
  if(endpoint||last.endsWith('Bytes')||last==='bytes')for(const value of[change.before,change.after])assert.ok(Number.isSafeInteger(value)&&value>=0);
  else for(const value of[change.before,change.after])assert.match(value,/^[a-f0-9]{64}$/u);
  assert.notDeepEqual(change.before,change.after);
  let parent=inverse;
  for(const part of path.slice(0,-1)){assert.ok(Object.hasOwn(parent,part));parent=parent[part];}
  assert.ok(Object.hasOwn(parent,last));assert.deepEqual(parent[last],change.after);
  parent[last]=structuredClone(change.before);
 }
 const restored=Buffer.from(structuredDataToLino(inverse));
 assert.equal(restored.length,receipt.previous.bytes,'the complete previous byte count is restored');
 assert.equal(digest(restored),receipt.previous.sha256,'the complete immutable previous artifact is restored');
 return inverse;
}
