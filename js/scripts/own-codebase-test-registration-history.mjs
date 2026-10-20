// Exact source views preserve every historical assertion while current test discovery grows.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const policy={
  "schemaVersion": 1,
  "model": "own codebase test registration source history 1",
  "revision": "52132727d7ce158762a4499b7032a42a9f16f3c3",
  "registrations": [
    {
      "file": "boolean-declaration-value-hosts.test.js",
      "group": "conformance"
    },
    {
      "file": "formal-function-declaration-values.test.js",
      "group": "conformance"
    },
    {
      "file": "own-codebase-baseline-naming-history.test.js",
      "group": "tooling"
    }
  ],
  "sources": [
    {
      "file": "js/scripts/test-groups.mjs",
      "predecessor": {
        "bytes": 7111,
        "sha256": "38be4eb51ddcb241e1a454616049be44a088aa35ac610d7dfdeb8fcc8535ddcb"
      },
      "current": {
        "bytes": 7218,
        "sha256": "cf053accd3c14f555ae85231ce57dd73d64c3fc8983e9279d8863dd843a37d43"
      },
      "edits": [
        {
          "before": "formal-function-links$|shared-numeric-function-links$",
          "after": "formal-function-links$|formal-function-declaration-values$|boolean-declaration-value-hosts$|shared-numeric-function-links$"
        },
        {
          "before": "ci-rocq-toolchain-cache$|export-parity$",
          "after": "ci-rocq-toolchain-cache$|export-parity$|own-codebase-baseline-naming-history$"
        }
      ]
    },
    {
      "file": "js/tests/issue-195-evidence-node-dependency.test.js",
      "predecessor": {
        "bytes": 6334,
        "sha256": "41426d592be6cfbad5781f959971b93090f8def2c9fa7aae87b75c35f3258e20"
      },
      "current": {
        "bytes": 7481,
        "sha256": "0f2f9cfdd17ad49c46b62f805738f7e43b35ee1d1e44a11db04102f75f1f731b"
      },
      "edits": [
        {
          "before": "  const classifier=readFileSync(new URL('../../'+location.classifier.file,import.meta.url));",
          "after": "  const classifier=restoreOwnCodebaseTestRegistrationClassifier(readFileSync(new URL('../../'+location.classifier.file,import.meta.url)));"
        },
        {
          "before": "  assert.deepEqual(partitionTests(['issue-195-evidence-node-dependency.test.js']).groups.tooling,['issue-195-evidence-node-dependency.test.js']);\n",
          "after": "  assert.deepEqual(partitionTests(['issue-195-evidence-node-dependency.test.js']).groups.tooling,['issue-195-evidence-node-dependency.test.js']);\n  const registrationHistory=readOwnCodebaseTestRegistrationHistory();\n  assert.equal(registrationHistory.revision,'52132727d7ce158762a4499b7032a42a9f16f3c3');\n  for(const row of registrationHistory.sources){\n    const current=readFileSync(new URL('../../'+row.file,import.meta.url));\n    assert.equal(current.length,row.current.bytes);\n    assert.equal(createHash('sha256').update(current).digest('hex'),row.current.sha256);\n  }\n  for(const row of registrationHistory.registrations){\n    assert.ok(files.includes(row.file));\n    const current=partitionTests([row.file]);\n    assert.deepEqual(current.unassigned,[]);assert.deepEqual(current.ambiguous,[]);\n    assert.deepEqual(current.groups[row.group],[row.file]);\n  }\n  assert.throws(()=>restoreOwnCodebaseTestRegistrationClassifier(Buffer.from('unknown complete classifier')));\n  assert.throws(()=>restoreNodeDependencyGeneratorTestHistory(readFileSync(new URL(import.meta.url),'utf8')+'\\n'));\n"
        },
        {
          "before": "import {readNodeDependencyInventoryOriginalView} from '../scripts/dependency-inventory-append-history.mjs';",
          "after": "import {readNodeDependencyInventoryOriginalView} from '../scripts/dependency-inventory-append-history.mjs';\nimport {readOwnCodebaseTestRegistrationHistory,restoreOwnCodebaseTestRegistrationClassifier} from '../scripts/own-codebase-test-registration-history.mjs';"
        }
      ]
    },
    {
      "file": "js/scripts/dependency-inventory-append-history.mjs",
      "predecessor": {
        "bytes": 3897,
        "sha256": "b565ef42762f2ccecce0b657a241fce87debbcce645b88cc93e3bbf5954dc511"
      },
      "current": {
        "bytes": 4106,
        "sha256": "e78a88fa8467ca3bca2c2565e9ecc1f59c43128c41ea601a1bc97b2a3f5f76a2"
      },
      "edits": [
        {
          "before": "import {checkInventory,collectDependencies} from './dependency-inventory.mjs';",
          "after": "import {checkInventory,collectDependencies} from './dependency-inventory.mjs';\nimport {restoreOwnCodebaseTestRegistrationTestSource} from './own-codebase-test-registration-history.mjs';"
        },
        {
          "before": "  assert.ok(source===original||source===current,'only the complete original or reviewed inventory test source is admitted');",
          "after": "  if(source!==original&&source!==current)source=restoreOwnCodebaseTestRegistrationTestSource(source);\n  assert.ok(source===original||source===current,'only the complete original or reviewed inventory test source is admitted');"
        }
      ]
    }
  ]
};
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
const exactKeys=(value,keys)=>{assert.ok(value&&typeof value==='object'&&!Array.isArray(value));assert.deepEqual(Object.keys(value).sort(),keys.slice().sort());};
const replay=(source,edits)=>{for(const edit of edits){assert.equal(source.split(edit.before).length,2);source=source.replace(edit.before,()=>edit.after);}return source;};

/** Return detached immutable source anchors and the three actual current assignments. */
export function readOwnCodebaseTestRegistrationPolicy(){return structuredClone(policy);}

/** Check complete independently committed predecessors and each finite byte inverse. */
export function verifyOwnCodebaseTestRegistrationHistory(history){
  exactKeys(history,['schemaVersion','model','revision','registrations','sources']);
  for(const key of ['schemaVersion','model','revision','registrations'])assert.deepEqual(history[key],policy[key]);
  assert.ok(Array.isArray(history.sources));assert.equal(history.sources.length,policy.sources.length);
  for(const [index,row]of history.sources.entries()){
    const expected=policy.sources[index];exactKeys(row,['file','predecessor','current','edits']);
    for(const key of ['file','current','edits'])assert.deepEqual(row[key],expected[key]);
    exactKeys(row.predecessor,['bytes','sha256','source','certificate']);
    assert.equal(typeof row.predecessor.source,'string');
    const bytes=Buffer.from(row.predecessor.source);assert.equal(bytes.toString('utf8'),row.predecessor.source);
    assert.deepEqual(pin(bytes),expected.predecessor);
    assert.equal(row.predecessor.bytes,expected.predecessor.bytes);assert.equal(row.predecessor.sha256,expected.predecessor.sha256);
    verifyGitArtifactCertificate(row.predecessor.certificate,bytes,{revision:policy.revision,path:row.file});
    const current=replay(row.predecessor.source,row.edits);assert.deepEqual(pin(current),row.current);
    let inverse=current;
    for(const edit of row.edits.toReversed()){assert.equal(inverse.split(edit.after).length,2);inverse=inverse.replace(edit.after,()=>edit.before);}
    assert.equal(inverse,row.predecessor.source,'every original source byte and assertion remains exact');
  }
  return history;
}

/** Normal offline controls need only this compact Links archive. */
export function readOwnCodebaseTestRegistrationHistory(){
  const source=readFileSync(new URL('../../parity/naming/own-codebase-test-registration-source-history.lino',import.meta.url),'utf8');
  return verifyOwnCodebaseTestRegistrationHistory(readBootstrapStructuredDataLino(source));
}

function restoreSource(source,file){
  assert.equal(typeof source,'string');
  const history=readOwnCodebaseTestRegistrationHistory();
  const row=history.sources.find(row=>row.file===file);assert.ok(row);
  const current=replay(row.predecessor.source,row.edits);
  assert.ok(source===row.predecessor.source||source===current,'unknown complete sources cannot acquire historical test authority');
  return row.predecessor.source;
}

/** The existing classifier byte count and digest assertions still see every original byte. */
export function restoreOwnCodebaseTestRegistrationClassifier(source){
  assert.ok(Buffer.isBuffer(source));
  const text=source.toString('utf8');assert.deepEqual(Buffer.from(text),source);
  return Buffer.from(restoreSource(text,'js/scripts/test-groups.mjs'));
}

/** The preceding helper still independently checks its original two supported source forms. */
export function restoreOwnCodebaseTestRegistrationTestSource(source){
  return restoreSource(source,'js/tests/issue-195-evidence-node-dependency.test.js');
}
