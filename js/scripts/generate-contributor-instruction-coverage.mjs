// Capture the two immutable instruction revisions and their reviewed coverage.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {readContributorInstructionUnits,CONTRIBUTOR_INSTRUCTION_COVERAGE_FILE} from './contributor-instruction-coverage.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';

const root=new URL('../../',import.meta.url);
const originalRevision='f0ae00412047227ccfb4826c30608841d16ea7a6';
const compactRevision='e0d407019196e5ea72e85c7b10afa09407108066';
const digest=value=>createHash('sha256').update(value).digest('hex');

export function generateContributorInstructionCoverage({check=false}={}) {
  const sources={};
  for(const [id,revision,path]of [['originalAgents',originalRevision,'AGENTS.md'],
    ['compactAgents',compactRevision,'AGENTS.md'],['sharedReference',compactRevision,'CLAUDE.md']]){
    const {certificate,blob}=captureGitArtifactCertificate(revision,path);
    sources[id]={text:blob.toString('utf8'),certificate};
  }
  const requestPath='parity/sources/compact-contributor-instructions-and-assertion-corrections-2026-10-16.txt';
  const versionPath='parity/sources/version-bounded-assertions-2026-10-16.txt';
  const requestSource=readFileSync(new URL(requestPath,root),'utf8'),versionSource=readFileSync(new URL(versionPath,root),'utf8');
  const current=readContributorInstructionUnits(sources.compactAgents.text),original=readContributorInstructionUnits(sources.originalAgents.text);
  assert.equal(current.units.length,20);assert.equal(original.units.length,28);
  const ids=['all contributors and setup','vision authority','source audit','atomic requirements','assertion corrections',
    'version ranges','execution evidence','required checks and checkpoints','completion boundary','JavaScript batches',
    'grammar generation','shared decorators','runtime parity','English names','artifact locations','bulk CI work',
    'local checks and resource limits','CI workloads','cache ownership','safe cleanup'];
  const mapping=[
    [0],[1],[2,3],[3,4],[6],[6],[4,7],[7],[8],
    [1,9,15,17],[9,12],[9],[10],[11],[15],[15],[16],[12],[13],[14],[14],
    [16,17],[16],[16,17],[18],[16,17],[15,16,18,19],[18,19],
  ];
  assert.equal(mapping.length,original.units.length);
  const policy={schemaVersion:1,model:'reviewed contributor instruction source coverage 1',sources,
    request:{path:requestPath,source:requestSource,sha256:digest(requestSource)},
    versionRequest:{path:versionPath,source:versionSource,sha256:digest(versionSource)},
    currentUnits:current.units.map((source,index)=>({id:ids[index],source})),
    coverage:original.units.map((source,index)=>({originalIndex:index,source,coveredBy:mapping[index].map(index=>ids[index]),
      ...(index===3?{authorizedCorrection:digest(requestSource)}:{})})),
    additions:[{unit:ids[5],authority:digest(versionSource)}],
    boundary:'This is a Root-reviewed exhaustive correspondence of all 25 original bullets and three surrounding instruction paragraphs to the shortened shared contract, with the explicitly authorized assertion correction and added version bounds. The original sources and every heading/context are independently authenticated. Checks enforce retained wording, source coverage and measured concision; they do not prove arbitrary English equivalence or a globally optimal word count. CLAUDE.md was absent at the preceding Git revision; its new single reference does not claim a reduction against an unavailable earlier file. AGENTS.md and the vision remain authoritative.'};
  const file=new URL(CONTRIBUTOR_INSTRUCTION_COVERAGE_FILE,root);
  if(check)checkProjectData(file,policy);else writeProjectData(file,policy);
  return policy;
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateContributorInstructionCoverage({check:process.argv.includes('--check')});
  console.log('Contributor coverage: 28 complete original units and 20 reviewed current units captured automatically.');
}
