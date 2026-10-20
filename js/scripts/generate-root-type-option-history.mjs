// Preserve the exact original native assertions while correcting their API types.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {readSourceTextRustBlock} from './source-text-byte-library-style.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {restoreNativeTestLintSource} from './native-test-lint-source-history.mjs';

const root=new URL('../../',import.meta.url),revision='6fba3503a710325310e359995a1a77fcedf8011a';
const pin=bytes=>({bytes:Buffer.byteLength(bytes),sha256:createHash('sha256').update(bytes).digest('hex')});
export function generateRootTypeOptionHistory({check=false}={}) {
  const file='rust/tests/unit/root_types.rs',original=captureGitArtifactCertificate(revision,file);
  verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:file});
  const sourceText=original.blob.toString('utf8'),current=restoreNativeTestLintSource(file,readFileSync(new URL(file,root),'utf8'));
  const actions=[
    {from:'metadata.link_type(),\n            root_type_kind(declaration["type"].as_str().unwrap())',
      to:'metadata.link_type(),\n            Some(root_type_kind(declaration["type"].as_str().unwrap()))'},
    {from:'link.metadata().link_type() == LinkType::Type',to:'link.metadata().link_type() == Some(LinkType::Type)'},
    {from:'let types = network',to:'let type_designations = network'},
    {from:'types.len()',to:'type_designations.len()'},
    {from:'types[0]',to:'type_designations[0]'},
  ];
  let corrected=sourceText;
  for(const action of actions){assert.equal(corrected.split(action.from).length,2);corrected=corrected.replace(action.from,action.to);}
  assert.equal(corrected,current,'only the two Option corrections and recorded local binding rename are admitted');
  let inverse=current;for(const action of [...actions].reverse()){assert.equal(inverse.split(action.to).length,2);inverse=inverse.replace(action.to,action.from);}
  assert.equal(inverse,sourceText,'every original source byte assertion and input survives exact inverse');
  const accessorFile='rust/src/link_network.rs',accessorSource=readFileSync(new URL(accessorFile,root),'utf8');
  const accessor=readSourceTextRustBlock(accessorSource,'fn','link_type');
  assert.ok(accessor.startsWith('fn link_type(&self) -> Option<LinkType>'));
  assert.ok(accessor.includes('self.link_type'));
  const fixtureFile='parity/fixtures/root-type-behavior.lino',fixture=readFileSync(new URL(fixtureFile,root));
  const priorFixture=captureGitArtifactCertificate(revision,fixtureFile);assert.ok(fixture.equals(priorFixture.blob));
  const history={schemaVersion:1,revision,sources:[{file,commit:revision,...pin(original.blob),sourceText,certificate:original.certificate,current:pin(current)}],actions,
    accessor:{file:accessorFile,...pin(accessorSource),declaration:accessor},fixture:{file:fixtureFile,...pin(fixture)},
    observedCompilerDiagnostics:{run:39359700443,job:118431586516,logBytes:54994,logSha256:'15040dab1c7f25252bfab6dc4b636a40f66e270b15e87891cf41a6168cffbd8b',
      error:'E0308',expected:'Option<LinkType>',actual:'LinkType',lines:[96,108]},
    observedLintDiagnostics:{run:39359700443,job:118431586371,logBytes:200564,logSha256:'8d6f8c7e9b9c62c2f138704db97e9c63b9a71943c84681390efcaf0f1c729feb',
      lint:'similar_names',originalBinding:'types',correctedBinding:'type_designations'},
    boundary:'The current actual accessor returns an optional metadata type. Both present-type expectations select Some and one local binding has an unambiguous name, without changing any original fixture, assertion, graph role or refusal. Historical source membership and exact source inverse supply no corrected native execution evidence.'};
  const output=new URL('../../parity/self-translation/root-type-option-source-history.lino',import.meta.url);
  if(check)checkProjectData(output,history);else writeProjectData(output,history);
  return history;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateRootTypeOptionHistory({check:process.argv.includes('--check')});
  console.log('Retained every original root-type input and assertion and recorded both observed optional accessor corrections.');
}
