// Historical inventory names retain their complete original source meaning.
// Actual current names are extracted separately by the unchanged naming policy.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const policy={
  "schemaVersion": 1,
  "model": "own codebase baseline historical naming 1",
  "revision": "897bfc8b2dcc5ec561feffa455ddeb9f95d24335",
  "sourceArchive": {
    "path": "parity/naming/own-codebase-baseline-predecessor-source-history.lino",
    "bytes": 9872519,
    "sha256": "2aab89c9bf21a9e6f3addf28776bed84ccfdb192e6bb69b59c3fa546e1734abc",
    "sources": 46
  },
  "views": [
    {
      "inventory": "B61 actual lexical binding source names for rust/src/translation/javascript/callables.rs",
      "file": "rust/src/translation/javascript/callables.rs",
      "names": [
        {
          "name": "read native source closure",
          "record": "repository.read-native-source-closure"
        }
      ],
      "current": {
        "bytes": 18170,
        "sha256": "5560bd782fe102cf39a2b1c5cbe1762b84529f3ac1ee91de665c0af7c50c9768"
      }
    },
    {
      "inventory": "B62 authentic empty String compiler style control name for js/tests/translation-frontend-rule-styles.test.js",
      "file": "js/tests/translation-frontend-rule-styles.test.js",
      "names": [
        {
          "name": "check source rule empty string compiler styles",
          "record": "repository.check-source-rule-empty-string-compiler-styles"
        }
      ],
      "current": {
        "bytes": 29042,
        "sha256": "b4204db7b19552ecb8a730c6859f61851ce5586da016b67c59adfbba06c9d0a7"
      }
    }
  ],
  "driver": {
    "file": "js/scripts/check-naming.mjs",
    "original": {
      "bytes": 1582,
      "sha256": "d4480b57fde445a43737364f3a155e83fee4766d1aaaa620cb5d03d7561d89c1"
    },
    "current": {
      "bytes": 1900,
      "sha256": "868168d71bf9f6c032e5846c6ec85a646c694f8702b556386c4e274a16c51509"
    },
    "edits": [
      {
        "before": "import { checkNamingFixtures, checkRepositoryNames, NAMING_FIXTURES } from './issue-195-naming.mjs';",
        "after": "import { checkNamingFixtures, checkRepositoryNames, extractNameInventory, NAME_INVENTORIES, NAMING_FIXTURES } from './issue-195-naming.mjs';\nimport { extractOwnCodebaseHistoricalNames } from './own-codebase-baseline-naming-history.mjs';"
      },
      {
        "before": "const { problems, records, names, formers, vocabulary } = checkRepositoryNames(root, wordnet);",
        "after": "const currentNames = extractNameInventory(root);\nconst historicalNames = extractOwnCodebaseHistoricalNames(root, NAME_INVENTORIES);\nconst { problems, records, names, formers, vocabulary } = checkRepositoryNames(root, wordnet, { names: [...currentNames, ...historicalNames] });"
      }
    ]
  }
};
const digest=bytes=>createHash('sha256').update(bytes).digest('hex');
const pin=bytes=>({bytes:Buffer.byteLength(bytes),sha256:digest(bytes)});
function exactKeys(value,keys){assert.ok(value&&typeof value==='object'&&!Array.isArray(value));assert.deepEqual(Object.keys(value).sort(),keys.slice().sort());}
let additionalSnapshot,originalSnapshot;

/** Return a detached copy; callers cannot change the trusted source anchors. */
export function readOwnCodebaseHistoricalNamingPolicy(){
  const text=readFileSync(new URL('../../parity/naming/array-identity-historical-inventories.lino',import.meta.url),'utf8');
  if(additionalSnapshot?.text===text)return{...structuredClone(policy),additionalSourceViews:structuredClone(additionalSnapshot.value)};
  additionalSnapshot=undefined;
  const additional=readBootstrapStructuredDataLino(text);
  exactKeys(additional,['schemaVersion','model','revision','sources','views','boundary']);
  assert.equal(additional.schemaVersion,1);assert.equal(additional.model,'array identity historical naming inventories 1');
  assert.equal(additional.revision,'6ffff3b2dc530a5f24c128dadf877f3fa106932b');
  assert.ok(Array.isArray(additional.sources)&&additional.sources.length>=4);
  assert.equal(new Set(additional.sources.map(row=>row.file+'@'+row.predecessor.revision)).size,additional.sources.length);
  for(const row of additional.sources){
    exactKeys(row,['file','predecessor','current']);exactKeys(row.predecessor,['revision','source','bytes','sha256','certificate']);
    assert.match(row.file,/^(?:js\/(?:src|scripts|tests)|rust\/(?:src|tests\/unit)|experiments)\/[A-Za-z_0-9./-]+\.(?:js|mjs|rs)$/u);assert.equal(row.file.includes('..'),false);
    assert.ok(['6ffff3b2dc530a5f24c128dadf877f3fa106932b','8bc0ef9725388c7927acf1854870bf46f5e365be'].includes(row.predecessor.revision));
    exactKeys(row.current,['bytes','sha256']);assert.equal(typeof row.predecessor.source,'string');
    assert.deepEqual(pin(row.predecessor.source),{bytes:row.predecessor.bytes,sha256:row.predecessor.sha256});
    verifyGitArtifactCertificate(row.predecessor.certificate,Buffer.from(row.predecessor.source),{revision:row.predecessor.revision,path:row.file});
    assert.ok(Number.isSafeInteger(row.current.bytes)&&row.current.bytes>0);assert.match(row.current.sha256,/^[a-f0-9]{64}$/u);
    // Selection authenticates the caller's actual root below. Reading immutable
    // policy must not select unrelated files from this checkout as authority.
  }
  for(const view of additional.views){
    exactKeys(view,['inventory','file','revision','names']);assert.ok(additional.sources.some(row=>row.file===view.file&&row.predecessor.revision===view.revision));
    assert.equal(typeof view.inventory,'string');assert.ok(Array.isArray(view.names)&&view.names.length>0);
    for(const row of view.names){exactKeys(row,['name','record']);assert.equal(typeof row.name,'string');assert.equal(typeof row.record,'string');}
  }
  additionalSnapshot={text,value:structuredClone(additional)};
  return{...structuredClone(policy),additionalSourceViews:additional};
}

/** Validate the compact complete source projection without loading the large archive. */
export function verifyOwnCodebaseHistoricalNamingSources(history){
  exactKeys(history,['schemaVersion','model','revision','sourceArchive','views','driver']);
  for(const key of ['schemaVersion','model','revision','sourceArchive'])assert.deepEqual(history[key],policy[key]);
  assert.ok(Array.isArray(history.views));assert.equal(history.views.length,policy.views.length);
  for(const [index,view]of history.views.entries()){
    const expected=policy.views[index];exactKeys(view,['inventory','file','names','current','predecessor']);
    for(const key of ['inventory','file','names','current'])assert.deepEqual(view[key],expected[key]);
    exactKeys(view.predecessor,['source','bytes','sha256','certificate']);
    assert.equal(typeof view.predecessor.source,'string');
    const bytes=Buffer.from(view.predecessor.source);assert.equal(bytes.toString('utf8'),view.predecessor.source);
    assert.deepEqual(pin(bytes),{bytes:view.predecessor.bytes,sha256:view.predecessor.sha256});
    verifyGitArtifactCertificate(view.predecessor.certificate,bytes,{revision:policy.revision,path:view.file});
  }
  exactKeys(history.driver,['file','original','current','edits','source','certificate']);
  for(const key of ['file','original','current','edits'])assert.deepEqual(history.driver[key],policy.driver[key]);
  assert.equal(typeof history.driver.source,'string');
  assert.deepEqual(pin(history.driver.source),policy.driver.original);
  verifyGitArtifactCertificate(history.driver.certificate,Buffer.from(history.driver.source),{revision:policy.revision,path:policy.driver.file});
  let current=history.driver.source;
  for(const edit of policy.driver.edits){assert.equal(current.split(edit.before).length,2);current=current.replace(edit.before,()=>edit.after);}
  assert.deepEqual(pin(current),policy.driver.current);
  let inverse=current;
  for(const edit of policy.driver.edits.toReversed()){assert.equal(inverse.split(edit.after).length,2);inverse=inverse.replace(edit.after,()=>edit.before);}
  assert.equal(inverse,history.driver.source,'every original driver byte and unchanged policy call remains recoverable');
  return history;
}

/** Bootstrap decoding avoids retaining a second full graph in the normal check. */
export function readOwnCodebaseHistoricalNamingSources(){
  const file=new URL('../../parity/naming/own-codebase-baseline-naming-history.lino',import.meta.url);
  const text=readFileSync(file,'utf8');
  if(originalSnapshot?.text!==text){originalSnapshot=undefined;originalSnapshot={text,value:verifyOwnCodebaseHistoricalNamingSources(readBootstrapStructuredDataLino(text))};}
  return structuredClone(originalSnapshot.value);
}

/** Supplement only two original inventories after authenticating current modules. */
export function extractOwnCodebaseHistoricalNames(root,inventories,history=readOwnCodebaseHistoricalNamingSources()){
  verifyOwnCodebaseHistoricalNamingSources(history);
  assert.deepEqual(pin(readFileSync(resolve(root,policy.driver.file))),policy.driver.current,'the entire naming driver has its reviewed additive source view');
  const names=[];
  for(const view of history.views){
    assert.deepEqual(pin(readFileSync(resolve(root,view.file))),view.current,'unknown complete current source cannot acquire historical naming authority');
    const entries=inventories.filter(entry=>entry.inventory===view.inventory&&entry.files.length===1&&entry.files[0]===view.file);
    assert.equal(entries.length,1,'one unchanged actual historical inventory extractor');
    const entry=entries[0],found=[...new Set(entry.extract(view.predecessor.source,view.file))].map(name=>({name,record:entry.recordOf(name)}));
    assert.deepEqual(found,view.names,'every name comes from the complete certified original source and unchanged extractor');
    names.push(...found.map(row=>({inventory:view.inventory,file:view.file,...row,sourceView:{
      kind:'complete certified historical source',revision:policy.revision,path:view.file,
      bytes:view.predecessor.bytes,sha256:view.predecessor.sha256,
      archive:'parity/naming/own-codebase-baseline-naming-history.lino',
      boundary:'Historical definitions keep their original identities. Every actual current inventory row is retained separately; changed source owners have distinct qualified records.'}})));
  }
  const additional=readOwnCodebaseHistoricalNamingPolicy().additionalSourceViews;
  for(const view of additional.views){
    const source=additional.sources.find(row=>row.file===view.file&&row.predecessor.revision===view.revision);
    const entries=inventories.filter(entry=>entry.inventory===view.inventory&&entry.files.includes(view.file));
    if(entries.length===0)continue;
    assert.equal(entries.length,1,'the same actual extractor owns the additional historical names');
    assert.deepEqual(pin(readFileSync(resolve(root,view.file))),source.current,'the selected root retains every complete actual source byte');
    const entry=entries[0],found=[...new Set(entry.extract(source.predecessor.source,view.file))]
      .map(name=>({name,record:entry.recordOf(name)}));
    const current=new Set(entry.extract(readFileSync(resolve(root,view.file),'utf8'),view.file));
    for(const row of view.names){
      assert.ok(found.some(candidate=>candidate.name===row.name&&candidate.record===row.record),'an unchanged extractor must derive each original name and identity');
      assert.equal(current.has(row.name),false,'actual current names stay in the separate current inventory');
      names.push({inventory:view.inventory,file:view.file,...row,sourceView:{
        kind:'complete certified historical source',revision:view.revision,path:view.file,
        bytes:source.predecessor.bytes,sha256:source.predecessor.sha256,
        archive:'parity/naming/array-identity-historical-inventories.lino',boundary:additional.boundary}});
    }
  }
  return names;
}
