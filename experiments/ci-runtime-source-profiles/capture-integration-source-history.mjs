import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../js/scripts/capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';

const integrationPredecessor='8bc0ef9725388c7927acf1854870bf46f5e365be';
const schedulingPredecessor='ec9f4efb0e9ef936114e09823981352f6db13700';
const inputs=[
  ['js/tests/ci-workflow-structure.test.js',integrationPredecessor,[
    '3f23d62d8b4d3081e71ada1c2f60c37ebb978550ceb9564cae7f622ad8780485',
  ]],
  ['js/tests/test-groups.test.js',integrationPredecessor,[
    '3d0284e857dce164f4dcfc06784547b5e5e0312e93f581a40941e08e941efef8',
    '12f0f07a38e2d95ef60eaba218dde5f6d45c22fdbfed0234188a7f1131e896d4',
  ]],
  ['js/scripts/test-groups.mjs',schedulingPredecessor,[
    'eb3714069fc2701c0de60aaa7ecd211a192c6a1db40d8920f33a542dffed37fa',
  ]],
  ['js/tests/issue-195-evidence-node-dependency.test.js',schedulingPredecessor,[
    '5c5ace4a4f3be5b60912ed5807655509aabe92ab74a160d15d6dcca17a657917',
  ]],
  ['rust/Cargo.toml',integrationPredecessor,[
    'acec16f4f563c7bd92a97e50b5efd59f0a3f3740b49a7c5d308edac62282a5dd',
  ]],
];

if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);
  assert.ok(args.length===0||(args.length===1&&args[0]==='--check'));
  const sources=inputs.map(([path,revision,current])=>{
    const {blob,certificate}=captureGitArtifactCertificate(revision,path);
    const source=blob.toString('utf8');
    assert.deepEqual(Buffer.from(source),blob);
    return {path,revision,current,source,certificate};
  });
  const file=new URL('../../parity/self-translation/integration-source-history.lino',import.meta.url);
  const history={schemaVersion:1,sources};
  if(args.length)checkProjectData(file,history);
  else writeProjectData(file,history);
  console.log(`Authenticated ${sources.length} complete source predecessors.`);
}
