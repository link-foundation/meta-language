// Bounded source-data proofs belong to the existing tooling test group.
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {mkdtempSync,mkdirSync,writeFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import test from 'node:test';
import {captureGitArtifactCertificate} from '../scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {readStructuredDataLino,structuredDataToLino} from '../src/structured-data-links.js';

function makeGitCertificateFixture() {
  const repository=mkdtempSync(join(tmpdir(),'meta-language-git-proof-'));
  const git=args=>execFileSync('git',['-c','core.hooksPath=/dev/null','-c','commit.gpgsign=false',
    '-c','user.name=Certificate fixture','-c','user.email=fixture@example.invalid',...args],{cwd:repository,encoding:'utf8'});
  git(['init','--quiet']);mkdirSync(join(repository,'folder'));
  const path='folder/λ.lino',blob=Buffer.from('(original: λ)\n');
  writeFileSync(join(repository,path),blob);writeFileSync(join(repository,'sibling.lino'),'(sibling: retained)\n');
  writeFileSync(join(repository,'folder/other.lino'),'(other: retained)\n');
  writeFileSync(join(repository,'binary.bin'),Buffer.from([0,255,10]));
  git(['add','--','.']);git(['commit','--quiet','-m','Original certificate fixture']);
  return{repository,git,path,blob,revision:git(['rev-parse','HEAD']).trim()};
}

test('real Git certificates preserve complete original objects and verify after the Git database is unavailable',()=>{
  const fixture=makeGitCertificateFixture();
  try{
    const {certificate,blob}=captureGitArtifactCertificate(fixture.revision,fixture.path,{repository:fixture.repository});
    assert.deepEqual(blob,fixture.blob);
    assert.deepEqual(Buffer.from(fixture.git(['show',fixture.revision+':'+fixture.path])),blob);
    const restored=readStructuredDataLino(structuredDataToLino(certificate));assert.deepEqual(restored,certificate);
    const binary=captureGitArtifactCertificate(fixture.revision,'binary.bin',{repository:fixture.repository});
    assert.deepEqual(verifyGitArtifactCertificate(binary.certificate,binary.blob,{revision:fixture.revision,path:'binary.bin'}),Buffer.from([0,255,10]));
    rmSync(join(fixture.repository,'.git'),{recursive:true});
    const verified=verifyGitArtifactCertificate(restored,blob,{revision:fixture.revision,path:fixture.path});
    assert.deepEqual(verified,fixture.blob);verified[0]=0;assert.deepEqual(blob,fixture.blob,'returned ownership does not mutate the verified input');
  }finally{rmSync(fixture.repository,{recursive:true,force:true});}
});

test('Git certificates refuse forged roots, every path or sibling edit and a valid replacement from another commit',()=>{
  const fixture=makeGitCertificateFixture();
  try{
    const {certificate,blob}=captureGitArtifactCertificate(fixture.revision,fixture.path,{repository:fixture.repository});
    const verify=value=>verifyGitArtifactCertificate(value,blob,{revision:fixture.revision,path:fixture.path});
    for(const mutate of [
      value=>{value.schemaVersion=2;},value=>{value.objectFormat='sha256';},
      value=>{value.revision='f'.repeat(40);},value=>{value.path='sibling.lino';},
      value=>{value.commitText+=' ';},value=>{value.trees.pop();},value=>{value.trees.push(value.trees[0]);},
      value=>{value.trees.reverse();},value=>{value.trees[0].objectId='0'.repeat(40);},
      value=>{value.trees[0].entries.reverse();},value=>{value.trees[0].entries.pop();},
      value=>{value.trees[0].entries[0].name+='changed';},
      value=>{value.trees[0].entries[0].mode='100755';},
      value=>{value.trees[0].entries[0].objectId='0'.repeat(40);},
      value=>{value.trees[0].entries.push(value.trees[0].entries[0]);},
      value=>{value.blob.bytes+=1;},value=>{value.blob.objectId='0'.repeat(40);},
      value=>{value.blob.sha256='0'.repeat(64);},value=>{value.extra='unproved';},
    ]){const altered=structuredClone(certificate);mutate(altered);assert.throws(()=>verify(altered));}
    const alteredBlob=Buffer.from(blob);alteredBlob[0]^=1;
    assert.throws(()=>verifyGitArtifactCertificate(certificate,alteredBlob,{revision:fixture.revision,path:fixture.path}));
    assert.throws(()=>verifyGitArtifactCertificate(certificate,blob,{revision:'0'.repeat(40),path:fixture.path}));
    assert.throws(()=>verifyGitArtifactCertificate(certificate,blob,{revision:fixture.revision,path:'../'+fixture.path}));
    let invoked=false;const hostile=structuredClone(certificate);
    Object.defineProperty(hostile,'trees',{enumerable:true,get(){invoked=true;throw new Error('must not invoke');}});
    assert.throws(()=>verify(hostile));assert.equal(invoked,false);
    writeFileSync(join(fixture.repository,fixture.path),'(different: content)\n');
    fixture.git(['add','--',fixture.path]);fixture.git(['commit','--quiet','-m','Different immutable source']);
    const replacement=captureGitArtifactCertificate(fixture.git(['rev-parse','HEAD']).trim(),fixture.path,{repository:fixture.repository});
    assert.throws(()=>verifyGitArtifactCertificate(replacement.certificate,replacement.blob,{revision:fixture.revision,path:fixture.path}));
  }finally{rmSync(fixture.repository,{recursive:true,force:true});}
});

test('Git certificate resource limits admit exact original objects and refuse every immediate predecessor',()=>{
  const fixture=makeGitCertificateFixture();
  try{
    const {certificate,blob}=captureGitArtifactCertificate(fixture.revision,fixture.path,{repository:fixture.repository});
    const limits={maxBlobBytes:blob.length,maxCommitBytes:Buffer.byteLength(certificate.commitText),maxPathDepth:certificate.trees.length,
      maxTreeEntries:Math.max(...certificate.trees.map(tree=>tree.entries.length)),
      maxTreeBytes:Math.max(...certificate.trees.map(tree=>Number(fixture.git(['cat-file','-s',tree.objectId]).trim())))};
    assert.deepEqual(verifyGitArtifactCertificate(certificate,blob,{revision:fixture.revision,path:fixture.path,limits}),blob);
    for(const key of Object.keys(limits)){
      assert.ok(limits[key]>1);
      assert.throws(()=>verifyGitArtifactCertificate(certificate,blob,{revision:fixture.revision,path:fixture.path,limits:{...limits,[key]:limits[key]-1}}));
      for(const value of [NaN,Infinity,-Infinity])assert.throws(()=>verifyGitArtifactCertificate(certificate,blob,{revision:fixture.revision,path:fixture.path,limits:{...limits,[key]:value}}));
    }
  }finally{rmSync(fixture.repository,{recursive:true,force:true});}
});
